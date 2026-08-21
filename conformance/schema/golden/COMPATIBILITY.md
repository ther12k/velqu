# Schema IR v2 — Golden Corpus and Compatibility Matrix

M25-001-A evidence. The JSON files in this directory are the canonical wire
nodes shared by `@velqu/schema` (TypeScript) and `q-schema-runtime` (Rust).
Both sides must deserialize, re-serialize byte-identically (camelCase `kind`
tags, absent options omitted), and classify validation without panicking.

Corpus files:

| File | Covers |
| --- | --- |
| `transform.json` | transform node, bounded name, nested input/output constraints |
| `file.json` | file node, absent `contentType` omitted |
| `file-content-type.json` | file node with `contentType` |
| `problem.json` | full problem node (typeUri + detail schema) |
| `problem-minimal.json` | problem node with required fields only |
| `nested-composition.json` | every v1+v2 kind in one object; optional/default, nullable, formats, pattern, bounds |

Round-trip tests: `conformance/schema/golden.conformance.test.ts` (builders)
and `m25_001_a_tests::golden_corpus_round_trips` in `q-schema-runtime`.

## Compatibility matrix

| Scenario | Behavior (owner) |
| --- | --- |
| Pack declares `schemaIrVersion: 2`, contains v1+v2 nodes | Accepted (M25-001-A, `q-pack`) |
| Pack declares `schemaIrVersion: 1` | **Rejected** at load with typed "schema IR version 1 not supported" (`q-pack` fail-closed check) — no silent v1→v2 reinterpretation |
| v1 pack compatibility adapter / fallback markers | **Not in M25-001-A** — owned by M25-001-B |
| TS builder receives executable callback for transform/file/problem | Not representable: builder signatures take declarative metadata only; runtime shapes are JSON-serializable |
| Builder bounds violation (name charset/length, maxBytes, status, title, typeUri) | Builder-time `throw` (TS) / source-located `CompileError` (compiler extract) |
| Runtime validates a value against transform/file/problem node | Typed `unsupported` field error until M25-002+ codecs |
| Canonical ordering / hash algorithm | Unchanged from M24; structural parity fixtures only — algorithm owned by M25-001-C |
| Handler-facing transform codec documentation | Owned by M25-001-D |

## Projection coverage (v2 metadata)

| Node | `tsTypeOfIr` (contract/Treaty) | OpenAPI |
| --- | --- | --- |
| `transform` | type of `output` | schema of `output` |
| `file` | `Uint8Array` | `{ type: string, format: binary, x-maxBytes, x-contentType? }` |
| `problem` | `{ title: string; status: number; detail?: T }` | object with `x-problem` metadata |
| `union` (v1 gap fixed) | `A \| B` | `oneOf` (previously invalid `type: "union"`) |

## Validation semantics fixed alongside v2

- Present-but-`null` on a non-nullable, non-optional object member: `type`
  field error (previously inserted unvalidated).
- Query-source array items coerce strings per item schema (previously body
  rules were force-applied inside arrays).
