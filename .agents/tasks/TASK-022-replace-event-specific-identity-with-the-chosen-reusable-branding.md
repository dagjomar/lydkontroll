---
id: TASK-022
title: Replace event-specific identity with the chosen reusable branding
status: done
priority: P1
type: feature
owner: unassigned
depends_on: [TASK-021]
plan: none
updated: 2026-07-04
---

# TASK-022: Replace event-specific identity with the chosen reusable branding

## Context

The UI still displays `Marius + Wenche`, while Cargo metadata, HTML metadata,
README/PLAN/project context, Tauri product/window names, server fixtures, and
release documentation retain event-specific or provisional identity. These
are one cross-cutting branding result and must follow `TASK-021` rather than
being independently guessed.

## Outcome

A fresh clone and fresh app-data profile present the selected reusable product
identity everywhere, with the event title behaving exactly as decided, and no
Marius/Wenche-specific text remains in shipped product or public project docs.

## Scope

- Apply the selected name and positioning to desktop, mobile, HTML, Tauri,
  Rust/package metadata, tests, and user-facing documentation.
- Implement the selected fixed/configurable event-title behavior, including
  persistence and desktop editing only if required by `TASK-021`.
- Preserve existing cue libraries through any schema/config migration.
- Update release/build paths and checks that depend on the product name.

## Non-goals

- Selecting the brand; that is `TASK-021`.
- Redrawing the icon; transparent macOS corners are `TASK-023`.
- Publishing to GitHub; that is `TASK-025`.

## Acceptance Criteria

- [x] Desktop and mobile show the chosen product name and event-title behavior.
- [x] A new user sees a useful non-couple-specific default.
- [x] If the title is configurable, edits persist across restart and appear on
      both desktop and mobile without disrupting playback or cue positions.
- [x] Existing schema-v1 user data loads without loss or reset.
- [x] Shipped metadata, tests, README, PLAN/project context, and release docs no
      longer describe the app as exclusively for Marius and Wenche's wedding.
- [x] A targeted repository search finds no unintended event-specific identity.

## Validation

```text
rg -n --glob '!src-tauri/target/**' --glob '!.git/**' 'Marius|Wenche'
npm test -- --run
cargo test --manifest-path src-tauri/Cargo.toml
npm run build
npm run lint
manual: launch with fresh and existing app data and compare desktop/mobile title
python3 scripts/ralph.py check
```

## Notes

- P1 because this is required before a credible public launch.
- Fully scoped but blocked by `TASK-021`; move to ready when that task supplies
  the exact identity and customization decision.
- 2026-07-04: Added schema-v1-compatible `eventTitle` persistence with `Mitt
arrangement` as the missing/empty display fallback, desktop editing, and
  authoritative desktop/mobile projection. Existing libraries without the new
  field load without migration failure.
- Replaced couple-specific UI, package, HTML, identifier, tests, README, and
  plan copy; regenerated all icon assets from a neutral rounded waveform SVG.
- Validation passed: `npm run lint`, `npm test -- --run` (17), `cargo test`
  (66 including loopback tests with socket permission), `npm run build`,
  `npm run icons:check`, targeted identity searches, and `git diff --check`.

- 2026-07-04: Task claimed.
