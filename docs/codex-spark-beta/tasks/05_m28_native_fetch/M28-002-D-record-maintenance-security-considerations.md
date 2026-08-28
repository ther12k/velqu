---
task_id: M28-002-D
parent_task: M28-002
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-002-D — Record maintenance/security considerations

## Atomic goal

Record maintenance/security considerations.

## Parent intent

Choose a maintainable implementation based on cold start, size, correctness, streaming, and pooling.

## Dependencies

- `M28-002-C` — `tasks/05_m28_native_fetch/M28-002-C-test-dns-tls-pool-behavior.md`

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
- `Cargo.toml`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Record maintenance/security considerations.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Decision is evidence-backed.
- No framework benchmark alone determines choice.
- Selected stack supports cancellation/backpressure.
- Fallback strategy documented.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Spike report.
- Raw measurements.
- Decision record.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-002-d: record maintenance security considerations
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-002-D) — PASS

- Date: 2026-08-28
- Branch/PR: m28-002-d (squash-merged; see git log for final hash)
- Closes: #315

### Changed files
- `docs/reports/m28-002-d-maintenance-security.md` (new): standing maintenance and security posture for the selected outbound stack — ownership/review-boundary table (policy = ADR-0033 object, trust = ADR-0034, pool bounds = M28-003, address validation = M28-008-A); dependency maintenance policy (pinned minimal ring-only feature sets, dedicated upgrade packets that re-run verify + C-probes + cost baseline, RUSTSEC review checklist, webpki-roots refresh policy with fail-closed staleness); five security considerations (no ambient configuration, no bypass surface, network-not-process trust per ADR-0035, dormant-until-wired, disclosed legacy-connector retry limitation); per-packet maintenance checklist.

### Command results
- `cargo test -p q-engine-quickjs` 16+97 · `-p q-http` 4+6+1 · `-p q-capabilities` 132+8 · `-p velqu-runtime` 1+31 — all pass
- `bun test` → 215 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- Documentation-only packet: no runtime or dependency changes.

### Guardrail mapping
- **Decision is evidence-backed** — considerations cite measured evidence (M28-002-A/B/C) and name the owning packet for each concern.
- **No framework benchmark alone determines choice** — posture adds the ongoing-criteria (CVE review, probe re-runs, cost baseline) that keep the choice honest over time.
- **Selected stack supports cancellation/backpressure** — unchanged; C-probe 6 remains the proof.
- **Fallback strategy documented** — unchanged (M28-002-A); maintenance checklist makes a fallback re-evaluation a recorded decision if the stack ever fails its criteria.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
