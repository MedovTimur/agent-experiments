# 09 - IntegrationIntent

Project class: deliberately tiny but approval-shaped.

## Idea

IntegrationIntent is a Vara Sails dapp where agents declare planned integrations before the integration exists.

It is intentionally small: a typed ledger of commitments. The useful wedge is that Cerberus repeatedly asks for a committed consumer. IntegrationIntent gives builders a callable place to publish and query that commitment instead of relying on free-form chat.

## Why It Could Pass Stage 1

- It solves a real recurring review blocker: "who will call this?"
- It is not an oracle, escrow, bounty board, or reputation app.
- It has a clear on-chain primitive: typed intent records with duplicate prevention and cancellation.
- It can be dogfooded by `@ladder-lab` before any external consumer commits.
- It does not claim that an intent is proof of completed work.

## First Workflow

`@ladder-lab` declares an intent to integrate with ProofPack after Stage 2 code exists:

```text
DeclareIntent(source_app, target_app, method_hash, intent_hash, expires_at)
```

Later, a reviewer or another app calls:

```text
GetIntentsForTarget(target_app)
```

to see who publicly committed to call the target app.

## Safety Model

- Only the caller records intents for itself.
- `intent_hash` cannot be zero.
- Duplicate `intent_hash` is rejected.
- Method labels and notes are bounded.
- Intents can be cancelled, but not deleted.
- Expired/cancelled intents are still queryable.

## Build Artifact

Static IntegrationIntent console:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Sails-style sketch:

- `contract-sketch/Cargo.toml`
- `contract-sketch/src/lib.rs`

