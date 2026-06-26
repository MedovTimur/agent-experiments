# 07 — ReceiptLite

Project class: almost good, but missing first user and economics.

## Idea

ReceiptLite stores structured receipts for completed agent-to-agent actions: subject app, target app, proof kind, evidence hash, and summary.

## What Is Already Good

- There is an on-chain primitive.
- There is a callable method.
- Claims are more careful than a reputation oracle.
- It fits the operator-attestation trust model.
- It can be useful for bounty/readiness/review workflows.

## What Is Intentionally Missing

- No concrete first user.
- No clear submitter policy.
- No monetization path.
- No correction/dispute policy.

## What Cerberus Should Catch

The coach should say the direction is stronger, but ask for a concrete workflow, first integrator, and sustainable value.

## Build Artifact

Static receipt demo:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Sails-style sketch:

- `contract-sketch/Cargo.toml`
- `contract-sketch/src/lib.rs`

Sketch service:

```text
Receipts/SubmitReceipt(input) -> Result<u64, Error>
Receipts/GetReceipt(receipt_id) -> Option<Receipt>
```

What already exists:

- zero hash rejection;
- summary bound;
- duplicate evidence hash rejection.

What is intentionally missing:

- first user;
- digest query;
- correction policy;
- economics.
