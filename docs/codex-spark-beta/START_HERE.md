# Start Here

## One-task rule

Work on exactly one task file at a time. Do not read sibling tasks unless the current task explicitly references them.

## Minimum context to provide the agent

1. `LOW_CONTEXT_AGENT_PROMPT.md`
2. the selected `tasks/.../<task>.md`
3. the task's milestone card under `context/milestones/`
4. only the source paths listed under **Read these source files**

## Required preflight

```bash
git status --short
git rev-parse HEAD
```

Stop if the tree is unexpectedly dirty or the task's dependencies are not complete.

## Required finish

- targeted tests pass;
- no unrelated changes;
- one atomic commit;
- final response follows `templates/TASK_RESULT_TEMPLATE.md`;
- do not claim the parent task or milestone is complete unless the selected task is its verification/evidence/gate packet.

## Architecture rule

Never change the core architecture merely to make a microtask easier. Rust + QuickJS-NG, compiler-without-dry-run, bounded resources, typed contracts, and the ordered ADR-0018/ADR-0020 roadmap remain in force.
