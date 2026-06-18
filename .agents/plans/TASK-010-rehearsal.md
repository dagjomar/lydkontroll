# Plan: TASK-010 — Release rehearsal and recovery

Status: accepted
Updated: 2026-06-18

## Problem

Event readiness needs a repeatable rehearsal, explicit pass/fail gates, and
recovery procedures rather than an informal final check.

## Current Evidence

- The application already has automated persistence, playback, command,
  WebSocket reconnect, mobile projection, and preflight coverage.
- `package-lock.json` and `src-tauri/Cargo.lock` pin the JavaScript and Rust
  dependency graphs.
- The current Tauri configuration builds the executable but has application
  bundling disabled.
- Packaged Tailscale CLI discovery, real iPhone network transitions, analog
  output, output switching/loss, sleep prevention, and recovery can only be
  accepted on the target Mac and phone.
- The Mac UI remains the supported fallback when mobile control is unavailable.

## Open Questions

- No product questions remain. The target-hardware rehearsal and its evidence
  are external completion gates.

## Options Considered

### Option A: Informal final check

- Benefits: fastest to perform.
- Costs and risks: no stable pass/fail criteria, build identity, failure
  injection, or repeatable recovery steps.

### Option B: Signed and notarized distribution

- Benefits: conventional installation on arbitrary Macs.
- Costs and risks: requires Apple credentials and distribution work that does
  not improve the known-event-Mac workflow.

### Option C: Locked local build plus recorded target-hardware rehearsal

- Benefits: reproducible from committed locks, verifies the exact event setup,
  and avoids credential scope while keeping evidence and recovery steps.
- Costs and risks: the build is for the known Mac and may require local
  Gatekeeper approval if copied to another machine.

## Decision

Use Option C. Build an Apple Silicon `.app` from the committed lockfiles and
record the Git commit, application version, artifact path, and SHA-256 before
rehearsal. Do not add signing credentials or generated bundles to Git.

The release gate is one uninterrupted 60-minute rehearsal on the event Mac
using the production cue library, charger, selected analog output, and an
iPhone with Wi-Fi disabled so control crosses Tailscale over mobile data.
Exercise MP3 and WAV cues, overlap, exclusive playback, retrigger, cue stop,
fade, stop/fade all, master volume, scene changes, app relaunch, and managed
file persistence.

Inject these failures while audio is active:

1. lock Safari and reopen it;
2. toggle iPhone airplane mode, restore cellular/Tailscale, and reconnect;
3. quit and restart Tailscale on the phone;
4. remove and restore Mac Wi-Fi/Tailscale only after confirming local controls
   remain available;
5. disconnect and restore the analog cable;
6. switch the macOS output away and back.

Any crash, interrupted active audio caused by phone/network loss, unavailable
local controls, wrong cue behavior, missing persisted configuration, insecure
non-Tailscale bind, or unrecoverable output failure blocks release. Mobile-only
loss is recoverable by operating from the Mac. Network/control-server recovery
may restart Tailscale or the app only after active audio has ended. Output
recovery uses System Settings followed by preflight and a test play.

## Implementation Slices

1. Enable `.app` bundling and add one release verification command that runs
   the complete automated suite.
2. Document build identity, rehearsal evidence, release gates, recovery, and
   event-day operation in a copy-paste-friendly runbook.
3. Run the automated suite, build the Apple Silicon application, and record the
   artifact identity.
4. Perform and record the target-hardware rehearsal. This slice requires the
   operator, event Mac peripherals, iPhone, Tailscale accounts, and analog
   output.

## Test Strategy

- Automated: frontend build/tests, ESLint/Prettier/binding drift, rustfmt,
  Clippy, Rust tests, Ralph checks, tracked-secret/media/bundle inspection, and
  a Tauri release build.
- Manual: execute the runbook on the exact event hardware for 60 minutes and
  record each gate as pass/fail with notes.
- Failure modes: package build failure, wrong architecture, Tailscale CLI not
  visible in the packaged app, phone reconnect failure, audio interruption
  during network loss, output loss without clear recovery, persistence loss,
  sleep, or tracked release/private artifacts.

## Rollback or Recovery

- Keep the previous rehearsed `.app` until the replacement passes the complete
  matrix.
- If mobile control fails, continue with local Mac controls; do not restart the
  app while audio is active.
- If the control server fails while idle, restart Tailscale and then the app,
  refresh preflight, and scan the newly displayed QR code.
- If audio output is wrong or lost, stop playback, select the analog output in
  macOS System Settings, refresh preflight, and run the three-second test.
- If persisted configuration is corrupt, preserve app data and recover
  `library.json.bak` before making further edits.

## Ready Checklist

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task can move to `ready`.
