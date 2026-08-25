---
task_id: M26-009-B
parent_task: M26-009
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-009-B — Standalone mode: embedded qpack executable

## Atomic goal

Standalone mode: embedded qpack executable.

## Parent intent

Support both small app updates and one-file deployment.

## Dependencies

- `M26-009-A` — `tasks/03_m26_qpack_v2/M26-009-A-shared-mode-velqu-runtime-plus-app-qpack.md`

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
5. Implement exactly this deliverable: Standalone mode: embedded qpack executable.
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
m26-009-b: standalone mode embedded qpack executable
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-009-B (PASS)

Deliverable: standalone deployment mode — one executable with the
verified pack embedded at compile time.

Provenance: built on the idle parallel session's uncommitted WIP (the
lib.rs extraction + feature/bin Cargo wiring, last touched ~4 h before
takeover, based on the same main.rs body as master). This packet
completed it: `verify_from_slice`, the standalone bin, the thin
shared-mode main, the smoke/report evidence, and all gates.

Changed files:

- `crates/q-runtime/src/lib.rs` (from WIP) — shared startup pipeline:
  `PackSource::{Path, Embedded}` + `run(source, cfg)`; `mode` field on
  the ready line; `LogMode::from_str` renamed `parse_mode` (clippy
  should_implement_trait once public).
- `crates/q-runtime/src/main.rs` — thin shared-mode CLI over the lib
  (identical flags; behavior unchanged — the 28 conformance tests pass
  unmodified).
- `crates/q-runtime/src/bin/velqu-standalone.rs` — standalone CLI
  (same flags minus --pack); `include_bytes!(env!("VELQU_STANDALONE_PACK"))`;
  built only under `--features standalone` (required-features gate so
  ordinary builds never need the env var).
- `crates/q-runtime/Cargo.toml` — `[lib] name = "velqu_runtime"`,
  feature + bin from WIP.
- `crates/q-pack/src/lib.rs` — `QPack::verify_from_slice(bytes,
  policy)`: same policy semantics and single-decode bytecode cache as
  the file path; the embedded artifact is still fully verified at
  startup — embedding grants no trust.
- `scripts/artifact-smoke.sh` — section 5: builds (if missing) and
  serves the standalone binary; asserts IDENTICAL /health/live and
  /hello/:name bodies vs shared mode and `mode":"standalone"` on the
  ready line.
- `docs/reports/m26-009-b-standalone-mode.md` — measured evidence.
- `benchmarks/manifest.json` — matched refresh (this packet changes
  runtime source → release-binary hash drift; rebuilt with verify's
  remap flags and refreshed via the sanctioned script).

Measured evidence (n=10 fresh processes per mode, release builds,
same host; full raw samples in the report):

- Cold start startupMs p50: shared 3.500 / standalone 2.976
  (p95 4.592 / 3.780); distributions overlap — same-host sanity delta,
  not a portability claim.
- RSS-after-ready VmRSS p50: shared 7,236 kB / standalone 7,124 kB.
- Artifact sizes: 5,201,208 B shared vs 5,224,216 B standalone
  (+23,008 B ≈ embedded 24,414 B pack).
- Route parity: both modes serve the same pack with identical bodies.

Tests:

- `verify_from_slice_matches_file_verification` (q-pack, 92 total) —
  slice vs file parity for accept/reject and the bytecode cache under
  both policies.
- `scripts/artifact-smoke.sh` → SMOKE-OK including the standalone
  section.
- velqu-runtime 28 passed unchanged (shared-mode conformance through
  the extracted lib).

Commands: q-pack 92+2; velqu-runtime 28; q-engine-quickjs 1+97; bun
125 pass / 0 fail; typecheck, fmt, clippy -D warnings clean;
`./scripts/verify` — ALL PASS (exit 0), including
validate-benchmark-evidence after the matched refresh.

Guardrails: identical conformance (shared suite + cross-mode smoke
parity); standalone has no compiler toolchain (links only the runtime
pipeline; G-004 preserved); shared-mode mismatch rejection unchanged
(smoke step 3); startup/RSS differences measured (report above).
