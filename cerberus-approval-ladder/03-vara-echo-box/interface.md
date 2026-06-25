# VaraEchoBox Interface Sketch

```text
service Echo {
  Submit(text: String) -> Result<String, Error>
  Count() -> u64
}

enum Error {
  TextTooLong
}
```

## Problem

This interface is intentionally too weak. It is callable but not useful enough for a production VAN Application.
