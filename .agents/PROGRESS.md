# Progress Log

## 2026-07-04 — README presentation task expanded

- Updated existing P1 `TASK-031` instead of creating a duplicate.
- Added final, event-generic Mac and iPhone screenshots, useful alt text, and
  reasonable repository-page weight to the existing Pages-link outcome.
- Kept the dependency on `TASK-030` so temporary retired-name mockups are not
  promoted into the README.

## 2026-07-04 — README marketing-site link captured

- Added P1 `TASK-031` for a prominent README link to the canonical
  GitHub Pages marketing site.
- Made it depend on `TASK-030` so the repository never advertises an unverified
  or unavailable URL.
- Scoped the change to a concise link while preserving technical, limitation,
  license, support, and security information.

## 2026-07-04 — TASK-030 website concept integrated

- Imported the owner-approved Stitch/AI Studio concept into an isolated
  `website/` Vite package without coupling it to the Tauri frontend.
- Downloaded all seven Google-hosted mockups into tracked static assets and
  removed Gemini, Express, dotenv, AI Studio environment, and language-switch
  dependencies from the launch path.
- Corrected repository links, Norwegian product claims, platform/Tailscale
  requirements, license language, metadata, focus treatment, and reduced-motion
  behavior; added a reproducible GitHub Pages Actions workflow.
- Passed zero-vulnerability npm audit, TypeScript, production build, public
  checks, diff checks, and visual review at 1280x720 and 390x844 with no browser
  errors.
- Kept the task in progress: the mockups still visibly contain the retired
  `Marius + Wenche` event identity and must be replaced before public deploy.

## 2026-07-04 — TASK-029 marketing design handoff complete

- Added a self-contained Norwegian `DESIGN.md` covering audience, workflow,
  reliability principles, supported functionality, requirements, limitations,
  security boundary, and source-available license posture.
- Extracted the implemented warm-dark palette into semantic roles and recorded
  typography, shape, motion, accessibility, and neutral waveform constraints.
- Proposed a full site hierarchy and practical Mac/iPhone screenshot list, then
  added a standalone Google Stitch prompt that separates fixed product truth
  from legitimate design freedom.
- Manually compared claims with PLAN, README, ADR-014/ADR-015, current UI and
  icon sources; passed Prettier, `git diff --check`, and Ralph validation.
- Next step requires owner input: generate/review the Stitch concept before
  planning and implementing dependent `TASK-030`.

Append concise, dated entries. Keep detailed implementation notes in task files.

## 2026-06-18 — Project harness established

- Added the Ralph-loop workflow, task schema, validator, and session handoff.
- Distilled stable product constraints from `PLAN.md`.
- Created the initial dependency-ordered backlog.
- Next: plan `TASK-001`, then scaffold the application in `TASK-002`.

## 2026-06-18 — Foundation architecture accepted

- Completed `TASK-001` with an implementation-ready architecture, module/data
  flow, failure modes, recovery behavior, and dependency-ordered slices.
- Accepted ADR-001 through ADR-006 covering Kira/CPAL audio, versioned atomic
  JSON, Rust/ts-rs contracts, Axum/Tauri integration, fail-closed Tailscale
  discovery, and layered testing.
- Refined `TASK-002` through `TASK-006` to encode the accepted boundaries and
  observable acceptance criteria.
- Moved `TASK-002` to `ready`; Ralph validation passes.

## 2026-06-18 — Runnable Tauri shell completed

- Scaffolded Tauri 2, Vite, React, strict TypeScript, and the Rust
  domain/application/ports/adapters boundaries.
- Added frontend behavior coverage, a Tauri mock-runtime smoke test, and a
  deterministic committed ts-rs contract with drift detection.
- Added npm/Cargo lock files and unified ESLint, Prettier, rustfmt, Clippy,
  build, and test commands with Rust-1.85-aware dependency resolution.
- Validated all automated commands and launched the native macOS app; the user
  visually confirmed the shell.
