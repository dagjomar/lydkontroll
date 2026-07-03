# Guided Project Workflow

Use this as the portable master prompt for this repository. It routes an
operator request into the existing Ralph workflows without replacing
`AGENTS.md`, task files, or the detailed prompt catalog in `HUMAN_README.md`.

## Instructions for the Agent

1. Read `AGENTS.md` and follow its startup protocol before changing project
   state. Run live Ralph commands; never infer the current task from this file.
2. If the user already described a clear operation, route it directly. If not,
   briefly offer these choices:

   - Continue Normally
   - Summarize the Task Queue
   - Plan a Task Without Implementing It
   - Implement a Specific Task
   - Add a New Feature to the Backlog
   - Add Feedback, a Bug, or a Missing Feature
   - Reprioritize
   - Review Project Health
   - Diagnose a Blocker
   - Repair the Harness

3. Accept ordinary free-form language. Match it to the nearest choice by the
   requested outcome, not by exact wording.
4. Ask only for a materially required missing input: a task ID for a specific
   plan, implementation, or blocker route; the idea or observation for intake;
   or the changed priority for reprioritization.
5. Confirm before acting only when plausible interpretations would cause
   materially different file changes. For mixed requests, separate the
   operations and preserve the strictest safety boundary.
6. Read the matching section in `HUMAN_README.md`, under either “Useful
   Copy-Paste Prompts” or “If the Harness Gets Confused.” Then follow that
   prompt as the selected workflow. Do not reproduce or silently weaken it
   here.
7. Read-only, `do not implement`, planning-only, diagnosis-only, and intake-only
   instructions are hard boundaries. Do not mutate product code or start an
   implementation task on those routes.
8. For implementation routes, verify dependencies and linked decisions before
   starting. For all mutating routes, update the durable records and validate
   exactly as the selected prompt and `AGENTS.md` require.

## Portable Invocation

Give any coding agent this instruction:

> Read `.agents/PROJECT_WORKFLOWS.md` and guide my request through the correct
> repository workflow: [describe what you want, or ask to see the choices].

Agents that do not support project-local skills can use this invocation
directly. The explicit prompts in `HUMAN_README.md` remain the troubleshooting
fallback and the readable definition of each route.

## Routing Examples

- “What should we do next?” routes to Continue Normally unless the user asks
  only for a report, in which case it routes to Summarize the Task Queue.
- “Prepare TASK-123 but do not code it” routes to Plan a Task Without
  Implementing It and preserves the no-code boundary.
- “The volume control jumps on my phone” routes to Add Feedback, a Bug, or a
  Missing Feature; it does not begin a fix.
- “Why is TASK-123 stuck?” routes to Diagnose a Blocker and remains read-only
  with respect to product code.
- “Clean up the backlog” is materially ambiguous between reprioritization and
  project-health repair, so ask which outcome the user wants.
