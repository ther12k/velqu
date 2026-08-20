# Packet Workflow — Branch, PR, Merge, Worktree

This is the required delivery process for every Codex Spark packet (and any
other change). It replaces direct commits to `master`.

## Per-packet flow

1. **Branch** — one branch per packet, named after the packet ID:
   ```bash
   git checkout master && git pull
   git checkout -b m24-001-b
   ```
2. **Do the work** — follow the packet file: read only its listed sources,
   smallest change, targeted tests green, completion record appended to the
   packet, `STATUS.md` checkbox marked.
3. **Push and open a PR**:
   ```bash
   git push -u origin m24-001-b
   gh pr create --title "m24-001-b: <short>" --body "Closes #61"
   ```
   The PR body must contain `Closes #<issue-number>` so merging closes the
   registered issue automatically, and must list the verification commands run.
4. **Merge** — squash-merge to keep one atomic commit per packet PR:
   ```bash
   gh pr merge --squash --delete-branch
   git checkout master && git pull
   ```
5. **Advance the queue** — after merge, regenerate `STATUS.md` ordering,
   `indexes/EXECUTION_QUEUE.md`, and `indexes/NEXT_25.md` from unchecked
   packet statuses (the completion commit inside the PR may do this).

PR merges are gated by the same rule as commits: targeted tests pass, docs
validators pass, no weakened assertions. `./scripts/verify` ALL PASS is
required for anything touching code, benchmarks, or evidence.

## Parallel work with git worktrees

Multiple packets can be worked simultaneously without dirtying each other's
trees. Worktrees live outside the repository:

```bash
mkdir -p ../velqu-worktrees
git worktree add ../velqu-worktrees/m24-001-b -b m24-001-b
```

Each worktree is an independent checkout: one packet per worktree, one branch
per worktree. The main checkout stays clean, which also keeps
`scripts/release-packet` (clean-tree requirement) usable at any time.

Rules:

- Pick packets whose dependencies are already merged — the head of
  `indexes/EXECUTION_QUEUE.md` and its unblocked siblings are safe splits.
- Never share a branch between worktrees.
- Each PR is reviewed/merged independently; regenerate the indexes after each
  merge (in master, not in the packet branch).
- Remove finished worktrees with `git worktree remove` and prune.

## Issue hygiene

- Every packet has a registered private issue in `ther12k/velqu`.
- The PR body's `Closes #N` is the only supported way to close packet issues —
  no manual `gh issue close` in the normal flow.
- Completion evidence lives in the merged commit (packet completion record),
  not in the issue; the issue comment only points at the commit.
