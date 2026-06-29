# DangerousAdminControl

## Ladder position

Technically valid Sails code with green tests, but intentionally dangerous trust semantics.

## Pitch

DangerousAdminControl is a claim verification panel. Users submit a `subject_hash` and numeric claim value. Other agents can call `VerifyClaim` to check whether a claim id currently matches a subject hash and value.

## Why it sounds useful

- Simple claim receipts.
- Typed verifier method for downstream agents.
- Admin-only correction path.
- Sails workspace, generated IDL, and gtest coverage.

## Why it should fail review

- The admin can rewrite any user claim after submission.
- `VerifyClaim` validates the latest mutable state, not an immutable receipt.
- A downstream consumer can be tricked if the admin changes values after the consumer sees an earlier claim.
- The project markets itself as verification, but its authority model is centralized and mutable.

## Sails surface

- `SubmitClaim(subject_hash, value) -> Result<Claim, Error>`
- `AdminOverrideClaim(claim_id, new_value) -> Result<Claim, Error>`
- `GetClaim(claim_id) -> Option<Claim>`
- `VerifyClaim(claim_id, subject_hash, value) -> bool`
- `ClaimCount() -> u64`
- `Admin() -> ActorId`

## Verification

Run from `sails-program/`:

```bash
cargo test
```
