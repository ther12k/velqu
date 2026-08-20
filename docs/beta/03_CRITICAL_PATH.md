---
type: Roadmap
title: Velqu Beta Critical Path
status: draft
tags:
- roadmap
- critical-path
- beta

---

# Beta Critical Path

```text
G0 — M23R2 gate closure
  ↓
M2.4 — zero-copy ingress and worker-local request slab
  ↓
M2.5 — schema-specialized codecs
  ↓
M2.6 — binary QPack v2
  ↓
M2.7 — capability linker and minimal Web runtime
  ↓
M2.8 — native fetch
  ↓
M3 — multi-worker service runtime
  ↓
M4A — actual-runtime developer preview/private alpha
  ↓
BETA — real-world proof, packaging, public beta release
```

## Parallel lanes

- Real-world benchmark infrastructure may begin after G0 planning and execute without adding out-of-order runtime features.
- Documentation/examples may track stable milestone outputs.
- Postgres and JWT implementation wait for the M2.7 capability ABI.
- Packaging preparation may begin after M2.6, while publishing waits for M4A and beta gates.
- Owner decisions may proceed at any point but block only `BETA-GATE`.

## Gate rule

No milestone consumes an upstream representation as a correctness or security control until that upstream gate passes. In particular:

- M2.4 may not trust FieldNeeds/SchemaId until G0 passes.
- M2.5 may not generate codecs against an unstable request bridge.
- M2.7 capabilities may not bypass the QPack v2 fingerprint.
- M3 may not hide single-worker correctness issues.
- Public beta may not use spot checks as canonical performance evidence.

## Kill and narrowing criteria

- If optional capabilities erase the cold-start/RSS thesis, split runtime profiles or defer the capability.
- If post-M2.5 representative dynamic routes remain materially below the approved JIT comparison threshold, position Velqu explicitly as cold-start/isolated-I/O specialized rather than inventing a universal claim.
- If poisoned-worker replacement is unreliable, public beta is limited to serverless one-worker mode until corrected.
- If Postgres cannot remain optional and lazy, it is released as a reference adapter rather than first-party beta commitment.
