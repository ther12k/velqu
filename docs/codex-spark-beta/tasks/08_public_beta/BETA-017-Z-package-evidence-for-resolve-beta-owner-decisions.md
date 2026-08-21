---
task_id: BETA-017-Z
parent_task: BETA-017
milestone: BETA
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-017-Z — Package evidence for Resolve beta owner decisions

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-017; update status only if verification passed.

## Parent intent

Close only the decisions necessary to publish a public beta.

## Dependencies

- `BETA-017-V` — `tasks/08_public_beta/BETA-017-V-verify-resolve-beta-owner-decisions.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

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
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Decisions are recorded in ADR/open-decision log.
- No agent invents owner authority.
- Security reporting channel exists.
- Platform/support scope is published.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```
```bash
./scripts/validate-okf
```

## Required evidence for this microtask

- Accepted decision records.
- Release authorization.
- Contact/support document.
- [ ] No open beta P0/P1 findings or unapproved waivers.
- [ ] A clean external user can install, scaffold, develop, test, build, and deploy a real Velqu app.
- [ ] Fetch, multi-worker service mode, Treaty, optional Postgres, and auth reference work on the actual runtime.
- [ ] Real-world, cold, warm, and CPU/JIT evidence is reproducible and honestly reported.
- [ ] Security baseline, two-hour/one-million-request soak, observability, config, proxy/drain, and clean packaging pass.
- [ ] Release packet is self-verifying and owner decisions are closed.
- [ ] Release is labeled beta, non-SLA, trusted-code-only, and not production-ready GA.
- Canonical microbenchmarks with repetitions.
- Real PostgreSQL W1/W2/W3.
- Controlled I/O and fan-out.
- CPU/JIT crossover.
- 1/2/4 worker scaling.
- Two-hour/one-million-request soak.
- No GA/SLA claim.
- No full Node/Bun compatibility.
- No hostile tenant sandbox.
- No WebSocket/SSE.
- No ORM in core.
- No Windows/macOS support promise unless separately accepted.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-017-z: package evidence for resolve beta owner decisions
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
## Completion record

- Status: **PASS**
- Parent verification confirmed: BETA-017-V PASS at `924d4c2` (merge #712); tree clean before evidence commit.
- Evidence collected (source paths):
  - Decision records: `docs/beta/governance/OPEN_DECISIONS.md` (OD-BETA-001..006, 008 accepted; template-aligned records), `docs/open-decisions.md` (OD-001..009 decided).
  - Release authorization: `docs/beta/governance/RELEASE_AUTHORITY.md` (Owner sole authority; `0.1.0-beta.1`).
  - Contact/support documents: `SECURITY.md` (private GitHub Security Advisories channel), `CONTRIBUTING.md`.
  - Platform/support scope: `docs/beta/governance/PLATFORM_SUPPORT.md`, `REVERSE_PROXY_POLICY.md`, `BENCHMARK_WORDING.md`.
  - Subtask completion records: `docs/codex-spark-beta/tasks/08_public_beta/BETA-017-{A..G,V}-*.md` (PRs #660, #663, #665, #673, #681, #696, #711, #712).
- Status updated: `docs/beta/04_TASK_LEDGER.md` BETA-017 → PASS (parent acceptance criteria proven by BETA-017-V).
- Index/checksum binding checked: root `REVIEW_INDEX.json`/`EVIDENCE_INDEX.json` remain M24 milestone indexes using the `BOUND_BY_RELEASE_PACKET_TO_CLEAN_HEAD` placeholder; `scripts/release-packet` rewrites commit/releaseCommit/generatedAt to the clean candidate HEAD at packet time; `reviewedImplementationCommit` 75bda51 is an ancestor of HEAD. Beta-milestone indexes are deferred to BETA-GATE packaging by design.
- Tests and exact results: `cargo test -p q-pack` PASS (37 unit + 2 fuzz); `cargo test -p q-engine-quickjs` PASS (1 unit + 96 engine); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS; `./scripts/verify` — `verify: ALL PASS` (benchmark artifact validation `errors: []`); `./scripts/validate-okf` PASS (174 links, manifest hashes).
- Reports unchanged: raw benchmark evidence and generated reports untouched; `generate-benchmark-reports.py --check` reports current (inside verify).
- Remaining risk / deferred by design: OD-BETA-007 (Postgres package) and OD-BETA-009 (support channel) stay open for their own gates; BETA-GATE owns the final self-verifying release packet.
- Next dependency-ready task: BETA-GATE remains blocked on BETA-001..BETA-016 (minus 017); M-line work continues at M25-001-A (#120).
- Working tree clean: yes after commit.
