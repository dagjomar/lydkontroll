# Progress Log

Append concise, dated entries. Keep detailed implementation notes in task files.

## 2026-06-18 — Project harness established

- Added the Ralph-loop workflow, task schema, validator, and session handoff.
- Distilled stable product constraints from `PLAN.md`.
- Created the initial dependency-ordered backlog.
- Next: plan `TASK-001`, then scaffold the application in `TASK-002`.

## 2026-06-18 — Foundation architecture accepted

- Completed `TASK-001` with an implementation-ready architecture, module/data
  flow, failure modes, recovery behavior, and dependency-ordered slices.
- Accepted ADR-001 through ADR-006 covering Kira/CPAL audio, versioned atomic
  JSON, Rust/ts-rs contracts, Axum/Tauri integration, fail-closed Tailscale
  discovery, and layered testing.
- Refined `TASK-002` through `TASK-006` to encode the accepted boundaries and
  observable acceptance criteria.
- Moved `TASK-002` to `ready`; Ralph validation passes.

## 2026-06-18 — Runnable Tauri shell completed

- Scaffolded Tauri 2, Vite, React, strict TypeScript, and the Rust
  domain/application/ports/adapters boundaries.
- Added frontend behavior coverage, a Tauri mock-runtime smoke test, and a
  deterministic committed ts-rs contract with drift detection.
- Added npm/Cargo lock files and unified ESLint, Prettier, rustfmt, Clippy,
  build, and test commands with Rust-1.85-aware dependency resolution.
- Validated all automated commands and launched the native macOS app; the user
  visually confirmed the shell.
- Next: implement `TASK-003` persistence and managed audio import.

## 2026-06-18 — Persistence and managed audio import completed

- Added schema-v1 scene, cue, and managed-audio Rust contracts with committed
  ts-rs TypeScript output and whole-directory drift detection.
- Implemented atomic JSON save/backup/recovery and injected app-data paths with
  Tauri resolution kept at the adapter boundary.
- Added staged MP3/WAV import with Symphonia frame decoding, fsync, UUID-backed
  managed filenames, and no persisted source paths.
- Added typed recovery errors and nine integration tests covering both formats,
  round trips, corruption, future schemas, interrupted writes, missing files,
  invalid references, and staging cleanup.
- Validated Rust/frontend tests, frontend build, bindings, formatting, lint,
  Clippy, and Ralph metadata.
- Next: refine the `TASK-004` audio-engine plan and move it to `ready`.

## 2026-06-18 — Deterministic local audio playback completed

- Accepted explicit retrigger, exclusive-barrier, cancellation, fade,
  completion, and failure semantics in the `TASK-004` plan and ADR-007.
- Added an application-owned `PlaybackEngine` plus narrow `AudioBackend` port,
  with eleven fake-backend tests independent of React, networking, clocks, and
  audio hardware.
- Added the Kira 0.12/CPAL production adapter for managed streaming MP3/WAV,
  per-cue/master volume, linear stop tweens, decoder failures, and completion
  polling.
- Documented the target-Mac analog/output-loss rehearsal and validated Rust and
  frontend tests, build, bindings, formatting, ESLint, and Clippy.
- Next: implement `TASK-005` authoritative commands, revisions,
  deduplication, and snapshots around the playback engine.

## 2026-06-18 — Authoritative command and state core completed

- Accepted ADR-008 and the `TASK-005` plan for mutex-serialized commands,
  polling, revisions, retry deduplication, snapshot publication, and bounded
  recoverable errors.
- Added versioned Rust/ts-rs commands, acknowledgements, failures, preflight
  facts, operator errors, playback projections, and complete snapshots.
- Added one `ApplicationService` shared through thin adapters, with cue lookup,
  managed-path resolution, deterministic playback mutation, backend polling,
  256-command FIFO deduplication, and revisioned subscribers.
- Added seven integration tests for serialized concurrent callers, retries and
  eviction, typed validation/backend failures, transition revisions, polling,
  preflight publication, and thin Tauri adapter use.
- Validated Rust/frontend tests, frontend build, generated binding drift,
  formatting, ESLint, and Clippy.
- Next: refine `TASK-006` and implement the fail-closed Tailscale/Axum transport.

## 2026-06-18 — Local Mac workflow promoted

- Reordered delivery so `TASK-007` desktop editing/local control is the next P0
  milestone and `TASK-006` remote transport follows at P1.
- Marked `TASK-007` `needs-planning` with an explicit end-to-end local
  checkpoint: import audio, create a cue, play it, and stop/fade it on the Mac
  without Tailscale.
- Kept the technical dependency graph honest; remote transport is postponed by
  priority, not coupled to the desktop UI.
