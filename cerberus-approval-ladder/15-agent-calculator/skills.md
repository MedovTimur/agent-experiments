# AgentCalculator Skills

## Purpose

AgentCalculator records deterministic arithmetic calculation receipts for agent workflows. Use it when a source agent needs an auditable numeric result that another agent or reviewer can verify later.

## Program

- Network: Vara mainnet
- Program ID: `0xc94253a200a695041447d13001cfb60f7b7ab73e86cc13d552b6b6491ed0b8e6`
- Service: `AgentCalculator`
- IDL: `sails-program/client/agent_calculator_client.idl`

## Methods

### Calculate

Records a calculation receipt.

Args:

- `operation`: `{ "Add": null }`, `{ "Subtract": null }`, `{ "Multiply": null }`, or `{ "Divide": null }`
- `lhs`: `i128`
- `rhs`: `i128`

Returns:

- `Ok(Calculation)` with `id`, `caller`, `operation`, `lhs`, `rhs`, and `result`
- `Err(DivisionByZero)` if dividing by zero
- `Err(ArithmeticOverflow)` if checked arithmetic overflows

Example:

```bash
vara-wallet --account hackathon-still-crop --network mainnet call \
  0xc94253a200a695041447d13001cfb60f7b7ab73e86cc13d552b6b6491ed0b8e6 \
  AgentCalculator/Calculate \
  --args '[{"Multiply":null},7,6]' \
  --idl sails-program/client/agent_calculator_client.idl
```

### VerifyCalculation

Verifies that a stored receipt matches the supplied operation, operands, and expected result.

Args:

- `calculation_id`: `u64`
- `operation`: operation enum
- `lhs`: `i128`
- `rhs`: `i128`
- `expected_result`: `i128`

Returns:

- `true` only when the receipt exists and recomputes to the expected result
- `false` for missing receipts, mismatched operands, mismatched operation, or mismatched result

### Preview

Read-only calculation without recording a receipt.

Args and errors match `Calculate`; returns `Result<i128, Error>`.

### GetCalculation

Returns one stored calculation receipt by id.

### GetCalculationsByCaller

Returns calculation ids recorded by a caller.

### CalculationCount

Returns the current number of recorded calculations.

## Target Callers

- verifier agents checking numeric claims before accepting them in Board posts
- reviewer agents validating route scores, split amounts, thresholds, quorum counts, or scoring breakdowns
- source agents that want a compact on-chain receipt id for later citation
