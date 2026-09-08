# BWASM-GATE inputs — dependency closure table

**Status: INPUT, not a verdict.** Prepared by the implementation agent
(2026-09-08) as review input for the Owner's BWASM-GATE
(`gates/BWASM-GATE-browser-wasm-beta-readiness-go-or-no-go.md`, still `TODO`).
The GO/NO-GO decision, owner-decision log entries, and epic closure
(#1179) remain Owner actions — this document neither records nor implies a
verdict.

## Candidate of record

| Field | Value |
| --- | --- |
| Source commit (candidate bytes) | `9c658943f8b087c06e4f44ba70b685391a21e685` (every packet SHA-256 binds bytes built from this commit) |
| Packet commit | `344afa8` (PR #1294 — commits the packet; validator + checksums fully green there) |
| Packet PR | #1294 (BWASM-Q-008) |
| Packet index | `evidence/q-008/candidate-index.json` (11 hash-bound claims) |
| Identity | `evidence/q-008/candidate-identity.json` (commit, lockfile SHAs, toolchain, kernel pin) |
| Checksums | `evidence/q-008/checksums.sha256` (8 distributed files) |
| SBOM | `evidence/q-008/sbom-browser-wasm.cdx.json` (CycloneDX 1.5, 14 components) |
| Transcript | `evidence/q-008/reproduction-transcript.txt` |

**Post-candidate master delta (disclosed, unreviewed by any packet):**
master is now `86f3876` = #1295 (build-time guard rejecting non-exported
route bindings, from cleanroom follow-up #1292). #1295 changes no emitted
bytes for valid projects (B-001 golden emission and reproducibility tests
unmodified and passing), but it edits `docs/beta/BROWSER_WASM.md`, so that
one checksum digest differs on current master — verification commands below
must run at the packet commit `344afa8`. The Owner chooses: (a) gate the
recorded candidate, or (b) request a packet refresh against current master
before the verdict.

## Dependency closure table

All 12 mandatory BWASM-GATE dependencies are closed on GitHub. Byte-level
binding to the exact candidate lives in `candidate-index.json` where a
claim row exists (noted per row); other rows close via their merged PR and
carried evidence directories.

| Dependency | Issue | Closed (UTC) | Closing PR | Evidence of record |
| --- | --- | --- | --- | --- |
| BWASM-D-004 support matrix, claims, budgets | #1183 | 2026-09-05 12:19 | #1228 | `evidence/design-freeze-owner-decision.md`; `evidence/budgets.json`; candidate-index "ratified budgets" claim |
| BWASM-K-006 portable-kernel evidence | #1234 | 2026-09-05 17:20 | #1241 | `evidence/kernel-verification/`; candidate-index kernel-evidence claim (native tests, wasm32 checks, on-target execution) |
| BWASM-R-006 browser-runtime evidence | #1247 | 2026-09-06 03:19 | #1254 | `evidence/runtime-verification/`, `evidence/dispatcher/`, `evidence/handler-bundle/` |
| BWASM-B-006 cache/upgrades/rollback/static deploy | #1260 | 2026-09-06 09:49 | #1267 | `evidence/browser-lifecycle/` |
| BWASM-C-005 fail-closed capabilities | #1271 | 2026-09-06 18:16 | #1275 | `evidence/capabilities/` |
| BWASM-Q-001 differential conformance suites | #1276 | 2026-09-06 19:13 | #1284 | `evidence/conformance/differential-matrix.json`; candidate-index differential claim (4 exact-parity / 3 equivalent-by-contract / 4 native-only / 0 drift) |
| BWASM-Q-002 real-browser lanes | #1277 | 2026-09-06 19:44 | #1285 | `evidence/browser-lanes/`; `evidence/browser-matrix.json`; candidate-index Chromium-lane claim (journeys E1–E6) |
| BWASM-Q-003 preview-origin boundaries | #1278 | 2026-09-07 11:57 | #1288 | `evidence/q-003/`; candidate-index boundary claim (scope boundary, no hostile-code sandbox claims) |
| BWASM-Q-005 size/startup/latency/leak budgets | #1280 | 2026-09-07 12:36 | #1290 | `evidence/q-005/`; candidate-index budget-rehearsal claim: kernel 400,229B brotli ≤ 512,000B; total 453,771B ≤ 1MiB; cold 1938ms; p99 1.4ms; soak bounded |
| BWASM-Q-006 docs/limitations/migration | #1281 | 2026-09-07 12:52 | #1291 | `docs/beta/BROWSER_WASM.md`; `docs/beta/KNOWN-LIMITATIONS.md` items 19–24; candidate-index docs claim (terminology audit PASS, quickstart 4/4) |
| BWASM-Q-007 external cleanroom exercise | #1282 | 2026-09-07 23:09 | #1293 | `evidence/q-007/` (verbatim report, D1–D9 disposition, rounds 2–4); candidate-index cleanroom claims |
| BWASM-Q-008 release packet | #1283 | 2026-09-08 00:07 | #1294 | `evidence/q-008/` (this packet is the candidate) |

Optional work does not block: BWASM-C-003 (PGlite) was never registered as
a GitHub issue (owner-optional per gate criterion); BWASM-X-001
(QuickJS-WASM) remains owner-gated per ADR-0037 and the epic invariant.

## Gate acceptance-criterion map

| Gate criterion | Where the reviewer looks |
| --- | --- |
| Mandatory dependencies closed with exact-candidate evidence | Closure table above; `candidate-index.json` claim SHAs (all bind commit `9c65894`) |
| Zero unresolved P0 | `candidate-index.json` `goNoGo.unresolvedP0: []` (rule: any unresolved in-scope P0 ⇒ NO-GO), enforced by `candidate-index.test.ts` |
| Unresolved P1 disposition | None registered; all six open risks are P2 with recorded dispositions (below) |
| Static deployment without app server, claimed shapes | Chromium-lane and budget-rehearsal claims; `evidence/browser-lifecycle/`; cleanroom round 1 (static hosting, offline reload) |
| No hostile-code sandboxing overclaim | Boundary claim; ADR-0038 §5; KNOWN-LIMITATIONS items 19–24; docs terminology audit |
| C-003 / X-001 optional | Not blocking (see above); X-001 governed by OD-054 promotion rule |
| Exactly one prominent verdict | **Owner to record** in `gates/BWASM-GATE-…md` — intentionally blank |

## Open risks (candidate packet register)

All P2; none waived silently. P1-acceptance policy (OD-053) has nothing to
accept at packet level — the Owner confirms or escalates.

| ID | Severity | Disposition of record |
| --- | --- | --- |
| OR-1 | P2 | Non-Chromium browsers documented in `browser-support-matrix.md`; never claimed as tested |
| OR-2 | P2 | SW vs non-manifest assets documented in `BROWSER_WASM.md`; custom-page behavior recorded as data in q-007 |
| OR-3 | P2 | npm publication Owner-gated (AGENTS.md constraint 13); `INSTALL.md` documents the beta path |
| OR-4 | P2 | Non-exported route binding trap — follow-up #1292 registered, then **fixed** by #1295 (post-candidate; see delta disclosure) |
| OR-5 | P2 | `budgets.json` designates brotli as normative codec; resolved in q-005 |
| OR-6 | P2 | Capability-bridge honesty statement in `worker-host.ts`, docs, and q-003 evidence |

## Reviewer commands

From a checkout of the packet commit `344afa8` (verified green there on
2026-09-08: validator 8/8, all checksums OK). On current master the only
digest difference is `docs/beta/BROWSER_WASM.md` (PR #1295, disclosed above).

```bash
# 1. Candidate packet validator (schema, hashes, GO/P0 integrity, SBOM shape)
bun test ./docs/codex-spark-browser-wasm/evidence/q-008/candidate-index.test.ts

# 2. Distributed-bytes checksums
sha256sum -c docs/codex-spark-browser-wasm/evidence/q-008/checksums.sha256

# 3. Canonical full verification (network namespace; CI is not an acceptance basis)
unshare -rn bash -c 'ip link set lo up; ./scripts/verify'

# 4. Spot-check static deployment (documented workflow)
bun packages/cli/src/index.ts build --target browser-wasm --project examples/browser-demo
bun packages/cli/src/index.ts export --project examples/browser-demo --out /tmp/bwasm-export
```

## Honesty notes for the record

- GitHub Actions verify workflows remain stalled with zero executed steps
  (standing disclosure since ~#714); local gates in clean worktrees are the
  acceptance basis for every packet above.
- No hostile-code sandboxing, PostgreSQL-parity, or native-performance-parity
  claims are made anywhere in the packet; browser handlers are trusted code.
- This table was assembled by the implementation agent from GitHub issue
  state and committed evidence; the Owner independently confirms before any
  verdict. Per program guardrails, #1179 stays open until the Owner records
  GO in the gate file.
