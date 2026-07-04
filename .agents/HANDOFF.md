# Latest Handoff

Updated: 2026-07-04

## What Just Happened

Integrated the accepted Stitch/AI Studio site for in-progress `TASK-030` under
`website/`. It now has local assets, accurate Norwegian claims, canonical
GitHub links, isolated dependencies, and a GitHub Pages Actions workflow.
Automated and responsive visual checks pass.

## Exact Next Action

Obtain event-generic replacements for the tracked files in
`website/public/images/`. The current mockups still show `Marius + Wenche`,
especially `desktop-overview.png` and `mobile-control.png`. Replace the images,
rerun `npm ci && npm run lint && npm run build` in `website/`, then deploy and
verify `https://dagjomar.github.io/lydkontroll/` on desktop and iPhone Safari.

## Important Context

- `TASK-030` remains in progress; do not mark it done before generic screenshots
  and live Pages verification are complete.
- The marketing site remains Norwegian-first; `TASK-026` follows it and owns
  complete English localization.
- Current mobile control requires iPhone/Safari and Tailscale. Desktop control
  remains local and independent of mobile connectivity.
- The project is source-available under PolyForm Noncommercial and has no
  public signed/notarized download. Preserve those claim boundaries.
- `TASK-027` remains future research; do not market speculative LAN or fallback
  modes.
- `TASK-031` adds the verified Pages URL prominently to `README.md` after
  `TASK-030` is complete; do not add the link before the site is live.

## Validation

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```