- Next: implement `TASK-003` persistence and managed audio import.

## 2026-06-18 — Persistence and managed audio import completed

- Added schema-v1 scene, cue, and managed-audio Rust contracts with committed
  ts-rs TypeScript output and whole-directory drift detection.
- Implemented atomic JSON save/backup/recovery and injected app-data paths with
  Tauri resolution kept at the adapter boundary.
- Added staged MP3/WAV import with Symphonia frame decoding, fsync, UUID-backed
  managed filenames, and no persisted source paths.
- Added typed recovery errors and nine integration tests covering both formats,
  round trips, corruption, future schemas, interrupted writes, missing files,
  invalid references, and staging cleanup.
- Validated Rust/frontend tests, frontend build, bindings, formatting, lint,
  Clippy, and Ralph metadata.
- Next: refine the `TASK-004` audio-engine plan and move it to `ready`.

## 2026-06-18 — Deterministic local audio playback completed

- Accepted explicit retrigger, exclusive-barrier, cancellation, fade,
  completion, and failure semantics in the `TASK-004` plan and ADR-007.
- Added an application-owned `PlaybackEngine` plus narrow `AudioBackend` port,
  with eleven fake-backend tests independent of React, networking, clocks, and
  audio hardware.
- Added the Kira 0.12/CPAL production adapter for managed streaming MP3/WAV,
  per-cue/master volume, linear stop tweens, decoder failures, and completion
  polling.
- Documented the target-Mac analog/output-loss rehearsal and validated Rust and
  frontend tests, build, bindings, formatting, ESLint, and Clippy.
- Next: implement `TASK-005` authoritative commands, revisions,
  deduplication, and snapshots around the playback engine.

## 2026-06-18 — Authoritative command and state core completed

- Accepted ADR-008 and the `TASK-005` plan for mutex-serialized commands,
  polling, revisions, retry deduplication, snapshot publication, and bounded
  recoverable errors.
- Added versioned Rust/ts-rs commands, acknowledgements, failures, preflight
  facts, operator errors, playback projections, and complete snapshots.
- Added one `ApplicationService` shared through thin adapters, with cue lookup,
  managed-path resolution, deterministic playback mutation, backend polling,
  256-command FIFO deduplication, and revisioned subscribers.
- Added seven integration tests for serialized concurrent callers, retries and
  eviction, typed validation/backend failures, transition revisions, polling,
  preflight publication, and thin Tauri adapter use.
- Validated Rust/frontend tests, frontend build, generated binding drift,
  formatting, ESLint, and Clippy.
- Next: refine `TASK-006` and implement the fail-closed Tailscale/Axum transport.

## 2026-06-18 — Local Mac workflow promoted

- Reordered delivery so `TASK-007` desktop editing/local control is the next P0
  milestone and `TASK-006` remote transport follows at P1.
- Marked `TASK-007` `needs-planning` with an explicit end-to-end local
  checkpoint: import audio, create a cue, play it, and stop/fade it on the Mac
  without Tailscale.
- Kept the technical dependency graph honest; remote transport is postponed by
  priority, not coupled to the desktop UI.
- Next: plan and implement `TASK-007`.

## 2026-06-18 — Desktop local-control workflow completed

- Accepted the `TASK-007` plan and ADR-009: desktop import/save operations are
  serialized, persisted first, then published through `ApplicationService`.
- Added native MP3/WAV selection, concrete Tauri-managed service wiring,
  resilient audio startup, missing-file recovery, and revisioned managed-audio
  metadata in authoritative snapshots.
- Replaced the scaffold with a responsive operator UI for scene/cue CRUD and
  ordering, cue configuration, import, trigger, active playback, stop/fade,
  master volume, and visible recoverable errors.
- Added Rust service/persistence coverage, three critical frontend state-flow
  tests, and a standalone browser preview used for visual QA.
