# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-002` was completed. The repository now has a runnable and tested Tauri 2,
Vite, React, TypeScript, and Rust shell. All automated validation passes, the
native macOS app launched successfully, and the user visually confirmed it.

## Exact Next Action

Start `TASK-003` and implement versioned scene/cue persistence plus managed
MP3/WAV import.

```text
python3 scripts/ralph.py start TASK-003 --owner "<agent/session>"
```

## Important Context

- Follow ADR-002 and ADR-003 plus the storage layout in the accepted TASK-001
  foundation plan.
- Keep Tauri app-data path resolution in an adapter and inject a root directory
  for integration tests.
- Rust contracts export deterministically to `src/generated/`; run
  `npm run bindings:check` after changing shared types.
- Cargo uses MSRV-aware dependency fallback so the committed graph works with
  Rust 1.85.
- Do not add playback, Axum, or Tailscale behavior in the persistence slice.

## Validation to Run

```text
cargo test --manifest-path src-tauri/Cargo.toml
npm run bindings:check
npm run lint
python3 scripts/ralph.py check
python3 scripts/ralph.py next
```
