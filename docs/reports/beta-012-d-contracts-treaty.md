# BETA-012-D — Contracts/Treaty

## Overview

Updated the Contracts/Treaty documentation to public-beta accuracy and added the missing contract-lock workflow that makes the "Contracts" half operational:

- `docs/beta/TREATY.md`:
  - New **Contract lock and diff** section documenting `velqu contract diff` with both tested outcomes: clean diff (`contract diff: no changes`, exit 0) and breaking detection (`breaking users.delete: route removed`, exit 2), plus the design consequence that a breaking-contract exit also means regenerated clients stop compiling against the old surface — by design.
  - Closing framing updated from "private-alpha evidence" to "public-beta evidence".
- `docs/beta/ROUTES-SCHEMAS.md`: closing evidence line updated from "private-alpha toolchain" to "public-beta toolchain".

## Every command/sample tested (2026-09-04, this worktree)

- `bun packages/cli/src/index.ts build --project examples/proof` — OK.
- `bun packages/cli/src/index.ts contract diff --project examples/proof` — "no changes", exit 0.
- Breaking path (old lock with an extra declared route `users.delete`): output shows `compatible users.create: route added` + `breaking users.delete: route removed`, exit 2 (BREAKING_CONTRACT).
- Compatible path (lock missing a route the new build declares): `users.create: route added`, exit 0.
- `bun test conformance/treaty examples/proof` — 21 pass / 0 fail (8 files), including runtime-local actual-binary Treaty tests.
- `bun run typecheck` — pass.

## Link check

`TREATY.md` and `ROUTES-SCHEMAS.md` reference `QUICKSTART.md` and repo paths that all exist; no new links added beyond repo paths verified above.

## Gates

- `cargo test -p velqu-runtime` — pass (8 suites ok)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Documentation change only; no runtime behavior modified.
- The breaking-diff sample used a temporary modified lock file outside the repository; nothing shipped.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
