# Current State

Last updated: 2026-06-18

## Phase

Deterministic local audio playback complete; authoritative command/state core
is next.

## Current Focus

Refine `TASK-005` into an implementation-ready task, then build one serialized
application service that owns the cue library and `PlaybackEngine`, emits
revisioned authoritative snapshots, and deduplicates bounded command IDs for
both future transports.

## Working Software

A runnable Tauri 2 shell with:

- a strict React/TypeScript/Vite frontend and native macOS window;
- a Rust library composition root plus minimal executable;
- domain/application/ports/adapters module boundaries;
- Vitest/React Testing Library and Tauri mock-runtime smoke tests;
- deterministic ts-rs output under `src/generated/`;
- ESLint, Prettier, rustfmt, Clippy, frontend build, and Rust test commands;
- a schema-v1 scene/cue/audio aggregate owned by Rust with generated TypeScript
  contracts;
- atomic JSON save/backup/recovery under the app-data root;
- staged, decoder-validated MP3/WAV import into managed storage;
- typed corrupt/future schema, missing-file, invalid-audio, and reference
  errors;
- a deterministic `PlaybackEngine` for overlap, exclusive barriers, retrigger,
  stop, per-cue fade, fade-all, master volume, completion, and backend failure;
- a Kira 0.12/CPAL streaming adapter for managed MP3/WAV files through the
  selected macOS system output;
- hardware-free playback coverage using a recording fake backend.

## Known Blockers

- No current implementation blocker.
- Native Tauri desktop WebDriver remains unavailable on macOS; use mock-runtime
  tests, standalone Playwright WebKit coverage, and manual native checks.
- Real analog playback, output switching/loss, and recovery are documented but
  remain target-Mac rehearsal gates.

## Next Milestone

One authoritative application state and command path shared by local Tauri and
future WebSocket transports.

## Accepted Foundation

- Kira/CPAL with Symphonia decoding behind an `AudioBackend` port.
- Playback ordering lives in a serialized application-owned engine; backend
  completion events, not timers, finalize active state.
- Versioned atomic JSON plus managed audio under Tauri's app-data directory.
- Rust Serde types as the command/state source of truth with ts-rs bindings.
- Axum on Tauri's async runtime, sharing one `ApplicationService`.
- Timeout-bounded `tailscale ip -4` discovery with local-address validation and
  no insecure bind fallback.
- Vitest/React Testing Library, Rust integration/mock-runtime tests, Playwright
  WebKit mobile coverage, and explicit target-hardware rehearsal.

## Risk Watchlist

- macOS analog output behavior and fade correctness under real overlapping
  playback;
- discovering and binding only the active Tailscale address;
- keeping desktop, mobile, and Rust state/protocol definitions synchronized;
- Safari reconnect behavior during real mobile-network transitions;
- ensuring imported files survive source-file moves and app restarts.
