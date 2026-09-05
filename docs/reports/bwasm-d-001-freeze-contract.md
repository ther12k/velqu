# BWASM-D-001 — Freeze the Browser-WASM Product and Runtime Contract

## Overview

Ratifies **ADR-0037** (`docs/okf/decisions/0037-browser-wasm-product-and-runtime-contract.md`),
which freezes the Browser-WASM product and runtime contract for the
`browser-hybrid` MVP profile. No runtime code changes; the ADR is the
deliverable, plus the task-record flip and this evidence report.

## Owner acceptance (provenance)

- Decider: repository owner.
- Provenance: the owner-delivered Browser-WASM GitHub packet
  (ZIP SHA-256 `a25e3610513f9a7c9a54c3fcf4dc104dfc13fe6df314d0f43602ba86fc1dd2bc`,
  research baseline `ther12k/velqu@84740c5`, 2026-09-04) specifies the
  hybrid architecture verbatim, and the owner's standing instruction of
  2026-09-05 directed freezing the contract through the four design
  decisions (BWASM-D-001..004) before any kernel implementation.
- Recorded in the ADR frontmatter (`owner-acceptance`) and dated
  2026-09-05.

## Decision table (ADR §1, §7)

| Surface | Class | Contract |
|---|---|---|
| Pack/QPack integrity verification | identical | ADR-0026 integrity-only, fail-closed, in-band digests |
| Routing by method/path | identical | native router semantics, K-003 split |
| Request/response schema validation | identical | `q-schema-runtime` semantics, K-004 |
| Problem mapping | identical | RFC 9457 shape, kernel-side |
| Capability ABI/lifecycle | adapted | ADR-0028..0031 over a postMessage bridge |
| `fetch` capability | adapted | declared allowlist, kernel-enforced, credentialless default |
| `defer` | adapted | bounded in-memory Worker-side, never durable |
| Postgres | deployment-required | typed fail-closed problem in-browser; async contract (BWASM-C-002) |
| Filesystem/ingress/scaling | unsupported | absent, never simulated |
| Handler JS engine | adapted | browser engine; **not** QuickJS-NG parity (explicit) |
| Explicitly simulated | (empty) | class defined to remain empty; additions need an owner decision |

## Architecture/sequence diagrams and ownership

- Sequence diagram (Request → WASM verify/route/validate → Worker
  handler → kernel validation → Response) is in ADR §3.
- Ownership is explicit at both boundaries: kernel owns everything
  before and after handler execution; the Worker is an execution island
  holding no artifact, router, registry, or ambient authority.

## Proof sketch of the boundary

The ADR §3 diagram is the normative proof sketch of
Request → WASM plan → Worker handler → WASM completion → Response; the
K-phase packets (K-005/K-006) must realize it and evidence it against
this ADR.

## Architecture review against the workspace graph

Measured dependency facts grounding the crate boundaries (ADR §6):

- `velqu-runtime` (native host): tokio, clap, serde, sha2, sourcemap,
  q-bridge/q-engine/q-engine-quickjs/q-http/q-pack/q-router/q-schema-runtime
  → **native-only**.
- `q-router` → q-engine, q-pack, thiserror (leaf, no tokio) → portable
  after the host split (K-003).
- `q-schema-runtime` → regex, serde, serde_json → portable as-is
  (K-004 qualification pending, D-002).
- `q-pack` → memmap2 (**native coupling** — the K-002 byte-core split
  motivation), ed25519-dalek, q-capabilities, q-engine, serde.
- `q-engine-quickjs` → rquickjs (native) → **native-only**; quickjs-wasm
  stays behind the ADR §1 decision gate.

## Rejected alternatives

Recorded in ADR §Rejected: full `q-runtime` wasm port; JS-only
reimplementation; QuickJS-in-WASM as MVP default; Service-Worker-
mandatory deployment. Each with rationale.

## Acceptance criteria — disposition

- [x] ADR includes architecture and sequence diagrams plus explicit
  ownership before and after handler execution (ADR §3).
- [x] Compiler execution may remain native/Bun for MVP; deployed runtime
  execution is browser-local (ADR §4).
- [x] Service Worker is explicitly an adapter over the canonical
  dispatcher (ADR §2), not mandatory for beta.
- [x] Default Worker handlers are explicitly not QuickJS-NG engine
  parity (ADR §8).
- [x] Owner acceptance recorded with date and provenance (ADR
  frontmatter; §Owner acceptance above) — before any K-phase merge.

## Commands run (doc-only change)

- `cargo tree -p velqu-runtime --depth 1`, `-p q-router`, `-p
  q-schema-runtime`, `-p q-pack` (dependency-graph review above)
- `./scripts/validate-okf` — pass (manifest hashes + internal links)
- `./scripts/verify` — ALL PASS (no executable change; full battery run
  for handoff completeness)

## Known limitations / residual risks / follow-ups

- The wasm32 compatibility measurements are D-002's deliverable; ADR §6
  records the expected split points, not measured proof.
- The threat model and isolation contract (preview origin, CSP, sandboxed
  iframe) are D-003's deliverable; ADR §8 defers to it explicitly.
- Support matrix, budgets, and measurement procedures are D-004's
  deliverable.
- Follow-ups: BWASM-D-002 (#1181), BWASM-D-003 (#1182), BWASM-D-004
  (#1183); BWASM-EPIC (#1179) closes after the design phase.
