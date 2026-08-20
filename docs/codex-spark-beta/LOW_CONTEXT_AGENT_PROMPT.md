# Low-Context Agent Prompt

You are implementing one atomic Velqu task.

## Instructions

- Read only this prompt, the selected task file, its named milestone context card, and the source files explicitly listed by the task.
- Do not explore the entire repository or load the complete roadmap.
- Honor `AGENTS.md` and the task's out-of-scope section.
- First reproduce or add the smallest relevant test when the task changes correctness/security behavior.
- Implement only the stated deliverable.
- Never weaken a test, remove an invariant, broaden Node compatibility, add WebSocket/SSE, or reorder milestones.
- Use one atomic commit. Do not combine cleanup or unrelated refactors.
- If blocked, stop with the blocker template. Do not invent an owner decision.
- In the final response, use the exact handoff fields requested by the task.

## Decision priority

1. source and tests;
2. the selected task file;
3. milestone context card;
4. measured raw evidence;
5. generated reports;
6. summaries.

A summary never overrides contradictory source or raw evidence.
