# M28-002-A — Native HTTP Client Stack Comparison (reqwest vs Hyper/Rustls)

Evidence-backed stack-selection spike for the M2.8 native outbound fetch.
Decision: **hyper 1 + hyper-util (client-legacy) + hyper-rustls (webpki
roots)** — the lower-level stack.

## Protocol

Two standalone minimal binaries (own cargo workspaces under
`benchmarks/stack-spike/`, intentionally outside the Velqu workspace so
production dependency graphs are untouched) perform the identical
exercise: lazy client construction → loopback plain-HTTP GET → bounded
streaming prefix read → early drop (cancellation). TLS (rustls +
webpki-roots) is compiled into both candidates; both run the same
policy-shaped feature sets (no default TLS, no ambient gzip/brotli/zstd,
no cookies, no SOCKS, no HTTP/2). Release profile, n=20 fresh-process
spawns per candidate, wall-clock milliseconds.

- `spike-reqwest`: reqwest 0.12, `default-features = false`,
  `features = ["rustls-tls-webpki-roots"]`.
- `spike-hyper`: hyper 1 (`http1`, `client`) + hyper-util 0.1 (`client`,
  `client-legacy`, `http1`, `tokio`) + hyper-rustls 0.27 (ring, webpki,
  `http1`, TLS 1.2) + http-body-util.

Both binaries print machine-readable markers (`status=200`,
`first_chunk_len`, `prefix_ok`, `done=true`); every sample in both
candidates passed the functional check (20/20 each).

## Raw measurements

Raw JSON with all 40 spawn samples: `benchmarks/raw/stack-spike/stack-comparison.json`.

| Metric | reqwest 0.12 | hyper + hyper-util + hyper-rustls | Delta |
| --- | --- | --- | --- |
| Release binary size | 4,662,200 B (4.45 MiB) | 3,664,544 B (3.50 MiB) | **−997,656 B (−21.4%)** |
| Unique crates in graph | 84 | 43 | **−41 (−48.8%)** |
| Spawn p50 (ms, n=20) | 3.124 | 2.889 | −0.235 |
| Spawn p95 (ms, n=20) | 4.843 | 3.318 | **−1.525** |
| Spawn p99 (ms, n=20) | 6.242 | 3.320 | **−2.922** |
| Functional check | 20/20 pass | 20/20 pass | equal |

Production context: the release `velqu-runtime` (5,553,128 B) already
depends on **hyper 1 + hyper-util 0.1** (http1, server features) for
ingress. Choosing the hyper stack for outbound shares those crates, so
the real incremental binary cost is materially below the standalone
spike delta; reqwest would additionally pull its wrapper layer and
duplicate the machinery it wraps (reqwest 0.12 is itself built on hyper
+ hyper-util).

## Qualitative matrix (no single benchmark decides)

| Criterion | reqwest | hyper (lower-level) | Notes |
| --- | --- | --- | --- |
| Policy fit (ADR-0033) | weaker | **stronger** | ADR-0033 disables reqwest conveniences anyway (ambient gzip, cookies, env proxy, auto-redirects, its own timeouts). Every disabled feature is a "must remember to turn off" bypass trap; the lower-level stack starts from nothing and the policy applies directly at dial/read time. |
| Pooling control (M28-003) | wrapped | **direct** | hyper-util exposes the pool directly (idle timeout, keepalive, connection bounds); M28-003 must bound idle/active connections and DNS cache — direct knobs, no wrapper translation. |
| Cancellation / backpressure | yes | **yes** | Both drop-cancel futures and stream bodies; both spikes prove early-drop mid-body. Equal on capability; hyper avoids a wrapper between our scheduler and the stream. |
| Maintainability | high-level | **moderate** | reqwest is more convenient; but it is a wrapper over the same core we already ship. One less layer to audit; the audit surface halves with the crate count. |
| Dependency risk | 84 crates | **43 crates** | Half the graph; supply-chain surface and upgrade coupling both shrink. rustls/ring identical in both. |
| Size / cold start | − | **−21% binary, tighter p95/p99** | Measured above; startup delta is small at this scale but the p95/p99 spread favors hyper. |

## Decision record

**Select: hyper 1 + hyper-util 0.1 (client-legacy) + hyper-rustls 0.27
(ring, webpki-roots, http1, TLS 1.2).**

Rationale: measured advantage on every quantitative axis (21% smaller,
half the dependency graph, tighter tail latencies), direct pool/body
control required by M28-003/M28-006, zero duplicated policy surfaces
against ADR-0033, and reuse of crates already in the production binary
for ingress. The framework-benchmark guardrail is honored: the decision
rests on the full matrix above — policy fit, pooling control,
cancellation semantics, maintainability, dependency risk — with the
measurements as one input, not the verdict.

**Fallback strategy (documented)**: if the hyper-util legacy client
proves insufficient during M28-003/M28-004 (e.g., pooling semantics
cannot be bounded as required), reqwest with the same policy-shaped
feature set is the prepared fallback — the fetch layer consumes the
stack behind the ADR-0033 policy object, so the swap is contained to the
connector/pool implementation and no policy or surface code changes.

## Reproducing

```bash
cd benchmarks/stack-spike/spike-reqwest && cargo build --release && ./target/release/spike-reqwest
cd ../spike-hyper && cargo build --release && ./target/release/spike-hyper
# measurements were collected with the protocol described above (n=20)
```
