---
id: TASK-009
title: Add event preflight and operator diagnostics
status: idea
priority: P1
type: feature
owner: unassigned
depends_on: [TASK-003, TASK-004, TASK-006, TASK-007]
plan: none
updated: 2026-06-18
---

# TASK-009: Add event preflight and operator diagnostics

## Outcome

Before the event, the operator can verify Tailscale binding, system audio
output, managed files, mobile URL/QR code, and playback readiness in one view.

## Acceptance Criteria

- [ ] Every configured file is checked and failures identify the affected cue.
- [ ] Tailscale address and server reachability are shown.
- [ ] The current macOS output is shown where the platform APIs permit it, or a
      clear manual verification step is provided.
- [ ] A safe test-play workflow is available.
- [ ] Preflight results distinguish blocking failures from warnings.

## Validation

```text
npm test
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```
