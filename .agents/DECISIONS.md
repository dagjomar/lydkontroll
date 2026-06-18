# Decision Log

Record choices that future work should not have to rediscover. Use the next
sequential ID and link the relevant task or plan.

## ADR-001: Use Kira behind an audio backend port

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-001
- **Context:** The product needs simultaneous MP3/WAV playback, per-instance
  control, master volume, deterministic stop/retrigger behavior, and smooth
  fades through the macOS default output.
- **Decision:** Use Kira with its CPAL backend and Symphonia decoding. Keep Kira
  types inside an `AudioBackend` adapter; test domain behavior with a fake
  backend and clock.
- **Consequences:** Built-in tweens and mixer tracks reduce custom real-time
  code. Resource limits, output loss, and actual analog playback still require
  explicit handling and target-Mac checks.
- **Alternatives:** Rodio was rejected because fades would need an
  application-owned volume scheduler. Raw CPAL/Symphonia was rejected because
  it requires unnecessary custom decoding, mixing, and callback code.

## ADR-002: Store versioned JSON and managed audio in app data

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-001
- **Context:** A single operator needs a small cue/scene aggregate that survives
  source-file moves and can be inspected and recovered before an event.
- **Decision:** Store `library.json` with an explicit schema version under
  Tauri's app-data directory, keep `library.json.bak`, and copy validated media
  into an `audio/` directory using stable IDs and atomic staging/rename steps.
- **Consequences:** Persistence stays simple and recoverable, but all writes
  must be serialized and migrations/corruption behavior must be tested.
- **Alternatives:** SQLite was rejected because there is no relational query or
  concurrency need. Tauri Store was rejected as a domain repository because it
  does not address migrations or managed media import.

## ADR-003: Rust owns protocol types and authoritative state

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-001
- **Context:** Tauri and WebSocket clients must invoke identical behavior and
  cannot safely maintain duplicate command/state definitions.
- **Decision:** Define Serde-compatible commands, acknowledgements, typed
  errors, and snapshots in Rust; derive committed TypeScript bindings with
  ts-rs. Route both transports through one serialized `ApplicationService`.
- **Consequences:** Protocol drift becomes testable. Rust types must use
  ts-rs-compatible Serde representations, and generated files must never be
  hand edited.
- **Alternatives:** Handwritten TypeScript mirrors were rejected because they
  drift. Transport-specific business logic was rejected because retries and
  playback rules would diverge.

## ADR-004: Run Axum on Tauri's async runtime

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-001
- **Context:** The app must serve embedded mobile assets and a WebSocket while
  sharing state with local Tauri commands and preserving playback on network
  failure.
- **Decision:** Run an Axum listener as a managed background task on Tauri's
  Tokio-compatible async runtime. Share the application service by `Arc`, use
  graceful shutdown/rebind, and embed production web assets in the Rust binary.
- **Consequences:** One process and one state core simplify consistency.
  Listener lifecycle and packaging of generated assets need explicit tests.
- **Alternatives:** A sidecar was rejected because it complicates packaging and
  lifecycle. A custom Hyper/Tungstenite stack was rejected because Axum already
  provides the required routing, WebSocket, state, and test seams.

## ADR-005: Discover Tailscale by CLI and fail closed

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-001
- **Context:** Tailscale is the version-one access boundary, so binding an
  ordinary LAN or wildcard address would be a security defect.
- **Decision:** Invoke `tailscale ip -4` with a timeout through an injectable
  process runner, parse exactly one address, require `100.64.0.0/10`, confirm it
  is assigned locally, and bind only that address on port `17321`. Missing CLI,
  invalid output, no address, or bind failure leaves the app in local-only mode
  with a typed preflight error.
- **Consequences:** Discovery follows Tailscale's authoritative view and never
  falls back insecurely. Actual packaged CLI locations must be verified on the
  event Mac. IPv6 is deferred.
- **Alternatives:** Interface-name guessing and CIDR-only scanning were rejected
  because macOS installation modes and interface names can vary. Wildcard or
  MagicDNS binds were rejected because they weaken the access boundary.

## ADR-006: Test layers instead of native macOS WebDriver

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-001
- **Context:** Tauri's native desktop WebDriver support excludes macOS, while
  the target includes both a Tauri desktop shell and iPhone Safari.
- **Decision:** Use Vitest/React Testing Library for frontend behavior, Rust
  unit/integration tests and Tauri's mock runtime for backend/boundary behavior,
  and Playwright against the standalone served UI in Chromium and WebKit Mobile
  Safari projects. Keep native Mac launch, real iPhone, Tailscale, and analog
  audio as documented manual gates.
- **Consequences:** Most behavior is automated without pretending Playwright is
  a native Tauri test. Release readiness still depends on repeatable hardware
  rehearsal evidence.
- **Alternatives:** Tauri desktop WebDriver on macOS is unavailable. UI-only
  tests were rejected because they cannot validate audio, persistence, or bind
  safety.

## ADR-007: Serialize playback semantics above Kira

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-004
- **Context:** Kira provides playback handles and tweens but does not define
  wedding-specific overlap, exclusive, retrigger, or authoritative-state rules.
- **Decision:** Own these rules in a mutable `PlaybackEngine` behind a narrow
  `AudioBackend` port. Retrigger immediately stops the old cue instance;
  exclusive playback starts only after every active instance completes its
  configured fade; a newer pending exclusive supersedes the older request; and
  backend completion events finalize state.
- **Consequences:** Ordering is deterministic under hardware-free tests, and
  the future application service can serialize commands around this engine.
  Kira remains responsible for decoding, mixing, CPAL output, and sample-level
  linear tweens. Real analog output and output-loss recovery remain release
  rehearsal gates.
- **Alternatives:** Encoding product rules directly in Kira handles was
  rejected because it couples authoritative state to an adapter. Timer-based
  fade completion was rejected because wall-clock guesses can diverge from the
  actual audio backend.

## ADR-008: Serialize commands, polling, revisions, and retries in one service

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-005
- **Context:** Tauri and future WebSocket callers can race with each other and
  with audio-backend completion polling. Retried mobile commands must not
  replay playback side effects.
- **Decision:** Share one `ApplicationService` through `Arc` and protect its
  complete mutable state with one mutex. Retain the 256 most recent command
  acknowledgements in FIFO order, increment one monotonic revision per
  snapshot-visible transition, publish complete snapshots after transitions,
  and retain at most 64 recoverable operator errors.
- **Consequences:** Commands and backend events have one deterministic order,
  retries are idempotent inside a bounded window, and transports remain thin.
  Slow adapter work must stay outside the service lock, and mutex poisoning is
  reported as a typed unavailable error.
- **Alternatives:** A Tokio actor was deferred because the current ports are
  synchronous and do not need runtime lifecycle. Per-transport locks and retry
  caches were rejected because behavior would diverge.
