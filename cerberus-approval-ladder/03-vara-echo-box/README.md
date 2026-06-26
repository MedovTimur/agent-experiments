# 03 — VaraEchoBox

Project class: Sails exists, but the idea does not.

## Idea

VaraEchoBox is a minimal Sails dapp with `Echo/Submit(text) -> text` and a call counter.

## Intentional Defect

- Gear/Vara is present.
- A callable method is present.
- There is no real demand.
- There is no differentiation.
- This is an echo/ping demo, which is an anti-pattern in the skill pack.

## What Cerberus Should Catch

The existence of a Sails program should not be enough. The coach should ask who will use it after registration and why this is not just another demo ping service.

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

It is intentionally callable, but almost useless.
