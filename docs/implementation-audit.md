---
type: Implementation Audit
title: OKF Bundle Ingestion and Implementation Audit
status: active
---

# Implementation audit (Stage 0)

Ingested bundle: `docs/okf/` (OKF v0.2, generated 2026-08-17, manifest
`docs/okf/MANIFEST.json`, structural validation `docs/okf/VALIDATION.json` —
all six structural checks PASS; 68 markdown files, 168 internal links).
The bundle was moved verbatim from the repository root to `docs/okf/` so the
repository root can follow the operational layout in
`docs/okf/engineering/repository-layout.md`. Content hashes are unchanged
(verified against `MANIFEST.json` during `scripts/validate-okf`).

Owner instruction for this implementation: the working product name is
**Velqu** (internal). "Project Q" in bundle documents refers to the same
system. No public naming/licensing decision is made here.

## Source-of-truth precedence (confirmed)

1. explicit owner instruction in the current task (working name Velqu; M0–M2 scope);
2. accepted decisions created during implementation (new ADRs under `docs/okf/decisions/`);
3. PRD `docs/okf/delivery/prd.md`;
4. ADRs `docs/okf/decisions/*`;
5. architecture specs `docs/okf/architecture/*`;
6. engineering standards `docs/okf/engineering/*`;
7. roadmap/backlog `docs/okf/delivery/*`;
8. references `docs/okf/references/*`;
9. design-session notes.

## Classification of material statements

The full requirement inventory lives in `docs/okf/project/requirements.md`
(P0/P1/P2). Classification of its load-bearing statements:

- **Accepted working decisions**: toolchain split (Bun dev, Rust+QuickJS-NG
  prod, ADR-0001/0006); cold-start-first optimization target (ADR-0002); no
  Node/Bun compatibility promise (ADR-0003); static contract, no app dry-run
  (ADR-0004); native routing + lazy bridge (ADR-0005); Treaty dual contract
  mode (ADR-0007); one runtime per worker (ADR-0008); schema IR with explicit
  fallback (ADR-0009); capability plugin model (ADR-0010); RFC 9457 typed
  results (ADR-0011); evidence before performance claims (ADR-0012);
  Rust-only host (ADR-0013); version-pinned bytecode trust (ADR-0014).
- **Hypotheses requiring spike**: native JSON parse/validate beats QuickJS
  JSON (bridge strategy experiment, M1); lazy native request handles are safe
  and cheap; source-mapped QuickJS exceptions are usable; static compiler can
  extract contracts without app execution; Treaty types scale to 1,000 routes.
- **Measurable targets/budgets**: `docs/okf/engineering/performance-budgets.md`
  (all values are targets/gates, none observed yet).
- **Implementation requirements**: `docs/okf/project/requirements.md` P0 set.
- **Deferred features**: everything in PRD §6 non-goals and master prompt §5.
- **Contradictions/ambiguities**: see below.
- **Owner decisions**: name/scope/repo/license/governance/release (PRD §18).
- **Stop conditions**: master prompt §15; performance budgets kill criteria.

## Required audit examinations

### Can the compiler avoid app execution?

Working answer: yes for the authorized authoring surface. `route()`,
`defineModule()`, `defineApp()`, `definePolicy()` are pure data constructors;
the compiler (M2) reads their literal arguments via TypeScript AST analysis
without evaluating the program. Services are referenced by ID only
(`defineService` factories are never invoked by the compiler — trap tests
required). Residual risk: dynamic composition (`routes: [...spread]`) — the
compiler must reject non-literal spreads/conditionals with source-located
diagnostics rather than executing anything.

### Can lazy native handles be made safe?

Working answer: yes via opaque generation-checked indirection (worker id,
runtime generation, invocation generation, slot, kind) specified in
`docs/specs/pack-format-v1.md`. No raw pointers cross FFI; the request store
is a Rust slab; JS accessors validate generation before materialization;
settlement invalidates the slot; late native completions carry generation and
are dropped on mismatch. Requires M1 expiry/ownership tests to be believed.

### Are schema semantics small enough?

Yes. Schema IR v1 (string/integer/number/boolean/literal/enum/optional/
nullable/array/object/union with the listed constraints) covers all proof
routes. Anything else fails the build or is an explicit, visible JS fallback.

### Could unit-local testing hide runtime defects?

Yes — that is why TRT-005 forbids counting unit-local Treaty results as
conformance. Runtime-local tests must drive the actual `q-runtime` binary
over HTTP. Both modes are implemented separately and labeled in `@q/testing`.

### Are benchmark candidates feature-matched?

Audited continuously via `benchmarks/fixtures/fixture-contract.json` (frozen)
and `docs/reports/fairness-audit.md`. Baselines implement the identical
observable behavior: same routes, statuses, validation semantics, payloads,
no compression/TLS, release builds, idiomatic per stack. Raw Rust is a
transport lower bound and is labeled as having no Treaty parity — it is not
used to imply framework feature equality.

### Does any scope accidentally become a Node compatibility project?

No. `node:`/`bun:` imports and runtime globals fail the build (COMP-006); the
engine exposes only `velqu.capabilities.*` natives; no `process`, no `Buffer`,
no filesystem/network ambient authority. The compatibility surface is the
declared capability list in the pack.

### Is any same-process sandbox claim overstated?

The documentation consistently says same-process QuickJS is for **trusted
application code only** (SEC-002). We preserve that wording everywhere and add
engine resource limits as robustness controls, never as a sandbox boundary.

## Environment/tooling facts recorded at implementation start

- rustc/cargo 1.96.0 (pinned via `rust-toolchain.toml`), Linux x86_64
  (kernel 7.0.0-28-generic).
- Bun 1.3.4 (dev toolchain), Node 24.11.0 available but not used for app
  runtime.
- Engine pin: rquickjs =0.12.2, which vendors **quickjs-ng 0.15.1**
  (`rquickjs-sys-0.12.2/quickjs` source tree, `QJS_VERSION 0.15.1`).
- Baseline pins: Elysia `2.0.0-beta.4` (npm `next` dist-tag at freeze time),
  Bun 1.3.4 native HTTP, Rust hyper 1.x for raw-rust.
- Frozen interface specs: `docs/specs/pack-format-v1.md`,
  `docs/specs/public-api-sketch.md`, `benchmarks/fixtures/fixture-contract.json`.

## Corrections made during ingestion

1. The bundle root README referenced `MASTER_AGENT_PROMPT.md` etc. at bundle
   root; the bundle now lives at `docs/okf/` with all internal links intact
   (relative links were preserved by moving the bundle as a whole).
2. Benchmark fixture list in master prompt §10.1 (`/native-live`, `/js-text`,
   ...) unified with PRD §13 proof routes into one canonical frozen contract
   (`/health/live` serves as the C0 liveness route; no separate `/native-live`
   alias) — recorded in `benchmarks/fixtures/fixture-contract.json`. This is a
   naming unification, not a semantic change; both sources describe the same
   behavior.
