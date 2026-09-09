---
task_id: M4A-008-D
parent_task: M4A-008
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-008-D — Fetch/capabilities

## Atomic goal

Fetch/capabilities.

## Parent intent

Provide an honest, runnable learning path.

## Dependencies

- `M4A-008-C` — `tasks/07_m4a_developer_preview/M4A-008-C-treaty.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Fetch/capabilities.
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
m4a-008-d: fetch capabilities
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-008-D) — PASS (2026-09-01)

- Branch/PR: m4a-008-d (squash-merged; see git log for final hash)
- Closes: #477

### Changed files
- `docs/beta/FETCH-CAPABILITIES.md`: source-backed private-alpha fetch guide
  covering explicit `--with-fetch` opt-in, generated upstream route, typed
  200/502 handling, capability-linker/egress boundary, timeout/body-limit/
  SSRF limitations, and verification commands.
- `packages/cli/src/scaffold.test.ts`: verifies optional fetch scaffold route,
  test, README capability notice, and `velqu.capabilities` metadata.
- `docs/beta/INDEX.md`, `docs/beta/README.md`: fetch guide links.

### Evidence
- Documentation local-link check: PASS.
- `packages/cli/src/scaffold.test.ts`: **6 pass / 0 fail** (including
  `--with-fetch adds the upstream route, test, and capability metadata`).
- `bun test`: **309 pass / 0 fail**
- `bun run typecheck`: clean
- `cargo test -p q-http`: PASS
- `cargo test -p q-capabilities`: PASS
- `cargo test -p velqu-runtime`: PASS
- `./scripts/verify`: **ALL PASS**

### Guardrail mapping
- **Every code sample is tested:** the fetch example mirrors the generated
  scaffold and its route metadata is asserted by the scaffold test.
- **Measured facts vs targets:** no network-performance claim.
- **No production-ready claim:** private alpha and fixture/egress caveats are
  explicit.
- **Known limitations prominent:** security policy, SSRF, network ownership,
  timeouts, response limits, and unavailable upstreams are called out.

### Disclosures
- Documentation/test packet; no production runtime behavior changes.
- Standing: CI verify workflows fail with zero executed steps since ~#714
  (infrastructure-side); disclosed per PR.
