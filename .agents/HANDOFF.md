# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-004` was completed. Playback semantics now live in a deterministic
application-owned engine, with a fake-backend test suite and a Kira 0.12/CPAL
adapter for managed MP3/WAV streaming through the macOS default output.

## Exact Next Action

Claim `TASK-005` for refinement. Resolve the service ownership, bounded
deduplication, polling, revision, and error-publication details in its task
notes or a linked plan, move it to `ready`, then start implementation.

```text
python3 scripts/ralph.py claim TASK-005 --owner "<agent/session>"
```

## Important Context

- `PlaybackEngine<B>` in `src-tauri/src/application/mod.rs` requires mutable
  serialized access and owns active/pending playback state.
- `AudioBackend` in `src-tauri/src/ports/mod.rs` is the only playback adapter
  boundary; transports must never invoke Kira directly.
- Retrigger stops the old same-cue instance immediately. Exclusive cues wait
  for all active configured fades to complete, and a newer pending exclusive
  replaces the old request.
- `poll()` converts backend completion/failure into engine events and may start
  a pending exclusive cue. `TASK-005` must serialize polling with commands and
  increment revisions for resulting state changes.
- Managed paths must be resolved from persistence metadata beneath the
  repository audio directory before constructing `CuePlaybackRequest`.
- Keep Tauri/Axum adapters thin. Tailscale and WebSocket behavior remain
  `TASK-006`.
- Real analog playback and output-loss recovery remain documented target-Mac
  rehearsal gates rather than automated proof.

## Validation to Run

```text
cargo test --manifest-path src-tauri/Cargo.toml
npm test -- --run
npm run build
npm run lint
python3 scripts/ralph.py check
python3 scripts/ralph.py next
```
