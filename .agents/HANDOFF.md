# Latest Handoff

Updated: 2026-07-04

## What Just Happened

Captured the requested marketing-site work without implementing it. Ready
`TASK-029` owns a reusable `DESIGN.md` and standalone Google Stitch brief based
on the actual app, accepted identity, and source palette. `TASK-030` owns the
later Norwegian GitHub Pages site and remains needs-planning behind `TASK-029`
and owner review of the resulting design concept. The owner then explicitly
raised both marketing tasks to P1 ahead of P2 English localization.

## Exact Next Action

Claim and start `TASK-029`. Inventory the current visual language from
`src/styles.css`, the neutral waveform assets, ADR-014/TASK-021 positioning,
README/PLAN product facts, and representative desktop/mobile workflows; then
draft and manually validate `DESIGN.md` as a self-contained Google Stitch
handoff. Do not build the Pages site or invent final sales/distribution claims.

## Important Context

- The desired sequence is: factual design/content handoff, Google Stitch
  concept, owner review, `TASK-030` implementation, then `TASK-026` English
  localization.
- Norwegian launches first. `TASK-026` owns complete English localization and
  translated positioning; a partial English marketing page is not wanted.
- Preserve `Lydkontroll`, event-generic Norwegian positioning, the calm
  reliability-first character, and the neutral rounded-waveform mark.
- App palette evidence is in `src/styles.css`; the default cue color is
  `#d88c68` in `src/components/Shell.tsx`.
- Public claims must say desktop control works locally without phone/network,
  while current mobile control requires Tailscale and iPhone/Safari.
- The project is source-available under PolyForm Noncommercial, not OSI open
  source. No public signed/notarized app distribution exists yet.
- `TASK-030` still needs owner choices for the accepted design, primary CTA,
  Pages structure/URL, analytics posture, and publishable screenshot set.
- `TASK-027` remains separate future research; do not market speculative LAN or
  automatic fallback modes.

## Validation

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```
