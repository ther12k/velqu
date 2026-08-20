# QuickJS Engine Context

Scheduler correctness is already heavily hardened: owner-scoped deadlines, microtask checkpoints, task cancellation, quarantine, readiness, and response-mapping budgets. Do not reopen it without a reproduced regression. New work must preserve queue-empty-or-quarantined message boundaries and physical task accounting.
