# Integrating This Task Pack into the Repository

Recommended location:

```text
docs/codex-spark-beta/
```

## Safe setup

```bash
unzip velqu-codex-spark-beta-microtasks-v1.zip
cp -R velqu-codex-spark-beta-microtasks-v1 docs/codex-spark-beta
```

Commit the task pack separately from runtime changes. After that, each agent task should produce its own atomic source commit.

## Do not do this

- Do not paste all 631 tasks into one model prompt.
- Do not let an agent mark future tasks complete because a parent summary claims completion.
- Do not replace the existing beta or production ledgers automatically; reconcile them through the G0 evidence tasks.
- Do not add generated benchmark data to an implementation commit unless the selected task requires it.
