# Real-World Benchmark: Unavoidable Candidate Differences

Status: normative for the real-world benchmark (BETA-002-D). This document is
hash-pinned into every candidate summary (`configHashes.differences`) and
cross-checked by the fairness audit, so a run set can only be compared when
every candidate was measured under this same declared set of differences.

The matched contract (BETA-002-A) makes the candidates answer identical SQL,
pool bounds, auth semantics, timeouts, logging/compression posture, and
byte-identical response bodies — the contract-response verifier
(BETA-002-C, `verify-contract.ts`) proves the response identity per candidate
before any timing counts. What remains different is documented here, on
purpose and without compensation.

## What is enforced identical

- SQL statements and parameter conventions (`candidates/parity.test.ts`)
- Connection pool bounds (20 connections, 5s connect, 30s idle)
- JWT rejection matrix (missing/malformed bearer → 401, benchmark token → session)
- Request/upstream deadlines (5s / 100ms), logging off, compression off
- Loopback HTTP/1.1 keep-alive, single-worker deployment posture
- Response bytes for all W1–W4 success and error fixtures
  (`contract-fixtures.ts` matrix, 18 fixtures × every candidate)
- Dataset (schema + seed hashes), workload definitions, version pins — all
  sha256-pinned per run and compared by `fairness.ts`

## Unavoidably different — measured as part of the candidate

These differences cannot be equalized without ceasing to measure the thing
the benchmark exists to measure. The harness's posture is: no compensation,
no normalization, raw rows retained (BETA-001-D), and conclusions must name
the candidate together with its environment.

### 1. Runtime process model and HTTP stack

- **Velqu**: Rust host owns sockets, routing, and teardown; handlers run as
  compiled QuickJS bytecode on one worker thread.
- **Hono / Elysia / bun-fetch**: one Bun process; JavaScript (JSC JIT) owns
  everything from socket to response.
- **Fastify**: one Node.js 22 LTS process; JavaScript (V8 JIT) owns
  everything.

Consequently event-loop pressure, GC pause profiles, JIT warmup, and HTTP
parsing/teardown costs differ by construction. They are inside the measured
surface: that is the point of a real-world benchmark.

### 2. JSON serialization path

Response bodies must be byte-identical (BETA-002-C), but the serializer that
produces them differs: Bun's `Response.json`, each framework's internal
serializer, Fastify's serializer, and Velqu's native JSON writer. The
serializer cost belongs to the candidate and is measured, not averaged away.

### 3. Native fetch clients (W4)

All candidates call the same controlled upstream with the same deadlines,
but through their own client: Bun fetch (hono/elysia/bun-fetch), Node's
undici (fastify), and Velqu's Rust fetch bridge. Connection pooling and
keep-alive internals differ; the upstream (latency source) is shared and
deterministic, so the client-side difference is attributable to the
candidate.

### 4. Router internals

Each framework keeps its own router (radix trees, registries, or Velqu's
compiled RoutePlan). The routing *behavior* is contract-verified; the
routing *cost* is candidate-specific and measured.

### 5. Runtime and toolchain versions

Framework packages are exact-pinned (`versions.json` + frozen
`candidates/bun.lock`, enforced by BETA-002-B tests), but the runtimes are
not equalized across candidates: Bun and Node versions are declared per run
in `summary.environment`. A comparison is only valid between runs that
declare the intended environments; `fairness.ts` compares host class
(os/arch) and fails on drift.

## Outside the measured surface (accepted behavioral divergences)

- **Malformed JSON request bodies on W2**: each framework's body parser
  produces its own error response (Hono/bun-fetch return the matched 400
  shape; Elysia and Fastify raise their own parser errors). The load
  generator never sends malformed bodies to W2, and the fixture matrix
  deliberately does not pin this path (BETA-002-C); it is therefore outside
  the measured surface. This is the one accepted behavioral divergence.
- **JWT library cost**: the matched contract authenticates with the
  benchmark token via a constant-time string compare. The benchmark does
  not measure JWT verification library cost; candidates are not required to
  substitute a real JWT library for it.

## Non-goals

No claim is made here about which difference dominates any workload, and no
performance number appears in this document. Measurements belong to
BETA-003+ and require matched, reproducible raw evidence (p50/p95/p99 with
retained samples) per the repository's evidence rules.
