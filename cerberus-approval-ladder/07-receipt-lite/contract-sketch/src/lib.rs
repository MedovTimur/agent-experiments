#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use sails_rs::cell::RefCell;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

const SUMMARY_MAX_BYTES: usize = 240;

#[derive(Encode, Decode, TypeInfo, Clone, Copy, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ProofKind {
    Integration,
    BountyCompletion,
    ReadinessCheck,
    Review,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct ReceiptInput {
    pub subject_app: ActorId,
    pub target_app: ActorId,
    pub proof_kind: ProofKind,
    pub evidence_hash: [u8; 32],
    pub summary: String,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Receipt {
    pub id: u64,
    pub submitter: ActorId,
    pub input: ReceiptInput,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
    ZeroEvidenceHash,
    SummaryTooLong,
    DuplicateEvidenceHash,
    UnknownReceipt,
}

#[derive(Default)]
pub struct ReceiptState {
    pub next_id: u64,
    pub receipts: BTreeMap<u64, Receipt>,
    pub evidence_index: BTreeMap<[u8; 32], u64>,
}

pub struct ReceiptsService<'a> {
    state: &'a RefCell<ReceiptState>,
}

impl<'a> ReceiptsService<'a> {
    pub fn new(state: &'a RefCell<ReceiptState>) -> Self {
        Self { state }
    }
}

#[sails_rs::service]
impl<'a> ReceiptsService<'a> {
    #[export]
    pub fn submit_receipt(&mut self, input: ReceiptInput) -> Result<u64, Error> {
        if input.evidence_hash == [0u8; 32] {
            return Err(Error::ZeroEvidenceHash);
        }
        if input.summary.len() > SUMMARY_MAX_BYTES {
            return Err(Error::SummaryTooLong);
        }

        let mut state = self.state.borrow_mut();
        if state.evidence_index.contains_key(&input.evidence_hash) {
            return Err(Error::DuplicateEvidenceHash);
        }

        state.next_id += 1;
        let id = state.next_id;
        let evidence_hash = input.evidence_hash;
        let receipt = Receipt {
            id,
            submitter: msg::source(),
            input,
        };
        state.receipts.insert(id, receipt);
        state.evidence_index.insert(evidence_hash, id);
        Ok(id)
    }

    #[export]
    pub fn get_receipt(&self, receipt_id: u64) -> Option<Receipt> {
        self.state.borrow().receipts.get(&receipt_id).cloned()
    }
}

pub struct Program {
    state: RefCell<ReceiptState>,
}

#[sails_rs::program]
impl Program {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(ReceiptState::default()),
        }
    }

    pub fn receipts(&self) -> ReceiptsService<'_> {
        ReceiptsService::new(&self.state)
    }
}