- Validated Cargo tests, Vitest, production build, ESLint, Prettier, generated
  bindings, rustfmt, Clippy, Ralph metadata, and diff hygiene.
- Next: refine and implement `TASK-006` Tailscale-only control transport.

## 2026-06-18 — Fail-closed Tailscale control transport completed

- Accepted the `TASK-006` threat/failure plan for explicit CLI candidates,
  bounded discovery, local CGNAT validation, listener lifecycle, embedded
  assets, reconnect, acknowledgements, and QR URL data.
- Added typed `tailscale ip -4` discovery with a three-second timeout and no
  wildcard, loopback, LAN, or invalid-address fallback.
- Added a gracefully stoppable Axum server that serves Tauri's embedded
  production assets and routes WebSocket commands through the shared
  `ApplicationService`.
- Added initial/current snapshots, acknowledgements, retry idempotency,
  malformed-frame recovery, revision polling, reconnect coverage, bind
  refusal, and graceful listener release tests.
- Exposed generated `ControlServerInfo` to the desktop for mobile URL/QR
  presentation and made startup failure a visible local-only preflight state.
- Validated Cargo tests, Vitest, production build, generated bindings,
  formatting, ESLint, Clippy, Ralph metadata, and diff hygiene.
- Next: refine and implement `TASK-008` reconnecting iPhone controls.

## 2026-06-18 — Safari transport runtime corrected

- Reproduced the iPhone failure where the embedded frontend called Tauri's
  native `invoke` bridge from ordinary Safari.
- Added runtime selection so the native window keeps Tauri commands while
  Safari uses `/ws` for snapshots, acknowledgements, commands, timeouts, and
  reconnect backoff.
- Rebuilt the production assets and validated Vitest, TypeScript/Vite build,
  ESLint, Prettier, generated bindings, rustfmt, and Clippy.
- Added and tested a UUID-v4 fallback for Safari versions where
  `crypto.randomUUID` is unavailable.

## 2026-06-18 — Reconnecting iPhone controls completed

- Accepted ADR-010 and the `TASK-008` plan for a dedicated Safari projection
  driven only by fresh authoritative WebSocket snapshots.
- Added touch-safe scene/cue controls, active playback actions, master volume,
  stop/fade-all controls, and persistent connection/acknowledgement status.
- Invalidated cached state on disconnect, guarded replaced socket generations,
  and suppressed duplicate actions while acknowledgements are pending.
- Added component and adapter tests for disconnect, reconnect, stale sockets,
  authoritative replacement, and duplicate taps.
- Validated Vitest, production build, lint, Ralph metadata, diff hygiene, and a
  390x844 rendered cue interaction.
- Next: refine and implement `TASK-009` event preflight and diagnostics.

## 2026-06-18 — Mobile cue-layout safety follow-up added

- Added `TASK-011` from operator feedback: the conditional `Spiller nå` region
  shifts cue positions when playback starts and risks an accidental cue tap.
- Classified it as a ready P0 bug after `TASK-008`, ahead of preflight work.
- Defined a fixed-height idle/active playback region, compact touch-safe
  emergency controls, automated state coverage, and a 390x844 position check.
- Next: implement `TASK-011`, then return to `TASK-009`.

## 2026-06-18 — Stable iPhone cue layout completed

- Kept the `Spiller nå` panel mounted with a fixed outer height, a quiet idle
  state, and internal scrolling for multiple active sounds.
- Compacted emergency and playback actions while preserving measured 44-pixel
  touch targets.
- Added idle-to-playing-to-idle regression coverage and made the mobile preview
  simulate playback commands for rendered checks.
- At 390x844, verified the first cue remained at document position
  585.9296875 pixels through idle, playing, and stopped states.
- Validated Vitest, production build, ESLint, Prettier, generated bindings,
  rustfmt, Clippy, Ralph metadata, and diff hygiene.
- Next: refine `TASK-009` event preflight and operator diagnostics.

