import { execFileSync, spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";

const requiredFiles = [
  "LICENSE",
  "NOTICE",
  "CLA.md",
  "CONTRIBUTING.md",
  "SECURITY.md",
  "SUPPORT.md",
  "THIRD_PARTY_LICENSES.md",
];

for (const file of requiredFiles) {
  readFileSync(file, "utf8");
}

const license = readFileSync("LICENSE", "utf8");
if (!license.startsWith("# PolyForm Noncommercial License 1.0.0")) {
  throw new Error("LICENSE is not PolyForm Noncommercial 1.0.0");
}

const readme = readFileSync("README.md", "utf8");
for (const phrase of [
  "source-available",
  "Commercial use",
  "Tailscale",
  "source and build instructions",
]) {
  if (!readme.includes(phrase)) {
    throw new Error(`README is missing required public boundary: ${phrase}`);
  }
}

execFileSync(process.execPath, ["scripts/check-release-tree.mjs"], {
  stdio: "inherit",
});

const historicalPaths = execFileSync(
  "git",
  ["rev-list", "--objects", "--all"],
  { encoding: "utf8" },
)
  .split(/\r?\n/u)
  .map((line) => line.replace(/^[0-9a-f]+\s*/u, ""))
  .filter(Boolean);

const forbiddenHistoricalPatterns = [
  { label: "user audio", pattern: /\.(?:m4a|mp3|wav)$/iu },
  { label: "application bundle", pattern: /\.(?:app|dmg)$/iu },
  { label: "signing material", pattern: /\.(?:key|p12|pem)$/iu },
  {
    label: "local library data",
    pattern: /(?:^|\/)library\.json(?:\.bak)?$/iu,
  },
  { label: "environment secrets", pattern: /(?:^|\/)\.env(?:\.|$)/u },
];

const historicalViolations = historicalPaths.flatMap((file) =>
  forbiddenHistoricalPatterns
    .filter(({ pattern }) => pattern.test(file))
    .map(({ label }) => `${file} (${label})`),
);

if (historicalViolations.length > 0) {
  console.error("Reachable Git history contains forbidden paths:");
  for (const violation of historicalViolations) {
    console.error(`- ${violation}`);
  }
  process.exit(1);
}

const trackedPathScan = spawnSync(
  "git",
  ["grep", "-I", "-n", "-E", "/Users/[[:alnum:]_.-]+/", "--", "."],
  { encoding: "utf8" },
);

if (![0, 1].includes(trackedPathScan.status)) {
  throw new Error(trackedPathScan.stderr || "git grep failed");
}

if (trackedPathScan.stdout.trim()) {
  console.error("Tracked files contain absolute macOS user paths:");
  console.error(trackedPathScan.stdout);
  process.exit(1);
}

console.log(
  `Public-readiness check passed (${historicalPaths.length} reachable object paths inspected).`,
);
