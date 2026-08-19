# Security and Reliability Plan

## Trust model

Velqu runs trusted application code in-process. Memory/stack/deadline limits reduce accidents and denial-of-service impact; they do not create a hostile multi-tenant sandbox.

## Required security domains

- HTTP parsing and admission limits;
- route/policy fail-closed behavior;
- QPack/bytecode integrity, ABI, and optional signatures;
- FFI/native handle ownership and stale-handle rejection;
- scheduler/job/native-operation bounds;
- capability lifecycle and dependency trust;
- fetch TLS, DNS, redirects, SSRF, streaming, and proxy configuration;
- database parameterization, pool bounds, cancellation, and secret handling;
- source maps, logs, traces, and problem redaction;
- compiler unsupported-import and deterministic-build controls;
- dependency, license, SBOM, provenance, and release signing.

## Test program

```text
unit and negative conformance
property tests
fuzzing with retained corpora
HTTP/QPack/schema/bridge parser fuzz targets
sanitizers and Miri where applicable
concurrency/model tests for ownership state machines
fault injection and chaos
24h/72h soak
10M-request resource-retention run
independent security review before RC
```

## Reliability invariants

After request or worker quiescence:

```text
live request slots == 0
pending Promise invocations == 0
pending native ops == 0
native tasks alive == 0
settlement entries == 0
job queue empty OR worker quarantined
boundary violations == 0
```

After quarantine:

```text
new JS work rejected
readiness false
pending work failed closed
native tasks aborted
worker replaced or process restart requested
liveness may remain true while process is alive
```

## Incident readiness

Before GA:

- security contact and disclosure policy;
- severity and response targets;
- compromised signing-key procedure;
- vulnerable engine/binding upgrade lane;
- bad-release rollback/yank procedure;
- CVE/advisory workflow;
- customer communication template.
