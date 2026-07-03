# Plan: TASK-028 — Guided agent entrypoint for project workflows

Status: accepted
Updated: 2026-07-04

## Problem

The repository has good workflow prompts but makes the user act as a manual
router. A guided entrypoint should make them discoverable without creating a
second, drifting orchestration system or depending entirely on one vendor.

## Current Evidence

- `HUMAN_README.md` contains the canonical copy-paste prompt catalog.
- `AGENTS.md` defines mandatory workflow and repository rules.
- Ralph task files and `python3 scripts/ralph.py next` own live work selection;
  static guidance must remain task-agnostic.
- Codex supports agent skills, but other agents' portability and discovery
  conventions have not been verified.

## Resolved Questions

- Codex discovers repository skills under `.agents/skills`, and that format is
  based on the open Agent Skills standard. Agents without skill support can use
  a plain Markdown master prompt directly.
- Routing lives in `.agents/PROJECT_WORKFLOWS.md`. `HUMAN_README.md` remains the
  transparent prompt catalog, while an automated check keeps their route names
  aligned.
- The Codex skill contains invocation and dispatch instructions only, then
  reads the agent-neutral master prompt instead of copying its catalog.
- Ten choices cover the existing workflows: continue, queue summary, planning,
  implementation, single or batch intake, reprioritization, health review,
  blocker diagnosis, and harness repair.
- Free-form intent may route directly when the operation is clear. Missing
  identifiers or feedback are requested before action; mixed or materially
  ambiguous intent is confirmed. Read-only and no-implementation boundaries
  always win.

## Options Considered

### Option A — Agent-neutral master prompt only

- Benefits: simplest and broadly portable.
- Costs and risks: weaker discovery and invocation in Codex.

### Option B — Codex skill only

- Benefits: polished Codex discovery and guided invocation.
- Costs and risks: poor portability and duplicated hidden workflow guidance.

### Option C — Agent-neutral master prompt plus thin Codex adapter

- Benefits: portable canonical behavior with a good Codex entrypoint.
- Costs and risks: artifacts need an explicit ownership/reference relationship.

## Decision

Accept Option C. Store the portable master prompt in
`.agents/PROJECT_WORKFLOWS.md` and a thin Codex/Open Agent Skills adapter in
`.agents/skills/lydkontroll-workflows/SKILL.md`. Add a deterministic check for
the expected route catalog, adapter reference, safety language, and stale task
IDs. Do not package a plugin: this is a single repository-local workflow.

## Implementation Slices

1. Define the intent taxonomy, routing rules, required inputs, and ambiguity
   behavior from `HUMAN_README.md`.
2. Add the agent-neutral master prompt and thin repository-local skill.
3. Update operator docs and add a lightweight drift check to normal linting.
4. Exercise every route through deterministic scenarios and document the
   remaining agent-without-skill fallback check.

## Test Strategy

- Automated: validate required files/references, prevent stale hard-coded task
  routing, and detect duplicated routing content if practical.
- Manual: test each route plus free-form, ambiguous, and read-only requests in
  fresh sessions, including an agent without skill support.
- Failure modes: undiscovered skill, unsupported format, catalog drift, missing
  task ID, mixed intent, and premature implementation.

## Rollback or Recovery

Direct prompts in `HUMAN_README.md` remain usable. A broken adapter can be
removed without changing Ralph state or canonical prompts.

## Ready Checklist

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task can move to `ready`.
