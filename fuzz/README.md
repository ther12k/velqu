# Fuzz crate (M6-002 / #1318)

`cargo-fuzz` package for the sustained fuzz campaigns. Excluded from the
Rust workspace (`Cargo.toml` `exclude = ["fuzz"]`) so workspace tests and
release builds are unaffected by the sanitizer/coverage toolchain.

## Targets (trust boundaries)

| target | boundary | invariant |
|---|---|---|
| `pack_verify` | QPack bytes → verifier (`reject_mixed_mode_bytes`, `detect_pack_format_mode`, `verify_from_slice` × both bytecode policies) | any bytes → Ok or typed Err; never panic/hang |
| `router_match` | (method, path) → `Router::resolve` over a fixed 5-route table (static/param/wildcard/multi-param) | total classification |
| `http_decode` | query strings + percent escapes (`parse_query_with_policy`, `percent_decode_with_policy`) | no amplification (output ≤ 2x/3x input asserted) |
| `schema_validate` | arbitrary JSON → `validate` against a nested IR incl. a `(a+)+` backtracking-bait pattern | deterministic classification; <1 s watchdog against catastrophic backtracking |
| `bridge_handles` | op streams against a capacity-8 slab (insert/settle/stale-access) | stale/foreign handles never grant access; live ≤ capacity |
| `codec_encoders` | response + RFC 9457 problem encoders (M25-005/006) with adversarial extension maps | emitted bytes are valid JSON; RFC 9457 invariants hold |
| `capabilities_policy` | SSRF gate (`resolve_and_validate`), redirect limiter, capability identity/inventory/DAG | metadata-resolved hosts always denied; pin sets always dialable; typed outcomes |

Plus `scripts/ts-fuzz-campaign.ts` (seeded property-fuzz over the
published `treaty()` API — the TypeScript encoder surface cargo-fuzz
cannot reach) and `scripts/unsafe-audit.py` (the explicit unsafe/FFI
audit artifact). Full acceptance→target mapping: [COVERAGE.md](COVERAGE.md).

## Running

```bash
rustup toolchain install nightly --profile minimal --component miri
cargo install cargo-fuzz --locked

cargo +nightly fuzz run <target> -- -max_total_time=60   # smoke
bash scripts/fuzz-campaign.sh [--wait-for-pid N]          # sustained + ledger
bash scripts/miri-campaign.sh                             # Miri over FFI-free crates
```

Findings policy (owner directive 2026-09-11): campaigns record
duration/seed/corpus and either a regression-test reference per finding
or an explicit zero-findings record. Ledgers land in
`benchmarks/raw/ga-m6-fuzz/`.

Miri exclusions are recorded with reasons in the miri ledger (rquickjs
C FFI cannot execute under Miri; those boundaries are covered by the
libFuzzer + ASan/UBSan campaigns instead).
