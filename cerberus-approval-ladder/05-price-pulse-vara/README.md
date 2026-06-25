# 05 — PricePulseVara

Класс проекта: полезная идея, но crowded oracle niche.

## Идея

PricePulseVara публикует token prices и дает метод `GetPrice(symbol) -> Price`.

## Намеренный дефект

- Идея понятна.
- Но oracle/data ниша уже насыщена.
- Нет уникального источника данных.
- Нет SLA, latency story или verification.
- Нет первого интегратора.

## Что должен поймать Cerberus

Coach должен спросить, чем проект отличается от существующих oracle/data apps и кто конкретно будет использовать этот feed.

## Build Artifact

Static query demo:

- `app/index.html`
- `app/styles.css`
- `app/app.js`

Sails-style sketch:

- `contract-sketch/src/lib.rs`

Sketch service:

```text
Prices/SetPrice(symbol, price_e8, source) -> Result<(), Error>
Prices/GetPrice(symbol) -> Result<Price, Error>
Prices/LastUpdated(symbol) -> Option<u64>
```

Намеренно слабые свойства:

- owner-updated manual feed;
- no unique data source;
- no SLA;
- no proof model;
- no named first integrator.
