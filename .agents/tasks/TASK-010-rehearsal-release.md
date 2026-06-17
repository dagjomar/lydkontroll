---
id: TASK-010
title: Harden, package, and rehearse the event release
status: idea
priority: P0
type: chore
owner: unassigned
depends_on: [TASK-008, TASK-009]
plan: .agents/plans/TASK-010-rehearsal.md
updated: 2026-06-18
---

# TASK-010: Harden, package, and rehearse the event release

## Outcome

A reproducible macOS build passes automated checks and a documented full
rehearsal with iPhone on mobile data, Mac on Wi-Fi, Tailscale, and analog audio.

## Acceptance Criteria

- [ ] Release build succeeds for Apple Silicon macOS.
- [ ] Automated tests cover persistence, playback, protocol, and reconnect.
- [ ] Phone/network loss does not interrupt active audio or local controls.
- [ ] The full manual rehearsal checklist is completed and recorded.
- [ ] Recovery steps and an event-day operator checklist are documented.
- [ ] No credentials, audio files, local data, or generated bundles are tracked.

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

Create the linked rehearsal and recovery plan before this task becomes ready.
