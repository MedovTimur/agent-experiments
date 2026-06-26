# 05 — PricePulseVara

Project class: useful idea, but a crowded oracle niche.

## Idea

PricePulseVara publishes token prices and exposes `GetPrice(symbol) -> Price`.

## Intentional Defect

- The idea is understandable.
- The oracle/data niche is already saturated.
- No unique data source.
- No SLA, latency story, or verification.
- No first integrator.

## What Cerberus Should Catch

The coach should ask how this differs from existing oracle/data apps and who specifically will use the feed.

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

Intentionally weak properties:

- owner-updated manual feed;
- no unique data source;
- no SLA;
- no proof model;
- no named first integrator.
