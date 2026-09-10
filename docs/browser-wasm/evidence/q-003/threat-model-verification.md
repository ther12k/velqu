# BWASM-Q-003 Threat-Model Verification

Status: PASS for documented Browser-WASM preview boundary controls.
Source commit: `6df1312` (origin/master before this packet).
Candidate scope commit: recorded in task result after commit.

## Boundary checks

| Boundary | Verification | Result |
|---|---|---|
| Preview/editor origin | Service Worker intercepts only same-origin URLs inside a path-segment-bounded scope; cross-origin and `/app2/` lookalike paths pass through. | PASS |
| Editor/auth/gateway traffic | `__velqu_editor__`, `/auth/`, `/oauth/`, and `/model-gateway/` are passthrough after base-path normalization. | PASS |
| Worker protocol | Version, message shape, session ID, invocation ID, result size, log volume, stale messages, and fatal recovery are tested. | PASS |
| Capability network | Fetch is default-deny, exact-origin allowlisted, method-bounded, credential-free, redirect-denied, and request/response bounded before or during I/O. | PASS |
| Storage | KV namespace is application/project scoped; cross-namespace isolation and IndexedDB failure behavior are covered by shared adapter tests. | PASS |
| Service Worker scope | Scope is origin-checked and segment-bounded; scope escapes never call `respondWith`. | PASS |

## Adversarial checks

- DOM/editor authority is not exposed through Worker protocol or browser capability graph.
- Cookie and authorization headers are stripped by the browser fetch adapter; `credentials: "omit"` is forced.
- Foreign and stale Worker results are dropped; invocation IDs are session-scoped.
- Oversized result/log payloads terminate or reject through bounded protocol paths.
- Redirect responses are rejected rather than followed by the capability adapter.
- Invalid or unavailable Service Worker registration returns explicit injected-fetch fallback.

## Residual risks and limitations

- Worker and WebAssembly isolation are **not** hostile-code or multi-tenant sandboxing. Same-origin generated handlers remain trusted application code under the documented deployment posture.
- This packet verifies source-level controls and deterministic browser-runtime tests. It does not claim an independent hostile-code penetration test, browser exploit resistance, or production CDN/header configuration.
- A deployment must serve preview on its isolated origin and preserve its CSP, Permissions Policy, and iframe policy. Header enforcement belongs to the host/deployment configuration and remains an operational check.
- Browser extensions, compromised origins, browser engine vulnerabilities, and a compromised static host are outside this packet.

## Commands

```text
bun test packages/browser-runtime/test/service-worker.test.ts packages/browser-runtime/test/worker-host.test.ts packages/browser-runtime/test/capabilities.test.ts
bun run typecheck
unshare -rn bash -c 'ip link set lo up; ./scripts/verify'
```

Targeted result: 64 pass, 0 fail.
Full verification is recorded in task handoff after the packet run.
