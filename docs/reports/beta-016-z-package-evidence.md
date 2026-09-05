# BETA-016-Z — Package Evidence for Run External Clean-install and Tutorial Verification

## Overview

Evidence packaging and handoff for parent task **BETA-016** ("Run
external clean-install and tutorial verification"). All seven child
packets (A–F, V) were delivered, merged, and verified; this packet binds
them into one evidence record and flips the parent ledger entry to PASS.

## BETA-016 packet inventory (all merged)

| Packet | Deliverable | PR | Report |
|---|---|---|---|
| BETA-016-A | Fresh Linux environment (digest-pinned container, Bun 1.4.0 + Rust 1.96.0, unprivileged user, fail-closed manifest probe) | #1214 | `docs/reports/beta-016-a-fresh-linux-vm-container.md` |
| BETA-016-B | External install of CLI/runtime from source archive (`INSTALL-OK`) + tooling-home forward fix | #1215 | `docs/reports/beta-016-b-install-cli-runtime.md` |
| BETA-016-C | External scaffold via QUICKSTART command verbatim (`SCAFFOLD-OK`, 3 routes clean) | #1216 | `docs/reports/beta-016-c-scaffold-app.md` |
| BETA-016-D | External tests/dev/build (`DEVBUILD-OK`) + three product fixes (scaffold scripts, dev-server runtime resolution, QUICKSTART `cli` link) | #1217 | `docs/reports/beta-016-d-run-tests-dev-build.md` |
| BETA-016-E | External deploy of the proof service behind an operator edge (`VERIFY-OK`) + rollback (`ROLLBACK-OK`) | #1218 | `docs/reports/beta-016-e-deploy-proof-service.md` |
| BETA-016-F | External Treaty client journey (`TREATY-OK`, no-skip contract tests) + E-rollback correction + process-hygiene fixes | #1219 | `docs/reports/beta-016-f-use-treaty-client.md` |
| BETA-016-V | Parent verification closure (all guardrails re-confirmed, closure-time hygiene) | #1220 | `docs/reports/beta-016-v-verify-external-journey.md` |

## Parent acceptance guardrails — final evidence mapping

| Guardrail | Evidence | Result |
|---|---|---|
| No local unpublished dependency | B: install from a sha256-recorded `git archive` tarball; deps resolve through documented `node_modules/@velqu/*` links; no publication assumed | PASS |
| Tutorial succeeds verbatim | C/D/F: QUICKSTART + INSTALL.md commands and bodies word-for-word (made verbatim-true by D's fixes: linked-path CLI, `--project .`, install-tree runtime resolution, `cli` link step) | PASS |
| Failures produce actionable diagnostics | Demonstrated in practice: D's three step-numbered defect diagnostics; E's privilege refusal; F's pre-flight guard naming the leftover-service cause; corrected rollback refusing a missing pidfile | PASS |
| Artifacts can be rolled back/uninstalled | Install/scaffold/app each a single `rm -rf`; E: `ROLLBACK-OK` with edge closed + upstream released + artifacts removed, post-rollback verify fails closed; F: teardown releases the port, immediate re-run clean | PASS |

## Cross-journey issues and resolutions (consolidated)

1. Archive without root directory broke extraction → `--prefix=velqu/` + guard (B).
2. Root-owned tooling homes broke unprivileged builds → chown + probe writability (B).
3. Scaffold scripts assumed a global `velqu` binary → linked-path invocation (D).
4. CLI default project leaked a monorepo assumption → `--project .` (D).
5. Dev server could not find the runtime externally → install-tree candidates + named fix path (D).
6. QUICKSTART link step omitted `cli` → documentation fixed (D).
7. E rollback resolved root's `$HOME`; service stop silently skipped → owner-aware paths, fail-closed pidfile, port-release assertions; lifecycle re-run proven (F).
8. Zombie processes defeated `kill -0` liveness; `pkill` absent → kernel-state reads + `/proc`-scan teardown (F).
9. `create --help` scaffolds instead of printing help → recorded as diagnostics wart, out of scope (C).

## Environment manifest

Image `velqu-beta-external:0.1.0-beta.1`
(digest `sha256:9076de16f6ec…a2f5570` after the B forward fix): Debian 12
bookworm x86_64, Bun 1.4.0, Rust 1.96.0 (minimal, repository lockfile),
unprivileged `beta` user, tooling homes writable, fail-closed probe
`MANIFEST-OK`. `fresh=no-velqu-material` is the pre-install image gate;
intentionally false post-journey. External transcripts retained in the
verification container's `~/out/` and in the per-packet reports.

## Verification transcript (targeted commands, this worktree)

- `cargo test -p q-pack` — pass (40+2)
- `cargo test -p velqu-runtime` — pass (37+3)
- `bun test` (in `unshare -rn` netns) — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS

## Artifact inventory

- External verification tooling (merged): `scripts/beta-external/` —
  `Dockerfile`, `manifest.sh`, `build-env.sh`, `install-cli-runtime.sh`,
  `scaffold-app.sh`, `run-tests-dev-build.sh`,
  `deploy-proof-service.sh`, `use-treaty-client.sh`.
- Product fixes carried by the journey (merged): scaffold script
  contract (`packages/cli/src/scaffold.ts`),
  dev-server runtime resolution (`packages/cli/src/dev-server.ts`),
  scaffold-test assertions
  (`packages/cli/src/profile-fetch-choices.test.ts`),
  `docs/beta/QUICKSTART.md`.
- Status bindings updated in this packet: task record
  `docs/codex-spark-beta/tasks/08_public_beta/BETA-016-Z-…md` (TODO →
  PASS + Result), `docs/codex-spark-beta/STATUS.md` checkbox,
  `docs/codex-spark-beta/indexes/TASK_INDEX.md` row, parent ledger
  `docs/beta/04_TASK_LEDGER.md` (`BETA-016` TODO → PASS).

## Disclosures

- Evidence packaging only; no runtime behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed
  steps at PR creation since roughly #714; local gates/evidence are the
  acceptance basis.
