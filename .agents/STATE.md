# Current State

Last updated: 2026-06-18

## Phase

The compact-status replacement candidate is built and automatically verified.
The previously rehearsed event candidate remains the fallback until one short
native desktop and audible-output smoke check passes.

## Current Focus

Complete `TASK-016` on the event Mac: launch the packaged replacement, check the
collapsed/expanded status UI, QR and refresh, then hear the three-second test
play fade.

## Working Software

A runnable Tauri 2 application with:

- strict React/TypeScript/Vite frontend and native macOS shell;
- Rust-owned schema-v1 cue persistence and managed MP3/WAV import;
- deterministic Kira/CPAL playback behind an application-owned engine;
- one mutex-serialized `ApplicationService` owning commands, retries,
  revisions, snapshots, polling, preflight facts, and recoverable errors;
- responsive Mac scene/cue editing and local playback controls;
- injectable, timeout-bounded `tailscale ip -4` discovery with explicit CLI
  candidates, exactly-one-address parsing, `100.64.0.0/10` validation, and
  local-interface confirmation;
- fail-closed binding only to the validated Tailscale IPv4 address on port
  `17321`, with no wildcard, loopback, or LAN fallback;
- a gracefully stoppable Axum task on Tauri's async runtime whose failure
  leaves local playback and editing operational;
- HTTP serving through Tauri's embedded production `frontendDist` asset
  resolver;
- WebSocket initial snapshots, command acknowledgements, authoritative
  post-command snapshots, revision polling, malformed-frame errors, and
  reconnect-from-current-state behavior;
- application-service deduplication preserving idempotency across retried
  command IDs;
- generated `ControlServerInfo` and a desktop command exposing the mobile URL
  for QR presentation;
- hardware-free discovery, HTTP, WebSocket, bind-failure, reconnect, and
  graceful-shutdown coverage.
- a dedicated iPhone Safari projection with touch-safe controls, persistent
  connection/acknowledgement status, fresh-snapshot reconnects, stale-socket
  guards, duplicate in-flight action suppression, and an expanding,
  viewport-capped now-playing overlay that leaves the cue grid interactive.
- a desktop preflight panel with Rust-refreshed managed-file and control-server
  facts, cue-specific missing-file blockers, manual output warnings, mobile
  URL/QR presentation, and instance-scoped three-second test playback.
- a single `npm run release:build` gate covering frontend/Rust validation,
  tracked private-artifact checks, and an Apple Silicon `.app` bundle;
- a release runbook with build identity, 60-minute failure-injection rehearsal,
  recovery procedures, and event-day checklists.

## Known Blockers

- `TASK-016` needs a human at the Mac because native Tauri UI automation is not
  available on macOS and audible output cannot be inferred from mocks.
- Network tests need permission to bind temporary loopback ports in restricted
  environments.
- Native Tauri desktop WebDriver remains unavailable on macOS; use
  mock-runtime tests, standalone Playwright WebKit coverage, and manual native
  checks.

## Next Milestone

Pass the short native smoke check in `TASK-016`, then promote the replacement
candidate recorded in `.agents/CANDIDATE.md` for event use.

## Accepted Foundation

- Kira/CPAL with Symphonia decoding behind an `AudioBackend` port.
- Playback ordering lives in a serialized application-owned engine; backend
  completion events, not timers, finalize active state.
- Versioned atomic JSON plus managed audio under Tauri's app-data directory.
- Rust Serde types as the command/state source of truth with ts-rs bindings.
- One mutex-serialized application service with revisioned snapshots,
  256-command FIFO retry deduplication, and bounded recoverable errors.
- Axum on Tauri's async runtime, sharing one `ApplicationService`.
- Timeout-bounded `tailscale ip -4` discovery with local-address validation and
  no insecure bind fallback.
- Tauri's production asset resolver as the embedded HTTP asset boundary.
- Vitest/React Testing Library, Rust integration/mock-runtime tests, Playwright
  WebKit mobile coverage, and explicit target-hardware rehearsal.

## Risk Watchlist

- packaged macOS visibility of the Tailscale CLI;
- Safari reconnect behavior during real Wi-Fi/mobile transitions;
- keeping desktop, mobile, and Rust state/protocol definitions synchronized;
- macOS analog output behavior and fade correctness under overlapping playback;
- ensuring imported files survive source-file moves and app restarts.
- preserving the identity and checksum of the exact rehearsed build.
