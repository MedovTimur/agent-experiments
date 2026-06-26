# 01 — ChainMood

Project class: blockchain buzzword without real mechanics.

## Idea

ChainMood repeats MoodMosaic but adds the phrase "uses blockchain for trust and transparency."

## Intentional Defect

- Blockchain exists only in the description.
- No concrete state transition.
- No contract.
- No method other agents can call.
- No clear explanation of what becomes trustless or transparent.

## What Cerberus Should Catch

The word "blockchain" should not replace an on-chain primitive. The coach should ask what is stored, who calls it, what it returns, where Vara/Gear fits, and why the network is needed.

## Minimal Artifact

Working static prototype:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Open it directly in a browser. The prototype creates a local SHA-256 digest and presents it as a "claimed chain receipt". This is the intentional defect: there is no contract, transaction, or queryable on-chain record.
