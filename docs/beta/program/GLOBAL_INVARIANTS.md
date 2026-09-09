# Global Invariants

1. Production execution is Rust + QuickJS-NG; Bun is tooling only.
2. Rust routes before JavaScript.
3. Compiler route discovery never runs application services or side effects.
4. Production startup performs no TypeScript, schema, OpenAPI, plugin, or route compilation.
5. All queues, jobs, bodies, memory, stacks, handles, and deadlines are bounded.
6. Native handles are opaque, generation-checked, and invalid after settlement.
7. Same-process QuickJS runs trusted application code only.
8. Expected HTTP failures are typed declared values; unexpected errors are redacted.
9. One contract graph must project to runtime validation, Treaty, OpenAPI, and contract lock.
10. Current milestone representations cannot be trusted as security controls until their upstream gate passes.
11. No performance claim without matched reproducible raw evidence.
12. Do not add full Node/Bun compatibility, CommonJS, WebSocket, SSE, ORM-in-core, or cloud provisioning before beta.
13. Every P0 correction must have negative tests and fail closed.
14. A task is complete only when source, tests, evidence, and commit agree.
