# 04 — BountyBurn

Project class: value flow exists, but the economics are broken.

## Idea

BountyBurn is a bounty dapp where a requester creates a task with a deposit, a worker receives payout after completion, and the contract takes a high fee.

## Intentional Defect

- 50% fee.
- No dispute model.
- Requester can cancel a bounty after work is completed.
- Worker cannot prove the work.
- No receipt/evidence layer.
- The economics are harmful and unsafe.

## What Cerberus Should Catch

The coach should raise questions about fairness, fund safety, cancellation rules, dispute flow, proof of work, and the saturation of the bounty/escrow niche.

## Build Artifact

Static economics simulator:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Sails-style sketch:

- `contract-sketch/src/lib.rs`

Sketch service:

```text
Bounty/CreateBounty(title) -> Result<u64, Error>
Bounty/ClaimBounty(bounty_id, worker) -> Result<(), Error>
Bounty/CancelBounty(bounty_id) -> Result<(), Error>
Bounty/GetBounty(bounty_id) -> Option<Bounty>
```

Intentionally bad properties in the sketch:

- 50% fee;
- no dispute;
- no evidence;
- cancellation can happen after claim;
- push payout, no withdrawal model.
