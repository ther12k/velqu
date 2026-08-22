# Unsupported Transformations — Failure Modes and Forward Mapping

Status: normative for Schema IR v2 (M25-001-D; see ADR-0022, ADR-0023).
This document enumerates every transformation class that the native schema
runtime cannot yet — or can never — execute, the exact typed failure each
layer produces, the sanctioned explicit-fallback escape hatch (ADR-0009), and
how each class gains a codec in the authorized M25 sequence (ADR-0018/0020).

One rule governs everything here: **no silent downgrade.** A construct either
has a native codec, fails with a typed problem, or is explicitly marked as
fallback with a visible reason.

## 1. Transformation classes

| Class | Representable in IR v2? | Native codec | Behavior today | Codec lands |
| --- | --- | --- | --- | --- |
| Declarative input→output pairing (`s.transform(input, output, name)`) | Yes — `transform` node | None yet | Typed `unsupported` field error at validation; usable in projections (types/OpenAPI use `output`) | M25-002 (strategy benchmark) → M25-003/M25-004 (generated decoders) → M25-005 (encoders) |
| File payload validation (`s_file`) | Yes — `file` node (metadata only) | None yet | Typed `unsupported` field error; OpenAPI projects `format: binary` + bounds | M25-004 (body decoders) |
| Problem schema validation (`s_problem`) | Yes — `problem` node (RFC 9457 metadata) | None yet | Typed `unsupported` field error; projections carry `x-problem` metadata | M25-006 (problem encoders) |
| Executable callbacks / refinements (`(v) => boolean` etc.) | **No** — not representable | Never native | Builder signatures reject them (declarative metadata only); a schema value carrying a function is not JSON-serializable and fails pack verification | Explicit `s_fallback` instead |
| Arbitrary classes / `instanceof` shapes | **No** | Never native | Same as above — not representable | Explicit `s_fallback` with `inner` best-effort shape |
| Unbounded or nondeterministic transforms (clock, random, I/O) | **No** — violates constraint 11 (bounded) and determinism | Never | Not representable; compiler diagnostics reject non-literal schema options | Redesign the boundary (out of scope) |
| Unrestricted regex patterns | **No** — runtime enforces a tiny anchored subset (`^usr_[0-9]+$`-style) | Subset only | `pattern` failures are typed `pattern` problems | Broader bounded matcher only with evidence (not scheduled) |

## 2. Failure modes by layer

Every layer fails **closed** with the same error identity so a schema author
sees one vocabulary end to end.

| Layer | Failure | Identity |
| --- | --- | --- |
| `@velqu/schema` builder | Reason outside `FALLBACK_REASONS`; unbounded transform name/file/problem options | `throw` with the bound in the message (`s_transform: name must match …`) |
| Compiler extraction (`extract.ts`) | Same bounds violations, non-literal arguments, unknown builder names | `CompileError` with `{file, line, column}` and an actionable hint (e.g. the closed reason list) |
| Pack load (`q-pack::verify`) | `schemaIrVersion` mismatch; declared `features` ≠ derived; fallback reason outside the vocabulary | `PackError::Rejected` — startup never reaches ready |
| Runtime validation (`q-schema-runtime`) | Value meets a `transform`/`file`/`problem` node; fallback marker without `inner`; unknown fallback reason | Field problems: code `unsupported`, `fallback`, or `invalid-schema` (RFC 9457 problem, 422 — never a 500) |
| Runtime validation (`q-schema-runtime`) | Schema-driven recursion deeper than `MAX_VALIDATE_DEPTH` (both the reference validator and the direct decoder programs, M25-004-C) | Field problem: code `depth` (RFC 9457 problem, 422) — stacks stay bounded per constraint 11 |

## 3. The explicit fallback marker

`s_fallback(reason, inner?)` (ADR-0009; M25-001-B) is the only sanctioned way
to route a construct to the generic path. The reason registry is closed and
mirrored in code (`FALLBACK_REASONS` in `@velqu/schema`,
`q_schema_runtime::FALLBACK_REASONS`):

| Reason | Meaning | Who sets it |
| --- | --- | --- |
| `unsupported-transform` | A transform name has no native codec yet | Developer (now); codec selector (M25-002-D) |
| `unrepresentable` | The construct is outside the native IR by design | Developer |
| `measured` | Benchmark evidence selected the generic path (M25-002-D / M25-005-D) | Compiler from evidence tables |
| `explicit` | Developer forces the generic path | Developer |

Semantics today: with `inner`, native validation applies the best-effort shape
(the marker is transparent); without `inner`, validation fails closed with a
typed `fallback` problem until the generic codec path lands (M25-004-B).
Reasons are load-time verified; every fallback usage is visible in the schema
manifest `features` tags and the OpenAPI `x-fallback` extension.

## 4. Forward mapping (authorized sequence)

- **M25-002** measures QuickJS parse/stringify vs generic Rust conversion vs
  generated codecs; the decision tables this spec's `measured` reason feeds.
- **M25-003/M25-004** generate direct params/query/header and JSON body
  decoders; unsupported transformations retain the QuickJS generic fallback
  (M25-004-B) bounded and deadline-aware (M25-007-C).
- **M25-005** generates per-status encoders; QuickJS stringify remains when
  measured better (M25-005-D).
- **M25-006** generates RFC 9457 problem encoders — the `problem` node gains
  its codec.
- **M25-007-A** tags fallback reason per route in the RoutePlan;
  **M25-007-D** surfaces codec choice and bridge crossings in `velqu inspect`.

Nothing in this document authorizes building those packets early; it defines
the contract they will satisfy.

## 5. How handlers relate to transform names

Transform nodes carry **no executable payload**; handlers keep executing in
QuickJS as always. A `transform` node declares the input/output contract for
a (future) native codec identified by `name`; when M25-002+ selects a codec
strategy, the same name binds the generated decoder/encoder to the route
binding. Until then the declaration is inert at runtime (typed `unsupported`
on the validation path) but fully live in every projection (types, OpenAPI,
Treaty, contract lock, semantic diff).
