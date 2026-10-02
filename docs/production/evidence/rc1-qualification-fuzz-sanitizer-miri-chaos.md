# RC1 qualification evidence — fuzz, sanitizers, Miri, chaos (2026-10-03)

Candidate **RC1 `8b3dce23`** (OD-011). Fresh runs per the
evidence-transfer doctrine; soak r5 (M6-009) is the remaining track and
is excluded from this packet. Provenance and attempt history:
`benchmarks/raw/ga-m6-fuzz/PROVENANCE-RC1.md`.

## M6-002 — sustained fuzz and property campaigns: **PASS**

Ledger: `benchmarks/raw/ga-m6-fuzz/campaign-ledger.json` →
`campaignPassed: true`, `totalNewFindings: 0`. Seven libFuzzer targets
(900 s each, rss-bounded) + the TS Treaty property campaign — all
zero-findings. Corpus growth recorded per target in the ledger.

## M6-003 — sanitizers, Miri, unsafe audit: **PASS**

- S1 ASan (with LSan) workspace pass — `asan-workspace.log`, zero
  findings.
- S2 UBSan over the QuickJS C FFI — `ubsan-quickjs-ffi.log`, zero
  findings.
- Miri over the five FFI-free crates — `miri-ledger.json`:
  `verdict: passed`, `coverageComplete: true`, zero findings;
  exclusions unchanged from the recorded scope with reasons.
- Unsafe-audit: the fail-closed classifier (`scripts/unsafe-audit.py`
  → `m6-unsafe-audit.md`) is part of the standard verify battery and
  ran green in the #1417 CI on this candidate line.

## M6-008 — chaos and fault injection: **PASS**

Record: `m6-008-chaos-rc1.md`; raw: `benchmarks/raw/ga-m6-008-chaos-rc1/`.
901 s, 2.24 M dispatched, completion 0.9900 exactly as configured, error
classes only the two injected ones at exactly their rates, 14 worker
poison/replacements, no monotonic retention.

## Found and fixed during qualification (disclosed)

1. **#1416 fuzz-kit drift** — two targets failed to compile after
   `property_order` landed post-b8fee349; harness-only fix (#1417,
   merged). Crates under test byte-identical to the RC.
2. **Environment**: sysroot `cc` lacks the libstdc++ dev path for
   libFuzzer links; `LIBRARY_PATH` fix recorded in provenance.

## Outstanding for the RC1 qualification set

- **M6-009 soak r5** — live since 2026-10-02T18:19:36Z on Halotec
  (r4-identical image and containment; q-soak `24bfc207…`); completes
  ~2026-10-05T18:20Z; its evidence packet follows termination and
  analysis.
