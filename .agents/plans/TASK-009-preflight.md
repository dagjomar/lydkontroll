# Plan: TASK-009 — Event preflight and operator diagnostics

Status: accepted
Updated: 2026-06-18

## Problem

The existing snapshot carries coarse preflight facts, but the Mac operator has
no single view that explains blockers, distinguishes warnings, identifies cues
whose managed files are missing, or offers a bounded playback check before the
event.

## Current Evidence

- `AppSnapshot.preflight` is Rust-owned and revisioned by
  `ApplicationService`.
- `ControlServerRuntime` exposes the validated Tailscale address and mobile URL.
- `JsonLibraryRepository` owns the managed audio directory and validates files.
- `LocalAudioBackend` reports whether the macOS system output initialized, but
  version one intentionally does not select or name output devices.
- Desktop playback already routes through the shared command path.

## Open Questions

- Exact output-device naming is explicitly deferred because the current audio
  adapter does not expose a stable cross-platform device-name API. Preflight
  will show a warning with a manual System Settings verification step.
- Automatic Tailscale server rebinding is deferred to release hardening.
  Preflight reports the running server or its startup blocker without weakening
  the fail-closed binding rule.

## Options Considered

### Keep preflight entirely in React

- Benefits: minimal Rust work.
- Costs and risks: duplicates file and server truth outside their owning
  adapters and cannot safely inspect managed storage.

### Refresh Rust facts and render them in React

- Benefits: preserves the authoritative-state boundary, supports cue-specific
  file diagnostics, and keeps transports consistent.
- Costs and risks: adds one desktop-only refresh command and a warning protocol
  variant.

## Decision

Use the Rust-owned refresh approach described in ADR-012. The desktop presents
five checks: managed audio, audio output, Tailscale/control server, mobile
access, and bounded test playback. Blocking failures use `unavailable`; manual
verification uses `warning`; successful checks use `ready`.

Generate the mobile QR locally in the frontend. Safe test play is available
only for saved cues while no audio is active; it plays for three seconds and
then fades only the playback instance it started through the normal command
path.

## Implementation Slices

1. Extend preflight status with warnings and add repository-backed managed-file
   diagnostics that name affected cues.
2. Add a desktop refresh command that republishes current Rust-owned preflight
   facts and returns control-server information.
3. Build the desktop preflight panel, local QR, manual output guidance, and
   bounded test-play workflow.
4. Add Rust and React regression tests and regenerate TypeScript contracts.

## Test Strategy

- Automated: repository diagnostics for missing files and cue names; desktop
  refresh projection; warning serialization/bindings; preflight UI severity,
  server URL/QR, refresh, and bounded test play.
- Manual: launch on the target Mac, confirm the displayed URL opens on iPhone,
  verify the selected macOS output, and listen to the three-second test fade.
- Failure modes: missing managed file, unavailable control server, unavailable
  audio backend, no saved cue, and active playback during attempted test.

## Rollback or Recovery

The feature is additive. If refresh fails, retain the last authoritative
snapshot and show the command error. Removing the panel and refresh command
restores the previous editor without changing persistence or playback data.

## Ready Checklist

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task can move to `ready`.
