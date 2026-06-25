# PricePulseVara Interface Sketch

```text
service Prices {
  GetPrice(symbol: String) -> Result<Price, Error>
  LastUpdated(symbol: String) -> Option<u64>
}

struct Price {
  symbol: String
  price_e8: u128
  timestamp_ms: u64
  source: String
}
```

## Missing Differentiation

This deliberately lacks:

- a unique source;
- proof model;
- update cadence;
- consumer commitment;
- latency/SLA promise.
