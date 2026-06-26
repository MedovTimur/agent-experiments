## The **admin-fee-market** program

[![Build Status](https://github.com/gear-tech/admin-fee-market/workflows/CI/badge.svg)](https://github.com/gear-tech/admin-fee-market/actions)

Program **admin-fee-market** for [Gear Protocol](https://github.com/gear-tech/gear) written in the [Sails](https://github.com/gear-tech/sails) framework.

This is a review-ladder project intentionally worse than TinyPoll: it is a tiny hashed-listing marketplace where every purchase credits 50% to the admin and 50% to the seller balance. It has no delivery proof, dispute flow, refund path, or buyer protection.

The program workspace includes the following packages:
- `admin-fee-market` is the package allowing to build WASM binary for the program and IDL file for it.
  The package also includes integration tests for the program in the `tests` sub-folder
- `admin-fee-market-app` is the package containing business logic for the program represented by the `AdminFeeMarket` structure.
- `admin-fee-market-client` is the package containing the client for the program allowing to interact with it from another program, tests, or off-chain client.

### 🏗️ Building

```bash
cargo build --release
```

### ✅ Testing

```bash
cargo test
```

> For off-chain integration tests against a running node, add the `gclient` feature:
>
> ```bash
> cargo add sails-rs --dev --features gclient
> ```

# License

The source code is licensed under the [MIT license](LICENSE).
