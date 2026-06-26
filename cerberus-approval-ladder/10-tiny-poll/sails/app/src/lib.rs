#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use core::cell::RefCell;
use sails_rs::prelude::*;

const MAX_OPTIONS: usize = 4;
const MAX_OPTION_BYTES: usize = 48;

#[sails_rs::sails_type]
#[derive(Clone, Debug, PartialEq)]
pub struct PollInput {
    pub question_hash: [u8; 32],
    pub option_labels: Vec<String>,
    pub closes_at: u64,
}

#[sails_rs::sails_type]
#[derive(Clone, Debug, PartialEq)]
pub struct Poll {
    pub id: u64,
    pub creator: ActorId,
    pub input: PollInput,
    pub closed: bool,
}

#[sails_rs::sails_type]
#[derive(Clone, Debug, PartialEq)]
pub struct PollResult {
    pub poll_id: u64,
    pub counts: Vec<u64>,
    pub closed: bool,
}

#[sails_rs::sails_type]
#[derive(Clone, Debug, PartialEq)]
pub enum TinyPollError {
    ZeroQuestionHash,
    ZeroEvidenceHash,
    InvalidOptionCount,
    OptionTooLong,
    UnknownPoll,
    PollClosed,
    InvalidOption,
    AlreadyVoted,
    NotCreator,
}

#[derive(Default, Clone)]
pub struct TinyPollState {
    pub next_id: u64,
    pub polls: BTreeMap<u64, Poll>,
    pub counts: BTreeMap<u64, Vec<u64>>,
    pub voted: BTreeMap<(u64, ActorId), u8>,
    pub evidence_by_vote: BTreeMap<(u64, ActorId), [u8; 32]>,
}

struct TinyPoll<S: StateMut<Item = TinyPollState, Error = Infallible> = RefCell<TinyPollState>> {
    state: S,
}

impl<S: StateMut<Item = TinyPollState, Error = Infallible>> TinyPoll<S> {
    pub fn new(state: S) -> Self {
        Self { state }
    }

    fn validate_input(input: &PollInput) -> Result<(), TinyPollError> {
        if input.question_hash == [0u8; 32] {
            return Err(TinyPollError::ZeroQuestionHash);
        }
        if input.option_labels.len() < 2 || input.option_labels.len() > MAX_OPTIONS {
            return Err(TinyPollError::InvalidOptionCount);
        }
        if input
            .option_labels
            .iter()
            .any(|label| label.len() > MAX_OPTION_BYTES)
        {
            return Err(TinyPollError::OptionTooLong);
        }
        Ok(())
    }
}

#[sails_rs::service]
impl<S: StateMut<Item = TinyPollState, Error = Infallible>> TinyPoll<S> {
    #[export]
    pub fn create_poll(&mut self, input: PollInput) -> Result<u64, TinyPollError> {
        TinyPoll::<S>::validate_input(&input)?;

        let creator = Syscall::message_source();
        let option_count = input.option_labels.len();
        let poll_id = {
            let mut state = self.state.get_mut();
            state.next_id += 1;
            let poll_id = state.next_id;
            state.polls.insert(
                poll_id,
                Poll {
                    id: poll_id,
                    creator,
                    input,
                    closed: false,
                },
            );
            state.counts.insert(poll_id, vec![0; option_count]);
            poll_id
        };

        Ok(poll_id)
    }

    #[export]
    pub fn vote(
        &mut self,
        poll_id: u64,
        option_index: u8,
        evidence_hash: [u8; 32],
    ) -> Result<(), TinyPollError> {
        if evidence_hash == [0u8; 32] {
            return Err(TinyPollError::ZeroEvidenceHash);
        }

        let voter = Syscall::message_source();
        {
            let mut state = self.state.get_mut();
            let poll = state
                .polls
                .get(&poll_id)
                .ok_or(TinyPollError::UnknownPoll)?;
            if poll.closed || Syscall::block_height() as u64 > poll.input.closes_at {
                return Err(TinyPollError::PollClosed);
            }
            if state.voted.contains_key(&(poll_id, voter)) {
                return Err(TinyPollError::AlreadyVoted);
            }

            let counts = state
                .counts
                .get_mut(&poll_id)
                .ok_or(TinyPollError::UnknownPoll)?;
            let slot = counts
                .get_mut(option_index as usize)
                .ok_or(TinyPollError::InvalidOption)?;
            *slot += 1;
            state.voted.insert((poll_id, voter), option_index);
            state
                .evidence_by_vote
                .insert((poll_id, voter), evidence_hash);
        }

        Ok(())
    }

