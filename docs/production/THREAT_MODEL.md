# Velqu Consolidated Threat Model and Trust Boundaries

M6-001 (GA delta, issue tracked via `docs/production/GA-RECONCILIATION.md`).
This is the single consolidated native + browser threat-model document the
reconciliation required. It **consolidates and cites**; it does not change
decisions — the ADRs remain authoritative for their scope, and any boundary
change requires an ADR, not an edit here.

Maintenance rule: this document is updated whenever a trust-relevant ADR is
accepted (reviewer checklist: ADR touching trust ⇒ THREAT_MODEL.md row
updated in the same packet or the immediately following evidence packet).

---

## 0. The load-bearing limitation (read first)

**Same-process QuickJS runs trusted application code only. It is not, and
must never be described as, a sandbox for hostile or untrusted code**
(ADR-0035; AGENTS.md constraint 14). The process interior — memory, file
descriptors, CPU, secrets already readable by the process — is **inside**
the trust boundary. The security model protects:

- the **host network** from the application (ADR-0033/0034);
- the **client** from information leaks (RFC 9457 problems, redacted
  unexpected errors);
- the **serving origin's authority** in the browser product (ADR-0038).

It does **not** protect the process from the application. Resource bounds
(queues, bodies, deadlines — AGENTS.md constraint 11) are robustness
controls against accidents and load, not adversarial containment.

---

## 1. Assets

