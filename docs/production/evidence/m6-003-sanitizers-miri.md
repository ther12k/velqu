# M6-003 — Sanitizers, Miri, and Unsafe Audits (evidence closure)

Issue #1318 (GA delta M6-003). Raw evidence for S1/S2/Miri is committed
under `benchmarks/raw/ga-m6-fuzz/` from the same campaign run as M6-002
(candidate `b8fee34909c0d6a4b3b9f76aebe9665cf3192c99`, verdicts recorded
in `campaign-ledger.json` `sanitizerCampaign` and `miri-ledger.json`).

## S1 — ASan (with LSan) workspace pass

- Command: `RUSTFLAGS='-Zsanitizer=address' cargo +nightly test
  --workspace --target x86_64-unknown-linux-gnu -Zbuild-std`.
- Result: **zero-findings** (exit 0; log
  `benchmarks/raw/ga-m6-fuzz/asan-workspace.log`).
- LeakSanitizer runs by default inside the Linux ASan build, so the S1
  pass is also the LSan record (applicability matrix:
  `fuzz/sanitizer-applicability.json`).

## S2 — UBSan over the QuickJS C FFI

- The C sources (quickjs-ng via rquickjs-sys cc build) compiled with
  `-fsanitize=undefined -fno-sanitize-recover=all`; test binary linked
  with the UBSan runtime; `cargo +nightly test -p q-engine-quickjs`.
- Result: **zero-findings** (log
  `benchmarks/raw/ga-m6-fuzz/ubsan-quickjs-ffi.log`).
- This stage is the compensation for the Miri-excluded foreign-C engine
  boundary. Rust-side UBSan is not separately run (rustc
  `-Zsanitizer` accepts one sanitizer per build; the Rust side runs
  under ASan S1) — recorded in the applicability matrix.

## Miri — FFI-free crates

- Driver: `scripts/miri-campaign.sh` (fail-closed ledger; scope from
  `fuzz/miri-scope.json`).
- Toolchain: rustc 1.100.0-nightly (67eda617e 2026-09-10).
- Included (all required, all recorded): `q-runtime-model`, `q-router`,
  `q-schema-runtime`, `q-bridge`, `q-pack` —
  `coverageComplete: true`, `anyFindings: false`, verdict **passed**
  (`benchmarks/raw/ga-m6-fuzz/miri-ledger.json`, per-crate logs alongside).
- Exclusions with recorded reasons and compensations: q-engine-quickjs /
  q-bytecode-tool (foreign C engine — compensated by S2 UBSan), q-engine /
  q-runtime / q-http (tokio/hyper async executor boundary), q-capabilities*
  (tokio harness; first-party unsafe-free logic under S1), q-browser-kernel
  (wasm-bindgen ABI), q-bench-support (instrumentation-only unsafe;
  explicit owner waiver 2026-09-12). Full list in the ledger's
  `excludedWithReason`.

## Unsafe / FFI audit

- `scripts/unsafe-audit.py` → `docs/production/evidence/m6-unsafe-audit.md`
  (already committed): 22 unsafe occurrences across q-bench-support,
  q-engine-quickjs, q-pack, q-runtime; **0 unclassified** (fail-closed —
  a new unclassified unsafe block breaks the gate).
- Findings-disposition discipline for the audit tool itself is recorded in
  the PR history of this program (reviews of 657f3106 → PR #1332:
  reclaim classification fixed; a text-pair-match narrowing in
  `classify()` is deferred to its next touch — cosmetic, not a coverage
  hole).

## TSan / Loom-style

Explicit applicability waiver recorded in
`fuzz/sanitizer-applicability.json` (owner review 2026-09-12): the
concurrency-critical first-party surface is exercised by deterministic
property suites (q-bridge slab lifecycle, M3-005/M3-008 fairness +
shutdown, M3-007 cancellation) and the 24h/72h soak under concurrent
producers; tokio/hyper do not compile cleanly under TSan without
`tokio_unstable` churn, and the serving path contains no hand-rolled
lock-free algorithm. Revisit trigger: any such structure entering the
serving path.

## No known memory-safety P0/P1

- Sanitizers/Miri: zero findings this campaign (above).
- The one fuzz-found defect in the program (RFC 9457 envelope shadowing)
  was a correctness bug, not memory unsafety; fixed in PR #1339 with a
  regression test (see the M6-002 report).
