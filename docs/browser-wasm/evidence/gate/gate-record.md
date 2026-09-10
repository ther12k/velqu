# BWASM-GATE — GO record

- **Verdict: GO**
- **Date:** 2026-09-11
- **Decided by:** the repository owner, in an interactive session (one-word
  verdict "GO" on the gate briefing; recorded verbatim in issue #1179).
- **Rule applied:** any unresolved in-scope P0 ⇒ NO-GO (`candidate-index.json`
  `goNoGo.rule`). Zero unresolved P0s.

## Exact candidate

| Field | Value |
|---|---|
| Candidate packet | `bwasm-q-008-candidate-index` (`evidence/q-008/candidate-index.json`) |
| Packet sha256 (as verified at gate close) | `433a5215c20b8f26b6a7edddf8168168e3f55046641bf7f533fca8c99476d976` |
| Source commit | `9c658943f8b087c06e4f44ba70b685391a21e685` |
| Evidence binding | every sha256 inside the packet binds bytes produced from that source commit |
| Evidence verdict in packet | GO, `unresolvedP0: []`, 11 claims, 6 open risks (all P2, each with a documented disposition) |
| Independent check | `evidence/gate/dependency-closure.md` — reviewer commands against packet commit `344afa8`, validator 8/8 PASS, checksums recomputed |

The six open risks (OR-1…OR-6) are recorded in the candidate packet with
dispositions; none is in-scope P0. Notably OR-1: only the Chromium lane is
tested — Firefox/WebKit stay experimental and are never claimed as tested.

## Scope of this verdict

- **Included:** the Browser-WASM MVP contract per ADR-0037 — Rust/WASM kernel
  (routing, schema validation, QPack verification, capability authorization,
  problem mapping), generated TS handlers in an isolated Worker,
  `fetch(Request): Promise<Response>` as the public boundary, static-asset
  deployment. The epic issue #1179 closes against this candidate.
- **Still optional and NOT started** (each separately gated):
  - `BWASM-X-001` — QuickJS-NG-in-WASM engine-parity spike (ADR-0037 keeps
    QuickJS-in-WASM optional unless a recorded owner decision changes the
    release contract).
  - `BWASM-C-003` — optional PGlite-backed local SQL capability.
- **Out of scope:** the GA/production track (ADR-0019, `docs/production/`),
  which follows the beta track independently.

## Standing claim limits (unchanged by GO)

No hostile-code sandbox claims (ADR-0035/ADR-0038: the browser boundary
protects the host page and origin, and the process interior runs trusted,
pack-compiled application code); no production-secrets, shared-persistence,
or native-performance-parity claims (ADR-0039 budgets are normative targets
with measured evidence kept separate).

## Artifacts at verdict time

- Program task tree and evidence: `docs/browser-wasm/` (moved from
  `docs/codex-spark-browser-wasm/` in the same packet as this record; live
  references updated).
- CI at merge of the gate packet: verify green on x86_64 and aarch64,
  chromium required browser lane green (first full-matrix green since CI
  lanes resumed executing), firefox/webkit lanes green.

## Post-verdict candidate deltas (disclosed)

The GO above binds the candidate's evidence CONTENT at 9c65894. Two
mechanical deltas were applied while closing the gate; neither touched
evidence bytes:

1. `candidate-index.json` claim paths and `checksums.sha256` path strings
   were rewritten for the `docs/browser-wasm/` move (the packet sha256 in
   the table is the post-rewrite bytes; claim hashes are unchanged and
   verify).
2. `checksums.sha256` digests for `docs/beta/BROWSER_WASM.md` and
   `packages/browser-runtime/kernel/kernel.json` were refreshed: those two
   distributed files legitimately evolved after packet assembly —
   `kernel.json` by the #1295 route-binding gate fix, a drift that predates
   this gate packet and had gone unnoticed because the packet validator was
   not in the verify lanes (it is now run there). All other digests remain
   the original candidate bytes.
3. The differential-conformance claim (`differential-matrix.json`) is now
   declared `regenerated-per-run` instead of byte-pinned: the matrix is
   rewritten by `differential.test.ts` on every run with environment-bound
   header data (source commit, binary hashes, build ids), so byte-pinning it
   could not survive any later run on any machine — a flaw in the packet's
   evidence design that this gate close surfaced. The claim's substance
   (4 exact-parity / 3 equivalent-by-contract / 4 native-only, zero drift)
   is now verified structurally by the packet test. The captured bytes at
   9c65894 remain the assembly-time record in git history.
