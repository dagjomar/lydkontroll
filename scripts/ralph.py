#!/usr/bin/env python3
"""Small dependency-free task manager for the repository's Ralph loop."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TASK_DIR = ROOT / ".agents" / "tasks"
TASK_PATTERN = "TASK-[0-9][0-9][0-9]-*.md"
VALID_STATUSES = {
    "idea",
    "needs-planning",
    "ready",
    "in-progress",
    "blocked",
    "done",
}
VALID_PRIORITIES = {"P0", "P1", "P2", "P3"}
VALID_TYPES = {"feature", "bug", "chore", "research", "spike", "docs"}
REQUIRED_FIELDS = {
    "id",
    "title",
    "status",
    "priority",
    "type",
    "owner",
    "depends_on",
    "plan",
    "updated",
}
PRIORITY_ORDER = {"P0": 0, "P1": 1, "P2": 2, "P3": 3}
STATUS_ORDER = {
    "in-progress": 0,
    "blocked": 1,
    "needs-planning": 2,
    "ready": 3,
    "idea": 4,
    "done": 5,
}


@dataclass
class Task:
    path: Path
    metadata: dict[str, object]
    body: str

    @property
    def id(self) -> str:
        return str(self.metadata.get("id", "MISSING-ID"))

    @property
    def status(self) -> str:
        return str(self.metadata.get("status", "missing-status"))

    @property
    def priority(self) -> str:
        return str(self.metadata.get("priority", "missing-priority"))

    @property
    def dependencies(self) -> list[str]:
        value = self.metadata.get("depends_on", [])
        return value if isinstance(value, list) else []


def parse_value(value: str) -> object:
    value = value.strip()
    if value.startswith("[") and value.endswith("]"):
        inner = value[1:-1].strip()
        if not inner:
            return []
        return [item.strip() for item in inner.split(",") if item.strip()]
    return value


def format_value(value: object) -> str:
    if isinstance(value, list):
        return f"[{', '.join(str(item) for item in value)}]"
    return str(value)


def parse_task(path: Path) -> Task:
    text = path.read_text(encoding="utf-8")
    match = re.match(r"\A---\n(.*?)\n---\n(.*)\Z", text, re.DOTALL)
    if not match:
        raise ValueError("missing valid front matter")

    metadata: dict[str, object] = {}
    for line in match.group(1).splitlines():
        if not line.strip():
            continue
        if ":" not in line:
            raise ValueError(f"invalid front matter line: {line}")
        key, value = line.split(":", 1)
        metadata[key.strip()] = parse_value(value)
    return Task(path=path, metadata=metadata, body=match.group(2))


def load_tasks() -> tuple[list[Task], list[str]]:
    tasks: list[Task] = []
    errors: list[str] = []
    for path in sorted(TASK_DIR.glob(TASK_PATTERN)):
        try:
            tasks.append(parse_task(path))
        except ValueError as error:
            errors.append(f"{path.relative_to(ROOT)}: {error}")
    return tasks, errors


def validate(tasks: list[Task], parse_errors: list[str]) -> list[str]:
    errors = list(parse_errors)
    ids: dict[str, Task] = {}

    for task in tasks:
        relative = task.path.relative_to(ROOT)
        missing = REQUIRED_FIELDS - task.metadata.keys()
        if missing:
            errors.append(f"{relative}: missing fields {sorted(missing)}")
            continue
        if task.id in ids:
            errors.append(f"{relative}: duplicate id {task.id}")
        ids[task.id] = task
        if not re.fullmatch(r"TASK-\d{3}", task.id):
            errors.append(f"{relative}: invalid id {task.id}")
        if not task.path.name.startswith(f"{task.id}-"):
            errors.append(f"{relative}: filename does not start with {task.id}-")
        if task.status not in VALID_STATUSES:
            errors.append(f"{relative}: invalid status {task.status}")
        if task.priority not in VALID_PRIORITIES:
            errors.append(f"{relative}: invalid priority {task.priority}")
        if task.metadata["type"] not in VALID_TYPES:
            errors.append(f"{relative}: invalid type {task.metadata['type']}")
        try:
            dt.date.fromisoformat(str(task.metadata["updated"]))
        except ValueError:
            errors.append(f"{relative}: updated must be YYYY-MM-DD")
        if task.status == "in-progress" and task.metadata["owner"] == "unassigned":
            errors.append(f"{relative}: in-progress task must have an owner")
        if task.status == "blocked" and "block" not in task.body.lower():
            errors.append(f"{relative}: blocked task must describe its blocker")
        plan = str(task.metadata["plan"])
        if plan != "none" and not (ROOT / plan).exists():
            errors.append(f"{relative}: linked plan does not exist: {plan}")

    for task in tasks:
        for dependency in task.dependencies:
            if dependency not in ids:
                errors.append(
                    f"{task.path.relative_to(ROOT)}: unknown dependency {dependency}"
                )
            elif dependency == task.id:
                errors.append(
                    f"{task.path.relative_to(ROOT)}: task depends on itself"
                )
        if task.status in {"ready", "in-progress"}:
            incomplete = [
                dependency
                for dependency in task.dependencies
                if dependency not in ids or ids[dependency].status != "done"
            ]
            if incomplete:
                errors.append(
                    f"{task.path.relative_to(ROOT)}: {task.status} task is waiting on "
                    f"{', '.join(incomplete)}"
                )

    errors.extend(find_cycles(tasks))
    return errors


def find_cycles(tasks: list[Task]) -> list[str]:
    graph = {task.id: task.dependencies for task in tasks}
    visiting: set[str] = set()
    visited: set[str] = set()
    errors: list[str] = []

    def visit(task_id: str, trail: list[str]) -> None:
        if task_id in visiting:
            cycle = trail[trail.index(task_id) :]
            errors.append(f"dependency cycle: {' -> '.join(cycle)}")
            return
        if task_id in visited or task_id not in graph:
            return
        visiting.add(task_id)
        for dependency in graph[task_id]:
            visit(dependency, trail + [dependency])
        visiting.remove(task_id)
        visited.add(task_id)

    for task_id in graph:
        visit(task_id, [task_id])
    return sorted(set(errors))


def task_sort_key(task: Task) -> tuple[int, int, str]:
    return (
        STATUS_ORDER.get(task.status, 99),
        PRIORITY_ORDER.get(task.priority, 99),
        task.id,
    )


def dependency_state(task: Task, by_id: dict[str, Task]) -> tuple[bool, list[str]]:
    incomplete = [
        dependency
        for dependency in task.dependencies
        if dependency not in by_id or by_id[dependency].status != "done"
    ]
    return not incomplete, incomplete


def print_status(tasks: list[Task], as_json: bool) -> None:
    by_id = {task.id: task for task in tasks}
    rows = []
    for task in sorted(tasks, key=task_sort_key):
        dependencies_done, incomplete = dependency_state(task, by_id)
        rows.append(
            {
                "id": task.id,
                "priority": task.priority,
                "status": task.status,
                "owner": task.metadata["owner"],
                "dependencies_ready": dependencies_done,
                "waiting_on": incomplete,
                "title": task.metadata["title"],
            }
        )

    if as_json:
        print(json.dumps(rows, indent=2))
        return

    print(f"{'ID':<9} {'PRI':<4} {'STATUS':<15} {'OWNER':<20} TITLE")
    for row in rows:
        waiting = (
            f" [waiting: {', '.join(row['waiting_on'])}]"
            if row["waiting_on"]
            else ""
        )
        print(
            f"{row['id']:<9} {row['priority']:<4} {row['status']:<15} "
            f"{str(row['owner']):<20} {row['title']}{waiting}"
        )


def choose_next(tasks: list[Task]) -> list[Task]:
    by_id = {task.id: task for task in tasks}
    in_progress = [task for task in tasks if task.status == "in-progress"]
    if in_progress:
        return sorted(in_progress, key=task_sort_key)

    blocked = [task for task in tasks if task.status == "blocked"]
    if blocked:
        return sorted(blocked, key=task_sort_key)

    candidates = []
    for task in tasks:
        dependencies_done, _ = dependency_state(task, by_id)
        if dependencies_done and task.status in {"needs-planning", "ready"}:
            candidates.append(task)
    return sorted(candidates, key=task_sort_key)


def write_task(task: Task) -> None:
    ordered_keys = [
        "id",
        "title",
        "status",
        "priority",
        "type",
        "owner",
        "depends_on",
        "plan",
        "updated",
    ]
    lines = ["---"]
    for key in ordered_keys:
        lines.append(f"{key}: {format_value(task.metadata[key])}")
    for key in task.metadata:
        if key not in ordered_keys:
            lines.append(f"{key}: {format_value(task.metadata[key])}")
    lines.extend(["---", task.body])
    task.path.write_text("\n".join(lines), encoding="utf-8")


def find_task(task_id: str, tasks: list[Task]) -> Task:
    normalized = task_id.upper()
    for task in tasks:
        if task.id == normalized:
            return task
    raise SystemExit(f"Unknown task: {task_id}")


def append_note(task: Task, note: str) -> None:
    today = dt.date.today().isoformat()
    task.body = task.body.rstrip() + f"\n\n- {today}: {note}\n"


def set_status(task: Task, status: str, owner: str | None, note: str | None) -> None:
    task.metadata["status"] = status
    task.metadata["updated"] = dt.date.today().isoformat()
    if owner is not None:
        task.metadata["owner"] = owner
    elif status == "done":
        task.metadata["owner"] = "unassigned"
    if note:
        append_note(task, note)
    write_task(task)
    print(f"{task.id} -> {status}")


def slugify(title: str) -> str:
    slug = re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")
    return slug or "task"


def create_task(title: str, priority: str, task_type: str) -> None:
    tasks, errors = load_tasks()
    validation_errors = validate(tasks, errors)
    if validation_errors:
        raise SystemExit("Fix existing task errors before creating a task.")
    next_number = max((int(task.id.split("-")[1]) for task in tasks), default=0) + 1
    task_id = f"TASK-{next_number:03d}"
    path = TASK_DIR / f"{task_id}-{slugify(title)}.md"
    today = dt.date.today().isoformat()
    path.write_text(
        f"""---
