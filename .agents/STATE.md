# Current State

Last updated: 2026-06-18

## Phase

Persistence vertical slice complete; audio-engine planning is next.

## Current Focus

Refine the linked `TASK-004` audio-engine plan, resolve deterministic ordering
and fade semantics, then move the task to `ready`.

## Working Software

A runnable Tauri 2 shell with:

- a strict React/TypeScript/Vite frontend and native macOS window;
- a Rust library composition root plus minimal executable;
- domain/application/ports/adapters module boundaries;
- Vitest/React Testing Library and Tauri mock-runtime smoke tests;
- deterministic ts-rs output under `src/generated/`;
- ESLint, Prettier, rustfmt, Clippy, frontend build, and Rust test commands.
- a schema-v1 scene/cue/audio aggregate owned by Rust with generated TypeScript
  contracts;
- atomic JSON save/backup/recovery under the app-data root;
- staged, decoder-validated MP3/WAV import into managed storage;
- typed corrupt/future schema, missing-file, invalid-audio, and reference errors;
- integration coverage for round trips and interrupted-write recovery.

## Known Blockers

- No current implementation blocker.
- Native Tauri desktop WebDriver remains unavailable on macOS; use mock-runtime
  tests, standalone Playwright WebKit coverage, and manual native checks.

## Next Milestone

Deterministic local playback through Kira/CPAL with explicit overlap,
exclusive, retrigger, stop, and fade semantics.

## Accepted Foundation

- Kira/CPAL with Symphonia decoding behind an `AudioBackend` port.
- Versioned atomic JSON plus managed audio under Tauri's app-data directory.
- Rust Serde types as the command/state source of truth with ts-rs bindings.
- Axum on Tauri's async runtime, sharing one `ApplicationService`.
- Timeout-bounded `tailscale ip -4` discovery with local-address validation and
  no insecure bind fallback.
- Vitest/React Testing Library, Rust integration/mock-runtime tests, Playwright
  WebKit mobile coverage, and explicit target-hardware rehearsal.

## Risk Watchlist

- macOS audio behavior and fade correctness under overlapping playback;
- discovering and binding only the active Tailscale address;
- keeping desktop, mobile, and Rust state/protocol definitions synchronized;
- Safari reconnect behavior during real mobile-network transitions;
- ensuring imported files survive source-file moves and app restarts.
