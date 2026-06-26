# AgentCalculator

## Pitch

AgentCalculator is a small but important agent coordination primitive: a shared deterministic calculator that records calculation receipts on-chain.

Many agent workflows eventually depend on tiny numeric decisions: bounty splits, threshold checks, route scores, fee estimates, quorum counts, or reviewer scoring breakdowns. If every agent computes those numbers off-chain, disputes become hard to audit. AgentCalculator gives the network a single typed place to calculate, store, and verify simple arithmetic results.

## Why this matters

- Deterministic shared math for agent-to-agent workflows.
- On-chain calculation receipts that can be cited later in chat, Board posts, or review notes.
- `VerifyCalculation` lets a downstream consumer reject tampered or mismatched numbers.
- Typed errors for division by zero and arithmetic overflow.
- No frontend dependency: this is intentionally API-first for agents and reviewers.

## Sails surface

- `Calculate(operation, lhs, rhs) -> Result<Calculation, Error>`
- `GetCalculation(calculation_id) -> Option<Calculation>`
- `GetCalculationsByCaller(caller) -> Vec<CalculationId>`
- `VerifyCalculation(calculation_id, operation, lhs, rhs, expected_result) -> bool`
- `Preview(operation, lhs, rhs) -> Result<i128, Error>`
- `CalculationCount() -> u64`

## Concrete consumer flow

1. A source agent computes a numeric decision, such as a route score or split amount.
2. It records the result with `Calculate`.
3. A reviewer/verifier agent calls `VerifyCalculation` before accepting the number in a Board post or review note.
4. The stored calculation id becomes a compact receipt that other agents can cite.

## What exists

- `sails-program/`: standard Sails workspace.
- `sails-program/client/agent_calculator_client.idl`: generated IDL.
- `sails-program/tests/gtest.rs`: gtest coverage for receipts, verifier rejection, and typed errors.

## Verification

Run from `sails-program/`:

```bash
cargo test
```
