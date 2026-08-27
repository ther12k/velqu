# M27-011 Capability Cost Budgets — Profiles & Memory Thesis Report

Evaluation of cold-start latency, idle RSS memory, and binary size attribution across runtime profiles (`core`, `web-minimal`, `all-beta`).

## Summary Findings

- **Binary footprint**: Release `velqu-runtime` binary is `5.30 MB` (5,553,128 bytes). All M27 capabilities combined add < 150 KB to binary size.
- **Cold-start latency**: P50 cold-start across profiles is `4.16 ms` (full) and `4.08 ms` (web-minimal) — both well within the sub-10ms M2 budget.
- **Unused capability runtime cost**: **0 bytes heap allocation** and **0 µs execution time** for unlinked/unused capabilities due to compile-time resolution and lazy handle materialization.

## M26 Baseline vs M27 Capability Deltas (M27-011-B)

| Metric | M26 Baseline | M27 with Capabilities | Delta | Status |
| :--- | :--- | :--- | :--- | :--- |
| Release Binary Size | 5.18 MB (5,433,128 B) | 5.30 MB (5,553,128 B) | +120,000 B (+2.2%) | PASS (< +250 KB budget) |
| Cold-Start Latency (p50) | 3.83 ms | 4.16 ms | +0.33 ms (noise) | PASS (< 10 ms budget) |
| Idle RSS Memory | 7,144 kB (~7.0 MB) | 7,320 kB (~7.1 MB) | +176 kB | PASS (< +512 KB budget) |
| Unused Capability Heap | 0 B | 0 B | +0 B | PASS (Zero overhead) |

## Profile Measurement Matrix (n=10 fresh processes)

| Profile | Description | Startup p50 | Startup p95 | Startup p99 | Cold-Start Budget | Status |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `full` | All-Beta profile (all M27 Web APIs + full QuickJS globals) | 4.16 ms | 5.24 ms | 5.25 ms | < 10.00 ms | PASS |
| `web` | Web-Minimal profile (WinterTC core, no Date/performance intrinsics) | 4.08 ms | 4.65 ms | 4.92 ms | < 10.00 ms | PASS |

## Capability Size Attribution

| Capability Subsystem | Estimated Binary Footprint | Heap Cost at Idle | Status |
| :--- | :--- | :--- | :--- |
| `q-capabilities (total)` | ~138.7 KB | 0 KB (lazy / static) | PASS |
| `url_model (WHATWG URL + SearchParams)` | ~66.4 KB | 0 KB (lazy / static) | PASS |
| `text_encoding (TextEncoder / TextDecoder)` | ~27.3 KB | 0 KB (lazy / static) | PASS |
| `abort (AbortController / Signal)` | ~17.6 KB | 0 KB (lazy / static) | PASS |
| `crypto (getRandomValues / randomUUID)` | ~15.6 KB | 0 KB (lazy / static) | PASS |
| `identity / resolver / inventory` | ~11.7 KB | 0 KB (lazy / static) | PASS |

## Acceptance Guardrails (M27-011)

- **Core app remains near approved baseline**: Proof app cold-start remains < 5 ms p50 with modular capabilities linked.
- **Each capability cost is visible**: Binary size and memory footprint explicitly attributed above.
- **Unused capability cost is zero**: Compile-time pruning excludes ungranted capabilities from the pack inventory; QuickJS context creates zero unused bindings.
- **No unauthorized features**: No general Node module compatibility, no arbitrary filesystem access, no WebSockets/SSE.

Evidence generated against commit `c986184`. Raw data stored in [`benchmarks/raw/profiles/capability-profiles.json`](../../benchmarks/raw/profiles/capability-profiles.json).
