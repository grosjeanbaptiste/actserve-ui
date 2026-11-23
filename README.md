# actserve-ui

Administration interface of [actserve](../actserve): servers, domains and DNS, WireGuard, orchestrations and workflows, ecosystem presets. A Dioxus app, one crate for web, desktop, tablet and phone.

> **Status (2026-11-22): does not build.** `Cargo.toml` depends on `bioxus` through the path `../bioxus/packages/bioxus`, which does not exist: the library is now [`biorcrux`](https://github.com/grosjeanbaptiste/biorcrux), consumed by git tag. `cargo install` fails with `failed to read .../bioxus/packages/bioxus/Cargo.toml`. The migration (dependency, `bioxus::` imports and stylesheet names, Dioxus on `main`) follows what `actcast` did in commit `b0b5952`; until it is merged, read `bioxus` below as `biorcrux`.

| Run | Command |
|---|---|
| Web | `make web` |
| Desktop | `make desktop` |
| iOS / Android | `dx serve --platform ios` / `android` (`--no-default-features --features mobile`) |
| All gates | `make ci-local` |

The layout follows the screen: bottom bar on phones, rail on tablets, sidebar on desktops (`bioxus::widgets::AppShell`). Pick blocks with the features `store`, `store-sqlite`, `sync`; nothing is imposed.

Icon: `assets/icon.png` is the BIOXUS icon. Replace it with your own and every platform follows (`build.rs` checks it is a valid PNG). For iOS, build the set with bioxus `scripts/make-ios-set.sh assets/icon.png stores/ios/AppIcon.appiconset`.

## Contents

- [How it works](#how-it-works)

## How it works

```mermaid
flowchart LR
    ui["actserve-ui (Dioxus: web, desktop, tablet, phone)"] --> api["actserve API"]
    ui --> bx["biorcrux (see status: path dependency to fix)"]
```
