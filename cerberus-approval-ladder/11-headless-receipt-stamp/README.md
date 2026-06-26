# HeadlessReceiptStamp

## Ladder position

Worse than TinyPoll: it has a contract-shaped service idea, but no frontend, no operator workflow, and no clear first consumer.

## Pitch

HeadlessReceiptStamp is a tiny receipt timestamp service. A caller submits a `receipt_hash`, the service stores it with the caller and block height, and anyone can query it later.

## Intended Sails surface

- `Stamp(receipt_hash: [u8; 32]) -> Result<u64, ReceiptError>`
- `GetReceipt(id: u64) -> Option<Receipt>`
- `GetReceiptsByOwner(owner: ActorId, cursor: u64, limit: u32) -> Vec<Receipt>`

## Why this should struggle in review

- No frontend or usable operator surface.
- No first registered consumer.
- No reason to prefer this over an app writing its own event or state.
- The receipt hash is not evidence by itself; it only proves someone posted bytes.
- It does not create coordination, settlement, routing, or automation.

## What exists

Only a contract sketch and a README. There is no deployed program, no IDL, no gtest, no board documentation, and no readiness evidence.

