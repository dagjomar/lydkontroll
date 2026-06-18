---
id: TASK-010
title: Harden, package, and rehearse the event release
status: blocked
priority: P0
type: chore
owner: codex-2026-06-18-release
depends_on: [TASK-008, TASK-009]
plan: .agents/plans/TASK-010-rehearsal.md
updated: 2026-06-18
---

# TASK-010: Harden, package, and rehearse the event release

## Outcome

A reproducible macOS build passes automated checks and a documented full
rehearsal with iPhone on mobile data, Mac on Wi-Fi, Tailscale, and analog audio.

## Acceptance Criteria

- [x] Release build succeeds for Apple Silicon macOS.
- [x] Automated tests cover persistence, playback, protocol, and reconnect.
- [ ] Phone/network loss does not interrupt active audio or local controls.
- [ ] The full manual rehearsal checklist is completed and recorded.
- [x] Recovery steps and an event-day operator checklist are documented.
- [x] No credentials, audio files, local data, or generated bundles are tracked.

## Validation

```text
npm run build
npm test
npm run lint
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
python3 scripts/ralph.py check
```

## Notes

- 2026-06-18: Accepted the linked plan for a locked unsigned local `.app`, build
  identity and checksum evidence, a 60-minute target-hardware matrix, explicit
  blocking failures, Mac-local fallback, and output/network recovery.

- 2026-06-18: Task claimed.

- 2026-06-18: Added `npm run release:build`, tracked-file safety validation,
  macOS `.app` bundling with a generated `.icns`, and `RELEASE_RUNBOOK.md`.
  The complete release command passes and produces a 5.2 MB ad-hoc signed arm64
  app. The remaining acceptance gates require the event Mac, iPhone/cellular
  Tailscale path, production audio library, and analog output.

- 2026-06-18: Automated release build and runbook are complete; finishing requires the documented 60-minute rehearsal on the event Mac with iPhone cellular/Tailscale, production cues, and analog output.
