# 08 — ProofPack

Класс проекта: сильный кандидат на Stage 1 approval.

## Идея

ProofPack — Vara Sails dapp для portable integration receipts. Он хранит компактные evidence envelopes о выполненных agent-to-agent действиях и дает другим сервисам query-friendly ссылку на результат.

## Почему это должно пройти лучше

- Это deployed Sails dapp, а не бот.
- Есть понятный on-chain primitive.
- Есть callable methods.
- Не заменяет oracle, escrow или reputation.
- Работает как receipt layer для bounty, readiness, review, dashboard и game/arena workflows.
- Claims ограничены: receipt is operator-attested evidence, not absolute truth.

## First User Hypothesis

Первый workflow: reviewer/readiness service или bounty app сохраняет receipt после проверки конкретной интеграции. Другой agent позже вызывает `GetSubjectDigest(subject_app)` и видит компактную историю evidence.

## Economics Hypothesis

V1 может быть бесплатным или иметь маленький fee per receipt. Value capture позже: premium digest, sponsored verification campaign, paid high-volume receipt submission или интеграция с bounty/escrow apps.

## Correction Policy

Receipts append-only. Ошибки исправляются через новый correction receipt, который ссылается на исходный receipt id. История не мутируется.

## Build Artifact

Static ProofPack console:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Sails-style sketch:

- `contract-sketch/Cargo.toml`
- `contract-sketch/src/lib.rs`

Sketch service:

```text
ProofPack/SubmitReceipt(input) -> Result<SubmitReceiptOutput, Error>
ProofPack/SubmitCorrection(input) -> Result<u64, Error>
ProofPack/GetReceipt(receipt_id) -> Option<Receipt>
ProofPack/GetSubjectDigest(subject_app) -> SubjectDigest
```

Safety model in sketch:

- evidence hash cannot be zero;
- duplicate evidence hash is rejected;
- summary and external_ref are bounded;
- corrections are append-only;
- digest stores counts/latest ids, not a trust score.
