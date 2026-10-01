# relay-service — external Velqu consumer (#1400)

A webhook/integration relay demonstrating the patterns an independent
consumer uses Velqu for: token-authenticated intake, typed failures at
declared statuses, outbound fetch enrichment, and a bounded in-memory
delivery store.

It consumes the **published** `@velqu/*@0.1.0-beta.1` packages from npm
(no Velqu checkout needed). `scripts/beta-external/relay-service.sh`
drives the full journey — install, check, test, build, serve on the real
Rust runtime, and assert happy paths plus typed failures.

## Routes

| Route | What it shows |
|---|---|
| `GET /health/live` | standard liveness |
| `POST /hooks` | `webhook.token` policy over the `authorization` header (typed 401 without `Authorization: Bearer q-relay-demo-token`), validated bounded body (422), bounded store insert (201) |
| `POST /hooks/:deliveryId/enrich` | outbound `fetch` to the loopback upstream (`UPSTREAM_BASE`, port 18971 — the driver binds it; loopback trust is the runtime's explicit integration-test policy), typed 502 on upstream loss, typed 404 for unknown deliveries |
| `GET /hooks/:deliveryId` | read-back with `enrichment: "pending"` until enriched; typed 404 |

The webhook token is a **fixture** (`q-relay-demo-token`), like the proof
app's session fixture — real deployments source it from their secret
channel. The policy reads the standard `authorization` header; custom
policy header names (e.g. `x-hook-token`) hit a compiler bug found by
this very consumer app (#1401 — extraction dropped the declared header;
fixed in-repo, usable from the next package publish). Webhook payloads
are a bounded JSON **text** field: the schema IR has no
arbitrary-record validator, and an unbounded blob would break the
bounded-by-construction posture. The store is in-memory best-effort —
durable delivery belongs to an external system.

## Manual run

```bash
bun install
bun run check && bun test && bun run build
# upstream stub (what the driver does):
bun -e 'Bun.serve({port:18971,fetch:()=>new Response(JSON.stringify({enrichment:"stub-enrichment"}),{headers:{"content-type":"application/json"}})})' &
VELQU_RUNTIME=/path/to/velqu-runtime bun run dev   # needs the runtime binary
```

Builds are reproducible only on Bun 1.4.0 / TypeScript 5.9.3 exactly
(the build refuses other versions by design).
