import { readFile } from "node:fs/promises";
import { spawnSync } from "node:child_process";

const bindingPath = new URL("../src/generated/AppMode.ts", import.meta.url);
const before = await readFile(bindingPath, "utf8");
const result = spawnSync(
  "cargo",
  ["test", "--manifest-path", "src-tauri/Cargo.toml", "export_bindings"],
  {
    encoding: "utf8",
    stdio: "pipe",
  },
);

if (result.status !== 0) {
  process.stderr.write(result.stdout);
  process.stderr.write(result.stderr);
  process.exit(result.status ?? 1);
}

const after = await readFile(bindingPath, "utf8");

if (before !== after) {
  process.stderr.write(
    "Generated TypeScript bindings were stale and have been refreshed. Review and commit the changes.\n",
  );
  process.exit(1);
}

process.stdout.write("Generated TypeScript bindings are current.\n");