id: {task_id}
title: {title}
status: idea
priority: {priority}
type: {task_type}
owner: unassigned
depends_on: []
plan: none
updated: {today}
---

# {task_id}: {title}

## Context

Explain why this work exists.

## Outcome

Describe the observable result.

## Scope

- Define included behavior.

## Non-goals

- Define excluded behavior.

## Acceptance Criteria

- [ ] Add testable acceptance criteria.

## Validation

```text
python3 scripts/ralph.py check
```

## Notes

Refine this task, add dependencies, and move it to `needs-planning` or `ready`.
""",
        encoding="utf-8",
    )
    print(path.relative_to(ROOT))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    status_parser = subparsers.add_parser("status", help="show all tasks")
    status_parser.add_argument("--json", action="store_true")
    subparsers.add_parser("next", help="show the next actionable task")
    subparsers.add_parser("check", help="validate task metadata and dependencies")

    new_parser = subparsers.add_parser("new", help="create an idea task")
    new_parser.add_argument("title")
    new_parser.add_argument("--priority", choices=sorted(VALID_PRIORITIES), default="P2")
    new_parser.add_argument("--type", choices=sorted(VALID_TYPES), default="feature")

    claim_parser = subparsers.add_parser(
        "claim", help="assign planning or triage work without changing status"
    )
    claim_parser.add_argument("task_id")
    claim_parser.add_argument("--owner", required=True)

    start_parser = subparsers.add_parser("start", help="claim a ready task")
    start_parser.add_argument("task_id")
    start_parser.add_argument("--owner", required=True)

    set_parser = subparsers.add_parser("set-status", help="change task status")
    set_parser.add_argument("task_id")
    set_parser.add_argument("status", choices=sorted(VALID_STATUSES))
    set_parser.add_argument("--owner")
    set_parser.add_argument("--note")

    args = parser.parse_args()
    if args.command == "new":
        create_task(args.title, args.priority, args.type)
        return 0

    tasks, parse_errors = load_tasks()
    errors = validate(tasks, parse_errors)
    if args.command == "check":
        if errors:
            print("\n".join(f"ERROR: {error}" for error in errors))
            return 1
        print(f"OK: {len(tasks)} task files are valid.")
        return 0
    if errors:
        print("Task data is invalid. Run `python3 scripts/ralph.py check`.", file=sys.stderr)
        return 1

    if args.command == "status":
        print_status(tasks, args.json)
        return 0
    if args.command == "next":
        choices = choose_next(tasks)
        if not choices:
            print("No actionable task. Refine an idea or resolve dependencies.")
            return 0
        print_status(choices, False)
        return 0

    task = find_task(args.task_id, tasks)
    if args.command == "claim":
        if task.status == "done":
            raise SystemExit(f"{task.id} is done and cannot be claimed.")
        by_id = {item.id: item for item in tasks}
        dependencies_done, incomplete = dependency_state(task, by_id)
        if not dependencies_done:
            raise SystemExit(f"{task.id} is waiting on {', '.join(incomplete)}.")
        task.metadata["owner"] = args.owner
        task.metadata["updated"] = dt.date.today().isoformat()
        write_task(task)
        print(f"{task.id} claimed by {args.owner}; status remains {task.status}.")
        return 0
    if args.command == "start":
        by_id = {item.id: item for item in tasks}
        dependencies_done, incomplete = dependency_state(task, by_id)
        if task.status != "ready":
            raise SystemExit(
                f"{task.id} is {task.status}; only ready tasks can be started."
            )
        if not dependencies_done:
            raise SystemExit(f"{task.id} is waiting on {', '.join(incomplete)}.")
        set_status(task, "in-progress", args.owner, "Task claimed.")
        return 0
    if args.command == "set-status":
        if args.status in {"ready", "in-progress"}:
            by_id = {item.id: item for item in tasks}
            dependencies_done, incomplete = dependency_state(task, by_id)
            if not dependencies_done:
                raise SystemExit(f"{task.id} is waiting on {', '.join(incomplete)}.")
        set_status(task, args.status, args.owner, args.note)
        return 0
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
