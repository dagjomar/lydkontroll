# Plan: TASK-004 — Audio engine

Status: accepted
Updated: 2026-06-18

## Problem

Playback is event-critical, but Kira handles sample output rather than product
semantics. The application needs deterministic overlap, exclusive, retrigger,
stop, fade, completion, and failure behavior that can be tested without audio
hardware.

## Decisions

### Ownership and serialization

- Add an application-owned `PlaybackEngine<B: AudioBackend>`. Its public
  methods require `&mut self`, so the future `ApplicationService` can serialize
  all commands and backend events through one owner.
- The engine owns active playback records keyed by generated playback-instance
  ID. Each record retains cue ID, backend handle, cue volume, and cue fade time.
- `AudioBackend` accepts managed file paths and returns opaque handles. Kira,
  CPAL, decoder, and tween types never cross the adapter boundary.
- The engine does not predict completion with sleeps or a wall clock. The
  backend reports `Finished`, `Stopped`, or `Failed` events; processing those
  events is the only way fading or naturally completed instances leave active
  state.

### Trigger ordering

- Commands and backend events are processed in call order.
- Triggering an overlap cue starts immediately and does not affect other cues.
- Retriggering any cue first issues an immediate stop for every active instance
  of that cue, removes those instances from authoritative state, and then
  applies the new trigger. Late events for removed handles are ignored.
- Triggering an exclusive cue transitions every active instance using that
  instance's configured cue fade time. A zero-duration fade is an immediate
  stop. The exclusive cue remains pending and starts only after all transitioned
  instances report completion.
- If no instances are active, an exclusive cue starts immediately.
- A newer exclusive trigger replaces an older pending exclusive trigger. A
  retrigger of a cue also clears a pending trigger for that cue before applying
  the replacement.
- Overlap triggers received while an exclusive cue is waiting may start, but
  become part of the pending exclusive barrier and are faded before the
  exclusive cue starts. This preserves the rule that an exclusive cue starts
  alone.

### Stop, fade, and volume

- `stop(playback_id)` is immediate and removes the instance after a successful
  backend stop command.
- `fade(playback_id)` uses that instance's configured `fade_ms`; zero means
  immediate stop. The instance remains visible with `Fading` status until the
  backend completion event arrives.
- `stop_all()` immediately stops and removes every active instance and clears a
  pending exclusive trigger.
- `fade_all(duration_ms)` uses the supplied duration; callers use the product
  default of 2000 ms. It clears a pending exclusive trigger and keeps instances
  visible as fading until completion.
- Cue volume and master volume are clamped to `0.0..=1.0`. Master volume is
  delegated to the backend mixer track and retained by the engine.
- A second fade command replaces the backend tween for that instance. Stop
  always wins over a fade.

### Backend failures and output loss

- Backend operations return typed errors. A failed start creates no active
  instance. A failed stop/fade leaves the instance active so the caller can
  report and retry rather than publishing a false stopped state.
- Per-instance backend failure events remove that instance and are returned as
  engine events for the future application-state publisher.
- Manager/output failures are returned as global backend errors. Existing
  playback is marked failed and cleared because its audible state is no longer
  trustworthy; the engine remains alive and future starts may retry through the
  adapter.
- Resource-capacity and decode/open failures are surfaced without panicking.

### Production Kira adapter

- Own one long-lived Kira `AudioManager` using the default CPAL backend and one
  mixer track for master volume.
- Resolve playback only from `<app-data>/audio/<managed-file-name>`; reject
  paths that escape that directory.
- Load validated MP3/WAV through Kira's streaming sound API, apply per-cue
  volume, and retain one handle per backend playback ID.
- Use Kira tweens with a linear easing curve for deterministic operator-facing
  fades. Poll Kira handle state to emit completion events and discard handles.
- The selected macOS system output is owned by CPAL. Output disconnect and
  device changes are recoverable operator-visible errors, not process panics.

## Implementation Slices

1. Add playback domain types, the narrow `AudioBackend` port, typed engine
   errors/events, and the deterministic `PlaybackEngine`.
2. Add a fake backend and tests for overlap, retrigger, exclusive barriers,
   pending replacement, late events, natural completion, stop, fade, fade-all,
   volume clamping, and operation failures.
3. Add the Kira/CPAL adapter with managed-path validation, streaming MP3/WAV
   playback, mixer/per-instance volume, tweens, state polling, and typed errors.
4. Add adapter tests that do not require hardware plus a target-Mac manual
   checklist for MP3/WAV, overlap, exclusive ordering, fades, analog output,
   output loss, and recovery.

## Test Strategy

- Unit tests use a recording fake backend; no sleeps, real clock, decoder, or
  audio device are involved.
- Integration tests exercise managed-path resolution and invalid/missing media
  failures without opening the system output.
- `cargo test --manifest-path src-tauri/Cargo.toml` covers all state-machine
  behavior.
- Manual target-Mac validation:
  1. select the intended analog output in macOS;
  2. play representative managed MP3 and WAV cues;
  3. verify overlap, same-cue retrigger, exclusive wait/start, stop, per-cue
     fade, and two-second fade-all;
  4. change or disconnect the output during playback and confirm a visible
     recoverable error;
  5. restore the output and confirm a later trigger can play.

## Non-goals

- Command IDs, deduplication, revisions, acknowledgements, and snapshot
  publication belong to `TASK-005`.
- Tauri commands, WebSockets, Tailscale, React UI, and preflight presentation
  are out of scope.
- Audio-device selection inside the app is out of scope; version one follows
  the macOS system output.

## Ready Checklist

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task can move to `ready`.
