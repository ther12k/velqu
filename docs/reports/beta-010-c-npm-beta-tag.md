# BETA-010-C — npm Packages Under the beta Tag

## Result

**Prepared, not published.** The workspace contains nine `@velqu/*` package
manifests, all currently marked `private: true`, so there are zero publishable
packages and no package was sent to npm. This is intentional: the Owner is the
sole release authority and the repository/license publication decision remains
open. The machine-readable inventory is
`docs/reports/beta-010-c-npm-package-inventory.json`.

## Inventory

`scripts/npm-package-inventory.sh` inspects every `packages/*/package.json`
without contacting npm and records name/version/private/type/entrypoints,
dependencies, publishConfig, license, repository, and publication status.

- 9 package manifests discovered.
- 9 private packages; 0 publishable packages.
- All current versions are `0.1.0`; package publication is not authorized by
  this task.
- No accidental source/compiler artifact is produced or uploaded.

## Publication contract when authorized

A later owner-authorized release packet must first decide repository/license
metadata, set a beta semver version (the authorized release line is
`0.1.0-beta.1`), add explicit `publishConfig`/access policy as appropriate,
run clean tarball inventory, and publish with the `beta` dist-tag. Publication
must be bound to one source commit and include checksums, SBOM, package
inventory, and install transcript. `npm publish` is deliberately not run here.

## Companion policy

- `docs/beta/governance/RELEASE_AUTHORITY.md`: Owner-only publish/withdraw/yank
  authority and required release evidence.
- `docs/beta/governance/PLATFORM_SUPPORT.md`: exact supported platform promise.
- `docs/beta/LIMITS-AND-NON-GOALS.md`: non-SLA, trusted-code-only boundaries.
- `docs/reports/beta-010-a-linux-x86-64-glibc-platform.md`: runtime platform
  artifact evidence.

## Gates

- `scripts/npm-package-inventory.sh` — pass (`PREPARED_NOT_PUBLISHED`)
- `bun install --frozen-lockfile` — pass
- `bun test` — 434 pass / 0 fail
- `bun run typecheck` — pass
- No npm network publication attempted.

## Disclosure

This packet proves safe publication preparation and correctly refuses to claim
an npm beta tag while every package is private and owner authorization/metadata
is unresolved. It does not expand package API stability, platform support, or
release authority.
