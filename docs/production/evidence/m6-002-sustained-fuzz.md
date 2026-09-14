# M6-002 — Sustained Fuzz and Property Campaigns (evidence closure)

Issue #1318 (GA delta M6-002). This report binds the committed raw
evidence to the acceptance criteria. The verdict comes from the recorded
ledger, not from this document: `benchmarks/raw/ga-m6-fuzz/campaign-ledger.json`.

## Final campaign (candidate b8fee349)

- Started 2026-09-13T22:33:27Z, seed 1789338807, libFuzzer via
  cargo-fuzz 0.13.2 on nightly 1.100.0-nightly.
- Candidate commit (verified by the fail-fast orchestrator before stage 1):
  `b8fee34909c0d6a4b3b9f76aebe9665cf3192c99` — the merge of PR #1339.
- 900 s per libFuzzer target, 600 s TS campaign, then S1/S2 sanitizer
  stages (recorded in the same ledger; see the M6-003 report).
- Ledger verdict: `campaignPassed: true`, `totalNewFindings: 0`,
  `fuzzFailed: false`, `sanitizerFailed: false`.

| Target | Surface | Duration | Exit | Findings | Corpus before → after |
|---|---|---|---|---|---|
| pack_verify | QPack verifier, mixed-mode rejection, bytecode policies | 900 s | 0 | 0 | 34610 → 41779 |
| router_match | method/path resolution | 900 s | 0 | 0 | 248 → 254 |
| http_decode | HTTP admission (query/percent, header bounds) | 900 s | 0 | 0 | 2953 → 3080 |
| schema_validate | schema codecs, classification, backtracking | 900 s | 0 | 0 | 15646 → 17818 |
| bridge_handles | generation checks, stale/foreign handle access | 900 s | 0 | 0 | 145 → 146 |
| codec_encoders | response/problem encoders (the bytes Treaty decodes) | 900 s | 0 | 0 | 11874 → 14030 |
| capabilities_policy | fetch SSRF gate, redirect limiter, capability metadata | 900 s | 0 | 0 | 4500 → 5524 |
| ts-treaty (TypeScript) | Treaty path interpolation, query serialization, response mapping | 600 s | 0 | 0 | seeded property campaign |

Surface mapping for every #1318-named boundary is in `fuzz/COVERAGE.md`
(nothing silently dropped). Corpus lives in `fuzz/corpus/<target>`
(gitignored by size policy); per-target before/after counts are recorded
in the ledger, and the enriched corpus is the starting point for future
campaigns.

## Findings → regression tests (the acceptance rule)

One real defect was found by this program, on the previous candidate
`47ddc3fc` (archived campaign: `benchmarks/raw/ga-m6-fuzz-47ddc3fc-PARTIAL-SUPERSEDED/`):

- **codec_encoders**: input `{"\xff":0,"status":{}}` made
  `ProblemProgram::encode` append an extension member named `status`
  after the frozen envelope, so the emitted problem+json parsed back with
  `status` as `{}` instead of the declared 422 (RFC 9457 duplicate-member
  shadowing). The crash artifact is retained at
  `.../disposition/crash-d370c25c69550dca3034052889f73fd2560a5a9e`.
- **Disposition (PR #1339, merged as b8fee349)**: reserved envelope
  member names (`type`/`title`/`status`/`instance`/`detail`/`errors`)
  are skipped at all three layers — `q-schema-runtime`
  `ProblemProgram::encode`, `q-runtime` `problems::body`, and the
  `q-engine-quickjs` worker extraction reserved set. Regression test
  `problem_encoder_skips_envelope_named_extensions`
  (`crates/q-schema-runtime/src/encoder.rs`) includes the exact campaign
  input. Post-fix: crash artifact replay passes; sustained confirmation
  rerun of codec_encoders alone (seed 1789340122, 6,289,132 execs /
  901 s) — zero findings.
- The partial 47ddc3fc soak invalidated by the candidate move is archived
  with its honest partial record at
  `benchmarks/raw/ga-m6-soak-72h-47ddc3fc-PARTIAL-SUPERSEDED/`.

## Honest provenance

`benchmarks/raw/ga-m6-fuzz/PROVENANCE.md` records the mid-run execution-
environment rebuild incident and the file-by-file post-hoc verification
of the completed ledgers. The fail-closed design held throughout: an
unfinalized ledger carries no verdict by construction.

## No outstanding crash/panic/UB/unbounded allocation

As of this campaign: zero new findings on the current candidate, and the
single program finding is fixed with a regression test (above). The
rss_limit_mb=4096 bound on every target covers unbounded allocation.
