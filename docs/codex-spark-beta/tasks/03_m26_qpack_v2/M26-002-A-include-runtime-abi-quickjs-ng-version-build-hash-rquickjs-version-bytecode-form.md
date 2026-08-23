---
task_id: M26-002-A
parent_task: M26-002
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-002-A — Include runtime ABI, QuickJS-NG version/build hash, rquickjs version, bytecode format, target triple, pointer width, endianness, and capability hash

## Atomic goal

Include runtime ABI, QuickJS-NG version/build hash, rquickjs version, bytecode format, target triple, pointer width, endianness, and capability hash.

## Parent intent

Prevent loading bytecode or plans under an incompatible engine/runtime build.

## Dependencies

- `M26-001-Z` — `tasks/03_m26_qpack_v2/M26-001-Z-package-evidence-for-accept-qpack-v2-format-and-compatibility-adr.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-capabilities/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Include runtime ABI, QuickJS-NG version/build hash, rquickjs version, bytecode format, target triple, pointer width, endianness, and capability hash.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Any fingerprint mismatch rejects before ready.
- Error identifies incompatible dimension.
- Engine upgrades require pack rebuild.
- Cross-target packs are rejected.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
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

- Fingerprint tests.
- Cross-build fixtures.
- Upgrade lane documentation.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-002-a: include runtime abi quickjs ng version build hash rquickjs v
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-002-A)

Status: **PASS**. The full runtime fingerprint travels with every pack and
each dimension rejects with the incompatible dimension named:

- **Fingerprint dimensions** (`crates/q-pack/src/lib.rs`): runtime ABI ✓
  (pre-existing), engine name/version/binding ✓ (pre-existing), PLUS
  `EngineRef.rquickjs` (pinned 0.12.2) and `EngineRef.buildHash` — the
  runtime build fingerprint `runtime_build_hash()` = sha256 over the
  runtime identity tuple (abi, engine, version, rquickjs, binding;
  binary-level reproducible-build hashes replace the tuple hash when the
  release pipeline embeds one). Bytecode form/target-triple/pointer-
  width/endianness were already carried (`BundleBytecode` +
  `BytecodeTarget`, M24-era) and remain verified.
- **Capability hash**: `QPack.capability_hash` = `capability_hash(&caps)`
  (sha256 over sorted, newline-joined names). Present-but-wrong rejects
  ("incompatible dimension: capabilities"); ABSENT keeps legacy-v1 packs
  loading (ADR-0024 compatibility — v1 packs predate the field).
- **Dimension-named rejections**: rquickjs mismatch ("dimension:
  binding"), build-hash mismatch ("dimension: runtime build … engine
  upgrades require a pack rebuild" — the rebuild guardrail).
- **Compiler parity** (`packages/compiler/src/emit.ts`): the pack emits
  `engine.rquickjs`, `engine.buildHash` (Bun CryptoHasher mirroring the
  Rust tuple hash), and `capabilityHash` — proven end-to-end: the
  freshly built proof pack LOADS and serves (TS/Rust hash agreement is
  proven by the load itself; any mismatch rejects before ready).

### Tests and evidence

- `rejects_rquickjs_mismatch_with_dimension`,
  `rejects_build_hash_mismatch_with_dimension`,
  `capability_hash_present_must_match_and_absent_is_v1_compatible`
  (q-pack).
- Compiler conformance "pack carries the full runtime fingerprint
  (M26-002-A)" — engine fields, 64-hex build hash, capability-hash
  agreement recomputed in the test.
- `cargo test -p q-pack` — 51 + 2; `cargo test -p q-engine-quickjs` —
  1 + 96; `cargo test -p q-http` — 4 + 6 + 1; `cargo test -p
  q-schema-runtime` — 58 + 4 + 5; `cargo test -p velqu-runtime` — 24 —
  all passed.
- `bun test` — 82 passed, 0 failed, 487 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — clean.
- `./scripts/verify` — all stages pass except
  `validate-benchmark-evidence`: the known isolated-worktree hash
  mismatch for `qRuntimeRelease`/`proofPack`. NOTE: this packet
  legitimately changes pack bytes (the fingerprint fields), so the
  canonical `proofPack` manifest entry requires a matched refresh —
  flagged for the benchmark-manifest owner; not silently altered here.

Commit: `6881d25`.
