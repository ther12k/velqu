# Browser-WASM Definition of Done

The program is not done when a demo runs locally. It is done only when `BWASM-GATE` records GO against one exact candidate.

## Product contract

- [ ] A supported Velqu application builds with the documented browser-WASM target.
- [ ] Generated output deploys to generic static HTTPS hosting.
- [ ] No Velqu application server, TCP listener, or native runtime is needed after deployment.
- [ ] The docs disclose static hosting and any external gateway dependencies.
- [ ] Browser-local and native production behavior are clearly separated.
- [ ] Deployment-only requirements are machine-readable before execution where statically knowable.

## Real WASM path

- [ ] A Rust/WASM kernel is mandatory on the request path.
- [ ] Artifact/manifest verification runs before activation.
- [ ] Route matching and parameter extraction use the kernel.
- [ ] Request and response schema validation use the kernel.
- [ ] Capability authorization and canonical problem mapping use the kernel.
- [ ] There is no silent JavaScript-only compatibility fallback.
- [ ] WASM ABI, imports, exports, limits, and version rejection are documented and tested.

## Browser runtime

- [ ] `@velqu/browser-runtime` is browser-safe and distributable.
- [ ] Public boundary is `Request -> Promise<Response>`.
- [ ] Handler ABI and route-ID mapping are deterministic and versioned.
- [ ] Handlers execute in an isolated Worker outside the editor/UI realm.
- [ ] Abort, timeout, Worker crash, stale message, and recovery paths are deterministic.
- [ ] Treaty works through the same routing/validation/capability semantics.
- [ ] Structured diagnostics cover loader through handler/capability result.

## Build and deployment

- [ ] Compiler emits deterministic browser artifacts and source-located diagnostics.
- [ ] Artifact manifest binds all bytes, versions, sizes, and digests.
- [ ] Forbidden/transitive server-only imports fail closed.
- [ ] Service Worker controls only the documented application scope.
- [ ] Root and non-root base paths are tested.
- [ ] Cold install, reload, offline, multi-tab update, interrupted update, and rollback are tested.
- [ ] N and N+1 artifacts cannot mix.
- [ ] CLI build/preview/inspect/export workflows work from a clean consumer.

## Capabilities and persistence

- [ ] Timer, approved crypto, bounded logging, and restricted fetch pass shared contracts.
- [ ] Outbound network is default deny and credential safe.
- [ ] Postgres authoring contract is asynchronous with migration evidence.
- [ ] Memory and IndexedDB KV adapters pass one shared contract.
- [ ] Persistence is namespaced and reset/export/migration behavior is documented.
- [ ] Deployment-required/forbidden/unknown capabilities fail before side effects.
- [ ] Simulations are explicit and never indistinguishable from real side effects.
- [ ] Optional PGlite, when included, is lazy loaded and does not claim full/native PostgreSQL parity.

## Security

- [ ] Threat model is accepted.
- [ ] Production-shaped separate-origin fixture exists.
- [ ] CSP, Permissions Policy, iframe sandbox, Service Worker scope, and postMessage validation are tested.
- [ ] Preview code cannot access editor DOM/storage/auth/provider credentials.
- [ ] Network exfiltration and import-bypass attempts are covered.
- [ ] Body/message/log/output limits are adversarially tested.
- [ ] Independent security review findings are resolved or explicitly block GO.
- [ ] Public docs do not claim a hostile-code sandbox without evidence supporting that exact wording.

## Conformance and support

- [ ] Shared native/browser corpus covers all public Browser-WASM behaviors.
- [ ] Every difference is classified and linked to a support decision.
- [ ] Mutation tests prove the differential suite catches meaningful drift.
- [ ] Every claimed browser has a blocking real-browser lane.
- [ ] Unsupported/unverified browsers are labeled accurately.
- [ ] Native Velqu verification remains green.
- [ ] Release-like artifact bytes, not only source imports, are tested.

## Performance and reliability

- [ ] Core and optional artifact sizes meet frozen budgets.
- [ ] Optional SQL/engine assets are not downloaded by core-only apps.
- [ ] Cold/warm startup, verification, instantiation, first request, steady request, and Worker restart meet budgets.
- [ ] Defined request/error/abort/restart soak shows no unbounded memory growth.
- [ ] Measurements include raw samples, statistics, environment, browser/device, and artifact hashes.
- [ ] Browser-local metrics are not represented as native server-throughput comparisons.

## External usability and documentation

- [ ] Public quickstart is executable in CI.
- [ ] Limitations/support/capability/diagnostic/migration/security docs are complete.
- [ ] Independent cleanroom participant builds and deploys from published/candidate artifacts only.
- [ ] Cleanroom app exercises routing, schemas, Treaty, local persistence, update/rollback, offline, and deployment-required UX.
- [ ] No workspace/source-path fallback is used.
- [ ] Cleanroom failures are classified, fixed, and re-proven or block GO.

## Candidate evidence

- [ ] One exact source commit is frozen.
- [ ] All packages, WASM, handler bundles, manifests, docs, and comparator binaries are inventoried.
- [ ] SHA-256 checksums verify.
- [ ] SBOM/license/provenance outputs cover distributed artifacts.
- [ ] Every claim maps to evidence from the exact candidate.
- [ ] No required log comes from a different commit or modified local bytes.
- [ ] Zero unresolved P0 remains.
- [ ] Every unresolved P1 has an explicit owner/rationale/scope/expiry or produces NO-GO.
- [ ] `BWASM-GATE` records exactly one verdict: **GO** or **NO-GO**.
