---
task_id: M4A-008-Z
parent_task: M4A-008
milestone: M4A
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-008-Z — Package evidence for Build documentation and examples

## Atomic goal

Create source-backed evidence and handoff for parent task M4A-008; update status only if verification passed.

## Parent intent

Provide an honest, runnable learning path.

## Dependencies

- `M4A-008-V` — `tasks/07_m4a_developer_preview/M4A-008-V-verify-build-documentation-and-examples.md`

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
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Every code sample is tested.
- Docs distinguish measured facts from targets.
- No production-ready claim.
- Known limitations are prominent.

## Targeted commands

```bash
cargo test -p velqu-runtime
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

- Docs test output.
- Link check.
- Example CI.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m4a-008-z: package evidence for build documentation and examples
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-008-Z) — PASS (2026-09-01)

- Branch/PR: m4a-008-z (squash-merged; see git log for final hash)
- Closes: #482
- Parent verification: M4A-008-V PASS (PR #1086); this packet packages the
  source-backed documentation/example evidence and flips the M4A-008 parent
  ledger.

### Evidence package

- **Documentation set:** `docs/beta/QUICKSTART.md`, `ROUTES-SCHEMAS.md`,
  `TREATY.md`, `FETCH-CAPABILITIES.md`, `RUNTIME-PROFILES.md`,
  `DEPLOYMENT-REVERSE-PROXY.md`, and `LIMITS-AND-NON-GOALS.md`; linked from
  `docs/beta/INDEX.md` and `README.md`.
- **Source-backed proofs:** CLI init/build/inspect/dev paths; canonical route
  and schema contracts; policy/service fixtures; Treaty unit/runtime/remote
  modes; explicit fetch capability; serverless/service:N profiles;
  reverse-proxy-first deployment; bounded limits and non-goals.
- **Executable coverage:** proof app, scaffold, Treaty, runtime, schema,
  capability, and web conformance suites; generated QPack/contract/OpenAPI/
  lock artifacts through the proof build.

### Parent guardrail proofs

1. **Every code sample is tested** — docs examples mirror generated scaffold,
   proof routes/policy/service, Treaty conformance, fetch scaffold, and runtime
   CLI commands.
2. **Measured facts vs targets** — no performance claim appears in the guide
   set; limits/profile docs require retained matched p50/p95/p99 evidence.
3. **No production-ready claim** — all guides use private-alpha/beta/non-SLA
   wording; production-ready is explicitly prohibited in the limits guide.
4. **Known limitations prominent** — workspace packages, trusted-code QuickJS,
   fixture credentials, bounded defer/fetch, platform/API limits, reverse-
   proxy-first TLS, and no implicit autoscaling are explicit.

### Gate results

- Documentation local-link check → **PASS**
- `cargo test -p velqu-runtime` → PASS
- `bun test` → **309 pass / 0 fail (48 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy `-D warnings` → clean
- Proof build → PASS
- `./scripts/verify` → **ALL PASS**

### Ledger

- `docs/beta/04_TASK_LEDGER.md`: M4A-008 is now **PASS** through child A–G/V/Z
  evidence; subsequent M4A-009 and M4A-010 remain TODO.
- STATUS.md and TASK_INDEX.md: Z flipped TODO → PASS.

### Disclosures

- Evidence-only packet; no production behavior changes.
- Standing: CI verify workflows fail with zero executed steps since ~#714
  (infrastructure-side); disclosed per PR.
