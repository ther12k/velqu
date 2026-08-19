# Benchmark and Evidence Program

## Principles

1. Measure complete paths, not engine micro-numbers presented as application results.
2. Keep raw samples and failures.
3. Use semantically matched candidates and release builds.
4. Separate runtime/process/container/scale-to-zero cold starts.
5. Show CPU, RSS, queue wait, pool wait, and tail latency—not RPS alone.
6. Publish negative and crossover results.

## Candidate set

Required where semantically comparable:

```text
raw Rust
raw Bun
Elysia 2 AOT on pinned Bun
Hono on the same Bun
Fastify on pinned Node
Velqu source mode when relevant
Velqu bytecode/QPack v2 release mode
```

## Micro and warm classes

```text
C0 native/static liveness
C1 dynamic JS text
C2 dynamic small JSON
C3 validated params/query/body
C4 policy-protected validated route
C5 first use of lazy capability
C6 fetch/streaming
C7 multi-worker mixed load
```

Run fixed-duration tests at concurrency 1, 10, 50, and 200, with randomized candidate order and at least five repetitions for milestone claims.

## Cold-start classes

Report separately:

1. engine/application load;
2. local process spawn to ready;
3. local process spawn to first semantically valid response;
4. container create to first response with image present;
5. actual platform scale-from-zero request;
6. optional image-pull-cold result.

Route counts:

```text
25
100
1,000
5,000
10,000
```

## Real-world workloads

### W1 authenticated primary-key read

```text
Bearer verification
UUID/path validation
indexed PostgreSQL SELECT
1 KB JSON response
```

### W2 transactional order write

```text
2 KB request body
schema validation
stock check
order + line-item writes
commit
201 response
```

### W3 paginated aggregate

```text
cursor/limit validation
join + COUNT/AVG
4/16/64 KB JSON variants
```

### W4 controlled I/O

```text
0/1/5/10/25/50 ms
payload 256 B / 1 KB / 16 KB / 64 KB
concurrency 1/10/50/200
```

### W5 outbound fan-out

```text
database read
2 parallel upstream calls
success/slow/timeout/malformed combinations
```

### W6 CPU/JIT crossover

Measure first request, first 10/100, after 1k/10k warmup, and steady state for scalar operations, array mapping, rule evaluation, and JSON transform sizes.

Use cumulative break-even:

```text
Velqu(N) = cold_velqu + N * warm_velqu
Bun(N)   = cold_bun   + N * warm_bun
```

## Required outputs

```text
benchmark-manifest.json
environment.json
artifact-hashes.json
raw/*.jsonl
summary.json
report.md
fairness-audit.md
```

Every public statement is constrained to the exact tested workload and environment.
