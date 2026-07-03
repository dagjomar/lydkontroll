---
id: TASK-026
title: Add complete English localization and language selection
status: needs-planning
priority: P2
type: feature
owner: unassigned
depends_on: []
plan: none
updated: 2026-07-04
---

# TASK-026: Add complete English localization and language selection

## Context

`ADR-014` deliberately launches `Lydkontroll` as a Norwegian-first project.
International positioning should begin only when the complete operator
experience—not merely the product name or README—is available in English.

## Outcome

An operator can select Norwegian or English, every shipped desktop/mobile
string follows that choice, and adding another language does not require
rewriting component logic.

## Scope

- Inventory and extract all user-facing desktop, mobile, preflight, command,
  persistence, and error strings.
- Add Norwegian and English message catalogs with Norwegian as the migration
  default.
- Persist a language preference independently of event content and expose a
  desktop language selector.
- Apply the selected language to the iPhone projection and HTML metadata where
  runtime behavior permits.
- Add complete English README/marketing positioning before describing the
  project as international.

## Non-goals

- Translating cue names, scene names, event titles, or imported filenames.
- Machine translation or a cloud localization service.
- Claiming support for languages without a complete reviewed catalog.

## Acceptance Criteria

- [ ] Norwegian remains the default for existing installations.
- [ ] English can be selected and persists across restart.
- [ ] Desktop and mobile render complete English UI without mixed Norwegian
      system strings.
- [ ] Event/scene/cue user content is preserved verbatim across language
      changes.
- [ ] Missing message keys fail a validation check.
- [ ] Layout tests cover long strings in both desktop and iPhone projections.

## Validation

```text
npm test -- --run
npm run build
npm run lint
python3 scripts/ralph.py check
```

## Notes

- Needs a plan covering message-catalog ownership, persisted preference
  location, server-rendered metadata limits, and migration behavior.
- English is the first additional language; the architecture should make later
  reviewed catalogs straightforward.