## 2026-06-18 — Three iPhone layout prototypes completed

- Added one-checkout query-selectable prototypes for an expanding now-playing
  overlay, cue-first controls below the cues, and separate cues/status tabs.
- Kept all variants on the same snapshot, WebSocket, command, acknowledgement,
  and reconnect path; query options also work against the real iPhone URL.
- Added a bottom comparison switcher, shared-command/layout tests, and a short
  operator guide in `MOBILE_LAYOUT_PROTOTYPES.md`.
- Reviewed all three at 390x844 with three active playback rows. Overlay
  maximized space, cue-first prioritized cue access but required scrolling for
  global controls, and tabs separated operation from setup most cleanly.
- Next: operator selects or combines a prototype, then `TASK-013` promotes it
  to the production layout before preflight work resumes.

## 2026-06-18 — Overlay iPhone layout adopted

- Recorded ADR-011 after the operator selected the expanding now-playing
  overlay from the three functional prototypes.
- Made overlay the sole production mobile layout and removed query selection,
  the comparison switcher, discarded cue-first/tab code, and the prototype
  guide.
- Capped the overlay below the cue grid and scroll only its active list in
  crowded states, preserving direct cue interaction underneath.
- At 390x844 with nine active rows, verified a 321-pixel overlay, internal
  scrolling, an uncovered and triggerable first cue, and 44-pixel action
  targets.
- Next: refine `TASK-009` event preflight and operator diagnostics.

## 2026-06-18 — Event preflight and operator diagnostics completed

- Accepted ADR-012 and the `TASK-009` plan for Rust-owned refreshed readiness
  facts with React-owned operator guidance and safe actions.
- Added cue-specific managed-file blockers, distinct manual warnings, current
  Tailscale server address, mobile URL, and a locally generated QR code.
- Added a safe test-play workflow that requires idle playback, triggers a saved
  cue through the shared command path, and fades only its own instance after
  three seconds.
- Added Rust and React regression coverage for file diagnostics, severity,
  mobile access, and bounded test playback.
- Verified the preflight layout at 1280px and 760px without horizontal
  overflow.
- Validated Vitest, Cargo tests, production build, ESLint, Prettier, generated
  bindings, rustfmt, Clippy, Ralph metadata, and diff hygiene.
- Next: refine `TASK-010` release rehearsal and recovery planning.

## 2026-06-18 — Release automation and runbook completed

- Accepted ADR-013 and the `TASK-010` plan for a locked local Apple Silicon
  build gated by recorded target-hardware rehearsal.
- Added one `npm run release:build` command for frontend/Rust checks, tracked
  private-artifact validation, and Tauri packaging.
- Enabled `.app` bundling and fixed the macOS icon requirement with a generated
  `.icns`; the resulting 5.2 MB bundle contains a thin arm64 executable.
- Added build identity, 60-minute failure-injection rehearsal, recovery, and
  event-day checklists in `RELEASE_RUNBOOK.md`.
- Validated the complete release command, all Rust/frontend tests, Clippy,
  formatting, generated bindings, tracked-file safety, Ralph metadata, and
  diff hygiene.
- Next: run and record the rehearsal on the event Mac, iPhone cellular/Tailscale
  path, production cue library, and analog output.

## 2026-06-18 — Target-hardware rehearsal passed

- Recorded the operator-confirmed manual rehearsal against commit
  `885422112d554da5f22001a39115d2afcdf30e46`, app version `0.1.0`, and the
  arm64 executable checksum in `.agents/REHEARSAL.md`.
- Every release-runbook gate passed, including the 60-minute run, real
  MP3/WAV playback, phone and network failure injection, Mac-local fallback,
  analog cable recovery, and macOS output recovery.
- Closed `TASK-010`; there are no remaining implementation or release blockers.
- Next: preserve and operate the rehearsed candidate; rebuild and rehearse
  before using any changed release.

## 2026-06-18 — Compact desktop preflight control completed

