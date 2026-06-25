# Cerberus Approval Ladder

Дата: 2026-06-25

Эта папка содержит последовательные проекты для эксперимента с @cerberus: от заведомо слабых заявок до сильного Vara/Gear Sails dapp. Каждый проект лежит отдельно и содержит pitch, проверяемый дефект и ожидаемую реакцию.

Главный план: `../../reports/cerberus-approval-ladder-plan-2026-06-25.md`

## Порядок

| Step | Folder | Project | Purpose |
|---|---|---|---|
| 00 | `00-moodmosaic` | MoodMosaic | Полностью оффчейн consumer toy |
| 01 | `01-chainmood` | ChainMood | Blockchain как buzzword без механики |
| 02 | `02-evm-mood-pass` | EvmMoodPass | On-chain есть, но не Vara/Gear Application |
| 03 | `03-vara-echo-box` | VaraEchoBox | Sails есть, спроса нет |
| 04 | `04-bounty-burn` | BountyBurn | Sails/economy есть, экономика сломана |
| 05 | `05-price-pulse-vara` | PricePulseVara | Полезно, но crowded oracle без отличия |
| 06 | `06-trust-score-one` | TrustScoreOne | Trust/reputation с плохой evidence model |
| 07 | `07-receipt-lite` | ReceiptLite | Почти хорошо, но нет first user/economics |
| 08 | `08-proof-pack` | ProofPack | Сильный кандидат на Stage 1 approval |

## Использование

1. Идем по папкам по порядку.
2. Берем `pitch.md`, отправляем Cerberus.
3. Записываем ответ в `result.md` по шаблону из плана.
4. Переходим к следующей ступени, исправляя ровно тот класс провала, который должен был всплыть.

## Build Status

- `00-02`: static browser prototypes. `02` additionally has an EVM-only Solidity contract and ABI.
- `03-08`: static browser prototypes plus Sails-style `contract-sketch` crates.
- Verification run:
  - `node --check` passed for every `app/app.js`.
  - `cargo check --manifest-path <project>/contract-sketch/Cargo.toml` passed for `03-08`.

These are not deployed programs yet. The contract sketches are intentionally lightweight Stage 1 / Stage 2a artifacts; the final approved candidate should be moved into a proper Sails workspace with generated IDL, gtests, and deploy scripts.

## Принцип

Ранние проекты намеренно недостроены. Это не технический долг, а часть эксперимента: мы проверяем, отсекает ли coach отсутствие Vara, отсутствие callable метода, отсутствие спроса, сломанную экономику, crowded нишу и некорректную trust model.
