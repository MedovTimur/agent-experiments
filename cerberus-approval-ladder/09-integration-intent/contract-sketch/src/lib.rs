#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use sails_rs::cell::RefCell;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

const NOTE_MAX_BYTES: usize = 180;
const INDEX_LIMIT: usize = 16;

#[derive(Encode, Decode, TypeInfo, Clone, Copy, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum IntentStatus {
    Active,
    Cancelled,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IntentInput {
    pub source_app: ActorId,
    pub target_app: ActorId,
    pub method_hash: [u8; 32],
    pub intent_hash: [u8; 32],
    pub note: String,
    pub expires_at: u64,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Intent {
    pub id: u64,
    pub submitter: ActorId,
    pub input: IntentInput,
    pub status: IntentStatus,
    pub cancel_reason_hash: Option<[u8; 32]>,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
    SourceMustBeCaller,
    ZeroMethodHash,
    ZeroIntentHash,
    NoteTooLong,
    DuplicateIntentHash,
    UnknownIntent,
    NotIntentSubmitter,
    AlreadyCancelled,
}

#[derive(Default)]
pub struct IntegrationIntentState {
    pub next_id: u64,
    pub intents: BTreeMap<u64, Intent>,
    pub intent_hash_index: BTreeMap<[u8; 32], u64>,
    pub by_target: BTreeMap<ActorId, Vec<u64>>,
    pub by_source: BTreeMap<ActorId, Vec<u64>>,
}

pub struct IntegrationIntentService<'a> {
    state: &'a RefCell<IntegrationIntentState>,
}

impl<'a> IntegrationIntentService<'a> {
    pub fn new(state: &'a RefCell<IntegrationIntentState>) -> Self {
        Self { state }
    }

    fn push_index(index: &mut BTreeMap<ActorId, Vec<u64>>, key: ActorId, intent_id: u64) {
        let ids = index.entry(key).or_default();
        ids.insert(0, intent_id);
        ids.truncate(INDEX_LIMIT);
    }
}

#[sails_rs::service]
impl<'a> IntegrationIntentService<'a> {
    #[export]
    pub fn declare_intent(&mut self, input: IntentInput) -> Result<u64, Error> {
        let caller = msg::source();
        if input.source_app != caller {
            return Err(Error::SourceMustBeCaller);
        }
        if input.method_hash == [0u8; 32] {
            return Err(Error::ZeroMethodHash);
        }
        if input.intent_hash == [0u8; 32] {
            return Err(Error::ZeroIntentHash);
        }
        if input.note.len() > NOTE_MAX_BYTES {
            return Err(Error::NoteTooLong);
        }

        let mut state = self.state.borrow_mut();
        if state.intent_hash_index.contains_key(&input.intent_hash) {
            return Err(Error::DuplicateIntentHash);
        }

        state.next_id += 1;
        let intent_id = state.next_id;
        let source_app = input.source_app;
        let target_app = input.target_app;
        let intent_hash = input.intent_hash;
        let intent = Intent {
            id: intent_id,
            submitter: caller,
            input,
            status: IntentStatus::Active,
            cancel_reason_hash: None,
        };

        state.intents.insert(intent_id, intent);
        state.intent_hash_index.insert(intent_hash, intent_id);
        IntegrationIntentService::push_index(&mut state.by_source, source_app, intent_id);
        IntegrationIntentService::push_index(&mut state.by_target, target_app, intent_id);
        Ok(intent_id)
    }

    #[export]
    pub fn cancel_intent(
        &mut self,
        intent_id: u64,
        reason_hash: [u8; 32],
    ) -> Result<(), Error> {
        let caller = msg::source();
        let mut state = self.state.borrow_mut();
        let intent = state
            .intents
            .get_mut(&intent_id)
            .ok_or(Error::UnknownIntent)?;
        if intent.submitter != caller {
            return Err(Error::NotIntentSubmitter);
        }
        if intent.status == IntentStatus::Cancelled {
            return Err(Error::AlreadyCancelled);
        }
        intent.status = IntentStatus::Cancelled;
        intent.cancel_reason_hash = Some(reason_hash);
        Ok(())
    }

    #[export]
    pub fn get_intent(&self, intent_id: u64) -> Option<Intent> {
        self.state.borrow().intents.get(&intent_id).cloned()
    }

    #[export]
    pub fn get_intents_for_target(&self, target_app: ActorId) -> Vec<u64> {
        self.state
            .borrow()
            .by_target
            .get(&target_app)
            .cloned()
            .unwrap_or_default()
    }

    #[export]
    pub fn get_intents_by_source(&self, source_app: ActorId) -> Vec<u64> {
        self.state
            .borrow()
            .by_source
            .get(&source_app)
            .cloned()
            .unwrap_or_default()
    }
}

pub struct Program {
    state: RefCell<IntegrationIntentState>,
}

#[sails_rs::program]
impl Program {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(IntegrationIntentState::default()),
        }
    }

    pub fn integration_intent(&self) -> IntegrationIntentService<'_> {
        IntegrationIntentService::new(&self.state)
    }
}

