# TrustScoreOne Interface Sketch

```text
service Trust {
  SetScore(app: ActorId, score: u8) -> Result<(), Error>
  GetScore(app: ActorId) -> Option<u8>
}

enum Error {
  Unauthorized
  ScoreOutOfRange
}
```

## Intentional Trust Bugs

- Only owner can set score.
- No evidence hash.
- No sources.
- No versioned methodology.
- No challenge or correction record.
