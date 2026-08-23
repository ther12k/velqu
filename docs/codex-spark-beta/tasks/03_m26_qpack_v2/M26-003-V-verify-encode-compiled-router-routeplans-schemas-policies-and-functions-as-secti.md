---
task_id: M26-003-V
parent_task: M26-003
milestone: M26
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-003-V — Verify Encode compiled router, RoutePlans, schemas, policies, and functions as sections

## Atomic goal

Prove every acceptance criterion for parent task M26-003 without broadening scope.

## Parent intent

Serialize the already verified runtime graph without changing semantics.

## Dependencies

- `M26-003-A` — `tasks/03_m26_qpack_v2/M26-003-A-define-dense-section-schemas.md`
- `M26-003-B` — `tasks/03_m26_qpack_v2/M26-003-B-store-router-nodes-edges-terminals-routeplans-schema-programs-policy-plans-funct.md`
- `M26-003-C` — `tasks/03_m26_qpack_v2/M26-003-C-use-offsets-and-bounds-checks.md`
- `M26-003-D` — `tasks/03_m26_qpack_v2/M26-003-D-bind-sections-to-execution-integrity.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-router/src/lib.rs`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No semantic reconstruction at startup.
- Bounds and index validation reject malformed packs.
- Binary and transitional representations are property-equivalent.
- Debug names are optional and non-hot.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
```
```bash
cargo test -p q-engine-quickjs
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
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```

## Required evidence for this microtask

- Round-trip/property tests.
- Mutation fuzzing.
- Section-size report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-003-v: verify encode compiled router routeplans schemas policies an
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification record — M26-003-V (PASS)

Parent: M26-003 "Encode compiled router, RoutePlans, schemas, policies, and
functions as sections" (serialize the already verified runtime graph
without changing semantics). All four implementation dependencies
(M26-003-A/B/C/D) are merged on master before this branch.

### Acceptance criterion mapping

1. **No semantic reconstruction at startup.**
   Source: `qpack2::graph::{router_section, plans_section,
   schemas_section, policy_section}` (crates/q-pack/src/lib.rs) — encoders
   consume the already-verified structures (`SerializedRouter`,
   `RoutePlanDecl`, `SchemaDecl`, policy rows+manifest) that
   `SerializedRouter::verify_against` / `QPack::verify` validated; decoders
   return those structures as data (no path re-walk, no schema re-derivation,
   no compilation).
   Tests: `graph_sections_round_trip` (decode∘encode == identity),
   `binary_and_transitional_representations_agree` (new, below),
   `current_mode_is_pinned_until_native_v2_lands` (v2 sections are not yet
   the production load path — no startup behavior changed in M26-003).

2. **Bounds and index validation reject malformed packs.**
   Source: `qpack2::reader::{parse_header, parse_directory_of_size,
   validate}` — magic/version/header-size, duplicate/overlap/range/past-end
   rules, unknown-flag reject, unknown section id reject, per-section
   sha256, required-id check; graph decoders enforce MAX_NODES and
   boundary-walks.
   Tests: `every_directory_rule_violation_rejects` (12 rules),
   `header_directory_mutation_never_panics` (4 000 rounds),
   `graph_sections_mutation_never_panics`,
   `dense_sections_never_panic_under_mutation` (2 000 rounds),
   integration fuzz `crates/q-pack/tests/fuzz_pack.rs`
   (`random_bytes_never_panic_the_pack_parser`,
   `mutated_valid_pack_never_panic_and_tamper_is_detected` — >200/256
   single-byte flips rejected).

3. **Binary and transitional representations are property-equivalent.**
   NEW test `binary_and_transitional_representations_agree`
   (crates/q-pack/src/lib.rs): the same graph fixture round-trips through
   the transitional JSON/serde form AND through the v2 binary sections, and
   the two decode paths yield identical structures (router, plans,
   schemas). Existing corroboration: `dense_sections_round_trip`,
   `graph_sections_round_trip`, `header_and_directory_round_trip`,
   `bound_file_round_trips_and_binds_every_section`.

4. **Debug names are optional and non-hot.**
   Source: section catalog §6 (`section::REQUIRED` pins the seven required
   ids; BUNDLE_BYTECODE is not required; `FLAG_OPTIONAL = 0x0001` defined
   and unknown flag bits reject in the directory walk).
   Tests: `header_and_directory_round_trip` (a file carrying ONLY the
   required catalog validates — optional section absent still verifies),
   `every_directory_rule_violation_rejects` (missing required section
   rejects), `layout_constants_match_spec` (constants pinned to spec),
   v1-level `verification_is_independent_of_debug_sidecars`.

### Section-size report evidence

`dense_section_size_report` and `graph_section_size_report` print the
structural size comparison (dense record tables vs JSON) with honest
thresholds (M26-003-A/B records); this packet adds no size claims.

### Changed files

- `crates/q-pack/src/lib.rs` — added test
  `binary_and_transitional_representations_agree` (criterion 3 direct
  proof). No production-code change: verification-only packet.

### Commands and results (this branch)

- `cargo test -p q-pack` — 67 passed + 2, 0 failed.
- `cargo test -p q-router` — 15 passed.
- `cargo test -p q-engine-quickjs` — 1 + 97 passed.
- `cargo test -p velqu-runtime` — 26 passed (after building
  `q-bytecode-tool` and release `velqu-runtime` in this fresh worktree; the
  two bytecode tests and the shutdown test require those binaries).
- `bun test` — 82 pass / 0 fail / 487 expect().
- `bun run typecheck` — clean.
- `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — all gates green except the pre-existing, documented
  `validate-benchmark-evidence` scoped failure (qRuntimeRelease + proofPack
  manifest hash mismatches inherited from M26-002-A pack-byte changes;
  flagged matched-evidence follow-up, not altered here).

No defects found requiring fixes beyond the added equivalence test. No
unrelated findings needing follow-up tasks.
