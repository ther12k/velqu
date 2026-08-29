---
task_id: M28-007-D
parent_task: M28-007
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-007-D — Bound decompression ratio and output

## Atomic goal

Bound decompression ratio and output.

## Parent intent

Handle redirects and encoded responses safely and predictably.

## Dependencies

- `M28-007-C` — `tasks/05_m28_native_fetch/M28-007-C-define-credential-header-stripping.md`

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
5. Implement exactly this deliverable: Bound decompression ratio and output.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Redirect loops fail boundedly.
- Sensitive headers never leak cross-origin.
- Zip-bomb style expansion is limited.
- Observed URL/status follows documented semantics.

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

- Security fixtures.
- Compression limits tests.
- Redirect conformance.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-007-d: bound decompression ratio and output
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-007-D) — PASS

- Date: 2026-08-29
- Branch/PR: m28-007-d (squash-merged; see git log for final hash)
- Closes: #345

### Changed files
- `crates/q-capabilities/src/fetch_policy.rs`: decompression bomb guard (ADR-0033 §8) —
  - `MAX_DECOMPRESSION_RATIO` = 1000:1 ceiling; `DECOMPRESSION_RATIO_THRESHOLD` = 1 KiB of compressed input before the ratio applies (small legitimate payloads with high local expansion must not false-positive).
  - `DecompressionGuard::new(output_limit)` / `from_policy(&FetchPolicy) -> Option<Self>` (Off / Gzip{false} -> None: no decompression, no guard; Gzip{true} -> guard bounded by the response body cap).
  - Push-based accounting: `compressed_input(n)` as bytes leave the wire, `decompressed_output(n)` as the decoder emits — each step checked, so a bomb fails typed at the step that crosses the line, never after buffering. A failed step accepts no bytes (`produced` unchanged).
  - Typed failures: `FetchPolicyError::DecompressedTooLarge { produced, max }` (output cap = ADR-0033 §9 response limit) and `FetchPolicyError::DecompressionBomb { compressed, produced, max_ratio }` (ratio ceiling past threshold).
- `crates/q-capabilities/src/lib.rs`: re-exports `DecompressionGuard`, `MAX_DECOMPRESSION_RATIO`, `DECOMPRESSION_RATIO_THRESHOLD`.

### Tests added (fetch_policy.rs, +5 → 167 lib tests)
- `decompression_output_is_capped_typed` (boundary at the cap, `DecompressedTooLarge{17,16}`, failed step accepts nothing)
- `zip_bomb_ratio_is_bounded_typed` (2048 compressed bytes -> 1000x fires `DecompressionBomb{compressed:2048, max_ratio:1000}` exactly at the ceiling)
- `small_payloads_are_not_ratio_limited_below_threshold` (4 -> 4000 bytes allowed below the 1 KiB input threshold)
- `guard_from_policy_matches_compression_posture` (Off/Gzip{false} -> None; Gzip{true} -> cap == response body limit)
- `bomb_fixture_output_cap_fires_before_ratio_when_tighter` (1 KiB fixture claiming 20 MiB: the tighter 16 MiB output cap fires first)

### Command results
- `cargo test -p q-capabilities` → **167 unit (was 162) + 4 backpressure + 8 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- Release binary hash unchanged (`ef142331…` matches manifest) — policy additions dead-code-eliminated until the executor wires in.

### Guardrail mapping
- **Zip-bomb style expansion is limited** — dual bound: output can never exceed the response body cap, and the expansion ratio is capped at 1000:1 once ≥ 1 KiB has been consumed; both typed, both enforced per-step.

### Disclosures
- Two heredoc edits failed their anchors (fmt had reflowed earlier code) and aborted without writing — re-applied with verified anchors / the file-edit tool. No partial state reached the tree at any point.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
