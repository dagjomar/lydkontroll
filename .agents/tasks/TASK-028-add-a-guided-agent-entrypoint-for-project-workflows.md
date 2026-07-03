---
id: TASK-028
title: Add a guided agent entrypoint for project workflows
status: done
priority: P2
type: chore
owner: codex-2026-07-04-task-028
depends_on: []
plan: .agents/plans/TASK-028-guided-agent-entrypoint.md
updated: 2026-07-04
---

# TASK-028: Add a guided agent entrypoint for project workflows

## Context

`HUMAN_README.md` contains copy-paste prompts for continuing work, planning,
implementation, feedback intake, reprioritization, health review, and blocker
diagnosis. A user currently has to find the guide, choose the correct prompt,
copy it, and fill in its placeholders before an agent can start the appropriate
Ralph workflow.

Original idea: add an agent skill or similarly discoverable project entrypoint
that first helps the user decide what operation they want, then selects and
applies the matching prompt/workflow. It should work across coding agents where
practical and otherwise be optimized for Codex. The simplest candidate is a
single master prompt that refers to the canonical prompt catalog, presents a
small set of choices, asks only for missing inputs, and dispatches the selected
workflow.

## Outcome

From a fresh agent session, a user can invoke one documented project
entrypoint, choose or describe their intent in plain language, and be guided
into the correct existing Ralph workflow without manually reading and copying
a prompt from `HUMAN_README.md`.

## Scope

- Inventory the canonical workflows and prompts in `HUMAN_README.md`.
- Decide the portable source-of-truth format and whether a thin Codex skill
  should wrap it for better discovery and invocation.
- Provide one guided entrypoint that presents understandable choices, accepts
  free-form intent, asks only for information required by the selected path,
  and then follows the matching repository workflow.
- Keep workflow rules grounded in `AGENTS.md` and live Ralph state rather than
  embedding transient task numbers or duplicating backlog state.
- Document invocation and graceful fallback for agents that do not support
  project-local skills.

## Non-goals

- Building a custom chat UI, command palette, plugin package, or orchestration
  service.
- Replacing `AGENTS.md`, Ralph, task files, or `HUMAN_README.md` as durable
  sources of truth.
- Making every agent product support the same proprietary skill packaging or
  interactive-choice mechanism.
- Automatically starting implementation before the user has chosen an
  operation and supplied materially required input.
- Redesigning Ralph or changing product code in this task.

## Acceptance Criteria

- [x] A fresh user has one clearly documented entrypoint that can route all
      canonical workflows currently listed in `HUMAN_README.md`.
- [x] The entrypoint presents concise choices but also maps an unstructured
      request to the nearest workflow and confirms only when ambiguity would
      materially change the operation.
- [x] Each route preserves the selected prompt's safety boundary, especially
      read-only requests and explicit "do not implement" instructions.
- [x] Shared guidance has one canonical source or a drift check; a Codex skill,
      if selected, is a thin adapter rather than a second prompt catalog.
- [x] Agents without skill support have a documented master-prompt fallback.
- [x] Guidance uses live Ralph state and does not hard-code a current task as
      the permanent starting point.
- [x] Fresh-session scenarios validate every route, missing details, ambiguous
      intent, and a request that must remain read-only.
- [x] `HUMAN_README.md` explains the entrypoint and retains direct prompts as
      transparent fallbacks for troubleshooting and portability.

## Validation

```text
manual: in a fresh Codex session, invoke the entrypoint and exercise every canonical route
manual: test free-form, ambiguous, incomplete, and read-only requests
manual: use only the master-prompt fallback with an agent that has no skill support
python3 scripts/ralph.py check
```

## Notes

- P2 because this lowers recurring operator friction and improves continuity,
  but does not block product localization or runtime reliability.
- No product-task dependency is required. Planning must inspect current agent
  instruction formats and Codex project-skill conventions before packaging.
- Keep this as one task: the portable master prompt and optional Codex adapter
  are one small workflow. Split only if agent-specific implementations cannot
  remain thin wrappers.
- Needs planning because portability, canonical prompt ownership,
  discoverability, and fallback behavior must be decided first.
- Accepted Option C after verifying current Codex guidance: repository-local
  skills are discovered under `.agents/skills` and use the open Agent Skills
  format. A plugin would add needless installation machinery for one
  repository-scoped workflow.
- `.agents/PROJECT_WORKFLOWS.md` is the agent-neutral router;
  `.agents/skills/lydkontroll-workflows/SKILL.md` is only a discovery adapter.
- `npm run workflows:check` validates all ten routes, the skill reference,
  ambiguity and read-only safeguards, and the absence of stale task routing.
- Passed lint, frontend and Rust tests, public-readiness, Ralph, and Git diff
  validation.

- 2026-07-04: Accepted Option C: portable master prompt plus thin repository-local skill; routing and safety drift are automated.

- 2026-07-04: Task claimed.

- 2026-07-04: Guided portable router, thin Codex skill, documentation, drift check, and full validation complete.
