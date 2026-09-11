# Publishing `@velqu/*` to npm — Owner Runbook

**Status: PUBLISHED (2026-09-09).** All seven packages are live on npm
as `0.1.0-beta.1` under the **`beta`** dist-tag (`latest` intentionally
unset; moves only by a recorded owner decision). Install with e.g.
`bun add @velqu/core@beta`. The packaging invariants below (tarball
layout, kernel assets, workspace-dep replacement) remain enforced by
`packages/publishing/src/publishing.test.ts`. Decision record:
`docs/open-decisions.md` OD-010 / BWASM OD-050.

> Publishing incident, for the record: `@velqu/browser-runtime` was
> uploaded in the owner's first run, but the registry's read path
> (packument) returned 404 for ~15-20 minutes after the version document
> was already reachable — plain registry propagation lag, not a phantom
> publish. During that window a republish attempt correctly failed with
> "cannot publish over previously published versions". No action was
> needed; the package became visible on its own.

## One-time setup

1. ~~Create the `velqu` npm organization~~ — **done by the owner**
   (2026-09-09). The org owns the `@velqu/*` scope.
2. Enable 2FA on the npm account (Account Settings → Security).
3. Local publish needs no token: run `npm login` (or `bun pm login`).
   A granular Automation token is only needed if CI publishes later.

## What was fixed for publishability

The cleanroom evaluation (BWASM-Q-007) proved hand-rolled tarballs were
uninstallable. This packet fixes the real causes:

| Defect | Fix |
| --- | --- |
| D1: tarball entries at archive root (no `package/` prefix), `private: true` manifests with `workspace:*` inter-deps | Real manifests (`bun publish`/`bun pm pack` produce npm-standard layout and replace `workspace:*` with concrete versions — asserted in the packaging tests) |
| D3: `@velqu/browser-runtime` shipped no kernel | `files` includes `kernel/` (vendored, hash-pinned; asserted) |
| bin was raw TypeScript | `packages/cli/bin/velqu.js` — `#!/usr/bin/env bun` shim importing built `dist/index.js`; `engines: bun >=1.4.0` declared |
| Hidden dependency edges | `@velqu/core` now declares `@velqu/schema`; `@velqu/cli` declares `@velqu/browser-runtime` (kernel assets) and `@velqu/compiler` |
| Sources-only entry points | Every package builds `dist/index.js` (ESM) + `dist/index.d.ts`; `exports` maps `types → dist d.ts`, `bun → src/index.ts` (Bun consumers always run fresh sources, including this monorepo), `default → dist` (Node ESM / generic bundlers) |
| Boundary violation | `@velqu/cli` no longer imports compiler sources by relative path; the symbol is exported from `@velqu/compiler`'s public entry |

## Publish flow

```bash
# 1. Inspect what would be published (no auth needed, nothing sent)
bun run publish:beta          # == scripts/publish-beta.sh --dry-run

# 2. Owner decision recorded (OD-050), then:
npm login                     # once per machine
bun run publish:beta:real     # dependency order; skips already-live versions
bash scripts/publish-beta.sh --only browser-pglite   # just one package
```

Publish order (dependency graph): `contract` → `schema` → `core` →
`treaty` → `browser-runtime` → `browser-pglite` → `compiler` → `cli`.

**Re-runs are idempotent (since #1315).** In real mode the script checks
the registry first and SKIPs any package whose exact version is already
live (the registry rejects republishing), so a re-run proceeds to the
packages still missing instead of dying at the first published one under
`set -e`. `--only <pkg>` restricts the run to a single package — safe
once the target's `@velqu/*` dependencies are already on the registry
(`browser-pglite` has none, which the packaging test asserts).
`@velqu/browser-pglite` (BWASM-C-003) is wired into the order and
qualified by the packaging tests; its first publish is an owner action
per OD-050. `--dry-run` performs no registry queries.

## Tag discipline

- Everything publishes under **`--tag beta`** — never `latest` while the
  project is pre-GA. `latest` moves only by a recorded owner decision.
- Install command for consumers: `bun add @velqu/core@beta` (or
  `@0.1.0-beta.1`).

## Post-publish verification

```bash
npm info @velqu/compiler@0.1.0-beta.1 dependencies   # workspace:* replaced
cd "$(mktemp -d)" && bun add @velqu/core@beta @velqu/schema@beta
bunx velqu --help                                    # via @velqu/cli@beta
```

## Honesty notes

- The CLI requires Bun at runtime (shebang + `engines`); it is a
  dev-time build tool — production execution remains the Rust
  `velqu-runtime` binary.
- `@velqu/browser-runtime` is browser-only; its vendored kernel is
  hash-pinned and builds fail closed on mismatch.
- Published packages include TypeScript sources (the `bun` exports
  condition) alongside built `dist/` output — deliberate, not an
  accident.
- Publishing before the Browser-WASM GO/NO-GO gate (#1179) is a product
  decision for the owner; nothing in the packaging depends on that gate.
