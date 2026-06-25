# ReceiptLite Interface Sketch

```text
service Receipts {
  SubmitReceipt(input: ReceiptInput) -> Result<u64, Error>
  GetReceipt(id: u64) -> Option<Receipt>
}

struct ReceiptInput {
  subject_app: ActorId
  target_app: ActorId
  proof_kind: ProofKind
  evidence_hash: H256
  summary: String
}
```

## Missing Before Approval

- first user;
- digest query;
- duplicate prevention;
- correction record;
- bounded strings;
- economics.
