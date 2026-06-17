# Human Guide to the Project Harness

This is the short operator manual for keeping the project moving across new
agent sessions.

The harness stores project context, backlog, decisions, progress, and handoffs
inside the repository. A new agent should not need chat history to continue.

## The Easiest Way to Continue

Open a new agent session in this repository and use this prompt:

> Follow `AGENTS.md` and the Ralph harness. Inspect the current state, take the
> next actionable task, complete it if possible, run its validation, update the
> task, progress, state, and handoff records, and commit the result. Stop and ask
> me only if a product decision or external blocker genuinely requires me.

That is the normal day-to-day workflow.

## Before Trusting a New Session

The agent should read:

- `AGENTS.md`: mandatory agent workflow and repository rules;
- `PLAN.md`: product requirements and source of truth;
- `.agents/STATE.md`: current project snapshot;
- `.agents/HANDOFF.md`: the previous session's exact next action;
- the selected task and any linked plan.

It should also run:

```bash
python3 scripts/ralph.py check
python3 scripts/ralph.py next
```

`check` verifies that task metadata and dependencies are consistent. `next`
shows the work the harness currently recommends.

## Commands You May Want

Show the full backlog:

```bash
python3 scripts/ralph.py status
```

Show the next actionable task:

```bash
python3 scripts/ralph.py next
```

Validate the harness:

```bash
python3 scripts/ralph.py check
```

Capture a new idea:

```bash
python3 scripts/ralph.py new "Task title" --priority P2 --type feature
```

After creating a task, ask an agent to refine its scope, dependencies,
acceptance criteria, and validation before implementation.

Claim planning or investigation work without starting implementation:

```bash
python3 scripts/ralph.py claim TASK-NNN --owner "session-name"
```

Start a task that is already marked `ready`:

```bash
python3 scripts/ralph.py start TASK-NNN --owner "session-name"
```

Mark a task blocked:

```bash
python3 scripts/ralph.py set-status TASK-NNN blocked \
  --note "Describe the blocker and what would unblock it"
```

Mark a task done:

```bash
python3 scripts/ralph.py set-status TASK-NNN done \
  --note "Acceptance criteria and validation completed"
```

You normally do not need to run task-changing commands yourself. It is safer to
tell the agent what outcome you want and let it update the harness.

## What the Task States Mean

- `idea`: captured, but not sufficiently refined to schedule.
- `needs-planning`: requires investigation or design before implementation.
- `ready`: scoped, testable, and all dependencies are complete.
- `in-progress`: actively being implemented by an owner.
- `blocked`: cannot advance; the reason and unblock condition must be written.
- `done`: acceptance criteria and validation are complete.

Priorities are `P0` (highest) through `P3` (lowest).

Task files under `.agents/tasks/` are the authoritative backlog. There is no
separate backlog table to keep synchronized.

## Useful Copy-Paste Prompts

### Continue Normally

> Follow the repository harness and take the next actionable task. Complete and
> validate it, update all durable project records, and commit the result.

### Plan a Task Without Implementing It

> Follow the repository harness and plan TASK-NNN. Investigate unresolved
> choices, write or update its linked plan, record durable decisions, refine its
> scope and acceptance criteria, and leave it ready for implementation. Do not
> implement production code.

### Implement a Specific Task

> Follow the repository harness and implement TASK-NNN. Respect its dependencies
> and linked decisions, run all relevant validation, update progress/state/
> handoff records, and commit the result.

### Add a New Feature to the Backlog

> Add this idea to the Ralph backlog: [describe the feature]. Analyze its
> priority, dependencies, scope, non-goals, acceptance criteria, and validation.
> Split it if it is too large. Do not implement it yet.

### Reprioritize

> Review the current backlog against PLAN.md and this new priority: [describe
> priority]. Update task priorities and dependencies where justified, explain
> the changed execution order, and do not implement anything.

### Review Project Health

> Audit the Ralph harness and current repository state. Check for stale
> ownership, vague tasks, missing acceptance criteria, incorrect dependencies,
> undocumented decisions, and mismatches with PLAN.md. Fix the project records
> but do not implement product features.

### Diagnose a Blocker

> Inspect TASK-NNN and its blocker. Diagnose the cause, investigate safe options,
> and update the task or plan with a recommended unblock path. Do not implement a
> fix unless I explicitly approve it.

## When You Should Intervene

Let agents handle routine technical choices that are already constrained by the
plan. Step in when:

- product behavior or event priorities need to change;
- two valid options have meaningful user-facing tradeoffs;
- scope should be added, removed, or postponed;
- hardware, accounts, credentials, or manual testing are required;
- a blocker needs money, access, external coordination, or your preference;
- the implementation technically passes but does not feel right for the event.

If you disagree with the next task, say what outcome matters more and ask the
agent to reprioritize the backlog before implementation.

## What a Healthy Session Should Leave Behind

Before finishing, an agent should:

1. update the active task and its acceptance criteria;
2. record durable architecture choices in `.agents/DECISIONS.md`;
3. update `.agents/STATE.md` when the project-level state changed;
4. append meaningful work to `.agents/PROGRESS.md`;
5. replace `.agents/HANDOFF.md` with the precise next action;
6. run task-specific tests and `python3 scripts/ralph.py check`;
7. commit the completed work with a focused Conventional Commit.

If a session did not finish its task, the task should remain `in-progress` or
be marked `blocked`, with a clear next step.

## If the Harness Gets Confused

Start with:

```bash
python3 scripts/ralph.py check
python3 scripts/ralph.py status
git status
```

Then prompt:

> Repair the Ralph harness without implementing product features. Reconcile task
> status, dependencies, ownership, STATE.md, HANDOFF.md, and the actual Git
> working tree. Preserve valid work, explain what was inconsistent, validate the
> harness, and commit the repair.

Do not manually delete task files merely because they are obsolete. Ask the
agent to close, replace, or document them so project history remains legible.

## Current Starting Point

At the time this guide was created, the next work was `TASK-001`: research and
record foundational architecture decisions. Run `python3 scripts/ralph.py next`
for the current answer rather than relying on this sentence later.
