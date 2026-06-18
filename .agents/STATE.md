# Current State

Last updated: 2026-06-18

## Phase

Foundation complete; persistence implementation is next.

## Current Focus

Start `TASK-003`: implement versioned cue/scene persistence and managed MP3/WAV
import using the accepted storage architecture.

## Working Software

A runnable Tauri 2 shell with:

- a strict React/TypeScript/Vite frontend and native macOS window;
- a Rust library composition root plus minimal executable;
- domain/application/ports/adapters module boundaries;
- Vitest/React Testing Library and Tauri mock-runtime smoke tests;
- deterministic ts-rs output under `src/generated/`;
- ESLint, Prettier, rustfmt, Clippy, frontend build, and Rust test commands.

## Known Blockers

- No current implementation blocker.
- Native Tauri desktop WebDriver remains unavailable on macOS; use mock-runtime
  tests, standalone Playwright WebKit coverage, and manual native checks.

## Next Milestone

Versioned scene/cue persistence and managed MP3/WAV import with atomic writes,
backup recovery, generated contracts, and integration tests.

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
