/**
 * Build publishable dist/ output for every @velqu/* package.
 *
 * Per package (dependency order):
 * - `bun build` bundles src/index.ts → dist/index.js with `@velqu/*` and
 *   `typescript` kept external (real npm dependencies, never inlined);
 * - `tsc -p tsconfig.build.json` emits declaration files alongside.
 *
 * The emitted `exports` map in every manifest keeps `"bun":
 * "./src/index.ts"`, so Bun consumers (including this monorepo's own
 * tooling) always run fresh TypeScript sources; the dist/ JS output
 * serves non-Bun resolvers (Node ESM, generic bundlers) and `types`
 * points at the emitted declarations.
 *
 * Each package's dist directory is gitignored — this script regenerates
 * it; run it before `bun publish` (scripts/publish-beta.sh does).
 */
import { $ } from "bun";
import { rmSync } from "node:fs";

const PKGS = [
  { name: "contract", target: "bun", externals: [] },
  { name: "schema", target: "bun", externals: [] },
  { name: "core", target: "bun", externals: ["@velqu/*"] },
  { name: "treaty", target: "bun", externals: [] },
  { name: "browser-runtime", target: "browser", externals: [] },
  // C-003: optional local-SQL adapter; the PGlite engine is a real npm
  // dependency — always external, never inlined (lazy dynamic import).
  { name: "browser-pglite", target: "browser", externals: ["@electric-sql/pglite"] },
  { name: "compiler", target: "bun", externals: ["@velqu/*", "typescript"] },
  { name: "cli", target: "bun", externals: ["@velqu/*", "typescript"] },
] as const;

for (const { name, target, externals } of PKGS) {
  const pkgDir = `packages/${name}`;
  rmSync(`${pkgDir}/dist`, { recursive: true, force: true });
  const extArgs = externals.flatMap((e) => ["--external", e]);
  await $`bun build ${pkgDir}/src/index.ts --outdir ${pkgDir}/dist --target ${target} --format esm ${extArgs}`;
  await $`bun x tsc -p ${pkgDir}/tsconfig.build.json`;
  console.log(`built ${pkgDir} (target ${target})`);
}
console.log("all packages built");
