# Plan: TASK-001 — Foundation architecture

Status: accepted
Updated: 2026-06-18

## Problem

The scaffold needs explicit choices for audio, persistence, networking, shared
types, Tailscale discovery, and testing. These choices cross most later tasks
and are costly to reverse.

The design must keep playback independent of React and network connectivity,
fail closed when Tailscale is unavailable, and remain testable without audio
hardware or a native macOS WebDriver.

## Current Evidence

- `PLAN.md` defines the product behavior and Apple Silicon macOS/iPhone Safari
  targets.
- `.agents/PROJECT.md` requires Rust to own playback, persistence, networking,
  commands, and authoritative state.
- [Kira 0.12](https://docs.rs/kira/0.12.1/kira/) provides CPAL-backed playback,
  Symphonia decoding for MP3/WAV, simultaneous sounds, mixer tracks, and
  first-class tweened parameter changes.
- [Rodio 0.22](https://docs.rs/rodio/0.22.2/rodio/) also decodes and mixes
  MP3/WAV and exposes controllable players, but smooth fades would require an
  application-owned scheduler repeatedly changing player volume.
- [CPAL](https://docs.rs/cpal/0.18.1/cpal/) exposes the system default output
  device and reports output-device/configuration failures, but is intentionally
  lower level than the application needs.
- Tauri 2 exposes an application data directory through
  `PathResolver::app_data_dir`, managed state, a mock runtime for Rust tests,
  and an async runtime with `spawn`/`spawn_blocking`.
- [Axum 0.8](https://docs.rs/axum/0.8.9/axum/) is Tokio-native and supports
  HTTP, WebSocket upgrades, shared state, and Tower middleware.
- [ts-rs 12](https://docs.rs/ts-rs/12.0.1/ts_rs/) derives TypeScript bindings
  from Serde-compatible Rust structs/enums and can export them during tests.
- The official Tailscale CLI exposes the current node address with
  [`tailscale ip -4`](https://tailscale.com/docs/reference/tailscale-cli#ip).
  It also documents waiting for an address before a service binds.
- Tauri documents that native desktop WebDriver testing is unavailable on
  macOS. Playwright can run the web UI in WebKit with an emulated Mobile Safari
  viewport.

Library versions above are evidence as of 2026-06-18, not a requirement to
float dependencies. `TASK-002` will select compatible versions and commit lock
files.

## Open Questions

- Exact exclusive-cue fade ordering, cancellation, and completion semantics are
  deferred to the required `TASK-004` audio-engine plan.
- The exact WebSocket retry window and bounded command-deduplication capacity
  are deferred to `TASK-006`, after the application service exists.
- Tailscale CLI discovery must be verified on the actual Mac installation
  during `TASK-006`; the adapter will support explicit executable candidates
  and expose a typed preflight error rather than silently falling back.

No open question blocks the scaffold.

## Options Considered

### Audio: Kira over CPAL

- Benefits:
  - Built-in tweens match cue fade, fade-all, and exclusive-transition needs.
  - Static and streaming sounds cover short effects and longer music.
  - Default CPAL backend follows the selected macOS system output.
  - Per-sound handles and mixer tracks map naturally to active instances and
    master volume.
- Costs and risks:
  - It is game-audio oriented and has resource-capacity limits that must be
    configured and surfaced as typed errors.
  - Hardware behavior and output-device loss still require target-Mac tests.

Selected. Wrap Kira behind an application-owned `AudioBackend` port so domain
semantics and tests do not depend on Kira types.

### Audio alternatives

- Rodio: capable and simpler, but the application would own fade timing and
  cancellation, increasing event-critical scheduling code.
- Raw CPAL plus Symphonia: maximum control, but requires custom decoding,
  buffering, mixing, and real-time callback work that the product does not
  justify.

### Persistence: versioned JSON plus managed files

- Benefits:
  - Human-inspectable and easy to back up/recover for a single operator.
  - The current data is one small scene/cue aggregate with no query workload.
  - Serde types can also generate the frontend contract.
- Costs and risks:
  - Writes must be atomic and schema migration must be explicit.
  - Concurrent writers would be unsafe, so all mutations must pass through one
    Rust application service.

Selected over SQLite and the Tauri Store plugin. SQLite adds migration and
query complexity without a current relational need; Store is not the domain
repository and does not solve managed-audio import.

### Server: Axum in the Tauri process

- Benefits:
  - Tokio-native HTTP/WebSocket stack with testable routers and Tower layers.
  - Can share an `Arc<ApplicationService>` with Tauri command adapters.
  - Playback remains in-process and independent of client connections.
- Costs and risks:
  - Startup, shutdown, and rebind lifecycle must be explicit.
  - Static mobile assets need a packaging strategy.

Selected. A background task runs on Tauri's async runtime and owns an Axum
listener bound to one explicit Tailscale socket address. Mobile assets will be
embedded in the Rust binary from the frontend production build so runtime file
paths cannot break the control page.

### Shared types: Rust Serde types plus ts-rs

- Benefits:
  - Rust remains the source of truth for commands, acknowledgements, snapshots,
    errors, and persisted data.
  - Generated TypeScript is inspectable and can be checked for drift in tests.
  - Serde tagging/renaming controls both transports consistently.
- Costs and risks:
  - Unsupported Serde attributes require explicit ts-rs annotations.
  - Generation must be deterministic and committed.

Selected over handwritten duplicate interfaces and broader command-generation
frameworks. Tauri command wrappers stay thin and invoke the same application
service used by Axum.

### Tailscale discovery: authoritative CLI plus local-address validation

- Benefits:
  - Uses Tailscale's own view of the current node address.
  - Easy to inject and test as a process-runner port.
  - Failure is visible and can refuse server startup.
- Costs and risks:
  - Packaged macOS installations may expose the CLI at different executable
    paths.
  - Subprocess execution needs a timeout and typed diagnostics.

Selected. Run `tailscale ip -4` with a short timeout, require exactly one valid
IPv4 address in `100.64.0.0/10`, cross-check that it is assigned locally, and
bind exactly `<address>:17321`. Never substitute wildcard, loopback, LAN, or
hostname binds. IPv6 is deferred until it is needed and tested.

### Test stack

- Frontend unit/component: Vitest, React Testing Library, `user-event`, and
  jsdom.
- Rust: built-in unit/integration tests, `tempfile` for filesystem isolation,
  Tokio paused time where useful, and explicit fakes for audio, clock,
  process-running, and event publication.
- Contract: ts-rs export tests plus a clean-tree/generated-diff check.
- Browser: Playwright against the standalone served web UI, with desktop
  Chromium and WebKit Mobile Safari projects. This does not claim to be native
  Tauri end-to-end coverage.
- Tauri boundary: mock-runtime tests for command wiring; manual launch on the
  supported Mac because macOS has no Tauri desktop WebDriver.
- Hardware/network: manual iPhone/Tailscale/analog-output rehearsal remains a
  release gate.

## Decision

Accepted choices are recorded as ADR-001 through ADR-006 in
`../DECISIONS.md`.

## Module Boundaries

The Rust crate should keep Tauri's entry point small and use these logical
modules:

```text
src-tauri/src/
  domain/         IDs, scene/cue data, commands, state, typed errors
  application/    serialized command service, deduplication, state revisions
  ports/          AudioBackend, Repository, Clock, TailscaleProbe, EventSink
  adapters/
    audio/        Kira/CPAL implementation
    persistence/  versioned JSON and managed-file import
    network/      Tailscale CLI probe and Axum HTTP/WebSocket server
    tauri/        command and lifecycle adapters
  lib.rs          composition root used by the app and tests
  main.rs         generated/minimal executable entry
```

The frontend should use:

```text
src/
  components/     reusable presentation controls
  features/       desktop editor, local control, mobile control, preflight
  generated/      committed ts-rs output; never hand edited
  services/       typed command/state client interfaces and adapters
  state/          client projection only, never playback authority
```

Domain and application modules must not import Tauri, Axum, Kira, or React.
Adapters may depend inward; domain code never depends outward.

## Data and Storage Layout

Under Tauri's `app_data_dir`:

```text
library.json
library.json.bak
audio/
  <audio-id>.<validated-extension>
staging/
```

`library.json` is a root object containing `schemaVersion: 1`, scenes, cues, and
managed audio metadata. Stable UUIDs identify scenes, cues, audio assets,
playback instances, and commands. Persist relative managed filenames, never
source paths.

Import copies into `staging/`, validates extension and decode metadata, flushes
the file, then atomically renames it into `audio/`. Saving serializes to a
temporary sibling, flushes it, rotates the previous valid file to
`library.json.bak`, and atomically replaces `library.json`. Startup validates
the schema and references before publishing state. Unknown schema versions and
corrupt JSON are typed recoverable errors; the backup is offered for recovery,
not silently substituted.

## Command and State Model

All control inputs use one Rust application method conceptually equivalent to:

```text
execute(CommandEnvelope) -> CommandAck
snapshot() -> AppSnapshot
subscribe() -> ordered AppSnapshot updates
```

`CommandEnvelope` carries `protocolVersion`, `commandId`, and a tagged command.
`CommandAck` echoes the ID and contains either the resulting state revision or
a typed error. `AppSnapshot` contains a monotonically increasing revision,
scene/cue data, active playback instances, master volume, connection/preflight
facts, and recoverable operator errors.

The application service serializes state-changing commands. A bounded
deduplication cache returns the previous acknowledgement for a retried
`commandId` without replaying side effects. Each accepted state change
increments the revision and publishes a fresh authoritative snapshot. Desktop
Tauri commands and WebSocket handlers are adapters around this same service.

React may optimistically show button press affordance, but it does not invent
active-playback state. Scene selection is UI/application state and never stops
already-playing instances.

## Audio Strategy

The Kira adapter owns one long-lived `AudioManager` using the default CPAL
output. Each trigger creates a new playback-instance ID and sound handle;
retriggering the same cue first transitions the prior instance according to the
`TASK-004` policy, then creates a new instance. A mixer track controls master
volume. Individual cue volume is applied per handle.

Fades use Kira tweens rather than a Tokio sleep loop. The application state
machine owns intent and ordering; Kira owns sample-level interpolation. A
completion monitor converts sound completion, stop completion, and backend
errors into serialized application events. Loss of the phone or WebSocket never
owns or drops the audio manager.

`TASK-004` must define exact exclusive ordering, tween curve, cancellation,
output-loss recovery, and how status polling is clocked before implementation.

## Server and Tailscale Lifecycle

On application startup:

1. Start the application/audio core independently.
2. Probe `tailscale ip -4` through the injected process runner.
3. Parse and validate one Tailscale IPv4 address and confirm it is local.
4. Bind an Axum listener to that address and port `17321`.
5. Publish the resulting URL/QR payload or a typed unavailable reason.

Server failure never shuts down local playback. If Tailscale is unavailable,
the desktop remains fully operational and preflight is red. A later successful
probe may start/rebind the server; address changes stop the old listener with
graceful shutdown before binding the new explicit address.

HTTP serves embedded production assets and a health/snapshot bootstrap route.
WebSocket connection startup sends the current snapshot. Every command receives
an acknowledgement and state revision; reconnect starts from a fresh snapshot,
not a client delta history.

## Data Flow

```text
React desktop ──Tauri command──┐
                              ├─> ApplicationService ─> domain state
iPhone Safari ──WebSocket──────┘          │
                                         ├─> Repository adapter
                                         ├─> AudioBackend adapter
                                         └─> snapshot/event publisher

publisher ──Tauri event/state query──> desktop projection
publisher ──WebSocket snapshot───────> mobile projection
```

Neither transport invokes Kira or persistence directly.

## Implementation Slices

### TASK-002: runnable shell

1. Scaffold Tauri 2, Vite, React, and strict TypeScript with npm lock files.
2. Add ESLint, Prettier, rustfmt/clippy commands, Vitest, React Testing Library,
   and one frontend behavior test.
3. Make Rust a testable library plus minimal executable; create empty
   domain/application/ports/adapters module boundaries and one mock-runtime
   smoke test without production behavior.
4. Establish ts-rs export location and a generated-type smoke contract.
5. Add documented build/test/lint commands and ignore generated bundles/local
   app data.

### TASK-003: persistence vertical slice

1. Define version-one Rust domain data and generated TypeScript bindings.
2. Implement repository load/save with atomic replacement and backup behavior.
3. Implement staged MP3/WAV import, decode validation, and managed IDs.
4. Add migration/corruption/missing-file/import integration tests using a
   temporary app-data root.

### TASK-004: audio vertical slice

1. Complete the linked engine plan with deterministic command ordering.
2. Implement and test the domain state machine with a fake audio backend/clock.
3. Implement the Kira adapter for streaming MP3/WAV, handles, tracks, and
   tweens.
4. Add target-Mac manual output and failure checks.

### TASK-005 and TASK-006: shared core and transport

1. Add the serialized application service, revisions, deduplication, and typed
   acknowledgements.
2. Wire thin Tauri command/state adapters.
3. Complete the Tailscale threat/failure plan.
4. Add the CLI probe, explicit bind, Axum routes/WebSocket, embedded assets,
   graceful lifecycle, and reconnect tests.

## Test Strategy

- Automated:
  - Domain state transitions with fake audio/clock/event ports.
  - JSON round trips, migrations, interrupted writes, corrupt primary/valid
    backup, managed import, missing files, and invalid decodes.
  - Generated TypeScript contract drift.
  - Application command deduplication, ordering, revisions, and typed errors.
  - Tailscale parser/timeout/path failures and rejection of non-Tailscale,
    wildcard, loopback, and non-local addresses.
  - Axum router, WebSocket acknowledgement, snapshot, disconnect, and reconnect
    tests without a Tauri window.
  - React behavior tests and Playwright WebKit mobile flows.
- Manual:
  - Launch the Tauri shell on Apple Silicon macOS.
  - Play representative MP3/WAV cues through the selected analog output.
  - Change/disconnect output and confirm visible failure behavior.
  - Connect an iPhone through Tailscale, then change Wi-Fi/mobile connectivity
    while playback continues.
- Important failure modes:
  - Decoder/import errors, missing managed files, corrupt or future schemas.
  - App interruption during import/save.
  - Output unavailable or lost during playback.
  - Resource capacity exhausted, duplicate triggers, cancelled fades.
  - Tailscale executable missing, command timeout, no IP, wrong IP, bind race,
    address change, and port conflict.
  - Duplicate/reordered WebSocket commands, stale snapshots, disconnects, and
    reconnects.

## Rollback or Recovery

- Kira, JSON storage, Tailscale probing, and Axum are adapters behind narrow
  ports. Replacing one should not change command or domain semantics.
- Keep the previous valid `library.json.bak`; never silently overwrite both the
  primary and recovery copy in one step.
- A transport startup failure degrades to local-only control and visible
  preflight failure, not application exit.
- Pin resolved dependencies in lock files. Upgrade one subsystem at a time and
  rerun its adapter, contract, and manual checks.

## Ready Checklist

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task can move to `done`, and `TASK-002` can move to `ready`.
