# BWASM-K-005 — Rust Browser Kernel and wasm-bindgen ABI

## Result

**PASS with one mandatory budget finding** — `q-browser-kernel` exists
and passes its full request-path suite natively (15/15) and
**on-target as WebAssembly** (14/14; one native-only test, see
limitations), the wasm-bindgen ABI is generated and hashed, and the
import audit is clean. The composed kernel measures **572,711 bytes
gzip-9, exceeding the ratified ≤500 KiB base-kernel budget** — recorded
below as a release-blocking finding for the Q-gate (ADR-0039: budget
misses are never silent).

## Deliverable

`crates/q-browser-kernel` (rlib + cdylib):

- **`BrowserKernel::init(pack_bytes)`** — bounded (16 MiB cap),
  mixed-mode rejection, full in-band integrity verification
  (`verify_from_slice`, ADR-0026 semantics; integrity-only in the
  browser per ADR-0037 §4), `BytecodePolicy::Skip` (browser handlers
  evaluate from the SOURCE bundle — native QuickJS bytecode has no
  browser meaning, ADR-0037 §8), router build. Every failure is a
  typed `artifact` problem (fail closed; panics never become success —
  the bindgen layer converts boundary panics to `internal` problems).
- **`plan_request`** — versioned bounded JSON message in; router
  resolve (K-003 semantics: 404 / 405+allow / found), raw path-param
  materialization, params/query/headers/body schema validation through
  q-schema-runtime (fail-closed on declared-but-missing schemas),
  **capability authorization at plan time** (every route-declared
  capability must be carried by the artifact inventory — the ADR-0037
  §5 deployment-required class returns a stable `capability` problem,
  501), declared statuses, deadline. Output: a complete `invoke` plan
  the Worker consumes verbatim.
- **`complete_invocation`** — declared-status enforcement (undeclared
  status ⇒ contract violation 500, native semantics), response-schema
  validation with field errors attached, handler-problem
  normalization through the native problem registry (stable URIs
  `https://velqu.dev/problems/…`).
- **`authorize_capability`** — bridge query for the future runtime
  layer, fail-closed without an inventory.
- **wasm-bindgen ABI** (`--features bindgen`, wasm32-unknown-unknown
  cdylib): `kernel_abi_version()`, `WasmKernel` constructor (artifact
  problem carried as error message JSON), `plan_request`,
  `complete_invocation`, `authorize_capability`, explicit `dispose`.
  Pinned to the installed CLI's schema (`=0.2.108`).

Kernel-specific problem ids (`artifact`, `capability`, `abi`) are
documented in-crate; everything else reuses the native registry.

## Evidence

### Test execution (report sources: transcript + CI disclosure)

| Run | Result |
|---|---|
| `cargo test -p q-browser-kernel` (native) | **15/15 pass** |
| `cargo test -p q-browser-kernel --target wasm32-wasip1` (executed as WebAssembly, K-004 harness) | **14/14 pass** |
| `cargo check --target wasm32-unknown-unknown -p q-browser-kernel` (+ `--features bindgen`) | PASS |
| wasm32 dependency audit (`cargo tree`, normal edges) | **0** tokio/hyper/rquickjs/memmap2/ed25519/getrandom/postgres/q-engine |

The request path proven end-to-end (no API server): pack bytes →
verified init → `plan_request` (`/health/live` → invoke plan with
handlerKey/allowedStatuses/deadline) → `complete_invocation` (200 +
body → normalized response). Negative classes pinned with stable
problems: unknown route (404), wrong method (405 + `allow`), ABI
mismatch (400, names both versions), oversized message (413),
malformed message (400), tampered pack (artifact), **undeclared
status (500 contract violation)**, unknown completion route,
capability-not-in-inventory (501), authorize-denied.

### WASM import audit (committed tooling + output)

`scripts/bwasm-wasm-import-audit.py` (dependency-free wasm import
parser) on the generated module:

```text
imports: 2
  ./q_browser_kernel_bg.js.__wbg___wbindgen_throw_...
  ./q_browser_kernel_bg.js.__wbg_Error_...
AUDIT-CLEAN: no host-runtime imports (no wasi/fs/socket/thread imports)
```

### Artifact hashes and sizes (release, default profile)

```text
wasm-bindgen --target web (CLI 0.2.108, schema-pinned)
q_browser_kernel_bg.wasm  raw 1,731,509  gzip-9 572,711  sha256 db72b8e82da56e7e5752878a8d3cc064ceba368711263bdcfbb04e0f7c8c48de
q_browser_kernel.js      raw    11,687  gzip-9   2,830  sha256 900618abaebc7aecb750e66a2859125375af61a9e056c9681c89e750bed7ee49
```

### ⚠ Budget finding (release-blocking for the Q-gate, recorded — not waived)

**Base kernel 572,711 B gzip-9 > 500 KiB ratified budget (ADR-0039).**
Context, honestly: this is the FIRST measurement of the fully composed
kernel, built with default release settings — no `wasm-opt`
(unavailable on this host), no size-oriented profile (opt-level/lto),
panic/unwind machinery and bindgen shims included, and it links the
entire byte-verification core + router + schema runtime + regex +
serde_json. Per ADR-0039, a budget change requires an amendment with
measured evidence; size-reduction work (profile tuning, wasm-opt in the
toolchain, export-surface minimization) belongs to the Q-005 budget
packet with this measurement as its baseline. K-005 neither hides the
miss nor unilaterally relaxes the budget.

## Known limitations / honest boundaries

- One test is native-only (`init_rejects_oversized_pack`): its 16 MiB
  allocation SIGSEGVs under Node's WASI preview1 (runner
  memory-growth limitation; the isolated test passes natively and the
  on-target path is a length comparison). Documented in-test.
- Browser packs in production will come from the future
  content-addressed artifact manifest (BWASM-B-002); today the kernel
  accepts the existing pack formats through the byte core — that is
  the K-005 boundary.
- The JS-side runtime (`@velqu/browser-runtime`, fetch dispatcher,
  Worker execution) is R-phase work; this packet proves the kernel
  request path and ABI, not a full browser application.
- `wasm-bindgen` is pinned to `=0.2.108` to match the installed CLI's
  schema; version bumps must update both sides together.

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
