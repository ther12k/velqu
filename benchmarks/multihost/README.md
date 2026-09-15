# Multi-host benchmark comparison kit (1-2 CPU containers)

Descriptive cross-environment comparison of ALL fixture candidates —
`velqu`, `raw-rust`, `raw-bun`, `elysia2` (Elysia on Bun), `lugas`
(LugasJS on Bun) — on whatever hardware you point it at (workstation,
Halotec server, Oracle VM), inside a CPU-limited container.

**This lane is descriptive evidence only (AGENTS constraint 12).** Hosts
differ in CPU, kernel, and virtualization; results are comparable across
candidates *within* a host and across hosts only as separate environments.
Nothing here feeds gate thresholds or ledger claims.

## What is pinned

- Builder base: the same digest-pinned `ubuntu:24.04` as the
  ga-native-repro lane, with the same apt pins (`gcc-13`, `binutils`,
  `libc6-dev`) — same-arch hosts build byte-identical velqu binaries.
- Bun `1.4.0`, rustc `1.96.0` (both recorded per run).
- Protocol: seed `20260915`, 10s cells, 5 repetitions, c=1/10/50, all
  candidates interleaved in one harness process (drift hits everyone
  equally).
- Contract: every candidate must pass the 27-assertion fixture checker
  (`benchmarks/harness/check-server.ts`) inside the container BEFORE any
  timing; a checker failure aborts the run.

## Run on a host

```bash
# docker (or podman's docker shim) required on the host; ~10-40 min build
# (release Rust build, single-threaded on 1-core hosts), then a ~20 min
# measured run at 5 candidates x 4 routes x 3 concurrency x 5 reps
benchmarks/multihost/run.sh local 2     # or halotec / oracle
# busy or tiny hosts: cap the build's parallelism —
BENCH_BUILD_ARGS="--build-arg BENCH_CARGO_JOBS=2" benchmarks/multihost/run.sh halotec 2
# podman-only host without the docker shim:
DOCKER=podman benchmarks/multihost/run.sh oracle 2
```

The build context is a clean `git archive HEAD` export (the tree itself
when transferred as an archive), so committed raw evidence never enters
the image and every host builds from identical inputs.

Artifacts: `benchmarks/raw/multihost/<run-id>/` — raw JSONL, summary,
`hashes-<label>.txt` (image digest, toolchain versions, CPU, binary
SHA-256s), `host-<label>.txt` (docker/kernel/virt identity). Copy that
directory back from remote hosts, then:

```bash
python3 benchmarks/multihost/aggregate.py   # -> benchmarks/raw/multihost/COMPARISON.md
```

## Notes

- Oracle VMs are frequently aarch64 — the image builds natively per arch;
  never compare numbers across architectures as one table.
- Virtualized hosts: check `steal` time (`vmstat 1`) while running;
  a noisy neighbor invalidates the run — rerun rather than explain.
- Local-workstation runs must not overlap the 72h soak (contention); run
  remote hosts any time, the workstation after the soak completes.
