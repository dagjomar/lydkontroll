# Latest Handoff

Updated: 2026-07-04

## What Just Happened

Completed `TASK-025`. The source-only public repository is live at
https://github.com/dagjomar/lydkontroll with the audited retained history,
PolyForm terms, CLA, public documentation, correct metadata/topics, issues, and
private vulnerability reporting. No binary release was created.

## Exact Next Action

Claim `TASK-026`, resolve its localization architecture and language-selection
questions, and split implementation into dependency-ordered slices. Do not let
translation work shift cue positions or weaken the current Norwegian-first
operator workflow.

## Important Context

- `eventTitle` remains inside schema version 1 and uses a serde default, so old
  libraries load as `Mitt arrangement` without reset.
- Conflict checking is an obvious-collision screen, not trademark clearance.
- `npm run icons:generate` reproducibly rebuilds icons from the SVG source;
  `npm run icons:check` validates transparent macOS corners.
- Finder/Dock/app-switcher inspection is still a useful human check, but the
  built `.app` icon byte-matches the asset whose alpha was verified.
- `TASK-024` is done; ADR-015 records the accepted licensing, distribution,
  contribution, and retained-history decisions.
- Canonical GitHub remote is `git@github.com:dagjomar/lydkontroll.git`; public
  URL is https://github.com/dagjomar/lydkontroll.
- Preliminary audit evidence lives in
  `.agents/audits/TASK-024-public-readiness-audit.md`.
- `TASK-027` is deliberately P3 and needs planning. Its open questions cover
  LAN authentication/binding, mode defaults and persistence, settings,
  preflight, onboarding, instructions, and marketing. Do not add automatic LAN
  fallback or weaken the current fail-closed Tailscale boundary speculatively.
- Current behavior should be stated plainly in public docs: desktop operation
  needs no phone or network; version-one mobile control requires Tailscale.
- Public-source launch is complete. Keep future pushes behind
  `npm run public:check` and normal project validation.
- The old manual event-candidate gate `TASK-016` remains truthfully blocked.

## Validation

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
npm run lint
npm test -- --run
cargo test --manifest-path src-tauri/Cargo.toml
npm run public:check
```
