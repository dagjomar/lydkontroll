# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-008` is complete. iPhone Safari now receives a dedicated touch-safe
projection with visible connection and acknowledgement state, authoritative
fresh-snapshot reconnects, stale-socket guards, and duplicate in-flight action
suppression.

## Exact Next Action

Refine `TASK-009` into a plan for event preflight and operator diagnostics,
move it to `ready`, then implement it.

```text
python3 scripts/ralph.py claim TASK-009 --owner "<agent/session>"
```

## Important Context

- `AppSnapshot.preflight` already contains control-server, audio-output, and
  audio-file readiness facts, but `TASK-009` must refine how they are refreshed
  and presented as blocking failures versus warnings.
- `get_control_server_info` exposes the mobile URL. `TASK-009` owns desktop QR
  presentation and periodic Tailscale re-probing/rebind orchestration.
- Managed storage already validates imports and missing files on load. Preflight
  must identify affected cues rather than duplicate persistence ownership.
- macOS output naming may not be available through the current CPAL boundary;
  the task explicitly permits a clear manual verification step.
- Keep safe test playback on the shared command path and do not weaken the
  fail-closed Tailscale bind policy.
- Real iPhone transitions, packaged CLI paths, analog output, and output
  switching/loss remain release-rehearsal gates.

## Validation to Run

```text
npm test -- --run
npm run build
npm run lint
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
python3 scripts/ralph.py next
```