- Replaced the permanently expanded desktop preflight area with a compact
  status pill that leaves the cue workspace unobstructed.
- The status light is green when there are no blocking readiness failures and
  red when an unavailable check needs attention.
- The existing refresh, diagnostics, QR access, and safe test playback remain
  available by expanding the control.
- Added regression coverage for collapsed ready/blocked states and preserved
  panel behavior; frontend tests, build, lint, bindings, rustfmt, and Clippy
  pass.
- Next: build and briefly rehearse this changed candidate before replacing the
  previously rehearsed event build.

## 2026-06-18 — Compact-status replacement candidate built

- Added `TASK-015` so the release action left in state/handoff is represented
  in the Ralph task graph.
- Passed `npm run release:build`, including frontend tests/build/lint, generated
  binding checks, Rust tests and Clippy, tracked-file safety, and arm64 Tauri
  packaging. Restricted mode initially denied the expected loopback test binds;
  the same gate passed with local bind permission.
- Added explicit regression coverage for the expanded panel's refresh action;
  the focused shell suite passes all six tests covering status states,
  expansion, QR, refresh, and the three-second trigger/fade sequence.
- Recorded source commit, version, bundle path, architecture, and executable
  checksum in `.agents/CANDIDATE.md`.
- Next: the user performs the short native launch and audible test-play check
  in `TASK-016`; keep the previously rehearsed candidate until it passes.

## 2026-06-19 — Smooth iPhone master volume candidate built

- Fixed Safari's interrupted master-volume drag by keeping the thumb local and
  enabled while commands are acknowledged.
- Serialized volume writes to one in flight and coalesced rapid input to the
  latest queued value while retaining authoritative revision reconciliation.
- Added regressions for continuous input, bounded command traffic, latest-value
  delivery, post-ack stability, and external idle updates.
- Passed 14 frontend tests, 63 Rust tests, production build, lint, generated
  bindings, rustfmt, Clippy, release-tree safety, arm64 packaging, and a 390x844
  responsive render check.
- Replaced `.agents/CANDIDATE.md` with commit and checksum identity for the new
  bundle.
- Next: run the five-point `TASK-016` smoke check on the event Mac and iPhone.

## 2026-06-25 — Audio and managed-file feedback triaged

- Deduplicated the feedback into two independently schedulable results.
- Added P0 bug `TASK-019` for controlled diagnosis of phase-sensitive or
  channel-cancelled playback through the event mixer's line input.
- Recorded that the current Kira adapter uses the selected macOS output and
  applies volume only; it does not invert, downmix, or M/S-process channels.
- Left `TASK-019` in needs-planning with a draft evidence-first plan because
  the correction scope depends on file analysis and the exact cable/mixer path.
- Added P1 ready feature `TASK-020` for safe deletion of unreferenced managed
  audio, explicitly blocking deletion of files still used by cues.
- Next: claim and refine `TASK-019` using an affected file, a known-good file,
  and the exact event hardware path; do not add a speculative global toggle.

## 2026-06-25 — Phase-sensitive playback resolved in source preparation

- User confirmed the corrected Logic export plays perfectly through the app.
- The earlier failed test had played the old managed audio because the changed
  cue configuration had not been saved.
- Closed `TASK-019` with no product-code change and no phase/M/S toggle.
- Recorded the operator workflow: correct and render in Logic, import/select
  the new file, save the configuration, then test the saved cue.
- Removed `TASK-019` from the `TASK-016` release-smoke dependency chain.
- Next: complete the native desktop, audible-output, and iPhone Safari checks
  in `TASK-016`.

## 2026-06-25 — Safe managed-audio deletion completed

- Completed `TASK-020` with a collapsed desktop file-management section that
  identifies duplicate-name imports by format, size, and managed filename.
- Files referenced by saved or draft cues are disabled and list every blocking
  cue; eligible files require explicit confirmation.
