# Runtime profiles

Velqu currently exposes two bounded startup profiles. The profile is an
explicit deployment choice and is reported by the runtime; there is no hidden
worker creation or fallback to an invented profile name.

## `serverless`

`serverless` starts exactly one QuickJS worker before readiness. It minimizes
cold-start work and is the default for `velqu init`, `velqu dev`, and `velqu
build`. It is the least surprising profile for local development and small
request volumes.

```bash
velqu dev --profile serverless
velqu build --profile serverless
```

## `service:N`

`service:N` starts the configured number of workers before readiness. `N` is
an integer from 1 through 64. Readiness is not declared until every configured
worker is ready, so a service profile has deterministic startup capacity.

```bash
velqu dev --profile service:4
velqu build --profile service:4
```

The profile is part of the runtime configuration and inspect/build evidence.
It does not change the route contract or make application code a hostile-code
sandbox; same-process QuickJS still executes trusted application code only.

## Fail-closed grammar

Only these values are accepted:

```text
serverless
service:1 ... service:64
```

Bare `service`, `throughput`, zero workers, values above 64, and malformed
suffixes are rejected with an actionable error. The CLI scaffold and Rust
runtime use the same grammar, preventing a generated command from failing
later at startup.

## Choosing a profile

| Situation | Profile | Reason |
| --- | --- | --- |
| Local learning or cold-start-sensitive deployment | `serverless` | exactly one worker before ready |
| Predictable multi-core service capacity | `service:N` | all N bounded workers ready before serving |
| Unknown or unbounded worker demand | neither | choose an explicit bounded N; autoscaling is outside this packet |

Profile selection is not a performance guarantee. Any cold-start or throughput
claim requires matched retained samples with p50/p95/p99 evidence under
`benchmarks/raw/`; this guide reports configuration semantics only.

## Verify

From the repository root:

```bash
bun install --frozen-lockfile
bun test packages/cli/src/profile-fetch-choices.test.ts
bun run typecheck
cargo test -p velqu-runtime
cargo test -p q-engine-quickjs
```

These tests cover default serverless generation, explicit `service:4`,
fail-closed invalid names/counts, and runtime readiness. This is private-alpha
documentation and is not a production-readiness claim.
