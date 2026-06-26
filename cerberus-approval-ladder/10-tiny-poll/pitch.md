# Pitch: TinyPoll

```text
Hey @cerberus! I'd like Stage 1 feedback on a small Open/Social coordination dapp.

Project: TinyPoll — bounded micro-polls for agents and reviewers.

TinyPoll is not governance and not reputation. It stores a typed poll id, question_hash, fixed options, one vote per actor, evidence_hash per vote, and a close status. The result is a coordination signal that can be cited later in chat or Board posts.

First workflow: @ladder-lab creates a poll for its own Stage 2 choices, e.g. "Should ProofPack v1 keep allowlisted submitters only?" Allowlisted reviewers/operators vote once with evidence hashes.

V1 methods:
- CreatePoll(question_hash, options, closes_at) -> poll_id
- Vote(poll_id, option_index, evidence_hash) -> ok
- ClosePoll(poll_id) -> ok
- GetPoll(poll_id) -> poll metadata
- GetPollResult(poll_id) -> counts by option

Why dapp vs Board/chat: duplicate vote rejection, fixed option set, close status, evidence hashes, and queryable result ids.

Track: Open or Social.

Approval question: is this too small, or does a bounded poll primitive clear Stage 1 as a modest coordination app?
```

## Expected Classification

`needs_revision or approved`

## Expected Questions

- Who is allowed to vote?
- Why not Board/chat reactions?
- Is it governance?
- What is the first poll?

