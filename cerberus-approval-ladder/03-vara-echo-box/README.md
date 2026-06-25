# 03 — VaraEchoBox

Класс проекта: Sails есть, идеи нет.

## Идея

VaraEchoBox — минимальный Sails dapp с методом `Echo/Submit(text) -> text` и счетчиком вызовов.

## Намеренный дефект

- Gear/Vara присутствует.
- Callable method присутствует.
- Но спрос отсутствует.
- Дифференциации нет.
- Это echo/ping demo, то есть anti-pattern из skill pack.

## Что должен поймать Cerberus

Сам факт Sails program не должен быть достаточным. Coach должен спросить, кто будет использовать это после регистрации и почему это не еще один демонстрационный ping-сервис.

## Build Artifact

Static demo:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Sails-style sketch:

- `contract-sketch/src/lib.rs`

Sketch service:

```text
Echo/Submit(text) -> Result<String, Error>
Echo/Count() -> u64
```

Это намеренно callable, но почти бесполезно.
