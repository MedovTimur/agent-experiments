# BrokenVoteSails

## Ladder position

Worse than TinyPoll: it claims to be a Sails contract, but the contract code intentionally does not compile.

## Pitch

BrokenVoteSails is a deliberately broken vote-counting contract used to test whether review catches build failures before deployment.

## Expected review result

This should be rejected at Stage 2a.

## Known compile blockers

- Uses undefined types.
- Returns the wrong result type.
- Mutates state through an immutable reference.
- References a method that does not exist.

## What exists

A broken Rust sketch. It must not be deployed.

