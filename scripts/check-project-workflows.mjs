import { readFileSync } from "node:fs";

const humanGuide = readFileSync("HUMAN_README.md", "utf8");
const masterPrompt = readFileSync(".agents/PROJECT_WORKFLOWS.md", "utf8");
const skill = readFileSync(
  ".agents/skills/lydkontroll-workflows/SKILL.md",
  "utf8",
);

const routes = [
  "Continue Normally",
  "Summarize the Task Queue",
  "Plan a Task Without Implementing It",
  "Implement a Specific Task",
  "Add a New Feature to the Backlog",
  "Add Feedback, a Bug, or a Missing Feature",
  "Reprioritize",
  "Review Project Health",
  "Diagnose a Blocker",
];

for (const route of routes) {
  if (!humanGuide.includes(`### ${route}`)) {
    throw new Error(`HUMAN_README.md is missing workflow route: ${route}`);
  }
  if (!masterPrompt.includes(`- ${route}`)) {
    throw new Error(`Master prompt is missing workflow route: ${route}`);
  }
}

if (!humanGuide.includes("## If the Harness Gets Confused")) {
  throw new Error("HUMAN_README.md is missing the harness repair workflow");
}
if (!masterPrompt.includes("- Repair the Harness")) {
  throw new Error("Master prompt is missing the harness repair route");
}

for (const phrase of [
  "ordinary free-form language",
  "materially required missing input",
  "Confirm before acting only when",
  "Read-only",
  "do not implement",
]) {
  if (!masterPrompt.includes(phrase)) {
    throw new Error(`Master prompt is missing safety guidance: ${phrase}`);
  }
}

if (!skill.includes(".agents/PROJECT_WORKFLOWS.md")) {
  throw new Error("Codex skill does not reference the canonical master prompt");
}
if (routes.some((route) => skill.includes(route))) {
  throw new Error("Codex skill duplicates the canonical route catalog");
}
if (/TASK-\d{3}/u.test(masterPrompt.replaceAll("TASK-123", ""))) {
  throw new Error("Master prompt contains a stale hard-coded task ID");
}

console.log(`Project workflow check passed (${routes.length + 1} routes).`);
