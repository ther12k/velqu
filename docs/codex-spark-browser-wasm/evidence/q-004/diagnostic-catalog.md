# BWASM-Q-004 Diagnostic Catalog

The Browser-WASM runtime provides stable, machine-readable diagnostic codes across 12 frozen lifecycle stages.

## Lifecycle Stages

1. `load`: Manifest and artifact byte loading.
2. `verify`: Content-addressed SHA-256 and integrity checks.
3. `instantiate`: Wasm module compilation, kernel initialization, and runtime readiness.
4. `route`: HTTP method, path matching, and route selection.
5. `validate`: Request schema validation (params, query, headers, body).
6. `invoke`: Worker handler invocation, execution, and response collection.
7. `capability`: Host capability authorization and execution.
8. `persist`: Namespaced IndexedDB / Memory KV persistence operations.
9. `cache`: Service Worker caching, cache hits, and cache storage.
10. `update`: Build ID detection and Service Worker activation.
11. `cancel`: In-flight abort signal cancellation.
12. `fail`: Unhandled or internal errors mapped to RFC-9457 problems.

## Diagnostic Codes

| Code | Stage | Severity | Description |
|---|---|---|---|
| `DIAG_LOAD_MANIFEST_OK` | `load` | INFO | Manifest JSON fetched and parsed successfully |
| `DIAG_LOAD_MANIFEST_FAIL` | `load` | ERROR | Manifest fetch failed or JSON was invalid |
| `DIAG_VERIFY_INTEGRITY_OK` | `verify` | INFO | All artifacts matched their expected SHA-256 digests |
| `DIAG_VERIFY_INTEGRITY_FAIL` | `verify` | ERROR | Artifact digest mismatch or size corruption detected |
| `DIAG_COMPAT_KERNEL_ABI_OK` | `instantiate` | INFO | Kernel ABI version matches runtime contract (ABI 1) |
| `DIAG_COMPAT_KERNEL_ABI_MISMATCH` | `instantiate` | ERROR | Kernel reports an incompatible ABI version |
| `DIAG_COMPAT_HANDLER_ABI_MISMATCH` | `instantiate` | ERROR | Handler bundle reports an incompatible ABI version |
| `DIAG_LIFECYCLE_INSTANTIATING` | `instantiate` | INFO | Initializing WebAssembly kernel and state |
| `DIAG_LIFECYCLE_READY` | `instantiate` | INFO | Runtime is ready to accept HTTP fetch requests |
| `DIAG_LIFECYCLE_DISPOSED` | `instantiate` | INFO | Runtime explicitly disposed; subsequent calls fail closed |
| `DIAG_ROUTE_MATCHED` | `route` | DEBUG | Inbound Request matched a declared route |
| `DIAG_ROUTE_NOT_FOUND` | `route` | WARN | Path did not match any declared route (404) |
| `DIAG_ROUTE_METHOD_NOT_ALLOWED` | `route` | WARN | Method not allowed for matched path (405) |
| `DIAG_VALIDATE_PASSED` | `validate` | DEBUG | Input data conformed to declared route schemas |
| `DIAG_VALIDATE_FAILED` | `validate` | WARN | Schema validation failure (422 Unprocessable Entity) |
| `DIAG_CAPABILITY_INVOKED` | `capability` | DEBUG | Capability handle invoked by authorized route |
| `DIAG_CAPABILITY_DENIED` | `capability` | WARN | Capability denied by kernel authorization or policy |
| `DIAG_CAPABILITY_DEPLOYMENT_REQUIRED` | `capability` | WARN | Route requires native server deployment (e.g. Postgres) |
| `DIAG_INVOKE_START` | `invoke` | DEBUG | Worker started execution of handler |
| `DIAG_INVOKE_SUCCESS` | `invoke` | DEBUG | Handler completed with declared response status |
| `DIAG_INVOKE_TIMEOUT` | `invoke` | ERROR | Handler exceeded deadline and Worker was terminated |
| `DIAG_INVOKE_CANCEL` | `cancel` | INFO | Request cancelled via AbortSignal |
| `DIAG_INVOKE_FAILED` | `invoke` | WARN | Handler returned a typed problem or threw an error |
| `DIAG_PERSIST_ACCESS` | `persist` | DEBUG | KV store get, set, delete, or list operation |
| `DIAG_PERSIST_ERROR` | `persist` | ERROR | Underlying IndexedDB operation failed |
| `DIAG_PERSIST_QUOTA_EXCEEDED` | `persist` | ERROR | Key or value size exceeded storage limits |
| `DIAG_PERSIST_MIGRATION_REQUIRED` | `persist` | WARN | Storage schema version mismatch requires migration |
| `DIAG_CACHE_HIT` | `cache` | DEBUG | Asset served from verified Cache Storage |
| `DIAG_CACHE_MISS` | `cache` | WARN | Requested asset was not present in offline cache |
| `DIAG_CACHE_STORED` | `cache` | INFO | Precached verified artifact stored in cache |
| `DIAG_SW_REGISTERED` | `cache` | INFO | Service Worker registered under scoped path |
| `DIAG_SW_UPDATE_AVAILABLE` | `update` | INFO | Newer build detected; waiting for next reload |
| `DIAG_SW_UPDATE_APPLIED` | `update` | INFO | New Service Worker activated |
| `DIAG_FAIL_INTERNAL` | `fail` | ERROR | Internal runtime or protocol exception |
| `DIAG_FAIL_REDACTED` | `fail` | ERROR | Exception detail redacted for production export |

## Failure Category Distinguishability

Per Acceptance Criterion 1, each failure category is mapped to a distinct code:
1. Integrity: `DIAG_VERIFY_INTEGRITY_FAIL`
2. Compatibility: `DIAG_COMPAT_KERNEL_ABI_MISMATCH`, `DIAG_COMPAT_HANDLER_ABI_MISMATCH`
3. Route: `DIAG_ROUTE_NOT_FOUND`, `DIAG_ROUTE_METHOD_NOT_ALLOWED`
4. Schema: `DIAG_VALIDATE_FAILED`
5. Capability: `DIAG_CAPABILITY_DENIED`
6. Handler: `DIAG_INVOKE_FAILED`
7. Timeout: `DIAG_INVOKE_TIMEOUT`
8. Persistence: `DIAG_PERSIST_ERROR`, `DIAG_PERSIST_QUOTA_EXCEEDED`, `DIAG_PERSIST_MIGRATION_REQUIRED`
9. Cache: `DIAG_CACHE_MISS`
10. Deployment-Required: `DIAG_CAPABILITY_DEPLOYMENT_REQUIRED`