- Added a staged repository transaction that removes metadata and managed bytes
  together and restores the original file when metadata persistence fails.
- Deletion now refreshes the authoritative snapshot and managed-file preflight.
- Added persistence regressions for referenced-file blocking, restart
  durability, physical file removal, and rollback, plus UI regressions for
  duplicate names, confirmation, cancellation, and cue blockers.
- Passed `cargo test --manifest-path src-tauri/Cargo.toml`,
  `npm test -- --run`, `npm run build`, and `npm run lint`.
- The in-app browser connection was unavailable, so visual preview automation
  was not repeated; component tests cover the new disclosure and actions.
- Next: complete the physical Mac/iPhone candidate check in `TASK-016`.

## 2026-07-03 — Public-launch feedback triaged

- Compared the new feedback with PLAN.md, the full task graph, shipped product
  identity, icon assets, and ADR-013's explicit public-distribution deferral.
- Preserved the original name ideas (`The Wedding MC`, `The Toastmaster`,
  `Toastmastah`, and `SoundMastah`) in P1 research `TASK-021`; left it
  needs-planning because public positioning, title customization, and the heart
  logo are material product choices.
- Added fully scoped, dependency-blocked P1 feature `TASK-022` to apply the
  chosen identity consistently and preserve existing user data.
- Added independent ready P1 bug `TASK-023` for transparent rounded macOS icon
  corners while retaining the current heart/waveform artwork for now.
- Added P1 needs-planning chore `TASK-024` for history-wide privacy, licensing,
  docs, security, and source-versus-binary distribution readiness.
- Added fully scoped, dependency-blocked P2 chore `TASK-025` for the external
  GitHub repository creation/push after branding, icon, and public-readiness
  gates pass.
- No product code was implemented. Next: claim and refine `TASK-021`, prepare a
  concise naming shortlist, and ask only for the resulting brand choice.

## 2026-07-03 — Transparent macOS icon corners completed

- Completed `TASK-023` by regenerating all tracked platform icon derivatives
  from the existing rounded `icon.svg` while preserving the waveform heart.
- Added `npm run icons:generate` and a release-gated alpha regression check for
  tracked PNGs and all ten representations inside `icon.icns`.
- Confirmed every checked corner has alpha zero and the packaged app contains a
  byte-identical `.icns`; visually inspected the retained large-size artwork.
- Passed `npm run release:build` with loopback permission: 16 frontend tests,
  65 Rust tests, lint, bindings, release-tree checks, and macOS app bundling.
- Removed three trailing blank lines in existing Ralph records because the
  repository-wide Prettier gate flagged them.
- Next: refine `TASK-021` and obtain the product identity/customization choice.

## 2026-07-03 — TASK-021 naming research checkpoint

- Claimed `TASK-021` and checked the original candidates plus adjacent generic
  names for obvious web/app conflicts.
- Recorded why `The Wedding MC`, `The Toastmaster`, and `Toastmastah` should be
  rejected; retained `SoundMastah` only as a playful English alternate.
- Recommended retaining `Lydkontroll` as an event-generic public name, adding
  an optional event title with `Mitt arrangement` fallback, and revising the
  heart into a neutral rounded waveform mark.
- Paused before acceptance because public identity is an owner product choice.

## 2026-07-04 — TASK-021 and TASK-022 Norwegian-first reusable identity

- Accepted ADR-014: retain `Lydkontroll` as an event-generic Norwegian-first
  product, defer complete English localization and language selection to the
  newly refined `TASK-026`, and avoid international claims until it ships.
- Added configurable `eventTitle` to schema v1 with a serde default, preserving
  existing libraries and showing `Mitt arrangement` when missing or empty.
- Added desktop editing and authoritative mobile projection with regression
  coverage, while retaining the existing stable header/cue layout.
- Removed Marius/Wenche identity from shipped UI, metadata, package identity,
  tests, README, and PLAN; regenerated platform icons from a neutral waveform.
