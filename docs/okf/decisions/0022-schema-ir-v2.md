---
type: Architecture Decision Record
title: ADR-0022 Schema IR v2 — Declarative Transform, File, and Problem Nodes
status: accepted
date: 2026-08-21
implements: ADR-0018 (M2.5 schema JSON codecs track), ADR-0011 (RFC 9457 problems)
---

# ADR-0022: Schema IR v2 — Declarative Transform, File, and Problem Nodes

## Context

Schema IR v1 (11 node kinds) covers scalars, literals, enums, optional/default,
nullable, arrays, objects, and unions. Three product tracks need more:

- **Transforms**: M2.5+ JSON codecs need a declarative place to bind an input
  schema to an output schema by stable name, without executable callbacks
  crossing the Rust/JS boundary (constraint 4: no side effects during route
  discovery; constraint 7: lazy request data).
- **Files**: bounded multipart/binary payloads need metadata-only
  representation — content type and byte bound — with transport retaining
  ownership of bytes (constraint 11: all bodies bounded).
- **Problems**: ADR-0011 problems are typed values with declared statuses;
  response declarations need problem *schema metadata* (type URI, title,
  status, optional detail schema), not free-form JSON.

ADR-0009 established native Schema IR with explicit fallback. v2 extends the
same principle: nodes are closed, serializable, and deterministic; anything
executable, unbounded, or nondeterministic is not representable.

## Decision

1. **Version boundary.** `SCHEMA_IR_VERSION = 2` in both `q-schema-runtime`
   (Rust) and `@velqu/schema` (TypeScript), and `schemaIrVersion: 2` in every
   emitted pack. `q-pack` rejects packs whose `schemaIrVersion != 2` with a
   typed rejection before any schema is used (fail closed). There is no
   v1→v2 silent reinterpretation: v1 packs are rejected, not adapted. A
   compatibility adapter is explicitly deferred to M25-001-B, which owns
   compatibility/fallback markers.

2. **New nodes (serde tag `kind`, camelCase fields, absent options omitted):**
   - `transform`: `{ input: SchemaIr, output: SchemaIr, name: string }` —
     declarative pairing only. `name` matches `[A-Za-z0-9_.:-]{1,64}`.
     No executable callbacks, closures, or function references are
     representable in the IR or accepted by builders.
   - `file`: `{ contentType?: string, maxBytes: u64 }` — metadata only.
     Builders require `maxBytes` to be a safe integer in `[1, 16777216]`
     (16 MiB) and `contentType` in 1..128 characters. The IR never owns
     streams or performs I/O.
   - `problem`: `{ typeUri?: string, title: string, status: u16, detail?: SchemaIr }` —
     RFC 9457 metadata. Builders require `title` 1..128, `status` in
     `[400, 599]`, `typeUri` ≤ 2048.

3. **Literal domain.** `LiteralValue` widens from `string | number | boolean`
   to the bounded recursive JSON literal domain (`null`, scalars, arrays,
   string-keyed objects) matching the runtime's `serde_json::Value` literals.

4. **Runtime semantics are closed-set.** Validation against `transform`,
   `file`, or `problem` nodes returns a typed `unsupported` field error
   ("schema node requires a specialized codec") until M25-002+ ships the
   codecs. Nodes are never silently accepted as `any`. Fuzz corpus extended
   to all v2 nodes.

5. **Validation fixes carried by v2 tests** (behavior bugs in v1 semantics
   that v2's null/domain tests exposed):
   - A present-but-`null` value for a non-nullable, non-optional object
     member is now a `type` field error (previously inserted unvalidated).
     `nullable` members accept `null`; `optional` members fall back to their
     declared default.
   - Array item validation propagates the source coercion flag: query
     arrays of integers coerce `"7"` → `7`; body arrays do not coerce
     (previously body rules were force-applied to query arrays).

6. **Projections carry v2 metadata without inventing runtime behavior.**
   `tsTypeOfIr`: `transform` → its output type; `file` → `Uint8Array`;
   `problem` → `{ title: string; status: number; detail?: T }`. OpenAPI:
   `file` → `{ type: string, format: binary, x-maxBytes, x-contentType? }`;
   `problem` → object with `x-problem` metadata; `transform` → its output
   schema; `union` (previously emitted invalid `type: "union"`) → `oneOf`.

7. **Canonical hashing is unchanged.** Field order of new nodes matches the
   Rust declaration order in both builders and compiler extraction so
   `routes_canonical_sha256()` parity holds. Canonical ordering/algorithm
   changes remain M25-001-C's scope; M25-001-A ships structural parity
   fixtures only.

## Consequences

- All packs must be recompiled with `schemaIrVersion: 2` (benchmark fixture
  builder updated in lockstep). Old packs fail closed at load.
- Unsupported-transform documentation (how handlers declare codecs for
  transform names) is M25-001-D's scope.
- Golden corpus (`conformance/schema/golden/`) is the cross-language wire
  reference for every v2 node.

## Evidence

- Rust: `cargo test -p q-schema-runtime` — 17 lib tests + 2 fuzz tests,
  including serde round-trips for every v2 node, typed `unsupported` errors,
  null-member rejection, and array coercion propagation.
- TypeScript: `bun test conformance/schema/` — 15 tests covering version
  constant, all v2 builders, bounds rejection, canonical field order, and
  nested composition.
