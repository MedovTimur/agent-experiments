# TinyPoll Sails Program

TinyPoll is a Social-track Vara Sails dapp for bounded micro-polls between agents and reviewers.

It is not governance and not reputation. V1 stores typed poll IDs, question hashes, fixed options, one vote per actor, an evidence hash per vote, close status, and queryable results.

## Service

Service: `TinyPoll`

Methods:

- `CreatePoll(input) -> Result<u64, TinyPollError>`
- `Vote(poll_id, option_index, evidence_hash) -> Result<(), TinyPollError>`
- `ClosePoll(poll_id) -> Result<(), TinyPollError>`
- `GetPoll(poll_id) -> Option<Poll>`
- `GetPollResult(poll_id) -> Option<PollResult>`

## Trust Model

- `question_hash` and `evidence_hash` must be nonzero.
- Options are fixed at creation and bounded to 2-4 labels.
- Each actor can vote once per poll.
- The poll creator can close the poll.
- Evidence is verified off-chain by the poll creator/reviewers; TinyPoll preserves the audit trail and does not claim on-chain truth.
- Poll results are advisory coordination inputs, not binding governance.

## Toolchain

This workspace uses Sails `1.0.0` and Rust `1.93.0`.

## Testing

```bash
cargo test
```

Current gtest coverage:

- poll lifecycle: create, read metadata, read result, vote, duplicate vote rejection, close;
- named validation errors: zero question hash, invalid option count, zero evidence hash, invalid option.

## IDL

Generated IDL is committed at:

```text
client/sails_client.idl
```
