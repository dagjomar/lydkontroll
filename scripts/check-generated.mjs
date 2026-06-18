import { readdir, readFile } from "node:fs/promises";
import { spawnSync } from "node:child_process";

const bindingDirectory = new URL("../src/generated/", import.meta.url);

async function snapshotBindings() {
  const names = (await readdir(bindingDirectory))
    .filter((name) => name.endsWith(".ts"))
    .sort();
  return new Map(
    await Promise.all(
      names.map(async (name) => [
        name,
        await readFile(new URL(name, bindingDirectory), "utf8"),
      ]),
    ),
  );
}

const before = await snapshotBindings();
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

const after = await snapshotBindings();

if (
  before.size !== after.size ||
  [...before].some(([name, contents]) => after.get(name) !== contents)
) {
  process.stderr.write(
    "Generated TypeScript bindings were stale and have been refreshed. Review and commit the changes.\n",
  );
  process.exit(1);
}

process.stdout.write("Generated TypeScript bindings are current.\n");
