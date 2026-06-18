# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-003` was completed. Rust now owns a schema-v1 cue library, atomic
save/backup/recovery, managed MP3/WAV import with decoder validation, and typed
recoverable storage errors. All automated validation passes.

## Exact Next Action

Claim `TASK-004` for planning, replace its placeholder plan with explicit
playback ordering/fade decisions, and move the task to `ready` once its ready
checklist is satisfied.

```text
python3 scripts/ralph.py claim TASK-004 --owner "<agent/session>"
```

## Important Context

- Follow ADR-001: Kira/CPAL stays behind `AudioBackend`; deterministic ordering
  belongs to an application-owned state machine tested with fakes.
- `CueLibrary`, `Cue`, `CueMode`, and managed audio metadata now live in
  `src-tauri/src/domain/mod.rs`.
- Production audio paths must resolve from managed filenames beneath the
  repository's `audio/` directory; source paths are deliberately unavailable.
- Resolve exclusive transition ordering, retrigger cancellation, fade
  completion, output-loss behavior, and state publication before implementation.
- Do not add command deduplication, Axum, Tailscale, or UI behavior in
  `TASK-004`.

## Validation to Run

```text
cargo test --manifest-path src-tauri/Cargo.toml
npm run bindings:check
npm run lint
python3 scripts/ralph.py check
python3 scripts/ralph.py next
```
