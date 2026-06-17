# Ralph Loop

This folder is the project's durable working memory. It is intentionally plain
Markdown so a new human or agent session can understand and update it without a
special service.

## The Loop

1. **Orient:** Read `PLAN.md`, `PROJECT.md`, `STATE.md`, and `HANDOFF.md`.
2. **Inspect:** Run `python3 scripts/ralph.py status` and
   `python3 scripts/ralph.py next`.
3. **Choose:** Continue an `in-progress` task, unblock a blocked task, plan the
   highest-priority `needs-planning` task, or start the highest-priority
   dependency-free `ready` task.
4. **Claim:** For planning, run
   `python3 scripts/ralph.py claim TASK-NNN --owner "<agent/session>"`. For
   implementation, run
   `python3 scripts/ralph.py start TASK-NNN --owner "<agent/session>"`.
5. **Work:** Keep the task notes and acceptance criteria current. Record
   architecture choices in `DECISIONS.md`.
6. **Verify:** Run the checks named in the task plus
   `python3 scripts/ralph.py check`.
7. **Close or hand off:** Mark completed work `done`; otherwise leave a precise
   next action in the task and in `HANDOFF.md`. Append the outcome to
   `PROGRESS.md`.
8. **Repeat:** `python3 scripts/ralph.py next` chooses from the updated state.

## Source-of-Truth Rules

- `PLAN.md` defines product scope and intended behavior.
- `PROJECT.md` distills stable engineering context and constraints.
- `tasks/TASK-NNN-*.md` files define backlog items and their current status.
- A linked file under `plans/` contains deeper implementation analysis.
- `STATE.md` is a short, curated snapshot, not a duplicate backlog.
- `HANDOFF.md` is the latest session-to-session baton pass.
- `PROGRESS.md` is an append-only history of meaningful work.
- `DECISIONS.md` records durable decisions and their rationale.

If documents conflict, fix the conflict in the same session. Product behavior
in `PLAN.md` wins unless an explicit decision records an approved change.

## Task States

- `idea`: captured but not refined enough to schedule.
- `needs-planning`: important, but open questions make implementation unsafe.
- `ready`: scoped, testable, dependency-free, and small enough to execute.
- `in-progress`: actively owned by one session.
- `blocked`: cannot advance; the blocker and unblock condition must be written.
- `done`: acceptance criteria and validation are complete.

Only one task should normally be `in-progress`. Parallel work is allowed when
task files do not overlap and each task has a distinct owner.

## Task Size and Readiness

A ready task should fit in one focused agent session when practical. Split it if
it mixes unrelated behavior, has unclear acceptance criteria, or requires
substantial design work.

A task is ready only when:

- its outcome is observable;
- scope and non-goals are explicit;
- dependencies are `done`;
- acceptance criteria are testable;
- validation commands or manual checks are listed;
- risky choices have a linked plan or decision.

## Commands

```text
python3 scripts/ralph.py status
python3 scripts/ralph.py next
python3 scripts/ralph.py check
python3 scripts/ralph.py new "Task title" --priority P1 --type feature
python3 scripts/ralph.py claim TASK-NNN --owner "codex/session-name"
python3 scripts/ralph.py start TASK-NNN --owner "codex/session-name"
python3 scripts/ralph.py set-status TASK-NNN ready
python3 scripts/ralph.py set-status TASK-NNN blocked --note "Waiting for ..."
python3 scripts/ralph.py set-status TASK-NNN done --note "Validated with ..."
```

`claim` assigns planning or triage work without changing status. `start` accepts
only a dependency-free `ready` task and moves it to `in-progress`. The tool
otherwise changes only task front matter and appends an optional timestamped
note. Rich task content should still be edited directly.
