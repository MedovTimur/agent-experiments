# ProofPack Interface Sketch

```text
service ProofPack {
  SubmitReceipt(input: ReceiptInput) -> Result<SubmitReceiptOutput, Error>
  SubmitCorrection(input: CorrectionInput) -> Result<u64, Error>
  GetReceipt(receipt_id: u64) -> Option<Receipt>
  GetSubjectDigest(subject_app: ActorId) -> SubjectDigest
}

struct ReceiptInput {
  subject_app: ActorId
  target_app: ActorId
  proof_kind: ProofKind
  evidence_hash: H256
  external_ref: Option<String>
  summary: String
  score_hint: Option<ScoreHint>
}

struct SubmitReceiptOutput {
  receipt_id: u64
  subject_digest: SubjectDigest
}

enum ProofKind {
  Integration
  BountyCompletion
  ReadinessCheck
  Review
  GameOutcome
}

enum ScoreHint {
  Positive
  Neutral
  NeedsReview
}

enum Error {
  InvalidActorId
  ZeroEvidenceHash
  SummaryTooLong
  ExternalRefTooLong
  UnsupportedProofKind
  DuplicateEvidenceHash
  UnknownReceipt
  UnauthorizedCorrection
}
```

## Safety Model

- Receipt is an evidence envelope, not a truth oracle.
- Evidence hash cannot be zero.
- Duplicate evidence hash is rejected.
- Summaries and external refs are bounded.
- Corrections are append-only.
- Digest is descriptive counts/latest ids, not a trust score.
