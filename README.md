# Lydkontroll

Norwegian-first Tauri 2 application for reliable local audio-cue playback at
live events, with a React/TypeScript interface and a Rust-owned application
core.

The current implementation includes managed cue persistence/import,
deterministic local playback, desktop editing, reconnecting iPhone control over
Tailscale, and event preflight diagnostics.

## What works without a network

The Mac application can import audio, edit scenes/cues, play, stop, fade, and
control master volume without a phone, Tailscale, or internet connection. Audio
continues and Mac-local control remains available if mobile connectivity drops.

Mobile control is optional. In version one it requires Tailscale on both the Mac
and iPhone; ordinary-LAN control and automatic network fallback are not
supported. The mobile interface sends commands and receives state only. Audio
files remain on the Mac.

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
npm run public:check
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

## Architecture

- React/TypeScript renders desktop and mobile projections but does not own
  playback, networking, or persistence behavior.
- Rust owns managed audio, the cue library, deterministic playback, command
  serialization, authoritative snapshots, preflight facts, and the control
  server.
- Serde Rust types generate the shared TypeScript contracts in `src/generated`.
- The HTTP/WebSocket server binds only to a validated Tailscale IPv4 address on
  port `17321`. Tailscale membership is the version-one access-control boundary;
  Lydkontroll adds no password or PIN.

See `PLAN.md` and `.agents/PROJECT.md` for product invariants and the durable
task/decision history.

## Limitations

- Supported targets are Apple Silicon macOS and iPhone Safari.
- macOS uses the selected system audio output; output selection happens in
  System Settings.
- Tailscale is required only for mobile control, but anyone allowed onto the
  relevant tailnet may be able to reach the controller.
- Hardware audio behavior and physical Safari gestures still require manual
  validation; mocks and browser automation cannot prove audible output.
- There are no downloadable, signed, or notarized public application bundles.

## Source-only release

The public repository provides source and build instructions. It does not
distribute the unsigned `.app` created by the local release workflow. Build an
Apple Silicon candidate locally with:

Build and validate an Apple Silicon `.app`:

```bash
npm ci
npm run release:build
```

Use [RELEASE_RUNBOOK.md](RELEASE_RUNBOOK.md) to record build identity, complete
the target-hardware rehearsal, recover from event-day failures, and operate the
rehearsed release.

## Licensing and contributions

Lydkontroll is **source-available**, not OSI open source. Original project code
is licensed under the [PolyForm Noncommercial License 1.0.0](LICENSE): you may
inspect, fork, modify, and redistribute it for permitted noncommercial purposes.
Commercial use, resale, paid distribution, or incorporation into a commercial
offering requires a separate license from the copyright holder.

Commercially licensed finished bundles, setup, or support may be offered
separately in the future. Their availability is not promised by this repository.
Dependencies keep their own licenses; see [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).

Contributions are welcome under [CONTRIBUTING.md](CONTRIBUTING.md). Substantive
contributions require acceptance of [CLA.md](CLA.md), which lets contributors
retain copyright while granting the project owner commercial and relicensing
rights. Obtain professional legal review before relying on these documents for
commercial distribution.

For support boundaries see [SUPPORT.md](SUPPORT.md). Report vulnerabilities
privately as described in [SECURITY.md](SECURITY.md).
