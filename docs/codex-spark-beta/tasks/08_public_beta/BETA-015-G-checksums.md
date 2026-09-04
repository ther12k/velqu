---
task_id: BETA-015-G
parent_task: BETA-015
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-015-G — Checksums

## Atomic goal

Checksums.

## Parent intent

Create a self-verifying public-beta packet.

## Dependencies

- `BETA-015-F` — `tasks/08_public_beta/BETA-015-F-sbom.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `conformance/security/security.conformance.test.ts`
- `crates/q-http/tests/fuzz_parsers.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Checksums.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Checksums verify from release directory.
- Artifacts map to one source commit.
- SBOM identifies dependencies/licenses.
- No stale historical metadata is current.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-schema-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Release packet.
- Verification transcript.
- Artifact inventory.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-015-g: checksums
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-015-G) — PASS (2026-09-05)

- Branch/PR: beta-015-g (squash-merged; see git log for final hash)
- Closes: #603

### Behavior implemented

Unified checksums deliverable of the beta release packet:
- `scripts/release-packet` now composes the SBOM (`scripts/sbom.sh`) and the 9 npm tarballs (`scripts/npm-package-tarballs.sh`) into the same `release/` directory during packet assembly.
- One top-level `SHA256SUMS.txt` covers every shipped file (packet artifacts, binaries, SBOM, npm tarballs), deterministically sorted (`LC_ALL=C`), verified via `sha256sum -c` from inside `release/`; the final line prints `CHECKSUMS-OK: <n> files covered`.
- Rehearsed at the clean packet commit: 20 files covered, all OK (transcript in the PR body).
- Fail-closed posture preserved: clean tree + release binaries required before any checksum is written.

### Changed files

- `scripts/release-packet` (SBOM + tarball composition; unified top-level checksum manifest)
- `docs/reports/beta-015-g-checksums.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-015-G-checksums.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `cargo test -p q-pack` — pass (100+2)
- `cargo test -p q-http` — pass (15)
- `cargo test -p q-schema-runtime` — pass (58)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` / `cargo clippy -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Packet assembly requires the release binaries and bun toolchain (fail-closed); publication remains Owner-gated.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
