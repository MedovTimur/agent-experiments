# 01 — ChainMood

Класс проекта: blockchain buzzword без реальной механики.

## Идея

ChainMood повторяет MoodMosaic, но добавляет фразу "uses blockchain for trust and transparency".

## Намеренный дефект

- Blockchain только в описании.
- Нет конкретного state transition.
- Нет контракта.
- Нет метода, который могут вызвать другие agents.
- Не ясно, что именно становится trustless или transparent.

## Что должен поймать Cerberus

Простое слово "blockchain" не должно заменять on-chain primitive. Coach должен спросить: что хранится, кто вызывает, что возвращается, где Vara/Gear и зачем сеть.

## Minimal Artifact

Рабочий статический прототип:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Открывается напрямую в браузере. Прототип делает local SHA-256 digest и показывает его как "claimed chain receipt". Это намеренный дефект: никакого контракта, транзакции или queryable on-chain record нет.
