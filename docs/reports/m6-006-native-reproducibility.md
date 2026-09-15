# M6-006 — Native-Binary Reproducibility on Independent Builders

- Date: 2026-09-15
- Candidate: `b8fee34909c0d6a4b3b9f76aebe9665cf3192c99` (tested runtime identity)
- Lane: `ga-native-repro.yml` (manual dispatch)
  - pinned-environment run (canonical):
    [34964344973](https://github.com/ther12k/velqu/actions/runs/34964344973)
  - earlier runner-VM run (superseded, retained):
    [34938409719](https://github.com/ther12k/velqu/actions/runs/34938409719)
- Raw evidence: `benchmarks/raw/native-repro/b8fee349/` (VM pair: hashes-a.txt,
  hashes-b.txt, capture-host.txt, run.txt) and `pinned/` (hashes-a.txt,
  hashes-b.txt, run.txt — the canonical pair)

## Contract

GA-RECONCILIATION M6-006 delta: "Reproducibility of the **native
binaries** (`velqu-runtime`) across two clean environments with pinned
toolchains", acceptance "reproduce or all allowed differences are
explained" and "Build images/toolchains are pinned". QPack artifact
reproducibility was already proven (`scripts/compare-builds`,
byte-identical across independent CLI builds, every verify run); this
packet supplies the native-binary half.

## Method

Two **separate GitHub-hosted runners** (different physical machines,
fresh caches, no shared state) each build the same pinned commit inside
the **same pinned builder environment**, then record full identity and
artifact SHA-256s; a compare job requires byte-exact hash equality.

Pinned environment (first run was on the runner VM image with only rustc
pinned; the lane now pins the complete native environment — the VM
image's C toolchain state was whatever the day's image carried, which
does not meet the "Build images/toolchains are pinned" acceptance):

1. build inside a **digest-pinned `ubuntu:24.04` container**
   (`sha256:224a1869083a311ef3f13648a154ba79832fbef6364d31493642ca03082da254`);
2. install the C toolchain at **exact apt versions**: `gcc-13
   13.3.0-6ubuntu2~24.04.1`, `binutils 2.42-4ubuntu2.10`, `libc6-dev
   2.39-0ubuntu8.9`;
3. check out the pinned commit `b8fee349`;
4. install rustc **1.96.0** (pinned);
5. build `velqu-runtime` and `velqu-bytecode` in release with the
   canonical reproducibility remaps (`RUSTFLAGS
   --remap-path-prefix=<repo>=/velqu-src`, `CFLAGS` prefix/debug maps —
   REBUILD.md §3);
6. record builder identity (builder image digest, gcc/cc, rustc, glibc,
   linker, uname, kernel, CPU) and artifact SHA-256s.

## Result (pinned environment — canonical)

**REPRODUCIBLE.** Both independent builders — genuinely different hosts
(Intel Xeon Platinum 8370C vs AMD EPYC 7763) — produced byte-identical
binaries:

| Artifact | Builder A | Builder B |
|---|---|---|
| `velqu-runtime` | `75c0120d5557…9bf669` | identical |
| `velqu-bytecode` | `5fe55267b9d3…85fe1` | identical |

Identity (both, from `pinned/hashes-*.txt`): builder container
`ubuntu:24.04@sha256:224a1869…`, gcc/cc 13.3.0
(`13.3.0-6ubuntu2~24.04.1`), GNU ld 2.42, glibc 2.39-0ubuntu8.9, rustc
1.96.0 (ac68faa20). The lane exits green only on byte equality; it did.

The earlier VM-image run (34938409719) also produced byte-identical a-vs-b
binaries (`b5cd700c…`/`616f8cd0…`) but its hashes **differ from the pinned
pair** — expected, and itself demonstrative: the two runs differ in
environment components that reach output bytes (the day's VM image state
vs the pinned container + exact deb versions). Each pair is internally
reproducible; only the pinned pair satisfies the pinning acceptance, and
it is the canonical evidence going forward.

## Capture-host difference — explained

The workstation reference build of the same commit (rustc 1.96.0, same
remaps, GNU ld 2.42, Ubuntu 24.04) hashes `930357da…` ≠ the pinned CI
pair (`75c0120d…`; the compare job records the parity line explicitly).
The workstation's environment uses a **user-space C toolchain** (gcc
12.2.0, Debian .deb-extracted, assembled after an earlier environment
rollback) to compile the QuickJS C sources inside `rquickjs-sys`; the
pinned lane uses gcc 13.3.0. Same rustc, different C compiler → different
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
