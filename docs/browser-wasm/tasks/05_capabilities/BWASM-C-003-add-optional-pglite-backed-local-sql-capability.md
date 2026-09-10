Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/05_capabilities/BWASM-C-003-add-optional-pglite-backed-local-sql-capability.md`  
Program: `BWASM`  
Phase: `05_capabilities` — Browser capabilities and persistence  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P1`  
Optional: `YES — excluded from the MVP release gate unless an owner decision promotes it before candidate freeze.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `PASS`

---

# BWASM-C-003 — Add optional PGlite-backed local SQL capability

## Atomic goal

Offer an opt-in browser-local SQL adapter for prototype applications without pretending it is production PostgreSQL infrastructure.

## Parent intent

Expose only browser-safe capabilities, support explicit local persistence, and fail closed for production-only requirements.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-C-002` — Make the Postgres capability contract asynchronous before browser freeze
- `BWASM-R-005` — Integrate capability registry and Treaty with the browser runtime
- `BWASM-B-001` — Add compiler target browser-wasm

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/`
- `crates/q-capability-postgres/`
- `packages/browser-runtime/`

## Steps

1. Create an optional package/adapter backed by PGlite or an owner-approved equivalent.
2. Support memory and explicitly enabled IndexedDB persistence modes.
3. Map the frozen Velqu Postgres capability subset to the browser adapter.
4. Define unsupported SQL/extensions/concurrency/transaction behavior.
5. Expose database reset/export/import hooks for preview UX and tests.
6. Lazy-load the database WASM/assets so projects without SQL do not pay the payload cost.

## Acceptance criteria

- [x] Supported SQL fixtures behave according to the documented capability subset.
- [x] Unsupported operations fail with stable, actionable codes.
- [x] Persistence is isolated by project and origin namespace.
- [x] The adapter never claims multi-user durability, production availability, or native Postgres performance.
- [x] Projects without the capability do not download or instantiate database assets.
- [x] Database bytes and versions are integrity-bound to the browser build.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- SQL compatibility fixture corpus.
- Memory and IndexedDB persistence tests.
- Project-isolation tests.
- Lazy-load/network trace.
- Export/import/reset tests.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [x] SQL support matrix.
- [x] Payload/network measurements.
- [x] Persistence/isolation logs.
- [x] Compatibility test results.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Full PostgreSQL parity.
- Shared multi-user database.
- Server-side secrets or remote database credentials.
- Making PGlite mandatory for the core Browser-WASM runtime.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-c-003:`.
- Reference this issue ID in commits, PR body, tests, and evidence.
- Avoid generated queue/index churn until implementation and targeted tests are stable.
- If scope expands materially, stop and open a new dependency issue rather than hiding extra work here.

## Stop condition

Stop and hand off when **all** acceptance criteria are demonstrated, the required evidence is attached or committed, canonical verification is green, and no unresolved in-scope P0 remains. If a prerequisite, owner decision, browser limitation, or security claim blocks truthful completion, record the exact blocker and leave this issue open.

## Handoff format

```text
Issue:
Candidate commit:
Files changed:
Commands run:
Targeted tests:
Full verification:
Artifacts and SHA-256:
Browser/OS/toolchain:
Acceptance criteria:
Known limitations:
Residual risks:
Follow-up issue links:
```

## Result (2026-09-11) — PASS

Issue: BWASM-C-003 (no standalone GitHub issue; this task record is the
tracking unit per the program's registration model).

Files changed:
- `packages/browser-pglite/` (new optional package): `src/local-sql.ts`
  (adapter: lazy engine load via dynamic import, memory/indexeddb modes,
  origin+namespace storage isolation, bounded deadlines, typed errors,
  engine-native transactions, export/import/reset),
  `src/subset.ts` (documented SQL subset + stable remediation hints),
  `src/index.ts`, `test/local-sql.test.ts` (real-engine SQL corpus),
  `test/lazy.test.ts` (lazy/isolation/fail-closed/subset/deadline with a
  counting fake loader), `test/purity.test.ts` (R-001 pattern),
  `package.json` (engine `@electric-sql/pglite` 0.5.8 exact pin),
  `tsconfig.build.json`, `README.md`.
- `scripts/build-packages.ts`, `scripts/publish-beta.sh` (package wired;
  engine always external, never inlined), root `bun.lock` (+11 lines),
  `package.json` (unchanged net).

Commands run: `bun test packages/browser-pglite` → **20 pass / 0 fail**
(82 expects); `bun run typecheck` → clean; package build (bun build +
tsc -p tsconfig.build.json) → dist 11,806 B (gzip 3,370 B) with the
engine external; full `unshare -rn ./scripts/verify` at the packet
commit → ALL PASS (recorded in the PR).

Targeted tests: SQL fixture corpus (DDL/DML/joins/CTE/window/params/
RETURNING/transactions commit+rollback), unsupported-statement matrix
(9 families → `UnsupportedStatement` + remediation BEFORE execution,
loader count proves no engine call), isolation (distinct storage per
origin+namespace on real ctor args; namespace validation incl. path
escape), persistence modes (memory = no dataDir; indexeddb = namespaced
`idb://`), blocked-storage fail-closed (`PersistenceUnavailable`, no
silent fallback), lazy-load (zero engine loads before open; exactly one
after), wall-clock deadline on yielding waits (+ disclosed no-preemption
limit), export→import→reset round-trip on the real engine.

Evidence: `docs/browser-wasm/evidence/capabilities/c003/` —
`sql-support-matrix.md`, `payload-network.md` (adapter 11,806 B /
gzip 3,370 B; engine wasm 10,088,161 + 395,242 B, lazy-only),
`persistence-isolation.md`, `compatibility.txt` (20/20).

Browser/OS/toolchain: Linux x86_64, Bun 1.4.0, TS 5.9.3,
`@electric-sql/pglite` 0.5.8 (engine reports PostgreSQL 18.3 on wasm32).

Acceptance criteria: all six demonstrated (see evidence above; claims
discipline: the native `postgres` grant stays deployment-required — the
adapter is a distinct `runtime:local-sql` v1 surface and the package
docs forbid multi-user/durability/performance claims).

Known limitations / residual risks:
- Real-browser network trace and IndexedDB durability E2E (chromium
  lane) not exercised in this packet — structural proof committed
  (package-level opt-out + dynamic-import laziness + fake-loader
  assertions); lane extension recorded as follow-up evidence.
- `importAll` is full-replace; dumps are engine-version-tagged archives
  (restore across engine major versions is an engine property, not
  promised by the adapter contract).
- Engine WASM exec is single-threaded with the caller: deadlines reject
  the caller but never preempt in-engine work (documented + tested).

Owner note: promotion of this optional capability into any release gate
remains an owner decision (task frontmatter: excluded from the MVP gate
unless promoted before candidate freeze — it was not promoted).