- Passed frontend, Rust, lint, build, binding, icon-alpha, identity-search, and
  Ralph validation. Next: plan `TASK-024` public-release safety decisions.

## 2026-07-04 — Optional operating-mode feedback captured

- Checked the existing backlog and separated this concern from `TASK-024`
  public-readiness documentation and `TASK-026` localization.
- Added P3 research `TASK-027` for eventual desktop-only, recommended
  Tailscale, and optional same-LAN operating modes without delaying the minimal
  first public release.
- Preserved the user's observation that Tailscale is unfamiliar, desktop-only
  use needs no network, and LAN mobile control is useful but loses reachability
  outside Wi-Fi.
- Left `TASK-027` needs-planning with explicit security, settings, onboarding,
  instructions, and marketing questions; automatic insecure LAN fallback is a
  non-goal.
- No product code was implemented. Next remains `TASK-024`; plan `TASK-027`
  after first-public-release work unless it is deliberately reprioritized.

## 2026-07-04 — Public-release history audit started

- Claimed planning work for `TASK-024` and audited the current tracked tree,
  commit paths, reachable objects, authorship, private-artifact patterns, and
  event-specific history.
- Found no committed audio, app/disk-image bundle, signing material, local cue
  library, or absolute user path in reachable history.
- Identified two irreversible publication disclosures: every commit contains
  the author's Gmail address, and older commits retain the retired
  Marius/Wenche wedding identity.
- Recommended a source-only first release and rejected an unsigned public app
  bundle. Left license and retained-versus-sanitized-history choices for the
  owner because they change legal rights and privacy exposure.
- Recorded the evidence in
  `.agents/audits/TASK-024-public-readiness-audit.md`; `TASK-024` remains
  needs-planning until those decisions are explicit.

## 2026-07-04 — TASK-024 public-release preparation complete

- Accepted ADR-015: source-only publication under PolyForm Noncommercial 1.0.0,
  separate owner-controlled commercial licensing, retained 31-commit history,
  and a CLA for commercial/relicensing rights in accepted contributions.
- Added the exact official license text, required notice, CLA, contribution
  guide, security policy, support boundary, and direct dependency-license
  review. Public copy clearly says source-available rather than OSI open source.
- Expanded README with desktop-only behavior, Tailscale trust boundary,
  architecture, limitations, source-only distribution, and commercial terms.
- Added `npm run public:check` to require public documents, scan current tracked
  files, and reject private artifact paths across all reachable Git history;
  expanded ignore/release checks for M4A, environment, signing, and library data.
- Passed `npm run release:check` with loopback permission: build, lint, 17
  frontend tests, 66 Rust tests, current bindings, icon alpha, 185 tracked-file
  checks, and 749 reachable object-path checks.
- Completed `TASK-024` and moved dependency-cleared `TASK-025` to ready. Remote
  creation still requires the owner's GitHub destination and authorization.

## 2026-07-04 — TASK-025 public GitHub publication complete

- Received explicit owner authorization and created public repository
  `dagjomar/lydkontroll` without initializing or replacing remote content.
- Pushed the audited retained `main` history and configured the event-generic
  description plus `tauri`, `rust`, `react`, `soundboard`, and `tailscale`
  topics; enabled issues and private vulnerability reporting and disabled wiki.
- Verified public visibility, `main` as default branch, the SSH `origin`, and
  the remote branch identity. No binary release or unsigned bundle was added.
- Canonical repository: https://github.com/dagjomar/lydkontroll
- Next independent slice: plan `TASK-026` English localization; the physical
  `TASK-016` candidate check remains truthfully blocked.

## 2026-07-04 — TASK-016 operational validation accepted

- The owner confirmed that the packaged app was used successfully in the real
  event setting on 2026-06-27.
- Treated that operational use as stronger evidence than the older manual smoke
  checklist and accepted the compact status, audible playback/fade, and iPhone
  control path without repeating the obsolete gate.
