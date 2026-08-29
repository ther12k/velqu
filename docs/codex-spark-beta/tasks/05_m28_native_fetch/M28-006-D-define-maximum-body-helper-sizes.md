---
task_id: M28-006-D
parent_task: M28-006
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-006-D — Define maximum body helper sizes

## Atomic goal

Define maximum body helper sizes.

## Parent intent

Support large bodies without unbounded buffering.

## Dependencies

- `M28-006-C` — `tasks/05_m28_native_fetch/M28-006-C-cancel-on-consumer-stop-disconnect.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `Cargo.toml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define maximum body helper sizes.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Large response does not allocate full body unless requested.
- Slow upstream/downstream remains bounded.
- Cancellation releases buffers/connections.
- Streaming errors are typed.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-bridge
```
```bash
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Streaming load tests.
- Slow consumer tests.
- Memory profile.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-006-d: define maximum body helper sizes
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-006-D) — PASS

- Date: 2026-08-29
- Branch/PR: m28-006-d (squash-merged; see git log for final hash)
- Closes: #339

### Changed files
- `crates/q-capabilities/src/fetch_policy.rs`: maximum body helper sizes, one contract for types + policy + enforcement —
  - `MAX_BODY_HELPER_BYTES` = 16 MiB, pinned equal to `MAX_FETCH_RESPONSE_BODY_BYTES` (ADR-0033 §9): a helper can never be asked to materialize more than the network layer admits; also ≤ `MAX_TEXT_BUFFER_LEN` (decode path stays within the text-encoding bound).
  - `BodyHelper` enum (ResponseText/ResponseJson/ResponseArrayBuffer/ResponseBytes) with stable `name()` and per-helper `max_bytes()` (shared cap today; named accessors keep future tightening a one-line change).
  - `check_body_helper_size(helper, byte_len)` — typed fail-closed check.
  - `FetchPolicyError::BodyTooLarge { helper, len, max }` (new variant) with a Display naming the helper and the cap.
- `crates/q-capabilities/src/lib.rs`: re-exports (`BodyHelper`, `check_body_helper_size`, `MAX_BODY_HELPER_BYTES`).
- `crates/q-engine-quickjs/src/worker.rs`: `__velquBodyHelperLimit()` native binding returning the pinned constant — single source of truth for the JS layer.
- `crates/q-engine-quickjs/src/prelude.rs`: `__velquBodyHelperCheck(helper, body)` — `text()`/`arrayBuffer()`/`bytes()` (and `json()` via `text()`) measure the body (typed arrays: `byteLength`; strings: the native encode bridge, whose over-ceiling throw maps to the typed helper error) and throw a synchronous `TypeError` naming the helper and the cap BEFORE `bodyUsed` flips or any derived copy is made. Guarded on the binding so the plain Web surface still runs under dev tooling; the production worker always installs it.

### Tests added
Unit (fetch_policy.rs, +2 → 149 q-capabilities lib tests):
- `body_helper_sizes_are_defined_and_fail_closed` (per-helper caps, boundary ok at cap, typed rejection above, Display names helper + "maximum helper size")
- `body_helper_cap_composes_with_network_and_text_limits` (== network response cap; == 16 MiB; const-assert ≤ MAX_TEXT_BUFFER_LEN)
Worker (worker.rs, +1 → 18):
- `body_helper_sizes_fail_closed_above_native_cap` — limit binding == 16777216; oversized Uint8Array body: bytes/arrayBuffer/text all throw the typed TypeError with `bodyUsed` still false; oversized string body maps through the encode bridge to the same typed error; json() inherits via text(); within-limit and null bodies materialize/consume normally.

### Command results
- `cargo test -p q-capabilities` → **149 unit + 4 backpressure + 8 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → **18 (was 17) + 101** · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- `benchmarks/manifest.json`: `qRuntimeRelease` hash refreshed — the prelude is embedded in the binary, so this packet legitimately changes the release artifact (unlike M28-006-B/C).

### Guardrail mapping
- **Large response does not allocate full body unless requested** — helpers are exactly the "requested" materialization points; they now refuse (typed, before copying) above the cap.
- **Streaming errors are typed** — `BodyTooLarge` joins the closed policy error set; the JS surface gets a named `TypeError`.
- **Memory profile** — the cap bounds every materializing helper at 16 MiB regardless of JS string/UTF-16 expansion.

### Disclosures
- One clippy iteration: `assertions_on_constants` (composition pin moved to a module-level `const _` assert) and an unused closure param (`_ctx`). No behavior change; all tests green after each fix.
- Fresh-worktree transients (velqu-bytecode helper, proof pack) resolved by the standard setup sequence before final gate runs.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
