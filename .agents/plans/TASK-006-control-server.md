# Plan: TASK-006 — Tailscale control server

Status: accepted
Updated: 2026-06-18

## Problem

The mobile server is protected only by its Tailscale bind boundary. Discovery,
local-address validation, listener lifecycle, and reconnect behavior therefore
must fail closed without coupling network availability to local playback.

## Decisions

### Discovery and validation

- Add an injectable synchronous `ProcessRunner` around `tailscale ip -4`.
- Try explicit packaged-install candidates in order: an optional configured
  path, `tailscale` from `PATH`,
  `/Applications/Tailscale.app/Contents/MacOS/Tailscale`, and
  `/usr/local/bin/tailscale`.
- Bound every attempt with a three-second timeout and terminate timed-out child
  processes.
- Accept UTF-8 output containing exactly one IPv4 address, require
  `100.64.0.0/10`, and confirm the address appears in an injected local-address
  inventory.
- Treat missing executables, timeout, non-zero exit, malformed/multiple output,
  non-Tailscale ranges, and non-local addresses as typed diagnostics. Never
  substitute wildcard, loopback, or LAN addresses.

### Server lifecycle

- Bind `TcpListener` to the validated address and port `17321` before spawning
  Axum, so startup has an immediate typed success/failure result.
- Own the listener in a `ControlServer` handle with a cancellation token and a
  join handle. Shutdown is graceful and independent of the application/audio
  service.
- Tauri startup attempts discovery and bind after the local coordinator exists.
  Failure updates `preflight.controlServer` to unavailable but leaves the
  desktop application operational.
- Re-probe/rebind orchestration is exposed by the adapter boundary; automatic
  periodic refresh is deferred to `TASK-009`, which owns preflight and
  diagnostics refresh behavior.

### HTTP assets and URL

- Serve `/` and static paths through an injected asset provider. Production
  uses Tauri's asset resolver, which reads `frontendDist` from the executable's
  embedded production assets; tests use an in-memory provider. `TASK-008` can
  change the frontend projection without changing this transport boundary.
- Expose `/health` for a minimal transport check.
- Publish `http://<tailscale-ip>:17321/` as `ControlServerInfo`; the desktop can
  render this string as QR-code payload data without the network adapter owning
  QR presentation.

### WebSocket protocol

- Upgrade at `/ws` and immediately send a tagged snapshot message produced by
  the shared `ApplicationService`.
- Accept JSON `CommandEnvelope` messages only. Execute them through that same
  service and return a tagged acknowledgement followed by the authoritative
  current snapshot.
- Malformed JSON returns a tagged protocol error without closing healthy
  connections. Binary frames are rejected with the same error.
- Reconnect has no delta replay: every new socket receives a fresh snapshot.
  Duplicate command IDs rely on `ApplicationService`'s bounded acknowledgement
  cache, so retries do not replay side effects.
- A lightweight polling loop publishes newer snapshots while a client is
  connected. The server does not own playback and disconnect never mutates it.

## Implementation Slices

1. Add typed discovery diagnostics, injected process/local-address seams, CLI
   candidate selection, timeout handling, parsing, CIDR validation, and tests.
2. Add tagged WebSocket server messages, embedded HTTP assets, Axum router, and
   router/WebSocket tests using a hardware-free application service.
3. Add explicit listener startup, graceful shutdown, server URL information,
   and bind-refusal/lifecycle tests.
4. Wire startup into Tauri after the desktop coordinator is managed; publish
   ready/unavailable control-server preflight without making startup fatal.
5. Run Rust, frontend, generated-contract, formatting, lint, and Ralph
   validation. Leave packaged CLI-path and real-iPhone behavior as release
   rehearsal gates.

## Test Strategy

- Unit tests:
  - exactly-one-address parsing;
  - timeout, command failure, malformed output, multiple addresses;
  - rejection of non-CGNAT, loopback, wildcard, LAN, and non-local addresses;
  - candidate fallback and configured-path precedence.
- Adapter tests:
  - embedded root and health routes;
  - initial WebSocket snapshot;
  - command acknowledgement plus resulting snapshot;
  - duplicate command ID returns the cached acknowledgement;
  - malformed frames do not invoke application behavior;
  - reconnect starts from the latest authoritative snapshot.
- Lifecycle tests:
  - explicit loopback test listener only through a test-only constructor;
  - occupied-port bind failure;
  - graceful shutdown releases the listener.
- Manual release gates:
  - verify actual Tailscale CLI location in the packaged app environment;
  - connect iPhone Safari over Tailscale and verify LAN-independent control;
  - switch Wi-Fi/mobile connectivity and confirm Mac playback continues.

## Failure and Recovery

- Any discovery or bind error leaves control transport unavailable and local
  playback/editor behavior intact.
- A listener task failure is reported through preflight/diagnostics; it never
  owns or drops `ApplicationService`.
- Re-running discovery may start a stopped server. Address changes require
  graceful shutdown before binding the newly validated address.
- IPv6, MagicDNS-only URLs, remote editing, and delta history are deferred.

## Ready Checklist

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task can move to `ready`.
