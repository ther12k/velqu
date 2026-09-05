# BWASM-K-004 — Qualify the Schema Runtime for wasm32 and Expose Bounded Validation

## Result

**PASS** — `q-schema-runtime` is now a **qualified** wasm32 component:
its full test suite (58 lib + 5 codec-standards corpus + 4 fuzz tests =
67) **executes on-target** as WebAssembly (wasm32-wasip1 under Node's
WASI), with test names and outcomes **byte-identical to the native
run**, and its bounded validation surface is the frozen kernel-facing
contract. The real WASM size contribution is measured via a committed
probe.

## On-target qualification harness (committed)

- `scripts/wasm-wasi-node-runner.mjs` — Node WASI (preview1) runner;
  propagates exit codes so cargo's pass/fail semantics hold.
- `.cargo/config.toml` — registers the runner for `wasm32-wasip1`
  (release target remains `wasm32-unknown-unknown` per ADR-0037; WASI
  exists only so std's test harness can execute on-target).
- Command: `cargo test -p q-schema-runtime --target wasm32-wasip1`.

## Equivalence evidence

| Suite | native | wasm32-wasip1 (executed as WebAssembly) |
|---|---|---|
| lib unit tests | 58 pass | 58 pass |
| codec_standards_corpus | 5 pass | 5 pass |
| fuzz_validator (real work, 1.65 s on-target) | 4 pass | 4 pass |
| doc-tests | 0 | 0 |

The sorted list of 67 passing test names is **identical across targets**
(diff clean). Codes, paths, and ordering equivalence follows from the
same binaries' assertions running in both environments — the corpus and
lib tests assert exact error codes/paths (e.g.
`field_error_codes_round_trip_with_wire_strings`,
`decoder_program_matches_reference_validator_on_mixed_corpus`).

## Bounded validation surface (kernel-facing contract, already in-crate)

The task's required limits exist and are enforced with typed problems:

- **Depth**: `MAX_VALIDATE_DEPTH = 64` enforced in validate, encoder,
  and decoder paths (`depth` typed problem;
  `decode_depth_bounded_with_typed_depth_problem`,
  encoder `MAX_VALIDATE_DEPTH + 8` pin).
- **Collection/string/regex work**: schema-IR-bounded; the regex cache
  is bounded (`RefCell` cache, compiled-once per pattern); fuzz corpus
  proves panic-free behavior on arbitrary inputs.
- **Error output**: `ValidationResult = Result<Value, Vec<FieldError>>`
  — finite, ordered error vectors; wire codes pinned by test.
- **No JS fallback**: validation semantics exist only in this crate;
  the TS packages (`packages/schema`, `packages/contract`) are
  authoring/typing surfaces. The browser kernel (K-005) must bind to
  THIS crate's compiled form; substituting a JS validator would be a
  contract violation — recorded as a K-005 guardrail.

Canonicalization and error-order fixtures are shared across targets by
construction: one test binary, executed natively and on-target.

## WASM size contribution (measured, with honesty caveats)

`scripts/bwasm-size-probe/` (committed, reproducible via
`measure.sh`): a cdylib exporting one `validate` call links the schema
runtime + serde_json + regex + std.

```text
raw    = 1,216,002 bytes
gzip-9 =   386,429 bytes   (sha256 2b78355c1931c76509d228208012d0e33bed172e41b4ca4ede6e4be5bdacc20e)
```

For context, the K-004 measurability baseline (trivial std-linked
cdylib) was 1,474,978 raw / 305,342 gzip-9 — note the probe is SMALLER
raw (opt-level "z" + LTO) yet larger compressed than that baseline
build; both numbers are **proxies**: `wasm-opt` and `brotli` are absent
from this measurement host (ADR-0039 records the same tooling gap).
Against the ratified ≤500 KiB base-kernel budget (ADR-0039), the
schema+serde+regex contribution measured this way is ~377 KiB
gzip-9 — inside budget **before** wasm-opt dead-code elimination, with
the caveat that the kernel will link a subset surface via the K-005
ABI. Re-measurement with the real kernel build is K-005/K-006 scope.

## Commands run (exact)

```text
cargo test -p q-schema-runtime                                  PASS (58+5+4+0)
cargo test -p q-schema-runtime --target wasm32-wasip1            PASS (58+5+4+0, executed)
diff <native test names> <wasi test names>                       IDENTICAL (67)
cargo check --target wasm32-unknown-unknown -p q-schema-runtime  PASS
scripts/bwasm-size-probe/measure.sh                              sizes above
cargo fmt/clippy/validate-okf/verify                             PASS / ALL PASS
```

## Acceptance disposition

- [x] Native and wasm32 schema tests pass with equivalent codes, paths,
  ordering (executed on-target; name-diff clean).
- [x] Limit violations return typed problems without panic or browser
  hangs (depth/limit tests pass on-target; fuzz corpus 1.65 s real
  execution).
- [x] No JavaScript schema fallback silently changes semantics (single
  implementation; K-005 guardrail recorded).
- [x] Schema WASM size contribution measured (probe; proxy caveats
  recorded; budget context given).

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