- Completed `TASK-016` and promoted the recorded replacement candidate. No
  product code changed; next remains planning `TASK-026` localization.

## 2026-07-04 — Guided agent entrypoint idea captured

- Added P2 chore `TASK-028` to replace manual prompt selection with one guided
  entrypoint that routes users into the existing Ralph workflows.
- Scoped the likely solution as an agent-neutral master prompt plus an optional
  thin Codex skill, pending verification of project-local skill conventions and
  realistic cross-agent portability.
- Preserved `AGENTS.md`, Ralph task files, and `HUMAN_README.md` as sources of
  truth; a custom UI, orchestration service, product changes, and speculative
  support for every vendor-specific format are non-goals.
- Left the task P2 and needs-planning with no product dependencies. It does not
  displace the current `TASK-026` localization milestone.

## 2026-07-04 — TASK-028 guided agent entrypoint complete

- Accepted ADR-016 after verifying the repository-local Codex skill convention
  and chose an agent-neutral master prompt plus a thin Agent Skills adapter.
- Added `.agents/PROJECT_WORKFLOWS.md` with ten intent routes, free-form and
  missing-input behavior, ambiguity confirmation, and strict read-only and
  no-implementation boundaries.
- Added the discoverable `$lydkontroll-workflows` adapter and documented the
  plain-Markdown fallback in `HUMAN_README.md`; direct prompts remain readable
  route definitions and troubleshooting fallbacks.
- Added `npm run workflows:check` to normal linting to detect route drift,
  adapter duplication, missing safety language, and stale task IDs.
- Passed lint, 17 frontend tests, 66 Rust tests, public-readiness checks, Ralph
  validation, and Git diff checks. Next remains planning `TASK-026`.

## 2026-07-04 — Marketing-site feature intake

- Captured the owner's goal of launching a Norwegian GitHub Pages marketing
  site, with complete English presentation deferred until the localization
  track is ready.
- Split the request into ready `TASK-029` for a reusable, source-backed
  `DESIGN.md`/Google Stitch handoff and `TASK-030` for the later reviewed-design
  implementation and deployment.
- Required the handoff to preserve the app's warm dark visual language,
  accepted event-generic identity, accurate functionality/limitations, useful
  page structure, screenshot guidance, and a standalone Stitch prompt.
- Left `TASK-030` needs-planning behind `TASK-029`; its draft plan records open
  decisions about the accepted concept, launch CTA, source layout, Pages URL,
  analytics/privacy, publishable screenshots, and later English structure.
- No product code, marketing page, final copy, or design artifact was created
  during intake.

## 2026-07-04 — Marketing track reprioritized ahead of English

- Recorded the owner's new delivery order: marketing and the Norwegian website
  come before English localization.
- Raised ready `TASK-029` and dependent `TASK-030` from P2 to P1; left
  `TASK-026` at P2 and documented that it follows the marketing track.
- Preserved the real dependency graph: `TASK-030` still depends only on the
  design/handoff result from `TASK-029`, while the ordering before `TASK-026`
  is expressed through priority and durable notes rather than a false
  technical dependency.
- No product, design, localization, or website implementation was performed.

## 2026-07-04 — TASK-030 Norwegian marketing site complete

- Replaced all seven retired event-specific website mockups with deterministic,
  event-generic desktop and iPhone captures from the actual app preview.
- Added an explicit image asset revision after live verification exposed stale
  same-name Pages caching; the corrected hero now shows `Mitt arrangement` and
  generic `Middag`, `Introduksjon`, and `Første dans` fixtures.
- Passed website TypeScript/build checks, the public-readiness audit, Ralph
  validation, Git diff checks, and GitHub Pages Actions run `28690125409`.
- Verified `https://dagjomar.github.io/lydkontroll/` visually at 1280x720 and
  390x844. Promoted dependent `TASK-031` to ready for the README link and final
  product screenshot presentation.
