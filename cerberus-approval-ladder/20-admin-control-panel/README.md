# AdminControlPanel

## Pitch

AdminControlPanel is a governed claim verification panel for agents that need operator-reviewed numeric claims. Users submit a `subject_hash` and a claim value. The appointed admin can publish corrections when a submitted value needs moderation or operational review, and downstream agents can call `VerifyClaim` against the current reviewed state.

## Why this matters

- Claims are stored with submitter, subject hash, value, and correction status.
- Admin correction creates an explicit `ClaimOverridden` event for auditability.
- `VerifyClaim` gives downstream agents a simple boolean check against the reviewed state.
- Typed errors distinguish unauthorized correction, missing claims, empty subject hashes, and arithmetic overflow.
- The service is useful for reviewer-maintained scoreboards, curated evidence registries, and operator-governed coordination flows.

## Sails surface

- `SubmitClaim(subject_hash, value) -> Result<Claim, Error>`
- `AdminOverrideClaim(claim_id, new_value) -> Result<Claim, Error>`
- `GetClaim(claim_id) -> Option<Claim>`
- `VerifyClaim(claim_id, subject_hash, value) -> bool`
- `ClaimCount() -> u64`
- `Admin() -> ActorId`

## Example workflows

- A reviewer maintains curated numeric claims for a project readiness report.
- A source agent submits a claim, then a reviewer corrects it after checking evidence.
- A verifier agent reads `GetClaim` and calls `VerifyClaim` before using the reviewed value.

## Verification

Run from `sails-program/`:

```bash
cargo test
```
