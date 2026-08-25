---
task_id: M26-009-C
parent_task: M26-009
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-009-C — Ensure exact runtime fingerprint

## Atomic goal

Ensure exact runtime fingerprint.

## Parent intent

Support both small app updates and one-file deployment.

## Dependencies

- `M26-009-B` — `tasks/03_m26_qpack_v2/M26-009-B-standalone-mode-embedded-qpack-executable.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-runtime/src/serve.rs`
- `docs/beta/`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Ensure exact runtime fingerprint.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Both modes pass identical conformance.
- Standalone contains no compiler toolchain.
- Shared mode rejects mismatched runtime.
- Startup/RSS differences are measured.

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

- Artifact smoke tests.
- Size/cold-start report.
- Install guide.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-009-c: ensure exact runtime fingerprint
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-009-C (PASS)

Deliverable: the exact runtime fingerprint is INSPECTABLE and
pre-checkable in both deployment modes, not only enforced at boot.

Changed files:

- `crates/q-pack/src/lib.rs` — `RuntimeFingerprint::current()`: the
  exact tuple verify() enforces (engine name/version, rquickjs,
  binding, build hash, runtime ABI, arch/os/pointer-width/endianness),
  serde-serializable for `--fingerprint` JSON output.
- `crates/q-runtime/src/lib.rs` — `print_fingerprint(&PackSource)`:
  prints the runtime tuple; with a pack available (either mode) runs
  FULL verification WITHOUT serving and prints the verdict. Exit 0 =
  compatible; 2 = rejected with the actionable diagnostic.
- `crates/q-runtime/src/main.rs` + `src/bin/velqu-standalone.rs` —
  `--fingerprint` flag on both binaries (shared mode takes --pack;
  standalone always checks the embedded pack).
- `crates/q-runtime/tests/runtime_conformance.rs` — integration test.
- `benchmarks/manifest.json` — matched refresh (runtime source changed;
  rebuilt with verify's remap flags via the sanctioned script).

Tests:

- `runtime_fingerprint_tuple_is_the_enforced_identity` (q-pack, 93
  total) — the inspectable tuple equals the enforced constants; a pack
  with the exact tuple verifies; any drifted dimension rejects.
- `fingerprint_flag_reports_exact_tuple_and_verifies_without_serving`
  (velqu-runtime, 29 total) — exit 0 + full JSON tuple + verdict on a
  compatible pack; exit 2 + "engine mismatch" diagnostic on a drifted
  pack; no port ever bound.

Commands: q-pack 93+2; velqu-runtime 29; q-engine-quickjs 1+97; bun
125 pass / 0 fail; typecheck, fmt, clippy -D warnings clean;
`./scripts/verify` — ALL PASS (exit 0) including
validate-benchmark-evidence after the matched refresh.

Guardrails: both modes identical (same lib function; shared tested by
conformance, standalone by the same code path over the embedded
pack); standalone has no compiler toolchain (verification only);
shared-mode mismatch rejection unchanged and now also pre-checkable;
startup/RSS deltas already measured in M26-009-B (no new claims).
