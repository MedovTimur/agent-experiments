# 08 — ProofPack

Project class: strong Stage 1 approval candidate.

## Idea

ProofPack is a Vara Sails dapp for portable integration receipts. It stores compact evidence envelopes for completed agent-to-agent actions and gives other services a query-friendly reference to the result.

## Why It Should Do Better

- It is a deployed Sails dapp, not just a bot.
- It has a clear on-chain primitive.
- It has callable methods.
- It does not replace oracle, escrow, or reputation systems.
- It works as a receipt layer for bounty, readiness, review, dashboard, and game/arena workflows.
- Claims are limited: a receipt is an operator-attested evidence envelope, not absolute truth.

## First User Hypothesis

First workflow: a reviewer/readiness service or bounty app stores a receipt after checking a concrete integration. Another agent later calls `GetSubjectDigest(subject_app)` and sees a compact evidence history.

## Economics Hypothesis

V1 can be free or use a small fee per receipt. Later value capture: premium digest, sponsored verification campaign, paid high-volume receipt submission, or integration with bounty/escrow apps.

## Correction Policy

Receipts are append-only. Mistakes are corrected through a new correction receipt that references the original receipt id. History is not mutated.

## Build Artifact

Static ProofPack console:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Sails-style sketch:

- `contract-sketch/Cargo.toml`
- `contract-sketch/src/lib.rs`

Sketch service:

```text
ProofPack/SubmitReceipt(input) -> Result<SubmitReceiptOutput, Error>
ProofPack/SubmitCorrection(input) -> Result<u64, Error>
ProofPack/GetReceipt(receipt_id) -> Option<Receipt>
ProofPack/GetSubjectDigest(subject_app) -> SubjectDigest
```

Safety model in sketch:

- evidence hash cannot be zero;
- duplicate evidence hash is rejected;
- summary and external_ref are bounded;
- corrections are append-only;
- digest stores counts/latest ids, not a trust score.
