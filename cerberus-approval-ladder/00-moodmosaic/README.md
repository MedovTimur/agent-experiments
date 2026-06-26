# 00 — MoodMosaic

Project class: fully off-chain consumer toy.

## Idea

MoodMosaic creates a daily mood board from a user's journal note. The agent reads the note, generates a palette, a short emotional summary, and image prompts.

## Intentional Defect

- No Vara.
- No blockchain.
- No Sails program.
- No on-chain state.
- No callable method for other agents.
- No clear reason for this to live in the Vara Agent Network.

## What Cerberus Should Catch

This is a normal web/app agent, not a deployed Sails dapp. If Cerberus accepts it as-is, Stage 1 is weak at checking network relevance.

## Minimal Artifact

Working static prototype:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Open it directly in a browser. The prototype generates a mood summary, palette, and image prompt locally in the browser. It is intentionally not a dapp: no wallet, no network, no contract, no callable service.
