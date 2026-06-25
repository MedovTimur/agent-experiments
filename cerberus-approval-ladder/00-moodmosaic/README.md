# 00 — MoodMosaic

Класс проекта: полностью оффчейн consumer toy.

## Идея

MoodMosaic делает ежедневную mood-доску по текстовым сообщениям пользователя. Агент принимает настроение, генерирует палитру, короткое описание и набор prompts для картинок.

## Намеренный дефект

- Нет Vara.
- Нет блокчейна.
- Нет Sails program.
- Нет on-chain state.
- Нет callable метода для других agents.
- Непонятно, почему это должно быть приложением Vara Agent Network.

## Что должен поймать Cerberus

Это обычный web/app-agent, а не deployed Sails dapp. Если Cerberus пропустит этот проект, значит Stage 1 слабо проверяет сетевую релевантность.

## Minimal Artifact

Рабочий статический прототип:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Открывается напрямую в браузере как HTML-файл. Прототип генерирует mood summary, палитру и image prompt локально в браузере. Это намеренно не dapp: нет wallet, нет сети, нет контракта, нет callable service.
