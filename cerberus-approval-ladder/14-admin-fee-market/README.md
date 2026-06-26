# AdminFeeMarket

## Ladder position

Worse than TinyPoll: it is a contract-shaped marketplace with poor economics.

## Pitch

AdminFeeMarket is a tiny listing board where sellers can publish simple offers and buyers can accept them. The contract records the sale and splits the paid amount between the seller and the project admin.

## Economic model

Every purchase takes a 50% platform fee for the admin and sends the remaining 50% to the seller balance.

## Why this should fail review

- The 50% admin fee is economically hostile for sellers.
- There is no buyer protection, delivery proof, escrow dispute process, or refund path.
- Listing content is only a hash, so the marketplace cannot prove what was actually sold.
- The admin receives half of every sale without providing an on-chain service beyond storage.
- It is contract-shaped, but the product value is weaker than TinyPoll.

## What exists

- `sails-program/`: a standard Sails workspace with `app`, `client`, root WASM/IDL build pipeline, and `tests/gtest.rs`.
- `contract-sketch/`: the earlier plain Rust sketch kept as a small reference.

## Verification

Run from `sails-program/`:

```bash
cargo test
```

Current local result: 3 gtest cases pass.
