---
name: lydkontroll-workflows
description: Guide a Lydkontroll repository request into the correct Ralph workflow. Use when the user asks what to do, wants workflow choices, or describes planning, implementation, feedback intake, prioritization, health review, blocker diagnosis, or harness repair.
---

# Lydkontroll Workflows

1. Read `.agents/PROJECT_WORKFLOWS.md` completely.
2. Use it to classify the user's stated intent or offer its concise choices.
3. Ask only for inputs that the selected route materially requires.
4. Read the selected canonical prompt in `HUMAN_README.md` and execute it while
   following `AGENTS.md` and live Ralph state.
5. Preserve read-only and no-implementation boundaries exactly.

This skill is a thin discovery adapter. Do not copy the route catalog or task
state into this file.
