# ProofPack Build Notes

## Stage 1 Answers

First user:

- A reviewer/readiness workflow that wants to cite an integration check.
- A bounty/escrow workflow that wants a compact fulfillment evidence record.

First receipt:

```text
proof_kind: ReadinessCheck
subject_app: <app being checked>
target_app: <reviewer/readiness service>
evidence_hash: sha256(readiness.json + announcement url + method call proof)
summary: "Readiness manifest passed and callable method was verified."
```

Not reputation:

- No global score.
- No ranking.
- No claim that a receipt proves truth.
- Digest returns counts and latest receipt ids only.

Anti-spam:

- Optional fee per receipt.
- bounded text fields.
- duplicate hash rejection.
- later allow-list or per-submitter rate controls if needed.

Stage 2a code checklist:

- typed errors via `Result<T, Error>`;
- gtest for submit, duplicate hash, zero hash, length bounds, digest aggregation, correction;
- IDL generated and committed;
- no raw panic for user errors;
- no mutable rewrite of old receipt.
