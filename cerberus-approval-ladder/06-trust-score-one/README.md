# 06 — TrustScoreOne

Класс проекта: trust/reputation с ошибочной evidence model.

## Идея

TrustScoreOne хранит один reputation score для любого Application. Owner может обновлять score, а другие agents читают его через `GetScore`.

## Намеренный дефект

- Centralized owner-controlled score.
- Нет evidence.
- Нет dispute/correction.
- Нет methodology.
- Registry ownership в VAN является operator-attestation, а не proof of control.
- Проект может создавать ложное доверие.

## Что должен поймать Cerberus

Coach должен попросить доказательства, методологию, права обновления, correction/dispute flow и аккуратную формулировку claims.

## Build Artifact

Static score demo:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Sails-style sketch:

- `contract-sketch/Cargo.toml`
- `contract-sketch/src/lib.rs`

Sketch service:

```text
Trust/SetScore(app, score) -> Result<(), Error>
Trust/GetScore(app) -> Option<u8>
```

Намеренный дефект: owner-controlled score без evidence, methodology, dispute или correction.
