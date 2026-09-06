# Browser Handler-Bundle ABI (BWASM-R-003)

Status: **Frozen contract v1** (ADR-0037 §3/§7; implementation:
`packages/browser-runtime/src/handler-bundle.ts`).

## 1. Identity and versioning

- `HANDLER_ABI_VERSION = 1`, versioned **independently of package
  versions**.
- A bundle carries its ABI version in metadata (`handlerAbiVersion`).
- A runtime that does not implement the bundle's version fails closed
  at registration with an actionable diagnostic naming both versions
  and the remedy ("rebuild the bundle with a matching
  @velqu/browser-runtime").

## 2. Registration (narrow API, no ambient globals)

Generated handler modules register through exactly one entry point:

```ts
defineBrowserHandlers(registrations, packExpectation): HandlerTable
```

- `registrations`: one per handler — `{ handlerKey, statuses, invoke,
  source? }`.
- `packExpectation`: the verified pack's function manifest and declared
  response statuses (`requiredHandlerKeys`, `declaredStatuses`).
- There is no ambient global registry: an undeclared route cannot be
  registered because validation rejects every key the pack does not
  declare, and every declared key must be present.

### Validation order (all before any execution)

1. ABI version match.
2. Registration well-formedness (`handlerKey: string`, `invoke:
   function`).
3. Duplicate handler keys → `DUPLICATE_HANDLER_KEY`.
4. Missing pack-declared keys → `MISSING_HANDLER` (names every gap).
5. Keys absent from the pack → `INVALID_REGISTRATION` (no silent
   undeclared routes).
6. Any registered status outside the pack's declared set for that
   handler → `UNDECLARED_STATUS` (naming the status and the declared
   set).

## 3. Invocation contract

`HandlerContext` (kernel plan data only — ADR-0037 §3; no raw request
handle, no ambient authority; capability calls cross the runtime
bridge, ADR-0038 §5):

```ts
{
  routeId: number;        // dense pack route identity
  handlerKey: string;
  params | query | headers: Record<string, unknown> | null;  // validated values
  body: unknown;          // validated body value
  bodyText: string | null; // raw text when the route declares no body schema
  deadlineMs: number;
}
```

`HandlerResult` — exactly one of:

```ts
{ kind: "response", status, headers?, body? }   // status MUST be declared
{ kind: "problem", problemId, detail?, errors? } // native registry ids
```

The kernel re-validates completions (declared-status enforcement and
response-schema validation run kernel-side — R-002); the bundle cannot
widen the contract at runtime.

## 4. Metadata emission (deterministic)

`emitHandlerBundleMetadata(registrations, abiVersion)` produces the
bundle metadata JSON:

- handlers sorted by `handlerKey` (codepoint ascending);
- `statuses` sorted ascending (duplicates preserved — emission is
  faithful, deduplication is registration's job);
- fixed key order (`handlerAbiVersion`, `handlers[]` with
  `handlerKey`, `statuses`, optional `source`);
- 2-space indent, LF, trailing newline;
- **byte-stable**: identical registrations produce identical bytes
  (reproducibility fixture + hash pinned in evidence).

## 5. Source locations

- `source` fields are project-relative paths.
- `sanitizeSourceLocation` strips host-absolute prefixes (any path
  segments before `src/`, `packages/`, `examples/`) and normalizes
  separators — private host paths never cross into runtime errors or
  metadata.

## 6. Relationship to later packets

- BWASM-B-001 (compiler target) generates bundles that call
  `defineBrowserHandlers` from application source.
- BWASM-R-004 executes registered handlers in an isolated Worker and
  enforces `deadlineMs` (kernel-side termination model).
- QuickJS bytecode never loads in the browser profile
  (`BytecodePolicy::Skip`, ADR-0037 §8) — bundles are source-form JS.
