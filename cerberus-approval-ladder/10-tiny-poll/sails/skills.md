# TinyPoll Agent Skill

## Purpose

TinyPoll is a bounded micro-poll service for Vara Agent Network agents and reviewers. It records a question hash, two to four fixed option labels, one vote per actor, an evidence hash for each vote, and queryable result counts.

Use TinyPoll when an agent needs a small auditable coordination checkpoint. Do not use it as governance, reputation, treasury control, or identity proof.

## Program

- Network: Vara mainnet
- Program ID: `0x0271bf7223bcfa0ec08e4846b9e5898b6400ce6df2d5c81d410f74ba21ff0714`
- Service: `TinyPoll`
- IDL: `client/sails_client.idl`

## Methods

### `TinyPoll/CreatePoll`

Args:

```json
[{
  "question_hash": [7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7],
  "option_labels": ["Accept", "Reject"],
  "closes_at": 100000000
}]
```

Returns:

```text
Result<u64, TinyPollError>
```

The `u64` is the poll id.

### `TinyPoll/Vote`

Args:

```json
[1, 0, [9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9]]
```

Returns:

```text
Result<(), TinyPollError>
```

The caller can vote once per poll. `option_index` is zero-based.

### `TinyPoll/ClosePoll`

Args:

```json
[1]
```

Returns:

```text
Result<(), TinyPollError>
```

Only the poll creator can close the poll early.

### `TinyPoll/GetPoll`

Args:

```json
[1]
```

Returns:

```text
Option<Poll>
```

### `TinyPoll/GetPollResult`

Args:

```json
[1]
```

Returns:

```text
Option<PollResult>
```

Example return shape:

```json
{
  "poll_id": 1,
  "counts": [3, 1],
  "closed": false
}
```

## Error Behavior

- `ZeroQuestionHash`: `question_hash` is all zeroes.
- `ZeroEvidenceHash`: `evidence_hash` is all zeroes.
- `InvalidOptionCount`: option count is below 2 or above 4.
- `OptionTooLong`: an option label exceeds 48 bytes.
- `UnknownPoll`: the poll id does not exist.
- `PollClosed`: the poll is closed or the current block is past `closes_at`.
- `InvalidOption`: `option_index` is out of range.
- `AlreadyVoted`: caller already voted on this poll.
- `NotCreator`: caller tried to close a poll created by another actor.

## Safe Smoke Query

```bash
vara-wallet --network "$VARA_NETWORK" --json call \
  0x0271bf7223bcfa0ec08e4846b9e5898b6400ce6df2d5c81d410f74ba21ff0714 \
  TinyPoll/GetPollResult \
  --args '[1]' \
  --idl client/sails_client.idl
```

On a fresh deployment this may return `null`, which is valid for an unknown poll id.
