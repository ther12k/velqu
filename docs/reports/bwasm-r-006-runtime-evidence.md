# BWASM-R-006 — Verify and Package Browser-Runtime Evidence

## Result

**PASS** — the full R-phase (R-001..005) is verified at one exact
commit with the runtime suite, a REAL-kernel integration run, bundle
audit, artifact hashes, and the full repository gate.

## Verification matrix (commit recorded in `00-environment.txt`)

| Check | Result | Evidence |
|---|---|---|
| Runtime suite (R-001 contract 12 + R-002 dispatcher 15 + R-003 bundle 11 + R-004 worker 13 + R-005 capability/treaty 10, incl. purity + fixtures) | **59 pass / 0 fail** | `01-runtime-suite.txt` |
| REAL-kernel integration: pack fixture (regenerated at this commit) → wasm-bindgen glue → plan → complete | `REAL-KERNEL-INTEGRATION-OK` | `02-real-kernel-integration.txt` |
| Browser-target bundle audit | 26,322 B; **0** `node:`/`Bun.`/`require(` occurrences; all five public surface names present | `03-bundle-audit.txt` |
| Artifact hashes | glue wasm + JS recorded at this commit | `04-artifact-hashes.txt` |
| typecheck (tsc -b) | pass | transcript |
| `./scripts/verify` | **ALL PASS** at this commit after full toolchain setup | `01-runtime-suite.txt` (tail) |

## Evidence honesty

- One earlier verify run failed ONLY on the known
  missing-debug-fixture class (`embed bytecode: NotFound` — debug
  `q-bytecode-tool` not yet built in this fresh worktree); documented,
  setup completed, re-run: ALL PASS. A second "failure" was an operator
  error — verify was accidentally invoked from the main checkout
  (owner's in-progress branch) instead of this worktree; the path-carrying
  log made it obvious and the run was discarded as invalid rather than
  interpreted.
- WASM kernel execution: the browser path runs the real
  `q-browser-kernel` wasm (nodejs glue at this commit); no JavaScript
  matcher/validator substitute exists in the runtime — dispatcher and
  registry delegate every decision to the kernel (R-002/R-005 tests
  assert this structurally).

## Semantic differences (documented, linked)

- `/health/ready` host-level probe: native-only surface; browser 404 is
  correct (ADR-0037 §1; evidenced in R-002's diff file).
- Response streaming / request streaming / binary bodies / full
  multipart: unsupported in the MVP — `UNSUPPORTED_SEMANTICS`
  inventory (R-002) with fail-closed problems; part of the frozen
  contract (ADR-0039 §4 offline/bounded semantics), no new owner
  decision required.
- Worker isolation honesty: NOT a hostile-code sandbox (ADR-0038 §5;
  carried in R-004's bootstrap and report).

## Known limitations

- Real-browser lanes (CSP, actual Worker construction) remain Q-002 —
  the injected-factory seam is the tested boundary here.
- Carried from K-phase: base-kernel size 572,711 B gzip-9 exceeds the
  ratified ≤500 KiB budget — Q-gate finding (BWASM-Q-005), not waived.

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
