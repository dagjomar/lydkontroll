# Current State

Last updated: 2026-07-04

## Phase

The retained repository history is publicly available at
https://github.com/dagjomar/lydkontroll as a source-only, source-available
project. Reusable identity, icon, privacy/history audit, licensing, contribution
terms, documentation, public validation, remote metadata, and vulnerability
reporting are complete.

## Current Focus

The public-source milestone is complete. The next independent product slice is
planning `TASK-026` for full English localization and language selection;
`TASK-027` retains later operating-mode research. `Lydkontroll` remains a
Norwegian-first, event-generic source-available product with configurable event
title.
Future desktop-only, Tailscale, and same-LAN product modes are captured as
non-blocking P3 research in `TASK-027`; version one remains desktop-capable and
Tailscale-only for mobile control.

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
- an optimistic iPhone master-volume projection that remains draggable while
  one command is in flight, coalesces rapid changes to the newest queued value,
  and reconciles with authoritative snapshots without snapback.
- a desktop preflight panel with Rust-refreshed managed-file and control-server
  facts, cue-specific missing-file blockers, manual output warnings, mobile
  URL/QR presentation, and instance-scoped three-second test playback.
- a compact desktop managed-file section that distinguishes repeated imports,
  blocks files used by saved or draft cues, requires confirmation, and removes
  unreferenced metadata and managed bytes with rollback on persistence failure.
- a single `npm run release:build` gate covering frontend/Rust validation,
  tracked private-artifact checks, and an Apple Silicon `.app` bundle;
- a release runbook with build identity, 60-minute failure-injection rehearsal,
  recovery procedures, and event-day checklists.
- a reproducible SVG-to-platform icon pipeline with automated transparent-corner
  checks for every macOS `.icns` representation.
- a reusable Norwegian-first `Lydkontroll` identity, configurable persisted
  event title with schema-v1 fallback, and neutral rounded-waveform icon.
- PolyForm Noncommercial 1.0.0 source terms, a separate-commercial-license
  posture, contributor CLA, support/security guidance, direct dependency
  license review, and a history-aware public-readiness gate.

## Known Blockers

- `TASK-016` needs a human at the Mac and iPhone because native Tauri UI
  automation is not available on macOS, audible output cannot be inferred from
  mocks, and physical Safari touch gestures cannot be fully reproduced here.
- Network tests need permission to bind temporary loopback ports in restricted
  environments.
- Native Tauri desktop WebDriver remains unavailable on macOS; use
  mock-runtime tests, standalone Playwright WebKit coverage, and manual native
  checks.

## Next Milestone

Refine `TASK-026` into an accepted localization architecture and dependency-
ordered implementation plan without destabilizing the Norwegian interface.

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
- ensuring corrected Logic exports are selected and configuration is saved
  before rehearsal;
- ensuring imported files survive source-file moves and app restarts.
- preserving the identity and checksum of the exact rehearsed build.
- preserving the audited public tree/history boundary as future commits land;
- avoiding ambiguous naming conflicts or an unsupported public brand promise;
- obtaining legal review before commercial sales or substantive contributions;
- keeping configurable branding from destabilizing the mobile cue layout.
- avoiding accidental exposure on untrusted local networks if optional LAN
  mobile control is added after version one.