- Next: plan and implement `TASK-007`.

## 2026-06-18 — Desktop local-control workflow completed

- Accepted the `TASK-007` plan and ADR-009: desktop import/save operations are
  serialized, persisted first, then published through `ApplicationService`.
- Added native MP3/WAV selection, concrete Tauri-managed service wiring,
  resilient audio startup, missing-file recovery, and revisioned managed-audio
  metadata in authoritative snapshots.
- Replaced the scaffold with a responsive operator UI for scene/cue CRUD and
  ordering, cue configuration, import, trigger, active playback, stop/fade,
  master volume, and visible recoverable errors.
- Added Rust service/persistence coverage, three critical frontend state-flow
  tests, and a standalone browser preview used for visual QA.
- Validated Cargo tests, Vitest, production build, ESLint, Prettier, generated
  bindings, rustfmt, Clippy, Ralph metadata, and diff hygiene.
- Next: refine and implement `TASK-006` Tailscale-only control transport.

## 2026-06-18 — Fail-closed Tailscale control transport completed

- Accepted the `TASK-006` threat/failure plan for explicit CLI candidates,
  bounded discovery, local CGNAT validation, listener lifecycle, embedded
  assets, reconnect, acknowledgements, and QR URL data.
- Added typed `tailscale ip -4` discovery with a three-second timeout and no
  wildcard, loopback, LAN, or invalid-address fallback.
- Added a gracefully stoppable Axum server that serves Tauri's embedded
  production assets and routes WebSocket commands through the shared
  `ApplicationService`.
- Added initial/current snapshots, acknowledgements, retry idempotency,
  malformed-frame recovery, revision polling, reconnect coverage, bind
  refusal, and graceful listener release tests.
- Exposed generated `ControlServerInfo` to the desktop for mobile URL/QR
  presentation and made startup failure a visible local-only preflight state.
- Validated Cargo tests, Vitest, production build, generated bindings,
  formatting, ESLint, Clippy, Ralph metadata, and diff hygiene.
- Next: refine and implement `TASK-008` reconnecting iPhone controls.

## 2026-06-18 — Safari transport runtime corrected

- Reproduced the iPhone failure where the embedded frontend called Tauri's
  native `invoke` bridge from ordinary Safari.
- Added runtime selection so the native window keeps Tauri commands while
  Safari uses `/ws` for snapshots, acknowledgements, commands, timeouts, and
  reconnect backoff.
- Rebuilt the production assets and validated Vitest, TypeScript/Vite build,
  ESLint, Prettier, generated bindings, rustfmt, and Clippy.
- Added and tested a UUID-v4 fallback for Safari versions where
  `crypto.randomUUID` is unavailable.

## 2026-06-18 — Reconnecting iPhone controls completed

- Accepted ADR-010 and the `TASK-008` plan for a dedicated Safari projection
  driven only by fresh authoritative WebSocket snapshots.
- Added touch-safe scene/cue controls, active playback actions, master volume,
  stop/fade-all controls, and persistent connection/acknowledgement status.
- Invalidated cached state on disconnect, guarded replaced socket generations,
  and suppressed duplicate actions while acknowledgements are pending.
- Added component and adapter tests for disconnect, reconnect, stale sockets,
  authoritative replacement, and duplicate taps.
- Validated Vitest, production build, lint, Ralph metadata, diff hygiene, and a
  390x844 rendered cue interaction.
- Next: refine and implement `TASK-009` event preflight and diagnostics.

## 2026-06-18 — Mobile cue-layout safety follow-up added

- Added `TASK-011` from operator feedback: the conditional `Spiller nå` region
  shifts cue positions when playback starts and risks an accidental cue tap.
- Classified it as a ready P0 bug after `TASK-008`, ahead of preflight work.
- Defined a fixed-height idle/active playback region, compact touch-safe
  emergency controls, automated state coverage, and a 390x844 position check.
- Next: implement `TASK-011`, then return to `TASK-009`.

## 2026-06-18 — Stable iPhone cue layout completed

- Kept the `Spiller nå` panel mounted with a fixed outer height, a quiet idle
  state, and internal scrolling for multiple active sounds.
- Compacted emergency and playback actions while preserving measured 44-pixel
  touch targets.
- Added idle-to-playing-to-idle regression coverage and made the mobile preview
  simulate playback commands for rendered checks.
- At 390x844, verified the first cue remained at document position
  585.9296875 pixels through idle, playing, and stopped states.
- Validated Vitest, production build, ESLint, Prettier, generated bindings,
  rustfmt, Clippy, Ralph metadata, and diff hygiene.
- Next: refine `TASK-009` event preflight and operator diagnostics.
