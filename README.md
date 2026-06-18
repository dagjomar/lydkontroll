# Lydkontroll

Tauri 2 application for reliable local wedding audio playback, with a
React/TypeScript interface and a Rust-owned application core.

The current implementation includes managed cue persistence/import,
deterministic local playback, desktop editing, reconnecting iPhone control over
Tailscale, and event preflight diagnostics.

## Prerequisites

- Apple Silicon macOS with Xcode command-line tools and the Tauri prerequisites
- Node.js 20.17 or newer
- Rust 1.85 or newer
- npm

Cargo is configured to prefer the newest dependency versions compatible with
the repository's declared Rust version.

## Install and run

```bash
npm install
npm run tauri dev
```

`npm run tauri dev` starts Vite on `127.0.0.1:1420`, compiles the Rust crate,
and opens the native macOS window.

## Validation

```bash
npm run build
npm test
npm run lint
cargo test --manifest-path src-tauri/Cargo.toml
```

`npm run lint` runs ESLint, Prettier verification, generated-contract drift
checks, `cargo fmt --check`, and Clippy with warnings denied.

## Generated contracts

Rust types are the source of truth for shared contracts. `ts-rs` writes
committed TypeScript files to `src/generated/`, as configured in
`.cargo/config.toml`.

```bash
npm run bindings:generate
npm run bindings:check
```

Do not edit generated TypeScript files by hand.

## Release candidate

Build and validate an Apple Silicon `.app`:

```bash
npm ci
npm run release:build
```

Use [RELEASE_RUNBOOK.md](RELEASE_RUNBOOK.md) to record build identity, complete
the target-hardware rehearsal, recover from event-day failures, and operate the
rehearsed release.
