---
type: Architecture Decision Record
title: ADR-0040 Benchmark Evidence Validation Environments
status: accepted
date: 2026-09-10
implements: "#1303 (benchmark harness drift), AGENTS.md constraint 12 (no performance claim without matched, reproducible evidence)"
owner-ratified: pending — recorded during #1303; the capture-host/CI split below restores the gate to a state that can actually pass in either environment; revisit if the owner wants stronger cross-machine provenance.
---

# ADR-0040: Benchmark Evidence Validation Environments

## Context

`scripts/validate-benchmark-evidence.py` validates the committed benchmark
manifest against the working tree. Two of its checks assumed an environment
that no longer describes where the gate runs:

1. **Capture-commit ancestry.** The check required
   `manifest.commit` to be a git ancestor of `HEAD`. The repo's delivery flow
   (AGENTS.md working rules) squash-merges each packet and deletes the branch:
   the pre-squash commit a manifest was captured at becomes unreachable once
   its ref is deleted. Post-merge checkouts — including master CI — therefore
   cannot resolve the capture commit at all, and shallow CI clones cannot
   resolve ancestry even for live branches. Master CI has failed on exactly
   this since the verify lanes began executing (run 34379032433:
   `capture commit 8aee413… is not an ancestor of HEAD 78772b6…`).

2. **Runtime binary byte identity.** The check hashed
   `target/release/velqu-runtime` and compared it to the manifest. The
   manifest hash is captured on the benchmark host; CI rebuilds the runtime
   with its own toolchain/linker, and byte identity across machines is not a
   contract of this repo (M26-007-D reproducibility is defined across
   independent builders on the same host). The check was structurally
   unpassable in CI for every commit, capture discipline notwithstanding.

A gate that cannot pass anywhere protects nothing; leaving it red erodes the
signal for real evidence breakage.

## Decision

`scripts/validate-benchmark-evidence.py` distinguishes what each environment
can prove:

- **Ancestry is checked strictly whenever git can answer it.** The capture
  commit must be a full 40-hex hash. If `git merge-base --is-ancestor` fails,
  the validator checks whether the commit object is reachable
  (`git rev-parse --verify <sha>^{commit}`): if present but not an ancestor,
  that is fatal (evidence captured from an unrelated history). If the object
  is absent — the squash-merge delivery case — ancestry is unprovable and is
  reported as a note, not an error. The verify workflow checks out with
  `fetch-depth: 0` so live-branch PR runs take the strict path.
- **Artifact hashes are checked byte-exact on the capture host** (the
  canonical local gate). In CI (`CI=true`), artifacts under `target/` —
  host-toolchain build outputs — are checked for presence only; byte identity
  remains the capture host's responsibility. Committed and
  deterministically-rebuilt artifacts (e.g. the proof pack, rebuilt by the
  verify lane itself) are byte-checked in both environments.
- Raw/summary parity, row-count, repetition, randomization, codec
  instrumentation, and startup-profile checks are unchanged and remain strict
  everywhere.

## Consequences

- Master CI can go green on the evidence lane without weakening any check
  that is decidable in that environment.
- Provenance of a manifest's capture commit relies on the commit having been
  pushed and observed while its ref lived (PR review), not on permanent git
  reachability. PR review of evidence packets is the compensating control.
- If cross-machine binary provenance becomes a requirement, the follow-up is
  a reproducible-toolchain container for capture and validation — a new ADR,
  not a silent change.
