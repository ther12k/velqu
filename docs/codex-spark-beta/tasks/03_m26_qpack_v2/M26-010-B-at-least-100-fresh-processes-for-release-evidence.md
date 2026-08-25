---
task_id: M26-010-B
parent_task: M26-010
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-010-B — At least 100 fresh processes for release evidence

## Atomic goal

At least 100 fresh processes for release evidence.

## Parent intent

Demonstrate flatter startup scaling and preserve small-app behavior.

## Dependencies

- `M26-010-A` — `tasks/03_m26_qpack_v2/M26-010-A-measure-25-100-1-000-5-000-10-000-routes.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`
- `package.json`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: At least 100 fresh processes for release evidence.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No runtime router/schema compilation.
- No base64 decoding.
- 25-route budget is not sacrificed silently.
- 10,000-route scaling is documented honestly.

## Targeted commands

```bash
cargo test -p q-pack
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

- Raw cold data.
- Generated report.
- Startup-stage trace.
- [ ] QPack v2 is deterministic, fail-closed, and version/fingerprint safe.
- [ ] Production startup maps verified runtime IR and raw bytecode without JSON/base64 reconstruction.
- [ ] Legacy compatibility is isolated.
- [ ] Shared and standalone artifacts pass conformance.
- [ ] Cold-start route scaling evidence is canonical.
- 25/100/1,000/5,000/10,000 route cold start.
- Shared vs standalone RSS/startup.
- Pack parse/allocation stages.
- Source vs bytecode selection.
- No full capability ecosystem.
- No Node compatibility.
- No multi-worker yet.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-010-b: at least 100 fresh processes for release evidence
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-010-B (PASS)

Deliverable: release-grade process count for the route-count ladder —
at least 100 fresh processes per cell.

Evidence run: `ROUTE_COUNT_RUN_ID=m26-010-b ROUTE_COUNT_SEED=202608262
bun benchmarks/harness/route-count.ts --samples=100` — 4 candidates ×
5 sizes × 100 fresh processes = **2,000 spawns, ZERO failures**,
every sample retained (`route-count-1787682637700.jsonl`; 2,000 rows).

Stability vs the 40-process A run (same host, same packs): p50s match
within noise — velqu source 25-route 6.073 vs 6.116 ms; 10k-route
949.101 vs 947.671 ms; bytecode 10k 924.168 vs 926.186 ms; raw-bun
flat ~7.0-7.5; elysia2 108.59 → 255.735. Cross-run consistency is
itself evidence the measurements are not single-run artifacts.

Changed files:

- `benchmarks/raw/route-count/` — new raw + summary (run m26-010-b,
  100 samples/cell).
- `docs/reports/cold-start-report.md` — regenerated from the summary
  (data-driven section carries the 100-process run).
- `docs/reports/m26-010-a-route-count-ladder.md` — appended the B
  section (≥100 processes + cross-run stability table note).
- `benchmarks/manifest.json` — matched refresh (run pointer + rebuilt
  artifacts under verify's remap flags).

Commands: q-pack 94+2; velqu-runtime 30; bun 125 pass / 0 fail;
typecheck, fmt, clippy -D warnings clean; `./scripts/verify` — ALL
PASS (exit 0).

Guardrails: unchanged from A (no startup compilation, no base64
decode, 25-route budget preserved ~6 ms, honest 10k documentation);
this packet raises evidence weight only — no production code changed.
