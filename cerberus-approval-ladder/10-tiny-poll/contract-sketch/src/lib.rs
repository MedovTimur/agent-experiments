#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use sails_rs::cell::RefCell;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

const MAX_OPTIONS: usize = 4;
const MAX_OPTION_BYTES: usize = 48;

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PollInput {
    pub question_hash: [u8; 32],
    pub option_labels: Vec<String>,
    pub closes_at: u64,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Poll {
    pub id: u64,
    pub creator: ActorId,
    pub input: PollInput,
    pub closed: bool,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PollResult {
    pub poll_id: u64,
    pub counts: Vec<u64>,
    pub closed: bool,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
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

#[derive(Default)]
pub struct TinyPollState {
    pub next_id: u64,
    pub polls: BTreeMap<u64, Poll>,
    pub counts: BTreeMap<u64, Vec<u64>>,
    pub voted: BTreeMap<(u64, ActorId), u8>,
    pub evidence_by_vote: BTreeMap<(u64, ActorId), [u8; 32]>,
}

pub struct TinyPollService<'a> {
    state: &'a RefCell<TinyPollState>,
}

impl<'a> TinyPollService<'a> {
    pub fn new(state: &'a RefCell<TinyPollState>) -> Self {
        Self { state }
    }
}

#[sails_rs::service]
impl<'a> TinyPollService<'a> {
    #[export]
    pub fn create_poll(&mut self, input: PollInput) -> Result<u64, Error> {
        if input.question_hash == [0u8; 32] {
            return Err(Error::ZeroQuestionHash);
        }
        if input.option_labels.len() < 2 || input.option_labels.len() > MAX_OPTIONS {
            return Err(Error::InvalidOptionCount);
        }
        if input
            .option_labels
            .iter()
            .any(|label| label.len() > MAX_OPTION_BYTES)
        {
            return Err(Error::OptionTooLong);
        }

        let mut state = self.state.borrow_mut();
        state.next_id += 1;
        let poll_id = state.next_id;
        let option_count = input.option_labels.len();
        state.polls.insert(
            poll_id,
            Poll {
                id: poll_id,
                creator: msg::source(),
                input,
                closed: false,
            },
        );
        state.counts.insert(poll_id, alloc::vec![0; option_count]);
        Ok(poll_id)
    }

    #[export]
    pub fn vote(
        &mut self,
        poll_id: u64,
        option_index: u8,
        evidence_hash: [u8; 32],
    ) -> Result<(), Error> {
        if evidence_hash == [0u8; 32] {
            return Err(Error::ZeroEvidenceHash);
        }

        let voter = msg::source();
        let mut state = self.state.borrow_mut();
        let poll = state.polls.get(&poll_id).ok_or(Error::UnknownPoll)?;
        if poll.closed {
            return Err(Error::PollClosed);
        }
        if state.voted.contains_key(&(poll_id, voter)) {
            return Err(Error::AlreadyVoted);
        }

        let counts = state.counts.get_mut(&poll_id).ok_or(Error::UnknownPoll)?;
        let slot = counts
            .get_mut(option_index as usize)
            .ok_or(Error::InvalidOption)?;
        *slot += 1;
        state.voted.insert((poll_id, voter), option_index);
        state
            .evidence_by_vote
            .insert((poll_id, voter), evidence_hash);
        Ok(())
    }

    #[export]
    pub fn close_poll(&mut self, poll_id: u64) -> Result<(), Error> {
        let caller = msg::source();
        let mut state = self.state.borrow_mut();
        let poll = state.polls.get_mut(&poll_id).ok_or(Error::UnknownPoll)?;
        if poll.creator != caller {
            return Err(Error::NotCreator);
        }
        poll.closed = true;
        Ok(())
    }

    #[export]
    pub fn get_poll(&self, poll_id: u64) -> Option<Poll> {
        self.state.borrow().polls.get(&poll_id).cloned()
    }

    #[export]
    pub fn get_poll_result(&self, poll_id: u64) -> Option<PollResult> {
        let state = self.state.borrow();
        let poll = state.polls.get(&poll_id)?;
        Some(PollResult {
            poll_id,
            counts: state.counts.get(&poll_id).cloned().unwrap_or_default(),
            closed: poll.closed,
        })
    }
}

pub struct Program {
    state: RefCell<TinyPollState>,
}

#[sails_rs::program]
impl Program {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(TinyPollState::default()),
        }
    }

    pub fn tiny_poll(&self) -> TinyPollService<'_> {
        TinyPollService::new(&self.state)
    }
}

