# Current State

Last updated: 2026-06-18

## Phase

Planning and foundation.

## Current Focus

Start `TASK-002`: scaffold the runnable Tauri 2, Vite, React, TypeScript, and
Rust shell using the accepted foundation architecture.

## Working Software

None yet. The repository contains product planning and the Ralph-loop project
harness only.

## Known Blockers

- The Tauri application has not been scaffolded.
- Native Tauri desktop WebDriver is unavailable on macOS; the accepted test
  strategy uses mock-runtime tests, standalone Playwright WebKit coverage, and
  manual native checks instead.

## Next Milestone

A runnable Tauri 2 application shell with frontend and Rust tests, linting, and
build commands exposed through `package.json`.

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
