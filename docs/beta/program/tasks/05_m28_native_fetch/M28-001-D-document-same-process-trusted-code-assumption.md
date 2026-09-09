---
task_id: M28-001-D
parent_task: M28-001
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-001-D — Document same-process trusted-code assumption

## Atomic goal

Document same-process trusted-code assumption.

## Parent intent

Freeze the public subset, trust boundaries, defaults, and non-goals.

## Dependencies

- `M28-001-C` — `tasks/05_m28_native_fetch/M28-001-C-define-unsupported-web-features.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `Cargo.toml`
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-http/tests/fuzz_parsers.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Document same-process trusted-code assumption.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Security defaults fail closed.
- Private/link-local/metadata behavior is explicit.
- Redirect revalidation is required.
- Direct TLS policy is documented.

## Targeted commands

```bash
cargo test -p q-pack
```
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
cargo test -p velqu-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- ADR.
- Threat model.
- Security test matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-001-d: document same process trusted code assumption
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-001-D) — PASS

- Date: 2026-08-28
- Branch/PR: m28-001-d (squash-merged; see git log for final hash)
- Closes: #309

### Changed files
- `docs/okf/decisions/0035-same-process-trusted-code-assumption.md` (new): **ADR-0035** — elevates AGENTS.md constraint 14 into a first-class decision: the QuickJS worker executes trusted, pack-compiled, owner-built application code only (trust derives from the build pipeline: ADR-0004 static discovery, ADR-0014 version-pinned bytecode, ADR-0026 integrity hooks). The process interior is inside the trust boundary — the security model protects the host network (ADR-0033) and the client (redaction), not the process from the application. Fetch is a network control, never an isolation boundary. Unsupported deployment modes named explicitly (user-uploaded packs, third-party-registry bundles, multi-tenant execution). Naming rule: no doc may call the engine/capability system/fetch policy a "sandbox" for untrusted code. Six-row threat model separating mitigated vs not-mitigated vs out-of-scope.
- `crates/q-capabilities/src/fetch_policy.rs`: added `TRUSTED_CODE_ASSUMPTION` — the canonical pinned statement — plus test `trusted_code_assumption_is_pinned` enforcing its three load-bearing properties (names trusted code, denies the sandbox framing, names the network as the adversary) and the process-interior scope clause.
- `crates/q-capabilities/src/lib.rs`: crate-root rustdoc gains a `# Trust model` section cross-referencing ADR-0035/AGENTS.md constraint 14; `TRUSTED_CODE_ASSUMPTION` re-exported.
- `docs/okf/decisions/index.md`: ADR-0035 entry.

### Command results
- `cargo test -p q-capabilities` → 132 unit + 8 integration passed (was 131+8)
- `cargo test -p q-pack` 96+2 · `-p q-engine-quickjs` 16+97 · `-p q-http` 4+6+1 · `-p q-schema-runtime` 58+5+4 · `-p velqu-runtime` 31 — all pass
- `bun test` → 215 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/validate-okf` → exit 0 (ADR-0035 accepted)
- `./scripts/verify` → **ALL PASS (exit 0)**

### Guardrail mapping
- **Security defaults fail closed** — unchanged; this packet adds no behavior, only the frozen trust statement that keeps future docs honest.
- **Private/link-local/metadata behavior is explicit** — ADR-0035 §2 anchors ADR-0033 §2 as network protection, explicitly not process isolation.
- **Redirect revalidation is required** — unchanged (ADR-0033 §4).
- **Direct TLS policy is documented** — ADR-0035 §3: fetch/TLS policy is a network control, never isolation evidence.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
