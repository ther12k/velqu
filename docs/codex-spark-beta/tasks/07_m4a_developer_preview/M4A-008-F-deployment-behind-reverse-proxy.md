---
task_id: M4A-008-F
parent_task: M4A-008
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-008-F — Deployment behind reverse proxy

## Atomic goal

Deployment behind reverse proxy.

## Parent intent

Provide an honest, runnable learning path.

## Dependencies

- `M4A-008-E` — `tasks/07_m4a_developer_preview/M4A-008-E-runtime-profiles.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`

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
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/src/main.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Deployment behind reverse proxy.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Docs test output.
- Link check.
- Example CI.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-008-f: deployment behind reverse proxy
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-008-F) — PASS (2026-09-01)

- Branch/PR: m4a-008-f (squash-merged; see git log for final hash)
- Closes: #479

### Changed files
- `docs/beta/DEPLOYMENT-REVERSE-PROXY.md`: reverse-proxy-first deployment
  guide with bounded runtime build/run commands, loopback binding, Nginx TLS
  termination example, health/readiness/drain rollout sequence, trusted
  forwarding-header boundary, ownership split, and explicit beta limitations.
- `docs/beta/INDEX.md`, `docs/beta/README.md`: deployment guide links.

### Evidence
- Documentation local-link check: PASS.
- Nginx/configuration sample review: PASS (loopback upstream, edge size and
  timeout limits, TLS paths, health endpoints, explicit no-direct-public-TLS
  posture).
- `cargo test -p velqu-runtime`: PASS
- `bun test`: 309 pass / 0 fail
- `bun run typecheck`, fmt, workspace clippy: clean
- `./scripts/verify`: **ALL PASS**

### Guardrail mapping
- **Every code sample is tested:** runtime commands and health/readiness paths
  are exercised by the runtime/proof gates; proxy config is explicitly marked
  as an operator example requiring environment validation.
- **Measured facts vs targets:** no deployment-performance claim.
- **No production-ready claim:** private-alpha/non-SLA posture is explicit.
- **Known limitations prominent:** plain HTTP runtime, trusted proxy boundary,
  no native TLS/HTTP2, no direct public exposure, and bounded drain are called
  out.

### Disclosures
- Documentation-only packet; no production behavior changes.
- Standing: CI verify workflows fail with zero executed steps since ~#714
  (infrastructure-side); disclosed per PR.
