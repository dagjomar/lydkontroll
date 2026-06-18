---
id: TASK-009
title: Add event preflight and operator diagnostics
status: done
priority: P1
type: feature
owner: codex-2026-06-18-preflight
depends_on: [TASK-003, TASK-004, TASK-006, TASK-007]
plan: .agents/plans/TASK-009-preflight.md
updated: 2026-06-18
---

# TASK-009: Add event preflight and operator diagnostics

## Outcome

Before the event, the operator can verify Tailscale binding, system audio
output, managed files, mobile URL/QR code, and playback readiness in one view.

## Acceptance Criteria

- [x] Every configured file is checked and failures identify the affected cue.
- [x] Tailscale address and server reachability are shown.
- [x] The current macOS output is shown where the platform APIs permit it, or a
      clear manual verification step is provided.
- [x] A safe test-play workflow is available.
- [x] Preflight results distinguish blocking failures from warnings.

## Validation

```text
npm test
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```

## Notes

- Rust owns refreshed readiness facts; React renders severity, guidance, QR,
  and safe actions.
- Output-device naming is deferred; provide a warning with a manual macOS
  verification step.
- Test play uses the shared command path, runs only while playback is idle, and
  automatically fades after three seconds.
- 2026-06-18: Task claimed.
- 2026-06-18: Implemented Rust-owned refresh, cue-specific missing-file
  diagnostics, warning severity, mobile URL/QR presentation, and an
  instance-scoped three-second test play.
- 2026-06-18: Verified at 1280px and 760px without horizontal overflow.

- 2026-06-18: Implemented and validated event preflight diagnostics.
