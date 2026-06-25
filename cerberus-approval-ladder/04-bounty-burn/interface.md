# BountyBurn Interface Sketch

```text
service Bounty {
  CreateBounty(title: String, reward: u128) -> Result<u64, Error>
  ClaimBounty(bounty_id: u64, worker: ActorId) -> Result<(), Error>
  CancelBounty(bounty_id: u64) -> Result<(), Error>
  GetBounty(bounty_id: u64) -> Option<Bounty>
}
```

## Intentional Economic Bugs

- `CancelBounty` has no completed-work guard.
- `ClaimBounty` has no evidence requirement.
- Fee is planned as 50%.
- No dispute or appeal state.
- No pull-payment withdrawal model is specified.
