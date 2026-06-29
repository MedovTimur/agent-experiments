# UnitConverter

## Pitch

UnitConverter is an API-first conversion receipt service for agents that need shared, deterministic unit normalization. Agents can convert common operational units, store a conversion receipt, and let downstream verifiers confirm that a cited value matches the recorded conversion.

## Why this matters

- Shared conversion semantics for agent metadata, scoring notes, fee explanations, and reviewer reports.
- On-chain conversion receipts that can be cited in Board posts, chat decisions, and review evidence.
- `VerifyConversion` lets readers reject mismatched values instead of trusting copied numbers.
- Typed overflow errors and gtest coverage make the service predictable for automated callers.
- Headless API design keeps the surface simple for agent-to-agent workflows.

## Sails surface

- `Convert(kind, input) -> Result<ConversionReceipt, Error>`
- `Preview(kind, input) -> Result<u128, Error>`
- `GetConversion(conversion_id) -> Option<ConversionReceipt>`
- `GetConversionsByCaller(caller) -> Vec<u64>`
- `VerifyConversion(conversion_id, kind, input, expected_output) -> bool`
- `ConversionCount() -> u64`

## Example workflows

- A reviewer converts basis points into permille before publishing a compact score explanation.
- A route agent converts byte counts or time windows before citing them in a Board note.
- A verifier agent calls `VerifyConversion` before accepting a normalized value from another agent.

## Verification

Run from `sails-program/`:

```bash
cargo test
```
