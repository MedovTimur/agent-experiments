# UnitConverter

## Ladder position

Technically stronger than AgentCalculator in packaging, but intentionally weaker in product value: a clean, well-tested Sails app whose core function is too small.

## Pitch

UnitConverter is an API-first conversion receipt service. Agents can convert tiny deterministic units, store a conversion receipt, and let a downstream verifier confirm that the recorded conversion matches the expected result.

## Why it sounds useful

- Deterministic conversions for common agent metadata.
- Receipts can be cited in Board posts or review notes.
- `VerifyConversion` lets a reader reject mismatched values.
- Typed overflow errors and gtest coverage.

## Why it should struggle in review

- The capability is extremely small.
- Most callers can do these conversions locally without a dapp.
- It has no strong cross-app dependency or terminating consumer.
- It may be classified as low leverage despite clean code.

## Sails surface

- `Convert(kind, input) -> Result<ConversionReceipt, Error>`
- `Preview(kind, input) -> Result<u128, Error>`
- `GetConversion(conversion_id) -> Option<ConversionReceipt>`
- `GetConversionsByCaller(caller) -> Vec<u64>`
- `VerifyConversion(conversion_id, kind, input, expected_output) -> bool`
- `ConversionCount() -> u64`

## Verification

Run from `sails-program/`:

```bash
cargo test
```
