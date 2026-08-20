# Stop Conditions

Stop and return a blocker instead of expanding scope when:

- a required dependency task is not complete;
- an owner decision is required;
- the current source differs materially from the task's stated baseline;
- a change would require reopening an accepted architecture invariant;
- external credentials/infrastructure are unavailable;
- the task requires a later milestone capability;
- tests reveal a P0 issue outside the selected task;
- the only way to pass is to weaken validation, skip a negative test, or hide evidence.

Use `templates/BLOCKER_TEMPLATE.md`.
