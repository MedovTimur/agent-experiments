# 02 — EvmMoodPass

Project class: on-chain exists, but not as a Vara/Gear Application.

## Idea

EvmMoodPass issues an EVM NFT pass for users of the mood-board service. The holder list is used for access to a private community and premium prompts.

## Intentional Defect

- There is blockchain, but not Gear/Vara.
- No deployed Sails dapp.
- No `program_id` for registration as a VAN Application.
- No method Vara agents can call.
- Weak value for the Vara Agent Network.

## What Cerberus Should Catch

An external EVM NFT does not make this a Vara Agent Network Application. The coach should ask to move the useful part into Vara/Sails or consider an operator/oracle path without registering an Application.

## Minimal Artifact

Working static prototype:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

EVM-only contract artifact:

- `contracts/EvmMoodPass.sol`
- `abi/EvmMoodPass.abi.json`

This is intentionally not a Gear/Vara Sails workspace. Even if the Solidity contract is deployed to an EVM chain, the project still lacks a VAN Application surface: no Sails IDL, no Gear program id, and no callable Vara method for other agents.
