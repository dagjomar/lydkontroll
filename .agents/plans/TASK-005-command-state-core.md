# Plan: TASK-005 — Authoritative command and playback state core

Status: accepted
Updated: 2026-06-18

## Problem

Desktop and mobile transports need one serialized owner for cue lookup,
playback mutation, retries, backend events, and authoritative snapshots.
Without that boundary, command ordering, duplicate delivery, and revision
semantics can diverge between Tauri and WebSocket adapters.

## Current Evidence

- ADR-003 assigns Rust ownership of protocol types and authoritative state.
- `PlaybackEngine<B>` already requires mutable access and owns active and
  pending playback state.
- `PlaybackEngine::poll()` can finish instances, report failures, and start a
  pending exclusive cue, so polling must use the same serialization boundary
  as commands.
- `CueLibrary` contains the metadata needed to resolve cue IDs to managed audio
  paths beneath the repository audio directory.
- TASK-006 will add Axum/WebSocket transport and must share this service.

## Open Questions

- Resolved: one `std::sync::Mutex` protects the complete mutable service state;
  adapters share the service through `Arc`.
- Resolved: retain the 256 most recent command acknowledgements in FIFO order.
  A repeated command ID returns the original acknowledgement without replaying
  validation or side effects.
- Resolved: increment the revision once after each accepted command or poll
  batch that changes snapshot-visible state. Rejected commands retain the
  current revision.
- Resolved: publish complete snapshots through subscribers after revision
  changes; subscribers receive the current snapshot immediately.
- Resolved: keep at most 64 recoverable operator errors. Backend playback
  failures append an error; command validation failures are returned in their
  acknowledgement and do not pollute persistent operator state.

## Options Considered

### Mutex-serialized generic service

- Benefits: preserves call order, works with the existing synchronous audio
  and persistence ports, is shareable by Tauri and Axum, and is easy to test
  without a runtime.
- Costs and risks: callers must not hold the lock across unrelated work;
  poisoned-lock handling must remain typed.

### Dedicated actor task

- Benefits: naturally serializes async callers.
- Costs and risks: adds Tokio/channel lifecycle before networking exists and
  complicates synchronous tests and Tauri startup.

### Transport-owned locking and retry caches

- Benefits: smaller initial core.
- Costs and risks: duplicates behavior and violates ADR-003.

## Decision

Use the mutex-serialized generic service and bounded FIFO caches described in
ADR-008. Rust Serde/ts-rs protocol types live in the application layer because
they describe use cases and authoritative projections rather than persisted
domain data.

## Implementation Slices

1. Add versioned commands, acknowledgements, errors, preflight facts, operator
   errors, and snapshots with generated TypeScript bindings.
2. Add `ApplicationService<B>` around `CueLibrary`, managed audio path
   resolution, `PlaybackEngine<B>`, revision state, deduplication, errors, and
   snapshot subscribers.
3. Add thin transport-neutral Tauri adapter helpers that invoke only the
   service, ready for command registration in the desktop slice.
4. Test ordering, all command transitions, bounded retry behavior, revisions,
   polling transitions, failure publication, subscribers, and typed failures.
5. Regenerate bindings and run frontend, Rust, formatting, lint, and Ralph
   validation.

## Test Strategy

- Automated:
  - Fake-backend service tests for cue trigger, stop/fade, master volume,
    stop/fade all, serialized ordering, and backend events.
  - Duplicate IDs replay acknowledgements and do not replay audio effects.
  - More than 256 unique commands evict the oldest ID.
  - Rejected commands retain revisions; accepted transitions increment once.
  - Subscribers receive initial and ordered updated snapshots.
  - Generated TypeScript drift check covers all protocol types.
- Manual:
  - No new hardware check; real output behavior remains TASK-010 rehearsal.
- Failure modes:
  - Unknown cue/playback, invalid protocol/UUID, missing metadata, backend
    failure, poisoned state lock, and disconnected subscribers.

## Rollback or Recovery

The service remains generic over `AudioBackend`, and transports depend only on
its public command/snapshot methods. The cache capacity and publisher can be
changed without changing command semantics or the audio adapter.

## Ready Checklist

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task can move to `ready`.
