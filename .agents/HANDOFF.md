# Latest Handoff

Updated: 2026-07-04

## What Just Happened

Completed `TASK-024`. The retained history is ready for source-only publication
under PolyForm Noncommercial 1.0.0, with separate commercial licensing and a
CLA for accepted contributions. Public docs, support/security boundaries,
dependency review, ignore rules, and the history-aware readiness gate pass.

## Exact Next Action

Ask the owner for the destination GitHub account or organization and explicit
authorization to create and push the public `lydkontroll` repository. Then claim
and start ready `TASK-025`, create the remote without overwriting anything,
configure metadata, push the audited history, and validate a fresh clone.

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
- Preliminary audit evidence lives in
  `.agents/audits/TASK-024-public-readiness-audit.md`.
- `TASK-027` is deliberately P3 and needs planning. Its open questions cover
  LAN authentication/binding, mode defaults and persistence, settings,
  preflight, onboarding, instructions, and marketing. Do not add automatic LAN
  fallback or weaken the current fail-closed Tailscale boundary speculatively.
- Current behavior should be stated plainly in public docs: desktop operation
  needs no phone or network; version-one mobile control requires Tailscale.
- Do not create or push a public GitHub remote until `TASK-022`, `TASK-023`, and
  `TASK-024` are done. They are now done, but external publication still needs
  the destination and explicit owner authorization.
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
