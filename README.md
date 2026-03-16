# I build a simple rust state machine, following the tutorial: "https://www.shawntabrizi.com/rust-state-machine/"

## Overview

This project is a simple blockchain-style state machine written in Rust.
It follows the tutorial from Shawn Tabrizi and includes a minimal runtime with:

- `system` pallet: block numbers and account nonces
- `balances` pallet: transfers between accounts
- `proof_of_existence` pallet: create and revoke ownership claims

## Project Structure

- `src/main.rs`: runtime wiring and block execution examples
- `src/system.rs`: system-level state
- `src/balances.rs`: balances and transfer logic
- `src/proof_of_existence.rs`: claim and revoke logic
- `src/support.rs`: shared types (`Block`, `Header`, `Extrinsic`, `Dispatch`)

## Requirements

- Rust (stable)
- Cargo

## Run

```bash
cargo run
```

## Test

Run all tests:

```bash
cargo test
```

Run tests by module:

```bash
cargo test --lib system::test:: -- --nocapture
cargo test --lib balances::tests:: -- --nocapture
cargo test --lib proof_of_existence::test:: -- --nocapture
```

## Notes

- You can intentionally add invalid extrinsics in `main.rs` to demonstrate runtime error handling.
- The tutorial uses `BTreeMap` instead of `HashMap` to keep state deterministic (sorted keys) and trait bounds simple (`Ord`).
- The `macros` crate comes from the tutorial; I did not implement those macros myself.
- I added `src/lib.rs` so module tests can be run as library tests (for example, `cargo test --lib system::test::`).
- We could add transaction fees (gas) by deducting a small amount from the caller for each extrinsic execution (and other blockchain features), but this tutorial does not implement fees (Nobody likes fees).

- Why can `cargo run` show `Extrinsic Error` but still finish successfully?
  In this project, an extrinsic failure is logged, but block execution continues.
  In `src/main.rs:62`, each dispatch result is captured as `_res` and errors are only printed via `map_err(...)`.
  The block still returns `Ok(())` after processing extrinsics (unless the block number check fails in `src/main.rs:54`).
  This is because we do not want to panic and stop the blockchain because one extrinsic fails