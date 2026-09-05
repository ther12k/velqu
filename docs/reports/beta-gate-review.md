# BETA-GATE Review — Public Beta Readiness and Release Exit Gate

Milestone exit decision for the public beta (**0.1.0-beta.1**, ADR-0020).

Candidate commit: `65da9ed` (clean tree; full battery re-run from this
exact commit — results below).

## Milestone Decision: PASS

All 17 parent tasks (`BETA-001` through `BETA-017`) have passing
verification and evidence packets (ledger
`docs/beta/04_TASK_LEDGER.md`: 17/17 PASS, zero waivers), and every
acceptance guardrail re-verified from the candidate commit.

## Parent task dependency closure

| Parent | Scope | Gate evidence (Z packets) |
|---|---|---|
| BETA-001 | Real-world executable benchmark harness | Harness runs matched candidates from artifacts; raw retained |
| BETA-002 | Matched competitor candidates | Same-workload parity; honest horizons |
| BETA-003 | Controlled I/O and CPU/JIT crossover suites | Suites executable; samples retained |
| BETA-004 | Optional first-party Postgres capability | Reference capability with capability gating |
| BETA-005 | JWT auth reference package | RFC-vector-backed reference; no real credentials |
| BETA-006 | Beta observability baseline | Readiness/metrics routes; bounded |
| BETA-007 | Configuration and secret handling | Provenance-tagged config; redaction |
| BETA-008 | Reverse-proxy drain and deployment semantics | Loopback-first posture; graceful drain proven |
| BETA-009 | Beta security and reliability baseline | Conformance suites (SEC/RUN) pass |
| BETA-010 | Supported beta platform and packaging matrix | Linux x86_64 glibc declared and tested |
| BETA-011 | Automated beta publishing and versioning | `0.1.0-beta.1` versioning; npm dist-tag policy rehearsed (not executed — Owner-gated) |
| BETA-012 | Beta documentation and limitations | `docs/beta/` suite; KNOWN-LIMITATIONS shipped |
| BETA-013 | Beta soak and leak qualification | 2.43M requests, flat heaps, bounded RSS drift |
| BETA-014 | Canonical beta benchmark report | Honest medians from raw artifacts; no unsupported claims |
| BETA-015 | Beta release evidence, SBOM, checksums | Self-verifying packet (`CHECKSUMS-OK`), CycloneDX 277/277 licenses |
| BETA-016 | External clean-install and tutorial verification | Full external journey in a fresh container (A–F); verbatim tutorial; rollback proven |
| BETA-017 | Owner decisions | OD-BETA-001…006, 008 Accepted with records (`docs/beta/governance/OPEN_DECISIONS.md`) |

## Gate battery (from candidate commit `65da9ed`, clean tree)

- `bun test` — **434 pass / 0 fail** (67 files; in `unshare -rn` netns)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass (manifest hashes + internal links)
- `./scripts/verify` — **ALL PASS** (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE)
- `cargo test -p q-pack` / `-p velqu-runtime` — pass (40+2 / 37+3)

## Indexes and artifacts

- `REVIEW_INDEX.json` / `EVIDENCE_INDEX.json` — milestone
  `BETA-PUBLIC-BETA-RELEASE`; commit fields are rewritten to the exact
  packet HEAD by `./scripts/release-packet` (commit-bound; grep-verified
  at packet time).
- Gate artifacts produced by `./scripts/release-packet` from the clean
  candidate HEAD: commit-named source archive (`source-<short>.zip`),
  Git bundle (`velqu-<short>.bundle`, `git bundle verify` clean),
  release binaries (fail-closed), SBOM (CycloneDX 1.5, 277 components,
  277/277 license coverage), npm tarballs (9 packages), CHANGELOG,
  KNOWN-LIMITATIONS, and one top-level SHA-256 manifest verified with
  `sha256sum -c` (`CHECKSUMS-OK`).

## Raw-to-report parity and honesty checks

- Benchmark claims trace to retained raw samples
  (`benchmarks/raw/*/summary.json`); the canonical report carries
  medians computed from raw artifacts (BETA-014-D corrections) — cold
  start 2.29× vs class best at steady state, no raw-rust overtake
  claims in 100-request horizons.
- Normative targets and measured results are not conflated; no
  performance claim exists without matched evidence (AGENTS.md #12).

## Unresolved items — honest disclosure (none hidden, none waived)

1. **OD-BETA-007 Open** (first-party Postgres vs reference capability) —
   product-scope decision for post-beta; BETA-004 shipped the reference
   capability and BETA-017 judged the beta unblocked.
2. **OD-BETA-009 Open** (support channel/response expectations) —
   beta-docs adjacent; recorded in the governance ledger.
3. **PACK_FORMAT_CURRENT pinned to v1** — owner-authorized follow-up
   packet required before the v2 flip (carried from M26).
4. **M3-009 numeric 2-worker scaling target** — owner decision pending;
   measurements published, target not asserted.
5. **npm publication Owner-gated** — all `@velqu/*` packages remain
   private; dist-tag policy rehearsed but not executed.
6. **License fields Owner-gated** — workspace crates
   `UNLICENSED-BEFORE-OWNER-DECISION`; npm `NOASSERTION` (SBOM posture).
7. **Standing CI disclosure** — verify workflows stall with zero
   executed steps at PR creation since ~#714; local gates are the
   acceptance basis for every packet in this milestone.

None of the above is a P0/P1 beta-exit criterion; items 1–2 are
governance decisions tracked in `docs/beta/governance/OPEN_DECISIONS.md`,
items 3–6 are carried owner-gated items already recorded in
`REVIEW_INDEX.json` openItems, and item 7 is environmental.

## Required evidence checklist

- [x] Milestone report — this document
- [x] Review index — `REVIEW_INDEX.json` (commit-bound at packet time)
- [x] Evidence index — `EVIDENCE_INDEX.json` (commit-bound at packet time)
- [x] Commit-named source archive — `source-<short>.zip` via release packet
- [x] Git bundle — `velqu-<short>.bundle` via release packet
- [x] SHA-256 manifest — top-level `SHA256SUMS.txt`, verified (`CHECKSUMS-OK`)
