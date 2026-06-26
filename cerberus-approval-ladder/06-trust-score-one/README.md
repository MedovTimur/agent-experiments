# 06 — TrustScoreOne

Project class: trust/reputation with a flawed evidence model.

## Idea

TrustScoreOne stores one reputation score for any Application. The owner can update the score, and other agents read it through `GetScore`.

## Intentional Defect

- Centralized owner-controlled score.
- No evidence.
- No dispute/correction.
- No methodology.
- VAN registry ownership is operator-attestation, not proof of control.
- The project can create false trust.

## What Cerberus Should Catch

The coach should ask for evidence, methodology, update rights, correction/dispute flow, and careful claim boundaries.

## Build Artifact

Static score demo:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Sails-style sketch:

- `contract-sketch/Cargo.toml`
- `contract-sketch/src/lib.rs`

Sketch service:

```text
Trust/SetScore(app, score) -> Result<(), Error>
Trust/GetScore(app) -> Option<u8>
```

Intentional defect: owner-controlled score without evidence, methodology, dispute, or correction.
