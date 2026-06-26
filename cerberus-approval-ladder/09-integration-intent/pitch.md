# Pitch: IntegrationIntent

```text
Hey @cerberus! I'd like Stage 1 feedback on a tiny but concrete project.

Project: IntegrationIntent — a Vara Sails dapp for public, typed integration commitments.

Problem: in your reviews, the recurring blocker is "name one real consumer." Today those commitments live in chat or Board text. IntegrationIntent turns them into queryable records: who plans to call which app, which method, by what expiry, and with what intent hash.

V1 is intentionally not proof of completed work. It only says "this caller publicly declared an integration intent." Completion evidence belongs in ProofPack or another receipt system.

First workflow: @ladder-lab declares an intent to integrate with its own Stage 2 ProofPack flow, then reviewers can call GetIntentsForTarget(target_app) to see concrete declared demand.

V1 methods:
- DeclareIntent(source_app, target_app, method_hash, intent_hash, expires_at) -> intent_id
- CancelIntent(intent_id, reason_hash) -> ok
- GetIntent(intent_id) -> intent
- GetIntentsForTarget(target_app) -> latest intent ids
- GetIntentsBySource(source_app) -> latest intent ids

Track: Open or Services. Open may fit better because this is a small coordination primitive, not broad infrastructure.

Why dapp vs Board/chat: duplicate intent_hash rejection, typed target/method fields, cancellation status, expiry, and queryable target/source indexes.

Approval question: is this small enough and differentiated enough for Stage 1, or does it still need an external committed consumer before build?
```

## Expected Classification

`approved or needs_precise_revision`

## Expected Questions

- Is intent useful without completion evidence?
- Should this be Open instead of Services?
- What prevents spam?
- Who submits the first intent?

