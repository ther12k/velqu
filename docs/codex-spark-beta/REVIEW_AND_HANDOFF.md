# Review and Handoff Rules

Every microtask ends with one atomic commit and a concise handoff.

Required fields:

- task ID;
- commit hash;
- changed files;
- behavior implemented;
- tests executed and exact results;
- evidence created;
- remaining risk or `none`;
- next dependency-ready task.

A verification task may fix only defects necessary to satisfy its parent acceptance criteria. A gate task should not silently implement missing work; it should fail the gate and name the missing task.
