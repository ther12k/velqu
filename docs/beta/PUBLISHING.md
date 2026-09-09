# Publishing `@velqu/*` to npm — Owner Runbook

**Status: publish-ready, publication owner-gated.** The seven workspace
packages are packaged, version-pinned to `0.1.0-beta.1`, and verified by
`packages/publishing/src/publishing.test.ts` (tarball layout, kernel
assets, workspace-dep replacement). The actual publication is an Owner
decision — record it under OD-050 (release channel) in
`docs/open-decisions.md` before the first real publish (AGENTS.md §13).

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
bun run publish:beta:real     # publishes all seven, --tag beta, dependency order
```

Publish order (dependency graph): `contract` → `schema` → `core` →
`treaty` → `browser-runtime` → `compiler` → `cli`.

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
