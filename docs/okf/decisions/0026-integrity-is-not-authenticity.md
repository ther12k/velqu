---
type: Architecture Decision Record
title: ADR-0026 Integrity Is Not Authenticity — Pack Verification Trust Model
status: accepted
date: 2026-08-23
implements: ADR-0024 (numeric mode policy), ADR-0025 (section directory), ADR-0014 (version-pinned trusted bytecode)
---

# ADR-0026: Integrity Is Not Authenticity — Pack Verification Trust Model

## Context

Pack verification re-hashes bundle, execution graph, and (in mode 2)
every section against in-band digests. It would be easy to over-read
this as "verified packs are trusted packs." These are different security
questions, and conflating them would misstate the runtime's guarantees.

## Decision

### 1. Definitions

- **Integrity** — evidence that bytes match the pack's own recorded
  digests: detects corruption and naive tampering. In-band, enforced at
  load, fail closed (`integrity failure …` rejections; mode 2 §3 rule 6).
- **Authenticity** — proof of *who authorized* the bytes. Cannot be
  established from the pack alone: any writer able to rewrite content can
  rewrite in-band digests too (pinned by test
  `self_consistent_digests_verify_without_trust_anchor`).

### 2. Runtime policy

- The runtime enforces **integrity only**. It has no key store, no trust
  anchors, no signature fields with special semantics, and accepts no
  authenticity-by-declaration inside the pack.
- **Authenticity is a deployment concern**, out-of-band by design:
  detached signatures over the pack file verified before deployment,
  build provenance (release packet SHA256SUMS bound to the source
  commit), or equivalent operator controls.
- Same-process QuickJS executes trusted application code only
  (constraint 14); nothing here is a sandbox and no verification step
  turns an untrusted pack into a safe one.
- Bytecode enters artifacts only via the compiler-owned rebuild path
  (`velqu-bytecode embed`, engine-version-pinned, digest-bound) —
  untrusted arbitrary bytecode remains forbidden (ADR-0014/0017).

### 3. Where each lives (layout view)

```text
v1 JSON pack                     v2 binary sections
  integrity {                      per-section content_sha256   (integrity)
    bundleSha256,                  header reserved bytes        (future ext;
    routesSha256,                                               still NOT signatures)
    bytecodeSha256?              }  authenticity: OUT OF BAND
authenticity: OUT OF BAND          [detached sig file | build provenance]
[detached sig file | provenance]   e.g. release/SHA256SUMS.txt binding
```

### 4. Compatibility matrix

| artifact state \ deployment | unsigned env | signed env (detached sig verified pre-run) |
|---|---|---|
| pristine producer output | loads (integrity pass) | loads |
| corrupted / naively tampered | rejected (integrity fail closed) | rejected before run |
| maliciously rewritten + re-hashed | **loads** — integrity cannot detect this; authenticity controls must catch it | rejected (signature mismatch) |
| unknown formatVersion | rejected (ADR-0024 fail closed) | rejected |

## Consequences

- Docs and error messages must never claim packs are "authenticated" or
  "trusted because verified"; they are integrity-checked.
- Mode-2 reserved header space stays available for future *metadata*,
  but adding in-band signatures would not remove the need for out-of-band
  verification and is out of scope without a new owner-track decision.
- Deployment guidance belongs to the beta docs (devex), not to q-pack.
