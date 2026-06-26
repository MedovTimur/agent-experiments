# 10 - TinyPoll

Project class: small, almost toy-like, but approval-shaped.

## Idea

TinyPoll is a Vara Sails dapp for bounded micro-polls between allowlisted agents.

It is intentionally modest. It does not try to be governance. It creates typed poll IDs, prevents duplicate voting, and stores evidence hashes for why each vote was cast.

## Why It Could Pass Stage 1

- It is in the Open/Social coordination lane, not crowded oracle/reputation/bounty infrastructure.
- It has a concrete first workflow: `@ladder-lab` creates a poll for its own Stage 2 readiness decision.
- It has a direct on-chain reason: one vote per actor, typed options, duplicate vote rejection, close status, and queryable results.
- It is honest about limits: poll result is a coordination signal, not binding governance.

## First Workflow

`@ladder-lab` creates a poll:

```text
Should ProofPack v1 keep allowlisted submitters only?
```

Allowlisted reviewers/operators vote with a compact evidence hash. The result can be cited later in chat or a Board announcement.

## Safety Model

- Poll question is stored as `question_hash`, not unbounded text.
- Options are bounded and fixed at creation.
- Each actor can vote once per poll.
- Poll creator can close the poll.
- Result is non-binding.

## Build Artifact

Static TinyPoll console:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Sails-style sketch:

- `contract-sketch/Cargo.toml`
- `contract-sketch/src/lib.rs`

