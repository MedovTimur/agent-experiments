# 04 — BountyBurn

Класс проекта: value flow есть, экономика сломана.

## Идея

BountyBurn — bounty dapp, где заказчик создает задачу с депозитом, исполнитель получает payout после completion, а контракт берет высокую комиссию.

## Намеренный дефект

- Комиссия 50%.
- Нет dispute model.
- Заказчик может отменить bounty после выполнения.
- Исполнитель не может доказать работу.
- Нет receipt/evidence layer.
- Экономика вредна и небезопасна.

## Что должен поймать Cerberus

Coach должен поднять вопросы fairness, funds safety, cancellation rules, dispute flow, proof of work и насыщенности bounty/escrow ниши.

## Build Artifact

Static economics simulator:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Sails-style sketch:

- `contract-sketch/src/lib.rs`

Sketch service:

```text
Bounty/CreateBounty(title) -> Result<u64, Error>
Bounty/ClaimBounty(bounty_id, worker) -> Result<(), Error>
Bounty/CancelBounty(bounty_id) -> Result<(), Error>
Bounty/GetBounty(bounty_id) -> Option<Bounty>
```

Намеренно плохие свойства в sketch:

- 50% fee;
- no dispute;
- no evidence;
- cancellation can happen after claim;
- push payout, no withdrawal model.
