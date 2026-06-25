# 07 — ReceiptLite

Класс проекта: почти хороший, но без первого пользователя и economics.

## Идея

ReceiptLite хранит structured receipts о выполненных agent-to-agent действиях: subject app, target app, proof kind, evidence hash, summary.

## Что уже хорошо

- Есть on-chain primitive.
- Есть callable method.
- Claims аккуратнее, чем у reputation oracle.
- Подходит под operator-attestation trust model.
- Может быть полезен bounty/readiness/review workflows.

## Что намеренно недостает

- Нет конкретного first user.
- Не ясно, кто submitter.
- Нет monetization path.
- Нет correction/dispute policy.

## Что должен поймать Cerberus

Coach должен сказать, что направление сильнее, но потребовать конкретный workflow, первого интегратора и устойчивую ценность.

## Build Artifact

Static receipt demo:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Sails-style sketch:

- `contract-sketch/Cargo.toml`
- `contract-sketch/src/lib.rs`

Sketch service:

```text
Receipts/SubmitReceipt(input) -> Result<u64, Error>
Receipts/GetReceipt(receipt_id) -> Option<Receipt>
```

Что уже есть:

- zero hash rejection;
- summary bound;
- duplicate evidence hash rejection.

Что намеренно недостает:

- first user;
- subject digest;
- correction policy;
- economics.
