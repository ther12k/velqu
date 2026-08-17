# Project Q Design Update Log

## 2026-08-17

- **Creation** — Created an OKF v0.2 design and product bundle for the proposed Rust + QuickJS cold-start-first TypeScript framework.
- **Review** — Reassessed the earlier architecture against current official Elysia 2, Eden Treaty, AWS LLRT, QuickJS, QuickJS-NG, rquickjs, Bun, hyper, Tokio, RFC 9457, and OKF sources.
- **Correction** — Reclassified native JSON decoding/validation and response serialization as measured hypotheses because cross-language materialization can erase native parsing gains.
- **Decision** — Kept Bun as the development toolchain while defining the production runtime as a separate Rust/QuickJS system.
- **Decision** — Selected QuickJS-NG only as the initial engine candidate behind an adapter; upstream QuickJS remains a benchmark alternative.
- **Decision** — Rejected simultaneous Rust and Zig implementation for the first version.
- **Decision** — Separated fast Treaty unit mode from native-runtime integration conformance.
- **Delivery** — Scoped the first agent handoff to M0–M2 and prohibited unsupported performance or compatibility claims.
- **Trust** — All design documents remain draft and unverified by a human reviewer.
- **Packaging** — Generated a machine-readable manifest, structural validation result, ZIP archive, and SHA-256 checksum for handoff.

## 2026-08-17 (implementation start)

- **Ingestion** — Bundle moved verbatim to `docs/okf/`; structural validation re-run (all PASS); implementation audit, open-decisions register, and live traceability created under `docs/`.
- **Owner instruction** — Working name for this implementation is "Velqu"; scope remains the authorized M0–M2 stop point.
- **Environment** — rustc 1.96.0, Bun 1.3.4, rquickjs =0.12.2 (vendoring quickjs-ng 0.15.1) verified to build and evaluate in this environment.
- **Freeze** — Benchmark fixture contract, application pack format v1, and public API sketch frozen under `benchmarks/fixtures/fixture-contract.json` and `docs/specs/`.

## 2026-08-17 (M0/M1 complete)

- **M0 PASS** — frozen fixture contract + canonical checker: all four candidates 27/27; type spike + scale runs recorded (budget miss at 100 routes: fixed tsc floor).
- **M1 PASS** — Rust host + single quickjs-ng worker: 45 tests green; route-before-JS, lazy bridge counters, cancellation matrix, limits, tamper rejection, redaction, source maps all demonstrated on the actual binary.
- **Measurement** — velqu C3 cold p50 2.9ms / p95 4.4ms vs matched Elysia 2 AOT p50 132.6ms / p95 152.0ms; idle RSS 6.2 MiB; 0 failures in 1680 cold samples.
- **Correction (evidence-driven)** — bridge benchmark: native (Rust serde) JSON inputs beat engine JSON.parse by 11–42% on this host (counter to the review's expectation) → adopted as compiler default; ADR-0015.
- **Negative result preserved** — 1,000-route cold start p50 15.7ms is +409% over 25 routes (budget ≤20%): FAIL recorded honestly; absolute value still ~10× faster than matched Elysia candidate.
- **Treaty deviation** — route-id navigation chosen over Eden-exact single-segment form (ambiguity on `/users`); open decision ID-011.
