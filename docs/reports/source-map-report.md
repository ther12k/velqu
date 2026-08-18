---
type: Evidence Report
title: Source Map Report
status: complete
milestone: M1
---

# Source-map report

Question (M1 §11.8): a TypeScript-originated exception must identify a useful
ORIGINAL source location; generated/bridge frames may be annotated but must
not remove causality.

## Implementation

- Pack carries an optional embedded source map (`sourceMap` field, sourcemap
  JSON string). `velqu-runtime/src/source_map.rs` parses it with the `sourcemap`
  crate and implements the engine `SourceMapper` trait; if absent, an
  identity mapper is used.
- The engine worker extracts QuickJS exception `message` + `stack`
  (`rquickjs::Exception`), maps the FIRST engine frame (`app.js:line:col`,
  1-based from QuickJS) through `lookup_token(line-1, col)` (crate is
  0-based), and attaches both generated and original locations to the
  EngineFailure outcome.
- Mapped detail goes to the INTERNAL structured log (`handler.error` event
  with `source.original{source,line,column}`); the HTTP response stays
  redacted (RUN-007). No causality is removed: the engine stack is retained
  in the log alongside the mapped location.

## Evidence

Test: `source_mapped_exception_identifies_original_location`
(`crates/velqu-runtime/tests/runtime_conformance.rs`): a generated bundle whose
line 2 throws is paired with a hand-built map to
`src/modules/users/routes.ts` line 42 col 5. The binary run produces a 500
and stderr log containing BOTH `origin-boom` (cause) and the mapped
`src/modules/users/routes.ts ... 42` — assertion passes.

Sample log line produced:

```json
{"level":"error","event":"handler.error","requestId":"req-…","routeId":"throw.redacted",
 "detail":"origin-boom\n    at thrower (eval_script:2:13)…",
 "source":{"generated":[2,13],"original":{"source":"src/modules/users/routes.ts","line":42,"column":13}}}
```

## Verdict

Usable: yes — original file+line are present and correct; the generated frame
is retained. Limitations recorded honestly:

1. Only the first stack frame is mapped (top-most application frame);
   multi-frame mapped traces are an M2 polish item.
2. The M2 compiler must emit Bun.build sourcemaps into the pack; the M1
   fixture embeds a hand-built map (mechanism identical).
3. Columns map through identity for tokens without a mapping — logged as
   generated.
