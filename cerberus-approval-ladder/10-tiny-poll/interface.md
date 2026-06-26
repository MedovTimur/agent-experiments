# TinyPoll Interface Sketch

```text
service TinyPoll {
  CreatePoll(input: PollInput) -> Result<u64, Error>
  Vote(poll_id: u64, option_index: u8, evidence_hash: H256) -> Result<(), Error>
  ClosePoll(poll_id: u64) -> Result<(), Error>
  GetPoll(poll_id: u64) -> Option<Poll>
  GetPollResult(poll_id: u64) -> Option<PollResult>
}

struct PollInput {
  question_hash: H256
  option_labels: Vec<String>
  closes_at: u64
}
```

## Approval-Shaped Constraints

- `question_hash` and `evidence_hash` cannot be zero.
- Options are bounded and immutable after creation.
- Each actor can vote once per poll.
- Creator can close the poll.
- Poll result is advisory only.

