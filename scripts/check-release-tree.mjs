import { execFileSync } from "node:child_process";

const trackedFiles = execFileSync("git", ["ls-files"], {
  encoding: "utf8",
})
  .split(/\r?\n/u)
  .filter(Boolean);

const forbiddenPatterns = [
  { label: "user audio", pattern: /\.(?:m4a|mp3|wav)$/iu },
  { label: "application bundle", pattern: /\.(?:app|dmg)$/iu },
  { label: "signing material", pattern: /\.(?:cer|key|p12|pem)$/iu },
  {
    label: "local library data",
    pattern: /(?:^|\/)library\.json(?:\.bak)?$/iu,
  },
  {
    label: "credential file",
    pattern: /(?:^|\/)(?:credentials?|secrets?)(?:\.|$)/iu,
  },
  { label: "environment secrets", pattern: /(?:^|\/)\.env(?:\.|$)/u },
];

const violations = trackedFiles.flatMap((file) =>
  forbiddenPatterns
    .filter(({ pattern }) => pattern.test(file))
    .map(({ label }) => `${file} (${label})`),
);

if (violations.length > 0) {
  console.error("Release tree contains forbidden tracked files:");
  for (const violation of violations) {
    console.error(`- ${violation}`);
  }
  process.exit(1);
}

console.log(
  `Release tree check passed (${trackedFiles.length} tracked files inspected).`,
);
