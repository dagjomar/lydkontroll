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

## ADR-009: Persist desktop library candidates before publishing them

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-007
- **Context:** The desktop editor must import managed files and mutate scenes
  and cues without making React authoritative or exposing unsaved state to
  playback and future remote subscribers.
- **Decision:** Serialize local save/import operations in a thin Tauri-managed
  desktop coordinator. Save a complete candidate through
  `JsonLibraryRepository` first, then replace the library held by
  `ApplicationService` and publish its revisioned snapshot. Use Tauri's native
  dialog plugin only to choose an MP3/WAV source path; Rust performs validation,
  copying, metadata creation, and persistence.
- **Consequences:** Failed writes leave the previous authoritative snapshot
  intact, and desktop playback still uses the shared command path. Complete
  library saves are intentionally coarse-grained; if remote editing is added,
  mutations should become versioned service commands with conflict handling.
- **Alternatives:** React-owned persistence was rejected because it duplicates
  authoritative state. Publishing edits before saving was rejected because
  playback could reference data that does not survive restart.

## ADR-010: Project mobile controls from fresh WebSocket snapshots

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-008
- **Context:** The embedded production frontend serves both Tauri and iPhone
  Safari. Mobile controls need visible connection certainty without becoming a
  second owner of playback state.
- **Decision:** Select a dedicated read-only mobile projection outside Tauri.
  Expose connection state from the WebSocket adapter, invalidate cached state
  on disconnect, accept the fresh snapshot from each new socket generation,
  ignore stale socket events, and suppress duplicate in-flight UI actions.
- **Consequences:** Safari always presents Rust-owned state and explicit
  connection/acknowledgement status. Mobile editing and automatic command retry
  remain out of scope; real network transitions still require rehearsal.
- **Alternatives:** Reusing the desktop editor was rejected as unsafe and
  touch-hostile. Keeping stale snapshots active while reconnecting was rejected
  because operators could mistake old playback state for current state.

## ADR-011: Use an expanding now-playing overlay on iPhone

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-013
- **Context:** A fixed playback region preserved cue coordinates but consumed
  scarce vertical space and clipped multiple active sounds. Three functional
  alternatives were tested at 390x844: overlay, global controls below cues, and
  separate cues/status tabs.
- **Decision:** Use the expanding overlay as the production iPhone layout. Show
  it only while playback is active, allow it to grow for multiple sounds up to
  the viewport limit, and keep the underlying cue screen interactive.
- **Consequences:** The live cue screen remains dense and global controls stay
  immediately available. The overlay may cover header/status content while
  active, which is acceptable because playback actions take temporary
  priority. Prototype switching and discarded layouts do not ship.
- **Alternatives:** Controls below cues made emergency actions slower to reach.
  Tabs created a useful future settings boundary but added navigation during
  live operation. The fixed region wasted too much permanent vertical space.

## ADR-012: Refresh preflight facts in Rust and render operator guidance

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-009
- **Context:** Event readiness combines managed-file storage, audio backend
  initialization, fail-closed Tailscale binding, mobile access, and a safe
  playback check. React cannot inspect those adapter-owned facts reliably.
- **Decision:** Add a desktop refresh boundary that recomputes managed-file
  diagnostics in Rust and republishes one revisioned `PreflightFacts`
  projection. Represent manual checks as warnings, keep hard failures
  unavailable, and let React render the operator view, local QR code, and a
  three-second instance-scoped test play through the normal command path.
- **Consequences:** Readiness severity stays consistent with authoritative
  state and missing files can name affected cues. Version one gives a clear
  manual output-selection step instead of claiming a device name the current
  adapter cannot expose. Automatic Tailscale rebinding remains release
  hardening work.
- **Alternatives:** React-owned file and network checks were rejected because
  they duplicate adapter truth. Treating manual output verification as ready
  was rejected because it hides an event-critical human check.

## ADR-013: Gate the local release with recorded target-hardware rehearsal

- **Date:** 2026-06-18
- **Status:** accepted
- **Task:** TASK-010
- **Context:** The event uses one known Apple Silicon Mac, iPhone, Tailscale
  accounts, and analog output. Several critical behaviors cannot be accepted
  through mocks, while signing/notarization would add credentials and
  distribution scope that the known-Mac workflow does not require.
- **Decision:** Build an unsigned local `.app` from committed npm/Cargo locks,
  record its commit, version, path, and SHA-256, then gate release on a
  documented 60-minute rehearsal using the production cue library and exact
  event hardware. Deliberately exercise phone, cellular/Tailscale, Mac network,
  and analog-output loss. Use local Mac control as the primary mobile/network
  fallback and keep the previous rehearsed build until its replacement passes.
- **Consequences:** Release evidence is reproducible and tied to one artifact,
  and hardware-only risks remain visible instead of being implied by automated
  tests. The app is not prepared for public distribution and may require local
  Gatekeeper approval if moved to a different Mac.
- **Alternatives:** An informal final check was rejected because it provides no
  stable gates or evidence. Signing and notarization were deferred because they
  require Apple credentials without improving the agreed known-event-Mac path.
