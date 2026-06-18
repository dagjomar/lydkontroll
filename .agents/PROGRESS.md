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
