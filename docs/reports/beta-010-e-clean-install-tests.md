# BETA-010-E — Clean Install Tests

## Overview

Verified that the shared-mode deployment pair (`velqu-runtime` + `app.qpack`) installs, verifies, serves, and shuts down cleanly in an isolated, pristine directory without node_modules, source code, compiler tooling, or package manager state.

## Test Script & Procedure

`scripts/clean-install-test.sh`:
1. Creates an isolated temporary directory (`/tmp/velqu-clean-env-XXXXXX`).
2. Copies only the standalone release binary `velqu-runtime` and verified pack `app.qpack`.
3. Validates that `--fingerprint --pack app.qpack` executes and confirms `"verdict":"compatible"`.
4. Starts `velqu-runtime` on loopback; verifies `GET /health/live` returns `{"status":"ok"}` and `GET /hello/clean-env` serves the application handler.
5. Sends `SIGTERM` and confirms prompt, deterministic process exit.
6. Verifies fail-closed behavior: nonexistent pack or corrupt pack data exits non-zero before serving.

## Execution Transcript

```text
$ scripts/clean-install-test.sh
CLEAN-INSTALL-TEST-OK: pristine directory runtime+pack verified, served, and shut down cleanly
```

## Evidence & Boundaries

- **Pristine directory execution**: no source trees, package manifests, or compiler tooling present in the run directory.
- **Exact platform requirement**: Linux x86_64 glibc ELF binary.
- **Artifact integrity**: corrupt/missing pack exits with error before TCP listener admission.
- Conforms to the shared-mode installation guide in `docs/beta/INSTALL.md`.

## Gates

- `scripts/clean-install-test.sh` — `CLEAN-INSTALL-TEST-OK`
- `cargo test -p q-pack` — pass
- `cargo test -p q-engine-quickjs` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Shared-mode deployment expects the exact matching runtime build and pack pair; mismatched builds fail closed before ready.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
