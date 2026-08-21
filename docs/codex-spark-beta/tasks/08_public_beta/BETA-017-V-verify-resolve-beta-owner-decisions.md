---
task_id: BETA-017-V
parent_task: BETA-017
milestone: BETA
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-017-V — Verify Resolve beta owner decisions

## Atomic goal

Prove every acceptance criterion for parent task BETA-017 without broadening scope.

## Parent intent

Close only the decisions necessary to publish a public beta.

## Dependencies

- `BETA-017-A` — `tasks/08_public_beta/BETA-017-A-repository-organization.md`
- `BETA-017-B` — `tasks/08_public_beta/BETA-017-B-license-contribution-model.md`
- `BETA-017-C` — `tasks/08_public_beta/BETA-017-C-release-authority.md`
- `BETA-017-D` — `tasks/08_public_beta/BETA-017-D-security-contact.md`
- `BETA-017-E` — `tasks/08_public_beta/BETA-017-E-supported-beta-platforms.md`
- `BETA-017-F` — `tasks/08_public_beta/BETA-017-F-reverse-proxy-first-statement.md`
- `BETA-017-G` — `tasks/08_public_beta/BETA-017-G-public-benchmark-wording.md`

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
- `packages/auth-jwt/ (create if absent)`
- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-http/tests/fuzz_parsers.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
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

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-017-v: verify resolve beta owner decisions
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
## Completion record

- Status: **PASS**
- Dependencies confirmed: BETA-017-A..G all `status: PASS` on this branch; OD-BETA-001..006 and OD-BETA-008 accepted in `docs/beta/governance/OPEN_DECISIONS.md` with dated decision records; OD-001..009 decided in `docs/open-decisions.md`. OD-BETA-007 (Postgres package; blocks Package list) and OD-BETA-009 (support channel; blocks Beta docs) are outside BETA-017's seven-decision scope and tracked by their own gates.
- Acceptance criteria mapped:
  - Decisions recorded in open-decision log — seven template-aligned decision records in `docs/beta/governance/OPEN_DECISIONS.md`; register cross-links in `docs/open-decisions.md`.
  - No agent invents owner authority — each record names Owner (ther12k) and date; owner-selected options documented in the A–G completion records and PR history (#660, #663, #665, #673, #681, #696, #711).
  - Security reporting channel exists — `SECURITY.md` (private GitHub Security Advisories; negative guarantee: public issues are not a disclosure channel), linked from `CONTRIBUTING.md`.
  - Platform/support scope published — `docs/beta/governance/PLATFORM_SUPPORT.md` (Linux x86_64 glibc only), `REVERSE_PROXY_POLICY.md` (proxy-terminated TLS), `BENCHMARK_WORDING.md` (scoped evidence-only claims), `RELEASE_AUTHORITY.md` (0.1.0-beta.1, owner-controlled).
- Tests and exact results: `cargo test -p q-pack` PASS (37 unit + 2 fuzz); `cargo test -p q-http` PASS (4 unit + 6 fuzz + 1 regression corpus); `cargo test -p q-schema-runtime` PASS (9 unit + 2 fuzz); `bun test` PASS (36/36, runtime-local conformance drove the release binary over HTTP); `bun run typecheck` PASS; `cargo fmt --all --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS; `./scripts/verify` — `verify: ALL PASS` including benchmark artifact validation (`errors: []`; reproducible-build flags from 75bda51 make fresh-worktree artifact hashes match the manifest, resolving the environment-bound caveat recorded in BETA-017-G).
- Fail-before-ready/cleanup inspection: runtime rejects tampered packs before ready (`tampered_pack_fails_before_ready`), poisoned runtime marks readiness false, graceful shutdown exits zero, queue saturation returns 503 — no weakened assertions found.
- Findings handled here: none (no defect in the seven decisions' artifacts required fixes). The BETA-017-G artifact-hash caveat was already resolved upstream by reproducible builds.
- Remaining risk / deferred by design: OD-BETA-007 and OD-BETA-009 remain open by design for their own gates; the broader BETA-GATE checklist items (clean-install tutorial, soak, packaging) belong to BETA-014/015/016 and BETA-GATE.
- Next dependency-ready task: BETA-017-Z (Package evidence for Resolve beta owner decisions) — #624.
- Working tree clean: yes after commit.
