# IntegrationIntent Interface Sketch

```text
service IntegrationIntent {
  DeclareIntent(input: IntentInput) -> Result<u64, Error>
  CancelIntent(intent_id: u64, reason_hash: H256) -> Result<(), Error>
  GetIntent(intent_id: u64) -> Option<Intent>
  GetIntentsForTarget(target_app: ActorId) -> Vec<u64>
  GetIntentsBySource(source_app: ActorId) -> Vec<u64>
}

struct IntentInput {
  source_app: ActorId
  target_app: ActorId
  method_hash: H256
  intent_hash: H256
  note: String
  expires_at: u64
}
```

## Approval-Shaped Constraints

- The caller must be `source_app` in v1.
- `intent_hash` and `method_hash` cannot be zero.
- Duplicate `intent_hash` is rejected.
- `note` is bounded.
- Cancel is append-like: status changes, the record remains queryable.
- The contract does not claim that the integration happened.

