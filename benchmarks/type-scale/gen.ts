/**
 * Deterministic synthetic route apps for the TypeScript scale spike.
 * Same shape per route; no cross-route type dependencies (avoids known
 * quadratic patterns).
 */
import { defineApp, defineModule, route } from "@velqu/core";
import { s } from "@velqu/schema";

const N = parseInt(process.argv[2] ?? "100", 10);
const outDir = process.argv[3];
if (!outDir) {
  console.error("usage: bun gen.ts <N> <outdir>");
  process.exit(1);
}

let src = `import { defineApp, defineModule, route } from "@velqu/core";\nimport { s } from "@velqu/schema";\n\n`;

const routes: string[] = [];
for (let i = 0; i < N; i++) {
  routes.push(`r${i}`);
  src += `const r${i} = route({
  id: "res${i}.get",
  method: "GET",
  path: "/res${i}/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: ${N} }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: ${N} }),
});
`;
}
src += `\nexport const app = defineApp({ id: "scale-${N}", modules: [ defineModule({ id: "res", routes: [${routes.join(", ")}] }) ] });\n`;

// client file: one positive + one negative call through the published-contract shape
let client = `import { treaty } from "@velqu/treaty";\nimport type { ProofApi } from "./api-types";\n\nconst api = treaty<ProofApi>({ baseUrl: "http://localhost:9", contract: {`;
for (let i = 0; i < N; i++) {
  client += `\n  "res${i}.get": { path: "/res${i}/item/:id", method: "GET" },`;
}
client += `\n} });\n\nconst ok = await api.res7.get({ id: 7 }).get();\nif (ok.data) console.log(ok.data.id);\n`;

let types = `import type { RouteContract } from "@velqu/contract";\n\nexport interface ProofApi {\n`;
for (let i = 0; i < N; i++) {
  types += `  "res${i}.get": RouteContract<"/res${i}/item/:id", "GET", { id: number }, Record<string, never>, undefined, { 200: { id: number; n: number } }>;\n`;
}
types += `}\n`;

await Bun.write(`${outDir}/app.ts`, src);
await Bun.write(`${outDir}/client.ts`, client);
await Bun.write(`${outDir}/api-types.ts`, types);
await Bun.write(
  `${outDir}/tsconfig.json`,
  JSON.stringify(
    {
      extends: "../../tsconfig.base.json",
      compilerOptions: {
        noEmit: true,
        baseUrl: ".",
        paths: {
          "@velqu/core": ["../../packages/core/src/index.ts"],
          "@velqu/schema": ["../../packages/schema/src/index.ts"],
          "@velqu/treaty": ["../../packages/treaty/src/index.ts"],
          "@velqu/contract": ["../../packages/contract/src/index.ts"],
        },
      },
      include: ["*.ts"],
    },
    null,
    2,
  ),
);
console.log(`generated ${N}-route app in ${outDir}`);
