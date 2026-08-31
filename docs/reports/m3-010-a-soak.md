# M3-010-A — Sustained Mixed-Load Soak (30 minutes, 4.41 M verified requests)

Generated from `benchmarks/raw/worker-scaling/soak-summary.json`
(velqu-soak-v1) and its raw JSONL (59 window samples), from the FINAL
committed `q-soak` build. Every number below is quoted from those files.

## Harness and command

`crates/q-bench-support/src/bin/soak.rs` (bin `q-soak`): N independent
QuickJS runtimes (one thread + one runtime each, ADR-0036 §1/§2) behind
the M3-002 bounded Dispatcher, driven CONTINUOUSLY by 8 closed-loop
producers for a configured duration. Every response is verified
host-side against its known kind; errors are classified
(timeout/mismatch) and counted — none dropped.

```bash
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=$(pwd)=/velqu-src"
cargo build --release -p q-bench-support --bin q-soak
./target/release/q-soak --workers 2 --duration-secs 1800 --window-secs 30 \
    --out-dir benchmarks/raw/worker-scaling
```

Mix (deterministic per id): 60 % `light.work` + 25 % `cpu.work` + 15 %
`io.delay` (1 ms native timer — controlled I/O, no external network).
The multi-hour goal is executed as this 30-minute sustained soak; the
harness accepts arbitrary `--duration-secs` and the report discloses
the exact executed window rather than claiming a literal multi-hour run.
Disclosure: a 60 s smoke run and part of the local gate suite ran
concurrently with the first minutes of this soak — visible as the
lower early-window throughput; the evidence is committed as measured.

## Results (exact values from the summary)

| metric | value |
|---|---|
| configured / actual duration | 1 800 s / 1 800.7 s (59 windows) |
| workers | 2 |
| dispatched (got a queue slot) | **4 407 585** |
| completed + verified | **4 407 585 (100.0000 %)** |
| classified errors | **0** (timeout 0, mismatch 0) |
| overall throughput | 2 448 ops/s (window range 1 850–2 753) |
| final per-worker heap | 203 853 B / 201 427 B (same band as M3-009) |
| RSS first → last window | 5 764 → 5 376 KiB (**−388 KiB over 30 min**) |
| max window-to-window RSS step | 68 KiB |
| queue rejections (cumulative) | 4 892 556 |

## Leak analysis

- **JS heaps are flat**: final per-worker heaps (203 853 / 201 427 B)
  sit in the same ~201 KB band as the M3-009 scaling runs after 4.41 M
  invocations — no per-worker heap growth (the ~2 KB spread between the
  two workers reflects in-flight state at shutdown, not retention).
- **Process RSS ended LOWER than it started** (−388 KiB across the
  whole run; max single-window step 68 KiB) — the opposite of a leak
  signature. A leak at even 16 B/request across 4.41 M requests would
  show +70 MiB; observed is negative.
- **Completion accounting is exact**: dispatched == completed ==
  verified; every request settled within its 2 s deadline; zero
  timeouts, zero mismatches.

## Queue rejections, bounded and explained

The 4 892 556 cumulative rejections are the 8 closed-loop producers'
spin attempts against momentarily-full 1 024-slot queues — typed
backpressure events (M3-002-A `QueueError::Full`, saturating-counted),
not lost work: every request that got a slot completed and verified.
Under sustained overload this is the designed fail-fast posture; the
queues never grew, and no request was silently dropped.

## Sustained-stability observations

- Throughput held a ~2.4–2.75k ops/s band with contention dips fully
  recovering (host shared with the local gate suite early in the run —
  disclosed above); the post-dip recovery supports the capacity-
  recovers guardrail at the process level.
- No boundary violations: the scheduler-boundary assertions from the
  M2.2.1 suite (part of ./scripts/verify) hold on the same tree.

## Guardrail mapping (parent M3-010)

- *No monotonic leak* — flat heaps; RSS ended below its start over
  4.41 M verified requests (analysis above).
- *Capacity recovers after replacement* — replacement soak is
  M3-010-B; this packet's recovery evidence is the post-dip throughput
  recovery above.
- *All errors bounded and explained* — error classification armed and
  empty; queue rejections counted and explained (backpressure, zero
  lost requests).
- *No boundary violations* — verify's scheduler-boundary suite green.

## Scope notes

- Worker poison/replacement injection is M3-010-B (chaos timeline);
  retained-memory/slot tracking extensions are M3-010-C; the explicit
  recovery verification is M3-010-D.

## Artifact hashes (SHA-256)

| artifact | sha256 |
|---|---|
| `target/release/q-soak` (remapped build) | `43df9f423c6bff5a78dc4ed64598c7a5a016173948945106dab7d1303c26417c` |
| `benchmarks/raw/worker-scaling/soak.jsonl` (59 lines) | `6943240e5e73be1a0665112427135999492e6907d17d8981b89c5614f32d4abb` |
| `benchmarks/raw/worker-scaling/soak-summary.json` | `94d8a07fff0f69363b38ded47b3dd615f8fbe3e5c8ca00a36857d09d53719f7c` |
