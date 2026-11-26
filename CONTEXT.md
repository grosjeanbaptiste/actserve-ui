# actserve-ui — state as of 2026-11-26

## Current state

- Created from the bioxus template (tag v0.0.6). Web, desktop and mobile features in `Cargo.toml`. `src/app.rs` holds six views (dashboard, servers, domains and DNS, WireGuard, orchestrations, ecosystem); `src/model.rs` the data they show.
- 2026-11-26: migrated to `biorcrux` v0.0.10 (git tag) and Dioxus `main`, as `actcast` b0b5952 did. `AppShell` now needs `nav_label`. `.cargo/config.toml` is removed: its `[patch]` pointed to the missing `../bioxus` and broke dependency resolution.
- Checked on a `git archive` export (the worktree sits under the checkout, so cargo read the checkout's `.cargo/config.toml`): `cargo test --all-targets` 7 passed; `cargo clippy --all-targets --all-features -- -D warnings` and `cargo fmt --check` clean.
- Demo data and test names still say "bioxus" (`bioxus.cloud` domains, `renders_the_actserve_bioxus_shell`): sample strings, not the library.

## Decisions

## Waiting on

- `cargo install` through `actinstall sync` after the merge (not yet run on this change).