    #[export]
    pub fn close_poll(&mut self, poll_id: u64) -> Result<(), TinyPollError> {
        {
            let caller = Syscall::message_source();
            let mut state = self.state.get_mut();
            let poll = state
                .polls
                .get_mut(&poll_id)
                .ok_or(TinyPollError::UnknownPoll)?;
            if poll.creator != caller {
                return Err(TinyPollError::NotCreator);
            }
            poll.closed = true;
        }

        Ok(())
    }

    #[export]
    pub fn get_poll(&self, poll_id: u64) -> Option<Poll> {
        self.state.get().polls.get(&poll_id).cloned()
    }

    #[export]
    pub fn get_poll_result(&self, poll_id: u64) -> Option<PollResult> {
        let state = self.state.get();
        let poll = state.polls.get(&poll_id)?;
        Some(PollResult {
            poll_id,
            counts: state.counts.get(&poll_id).cloned().unwrap_or_default(),
            closed: poll.closed,
        })
    }
}

#[derive(Default)]
pub struct Program {
    state: RefCell<TinyPollState>,
}

#[sails_rs::program]
impl Program {
    pub fn create() -> Self {
        Self::default()
    }

    pub fn tiny_poll(&self) -> TinyPoll<&RefCell<TinyPollState>> {
        TinyPoll::new(&self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sails_rs::gstd::services::Service as _;

    fn valid_input() -> PollInput {
        PollInput {
            question_hash: [7u8; 32],
            option_labels: vec![
                "Keep allowlist".to_string(),
                "Open public submit".to_string(),
            ],
            closes_at: 100,
        }
    }

    #[test]
    fn creates_and_reads_poll() {
        let creator = ActorId::from(42);
        Syscall::with_message_source(creator);
        Syscall::with_block_height(1);

        let state = RefCell::new(TinyPollState::default());
        let mut service = TinyPoll::new(&state).expose(0);
        let poll_id = service.create_poll(valid_input()).unwrap();

        let poll = service.get_poll(poll_id).unwrap();
        assert_eq!(creator, poll.creator);
        assert_eq!(2, poll.input.option_labels.len());

        let result = service.get_poll_result(poll_id).unwrap();
        assert_eq!(vec![0, 0], result.counts);
        assert!(!result.closed);
    }

    #[test]
    fn enforces_one_vote_per_actor() {
        let voter = ActorId::from(7);
        Syscall::with_message_source(voter);
        Syscall::with_block_height(1);

        let state = RefCell::new(TinyPollState::default());
        let mut service = TinyPoll::new(&state).expose(0);
        let poll_id = service.create_poll(valid_input()).unwrap();

        assert_eq!(Ok(()), service.vote(poll_id, 0, [9u8; 32]));
        assert_eq!(
            Err(TinyPollError::AlreadyVoted),
            service.vote(poll_id, 1, [8u8; 32])
        );
    }

    #[test]
    fn validates_inputs() {
        Syscall::with_message_source(ActorId::from(1));
        Syscall::with_block_height(1);

        let state = RefCell::new(TinyPollState::default());
        let mut service = TinyPoll::new(&state).expose(0);

        let mut zero_question = valid_input();
        zero_question.question_hash = [0u8; 32];
        assert_eq!(
            Err(TinyPollError::ZeroQuestionHash),
            service.create_poll(zero_question)
        );

        let poll_id = service.create_poll(valid_input()).unwrap();
        assert_eq!(
            Err(TinyPollError::ZeroEvidenceHash),
            service.vote(poll_id, 0, [0u8; 32])
        );
        assert_eq!(
            Err(TinyPollError::InvalidOption),
            service.vote(poll_id, 3, [5u8; 32])
        );
    }
}
