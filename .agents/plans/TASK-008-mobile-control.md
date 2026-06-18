# Plan: TASK-008 — Reconnecting iPhone control interface

Status: accepted
Updated: 2026-06-18

## Problem

The control server already serves the production frontend and authoritative
WebSocket snapshots, but Safari currently receives the desktop editor. Mobile
control needs a touch-safe, read-only projection that makes connection and
command state obvious and never treats cached state as current after a break.

## Current Evidence

- `desktopApi.ts` already selects WebSocket transport outside Tauri, reconnects
  with bounded backoff, and receives full snapshots and acknowledgements.
- `ApplicationService` deduplicates command IDs and the server sends a fresh
  snapshot on every connection.
- `Shell.tsx` owns desktop editing and should remain unchanged for Tauri.

## Decisions

- Use the same production build and select a dedicated mobile projection when
  the frontend is not running inside Tauri.
- Extend the frontend adapter with observable connection state. On disconnect,
  invalidate the cached snapshot and require the next socket generation to
  deliver a fresh snapshot.
- Ignore events from replaced socket generations.
- Disable controls while disconnected and suppress duplicate taps while a
  command is awaiting acknowledgement. Show the pending/confirmed/error state.
- Keep mobile editing, delta replay, and automatic command retry out of scope.
  A timed-out operator action may be tapped again with a new command ID; Rust
  remains authoritative.

## Implementation Slices

1. Harden the WebSocket adapter with connection subscriptions, fresh-snapshot
   reconnect behavior, and stale-socket guards.
2. Add the mobile scene/cue, active playback, master volume, stop, and fade
   projection with persistent status presentation.
3. Add component and adapter tests for disconnect, reconnect, duplicate taps,
   and stale socket events.
4. Run frontend, build, lint, and Ralph validation.

## Test Strategy

- Automated: React tests for touch controls, visible connection/ack state,
  duplicate-tap suppression, and authoritative revision replacement; adapter
  tests for disconnect invalidation, reconnect, and stale socket events.
- Manual: iPhone Safari through Tailscale, including Wi-Fi/mobile transitions.
- Failure modes: socket closes during a command, acknowledgement timeout,
  malformed server message, and a late event from an old socket.

## Rollback or Recovery

The desktop shell remains selected by the Tauri runtime. Reverting the mobile
projection does not affect playback, persistence, or the server protocol.

## Ready Checklist

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task can move to `ready`.
