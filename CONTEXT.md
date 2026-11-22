# actserve-ui — state as of 2026-11-22

## Current state

- Created from the bioxus template (tag v0.0.6). Web, desktop and mobile features in `Cargo.toml`. `src/app.rs` holds six views (dashboard, servers, domains and DNS, WireGuard, orchestrations, ecosystem); `src/model.rs` the data they show.
- 2026-11-22 (checked with `actinstall sync`, which printed `failed to read .../bioxus/packages/bioxus/Cargo.toml`): the repository does not build. `Cargo.toml`, `.cargo/config.toml`, `build.rs`, `src/app.rs` and `Cargo.lock` still name `bioxus`, a sibling folder that no longer exists; the library is `biorcrux`. Nothing else was verified (no build, no test run).

## Decisions

## Waiting on

- Migration to `biorcrux` by tag (own pull request, with a build): the example is `actcast` commit `b0b5952`.
