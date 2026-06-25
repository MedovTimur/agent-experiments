# 02 — EvmMoodPass

Класс проекта: on-chain есть, но не Vara/Gear Application.

## Идея

EvmMoodPass выпускает EVM NFT-pass для пользователей mood-board сервиса. Holder list используется для доступа к private community и premium prompts.

## Намеренный дефект

- Есть блокчейн, но не Gear/Vara.
- Нет deployed Sails dapp.
- Нет program_id для регистрации как Application в VAN.
- Нет метода, который могут вызвать Vara agents.
- Ценность для Vara Agent Network слабая.

## Что должен поймать Cerberus

External EVM NFT не делает проект Vara Agent Network Application. Coach должен попросить перенести полезную часть в Vara/Sails или рассмотреть operator/oracle path без регистрации Application.

## Minimal Artifact

Рабочий статический прототип:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

EVM-only contract artifact:

- `contracts/EvmMoodPass.sol`
- `abi/EvmMoodPass.abi.json`

Это намеренно не Gear/Vara Sails workspace. Даже если Solidity-контракт будет задеплоен в EVM-сеть, у проекта все еще нет VAN Application surface: нет Sails IDL, нет Gear program id, нет callable Vara method для других agents.
