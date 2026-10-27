# Benchmark-manifest capture integrity — root cause of the local verify failure (#1429)

- Date: 2026-10-27
- Investigated at: master `2990d3a0` (clean); capture commit `1638c44`
- Raw hash record: `benchmarks/raw/native-repro/manifest-root-cause-2026-10-27/hashes.txt`
- Issue: #1429 — `validate-benchmark-evidence` fails on fresh rebuild with
  `qRuntimeRelease` hash mismatch while CI stays green

## Summary

The benchmark manifest's `qRuntimeRelease` byte-identity check has been
**structurally unpassable under `scripts/verify` since the manifest was
created on 2026-09-16** — at every commit, on every host. It was never a
reproducibility defect: builds are byte-reproducible on the capture host
across six weeks and across checkout paths (proven below). Three independent
defects made the gate unpassable, and the gate's failure message conflated
them into an unexplainable "hash mismatch".

## Findings

### 1. The manifest never described the evidence (capture-flow violation)

The committed benchmark runs were captured **2026-09-10** (runIds decode to
15:37–15:42Z; summaries committed via #1304). The manifest was refreshed
standalone on **2026-09-16 at 06:34Z** at `1638c44` — without re-running the
suite — binding the 09-10 evidence to a freshly built 09-16 binary. The
route-count summary's self-recorded harness binary (`binaryHashes`):
`c52385af…`; the manifest's `qRuntimeRelease`: `34076ab6…`. The manifest's
central claim — *this binary produced these numbers* — was false at capture
time and remained false in every commit since.

`scripts/refresh-benchmark-manifest.py` had no guard against this: it hashed
whatever `target/release/velqu-runtime` happened to contain.

### 2. Two canonical build modes, byte-different outputs (mode split)

The repo documents and tooling uses two different "canonical" builds of the
same tree:

| Flow | Command | `velqu-runtime` at `1638c44` | at `2990d3a0` |
|---|---|---|---|
| REBUILD.md §2, release packet, ga-native-repro lane | `cargo build --release -p velqu-runtime -p q-bytecode-tool` | `34076ab6…` | `11012a5f…` |
| `scripts/verify`, `scripts/benchmark` | `cargo build --release --workspace` | `1a5888fc…` | `e8701319…` |

Workspace-wide feature unification compiles ~100 dependency units under
different feature sets than package-mode resolution, producing a
byte-different runtime at both commits tested. The 09-16 capture hashed a
package-mode binary (following REBUILD.md); `scripts/verify` has always
validated its own workspace-mode build. **No commit, on any host, could ever
pass the check.**

### 3. Reproducibility itself is sound (no defect)

- A fresh package-mode build at `1638c44` on 2026-10-27 reproduces the
  manifest hash `34076ab6…` **byte-exactly** — six weeks after capture, in a
  different checkout path, on kernel 7.0.0-38 vs the capture-time
  7.0.0-31. Same toolchain (user-space cc 12.2.0, rustc 1.96.0, ld 2.42).
- A fresh workspace build at `2990d3a0` reproduces `e8701319…`
  byte-identically across two clean-room checkout paths.
- Cross-builder reproducibility is separately established by the pinned
  ga-native-repro lane (M6-006 report; `b8fee349` byte-identical across two
  independent pinned-container builders, capture-host difference explained by
  gcc provenance).

No ga-native-repro dispatch was needed for this root cause: the capture host
reproduces its own capture exactly, so the diverging input between the
manifest hash and a fresh verify build is identified directly — the build
mode, compounded by post-capture source drift (`1638c44..2990d3a0`:
26 files, +2150 lines under `crates/`). Renewing the lane's cross-builder
evidence belongs at the next RC/checkpoint candidate, as its report already
states.

### 4. The issue's "actual" hash was itself non-canonical

The `2d9b6866…` cited in #1429 as "fresh build at 2990d3a0" is not
reproducible from any canonical build of that commit (workspace `e8701319…`,
package `11012a5f…`). It most plausibly came from a packet-worktree build
(different dependency graph) or from ambient `RUSTFLAGS`/`CFLAGS` composed
into verify's non-hermetic `${RUSTFLAGS:-}` export — itself a byte-identity
hazard this packet now warns about.

## Fix (this packet)

`Closes #1429` via the validator's honest-mode arm of the acceptance, plus a
hardened capture flow so the deferred manifest refresh cannot repeat the
defect:

1. **`scripts/validate-benchmark-evidence.py`** distinguishes what is
   decidable:
   - hash **match** → pass (the capture-host canonical gate, unchanged);
   - mismatch **with runtime-input drift** since the capture commit (or an
     undecidable capture state / dirty runtime inputs) → **loud note** naming
     the capture, the drift, and the refresh command — per the ratified
     ADR-0040 principle that a gate must not fail where its check is
     structurally unpassable;
   - mismatch **without drift** → **error** (real reproducibility failure on
     this host: toolchain or flag divergence);
   - **new evidence-binding check**: every suite summary's recorded
     `binaryHashes` must match the manifest's `qRuntimeRelease` — strict
     error for v3 manifests, loud note for legacy v2. This check is decidable
     from tracked files alone, in every environment, and would have caught
     finding 1 at commit time.
2. **`scripts/refresh-benchmark-manifest.py`** (v3) refuses to produce a
   lying manifest: ambient flags beyond the canonical remaps are refused;
   uncommitted changes under build inputs are refused; the canonical
   package-pair release build runs inside the refresh (never hashing a stale
   or wrong-mode artifact); and every suite summary must have been recorded
   by the freshly built binary (finding 1 becomes mechanically impossible).
3. **One canonical mode**: `scripts/verify` and `scripts/benchmark` keep the
   workspace build for whole-workspace release coverage but finish with the
   canonical pair (`-p velqu-runtime -p q-bytecode-tool`), leaving
   `target/release/velqu-runtime` in exactly the mode REBUILD.md, release
   packets, ga-native-repro, and the manifest capture use. REBUILD.md §3
   records the mode and the ambient-flag requirement.
4. **Harness self-identification**: cold-start and warm summaries now record
   `binaryHashes` (route-count already did), completing the binding chain for
   all three suites.

## Residual state and follow-up

On master today the manifest is v2 and provably inconsistent (finding 1).
The validator reports this as a loud note, not an error, until the next
canonical capture — strict v3 enforcement begins with the first v3 manifest.
The **manifest refresh itself is deferred to the next evidence checkpoint**
(the post-merge SHA of the pending review-fix packets, or the RC2 candidate):
refreshing at `2990d3a0` now would be invalidated by the pending runtime
changes and would discard the pending-merge sequencing. At that checkpoint:
`scripts/benchmark` (full suite, canonical mode) →
`scripts/refresh-benchmark-manifest.py` (v3, binding-guarded) → verify →
commit; and dispatch `ga-native-repro` at the same SHA to renew cross-builder
evidence with `expected_runtime_sha` set to the fresh capture hash.
