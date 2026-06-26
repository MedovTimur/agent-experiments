# Cerberus Approval Ladder

Date: 2026-06-25

This folder contains a sequence of project experiments for @cerberus review: from intentionally weak submissions to a stronger Vara/Gear Sails dapp candidate. Each project lives in its own folder and includes a pitch, the intended review failure mode, and the expected coach reaction.

Main plan: `../docs/cerberus-approval-ladder-plan-2026-06-25.md`

## Order

| Step | Folder | Project | Purpose |
|---|---|---|---|
| 00 | `00-moodmosaic` | MoodMosaic | Fully off-chain consumer toy |
| 01 | `01-chainmood` | ChainMood | Blockchain as a buzzword without mechanics |
| 02 | `02-evm-mood-pass` | EvmMoodPass | On-chain, but not a Vara/Gear Application |
| 03 | `03-vara-echo-box` | VaraEchoBox | Sails exists, demand does not |
| 04 | `04-bounty-burn` | BountyBurn | Sails/economy exists, economics are broken |
| 05 | `05-price-pulse-vara` | PricePulseVara | Useful, but a crowded oracle with no edge |
| 06 | `06-trust-score-one` | TrustScoreOne | Trust/reputation with a weak evidence model |
| 07 | `07-receipt-lite` | ReceiptLite | Almost useful, but missing first user/economics |
| 08 | `08-proof-pack` | ProofPack | Stronger Stage 1 approval candidate |
| 09 | `09-integration-intent` | IntegrationIntent | Tiny commitment ledger for declared consumers |
| 10 | `10-tiny-poll` | TinyPoll | Tiny bounded poll primitive for agent decisions |

## Usage

1. Move through folders in order.
2. Send the folder's `pitch.md` to Cerberus.
3. Record the response in `result.md` using `result-template.md`.
4. Move to the next step, fixing exactly the failure class surfaced by the previous response.

## Build Status

- `00-02`: static browser prototypes. `02` additionally has an EVM-only Solidity contract and ABI.
- `03-10`: static browser prototypes plus Sails-style `contract-sketch` crates.
- Verification run:
  - `node --check` passed for every `app/app.js`.
  - `cargo check --manifest-path <project>/contract-sketch/Cargo.toml` passed for `03-08`.

These are not deployed programs yet. The contract sketches are intentionally lightweight Stage 1 / Stage 2a artifacts; the final approved candidate should be moved into a proper Sails workspace with generated IDL, gtests, and deploy scripts.

## Principle

The early projects are intentionally underbuilt. That is not accidental technical debt; it is the experiment. We are testing whether the coach catches missing Vara relevance, missing callable methods, missing demand, broken economics, crowded niches, and incorrect trust models.
