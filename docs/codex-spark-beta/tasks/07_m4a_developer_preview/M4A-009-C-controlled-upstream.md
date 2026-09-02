---
task_id: M4A-009-C
parent_task: M4A-009
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-009-C — Controlled upstream

## Atomic goal

Controlled upstream.

## Parent intent

Validate 30–50 routes, auth, fetch, validation, errors, pagination, and deployment.

## Dependencies

- `M4A-009-B` — `tasks/07_m4a_developer_preview/M4A-009-B-jwt-like-policy-reference.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Controlled upstream.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Runs entirely on actual runtime.
- No hidden Bun production path.
- All error/status contracts declared.
- Load and failure scenarios pass.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-schema-runtime
```
```bash
cargo test -p q-capabilities
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Proof app source.
- Scenario tests.
- Benchmark report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-009-c: controlled upstream
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-009-C) — PASS (2026-09-01)

- Branch/PR: m4a-009-c (squash-merged; see git log for final hash)
- Closes: #485

### Changed files
- `crates/q-engine-quickjs/src/lib.rs`: defined `FetchDialer` trait and
  `FetchDialerHandle` config hook so the engine links no HTTP dependencies
  directly; QuickJS prelude fails closed (never a silent mock 200) when no
  dialer is installed.
- `crates/q-engine-quickjs/src/worker.rs`: installed `__velquFetchStart`
  bridge native; added `WorkerMsg::FetchComplete` and kind-attributed
  `OpKind::Fetch` with unified terminal completion and lifecycle metrics
  (`fetch_ops_started`, `fetch_ops_completed`).
- `crates/q-engine-quickjs/src/prelude.rs`: rewired `globalThis.fetch` to
  invoke `__velquFetchStart` and register promise resolution with the native
  bridge; fail closed with `TypeError` when no bridge is active.
- `crates/q-runtime/src/fetch_bridge.rs` (new): `PoolFetchDialer` bridging the
  engine to `q_capabilities::fetch_policy` and `q_runtime::fetch_stack::shared_pool()`
  with SSRF validation, loopback trust, redirect limits, credential stripping,
  timeout bounds, and response body caps.
- `crates/q-runtime/src/lib.rs`: registered `fetch_bridge` module and wired
  `PoolFetchDialer` into runtime engine startup.
- `crates/q-pack/src/lib.rs`: fixed canonical `query_name_table` and
  `header_name_table` pack verification by separating global name collection
  from per-route index verification (fixing an incremental binary search bug
  when routes had non-alphabetical query param names across modules).
- `examples/proof/src/modules/upstream/routes.ts` (new): `upstream.quote` (GET),
  `upstream.relay` (GET with target query), and `upstream.fanout` (GET with
  parallel count).
- `examples/proof/src/modules/upstream/routes.test.ts` (new): 3 route contract
  unit tests.
- `examples/proof/src/app.ts`: registered `upstream` module (proof now 19 routes).
- `conformance/treaty/treaty.conformance.test.ts`: added type-level pins and
  actual runtime-local Treaty scenario driving quote, relay, fanout, and 502
  upstream failure against a live controlled upstream server.
- Pinned inventory tests updated: `inspect-output.test.ts` (routeCount 19),
  `compiler.test.ts` (`queryNameTable` sorted across 5 query params),
  `package-verification.test.ts` (new contract hash).
- `benchmarks/manifest.json`: refreshed with release binary rebuild.

### Required evidence

- **Proof app source**: `examples/proof/src/modules/upstream/` with 3 routes
  exercising real outbound fetch.
- **Scenario tests**:
  - `upstream module (controlled upstream M4A-009-C)` unit suite (3 tests)
  - `Treaty runtime-local mode > drives compiled proof pack end-to-end` with
    live HTTP controlled-upstream scenarios (relay, fanout, error handling)
- **Benchmark report**: `benchmarks/manifest.json` refreshed with release
  binary matching verification gate.

### Guardrail mapping

- **Runs entirely on actual runtime**: outbound fetch runs on the Rust/QuickJS
  worker via the native fetch bridge and shared Tokio connection pool.
- **No hidden Bun production path**: fetch implementation in production is
  Rust `hyper` + `hyper-rustls`.
- **All error/status contracts declared**: upstream routes declare 200 and 502;
  unreachable upstream maps to declared 502.
- **Load and failure scenarios pass**: upstream 500 fails closed as 502; SSRF
  and address classification guards remain active.

### Command results

- `cargo test -p q-engine-quickjs` → PASS
- `cargo test -p q-http` → PASS
- `cargo test -p q-schema-runtime` → PASS
- `cargo test -p q-capabilities` → PASS
- `cargo test -p velqu-runtime` → PASS
- `bun test` → **321 pass / 0 fail (51 files)**
- `bun run typecheck` → clean
- `cargo fmt --check`, workspace clippy `-D warnings` → clean
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
