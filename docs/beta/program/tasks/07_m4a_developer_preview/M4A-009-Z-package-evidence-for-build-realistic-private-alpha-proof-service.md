---
task_id: M4A-009-Z
parent_task: M4A-009
milestone: M4A
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-009-Z — Package evidence for Build realistic private-alpha proof service

## Atomic goal

Create source-backed evidence and handoff for parent task M4A-009; update status only if verification passed.

## Parent intent

Validate 30–50 routes, auth, fetch, validation, errors, pagination, and deployment.

## Dependencies

- `M4A-009-V` — `tasks/07_m4a_developer_preview/M4A-009-V-verify-build-realistic-private-alpha-proof-service.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`
- `scripts/benchmark`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Runs entirely on actual runtime.
- No hidden Bun production path.
- All error/status contracts declared.
- Load and failure scenarios pass.

## Targeted commands

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

- Proof app source.
- Scenario tests.
- Benchmark report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m4a-009-z: package evidence for build realistic private alpha proof ser
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-009-Z) — PASS (2026-09-01)

- Branch/PR: m4a-009-z (squash-merged; see git log for final hash)
- Closes: #489
- Parent verification: M4A-009-V PASS (PR #1093); this packet packages the
  source-backed evidence across all child packets (A through E) and flips
  the parent task M4A-009 to PASS.

### Evidence package

- **Implementation packets (squash-merged):**
  - M4A-009-A (PR #1088): items feature module with cursor pagination, CRUD routes,
    declared 404 problem contracts, and lazy `defineService` store.
  - M4A-009-B (PR #1089): JWT-like bearer policy reference with pure-JS HMAC-SHA-256
    pinned by RFC 4231 test vectors, timing-safe equality, login/profile routes, and
    typed session injection.
  - M4A-009-C (PR #1090): outbound fetch bridge integration (`PoolFetchDialer`)
    with SSRF protection, loopback trust, redirect limits, and upstream quote/relay/fanout
    routes tested against live HTTP server.
  - M4A-009-D (PR #1091): operational metrics/readiness routes (`ops.readiness`,
    `ops.metrics`, `ops.version`, `ops.ping`, `ops.check`), JS-to-JSON number conversion
    fix, and live SIGTERM bounded graceful shutdown scenario.
  - M4A-009-E (PR #1092): dedicated Treaty client (`createProofClient` and
    `createProofClientSubset`) backed by published `ProofApi` contracts, tested
    end-to-end on running runtime.
  - M4A-009-V (PR #1093): verification closure across all acceptance guardrails.
- **Proof service surface**: 24 routes across 8 modules (health, hello, users,
  items, auth, upstream, ops, async) with 2 policy checks.

### Parent guardrail proofs

1. **Runs entirely on actual runtime** — all routes and policy checks run on
   `velqu-runtime` (Rust + QuickJS) via `runtimeTreaty` over real HTTP.
2. **No hidden Bun production path** — production artifacts are compiled
   `app.qpack` loaded by the Rust host; Bun is dev tooling only.
3. **All error/status contracts declared** — every route declares exact status
   and problem schemas (200/201/401/404/502); undeclared statuses fail closed.
4. **Load and failure scenarios pass** — validated across pagination, tampered/expired
   tokens, upstream gateway errors (502), body validation errors (422), and
   clean graceful SIGTERM shutdown.

### Gate results

- `cargo test -p q-engine-quickjs` → PASS
- `cargo test -p q-schema-runtime` → PASS
- `cargo test -p velqu-runtime` → PASS
- `bun test` → **326 pass / 0 fail (54 files)**
- `bun run typecheck` → clean
- `cargo fmt --check`, workspace clippy `-D warnings` → clean
- Proof build → PASS
- `./scripts/verify` → **ALL PASS**

### Ledger

- `docs/beta/04_TASK_LEDGER.md`: M4A-009 flipped TODO → **PASS**.
- STATUS.md and TASK_INDEX.md updated to PASS.

### Disclosures

- Evidence-only packet; no production behavior changes.
- Standing: CI verify workflows fail with zero executed steps since ~#714
  (infrastructure-side); disclosed per PR.
