# Velqu Low-Context Beta Microtask Pack

This package converts the beta roadmap into atomic Markdown work packets for a small-context coding agent such as Codex Spark.

## Baseline

- Reviewed implementation: `4e6904951729ea14b48ca39a9564a950cc83e`.
- Evidence capture checkpoint: `03cc48955c2f8b05c29cf6ca196572c67ed5dd2d`.
- The final release commit and packet artifact names are bound by `scripts/release-packet` after the clean commit is fixed, then checksum-validated.
- G0 status is **PASS**; M2.4 remains the next dependency-ready implementation milestone.
- Every atomic packet and gate is registered as a private GitHub issue in [`ther12k/velqu`](https://github.com/ther12k/velqu), with priority, milestone, and mode labels.

## How to use

1. Read `START_HERE.md` once.
2. Pick the first dependency-ready task from `indexes/EXECUTION_QUEUE.md`.
3. Give the agent only:
   - `LOW_CONTEXT_AGENT_PROMPT.md`;
   - the selected task file;
   - the one milestone context card named by that task;
   - the source files listed in that task.
4. Require one atomic commit and the handoff format in the task file.
5. Mark the task in `STATUS.md` only after its tests and acceptance criteria pass.

Do **not** paste the entire task pack into one model context.
