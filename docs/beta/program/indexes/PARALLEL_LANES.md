# Parallel Lanes

Default to sequential execution. Parallel work is safe only when dependencies and file ownership do not overlap.

## Allowed parallel lanes

- G0 benchmark-harness preparation may run beside G0 router/manifest implementation after the baseline task passes.
- Real-world benchmark infrastructure may run after G0 planning, but must not add Postgres runtime capability before M2.7.
- Documentation and examples may follow already accepted APIs; they must not define unstable behavior.
- Owner-decision preparation may run at any time, but only the owner can decide.
- Packaging design may begin after M2.6 design stabilizes; publishing waits for beta gates.

## Never parallelize blindly

- two tasks editing `crates/q-engine-quickjs/src/worker.rs`;
- router layout and M2.4 admission logic before G0 passes;
- schema IR and generated codecs without a frozen IR revision;
- capability ABI and a concrete Postgres implementation in the same unreviewed change;
- worker dispatch and quarantine/replacement changes in unrelated branches.

When uncertain, use the dependency-safe queue.
