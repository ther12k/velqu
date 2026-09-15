# M6-006 — Native-Binary Reproducibility on Independent Builders

- Date: 2026-09-15
- Candidate: `b8fee34909c0d6a4b3b9f76aebe9665cf3192c99` (tested runtime identity)
- Lane: `ga-native-repro.yml` (manual dispatch), run
  [34938409719](https://github.com/ther12k/velqu/actions/runs/34938409719)
- Raw evidence: `benchmarks/raw/native-repro/b8fee349/` (hashes-a.txt,
  hashes-b.txt, capture-host.txt, run.txt)

## Contract

GA-RECONCILIATION M6-006 delta: "Reproducibility of the **native
binaries** (`velqu-runtime`) across two clean environments with pinned
toolchains", acceptance "reproduce or all allowed differences are
explained". QPack artifact reproducibility was already proven
(`scripts/compare-builds`, byte-identical across independent CLI builds,
every verify run); this packet supplies the native-binary half.

## Method

Two **separate GitHub-hosted runners** (different physical machines,
fresh caches, no shared state) each:

1. check out the pinned commit `b8fee349`;
2. install rustc **1.96.0** (pinned, `rust-toolchain.toml` honored);
3. build `velqu-runtime` and `velqu-bytecode` in release with the
   canonical reproducibility remaps (`RUSTFLAGS
   --remap-path-prefix=<repo>=/velqu-src`, `CFLAGS` prefix/debug maps —
   REBUILD.md §3);
4. record builder identity (uname, kernel, CPU, linker, rustc version)
   and artifact SHA-256s.

A compare job requires byte-exact hash equality across the two builders.

## Result

**REPRODUCIBLE.** Both independent builders produced byte-identical
binaries:

| Artifact | Builder A | Builder B |
|---|---|---|
| `velqu-runtime` | `b5cd700c0f56…8655b1c1c` | identical |
| `velqu-bytecode` | `616f8cd09102…ddfcd85fe1` | identical |

Builder identity (both): Linux 6.17.0-1022-azure x86_64, AMD EPYC 7763,
GNU ld 2.42, rustc 1.96.0 (ac68faa20). The lane exits green only on
byte equality; it did.

## Capture-host difference — explained

The workstation reference build of the same commit (rustc 1.96.0, same
remaps, GNU ld 2.42, Ubuntu 24.04) hashes `930357da…` ≠ the CI pair. The
workstation's environment uses a **user-space C toolchain** (gcc 12.2.0,
Debian .deb-extracted, assembled after an earlier environment rollback)
to compile the QuickJS C sources inside `rquickjs-sys`; the runners use
stock Ubuntu gcc 13.3.0. Same rustc, different C compiler → different
QuickJS objects → different final binary. This is an environment
difference with a specific cause, recorded in `capture-host.txt` per the
"all allowed differences are explained" clause — it is not a
reproducibility defect of the build system, and the two-clean-environment
qualification pair is CI-vs-CI.

## Scope

Evidence packet only. M6-006 stays TODO in the ledger: its dependency
chain (M6-005 and upstream blockers) governs promotion, per the corrected
closure sequence on #1343. The per-packet provenance machinery (M6-005,
#1360) records builder/toolchain identity for every future release
packet; rerunning this lane at the RC candidate renews the evidence.