| Asset | Where |
|---|---|
| User request/response data | native listener → worker; browser Worker ↔ kernel |
| Host network reachability | outbound fetch (capability-gated) |
| DB data and credentials | `runtime:postgres` capability (params/secrets) |
| Process interior (secrets in env/files reachable by the process) | **not defended against the app** (§0) |
| Pack/artifact integrity | QPack load-time verification (native), in-band verify (browser) |
| Release artifacts (binaries, tarballs, SBOM, manifest) | release packet + signing (#1321 infra) |
| Serving origin authority (cookies, DOM, storage) | browser product, operator's origin |
| Supply chain (dependencies, toolchain) | Cargo.lock/bun.lock pins, SBOM, advisory policy |

## 2. Actors

| Actor | Trust role |
|---|---|
| Application operator / pack deployer | Owner of the process and origin; accountable for handler code (incl. AI-generated code they ship) |
| Artifact author/compiler | Produces the pack; **authenticity is out-of-band deployment policy** (ADR-0026), never self-declared |
| End user / network client | Untrusted: sends arbitrary bytes to the listener |
| Network adversary | Controls DNS answers, TLS peers, redirect targets, stale/tampered artifacts |
| Reverse proxy (edge) | Operator-controlled; its forwarded headers are **data, never identity** (ADR-0034 §3) |
| Velqu native runtime (Rust host + QuickJS worker) | Mechanism: routing, bounds, capability enforcement |
| Browser platform | Trusted computing base for origin isolation, CSP, storage partitioning (ADR-0038) |
| Supply-chain adversary | Compromised dependency or release artifact path |

## 3. Trust boundaries and entry points (native)

1. **Listener admission** (network → host): bounded header/query/body
   parsing; method+path routing only — `Host` never routes (ADR-0034 §6);
   forwarded headers never trusted (ADR-0034 §3); default loopback bind
   (ADR-0034 §5); TLS termination is the edge's job.
2. **Pack load** (disk → runtime): integrity/version/engine/ABI checks
   fail-closed before ready (ADR-0014 version-pinned bytecode, no
   arbitrary-bytecode path; ADR-0026 integrity; authenticity out-of-band).
   The compiler never dry-runs the app (ADR-0004) — no side effects during
   route discovery.
3. **Worker/bridge boundary** (host ↔ QuickJS): request data crossing into
   JS is lazy; native handles are opaque, generation-checked, expire at
   settlement (AGENTS.md constraints 7–8); expected failures are typed
   values; unexpected errors are redacted; everything bounded.
4. **Capability calls** (app → host): every side effect is a declared,
   compiler-granted capability (ADR-0010/0029); undeclared routes see no
   surface (absent-API fail closed); unknown grants fail the build.
5. **Outbound fetch** (app → network): `runtime:fetch@1` under the
   closed policy of ADR-0033 — scheme allowlist, resolved-IP SSRF
   classification (deny-by-default incl. IPv4-mapped normalization and
   globally-reachable-only IPv6), validate-after-resolve/connect-to-
   validated anti-rebinding, per-hop redirect revalidation with credential
   stripping, no ambient proxy trust, webpki roots + mandatory hostname
   validation, layered deadlines, bounded compression and bodies.
6. **DB capability** (app → Postgres): lazy bounded pool behind
   `runtime:postgres`; zero-I/O construction; fail-closed ceiling and
   connect deadlines; loopback/no-TLS posture until an owner TLS decision;
   parameterized query surface.
7. **Release artifact path** (build → operator): SHA256SUMS manifest over
   the full packet, detached OpenPGP signature, machine-read GPG status
   (GOODSIG+VALIDSIG required; revoked/expired/untrusted explicitly
   rejected), trusted-publishers registry as revocation authority over
   `--trusted-key`, fail-closed registry parse, explicit-key no-fallback
   (see `docs/production/operational/RELEASE_SIGNING_AND_DISASTER_RECOVERY.md`).

## 4. Trust boundaries (browser product)

From ADR-0038 (authoritative): origin boundary (the hard isolation);
kernel ↔ Worker validated-message boundary (schema-checked, bounded,
prototype-free); artifact provenance boundary (in-band integrity after
load; authenticity = operator's origin); capability boundary (kernel-side
authorization against the manifest). Untrusted preview runs on a separate
registrable origin + sandboxed iframe — **never** a runtime property.
Ambient-API honesty: in trusted mode the bridge cannot prevent direct
browser-API calls; that is a review/compiler convention with CSP
backstops, not a runtime guarantee.

## 5. Abuse cases → controls → evidence

Every high risk maps to a test or control; "evidence" links to the
committed artifact.

| # | Abuse case | Control | Evidence |
|---|---|---|---|
| N1 | Malformed/tampered QPack or injected bytecode | load-time integrity/version/ABI fail-closed; version-pinned bytecode | `pack_verify` fuzz target (sustained, zero findings — `benchmarks/raw/ga-m6-fuzz/`); q-pack tests |
| N2 | Route/schema graph confusion | mandatory semantic manifest, dense numeric IDs, non-shadowing router | `router_match` fuzz; q-router tests (M2.3) |
| N3 | Header/query/body parser abuse | bounded admission, percent-decode hardening | `http_decode` fuzz; q-http tests |
| N4 | Schema codec misclassification / backtracking blowup | bounded deterministic classification | `schema_validate` fuzz; q-schema-runtime tests |
| N5 | Stale/foreign handle use after settlement | generation checks, opaque handles, expiry | `bridge_handles` fuzz; q-bridge slab lifecycle tests |
| N6 | Problem-envelope corruption (RFC 9457) | reserved-member skip at encoder/builder/worker layers | `codec_encoders` fuzz (found + fixed: #1339 regression test `problem_encoder_skips_envelope_named_extensions`) |
| N7 | SSRF / metadata theft / DNS rebinding / redirect pivot / decompression bomb / scheme confusion | ADR-0033 policy (§3 table) | `capabilities_policy` fuzz; `fetch_policy.rs` unit matrix (incl. IPv6 globally-reachable rule from fuzz observation 3) |
| N8 | Undeclared route dials out; handler widens own trust | capability grant + absent-API fail closed; runtime-owned policy, no JS config surface (ADR-0034 §1–2) | M27-010-D absent-API test; fetch policy tests |
| N9 | Forwarded-header identity spoofing; Host-header routing confusion | headers-are-data; connection peer only; Host never routes (ADR-0034 §3,§6) | distrust-list tests in `fetch_policy.rs` |
| N10 | Secret/error disclosure to clients | RFC 9457 problems; redaction before the wire; `SecretString` | `security-review.md`; q-runtime tests |
| N11 | Worker poison / cancellation storms / fairness starvation | bounded cancellation, ownership/drain, dispatcher fairness | M3-005/M3-007/M3-008 suites; chaos report (`beta-009-d`); 24h/72h soak |
| N12 | Unbounded allocation / memory unsafety anywhere in the first-party surface | campaign rss_limit bounds; ASan+LSan workspace (S1); UBSan C-FFI (S2); Miri over FFI-free crates; unsafe audit 0 unclassified | `benchmarks/raw/ga-m6-fuzz/` (S1/S2/miri ledgers); `docs/production/evidence/m6-unsafe-audit.md`; `fuzz/sanitizer-applicability.json` (TSan waiver) |
| N13 | Tampered/substituted release artifact; revoked signer sneaks a packet | signed manifest, machine-read GPG status, registry revocation authority, no-fallback explicit-key contract, fail-closed registry parse | `scripts/verify-release-packet.sh` + 17-test suite (`scripts/verify-release-packet.test.ts`); signing DR doc |
| N14 | Supply-chain dependency compromise | pinned lockfiles; SBOM with license coverage; advisory policy (M6-004 tracks the standing policy) | `beta-015` SBOM evidence; `beta-009-b` inventory |
| N15 | Operator misconfiguration exposure | loopback default; reverse-proxy-first posture; canary + rollback runbooks | ADR-0034 §4–5; `docs/production/operational/CANARY_PROGRAM.md` |
| B1 | Tampered/stale artifact served in browser | in-band integrity fail-closed after load; hash-keyed cache re-verification | `docs/browser-wasm/evidence/q-003/threat-model-verification.md` |
| B2 | Handler exfiltration beyond declared endpoints (browser) | kernel allowlist + CSP `connect-src` backstop; credential-stripping adapter | BWASM-Q-003 verification |
| B3 | Malicious Worker message; Worker DoS | schema-validated bounded protocol; deadline → kill-and-replace | BWASM-Q-003; worker-host tests |
| B4 | Cross-artifact storage overreach | per-artifact-identity namespacing, no global handles | capabilities tests (browser) |
| B5 | Untrusted preview escape attempt | separate origin + sandboxed iframe deployment contract (not a runtime claim) | ADR-0038 §3; forbidden-claims rule |

## 6. Explicit non-claims (residual risk register)

- No hostile-code / multi-tenant sandboxing claim, native or browser (§0;
  ADR-0035/0038 forbidden claims).
- Application exfiltration of secrets already inside the process: not
  mitigated — deployment concern (ADR-0035 table).
- Adversarial resource exhaustion by trusted code: partially covered
  (bounds cover accident-class load); operator monitoring concern.
- Runtime TLS termination, signed proxy identity forwarding: not in scope
  (ADR-0034 non-goals); edge-owned.
- DB TLS: absent until an owner decision; loopback deployments only
  (BETA-004-B posture).
- Signed proxy identity, Windows/macOS, independent hostile-code
  penetration testing: outside this baseline (beta-009-c limitations
  carried forward).
- Publisher key in `trusted-publishers.json` is provisioning-pending
  (bring-up key, owner ratification still owed before RC) — see
  `docs/open-decisions.md` / #1321 residuals.

## 7. Evidence index

- Sustained fuzz/sanitizer/Miri campaign (M6-002/M6-003):
  `benchmarks/raw/ga-m6-fuzz/` + `docs/production/evidence/m6-002-sustained-fuzz.md`
  + `docs/production/evidence/m6-003-sanitizers-miri.md` + `fuzz/COVERAGE.md`.
- Unsafe/FFI audit: `docs/production/evidence/m6-unsafe-audit.md`.
- Release signing/verification: `docs/production/operational/RELEASE_SIGNING_AND_DISASTER_RECOVERY.md`,
  `scripts/verify-release-packet.sh` (+ test suite).
- Browser boundary verification: `docs/browser-wasm/evidence/q-003/threat-model-verification.md`.
- Beta boundary review (superseded by this consolidation, retained as
  source): `docs/reports/beta-009-c-threat-model-review.md`.
- Risk register: `docs/beta/governance/RISK_REGISTER.md`,
  `docs/production/RISK_REGISTER.md`.
- ADRs: 0010, 0014, 0026, 0029, 0033, 0034, 0035, 0038 (and 0003/0004/0011
  as background).
