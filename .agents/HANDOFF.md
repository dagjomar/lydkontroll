# Latest Handoff

Updated: 2026-07-04

## What Just Happened

Claimed `TASK-024` planning and completed a preliminary public-tree/history
audit. No private artifacts or absolute user paths were found, but an ordinary
public push exposes the author's Gmail address from every commit and the retired
Marius/Wenche identity from older commits. Source-only is the recommended first
release; the task remains needs-planning for genuine owner decisions.

## Exact Next Action

Obtain the owner's three decisions: MIT versus Apache-2.0 (or another named
license), confirm source-only first release, and retain versus sanitize Git
history after reviewing the Gmail/retired-name disclosures. Then finish the
plan, move `TASK-024` to ready, start it, and implement the public-readiness
docs/checks before allowing `TASK-025` to create a public remote.

## Important Context

- `eventTitle` remains inside schema version 1 and uses a serde default, so old
  libraries load as `Mitt arrangement` without reset.
- Conflict checking is an obvious-collision screen, not trademark clearance.
- `npm run icons:generate` reproducibly rebuilds icons from the SVG source;
  `npm run icons:check` validates transparent macOS corners.
- Finder/Dock/app-switcher inspection is still a useful human check, but the
  built `.app` icon byte-matches the asset whose alpha was verified.
- `TASK-024` must settle license and source-only versus signed/notarized binary
  releases and audit the intended Git history.
- Preliminary audit evidence lives in
  `.agents/audits/TASK-024-public-readiness-audit.md`.
- `TASK-027` is deliberately P3 and needs planning. Its open questions cover
  LAN authentication/binding, mode defaults and persistence, settings,
  preflight, onboarding, instructions, and marketing. Do not add automatic LAN
  fallback or weaken the current fail-closed Tailscale boundary speculatively.
- Current behavior should be stated plainly in public docs: desktop operation
  needs no phone or network; version-one mobile control requires Tailscale.
- Do not create or push a public GitHub remote until `TASK-022`, `TASK-023`, and
  `TASK-024` are done.
- The old manual event-candidate gate `TASK-016` remains truthfully blocked.

## Validation

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
npm run lint
npm test -- --run
cargo test --manifest-path src-tauri/Cargo.toml
```
