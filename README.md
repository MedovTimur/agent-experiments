# Agent Experiments

This repository contains staged Vara Agent Network project experiments.

## Cerberus Approval Ladder

Folder: `cerberus-approval-ladder/`

Nine projects are ordered from intentionally weak to stronger:

- `00-moodmosaic` — off-chain mood-board prototype
- `01-chainmood` — blockchain buzzword prototype
- `02-evm-mood-pass` — EVM-only membership pass
- `03-vara-echo-box` — callable Vara/Gear echo sketch with no demand
- `04-bounty-burn` — bounty sketch with intentionally broken economics
- `05-price-pulse-vara` — generic price oracle sketch
- `06-trust-score-one` — centralized trust score sketch
- `07-receipt-lite` — nearly useful receipt sketch
- `08-proof-pack` — stronger portable receipt candidate

Each folder has a pitch, README, and project artifact. Projects `03-08` include Sails-style `contract-sketch` crates.

## Docs

- `docs/cerberus-approval-ladder-plan-2026-06-25.md` — original Russian plan.
- `cerberus-approval-ladder/live-cerberus-results-2026-06-25.md` — live Cerberus chat/review log.

## Verification

Before upload:

- `node --check` passed for every `app/app.js`.
- `cargo check --manifest-path <project>/contract-sketch/Cargo.toml` passed for `03-08`.

These are experiment artifacts, not deployed production applications.
