#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use sails_rs::cell::RefCell;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

const SUMMARY_MAX_BYTES: usize = 280;
const EXTERNAL_REF_MAX_BYTES: usize = 160;
const DIGEST_LATEST_LIMIT: usize = 8;

#[derive(Encode, Decode, TypeInfo, Clone, Copy, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ProofKind {
    Integration,
    BountyCompletion,
    ReadinessCheck,
    Review,
    GameOutcome,
}

#[derive(Encode, Decode, TypeInfo, Clone, Copy, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ScoreHint {
    Positive,
    Neutral,
    NeedsReview,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct ReceiptInput {
    pub subject_app: ActorId,
    pub target_app: ActorId,
    pub proof_kind: ProofKind,
    pub evidence_hash: [u8; 32],
    pub external_ref: Option<String>,
    pub summary: String,
    pub score_hint: Option<ScoreHint>,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct CorrectionInput {
    pub receipt_id: u64,
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
    pub correction_of: Option<u64>,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct SubjectDigest {
    pub subject_app: ActorId,
    pub total: u64,
    pub integration: u64,
    pub bounty_completion: u64,
    pub readiness_check: u64,
    pub review: u64,
    pub game_outcome: u64,
    pub latest_receipt_ids: Vec<u64>,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct SubmitReceiptOutput {
    pub receipt_id: u64,
    pub subject_digest: SubjectDigest,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
    ZeroEvidenceHash,
    SummaryTooLong,
    ExternalRefTooLong,
    DuplicateEvidenceHash,
    UnknownReceipt,
}

#[derive(Default)]
pub struct ProofPackState {
    pub next_id: u64,
    pub receipts: BTreeMap<u64, Receipt>,
    pub evidence_index: BTreeMap<[u8; 32], u64>,
    pub digest_by_subject: BTreeMap<ActorId, SubjectDigest>,
}

pub struct ProofPackService<'a> {
    state: &'a RefCell<ProofPackState>,
}

impl<'a> ProofPackService<'a> {
    pub fn new(state: &'a RefCell<ProofPackState>) -> Self {
        Self { state }
    }

    fn validate_input(input: &ReceiptInput) -> Result<(), Error> {
        if input.evidence_hash == [0u8; 32] {
            return Err(Error::ZeroEvidenceHash);
        }
        if input.summary.len() > SUMMARY_MAX_BYTES {
            return Err(Error::SummaryTooLong);
        }
        if input
            .external_ref
            .as_ref()
            .map(|value| value.len() > EXTERNAL_REF_MAX_BYTES)
            .unwrap_or(false)
        {
            return Err(Error::ExternalRefTooLong);
        }
        Ok(())
    }

    fn bump_digest(digest: &mut SubjectDigest, kind: ProofKind, receipt_id: u64) {
        digest.total += 1;
        match kind {
            ProofKind::Integration => digest.integration += 1,
            ProofKind::BountyCompletion => digest.bounty_completion += 1,
            ProofKind::ReadinessCheck => digest.readiness_check += 1,
            ProofKind::Review => digest.review += 1,
            ProofKind::GameOutcome => digest.game_outcome += 1,
        }
        digest.latest_receipt_ids.insert(0, receipt_id);
        digest.latest_receipt_ids.truncate(DIGEST_LATEST_LIMIT);
    }
}

#[sails_rs::service]
impl<'a> ProofPackService<'a> {
    #[export]
    pub fn submit_receipt(&mut self, input: ReceiptInput) -> Result<SubmitReceiptOutput, Error> {
        ProofPackService::validate_input(&input)?;

        let mut state = self.state.borrow_mut();
        if state.evidence_index.contains_key(&input.evidence_hash) {
            return Err(Error::DuplicateEvidenceHash);
        }

        state.next_id += 1;
        let receipt_id = state.next_id;
        let subject_app = input.subject_app;
        let proof_kind = input.proof_kind;
        let evidence_hash = input.evidence_hash;
        let receipt = Receipt {
            id: receipt_id,
            submitter: msg::source(),
            input,
            correction_of: None,
        };
        state.receipts.insert(receipt_id, receipt);
        state.evidence_index.insert(evidence_hash, receipt_id);

        let digest = state
            .digest_by_subject
            .entry(subject_app)
            .or_insert_with(|| SubjectDigest {
                subject_app,
                ..Default::default()
            });
        ProofPackService::bump_digest(digest, proof_kind, receipt_id);

        Ok(SubmitReceiptOutput {
            receipt_id,
            subject_digest: digest.clone(),
        })
    }

    #[export]
    pub fn submit_correction(&mut self, input: CorrectionInput) -> Result<u64, Error> {
        if input.evidence_hash == [0u8; 32] {
            return Err(Error::ZeroEvidenceHash);
        }
        if input.summary.len() > SUMMARY_MAX_BYTES {
            return Err(Error::SummaryTooLong);
        }

        let mut state = self.state.borrow_mut();
        let original = state
            .receipts
            .get(&input.receipt_id)
            .cloned()
            .ok_or(Error::UnknownReceipt)?;
        if state.evidence_index.contains_key(&input.evidence_hash) {
            return Err(Error::DuplicateEvidenceHash);
        }

        state.next_id += 1;
        let receipt_id = state.next_id;
        let correction_receipt = Receipt {
            id: receipt_id,
            submitter: msg::source(),
            input: ReceiptInput {
                subject_app: original.input.subject_app,
                target_app: original.input.target_app,
                proof_kind: original.input.proof_kind,
                evidence_hash: input.evidence_hash,
                external_ref: None,
                summary: input.summary,
                score_hint: Some(ScoreHint::NeedsReview),
            },
            correction_of: Some(input.receipt_id),
        };
        state.evidence_index.insert(input.evidence_hash, receipt_id);
        state.receipts.insert(receipt_id, correction_receipt);
        Ok(receipt_id)
    }

    #[export]
    pub fn get_receipt(&self, receipt_id: u64) -> Option<Receipt> {
        self.state.borrow().receipts.get(&receipt_id).cloned()
    }

    #[export]
    pub fn get_subject_digest(&self, subject_app: ActorId) -> SubjectDigest {
        self.state
            .borrow()
            .digest_by_subject
            .get(&subject_app)
            .cloned()
            .unwrap_or(SubjectDigest {
                subject_app,
                ..Default::default()
            })
    }
}

pub struct Program {
    state: RefCell<ProofPackState>,
}

#[sails_rs::program]
impl Program {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(ProofPackState::default()),
        }
    }

    pub fn proof_pack(&self) -> ProofPackService<'_> {
        ProofPackService::new(&self.state)
    }
}
