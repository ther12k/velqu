# Task Index

Total atomic packets: **631**.


## G0 — Trusted Numeric Graph and Evidence Truth

All 59 G0 atomic packets and the G0 gate are **PASS** at the recorded evidence checkpoint. The task titles below preserve the original acceptance language; the Status column is the current source of truth.

| ID | P | Kind | Status | Dependencies | Task |
|---|---:|---|---|---|---|
| [G0-001-A](../tasks/00_g0_gate_close/G0-001-A-verify-source-4e69049-zip-and-velqu-4e69049-bundle-against-source-commit-txt-and.md) | P0 | implement | PASS | — | Verify source-4e69049.zip and velqu-4e69049.bundle against SOURCE-COMMIT.txt and the supplied SHA256SUMS manifest |
| [G0-001-B](../tasks/00_g0_gate_close/G0-001-B-reconcile-review-index-json-and-evidence-index-json-so-their-commit-milestone-te.md) | P0 | implement | PASS | G0-001-A | Reconcile current REVIEW_INDEX.json and EVIDENCE_INDEX.json |
| [G0-001-C](../tasks/00_g0_gate_close/G0-001-C-update-docs-beta-00-current-baseline-md-docs-beta-04-task-ledger-md-and-related.md) | P0 | implement | PASS | G0-001-B | Reconcile beta baseline, ledger, and current status documents |
| [G0-001-D](../tasks/00_g0_gate_close/G0-001-D-capture-compiler-rust-bun-quickjs-ng-rquickjs-os-cpu-load-generator-and-benchmar.md) | P0 | implement | PASS | G0-001-C | Capture compiler, Rust, Bun, QuickJS-NG, rquickjs, OS, CPU, load-generator, and benchmark-tool versions in one current environment report |
| [G0-001-E](../tasks/00_g0_gate_close/G0-001-E-quarantine-stale-historical-release-metadata-under-an-explicitly-historical-dire.md) | P0 | implement | PASS | G0-001-D | Quarantine stale historical release metadata under an explicitly historical directory or remove it from the current release packet |
| [G0-001-V](../tasks/00_g0_gate_close/G0-001-V-verify-freeze-and-reconcile-the-4e69049-release-baseline.md) | P0 | verify | PASS | G0-001-A, G0-001-B, G0-001-C, G0-001-D, G0-001-E | Verify Freeze and reconcile the 4e69049 release baseline |
| [G0-001-Z](../tasks/00_g0_gate_close/G0-001-Z-package-evidence-for-freeze-and-reconcile-the-4e69049-release-baseline.md) | P0 | evidence | PASS | G0-001-V | Package evidence for Freeze and reconcile the 4e69049 release baseline |
| [G0-002-A](../tasks/00_g0_gate_close/G0-002-A-require-velqufunctionmanifest-for-the-current-numeric-pack-version-and-reject-a.md) | P0 | implement | PASS | G0-001-Z | Require __velquFunctionManifest for the current numeric pack version and reject a missing manifest before ready |
| [G0-002-B](../tasks/00_g0_gate_close/G0-002-B-remove-the-current-numeric-count-only-velqufunctions-fallback-from-workerinner-l.md) | P0 | implement | PASS | G0-002-A | Remove the current numeric count-only __velquFunctions fallback from WorkerInner::load |
| [G0-002-C](../tasks/00_g0_gate_close/G0-002-C-validate-every-numeric-vector-entry-by-exact-index-key-kind-and-callability.md) | P0 | implement | PASS | G0-002-B | Validate every numeric vector entry by exact index, key, kind, and callability |
| [G0-002-D](../tasks/00_g0_gate_close/G0-002-D-move-any-count-only-behavior-behind-an-explicit-legacy-pack-version-adapter-with.md) | P0 | implement | PASS | G0-002-C | Move any count-only behavior behind an explicit legacy pack version adapter with separate tests |
| [G0-002-V](../tasks/00_g0_gate_close/G0-002-V-verify-make-the-semantic-function-manifest-mandatory.md) | P0 | verify | PASS | G0-002-A, G0-002-B, G0-002-C, G0-002-D | Verify Make the semantic function manifest mandatory |
| [G0-002-Z](../tasks/00_g0_gate_close/G0-002-Z-package-evidence-for-make-the-semantic-function-manifest-mandatory.md) | P0 | evidence | PASS | G0-002-V | Package evidence for Make the semantic function manifest mandatory |
| [G0-003-A](../tasks/00_g0_gate_close/G0-003-A-define-the-execution-graph-projection-to-include-function-manifest-routeplans-po.md) | P0 | implement | PASS | G0-001-Z | Define the execution graph projection to include function manifest, RoutePlans, policy bindings, schema manifest, capability bindings, and every serialized router node/terminal |
| [G0-003-B](../tasks/00_g0_gate_close/G0-003-B-recompute-and-verify-the-execution-graph-hash-inside-qpack-verify-before-ready.md) | P0 | implement | PASS | G0-003-A | Recompute and verify the execution graph hash inside QPack::verify before ready |
| [G0-003-C](../tasks/00_g0_gate_close/G0-003-C-implement-serializedrouter-semantic-verification-non-empty-root-unique-static-ed.md) | P0 | implement | PASS | G0-003-B | Implement SerializedRouter semantic verification: non-empty root, unique static edges, exact method masks, route method/path agreement, reachability, capture counts, and valid RouteId references |
| [G0-003-D](../tasks/00_g0_gate_close/G0-003-D-add-tamper-fixtures-that-redirect-a-terminal-to-a-different-valid-routeid-mutate.md) | P0 | implement | PASS | G0-003-C | Add tamper fixtures that redirect a terminal to a different valid RouteId, mutate a method slot, alter a path shape, and mutate schema-manifest IR |
| [G0-003-V](../tasks/00_g0_gate_close/G0-003-V-verify-bind-router-and-schema-manifests-into-the-execution-graph-hash.md) | P0 | verify | PASS | G0-003-A, G0-003-B, G0-003-C, G0-003-D | Verify Bind router and schema manifests into the execution graph hash |
| [G0-003-Z](../tasks/00_g0_gate_close/G0-003-Z-package-evidence-for-bind-router-and-schema-manifests-into-the-execution-graph-h.md) | P0 | evidence | PASS | G0-003-V | Package evidence for Bind router and schema manifests into the execution graph hash |
| [G0-004-A](../tasks/00_g0_gate_close/G0-004-A-change-the-current-numeric-startup-path-so-router-from-pack-consumes-the-verifie.md) | P0 | implement | PASS | G0-003-Z | Change the current numeric startup path so Router::from_pack consumes the verified serialized nodes directly and never calls Router::build |
| [G0-004-B](../tasks/00_g0_gate_close/G0-004-B-keep-router-build-only-in-the-reference-matcher-compiler-tests-and-explicit-lega.md) | P0 | implement | PASS | G0-004-A | Keep Router::build only in the reference matcher/compiler tests and explicit legacy adapter |
| [G0-004-C](../tasks/00_g0_gate_close/G0-004-C-return-routeid-plus-capture-ranges-from-the-serialized-matcher-and-derive-404-40.md) | P0 | implement | PASS | G0-004-B | Return RouteId plus capture ranges from the serialized matcher and derive 404/405/Allow without rebuilding routes |
| [G0-004-D](../tasks/00_g0_gate_close/G0-004-D-add-a-genuine-generated-property-suite-comparing-serialized-matching-with-the-in.md) | P0 | implement | PASS | G0-004-C | Add a genuine generated property suite comparing serialized matching with the independent reference matcher across methods, static/param/wildcard paths, parameter names, 404, 405, and Allow |
| [G0-004-V](../tasks/00_g0_gate_close/G0-004-V-verify-load-the-serialized-router-directly.md) | P0 | verify | PASS | G0-004-A, G0-004-B, G0-004-C, G0-004-D | Verify Load the serialized router directly |
| [G0-004-Z](../tasks/00_g0_gate_close/G0-004-Z-package-evidence-for-load-the-serialized-router-directly.md) | P0 | evidence | PASS | G0-004-V | Package evidence for Load the serialized router directly |
| [G0-005-A](../tasks/00_g0_gate_close/G0-005-A-make-router-match-results-carry-routeid-and-use-routeid-to-index-a-dense-verifie.md) | P0 | implement | PASS | G0-002-Z, G0-004-Z | Make Router match results carry RouteId and use RouteId to index a dense verified RoutePlan vector |
| [G0-005-B](../tasks/00_g0_gate_close/G0-005-B-introduce-a-dense-policyplan-manifest-so-policyid-resolves-to-the-exact-pre-veri.md) | P0 | implement | PASS | G0-005-A | Introduce a dense PolicyPlan manifest so PolicyId resolves to the exact pre-verified policy HandlerId |
| [G0-005-C](../tasks/00_g0_gate_close/G0-005-C-require-a-dense-complete-schemaid-manifest-and-use-schemaid-for-all-request-vali.md) | P0 | implement | PASS | G0-005-B | Require a dense complete SchemaId manifest and use SchemaId for all request validation lookups |
| [G0-005-D](../tasks/00_g0_gate_close/G0-005-D-move-route-policy-handler-and-schema-names-into-debug-inspection-tables-rather-t.md) | P0 | implement | PASS | G0-005-C | Move route, policy, handler, and schema names into debug/inspection tables rather than request execution |
| [G0-005-E](../tasks/00_g0_gate_close/G0-005-E-add-counters-or-assertions-proving-the-current-numeric-path-performs-zero-string.md) | P0 | implement | PASS | G0-005-D | Add counters or assertions proving the current numeric path performs zero string identity lookup |
| [G0-005-V](../tasks/00_g0_gate_close/G0-005-V-verify-complete-operational-routeid-policyid-and-schemaid-usage.md) | P0 | verify | PASS | G0-005-A, G0-005-B, G0-005-C, G0-005-D, G0-005-E | Verify Complete operational RouteId, PolicyId, and SchemaId usage |
| [G0-005-Z](../tasks/00_g0_gate_close/G0-005-Z-package-evidence-for-complete-operational-routeid-policyid-and-schemaid-usage.md) | P0 | evidence | PASS | G0-005-V | Package evidence for Complete operational RouteId, PolicyId, and SchemaId usage |
| [G0-006-A](../tasks/00_g0_gate_close/G0-006-A-define-a-dedicated-public-canonical-model-covering-method-path-path-query-header.md) | P1 | implement | PASS | G0-003-Z, G0-005-Z | Define a dedicated public canonical model covering method, path, path/query/header/body schemas, coercion, content types, statuses, response bodies, public problems, security, and deprecation metadata |
| [G0-006-B](../tasks/00_g0_gate_close/G0-006-B-exclude-function-names-ids-policy-implementation-handler-serializer-strategy-rou.md) | P1 | implement | PASS | G0-006-A | Exclude function names/IDs, policy implementation handler, serializer strategy, router layout, internal capability indexes, and unreachable private schemas |
| [G0-006-C](../tasks/00_g0_gate_close/G0-006-C-recompute-and-require-publiccontracthash-for-current-numeric-packs-inside-qpack.md) | P1 | implement | PASS | G0-006-B | Recompute and require publicContractHash for current numeric packs inside QPack::verify |
| [G0-006-D](../tasks/00_g0_gate_close/G0-006-D-add-stability-change-tests-for-internal-reorder-handler-rename-serializer-change.md) | P1 | implement | PASS | G0-006-C | Add stability/change tests for internal reorder, handler rename, serializer change, header/body/content-type/security changes, and arbitrary supplied hashes |
| [G0-006-V](../tasks/00_g0_gate_close/G0-006-V-verify-separate-and-verify-public-contract-identity.md) | P1 | verify | PASS | G0-006-A, G0-006-B, G0-006-C, G0-006-D | Verify Separate and verify public contract identity |
| [G0-006-Z](../tasks/00_g0_gate_close/G0-006-Z-package-evidence-for-separate-and-verify-public-contract-identity.md) | P1 | evidence | PASS | G0-006-V | Package evidence for Separate and verify public contract identity |
| [G0-007-A](../tasks/00_g0_gate_close/G0-007-A-introduce-an-explicit-current-pack-format-execution-mode-for-numeric-artifacts.md) | P1 | implement | PASS | G0-002-Z, G0-005-Z | Introduce an explicit current pack format/execution mode for numeric artifacts |
| [G0-007-B](../tasks/00_g0_gate_close/G0-007-B-remove-handlertable-from-the-current-numeric-pack-schema-and-compiler-output.md) | P1 | implement | PASS | G0-007-A | Remove handlerTable from the current numeric pack schema and compiler output |
| [G0-007-C](../tasks/00_g0_gate_close/G0-007-C-require-function-policy-schema-routeplan-and-serialized-router-manifests-in-nume.md) | P1 | implement | PASS | G0-007-B | Require function, policy, schema, RoutePlan, and serialized-router manifests in numeric mode |
| [G0-007-D](../tasks/00_g0_gate_close/G0-007-D-isolate-v1-legacy-loading-in-a-versioned-compatibility-adapter-and-reject-mixed.md) | P1 | implement | PASS | G0-007-C | Isolate v1 legacy loading in a versioned compatibility adapter and reject mixed-mode artifacts |
| [G0-007-E](../tasks/00_g0_gate_close/G0-007-E-verify-current-numeric-startup-allocates-no-legacy-handler-cache-or-registration.md) | P1 | implement | PASS | G0-007-D | Verify current numeric startup allocates no legacy handler cache or registration map |
| [G0-007-V](../tasks/00_g0_gate_close/G0-007-V-verify-remove-duplicate-legacy-state-from-current-packs.md) | P1 | verify | PASS | G0-007-A, G0-007-B, G0-007-C, G0-007-D, G0-007-E | Verify Remove duplicate legacy state from current packs |
| [G0-007-Z](../tasks/00_g0_gate_close/G0-007-Z-package-evidence-for-remove-duplicate-legacy-state-from-current-packs.md) | P1 | evidence | PASS | G0-007-V | Package evidence for Remove duplicate legacy state from current packs |
| [G0-008-A](../tasks/00_g0_gate_close/G0-008-A-run-warm-workloads-at-concurrency-1-10-and-50-for-at-least-five-independent-repe.md) | P1 | implement | PASS | G0-004-Z, G0-005-Z, G0-007-Z | Run warm workloads at concurrency 1, 10, and 50 for at least five independent repetitions with randomized candidate order |
| [G0-008-B](../tasks/00_g0_gate_close/G0-008-B-run-fresh-process-cold-start-measurements-for-25-1-000-and-10-000-routes-with-ra.md) | P1 | implement | PASS | G0-008-A | Run fresh-process cold-start measurements for 25, 1,000, and 10,000 routes with randomized order and the required sample count |
| [G0-008-C](../tasks/00_g0_gate_close/G0-008-C-capture-cpu-rss-errors-p50-p95-p99-binary-pack-hashes-machine-state-and-load-gen.md) | P1 | implement | PASS | G0-008-B | Capture CPU, RSS, errors, p50/p95/p99, binary/pack hashes, machine state, and load-generator configuration |
| [G0-008-D](../tasks/00_g0_gate_close/G0-008-D-capture-allocation-startup-profiles-including-the-10-000-route-json-pack-parsing.md) | P1 | implement | PASS | G0-008-C | Capture allocation/startup profiles, including the 10,000-route JSON-pack parsing cost |
| [G0-008-E](../tasks/00_g0_gate_close/G0-008-E-generate-markdown-reports-from-raw-data-and-make-verification-fail-when-raw-repo.md) | P1 | implement | PASS | G0-008-D | Generate Markdown reports from raw data and make verification fail when raw/report values diverge |
| [G0-008-V](../tasks/00_g0_gate_close/G0-008-V-verify-close-canonical-performance-evidence.md) | P1 | verify | PASS | G0-008-A, G0-008-B, G0-008-C, G0-008-D, G0-008-E | Verify Close canonical performance evidence |
| [G0-008-Z](../tasks/00_g0_gate_close/G0-008-Z-package-evidence-for-close-canonical-performance-evidence.md) | P1 | evidence | PASS | G0-008-V | Package evidence for Close canonical performance evidence |
| [G0-009-A](../tasks/00_g0_gate_close/G0-009-A-generate-review-index-and-evidence-index-only-after-the-source-commit-is-fixed-a.md) | P1 | implement | PASS | G0-001-Z, G0-008-Z | Generate REVIEW_INDEX and EVIDENCE_INDEX only after the source commit is fixed, and bind them to that exact commit |
| [G0-009-B](../tasks/00_g0_gate_close/G0-009-B-replace-placeholder-pending-commit-references-in-every-pass-task-with-concrete-c.md) | P1 | implement | PASS | G0-009-A | Replace placeholder/PENDING commit references in every PASS task with concrete commit, source, tests, raw evidence, report, and artifact hashes |
| [G0-009-C](../tasks/00_g0_gate_close/G0-009-C-extend-the-production-beta-ledger-validator-to-verify-evidence-paths-test-names.md) | P1 | implement | PASS | G0-009-B | Extend the production/beta ledger validator to verify evidence paths, test names, dependency status, commit format, and artifact hashes for PASS tasks |
| [G0-009-D](../tasks/00_g0_gate_close/G0-009-D-update-the-beta-baseline-and-g0-task-ledger-so-all-sources-of-truth-agree.md) | P1 | implement | PASS | G0-009-C | Update the beta baseline and G0 task ledger so all sources of truth agree |
| [G0-009-E](../tasks/00_g0_gate_close/G0-009-E-generate-a-current-only-release-packet-whose-internal-sha256sum-c-passes.md) | P1 | implement | PASS | G0-009-D | Generate a current-only release packet whose internal sha256sum -c passes |
| [G0-009-V](../tasks/00_g0_gate_close/G0-009-V-verify-create-self-verifying-milestone-and-evidence-indexes.md) | P1 | verify | PASS | G0-009-A, G0-009-B, G0-009-C, G0-009-D, G0-009-E | Verify Create self-verifying milestone and evidence indexes |
| [G0-009-Z](../tasks/00_g0_gate_close/G0-009-Z-package-evidence-for-create-self-verifying-milestone-and-evidence-indexes.md) | P1 | evidence | PASS | G0-009-V | Package evidence for Create self-verifying milestone and evidence indexes |
| [G0-GATE](../gates/G0-GATE.md) | P0 | gate | PASS | G0-001-Z, G0-002-Z, G0-003-Z, G0-004-Z, G0-005-Z, G0-006-Z, G0-007-Z, G0-008-Z, G0-009-Z | M23R2 Gate Closure — Trusted Numeric Artifact and Router exit gate |

## M24 — Zero-Copy Ingress and Worker-Local Request Bridge

| ID | P | Kind | Status | Dependencies | Task |
|---|---:|---|---|---|---|
| [M24-001-A](../tasks/01_m24_zero_copy_ingress/M24-001-A-accept-an-adr-with-ownership-diagrams-and-terminal-invariants.md) | P0 | implement | PASS | G0-GATE | Accept an ADR with ownership diagrams and terminal invariants |
| [M24-001-B](../tasks/01_m24_zero_copy_ingress/M24-001-B-specify-body-ownership-queue-admission-disconnect-cancellation-and-request-slot.md) | P0 | implement | PASS | M24-001-A | Specify body ownership, queue admission, disconnect cancellation, and request-slot lifecycle |
| [M24-001-C](../tasks/01_m24_zero_copy_ingress/M24-001-C-define-no-copy-and-bounded-copy-boundaries.md) | P0 | implement | PASS | M24-001-B | Define no-copy and bounded-copy boundaries |
| [M24-001-D](../tasks/01_m24_zero_copy_ingress/M24-001-D-define-overload-responses-and-metrics.md) | P0 | implement | PASS | M24-001-C | Define overload responses and metrics |
| [M24-001-V](../tasks/01_m24_zero_copy_ingress/M24-001-V-verify-freeze-ingress-ownership-and-backpressure-design.md) | P0 | verify | PASS | M24-001-A, M24-001-B, M24-001-C, M24-001-D | Verify Freeze ingress ownership and backpressure design |
| [M24-001-Z](../tasks/01_m24_zero_copy_ingress/M24-001-Z-package-evidence-for-freeze-ingress-ownership-and-backpressure-design.md) | P0 | evidence | PASS | M24-001-V | Package evidence for Freeze ingress ownership and backpressure design |
| [M24-002-A](../tasks/01_m24_zero_copy_ingress/M24-002-A-keep-method-uri-headermap-and-body-stream-in-native-forms.md) | P0 | implement | PASS | M24-001-Z | Keep Method, Uri, HeaderMap, and body stream in native forms |
| [M24-002-B](../tasks/01_m24_zero_copy_ingress/M24-002-B-match-routeid-using-method-path-before-creating-request-metadata.md) | P0 | implement | PASS | M24-002-A | Match RouteId using method/path before creating request metadata |
| [M24-002-C](../tasks/01_m24_zero_copy_ingress/M24-002-C-read-fieldneeds-from-routeplan.md) | P0 | implement | PASS | M24-002-B | Read FieldNeeds from RoutePlan |
| [M24-002-D](../tasks/01_m24_zero_copy_ingress/M24-002-D-bypass-request-object-creation-for-policy-free-routes-that-need-no-request-field.md) | P0 | implement | PASS | M24-002-C | Bypass request-object creation for policy-free routes that need no request fields |
| [M24-002-V](../tasks/01_m24_zero_copy_ingress/M24-002-V-verify-route-before-request-materialization.md) | P0 | verify | PASS | M24-002-A, M24-002-B, M24-002-C, M24-002-D | Verify Route before request materialization |
| [M24-002-Z](../tasks/01_m24_zero_copy_ingress/M24-002-Z-package-evidence-for-route-before-request-materialization.md) | P0 | evidence | PASS | M24-002-V | Package evidence for Route before request materialization |
| [M24-003-A](../tasks/01_m24_zero_copy_ingress/M24-003-A-move-request-slots-into-each-quickjs-worker.md) | P0 | implement | PASS | M24-001-Z, M24-002-Z | Move request slots into each QuickJS worker |
| [M24-003-B](../tasks/01_m24_zero_copy_ingress/M24-003-B-use-slot-plus-generation-handles.md) | P0 | implement | PASS | M24-003-A | Use slot plus generation handles |
| [M24-003-C](../tasks/01_m24_zero_copy_ingress/M24-003-C-invalidate-at-settlement-timeout-cancellation-quarantine-and-shutdown.md) | P0 | implement | PASS | M24-003-B | Invalidate at settlement, timeout, cancellation, quarantine, and shutdown |
| [M24-003-D](../tasks/01_m24_zero_copy_ingress/M24-003-D-reject-stale-or-cross-worker-handles-deterministically.md) | P0 | implement | PASS | M24-003-C | Reject stale or cross-worker handles deterministically |
| [M24-003-V](../tasks/01_m24_zero_copy_ingress/M24-003-V-verify-implement-worker-local-generation-checked-request-slab.md) | P0 | verify | PASS | M24-003-A, M24-003-B, M24-003-C, M24-003-D | Verify Implement worker-local generation-checked request slab |
| [M24-003-Z](../tasks/01_m24_zero_copy_ingress/M24-003-Z-package-evidence-for-implement-worker-local-generation-checked-request-slab.md) | P0 | evidence | PASS | M24-003-V | Package evidence for Implement worker-local generation-checked request slab |
| [M24-004-A](../tasks/01_m24_zero_copy_ingress/M24-004-A-store-capture-start-end-ranges-against-the-uri-path.md) | P1 | implement | PASS | M24-002-Z, M24-003-Z | Store capture start/end ranges against the URI path |
| [M24-004-B](../tasks/01_m24_zero_copy_ingress/M24-004-B-bind-route-specific-parameter-names-after-routeid-selection.md) | P1 | implement | PASS | M24-004-A | Bind route-specific parameter names after RouteId selection |
| [M24-004-C](../tasks/01_m24_zero_copy_ingress/M24-004-C-validate-numeric-uuid-formats-directly-from-bytes-where-possible.md) | P1 | implement | PASS | M24-004-B | Validate numeric/UUID formats directly from bytes where possible |
| [M24-004-D](../tasks/01_m24_zero_copy_ingress/M24-004-D-materialize-js-strings-lazily.md) | P1 | implement | PASS | M24-004-C | Materialize JS strings lazily |
| [M24-004-V](../tasks/01_m24_zero_copy_ingress/M24-004-V-verify-capture-path-parameters-as-byte-ranges.md) | P1 | verify | PASS | M24-004-A, M24-004-B, M24-004-C, M24-004-D | Verify Capture path parameters as byte ranges |
| [M24-004-Z](../tasks/01_m24_zero_copy_ingress/M24-004-Z-package-evidence-for-capture-path-parameters-as-byte-ranges.md) | P1 | evidence | PASS | M24-004-V | Package evidence for Capture path parameters as byte ranges |
| [M24-005-A](../tasks/01_m24_zero_copy_ingress/M24-005-A-compile-header-name-ids-into-routeplan.md) | P0 | implement | PASS | M24-003-Z | Compile header-name IDs into RoutePlan |
| [M24-005-B](../tasks/01_m24_zero_copy_ingress/M24-005-B-read-header-values-by-id-on-demand.md) | P0 | implement | PASS | M24-005-A | Read header values by ID on demand |
| [M24-005-C](../tasks/01_m24_zero_copy_ingress/M24-005-C-define-duplicate-header-behavior-and-byte-string-conversion.md) | P0 | implement | PASS | M24-005-B | Define duplicate header behavior and byte/string conversion |
| [M24-005-D](../tasks/01_m24_zero_copy_ingress/M24-005-D-keep-full-headers-escape-hatch-explicit-and-costed.md) | P0 | implement | PASS | M24-005-C | Keep full Headers escape hatch explicit and costed |
| [M24-005-V](../tasks/01_m24_zero_copy_ingress/M24-005-V-verify-implement-declared-header-lazy-access.md) | P0 | verify | PASS | M24-005-A, M24-005-B, M24-005-C, M24-005-D | Verify Implement declared-header lazy access |
| [M24-005-Z](../tasks/01_m24_zero_copy_ingress/M24-005-Z-package-evidence-for-implement-declared-header-lazy-access.md) | P0 | evidence | PASS | M24-005-V | Package evidence for Implement declared-header lazy access |
| [M24-006-A](../tasks/01_m24_zero_copy_ingress/M24-006-A-compile-query-cookie-field-ids.md) | P1 | implement | PASS | M24-003-Z, M24-004-Z | Compile query/cookie field IDs |
| [M24-006-B](../tasks/01_m24_zero_copy_ingress/M24-006-B-provide-repeated-key-policy.md) | P1 | implement | PASS | M24-006-A | Provide repeated-key policy |
| [M24-006-C](../tasks/01_m24_zero_copy_ingress/M24-006-C-define-percent-decoding-and-invalid-byte-behavior.md) | P1 | implement | PASS | M24-006-B | Define percent decoding and invalid-byte behavior |
| [M24-006-D](../tasks/01_m24_zero_copy_ingress/M24-006-D-cache-decoded-fields-per-request-slot.md) | P1 | implement | PASS | M24-006-C | Cache decoded fields per request slot |
| [M24-006-V](../tasks/01_m24_zero_copy_ingress/M24-006-V-verify-implement-lazy-query-and-cookie-decoding.md) | P1 | verify | PASS | M24-006-A, M24-006-B, M24-006-C, M24-006-D | Verify Implement lazy query and cookie decoding |
| [M24-006-Z](../tasks/01_m24_zero_copy_ingress/M24-006-Z-package-evidence-for-implement-lazy-query-and-cookie-decoding.md) | P1 | evidence | PASS | M24-006-V | Package evidence for Implement lazy query and cookie decoding |
| [M24-007-A](../tasks/01_m24_zero_copy_ingress/M24-007-A-drive-body-behavior-from-routeplan-not-http-method.md) | P0 | implement | PASS | M24-001-Z, M24-003-Z | Drive body behavior from RoutePlan, not HTTP method |
| [M24-007-B](../tasks/01_m24_zero_copy_ingress/M24-007-B-use-bytes-and-avoid-bytes-to-vec-copies.md) | P0 | implement | PASS | M24-007-A | Use Bytes and avoid Bytes-to-Vec copies |
| [M24-007-C](../tasks/01_m24_zero_copy_ingress/M24-007-C-enforce-content-length-and-streaming-limits.md) | P0 | implement | PASS | M24-007-B | Enforce content length and streaming limits |
| [M24-007-D](../tasks/01_m24_zero_copy_ingress/M24-007-D-cache-one-decoded-representation-and-reject-incompatible-second-reads.md) | P0 | implement | PASS | M24-007-C | Cache one decoded representation and reject incompatible second reads |
| [M24-007-V](../tasks/01_m24_zero_copy_ingress/M24-007-V-verify-implement-bounded-read-once-body-admission.md) | P0 | verify | PASS | M24-007-A, M24-007-B, M24-007-C, M24-007-D | Verify Implement bounded read-once body admission |
| [M24-007-Z](../tasks/01_m24_zero_copy_ingress/M24-007-Z-package-evidence-for-implement-bounded-read-once-body-admission.md) | P0 | evidence | PASS | M24-007-V | Package evidence for Implement bounded read-once body admission |
| [M24-008-A](../tasks/01_m24_zero_copy_ingress/M24-008-A-create-shared-context-request-prototypes-or-native-classes.md) | P1 | implement | PASS | M24-003-Z, M24-005-Z, M24-006-Z, M24-007-Z | Create shared Context/Request prototypes or native classes |
| [M24-008-B](../tasks/01_m24_zero_copy_ingress/M24-008-B-store-only-opaque-handle-and-route-plan-references-per-request.md) | P1 | implement | PASS | M24-008-A | Store only opaque handle and route plan references per request |
| [M24-008-C](../tasks/01_m24_zero_copy_ingress/M24-008-C-cache-native-capability-objects.md) | P1 | implement | PASS | M24-008-B | Cache native capability objects |
| [M24-008-D](../tasks/01_m24_zero_copy_ingress/M24-008-D-keep-full-web-request-construction-as-explicit-fallback.md) | P1 | implement | PASS | M24-008-C | Keep full Web Request construction as explicit fallback |
| [M24-008-V](../tasks/01_m24_zero_copy_ingress/M24-008-V-verify-replace-per-request-js-closures-with-native-backed-prototypes.md) | P1 | verify | PASS | M24-008-A, M24-008-B, M24-008-C, M24-008-D | Verify Replace per-request JS closures with native-backed prototypes |
| [M24-008-Z](../tasks/01_m24_zero_copy_ingress/M24-008-Z-package-evidence-for-replace-per-request-js-closures-with-native-backed-prototyp.md) | P1 | evidence | PASS | M24-008-V | Package evidence for Replace per-request JS closures with native-backed prototypes |
| [M24-009-A](../tasks/01_m24_zero_copy_ingress/M24-009-A-add-counters-histograms-for-route-queue-decode-bridge-js-encode-and-write-stages.md) | P1 | implement | PASS | M24-002-Z, M24-003-Z | Add counters/histograms for route, queue, decode, bridge, JS, encode, and write stages |
| [M24-009-B](../tasks/01_m24_zero_copy_ingress/M24-009-B-use-disabled-by-default-or-sampled-recording.md) | P1 | implement | PASS | M24-009-A | Use disabled-by-default or sampled recording |
| [M24-009-C](../tasks/01_m24_zero_copy_ingress/M24-009-C-expose-slab-queue-body-gauges.md) | P1 | implement | PASS | M24-009-B | Expose slab/queue/body gauges |
| [M24-009-D](../tasks/01_m24_zero_copy_ingress/M24-009-D-measure-instrumentation-overhead.md) | P1 | implement | PASS | M24-009-C | Measure instrumentation overhead |
| [M24-009-V](../tasks/01_m24_zero_copy_ingress/M24-009-V-verify-add-ingress-and-bridge-observability.md) | P1 | verify | PASS | M24-009-A, M24-009-B, M24-009-C, M24-009-D | Verify Add ingress and bridge observability |
| [M24-009-Z](../tasks/01_m24_zero_copy_ingress/M24-009-Z-package-evidence-for-add-ingress-and-bridge-observability.md) | P1 | evidence | PASS | M24-009-V | Package evidence for Add ingress and bridge observability |
| [M24-010-A](../tasks/01_m24_zero_copy_ingress/M24-010-A-fuzz-paths-queries-headers-cookies-bodies-handles-and-cancellation-orderings.md) | P0 | implement | PASS | M24-004-Z, M24-005-Z, M24-006-Z, M24-007-Z, M24-008-Z | Fuzz paths, queries, headers, cookies, bodies, handles, and cancellation orderings |
| [M24-010-B](../tasks/01_m24_zero_copy_ingress/M24-010-B-differentially-compare-legacy-reference-decoding-where-applicable.md) | P0 | implement | PASS | M24-010-A | Differentially compare legacy/reference decoding where applicable |
| [M24-010-C](../tasks/01_m24_zero_copy_ingress/M24-010-C-run-property-tests-for-slot-lifecycle.md) | P0 | implement | PASS | M24-010-B | Run property tests for slot lifecycle |
| [M24-010-D](../tasks/01_m24_zero_copy_ingress/M24-010-D-capture-and-minimize-failures.md) | P0 | implement | PASS | M24-010-C | Capture and minimize failures |
| [M24-010-V](../tasks/01_m24_zero_copy_ingress/M24-010-V-verify-complete-ingress-bridge-fuzzing-and-conformance.md) | P0 | verify | PASS | M24-010-A, M24-010-B, M24-010-C, M24-010-D | Verify Complete ingress bridge fuzzing and conformance |
| [M24-010-Z](../tasks/01_m24_zero_copy_ingress/M24-010-Z-package-evidence-for-complete-ingress-bridge-fuzzing-and-conformance.md) | P0 | evidence | PASS | M24-010-V | Package evidence for Complete ingress bridge fuzzing and conformance |
| [M24-GATE](../gates/M24-GATE.md) | P0 | gate | PASS | M24-001-Z, M24-002-Z, M24-003-Z, M24-004-Z, M24-005-Z, M24-006-Z, M24-007-Z, M24-008-Z, M24-009-Z, M24-010-Z | M2.4 — Zero-Copy Ingress and Worker-Local Request Bridge exit gate |

## M25 — Schema-Specialized Validation and JSON Codecs

| ID | P | Kind | Status | Dependencies | Task |
|---|---:|---|---|---|---|
| [M25-001-A](../tasks/02_m25_schema_codecs/M25-001-A-specify-objects-arrays-unions-literals-enums-formats-defaults-optional-null-tran.md) | P0 | implement | PASS | M24-GATE | Specify objects, arrays, unions, literals, enums, formats, defaults, optional/null, transforms, files, and problem schemas |
| [M25-001-B](../tasks/02_m25_schema_codecs/M25-001-B-define-compatibility-and-fallback-markers.md) | P0 | implement | PASS | M25-001-A | Define compatibility and fallback markers |
| [M25-001-C](../tasks/02_m25_schema_codecs/M25-001-C-canonicalize-ordering-and-hashing.md) | P0 | implement | PASS | M25-001-B | Canonicalize ordering and hashing |
| [M25-001-D](../tasks/02_m25_schema_codecs/M25-001-D-document-unsupported-transformations.md) | P0 | implement | PASS | M25-001-C | Document unsupported transformations |
| [M25-001-V](../tasks/02_m25_schema_codecs/M25-001-V-verify-define-canonical-schema-ir-v2.md) | P0 | verify | PASS | M25-001-A, M25-001-B, M25-001-C, M25-001-D | Verify Define canonical Schema IR v2 |
| [M25-001-Z](../tasks/02_m25_schema_codecs/M25-001-Z-package-evidence-for-define-canonical-schema-ir-v2.md) | P0 | evidence | PASS | M25-001-V | Package evidence for Define canonical Schema IR v2 |
| [M25-002-A](../tasks/02_m25_schema_codecs/M25-002-A-compare-quickjs-parse-stringify-generic-rust-conversion-and-generated-schema-awa.md) | P1 | implement | PASS | M25-001-Z | Compare QuickJS parse/stringify, generic Rust conversion, and generated schema-aware codecs |
| [M25-002-B](../tasks/02_m25_schema_codecs/M25-002-B-use-256b-1kb-16kb-64kb-nested-objects-arrays-100-1-000-optional-null-and-problem.md) | P1 | implement | PASS | M25-002-A | Use 256B, 1KB, 16KB, 64KB, nested objects, arrays 100/1,000, optional/null, and problems |
| [M25-002-C](../tasks/02_m25_schema_codecs/M25-002-C-capture-cpu-allocation-bridge-time-and-tails.md) | P1 | implement | PASS | M25-002-B | Capture CPU, allocation, bridge time, and tails |
| [M25-002-D](../tasks/02_m25_schema_codecs/M25-002-D-select-strategies-by-evidence.md) | P1 | implement | PASS | M25-002-C | Select strategies by evidence |
| [M25-002-V](../tasks/02_m25_schema_codecs/M25-002-V-verify-build-reproducible-decoder-encoder-strategy-benchmark.md) | P1 | verify | PASS | M25-002-A, M25-002-B, M25-002-C, M25-002-D | Verify Build reproducible decoder/encoder strategy benchmark |
| [M25-002-Z](../tasks/02_m25_schema_codecs/M25-002-Z-package-evidence-for-build-reproducible-decoder-encoder-strategy-benchmark.md) | P1 | evidence | PASS | M25-002-V | Package evidence for Build reproducible decoder/encoder strategy benchmark |
| [M25-003-A](../tasks/02_m25_schema_codecs/M25-003-A-generate-direct-decoder-programs-keyed-by-schemaid.md) | P0 | implement | PASS | M25-001-Z, M24-GATE | Generate direct decoder programs keyed by SchemaId |
| [M25-003-B](../tasks/02_m25_schema_codecs/M25-003-B-validate-byte-ranges-and-header-query-values-without-generic-object-trees.md) | P0 | implement | PASS | M25-003-A | Validate byte ranges and header/query values without generic object trees |
| [M25-003-C](../tasks/02_m25_schema_codecs/M25-003-C-return-typed-rfc-9457-problems.md) | P0 | implement | PASS | M25-003-B | Return typed RFC 9457 problems |
| [M25-003-D](../tasks/02_m25_schema_codecs/M25-003-D-preserve-declared-coercion-semantics-exactly.md) | P0 | implement | PASS | M25-003-C | Preserve declared coercion semantics exactly |
| [M25-003-V](../tasks/02_m25_schema_codecs/M25-003-V-verify-generate-params-query-header-decoders.md) | P0 | verify | PASS | M25-003-A, M25-003-B, M25-003-C, M25-003-D | Verify Generate params/query/header decoders |
| [M25-003-Z](../tasks/02_m25_schema_codecs/M25-003-Z-package-evidence-for-generate-params-query-header-decoders.md) | P0 | evidence | PASS | M25-003-V | Package evidence for Generate params/query/header decoders |
| [M25-004-A](../tasks/02_m25_schema_codecs/M25-004-A-implement-generated-direct-decode-where-supported.md) | P0 | implement | PASS | M25-001-Z, M24-007-Z | Implement generated direct decode where supported |
| [M25-004-B](../tasks/02_m25_schema_codecs/M25-004-B-retain-quickjs-generic-fallback-for-unsupported-transformations.md) | P0 | implement | PASS | M25-004-A | Retain QuickJS/generic fallback for unsupported transformations |
| [M25-004-C](../tasks/02_m25_schema_codecs/M25-004-C-enforce-depth-size-array-string-and-numeric-limits.md) | P0 | implement | PASS | M25-004-B | Enforce depth, size, array, string, and numeric limits |
| [M25-004-D](../tasks/02_m25_schema_codecs/M25-004-D-propagate-cancellation-and-request-deadlines.md) | P0 | implement | PASS | M25-004-C | Propagate cancellation and request deadlines |
| [M25-004-V](../tasks/02_m25_schema_codecs/M25-004-V-verify-generate-json-body-decoders.md) | P0 | verify | PASS | M25-004-A, M25-004-B, M25-004-C, M25-004-D | Verify Generate JSON body decoders |
| [M25-004-Z](../tasks/02_m25_schema_codecs/M25-004-Z-package-evidence-for-generate-json-body-decoders.md) | P0 | evidence | PASS | M25-004-V | Package evidence for Generate JSON body decoders |
| [M25-005-A](../tasks/02_m25_schema_codecs/M25-005-A-generate-per-status-encoders.md) | P0 | implement | PASS | M25-001-Z, M25-002-Z | Generate per-status encoders |
| [M25-005-B](../tasks/02_m25_schema_codecs/M25-005-B-read-declared-properties-in-fixed-order.md) | P0 | implement | PASS | M25-005-A | Read declared properties in fixed order |
| [M25-005-C](../tasks/02_m25_schema_codecs/M25-005-C-handle-optional-null-union-fields.md) | P0 | implement | PASS | M25-005-B | Handle optional/null/union fields |
| [M25-005-D](../tasks/02_m25_schema_codecs/M25-005-D-keep-quickjs-stringify-or-generic-fallback-when-measured-better.md) | P0 | implement | PASS | M25-005-C | Keep QuickJS stringify or generic fallback when measured better |
| [M25-005-V](../tasks/02_m25_schema_codecs/M25-005-V-verify-generate-status-specific-response-encoders.md) | P0 | verify | PASS | M25-005-A, M25-005-B, M25-005-C, M25-005-D | Verify Generate status-specific response encoders |
| [M25-005-Z](../tasks/02_m25_schema_codecs/M25-005-Z-package-evidence-for-generate-status-specific-response-encoders.md) | P0 | evidence | PASS | M25-005-V | Package evidence for Generate status-specific response encoders |
| [M25-006-A](../tasks/02_m25_schema_codecs/M25-006-A-generate-problem-type-status-title-detail-custom-field-encoders.md) | P0 | implement | PASS | M25-001-Z, M25-005-Z | Generate problem type/status/title/detail/custom-field encoders |
| [M25-006-B](../tasks/02_m25_schema_codecs/M25-006-B-redact-unexpected-failures.md) | P0 | implement | PASS | M25-006-A | Redact unexpected failures |
| [M25-006-C](../tasks/02_m25_schema_codecs/M25-006-C-ensure-policy-provided-errors-flow-into-treaty-unions.md) | P0 | implement | PASS | M25-006-B | Ensure policy-provided errors flow into Treaty unions |
| [M25-006-D](../tasks/02_m25_schema_codecs/M25-006-D-include-content-type-and-instance-behavior.md) | P0 | implement | PASS | M25-006-C | Include content type and instance behavior |
| [M25-006-V](../tasks/02_m25_schema_codecs/M25-006-V-verify-generate-rfc-9457-problem-encoders.md) | P0 | verify | PASS | M25-006-A, M25-006-B, M25-006-C, M25-006-D | Verify Generate RFC 9457 problem encoders |
| [M25-006-Z](../tasks/02_m25_schema_codecs/M25-006-Z-package-evidence-for-generate-rfc-9457-problem-encoders.md) | P0 | evidence | PASS | M25-006-V | Package evidence for Generate RFC 9457 problem encoders |
| [M25-007-A](../tasks/02_m25_schema_codecs/M25-007-A-tag-fallback-reason-in-routeplan.md) | P1 | implement | PASS | M25-003-Z, M25-004-Z, M25-005-Z | Tag fallback reason in RoutePlan |
| [M25-007-B](../tasks/02_m25_schema_codecs/M25-007-B-support-raw-response-full-request-escape-hatches.md) | P1 | implement | PASS | M25-007-A | Support raw Response/full Request escape hatches |
| [M25-007-C](../tasks/02_m25_schema_codecs/M25-007-C-keep-fallback-bounded-and-deadline-aware.md) | P1 | implement | PASS | M25-007-B | Keep fallback bounded and deadline-aware |
| [M25-007-D](../tasks/02_m25_schema_codecs/M25-007-D-expose-bridge-crossings-and-codec-choice-in-velqu-inspect.md) | P1 | implement | PASS | M25-007-C | Expose bridge crossings and codec choice in `velqu inspect` |
| [M25-007-V](../tasks/02_m25_schema_codecs/M25-007-V-verify-implement-explicit-generic-and-web-fallback-paths.md) | P1 | verify | PASS | M25-007-A, M25-007-B, M25-007-C, M25-007-D | Verify Implement explicit generic and Web fallback paths |
| [M25-007-Z](../tasks/02_m25_schema_codecs/M25-007-Z-package-evidence-for-implement-explicit-generic-and-web-fallback-paths.md) | P1 | evidence | PASS | M25-007-V | Package evidence for Implement explicit generic and Web fallback paths |
| [M25-008-A](../tasks/02_m25_schema_codecs/M25-008-A-generate-all-projections-from-canonical-ir.md) | P0 | implement | PASS | M25-001-Z, M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z | Generate all projections from canonical IR |
| [M25-008-B](../tasks/02_m25_schema_codecs/M25-008-B-add-parity-checks-to-verification.md) | P0 | implement | PASS | M25-008-A | Add parity checks to verification |
| [M25-008-C](../tasks/02_m25_schema_codecs/M25-008-C-publish-compact-contract-metadata.md) | P0 | implement | PASS | M25-008-B | Publish compact contract metadata |
| [M25-008-D](../tasks/02_m25_schema_codecs/M25-008-D-update-semantic-diff-to-schema-ir-v2.md) | P0 | implement | PASS | M25-008-C | Update semantic diff to Schema IR v2 |
| [M25-008-V](../tasks/02_m25_schema_codecs/M25-008-V-verify-unify-treaty-openapi-lock-and-runtime-schema-projection.md) | P0 | verify | PASS | M25-008-A, M25-008-B, M25-008-C, M25-008-D | Verify Unify Treaty, OpenAPI, lock, and runtime schema projection |
| [M25-008-Z](../tasks/02_m25_schema_codecs/M25-008-Z-package-evidence-for-unify-treaty-openapi-lock-and-runtime-schema-projection.md) | P0 | evidence | PASS | M25-008-V | Package evidence for Unify Treaty, OpenAPI, lock, and runtime schema projection |
| [M25-009-A](../tasks/02_m25_schema_codecs/M25-009-A-fuzz-encoded-decoded-values.md) | P0 | implement | PASS | M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z | Fuzz encoded/decoded values |
| [M25-009-B](../tasks/02_m25_schema_codecs/M25-009-B-compare-generated-output-with-standards-reference-json-behavior.md) | P0 | implement | PASS | M25-009-A | Compare generated output with standards/reference JSON behavior |
| [M25-009-C](../tasks/02_m25_schema_codecs/M25-009-C-run-malformed-and-boundary-values.md) | P0 | implement | PASS | M25-009-B | Run malformed and boundary values |
| [M25-009-D](../tasks/02_m25_schema_codecs/M25-009-D-minimize-failures-into-permanent-fixtures.md) | P0 | implement | PASS | M25-009-C | Minimize failures into permanent fixtures |
| [M25-009-V](../tasks/02_m25_schema_codecs/M25-009-V-verify-add-codec-fuzzing-and-differential-tests.md) | P0 | verify | PASS | M25-009-A, M25-009-B, M25-009-C, M25-009-D | Verify Add codec fuzzing and differential tests |
| [M25-009-Z](../tasks/02_m25_schema_codecs/M25-009-Z-package-evidence-for-add-codec-fuzzing-and-differential-tests.md) | P0 | evidence | PASS | M25-009-V | Package evidence for Add codec fuzzing and differential tests |
| [M25-010-A](../tasks/02_m25_schema_codecs/M25-010-A-run-c2-plus-medium-large-json-workloads.md) | P1 | implement | PASS | M25-002-Z, M25-009-Z | Run C2 plus medium/large JSON workloads |
| [M25-010-B](../tasks/02_m25_schema_codecs/M25-010-B-measure-generated-code-pack-size.md) | P1 | implement | PASS | M25-010-A | Measure generated code/pack size |
| [M25-010-C](../tasks/02_m25_schema_codecs/M25-010-C-report-cold-start-delta-at-25-1-000-routes.md) | P1 | implement | PASS | M25-010-B | Report cold-start delta at 25/1,000 routes |
| [M25-010-D](../tasks/02_m25_schema_codecs/M25-010-D-record-cpu-and-rss.md) | P1 | implement | PASS | M25-010-C | Record CPU and RSS |
| [M25-010-V](../tasks/02_m25_schema_codecs/M25-010-V-verify-close-codec-performance-and-cold-start-evidence.md) | P1 | verify | PASS | M25-010-A, M25-010-B, M25-010-C, M25-010-D | Verify Close codec performance and cold-start evidence |
| [M25-010-Z](../tasks/02_m25_schema_codecs/M25-010-Z-package-evidence-for-close-codec-performance-and-cold-start-evidence.md) | P1 | evidence | PASS | M25-010-V | Package evidence for Close codec performance and cold-start evidence |
| [M25-GATE](../gates/M25-GATE.md) | P0 | gate | PASS | M25-001-Z, M25-002-Z, M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z, M25-007-Z, M25-008-Z, M25-009-Z, M25-010-Z | M2.5 — Schema-Specialized Input and JSON Output Pipeline exit gate |

## M26 — Binary QPack v2 and Reproducible Artifact ABI

| ID | P | Kind | Status | Dependencies | Task |
|---|---:|---|---|---|---|
| [M26-001-A](../tasks/03_m26_qpack_v2/M26-001-A-define-numeric-current-mode-and-legacy-v1-adapter.md) | P0 | implement | PASS | M25-GATE | Define numeric current mode and legacy v1 adapter |
| [M26-001-B](../tasks/03_m26_qpack_v2/M26-001-B-specify-section-directory-alignment-bounds-optional-sections-and-versioning.md) | P0 | implement | PASS | M26-001-A | Specify section directory, alignment, bounds, optional sections, and versioning |
| [M26-001-C](../tasks/03_m26_qpack_v2/M26-001-C-separate-integrity-from-authenticity.md) | P0 | implement | PASS | M26-001-B | Separate integrity from authenticity |
| [M26-001-D](../tasks/03_m26_qpack_v2/M26-001-D-define-debug-source-sidecar-policy.md) | P0 | implement | PASS | M26-001-C | Define debug/source sidecar policy |
| [M26-001-V](../tasks/03_m26_qpack_v2/M26-001-V-verify-accept-qpack-v2-format-and-compatibility-adr.md) | P0 | verify | PASS | M26-001-A, M26-001-B, M26-001-C, M26-001-D | Verify Accept QPack v2 format and compatibility ADR |
| [M26-001-Z](../tasks/03_m26_qpack_v2/M26-001-Z-package-evidence-for-accept-qpack-v2-format-and-compatibility-adr.md) | P0 | evidence | PASS | M26-001-V | Package evidence for Accept QPack v2 format and compatibility ADR |
| [M26-002-A](../tasks/03_m26_qpack_v2/M26-002-A-include-runtime-abi-quickjs-ng-version-build-hash-rquickjs-version-bytecode-form.md) | P0 | implement | PASS | M26-001-Z | Include runtime ABI, QuickJS-NG version/build hash, rquickjs version, bytecode format, target triple, pointer width, endianness, and capability hash |
| [M26-002-B](../tasks/03_m26_qpack_v2/M26-002-B-fail-closed-on-mismatch.md) | P0 | implement | PASS | M26-002-A | Fail closed on mismatch |
| [M26-002-C](../tasks/03_m26_qpack_v2/M26-002-C-provide-explicit-source-rebuild-path.md) | P0 | implement | PASS | M26-002-B | Provide explicit source rebuild path |
| [M26-002-D](../tasks/03_m26_qpack_v2/M26-002-D-never-silently-fall-back.md) | P0 | implement | PASS | M26-002-C | Never silently fall back |
| [M26-002-V](../tasks/03_m26_qpack_v2/M26-002-V-verify-define-strict-runtime-and-bytecode-fingerprint.md) | P0 | verify | PASS | M26-002-A, M26-002-B, M26-002-C, M26-002-D | Verify Define strict runtime and bytecode fingerprint |
| [M26-002-Z](../tasks/03_m26_qpack_v2/M26-002-Z-package-evidence-for-define-strict-runtime-and-bytecode-fingerprint.md) | P0 | evidence | PASS | M26-002-V | Package evidence for Define strict runtime and bytecode fingerprint |
| [M26-003-A](../tasks/03_m26_qpack_v2/M26-003-A-define-dense-section-schemas.md) | P0 | implement | PASS | M26-001-Z, G0-GATE, M25-GATE | Define dense section schemas |
| [M26-003-B](../tasks/03_m26_qpack_v2/M26-003-B-store-router-nodes-edges-terminals-routeplans-schema-programs-policy-plans-funct.md) | P0 | implement | PASS | M26-003-A | Store router nodes/edges/terminals, RoutePlans, schema programs, policy plans, function manifest, debug names, and capability inventory |
| [M26-003-C](../tasks/03_m26_qpack_v2/M26-003-C-use-offsets-and-bounds-checks.md) | P0 | implement | PASS | M26-003-B | Use offsets and bounds checks |
| [M26-003-D](../tasks/03_m26_qpack_v2/M26-003-D-bind-sections-to-execution-integrity.md) | P0 | implement | PASS | M26-003-C | Bind sections to execution integrity |
| [M26-003-V](../tasks/03_m26_qpack_v2/M26-003-V-verify-encode-compiled-router-routeplans-schemas-policies-and-functions-as-secti.md) | P0 | verify | PASS | M26-003-A, M26-003-B, M26-003-C, M26-003-D | Verify Encode compiled router, RoutePlans, schemas, policies, and functions as sections |
| [M26-003-Z](../tasks/03_m26_qpack_v2/M26-003-Z-package-evidence-for-encode-compiled-router-routeplans-schemas-policies-and-func.md) | P0 | evidence | PASS | M26-003-V | Package evidence for Encode compiled router, RoutePlans, schemas, policies, and functions as sections |
| [M26-004-A](../tasks/03_m26_qpack_v2/M26-004-A-store-raw-module-bytecode-section.md) | P0 | implement | PASS | M26-002-Z, M26-003-Z | Store raw module bytecode section |
| [M26-004-B](../tasks/03_m26_qpack_v2/M26-004-B-load-exactly-once.md) | P0 | implement | PASS | M26-004-A | Load exactly once |
| [M26-004-C](../tasks/03_m26_qpack_v2/M26-004-C-make-source-optional-sidecar-development-section.md) | P0 | implement | PASS | M26-004-B | Make source optional sidecar/development section |
| [M26-004-D](../tasks/03_m26_qpack_v2/M26-004-D-include-prelude-and-handler-manifest-in-the-compiled-module.md) | P0 | implement | PASS | M26-004-C | Include prelude and handler manifest in the compiled module |
| [M26-004-V](../tasks/03_m26_qpack_v2/M26-004-V-verify-embed-raw-quickjs-bytecode-without-base64.md) | P0 | verify | PASS | M26-004-A, M26-004-B, M26-004-C, M26-004-D | Verify Embed raw QuickJS bytecode without base64 |
| [M26-004-Z](../tasks/03_m26_qpack_v2/M26-004-Z-package-evidence-for-embed-raw-quickjs-bytecode-without-base64.md) | P0 | evidence | PASS | M26-004-V | Package evidence for Embed raw QuickJS bytecode without base64 |
| [M26-005-A](../tasks/03_m26_qpack_v2/M26-005-A-use-mmap-read-only-bytes-where-supported.md) | P0 | implement | PASS | M26-003-Z | Use mmap/read-only bytes where supported |
| [M26-005-B](../tasks/03_m26_qpack_v2/M26-005-B-validate-all-section-bounds-before-access.md) | P0 | implement | PASS | M26-005-A | Validate all section bounds before access |
| [M26-005-C](../tasks/03_m26_qpack_v2/M26-005-C-avoid-unsafe-unchecked-access-unless-independently-audited.md) | P0 | implement | PASS | M26-005-B | Avoid unsafe unchecked access unless independently audited |
| [M26-005-D](../tasks/03_m26_qpack_v2/M26-005-D-support-embedded-pack-bytes-in-standalone-binary.md) | P0 | implement | PASS | M26-005-C | Support embedded pack bytes in standalone binary |
| [M26-005-V](../tasks/03_m26_qpack_v2/M26-005-V-verify-implement-zero-copy-or-bounded-copy-pack-reader.md) | P0 | verify | PASS | M26-005-A, M26-005-B, M26-005-C, M26-005-D | Verify Implement zero-copy or bounded-copy pack reader |
| [M26-005-Z](../tasks/03_m26_qpack_v2/M26-005-Z-package-evidence-for-implement-zero-copy-or-bounded-copy-pack-reader.md) | P0 | evidence | PASS | M26-005-V | Package evidence for Implement zero-copy or bounded-copy pack reader |
| [M26-006-A](../tasks/03_m26_qpack_v2/M26-006-A-hash-required-execution-sections.md) | P1 | implement | PASS | M26-003-Z, M26-004-Z | Hash required execution sections |
| [M26-006-B](../tasks/03_m26_qpack_v2/M26-006-B-provide-ed25519-compatible-signature-slot-hook.md) | P1 | implement | PASS | M26-006-A | Provide Ed25519-compatible signature slot/hook |
| [M26-006-C](../tasks/03_m26_qpack_v2/M26-006-C-define-key-discovery-configuration.md) | P1 | implement | PASS | M26-006-B | Define key discovery/configuration |
| [M26-006-D](../tasks/03_m26_qpack_v2/M26-006-D-keep-unsigned-local-development-supported-with-explicit-policy.md) | P1 | implement | PASS | M26-006-C | Keep unsigned local development supported with explicit policy |
| [M26-006-V](../tasks/03_m26_qpack_v2/M26-006-V-verify-implement-execution-integrity-and-authenticity-hooks.md) | P1 | verify | PASS | M26-006-A, M26-006-B, M26-006-C, M26-006-D | Verify Implement execution integrity and authenticity hooks |
| [M26-006-Z](../tasks/03_m26_qpack_v2/M26-006-Z-package-evidence-for-implement-execution-integrity-and-authenticity-hooks.md) | P1 | evidence | PASS | M26-006-V | Package evidence for Implement execution integrity and authenticity hooks |
| [M26-007-A](../tasks/03_m26_qpack_v2/M26-007-A-remove-timestamps-non-deterministic-map-order.md) | P1 | implement | PASS | M26-003-Z, M26-004-Z | Remove timestamps/non-deterministic map order |
| [M26-007-B](../tasks/03_m26_qpack_v2/M26-007-B-pin-compiler-runtime-versions.md) | P1 | implement | PASS | M26-007-A | Pin compiler/runtime versions |
| [M26-007-C](../tasks/03_m26_qpack_v2/M26-007-C-canonicalize-section-ordering-and-padding.md) | P1 | implement | PASS | M26-007-B | Canonicalize section ordering and padding |
| [M26-007-D](../tasks/03_m26_qpack_v2/M26-007-D-compare-independent-build-outputs.md) | P1 | implement | PASS | M26-007-C | Compare independent build outputs |
| [M26-007-V](../tasks/03_m26_qpack_v2/M26-007-V-verify-guarantee-reproducible-release-packs.md) | P1 | verify | PASS | M26-007-A, M26-007-B, M26-007-C, M26-007-D | Verify Guarantee reproducible release packs |
| [M26-007-Z](../tasks/03_m26_qpack_v2/M26-007-Z-package-evidence-for-guarantee-reproducible-release-packs.md) | P1 | evidence | PASS | M26-007-V | Package evidence for Guarantee reproducible release packs |
| [M26-008-A](../tasks/03_m26_qpack_v2/M26-008-A-implement-separate-v1-reader-adapter.md) | P1 | implement | PASS | M26-001-Z, M26-005-Z | Implement separate v1 reader/adapter |
| [M26-008-B](../tasks/03_m26_qpack_v2/M26-008-B-provide-velqu-pack-migrate-or-rebuild-guidance.md) | P1 | implement | PASS | M26-008-A | Provide `velqu pack migrate` or rebuild guidance |
| [M26-008-C](../tasks/03_m26_qpack_v2/M26-008-C-deprecate-mixed-mode-packs.md) | P1 | implement | PASS | M26-008-B | Deprecate mixed-mode packs |
| [M26-008-D](../tasks/03_m26_qpack_v2/M26-008-D-test-deterministic-failures-for-unsupported-legacy-features.md) | P1 | implement | PASS | M26-008-C | Test deterministic failures for unsupported legacy features |
| [M26-008-V](../tasks/03_m26_qpack_v2/M26-008-V-verify-provide-explicit-v1-compatibility-and-migration-tool.md) | P1 | verify | PASS | M26-008-A, M26-008-B, M26-008-C, M26-008-D | Verify Provide explicit v1 compatibility and migration tool |
| [M26-008-Z](../tasks/03_m26_qpack_v2/M26-008-Z-package-evidence-for-provide-explicit-v1-compatibility-and-migration-tool.md) | P1 | evidence | PASS | M26-008-V | Package evidence for Provide explicit v1 compatibility and migration tool |
| [M26-009-A](../tasks/03_m26_qpack_v2/M26-009-A-shared-mode-velqu-runtime-plus-app-qpack.md) | P1 | implement | PASS | M26-004-Z, M26-005-Z | Shared mode: `velqu-runtime` plus app.qpack |
| [M26-009-B](../tasks/03_m26_qpack_v2/M26-009-B-standalone-mode-embedded-qpack-executable.md) | P1 | implement | PASS | M26-009-A | Standalone mode: embedded qpack executable |
| [M26-009-C](../tasks/03_m26_qpack_v2/M26-009-C-ensure-exact-runtime-fingerprint.md) | P1 | implement | PASS | M26-009-B | Ensure exact runtime fingerprint |
| [M26-009-D](../tasks/03_m26_qpack_v2/M26-009-D-define-source-map-debug-sidecars.md) | P1 | implement | PASS | M26-009-C | Define source-map/debug sidecars |
| [M26-009-V](../tasks/03_m26_qpack_v2/M26-009-V-verify-build-shared-runtime-and-standalone-deployment-artifacts.md) | P1 | verify | PASS | M26-009-A, M26-009-B, M26-009-C, M26-009-D | Verify Build shared-runtime and standalone deployment artifacts |
| [M26-009-Z](../tasks/03_m26_qpack_v2/M26-009-Z-package-evidence-for-build-shared-runtime-and-standalone-deployment-artifacts.md) | P1 | evidence | PASS | M26-009-V | Package evidence for Build shared-runtime and standalone deployment artifacts |
| [M26-010-A](../tasks/03_m26_qpack_v2/M26-010-A-measure-25-100-1-000-5-000-10-000-routes.md) | P1 | implement | PASS | M26-004-Z, M26-005-Z, M26-009-Z | Measure 25/100/1,000/5,000/10,000 routes |
| [M26-010-B](../tasks/03_m26_qpack_v2/M26-010-B-at-least-100-fresh-processes-for-release-evidence.md) | P1 | implement | PASS | M26-010-A | At least 100 fresh processes for release evidence |
| [M26-010-C](../tasks/03_m26_qpack_v2/M26-010-C-randomize-source-bytecode-competitor-order.md) | P1 | implement | PASS | M26-010-B | Randomize source/bytecode/competitor order |
| [M26-010-D](../tasks/03_m26_qpack_v2/M26-010-D-record-p50-p95-p99-rss-stage-timings-and-hashes.md) | P1 | implement | PASS | M26-010-C | Record p50/p95/p99, RSS, stage timings, and hashes |
| [M26-010-V](../tasks/03_m26_qpack_v2/M26-010-V-verify-close-route-count-cold-start-evidence.md) | P1 | verify | PASS | M26-010-A, M26-010-B, M26-010-C, M26-010-D | Verify Close route-count cold-start evidence |
| [M26-010-Z](../tasks/03_m26_qpack_v2/M26-010-Z-package-evidence-for-close-route-count-cold-start-evidence.md) | P1 | evidence | PASS | M26-010-V | Package evidence for Close route-count cold-start evidence |
| [M26-GATE](../gates/M26-GATE.md) | P0 | gate | PASS | M26-001-Z, M26-002-Z, M26-003-Z, M26-004-Z, M26-005-Z, M26-006-Z, M26-007-Z, M26-008-Z, M26-009-Z, M26-010-Z | M2.6 — Binary QPack v2 and Reproducible Artifact ABI exit gate |

## M27 — Capability Linker and Minimal Web Runtime

| ID | P | Kind | Status | Dependencies | Task |
|---|---:|---|---|---|---|
| [M27-001-A](../tasks/04_m27_capability_linker/M27-001-A-accept-adr.md) | P0 | implement | PASS | M26-GATE | Accept ADR |
| [M27-001-B](../tasks/04_m27_capability_linker/M27-001-B-define-capabilityid-version-dependencies.md) | P0 | implement | PASS | M27-001-A | Define CapabilityId/version/dependencies |
| [M27-001-C](../tasks/04_m27_capability_linker/M27-001-C-define-native-operation-owner-deadline-state.md) | P0 | implement | PASS | M27-001-B | Define native operation owner/deadline state |
| [M27-001-D](../tasks/04_m27_capability_linker/M27-001-D-define-lifecycle-phases-and-bounded-shutdown.md) | P0 | implement | PASS | M27-001-C | Define lifecycle phases and bounded shutdown |
| [M27-001-V](../tasks/04_m27_capability_linker/M27-001-V-verify-define-capability-abi-and-lifecycle-state-machine.md) | P0 | verify | PASS | M27-001-A, M27-001-B, M27-001-C, M27-001-D | Verify Define capability ABI and lifecycle state machine |
| [M27-001-Z](../tasks/04_m27_capability_linker/M27-001-Z-package-evidence-for-define-capability-abi-and-lifecycle-state-machine.md) | P0 | evidence | PASS | M27-001-V | Package evidence for Define capability ABI and lifecycle state machine |
| [M27-002-A](../tasks/04_m27_capability_linker/M27-002-A-build-dependency-dag.md) | P0 | implement | PASS | M27-001-Z | Build dependency DAG |
| [M27-002-B](../tasks/04_m27_capability_linker/M27-002-B-reject-cycles-missing-conflicting-versions.md) | P0 | implement | PASS | M27-002-A | Reject cycles/missing/conflicting versions |
| [M27-002-C](../tasks/04_m27_capability_linker/M27-002-C-emit-capability-inventory-hash-into-qpack.md) | P0 | implement | PASS | M27-002-B | Emit capability inventory/hash into QPack |
| [M27-002-D](../tasks/04_m27_capability_linker/M27-002-D-remove-unused-modules.md) | P0 | implement | PASS | M27-002-C | Remove unused modules |
| [M27-002-V](../tasks/04_m27_capability_linker/M27-002-V-verify-implement-compile-time-capability-dependency-resolver.md) | P0 | verify | PASS | M27-002-A, M27-002-B, M27-002-C, M27-002-D | Verify Implement compile-time capability dependency resolver |
| [M27-002-Z](../tasks/04_m27_capability_linker/M27-002-Z-package-evidence-for-implement-compile-time-capability-dependency-resolver.md) | P0 | evidence | PASS | M27-002-V | Package evidence for Implement compile-time capability dependency resolver |
| [M27-003-A](../tasks/04_m27_capability_linker/M27-003-A-build-configurable-intrinsic-profiles.md) | P1 | implement | PASS | M27-002-Z | Build configurable intrinsic profiles |
| [M27-003-B](../tasks/04_m27_capability_linker/M27-003-B-compile-application-requirements.md) | P1 | implement | PASS | M27-003-A | Compile application requirements |
| [M27-003-C](../tasks/04_m27_capability_linker/M27-003-C-report-missing-api-intrinsic-diagnostics.md) | P1 | implement | PASS | M27-003-B | Report missing API/intrinsic diagnostics |
| [M27-003-D](../tasks/04_m27_capability_linker/M27-003-D-retain-full-profile-for-compatibility-testing.md) | P1 | implement | PASS | M27-003-C | Retain full profile for compatibility testing |
| [M27-003-V](../tasks/04_m27_capability_linker/M27-003-V-verify-introduce-custom-quickjs-context-profiles.md) | P1 | verify | PASS | M27-003-A, M27-003-B, M27-003-C, M27-003-D | Verify Introduce custom QuickJS context profiles |
| [M27-003-Z](../tasks/04_m27_capability_linker/M27-003-Z-package-evidence-for-introduce-custom-quickjs-context-profiles.md) | P1 | evidence | PASS | M27-003-V | Package evidence for Introduce custom QuickJS context profiles |
| [M27-004-A](../tasks/04_m27_capability_linker/M27-004-A-port-timer-cancellation-accounting.md) | P0 | implement | PASS | M27-001-Z, M27-002-Z | Port timer cancellation/accounting |
| [M27-004-B](../tasks/04_m27_capability_linker/M27-004-B-define-console-levels-and-redaction.md) | P0 | implement | PASS | M27-004-A | Define console levels and redaction |
| [M27-004-C](../tasks/04_m27_capability_linker/M27-004-C-keep-logs-asynchronous-bounded.md) | P0 | implement | PASS | M27-004-B | Keep logs asynchronous/bounded |
| [M27-004-D](../tasks/04_m27_capability_linker/M27-004-D-support-shutdown-and-quarantine.md) | P0 | implement | PASS | M27-004-C | Support shutdown and quarantine |
| [M27-004-V](../tasks/04_m27_capability_linker/M27-004-V-verify-implement-console-and-timer-core-capabilities.md) | P0 | verify | PASS | M27-004-A, M27-004-B, M27-004-C, M27-004-D | Verify Implement console and timer core capabilities |
| [M27-004-Z](../tasks/04_m27_capability_linker/M27-004-Z-package-evidence-for-implement-console-and-timer-core-capabilities.md) | P0 | evidence | PASS | M27-004-V | Package evidence for Implement console and timer core capabilities |
| [M27-005-A](../tasks/04_m27_capability_linker/M27-005-A-adopt-or-adapt-a-proven-implementation.md) | P1 | implement | PASS | M27-001-Z, M27-003-Z | Adopt or adapt a proven implementation |
| [M27-005-B](../tasks/04_m27_capability_linker/M27-005-B-run-selected-wpt-wintertc-cases.md) | P1 | implement | PASS | M27-005-A | Run selected WPT/WinterTC cases |
| [M27-005-C](../tasks/04_m27_capability_linker/M27-005-C-define-host-path-encoding-behavior.md) | P1 | implement | PASS | M27-005-B | Define host/path encoding behavior |
| [M27-005-D](../tasks/04_m27_capability_linker/M27-005-D-keep-parser-limits-explicit.md) | P1 | implement | PASS | M27-005-C | Keep parser limits explicit |
| [M27-005-V](../tasks/04_m27_capability_linker/M27-005-V-verify-implement-url-and-urlsearchparams.md) | P1 | verify | PASS | M27-005-A, M27-005-B, M27-005-C, M27-005-D | Verify Implement URL and URLSearchParams |
| [M27-005-Z](../tasks/04_m27_capability_linker/M27-005-Z-package-evidence-for-implement-url-and-urlsearchparams.md) | P1 | evidence | PASS | M27-005-V | Package evidence for Implement URL and URLSearchParams |
| [M27-006-A](../tasks/04_m27_capability_linker/M27-006-A-support-utf-8-baseline.md) | P1 | implement | PASS | M27-001-Z, M27-003-Z | Support UTF-8 baseline |
| [M27-006-B](../tasks/04_m27_capability_linker/M27-006-B-define-invalid-sequence-replacement-behavior.md) | P1 | implement | PASS | M27-006-A | Define invalid sequence/replacement behavior |
| [M27-006-C](../tasks/04_m27_capability_linker/M27-006-C-integrate-typedarray-ownership.md) | P1 | implement | PASS | M27-006-B | Integrate TypedArray ownership |
| [M27-006-D](../tasks/04_m27_capability_linker/M27-006-D-run-wpt-subset.md) | P1 | implement | PASS | M27-006-C | Run WPT subset |
| [M27-006-V](../tasks/04_m27_capability_linker/M27-006-V-verify-implement-textencoder-and-textdecoder.md) | P1 | verify | PASS | M27-006-A, M27-006-B, M27-006-C, M27-006-D | Verify Implement TextEncoder and TextDecoder |
| [M27-006-Z](../tasks/04_m27_capability_linker/M27-006-Z-package-evidence-for-implement-textencoder-and-textdecoder.md) | P1 | evidence | PASS | M27-006-V | Package evidence for Implement TextEncoder and TextDecoder |
| [M27-007-A](../tasks/04_m27_capability_linker/M27-007-A-define-signal-state-listeners-reason.md) | P0 | implement | PASS | M27-001-Z, M27-003-Z | Define signal state/listeners/reason |
| [M27-007-B](../tasks/04_m27_capability_linker/M27-007-B-bridge-route-deadline-and-explicit-cancellation.md) | P0 | implement | PASS | M27-007-A | Bridge route deadline and explicit cancellation |
| [M27-007-C](../tasks/04_m27_capability_linker/M27-007-C-prevent-listener-leaks.md) | P0 | implement | TODO | M27-007-B | Prevent listener leaks |
| [M27-007-D](../tasks/04_m27_capability_linker/M27-007-D-make-cancellation-idempotent.md) | P0 | implement | TODO | M27-007-C | Make cancellation idempotent |
| [M27-007-V](../tasks/04_m27_capability_linker/M27-007-V-verify-implement-abortcontroller-and-abortsignal.md) | P0 | verify | TODO | M27-007-A, M27-007-B, M27-007-C, M27-007-D | Verify Implement AbortController and AbortSignal |
| [M27-007-Z](../tasks/04_m27_capability_linker/M27-007-Z-package-evidence-for-implement-abortcontroller-and-abortsignal.md) | P0 | evidence | TODO | M27-007-V | Package evidence for Implement AbortController and AbortSignal |
| [M27-008-A](../tasks/04_m27_capability_linker/M27-008-A-implement-getrandomvalues-and-randomuuid-through-os-csprng.md) | P0 | implement | TODO | M27-001-Z, M27-003-Z | Implement `getRandomValues` and `randomUUID` through OS CSPRNG |
| [M27-008-B](../tasks/04_m27_capability_linker/M27-008-B-enforce-typed-array-and-size-constraints.md) | P0 | implement | TODO | M27-008-A | Enforce typed-array and size constraints |
| [M27-008-C](../tasks/04_m27_capability_linker/M27-008-C-define-unavailable-entropy-failure.md) | P0 | implement | TODO | M27-008-B | Define unavailable-entropy failure |
| [M27-008-D](../tasks/04_m27_capability_linker/M27-008-D-do-not-implement-custom-cryptography.md) | P0 | implement | TODO | M27-008-C | Do not implement custom cryptography |
| [M27-008-V](../tasks/04_m27_capability_linker/M27-008-V-verify-implement-crypto-random-subset.md) | P0 | verify | TODO | M27-008-A, M27-008-B, M27-008-C, M27-008-D | Verify Implement crypto random subset |
| [M27-008-Z](../tasks/04_m27_capability_linker/M27-008-Z-package-evidence-for-implement-crypto-random-subset.md) | P0 | evidence | TODO | M27-008-V | Package evidence for Implement crypto random subset |
| [M27-009-A](../tasks/04_m27_capability_linker/M27-009-A-define-rust-side-sdk-traits-and-metadata.md) | P1 | implement | TODO | M27-001-Z, M27-002-Z | Define Rust-side SDK traits and metadata |
| [M27-009-B](../tasks/04_m27_capability_linker/M27-009-B-provide-test-harness-and-example-capability.md) | P1 | implement | TODO | M27-009-A | Provide test harness and example capability |
| [M27-009-C](../tasks/04_m27_capability_linker/M27-009-C-expose-build-inspect-diagnostics.md) | P1 | implement | TODO | M27-009-B | Expose build/inspect diagnostics |
| [M27-009-D](../tasks/04_m27_capability_linker/M27-009-D-define-semver-abi-compatibility.md) | P1 | implement | TODO | M27-009-C | Define semver/ABI compatibility |
| [M27-009-V](../tasks/04_m27_capability_linker/M27-009-V-verify-publish-capability-sdk-and-inspection-surface.md) | P1 | verify | TODO | M27-009-A, M27-009-B, M27-009-C, M27-009-D | Verify Publish capability SDK and inspection surface |
| [M27-009-Z](../tasks/04_m27_capability_linker/M27-009-Z-package-evidence-for-publish-capability-sdk-and-inspection-surface.md) | P1 | evidence | TODO | M27-009-V | Package evidence for Publish capability SDK and inspection surface |
| [M27-010-A](../tasks/04_m27_capability_linker/M27-010-A-pin-wpt-wintertc-subsets.md) | P1 | implement | TODO | M27-005-Z, M27-006-Z, M27-007-Z, M27-008-Z | Pin WPT/WinterTC subsets |
| [M27-010-B](../tasks/04_m27_capability_linker/M27-010-B-record-skips-and-reasons.md) | P1 | implement | TODO | M27-010-A | Record skips and reasons |
| [M27-010-C](../tasks/04_m27_capability_linker/M27-010-C-automate-regression-reports.md) | P1 | implement | TODO | M27-010-B | Automate regression reports |
| [M27-010-D](../tasks/04_m27_capability_linker/M27-010-D-keep-unsupported-apis-explicit.md) | P1 | implement | TODO | M27-010-C | Keep unsupported APIs explicit |
| [M27-010-V](../tasks/04_m27_capability_linker/M27-010-V-verify-establish-web-api-conformance-program.md) | P1 | verify | TODO | M27-010-A, M27-010-B, M27-010-C, M27-010-D | Verify Establish Web API conformance program |
| [M27-010-Z](../tasks/04_m27_capability_linker/M27-010-Z-package-evidence-for-establish-web-api-conformance-program.md) | P1 | evidence | TODO | M27-010-V | Package evidence for Establish Web API conformance program |
| [M27-011-A](../tasks/04_m27_capability_linker/M27-011-A-measure-core-web-minimal-and-all-beta-profiles.md) | P1 | implement | TODO | M27-002-Z, M27-010-Z | Measure core, web-minimal, and all-beta profiles |
| [M27-011-B](../tasks/04_m27_capability_linker/M27-011-B-record-binary-startup-and-idle-rss-deltas.md) | P1 | implement | TODO | M27-011-A | Record binary, startup, and idle RSS deltas |
| [M27-011-C](../tasks/04_m27_capability_linker/M27-011-C-identify-eager-initialization.md) | P1 | implement | TODO | M27-011-B | Identify eager initialization |
| [M27-011-D](../tasks/04_m27_capability_linker/M27-011-D-make-expensive-modules-lazy-when-safe.md) | P1 | implement | TODO | M27-011-C | Make expensive modules lazy when safe |
| [M27-011-V](../tasks/04_m27_capability_linker/M27-011-V-verify-close-capability-cost-budgets.md) | P1 | verify | TODO | M27-011-A, M27-011-B, M27-011-C, M27-011-D | Verify Close capability cost budgets |
| [M27-011-Z](../tasks/04_m27_capability_linker/M27-011-Z-package-evidence-for-close-capability-cost-budgets.md) | P1 | evidence | TODO | M27-011-V | Package evidence for Close capability cost budgets |
| [M27-GATE](../gates/M27-GATE.md) | P0 | gate | TODO | M27-001-Z, M27-002-Z, M27-003-Z, M27-004-Z, M27-005-Z, M27-006-Z, M27-007-Z, M27-008-Z, M27-009-Z, M27-010-Z, M27-011-Z | M2.7 — Capability Linker and Minimal Web Runtime exit gate |

## M28 — Native Outbound Fetch

| ID | P | Kind | Status | Dependencies | Task |
|---|---:|---|---|---|---|
| [M28-001-A](../tasks/05_m28_native_fetch/M28-001-A-define-url-schemes-redirect-policy-dns-rebinding-controls-proxy-behavior-tls-roo.md) | P0 | implement | TODO | M27-GATE | Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits |
| [M28-001-B](../tasks/05_m28_native_fetch/M28-001-B-specify-reverse-proxy-and-outbound-trust.md) | P0 | implement | TODO | M28-001-A | Specify reverse-proxy and outbound trust |
| [M28-001-C](../tasks/05_m28_native_fetch/M28-001-C-define-unsupported-web-features.md) | P0 | implement | TODO | M28-001-B | Define unsupported Web features |
| [M28-001-D](../tasks/05_m28_native_fetch/M28-001-D-document-same-process-trusted-code-assumption.md) | P0 | implement | TODO | M28-001-C | Document same-process trusted-code assumption |
| [M28-001-V](../tasks/05_m28_native_fetch/M28-001-V-verify-accept-fetch-tls-redirect-and-ssrf-security-adr.md) | P0 | verify | TODO | M28-001-A, M28-001-B, M28-001-C, M28-001-D | Verify Accept fetch, TLS, redirect, and SSRF security ADR |
| [M28-001-Z](../tasks/05_m28_native_fetch/M28-001-Z-package-evidence-for-accept-fetch-tls-redirect-and-ssrf-security-adr.md) | P0 | evidence | TODO | M28-001-V | Package evidence for Accept fetch, TLS, redirect, and SSRF security ADR |
| [M28-002-A](../tasks/05_m28_native_fetch/M28-002-A-compare-reqwest-and-lower-level-hyper-rustls-approach.md) | P1 | implement | TODO | M28-001-Z | Compare reqwest and lower-level Hyper/Rustls approach |
| [M28-002-B](../tasks/05_m28_native_fetch/M28-002-B-measure-dependency-binary-startup-cost.md) | P1 | implement | TODO | M28-002-A | Measure dependency/binary/startup cost |
| [M28-002-C](../tasks/05_m28_native_fetch/M28-002-C-test-dns-tls-pool-behavior.md) | P1 | implement | TODO | M28-002-B | Test DNS/TLS/pool behavior |
| [M28-002-D](../tasks/05_m28_native_fetch/M28-002-D-record-maintenance-security-considerations.md) | P1 | implement | TODO | M28-002-C | Record maintenance/security considerations |
| [M28-002-V](../tasks/05_m28_native_fetch/M28-002-V-verify-select-native-http-client-stack-from-evidence.md) | P1 | verify | TODO | M28-002-A, M28-002-B, M28-002-C, M28-002-D | Verify Select native HTTP client stack from evidence |
| [M28-002-Z](../tasks/05_m28_native_fetch/M28-002-Z-package-evidence-for-select-native-http-client-stack-from-evidence.md) | P1 | evidence | TODO | M28-002-V | Package evidence for Select native HTTP client stack from evidence |
| [M28-003-A](../tasks/05_m28_native_fetch/M28-003-A-lazy-pool-initialization.md) | P0 | implement | TODO | M28-002-Z | Lazy pool initialization |
| [M28-003-B](../tasks/05_m28_native_fetch/M28-003-B-bound-idle-active-connections-and-dns-cache.md) | P0 | implement | TODO | M28-003-A | Bound idle/active connections and DNS cache |
| [M28-003-C](../tasks/05_m28_native_fetch/M28-003-C-use-verified-tls-roots-and-hostname-validation.md) | P0 | implement | TODO | M28-003-B | Use verified TLS roots and hostname validation |
| [M28-003-D](../tasks/05_m28_native_fetch/M28-003-D-define-keepalive-and-shutdown.md) | P0 | implement | TODO | M28-003-C | Define keepalive and shutdown |
| [M28-003-V](../tasks/05_m28_native_fetch/M28-003-V-verify-implement-connection-pooling-dns-and-tls.md) | P0 | verify | TODO | M28-003-A, M28-003-B, M28-003-C, M28-003-D | Verify Implement connection pooling, DNS, and TLS |
| [M28-003-Z](../tasks/05_m28_native_fetch/M28-003-Z-package-evidence-for-implement-connection-pooling-dns-and-tls.md) | P0 | evidence | TODO | M28-003-V | Package evidence for Implement connection pooling, DNS, and TLS |
| [M28-004-A](../tasks/05_m28_native_fetch/M28-004-A-implement-method-url-selected-headers-body-types-status-and-response-methods.md) | P0 | implement | TODO | M28-003-Z, M27-005-Z, M27-006-Z | Implement method, URL, selected headers, body types, status, and response methods |
| [M28-004-B](../tasks/05_m28_native_fetch/M28-004-B-use-lazy-native-backed-objects.md) | P0 | implement | TODO | M28-004-A | Use lazy native-backed objects |
| [M28-004-C](../tasks/05_m28_native_fetch/M28-004-C-define-clone-body-used-semantics-for-beta.md) | P0 | implement | TODO | M28-004-B | Define clone/body-used semantics for beta |
| [M28-004-D](../tasks/05_m28_native_fetch/M28-004-D-keep-unsupported-api-diagnostics-explicit.md) | P0 | implement | TODO | M28-004-C | Keep unsupported API diagnostics explicit |
| [M28-004-V](../tasks/05_m28_native_fetch/M28-004-V-verify-implement-request-response-and-headers-subset.md) | P0 | verify | TODO | M28-004-A, M28-004-B, M28-004-C, M28-004-D | Verify Implement Request, Response, and Headers subset |
| [M28-004-Z](../tasks/05_m28_native_fetch/M28-004-Z-package-evidence-for-implement-request-response-and-headers-subset.md) | P0 | evidence | TODO | M28-004-V | Package evidence for Implement Request, Response, and Headers subset |
| [M28-005-A](../tasks/05_m28_native_fetch/M28-005-A-combine-explicit-abort-route-deadline-disconnect-shutdown-and-quarantine.md) | P0 | implement | TODO | M28-003-Z, M27-007-Z | Combine explicit abort, route deadline, disconnect, shutdown, and quarantine |
| [M28-005-B](../tasks/05_m28_native_fetch/M28-005-B-use-one-terminal-state-for-each-operation.md) | P0 | implement | TODO | M28-005-A | Use one terminal state for each operation |
| [M28-005-C](../tasks/05_m28_native_fetch/M28-005-C-cancel-dns-connect-body-streaming.md) | P0 | implement | TODO | M28-005-B | Cancel DNS/connect/body streaming |
| [M28-005-D](../tasks/05_m28_native_fetch/M28-005-D-map-failures-deterministically.md) | P0 | implement | TODO | M28-005-C | Map failures deterministically |
| [M28-005-V](../tasks/05_m28_native_fetch/M28-005-V-verify-propagate-abortsignal-and-route-deadlines.md) | P0 | verify | TODO | M28-005-A, M28-005-B, M28-005-C, M28-005-D | Verify Propagate AbortSignal and route deadlines |
| [M28-005-Z](../tasks/05_m28_native_fetch/M28-005-Z-package-evidence-for-propagate-abortsignal-and-route-deadlines.md) | P0 | evidence | TODO | M28-005-V | Package evidence for Propagate AbortSignal and route deadlines |
| [M28-006-A](../tasks/05_m28_native_fetch/M28-006-A-bound-read-write-buffers.md) | P0 | implement | TODO | M28-004-Z, M28-005-Z | Bound read/write buffers |
| [M28-006-B](../tasks/05_m28_native_fetch/M28-006-B-propagate-downstream-backpressure.md) | P0 | implement | TODO | M28-006-A | Propagate downstream backpressure |
| [M28-006-C](../tasks/05_m28_native_fetch/M28-006-C-cancel-on-consumer-stop-disconnect.md) | P0 | implement | TODO | M28-006-B | Cancel on consumer stop/disconnect |
| [M28-006-D](../tasks/05_m28_native_fetch/M28-006-D-define-maximum-body-helper-sizes.md) | P0 | implement | TODO | M28-006-C | Define maximum body helper sizes |
| [M28-006-V](../tasks/05_m28_native_fetch/M28-006-V-verify-implement-streaming-and-strict-backpressure.md) | P0 | verify | TODO | M28-006-A, M28-006-B, M28-006-C, M28-006-D | Verify Implement streaming and strict backpressure |
| [M28-006-Z](../tasks/05_m28_native_fetch/M28-006-Z-package-evidence-for-implement-streaming-and-strict-backpressure.md) | P0 | evidence | TODO | M28-006-V | Package evidence for Implement streaming and strict backpressure |
| [M28-007-A](../tasks/05_m28_native_fetch/M28-007-A-limit-redirect-count.md) | P1 | implement | TODO | M28-003-Z, M28-004-Z | Limit redirect count |
| [M28-007-B](../tasks/05_m28_native_fetch/M28-007-B-reapply-ssrf-dns-policy-on-every-hop.md) | P1 | implement | TODO | M28-007-A | Reapply SSRF/DNS policy on every hop |
| [M28-007-C](../tasks/05_m28_native_fetch/M28-007-C-define-credential-header-stripping.md) | P1 | implement | TODO | M28-007-B | Define credential/header stripping |
| [M28-007-D](../tasks/05_m28_native_fetch/M28-007-D-bound-decompression-ratio-and-output.md) | P1 | implement | TODO | M28-007-C | Bound decompression ratio and output |
| [M28-007-V](../tasks/05_m28_native_fetch/M28-007-V-verify-implement-redirect-and-compression-policy.md) | P1 | verify | TODO | M28-007-A, M28-007-B, M28-007-C, M28-007-D | Verify Implement redirect and compression policy |
| [M28-007-Z](../tasks/05_m28_native_fetch/M28-007-Z-package-evidence-for-implement-redirect-and-compression-policy.md) | P1 | evidence | TODO | M28-007-V | Package evidence for Implement redirect and compression policy |
| [M28-008-A](../tasks/05_m28_native_fetch/M28-008-A-resolve-and-validate-addresses-before-connect.md) | P0 | implement | TODO | M28-001-Z, M28-003-Z, M28-007-Z | Resolve and validate addresses before connect |
| [M28-008-B](../tasks/05_m28_native_fetch/M28-008-B-revalidate-redirects-and-connection-targets.md) | P0 | implement | TODO | M28-008-A | Revalidate redirects and connection targets |
| [M28-008-C](../tasks/05_m28_native_fetch/M28-008-C-support-allow-deny-configuration.md) | P0 | implement | TODO | M28-008-B | Support allow/deny configuration |
| [M28-008-D](../tasks/05_m28_native_fetch/M28-008-D-define-proxy-interaction.md) | P0 | implement | TODO | M28-008-C | Define proxy interaction |
| [M28-008-V](../tasks/05_m28_native_fetch/M28-008-V-verify-implement-ssrf-and-network-egress-controls.md) | P0 | verify | TODO | M28-008-A, M28-008-B, M28-008-C, M28-008-D | Verify Implement SSRF and network egress controls |
| [M28-008-Z](../tasks/05_m28_native_fetch/M28-008-Z-package-evidence-for-implement-ssrf-and-network-egress-controls.md) | P0 | evidence | TODO | M28-008-V | Package evidence for Implement SSRF and network egress controls |
| [M28-009-A](../tasks/05_m28_native_fetch/M28-009-A-expose-pool-wait-dns-connect-tls-ttfb-body-errors-cancellations.md) | P1 | implement | TODO | M28-003-Z, M28-005-Z, M28-006-Z | Expose pool wait, DNS, connect, TLS, TTFB, body, errors, cancellations |
| [M28-009-B](../tasks/05_m28_native_fetch/M28-009-B-sample-aggregate-metrics.md) | P1 | implement | TODO | M28-009-A | Sample/aggregate metrics |
| [M28-009-C](../tasks/05_m28_native_fetch/M28-009-C-drain-pool-on-shutdown.md) | P1 | implement | TODO | M28-009-B | Drain pool on shutdown |
| [M28-009-D](../tasks/05_m28_native_fetch/M28-009-D-quarantine-rejects-new-work.md) | P1 | implement | TODO | M28-009-C | Quarantine rejects new work |
| [M28-009-V](../tasks/05_m28_native_fetch/M28-009-V-verify-integrate-lifecycle-observability-and-shutdown.md) | P1 | verify | TODO | M28-009-A, M28-009-B, M28-009-C, M28-009-D | Verify Integrate lifecycle, observability, and shutdown |
| [M28-009-Z](../tasks/05_m28_native_fetch/M28-009-Z-package-evidence-for-integrate-lifecycle-observability-and-shutdown.md) | P1 | evidence | TODO | M28-009-V | Package evidence for Integrate lifecycle, observability, and shutdown |
| [M28-010-A](../tasks/05_m28_native_fetch/M28-010-A-run-selected-wpt-cases.md) | P0 | implement | TODO | M28-004-Z, M28-005-Z, M28-006-Z, M28-007-Z, M28-008-Z | Run selected WPT cases |
| [M28-010-B](../tasks/05_m28_native_fetch/M28-010-B-create-deterministic-dns-tls-redirect-slow-body-fixtures.md) | P0 | implement | TODO | M28-010-A | Create deterministic DNS/TLS/redirect/slow/body fixtures |
| [M28-010-C](../tasks/05_m28_native_fetch/M28-010-C-fuzz-headers-and-urls.md) | P0 | implement | TODO | M28-010-B | Fuzz headers and URLs |
| [M28-010-D](../tasks/05_m28_native_fetch/M28-010-D-test-proxy-and-cancellation.md) | P0 | implement | TODO | M28-010-C | Test proxy and cancellation |
| [M28-010-V](../tasks/05_m28_native_fetch/M28-010-V-verify-complete-fetch-conformance-and-fault-testing.md) | P0 | verify | TODO | M28-010-A, M28-010-B, M28-010-C, M28-010-D | Verify Complete fetch conformance and fault testing |
| [M28-010-Z](../tasks/05_m28_native_fetch/M28-010-Z-package-evidence-for-complete-fetch-conformance-and-fault-testing.md) | P0 | evidence | TODO | M28-010-V | Package evidence for Complete fetch conformance and fault testing |
| [M28-011-A](../tasks/05_m28_native_fetch/M28-011-A-run-1-5-10-25ms-upstream-latency.md) | P1 | implement | TODO | M28-009-Z, M28-010-Z | Run 1/5/10/25ms upstream latency |
| [M28-011-B](../tasks/05_m28_native_fetch/M28-011-B-run-one-two-and-four-parallel-calls.md) | P1 | implement | TODO | M28-011-A | Run one, two, and four parallel calls |
| [M28-011-C](../tasks/05_m28_native_fetch/M28-011-C-mix-timeout-success-malformed-responses.md) | P1 | implement | TODO | M28-011-B | Mix timeout/success/malformed responses |
| [M28-011-D](../tasks/05_m28_native_fetch/M28-011-D-test-concurrency-1-10-50-200.md) | P1 | implement | TODO | M28-011-C | Test concurrency 1/10/50/200 |
| [M28-011-V](../tasks/05_m28_native_fetch/M28-011-V-verify-run-controlled-upstream-and-fan-out-benchmarks.md) | P1 | verify | TODO | M28-011-A, M28-011-B, M28-011-C, M28-011-D | Verify Run controlled upstream and fan-out benchmarks |
| [M28-011-Z](../tasks/05_m28_native_fetch/M28-011-Z-package-evidence-for-run-controlled-upstream-and-fan-out-benchmarks.md) | P1 | evidence | TODO | M28-011-V | Package evidence for Run controlled upstream and fan-out benchmarks |
| [M28-GATE](../gates/M28-GATE.md) | P0 | gate | TODO | M28-001-Z, M28-002-Z, M28-003-Z, M28-004-Z, M28-005-Z, M28-006-Z, M28-007-Z, M28-008-Z, M28-009-Z, M28-010-Z, M28-011-Z | M2.8 — Native Outbound Fetch exit gate |

## M3 — Multi-Worker Service Runtime

| ID | P | Kind | Status | Dependencies | Task |
|---|---:|---|---|---|---|
| [M3-001-A](../tasks/06_m3_multi_worker/M3-001-A-accept-adr.md) | P0 | implement | TODO | M28-GATE | Accept ADR |
| [M3-001-B](../tasks/06_m3_multi_worker/M3-001-B-document-module-level-state-replication.md) | P0 | implement | TODO | M3-001-A | Document module-level state replication |
| [M3-001-C](../tasks/06_m3_multi_worker/M3-001-C-forbid-jsvalue-sharing.md) | P0 | implement | TODO | M3-001-B | Forbid JSValue sharing |
| [M3-001-D](../tasks/06_m3_multi_worker/M3-001-D-define-service-capability-shared-handles-and-thread-safety.md) | P0 | implement | TODO | M3-001-C | Define service/capability shared handles and thread safety |
| [M3-001-V](../tasks/06_m3_multi_worker/M3-001-V-verify-freeze-independent-worker-state-semantics.md) | P0 | verify | TODO | M3-001-A, M3-001-B, M3-001-C, M3-001-D | Verify Freeze independent-worker state semantics |
| [M3-001-Z](../tasks/06_m3_multi_worker/M3-001-Z-package-evidence-for-freeze-independent-worker-state-semantics.md) | P0 | evidence | TODO | M3-001-V | Package evidence for Freeze independent-worker state semantics |
| [M3-002-A](../tasks/06_m3_multi_worker/M3-002-A-use-bounded-per-worker-queues.md) | P0 | implement | TODO | M3-001-Z | Use bounded per-worker queues |
| [M3-002-B](../tasks/06_m3_multi_worker/M3-002-B-select-worker-using-outstanding-load-strategy.md) | P0 | implement | TODO | M3-002-A | Select worker using outstanding-load strategy |
| [M3-002-C](../tasks/06_m3_multi_worker/M3-002-C-define-admission-and-overload-response.md) | P0 | implement | TODO | M3-002-B | Define admission and overload response |
| [M3-002-D](../tasks/06_m3_multi_worker/M3-002-D-preserve-routeid-routeplan-before-dispatch.md) | P0 | implement | TODO | M3-002-C | Preserve RouteId/RoutePlan before dispatch |
| [M3-002-V](../tasks/06_m3_multi_worker/M3-002-V-verify-implement-bounded-worker-dispatcher.md) | P0 | verify | TODO | M3-002-A, M3-002-B, M3-002-C, M3-002-D | Verify Implement bounded worker dispatcher |
| [M3-002-Z](../tasks/06_m3_multi_worker/M3-002-Z-package-evidence-for-implement-bounded-worker-dispatcher.md) | P0 | evidence | TODO | M3-002-V | Package evidence for Implement bounded worker dispatcher |
| [M3-003-A](../tasks/06_m3_multi_worker/M3-003-A-serverless-starts-one-worker-only.md) | P1 | implement | TODO | M3-002-Z | Serverless starts one worker only |
| [M3-003-B](../tasks/06_m3_multi_worker/M3-003-B-service-marks-ready-after-worker-0-and-adds-workers-adaptively.md) | P1 | implement | TODO | M3-003-A | Service marks ready after worker 0 and adds workers adaptively |
| [M3-003-C](../tasks/06_m3_multi_worker/M3-003-C-throughput-initializes-configured-workers-before-ready.md) | P1 | implement | TODO | M3-003-B | Throughput initializes configured workers before ready |
| [M3-003-D](../tasks/06_m3_multi_worker/M3-003-D-expose-profile-in-inspect-config.md) | P1 | implement | TODO | M3-003-C | Expose profile in inspect/config |
| [M3-003-V](../tasks/06_m3_multi_worker/M3-003-V-verify-implement-serverless-service-and-throughput-profiles.md) | P1 | verify | TODO | M3-003-A, M3-003-B, M3-003-C, M3-003-D | Verify Implement serverless, service, and throughput profiles |
| [M3-003-Z](../tasks/06_m3_multi_worker/M3-003-Z-package-evidence-for-implement-serverless-service-and-throughput-profiles.md) | P1 | evidence | TODO | M3-003-V | Package evidence for Implement serverless, service, and throughput profiles |
| [M3-004-A](../tasks/06_m3_multi_worker/M3-004-A-share-immutable-mapped-qpack-bytes.md) | P0 | implement | TODO | M3-002-Z, M26-GATE | Share immutable mapped QPack bytes |
| [M3-004-B](../tasks/06_m3_multi_worker/M3-004-B-create-separate-quickjs-runtimes-functions-context-state.md) | P0 | implement | TODO | M3-004-A | Create separate QuickJS runtimes/functions/context state |
| [M3-004-C](../tasks/06_m3_multi_worker/M3-004-C-validate-capability-compatibility-per-worker.md) | P0 | implement | TODO | M3-004-B | Validate capability compatibility per worker |
| [M3-004-D](../tasks/06_m3_multi_worker/M3-004-D-bound-startup-parallelism.md) | P0 | implement | TODO | M3-004-C | Bound startup parallelism |
| [M3-004-V](../tasks/06_m3_multi_worker/M3-004-V-verify-implement-deterministic-worker-initialization-and-artifact-sharing.md) | P0 | verify | TODO | M3-004-A, M3-004-B, M3-004-C, M3-004-D | Verify Implement deterministic worker initialization and artifact sharing |
| [M3-004-Z](../tasks/06_m3_multi_worker/M3-004-Z-package-evidence-for-implement-deterministic-worker-initialization-and-artifact.md) | P0 | evidence | TODO | M3-004-V | Package evidence for Implement deterministic worker initialization and artifact sharing |
| [M3-005-A](../tasks/06_m3_multi_worker/M3-005-A-remove-quarantined-worker-from-dispatch.md) | P0 | implement | TODO | M3-002-Z, M3-004-Z | Remove quarantined worker from dispatch |
| [M3-005-B](../tasks/06_m3_multi_worker/M3-005-B-fail-settle-its-pending-work.md) | P0 | implement | TODO | M3-005-A | Fail/settle its pending work |
| [M3-005-C](../tasks/06_m3_multi_worker/M3-005-C-initialize-replacement-under-bounded-policy.md) | P0 | implement | TODO | M3-005-B | Initialize replacement under bounded policy |
| [M3-005-D](../tasks/06_m3_multi_worker/M3-005-D-aggregate-readiness-from-usable-capacity.md) | P0 | implement | TODO | M3-005-C | Aggregate readiness from usable capacity |
| [M3-005-V](../tasks/06_m3_multi_worker/M3-005-V-verify-implement-quarantine-replacement-and-readiness-aggregation.md) | P0 | verify | TODO | M3-005-A, M3-005-B, M3-005-C, M3-005-D | Verify Implement quarantine, replacement, and readiness aggregation |
| [M3-005-Z](../tasks/06_m3_multi_worker/M3-005-Z-package-evidence-for-implement-quarantine-replacement-and-readiness-aggregation.md) | P0 | evidence | TODO | M3-005-V | Package evidence for Implement quarantine, replacement, and readiness aggregation |
| [M3-006-A](../tasks/06_m3_multi_worker/M3-006-A-define-thresholds-hysteresis.md) | P1 | implement | TODO | M3-003-Z, M3-005-Z | Define thresholds/hysteresis |
| [M3-006-B](../tasks/06_m3_multi_worker/M3-006-B-bound-min-max-workers.md) | P1 | implement | TODO | M3-006-A | Bound min/max workers |
| [M3-006-C](../tasks/06_m3_multi_worker/M3-006-C-drain-before-scale-down.md) | P1 | implement | TODO | M3-006-B | Drain before scale-down |
| [M3-006-D](../tasks/06_m3_multi_worker/M3-006-D-avoid-oscillation.md) | P1 | implement | TODO | M3-006-C | Avoid oscillation |
| [M3-006-V](../tasks/06_m3_multi_worker/M3-006-V-verify-implement-adaptive-scale-up-and-scale-down.md) | P1 | verify | TODO | M3-006-A, M3-006-B, M3-006-C, M3-006-D | Verify Implement adaptive scale-up and scale-down |
| [M3-006-Z](../tasks/06_m3_multi_worker/M3-006-Z-package-evidence-for-implement-adaptive-scale-up-and-scale-down.md) | P1 | evidence | TODO | M3-006-V | Package evidence for Implement adaptive scale-up and scale-down |
| [M3-007-A](../tasks/06_m3_multi_worker/M3-007-A-track-invocation-to-worker-ownership.md) | P0 | implement | TODO | M3-002-Z, M3-004-Z | Track invocation-to-worker ownership |
| [M3-007-B](../tasks/06_m3_multi_worker/M3-007-B-stop-admission-on-drain.md) | P0 | implement | TODO | M3-007-A | Stop admission on drain |
| [M3-007-C](../tasks/06_m3_multi_worker/M3-007-C-allow-bounded-in-flight-completion.md) | P0 | implement | TODO | M3-007-B | Allow bounded in-flight completion |
| [M3-007-D](../tasks/06_m3_multi_worker/M3-007-D-abort-after-shutdown-deadline.md) | P0 | implement | TODO | M3-007-C | Abort after shutdown deadline |
| [M3-007-V](../tasks/06_m3_multi_worker/M3-007-V-verify-implement-multi-worker-cancellation-and-graceful-shutdown.md) | P0 | verify | TODO | M3-007-A, M3-007-B, M3-007-C, M3-007-D | Verify Implement multi-worker cancellation and graceful shutdown |
| [M3-007-Z](../tasks/06_m3_multi_worker/M3-007-Z-package-evidence-for-implement-multi-worker-cancellation-and-graceful-shutdown.md) | P0 | evidence | TODO | M3-007-V | Package evidence for Implement multi-worker cancellation and graceful shutdown |
| [M3-008-A](../tasks/06_m3_multi_worker/M3-008-A-add-route-global-queue-limits-or-weighted-admission.md) | P1 | implement | TODO | M3-002-Z, M3-006-Z | Add route/global queue limits or weighted admission |
| [M3-008-B](../tasks/06_m3_multi_worker/M3-008-B-define-long-running-js-policy.md) | P1 | implement | TODO | M3-008-A | Define long-running JS policy |
| [M3-008-C](../tasks/06_m3_multi_worker/M3-008-C-expose-load-shed-reasons.md) | P1 | implement | TODO | M3-008-B | Expose load-shed reasons |
| [M3-008-D](../tasks/06_m3_multi_worker/M3-008-D-test-mixed-workloads.md) | P1 | implement | TODO | M3-008-C | Test mixed workloads |
| [M3-008-V](../tasks/06_m3_multi_worker/M3-008-V-verify-add-fairness-and-overload-controls.md) | P1 | verify | TODO | M3-008-A, M3-008-B, M3-008-C, M3-008-D | Verify Add fairness and overload controls |
| [M3-008-Z](../tasks/06_m3_multi_worker/M3-008-Z-package-evidence-for-add-fairness-and-overload-controls.md) | P1 | evidence | TODO | M3-008-V | Package evidence for Add fairness and overload controls |
| [M3-009-A](../tasks/06_m3_multi_worker/M3-009-A-measure-1-2-4-workers.md) | P1 | implement | TODO | M3-003-Z, M3-006-Z, M3-008-Z | Measure 1/2/4 workers |
| [M3-009-B](../tasks/06_m3_multi_worker/M3-009-B-report-throughput-p50-p95-p99-queue-time-cpu-rss-errors.md) | P1 | implement | TODO | M3-009-A | Report throughput, p50/p95/p99, queue time, CPU, RSS, errors |
| [M3-009-C](../tasks/06_m3_multi_worker/M3-009-C-run-c1-c2-c3-and-controlled-i-o.md) | P1 | implement | TODO | M3-009-B | Run C1/C2/C3 and controlled I/O |
| [M3-009-D](../tasks/06_m3_multi_worker/M3-009-D-record-physical-core-topology.md) | P1 | implement | TODO | M3-009-C | Record physical core topology |
| [M3-009-V](../tasks/06_m3_multi_worker/M3-009-V-verify-close-multi-worker-scaling-and-memory-evidence.md) | P1 | verify | TODO | M3-009-A, M3-009-B, M3-009-C, M3-009-D | Verify Close multi-worker scaling and memory evidence |
| [M3-009-Z](../tasks/06_m3_multi_worker/M3-009-Z-package-evidence-for-close-multi-worker-scaling-and-memory-evidence.md) | P1 | evidence | TODO | M3-009-V | Package evidence for Close multi-worker scaling and memory evidence |
| [M3-010-A](../tasks/06_m3_multi_worker/M3-010-A-run-multi-hour-mixed-load.md) | P0 | implement | TODO | M3-005-Z, M3-007-Z, M3-009-Z | Run multi-hour mixed load |
| [M3-010-B](../tasks/06_m3_multi_worker/M3-010-B-inject-worker-poison-upstream-timeout-disconnect-and-shutdown.md) | P0 | implement | TODO | M3-010-A | Inject worker poison, upstream timeout, disconnect, and shutdown |
| [M3-010-C](../tasks/06_m3_multi_worker/M3-010-C-track-retained-memory-and-task-slot-counts.md) | P0 | implement | TODO | M3-010-B | Track retained memory and task/slot counts |
| [M3-010-D](../tasks/06_m3_multi_worker/M3-010-D-verify-recovery.md) | P0 | implement | TODO | M3-010-C | Verify recovery |
| [M3-010-V](../tasks/06_m3_multi_worker/M3-010-V-verify-run-multi-worker-soak-and-recovery.md) | P0 | verify | TODO | M3-010-A, M3-010-B, M3-010-C, M3-010-D | Verify Run multi-worker soak and recovery |
| [M3-010-Z](../tasks/06_m3_multi_worker/M3-010-Z-package-evidence-for-run-multi-worker-soak-and-recovery.md) | P0 | evidence | TODO | M3-010-V | Package evidence for Run multi-worker soak and recovery |
| [M3-GATE](../gates/M3-GATE.md) | P0 | gate | TODO | M3-001-Z, M3-002-Z, M3-003-Z, M3-004-Z, M3-005-Z, M3-006-Z, M3-007-Z, M3-008-Z, M3-009-Z, M3-010-Z | M3 — Multi-Worker Service Runtime exit gate |

## M4A — Actual-Runtime Developer Preview and Private Alpha

| ID | P | Kind | Status | Dependencies | Task |
|---|---:|---|---|---|---|
| [M4A-001-A](../tasks/07_m4a_developer_preview/M4A-001-A-watch-source-and-contracts.md) | P0 | implement | TODO | M3-GATE | Watch source and contracts |
| [M4A-001-B](../tasks/07_m4a_developer_preview/M4A-001-B-build-incremental-temporary-qpack.md) | P0 | implement | TODO | M4A-001-A | Build incremental temporary QPack |
| [M4A-001-C](../tasks/07_m4a_developer_preview/M4A-001-C-load-new-worker-before-switching-traffic.md) | P0 | implement | TODO | M4A-001-B | Load new worker before switching traffic |
| [M4A-001-D](../tasks/07_m4a_developer_preview/M4A-001-D-drain-old-worker-and-surface-compile-runtime-errors.md) | P0 | implement | TODO | M4A-001-C | Drain old worker and surface compile/runtime errors |
| [M4A-001-V](../tasks/07_m4a_developer_preview/M4A-001-V-verify-implement-actual-runtime-velqu-dev-loop.md) | P0 | verify | TODO | M4A-001-A, M4A-001-B, M4A-001-C, M4A-001-D | Verify Implement actual-runtime `velqu dev` loop |
| [M4A-001-Z](../tasks/07_m4a_developer_preview/M4A-001-Z-package-evidence-for-implement-actual-runtime-velqu-dev-loop.md) | P0 | evidence | TODO | M4A-001-V | Package evidence for Implement actual-runtime `velqu dev` loop |
| [M4A-002-A](../tasks/07_m4a_developer_preview/M4A-002-A-implement-and-document-velqu-dev-build-inspect-contract-diff-test-pack-inspect-m.md) | P1 | implement | TODO | M4A-001-Z, M26-GATE | Implement and document `velqu dev`, `build`, `inspect`, `contract diff`, `test`, `pack inspect/migrate`, and diagnostics |
| [M4A-002-B](../tasks/07_m4a_developer_preview/M4A-002-B-stable-exit-codes.md) | P1 | implement | TODO | M4A-002-A | Stable exit codes |
| [M4A-002-C](../tasks/07_m4a_developer_preview/M4A-002-C-machine-readable-output-option.md) | P1 | implement | TODO | M4A-002-B | Machine-readable output option |
| [M4A-002-D](../tasks/07_m4a_developer_preview/M4A-002-D-helpful-actionable-errors.md) | P1 | implement | TODO | M4A-002-C | Helpful actionable errors |
| [M4A-002-V](../tasks/07_m4a_developer_preview/M4A-002-V-verify-complete-cli-command-surface.md) | P1 | verify | TODO | M4A-002-A, M4A-002-B, M4A-002-C, M4A-002-D | Verify Complete CLI command surface |
| [M4A-002-Z](../tasks/07_m4a_developer_preview/M4A-002-Z-package-evidence-for-complete-cli-command-surface.md) | P1 | evidence | TODO | M4A-002-V | Package evidence for Complete CLI command surface |
| [M4A-003-A](../tasks/07_m4a_developer_preview/M4A-003-A-starter-api.md) | P1 | implement | TODO | M4A-002-Z | Starter API |
| [M4A-003-B](../tasks/07_m4a_developer_preview/M4A-003-B-treaty-client-example.md) | P1 | implement | TODO | M4A-003-A | Treaty client example |
| [M4A-003-C](../tasks/07_m4a_developer_preview/M4A-003-C-testing-setup.md) | P1 | implement | TODO | M4A-003-B | Testing setup |
| [M4A-003-D](../tasks/07_m4a_developer_preview/M4A-003-D-optional-fetch-profile-choices.md) | P1 | implement | TODO | M4A-003-C | Optional fetch/profile choices |
| [M4A-003-V](../tasks/07_m4a_developer_preview/M4A-003-V-verify-implement-project-scaffolding.md) | P1 | verify | TODO | M4A-003-A, M4A-003-B, M4A-003-C, M4A-003-D | Verify Implement project scaffolding |
| [M4A-003-Z](../tasks/07_m4a_developer_preview/M4A-003-Z-package-evidence-for-implement-project-scaffolding.md) | P1 | evidence | TODO | M4A-003-V | Package evidence for Implement project scaffolding |
| [M4A-004-A](../tasks/07_m4a_developer_preview/M4A-004-A-unit-local-direct-generated-dispatcher.md) | P0 | implement | TODO | M25-GATE, M4A-001-Z | Unit-local direct generated dispatcher |
| [M4A-004-B](../tasks/07_m4a_developer_preview/M4A-004-B-runtime-local-actual-rust-quickjs-process.md) | P0 | implement | TODO | M4A-004-A | Runtime-local actual Rust/QuickJS process |
| [M4A-004-C](../tasks/07_m4a_developer_preview/M4A-004-C-remote-fetch-client.md) | P0 | implement | TODO | M4A-004-B | Remote fetch client |
| [M4A-004-D](../tasks/07_m4a_developer_preview/M4A-004-D-exact-method-body-query-status-problem-typing.md) | P0 | implement | TODO | M4A-004-C | Exact method/body/query/status/problem typing |
| [M4A-004-V](../tasks/07_m4a_developer_preview/M4A-004-V-verify-complete-treaty-unit-local-runtime-local-and-remote-modes.md) | P0 | verify | TODO | M4A-004-A, M4A-004-B, M4A-004-C, M4A-004-D | Verify Complete Treaty unit-local, runtime-local, and remote modes |
| [M4A-004-Z](../tasks/07_m4a_developer_preview/M4A-004-Z-package-evidence-for-complete-treaty-unit-local-runtime-local-and-remote-modes.md) | P0 | evidence | TODO | M4A-004-V | Package evidence for Complete Treaty unit-local, runtime-local, and remote modes |
| [M4A-005-A](../tasks/07_m4a_developer_preview/M4A-005-A-generate-d-ts-client-openapi-contract-lock.md) | P1 | implement | TODO | M4A-004-Z | Generate d.ts/client/OpenAPI/contract lock |
| [M4A-005-B](../tasks/07_m4a_developer_preview/M4A-005-B-tree-shakable-client.md) | P1 | implement | TODO | M4A-005-A | Tree-shakable client |
| [M4A-005-C](../tasks/07_m4a_developer_preview/M4A-005-C-version-and-public-contract-hash.md) | P1 | implement | TODO | M4A-005-B | Version and public contract hash |
| [M4A-005-D](../tasks/07_m4a_developer_preview/M4A-005-D-package-verification.md) | P1 | implement | TODO | M4A-005-C | Package verification |
| [M4A-005-V](../tasks/07_m4a_developer_preview/M4A-005-V-verify-publish-compact-contract-and-sdk-artifacts.md) | P1 | verify | TODO | M4A-005-A, M4A-005-B, M4A-005-C, M4A-005-D | Verify Publish compact contract and SDK artifacts |
| [M4A-005-Z](../tasks/07_m4a_developer_preview/M4A-005-Z-package-evidence-for-publish-compact-contract-and-sdk-artifacts.md) | P1 | evidence | TODO | M4A-005-V | Package evidence for Publish compact contract and SDK artifacts |
| [M4A-006-A](../tasks/07_m4a_developer_preview/M4A-006-A-structured-diagnostic-codes.md) | P0 | implement | TODO | M4A-001-Z, M4A-002-Z | Structured diagnostic codes |
| [M4A-006-B](../tasks/07_m4a_developer_preview/M4A-006-B-source-map-aware-stacks.md) | P0 | implement | TODO | M4A-006-A | Source-map-aware stacks |
| [M4A-006-C](../tasks/07_m4a_developer_preview/M4A-006-C-redaction-policy.md) | P0 | implement | TODO | M4A-006-B | Redaction policy |
| [M4A-006-D](../tasks/07_m4a_developer_preview/M4A-006-D-inspect-route-plan-fields-codecs-capabilities-crossings-and-debug-names.md) | P0 | implement | TODO | M4A-006-C | Inspect route plan, fields, codecs, capabilities, crossings, and debug names |
| [M4A-006-V](../tasks/07_m4a_developer_preview/M4A-006-V-verify-finalize-diagnostics-source-maps-and-inspect-output.md) | P0 | verify | TODO | M4A-006-A, M4A-006-B, M4A-006-C, M4A-006-D | Verify Finalize diagnostics, source maps, and inspect output |
| [M4A-006-Z](../tasks/07_m4a_developer_preview/M4A-006-Z-package-evidence-for-finalize-diagnostics-source-maps-and-inspect-output.md) | P0 | evidence | TODO | M4A-006-V | Package evidence for Finalize diagnostics, source maps, and inspect output |
| [M4A-007-A](../tasks/07_m4a_developer_preview/M4A-007-A-define-deferred-owner-queue-deadline-cancellation-shutdown.md) | P0 | implement | TODO | M27-GATE, M3-GATE | Define deferred owner, queue, deadline, cancellation, shutdown |
| [M4A-007-B](../tasks/07_m4a_developer_preview/M4A-007-B-separate-cleanup-from-best-effort-work.md) | P0 | implement | TODO | M4A-007-A | Separate cleanup from best-effort work |
| [M4A-007-C](../tasks/07_m4a_developer_preview/M4A-007-C-expose-metrics.md) | P0 | implement | TODO | M4A-007-B | Expose metrics |
| [M4A-007-D](../tasks/07_m4a_developer_preview/M4A-007-D-forbid-unbounded-recursive-spawning.md) | P0 | implement | TODO | M4A-007-C | Forbid unbounded recursive spawning |
| [M4A-007-V](../tasks/07_m4a_developer_preview/M4A-007-V-verify-implement-bounded-defer-and-lifecycle-hooks.md) | P0 | verify | TODO | M4A-007-A, M4A-007-B, M4A-007-C, M4A-007-D | Verify Implement bounded `defer` and lifecycle hooks |
| [M4A-007-Z](../tasks/07_m4a_developer_preview/M4A-007-Z-package-evidence-for-implement-bounded-defer-and-lifecycle-hooks.md) | P0 | evidence | TODO | M4A-007-V | Package evidence for Implement bounded `defer` and lifecycle hooks |
| [M4A-008-A](../tasks/07_m4a_developer_preview/M4A-008-A-quickstart.md) | P1 | implement | TODO | M4A-002-Z, M4A-004-Z, M4A-006-Z | Quickstart |
| [M4A-008-B](../tasks/07_m4a_developer_preview/M4A-008-B-routes-schemas-policies-services.md) | P1 | implement | TODO | M4A-008-A | Routes/schemas/policies/services |
| [M4A-008-C](../tasks/07_m4a_developer_preview/M4A-008-C-treaty.md) | P1 | implement | TODO | M4A-008-B | Treaty |
| [M4A-008-D](../tasks/07_m4a_developer_preview/M4A-008-D-fetch-capabilities.md) | P1 | implement | TODO | M4A-008-C | Fetch/capabilities |
| [M4A-008-E](../tasks/07_m4a_developer_preview/M4A-008-E-runtime-profiles.md) | P1 | implement | TODO | M4A-008-D | Runtime profiles |
| [M4A-008-F](../tasks/07_m4a_developer_preview/M4A-008-F-deployment-behind-reverse-proxy.md) | P1 | implement | TODO | M4A-008-E | Deployment behind reverse proxy |
| [M4A-008-G](../tasks/07_m4a_developer_preview/M4A-008-G-limits-and-non-goals.md) | P1 | implement | TODO | M4A-008-F | Limits and non-goals |
| [M4A-008-V](../tasks/07_m4a_developer_preview/M4A-008-V-verify-build-documentation-and-examples.md) | P1 | verify | TODO | M4A-008-A, M4A-008-B, M4A-008-C, M4A-008-D, M4A-008-E, M4A-008-F, M4A-008-G | Verify Build documentation and examples |
| [M4A-008-Z](../tasks/07_m4a_developer_preview/M4A-008-Z-package-evidence-for-build-documentation-and-examples.md) | P1 | evidence | TODO | M4A-008-V | Package evidence for Build documentation and examples |
| [M4A-009-A](../tasks/07_m4a_developer_preview/M4A-009-A-feature-modules.md) | P0 | implement | TODO | M4A-004-Z, M4A-007-Z, M28-GATE | Feature modules |
| [M4A-009-B](../tasks/07_m4a_developer_preview/M4A-009-B-jwt-like-policy-reference.md) | P0 | implement | TODO | M4A-009-A | JWT-like policy reference |
| [M4A-009-C](../tasks/07_m4a_developer_preview/M4A-009-C-controlled-upstream.md) | P0 | implement | TODO | M4A-009-B | Controlled upstream |
| [M4A-009-D](../tasks/07_m4a_developer_preview/M4A-009-D-metrics-readiness-shutdown.md) | P0 | implement | TODO | M4A-009-C | Metrics/readiness/shutdown |
| [M4A-009-E](../tasks/07_m4a_developer_preview/M4A-009-E-treaty-client.md) | P0 | implement | TODO | M4A-009-D | Treaty client |
| [M4A-009-V](../tasks/07_m4a_developer_preview/M4A-009-V-verify-build-realistic-private-alpha-proof-service.md) | P0 | verify | TODO | M4A-009-A, M4A-009-B, M4A-009-C, M4A-009-D, M4A-009-E | Verify Build realistic private-alpha proof service |
| [M4A-009-Z](../tasks/07_m4a_developer_preview/M4A-009-Z-package-evidence-for-build-realistic-private-alpha-proof-service.md) | P0 | evidence | TODO | M4A-009-V | Package evidence for Build realistic private-alpha proof service |
| [M4A-010-A](../tasks/07_m4a_developer_preview/M4A-010-A-provide-clean-install-packet.md) | P1 | implement | TODO | M4A-003-Z, M4A-008-Z, M4A-009-Z | Provide clean install packet |
| [M4A-010-B](../tasks/07_m4a_developer_preview/M4A-010-B-collect-task-based-feedback.md) | P1 | implement | TODO | M4A-010-A | Collect task-based feedback |
| [M4A-010-C](../tasks/07_m4a_developer_preview/M4A-010-C-classify-p0-p1-p2.md) | P1 | implement | TODO | M4A-010-B | Classify P0/P1/P2 |
| [M4A-010-D](../tasks/07_m4a_developer_preview/M4A-010-D-fix-beta-blocking-findings-and-publish-limitations.md) | P1 | implement | TODO | M4A-010-C | Fix beta-blocking findings and publish limitations |
| [M4A-010-V](../tasks/07_m4a_developer_preview/M4A-010-V-verify-run-invited-developer-alpha-and-close-p0-p1-feedback.md) | P1 | verify | TODO | M4A-010-A, M4A-010-B, M4A-010-C, M4A-010-D | Verify Run invited developer alpha and close P0/P1 feedback |
| [M4A-010-Z](../tasks/07_m4a_developer_preview/M4A-010-Z-package-evidence-for-run-invited-developer-alpha-and-close-p0-p1-feedback.md) | P1 | evidence | TODO | M4A-010-V | Package evidence for Run invited developer alpha and close P0/P1 feedback |
| [M4A-GATE](../gates/M4A-GATE.md) | P0 | gate | TODO | M4A-001-Z, M4A-002-Z, M4A-003-Z, M4A-004-Z, M4A-005-Z, M4A-006-Z, M4A-007-Z, M4A-008-Z, M4A-009-Z, M4A-010-Z | M4A — Developer Preview and Private Alpha exit gate |

## BETA — Public Beta 0.1.0-beta.1

| ID | P | Kind | Status | Dependencies | Task |
|---|---:|---|---|---|---|
| [BETA-001-A](../tasks/08_public_beta/BETA-001-A-add-postgres-compose-seed-reset-controlled-upstream-result-schema-load-generator.md) | P1 | implement | TODO | G0-GATE | Add Postgres compose, seed/reset, controlled upstream, result schema, load generator, and report generator |
| [BETA-001-B](../tasks/08_public_beta/BETA-001-B-pin-candidate-versions.md) | P1 | implement | TODO | BETA-001-A | Pin candidate versions |
| [BETA-001-C](../tasks/08_public_beta/BETA-001-C-define-fairness-checks.md) | P1 | implement | TODO | BETA-001-B | Define fairness checks |
| [BETA-001-D](../tasks/08_public_beta/BETA-001-D-keep-raw-samples.md) | P1 | implement | TODO | BETA-001-C | Keep raw samples |
| [BETA-001-V](../tasks/08_public_beta/BETA-001-V-verify-make-the-real-world-benchmark-harness-executable.md) | P1 | verify | TODO | BETA-001-A, BETA-001-B, BETA-001-C, BETA-001-D | Verify Make the real-world benchmark harness executable |
| [BETA-001-Z](../tasks/08_public_beta/BETA-001-Z-package-evidence-for-make-the-real-world-benchmark-harness-executable.md) | P1 | evidence | TODO | BETA-001-V | Package evidence for Make the real-world benchmark harness executable |
| [BETA-002-A](../tasks/08_public_beta/BETA-002-A-match-sql-pool-jwt-timeouts-logging-responses-compression-and-deployment-limits.md) | P1 | implement | TODO | BETA-001-Z | Match SQL, pool, JWT, timeouts, logging, responses, compression, and deployment limits |
| [BETA-002-B](../tasks/08_public_beta/BETA-002-B-pin-versions.md) | P1 | implement | TODO | BETA-002-A | Pin versions |
| [BETA-002-C](../tasks/08_public_beta/BETA-002-C-add-contract-response-verification.md) | P1 | implement | TODO | BETA-002-B | Add contract-response verification |
| [BETA-002-D](../tasks/08_public_beta/BETA-002-D-document-unavoidable-differences.md) | P1 | implement | TODO | BETA-002-C | Document unavoidable differences |
| [BETA-002-V](../tasks/08_public_beta/BETA-002-V-verify-implement-matched-competitor-candidates.md) | P1 | verify | TODO | BETA-002-A, BETA-002-B, BETA-002-C, BETA-002-D | Verify Implement matched competitor candidates |
| [BETA-002-Z](../tasks/08_public_beta/BETA-002-Z-package-evidence-for-implement-matched-competitor-candidates.md) | P1 | evidence | TODO | BETA-002-V | Package evidence for Implement matched competitor candidates |
| [BETA-003-A](../tasks/08_public_beta/BETA-003-A-run-0-1-5-10-25ms-i-o-payload-matrices-and-cpu-operation-levels.md) | P1 | implement | TODO | BETA-001-Z, M28-GATE, M3-GATE | Run 0/1/5/10/25ms I/O, payload matrices, and CPU operation levels |
| [BETA-003-B](../tasks/08_public_beta/BETA-003-B-measure-first-request-through-steady-state.md) | P1 | implement | TODO | BETA-003-A | Measure first request through steady state |
| [BETA-003-C](../tasks/08_public_beta/BETA-003-C-calculate-cumulative-crossover-request-counts.md) | P1 | implement | TODO | BETA-003-B | Calculate cumulative crossover request counts |
| [BETA-003-D](../tasks/08_public_beta/BETA-003-D-report-losses-honestly.md) | P1 | implement | TODO | BETA-003-C | Report losses honestly |
| [BETA-003-V](../tasks/08_public_beta/BETA-003-V-verify-run-controlled-i-o-and-cpu-jit-crossover-suites.md) | P1 | verify | TODO | BETA-003-A, BETA-003-B, BETA-003-C, BETA-003-D | Verify Run controlled I/O and CPU/JIT crossover suites |
| [BETA-003-Z](../tasks/08_public_beta/BETA-003-Z-package-evidence-for-run-controlled-i-o-and-cpu-jit-crossover-suites.md) | P1 | evidence | TODO | BETA-003-V | Package evidence for Run controlled I/O and CPU/JIT crossover suites |
| [BETA-004-A](../tasks/08_public_beta/BETA-004-A-use-capability-abi.md) | P0 | implement | TODO | M27-GATE, BETA-001-Z | Use capability ABI |
| [BETA-004-B](../tasks/08_public_beta/BETA-004-B-lazy-pool.md) | P0 | implement | TODO | BETA-004-A | Lazy pool |
| [BETA-004-C](../tasks/08_public_beta/BETA-004-C-parameterized-queries-transactions.md) | P0 | implement | TODO | BETA-004-B | Parameterized queries/transactions |
| [BETA-004-D](../tasks/08_public_beta/BETA-004-D-deadline-cancellation-shutdown.md) | P0 | implement | TODO | BETA-004-C | Deadline/cancellation/shutdown |
| [BETA-004-E](../tasks/08_public_beta/BETA-004-E-pool-limits-and-observability.md) | P0 | implement | TODO | BETA-004-D | Pool limits and observability |
| [BETA-004-F](../tasks/08_public_beta/BETA-004-F-no-orm.md) | P0 | implement | TODO | BETA-004-E | No ORM |
| [BETA-004-V](../tasks/08_public_beta/BETA-004-V-verify-implement-optional-first-party-postgres-capability.md) | P0 | verify | TODO | BETA-004-A, BETA-004-B, BETA-004-C, BETA-004-D, BETA-004-E, BETA-004-F | Verify Implement optional first-party Postgres capability |
| [BETA-004-Z](../tasks/08_public_beta/BETA-004-Z-package-evidence-for-implement-optional-first-party-postgres-capability.md) | P0 | evidence | TODO | BETA-004-V | Package evidence for Implement optional first-party Postgres capability |
| [BETA-005-A](../tasks/08_public_beta/BETA-005-A-support-one-approved-jwt-algorithm-profile.md) | P0 | implement | TODO | M27-GATE, M25-GATE | Support one approved JWT algorithm/profile |
| [BETA-005-B](../tasks/08_public_beta/BETA-005-B-key-loading-rotation-hooks.md) | P0 | implement | TODO | BETA-005-A | Key loading/rotation hooks |
| [BETA-005-C](../tasks/08_public_beta/BETA-005-C-expiry-audience-issuer-checks.md) | P0 | implement | TODO | BETA-005-B | Expiry/audience/issuer checks |
| [BETA-005-D](../tasks/08_public_beta/BETA-005-D-typed-401-403-problems.md) | P0 | implement | TODO | BETA-005-C | Typed 401/403 problems |
| [BETA-005-E](../tasks/08_public_beta/BETA-005-E-no-secret-logging.md) | P0 | implement | TODO | BETA-005-D | No secret logging |
| [BETA-005-V](../tasks/08_public_beta/BETA-005-V-verify-implement-jwt-auth-reference-package.md) | P0 | verify | TODO | BETA-005-A, BETA-005-B, BETA-005-C, BETA-005-D, BETA-005-E | Verify Implement JWT/auth reference package |
| [BETA-005-Z](../tasks/08_public_beta/BETA-005-Z-package-evidence-for-implement-jwt-auth-reference-package.md) | P0 | evidence | TODO | BETA-005-V | Package evidence for Implement JWT/auth reference package |
| [BETA-006-A](../tasks/08_public_beta/BETA-006-A-request-route-status-duration.md) | P0 | implement | TODO | M3-GATE, M28-GATE | Request/route/status/duration |
| [BETA-006-B](../tasks/08_public_beta/BETA-006-B-worker-queues-quarantine-replacements.md) | P0 | implement | TODO | BETA-006-A | Worker queues/quarantine/replacements |
| [BETA-006-C](../tasks/08_public_beta/BETA-006-C-fetch-and-db-pools.md) | P0 | implement | TODO | BETA-006-B | Fetch and DB pools |
| [BETA-006-D](../tasks/08_public_beta/BETA-006-D-memory-tasks-slots.md) | P0 | implement | TODO | BETA-006-C | Memory/tasks/slots |
| [BETA-006-E](../tasks/08_public_beta/BETA-006-E-optional-trace-integration-or-trace-ids.md) | P0 | implement | TODO | BETA-006-D | Optional trace integration or trace IDs |
| [BETA-006-F](../tasks/08_public_beta/BETA-006-F-redaction.md) | P0 | implement | TODO | BETA-006-E | Redaction |
| [BETA-006-V](../tasks/08_public_beta/BETA-006-V-verify-implement-beta-observability-baseline.md) | P0 | verify | TODO | BETA-006-A, BETA-006-B, BETA-006-C, BETA-006-D, BETA-006-E, BETA-006-F | Verify Implement beta observability baseline |
| [BETA-006-Z](../tasks/08_public_beta/BETA-006-Z-package-evidence-for-implement-beta-observability-baseline.md) | P0 | evidence | TODO | BETA-006-V | Package evidence for Implement beta observability baseline |
| [BETA-007-A](../tasks/08_public_beta/BETA-007-A-environment-file-configuration.md) | P0 | implement | TODO | M27-GATE | Environment/file configuration |
| [BETA-007-B](../tasks/08_public_beta/BETA-007-B-validation-at-startup.md) | P0 | implement | TODO | BETA-007-A | Validation at startup |
| [BETA-007-C](../tasks/08_public_beta/BETA-007-C-secret-value-wrapper-redaction.md) | P0 | implement | TODO | BETA-007-B | Secret value wrapper/redaction |
| [BETA-007-D](../tasks/08_public_beta/BETA-007-D-profile-specific-settings.md) | P0 | implement | TODO | BETA-007-C | Profile-specific settings |
| [BETA-007-E](../tasks/08_public_beta/BETA-007-E-no-dynamic-code-execution.md) | P0 | implement | TODO | BETA-007-D | No dynamic code execution |
| [BETA-007-V](../tasks/08_public_beta/BETA-007-V-verify-implement-configuration-and-secret-handling.md) | P0 | verify | TODO | BETA-007-A, BETA-007-B, BETA-007-C, BETA-007-D, BETA-007-E | Verify Implement configuration and secret handling |
| [BETA-007-Z](../tasks/08_public_beta/BETA-007-Z-package-evidence-for-implement-configuration-and-secret-handling.md) | P0 | evidence | TODO | BETA-007-V | Package evidence for Implement configuration and secret handling |
| [BETA-008-A](../tasks/08_public_beta/BETA-008-A-trusted-proxy-configuration.md) | P0 | implement | TODO | M3-GATE, BETA-006-Z | Trusted proxy configuration |
| [BETA-008-B](../tasks/08_public_beta/BETA-008-B-forwarded-header-policy.md) | P0 | implement | TODO | BETA-008-A | Forwarded header policy |
| [BETA-008-C](../tasks/08_public_beta/BETA-008-C-liveness-readiness-startup-endpoints.md) | P0 | implement | TODO | BETA-008-B | Liveness/readiness/startup endpoints |
| [BETA-008-D](../tasks/08_public_beta/BETA-008-D-graceful-drain-and-termination.md) | P0 | implement | TODO | BETA-008-C | Graceful drain and termination |
| [BETA-008-E](../tasks/08_public_beta/BETA-008-E-container-example.md) | P0 | implement | TODO | BETA-008-D | Container example |
| [BETA-008-V](../tasks/08_public_beta/BETA-008-V-verify-implement-reverse-proxy-drain-and-deployment-semantics.md) | P0 | verify | TODO | BETA-008-A, BETA-008-B, BETA-008-C, BETA-008-D, BETA-008-E | Verify Implement reverse-proxy, drain, and deployment semantics |
| [BETA-008-Z](../tasks/08_public_beta/BETA-008-Z-package-evidence-for-implement-reverse-proxy-drain-and-deployment-semantics.md) | P0 | evidence | TODO | BETA-008-V | Package evidence for Implement reverse-proxy, drain, and deployment semantics |
| [BETA-009-A](../tasks/08_public_beta/BETA-009-A-run-fuzz-suites-for-pack-router-schema-bridge-http.md) | P0 | implement | TODO | M28-GATE, M3-GATE, BETA-004-Z, BETA-005-Z, BETA-007-Z | Run fuzz suites for pack/router/schema/bridge/HTTP |
| [BETA-009-B](../tasks/08_public_beta/BETA-009-B-dependency-vulnerability-and-license-scan.md) | P0 | implement | TODO | BETA-009-A | Dependency vulnerability and license scan |
| [BETA-009-C](../tasks/08_public_beta/BETA-009-C-threat-model-review.md) | P0 | implement | TODO | BETA-009-B | Threat-model review |
| [BETA-009-D](../tasks/08_public_beta/BETA-009-D-chaos-tests-for-upstream-db-worker-poison.md) | P0 | implement | TODO | BETA-009-C | Chaos tests for upstream/DB/worker poison |
| [BETA-009-E](../tasks/08_public_beta/BETA-009-E-no-known-critical-high-exploitable-issue.md) | P0 | implement | TODO | BETA-009-D | No known critical/high exploitable issue |
| [BETA-009-V](../tasks/08_public_beta/BETA-009-V-verify-run-beta-security-and-reliability-baseline.md) | P0 | verify | TODO | BETA-009-A, BETA-009-B, BETA-009-C, BETA-009-D, BETA-009-E | Verify Run beta security and reliability baseline |
| [BETA-009-Z](../tasks/08_public_beta/BETA-009-Z-package-evidence-for-run-beta-security-and-reliability-baseline.md) | P0 | evidence | TODO | BETA-009-V | Package evidence for Run beta security and reliability baseline |
| [BETA-010-A](../tasks/08_public_beta/BETA-010-A-linux-x86-64-glibc-mandatory-working-assumption.md) | P1 | implement | TODO | M26-GATE, M4A-002-Z | Linux x86_64 glibc mandatory working assumption |
| [BETA-010-B](../tasks/08_public_beta/BETA-010-B-linux-arm64-glibc-when-ci-is-available.md) | P1 | implement | TODO | BETA-010-A | Linux ARM64 glibc when CI is available |
| [BETA-010-C](../tasks/08_public_beta/BETA-010-C-npm-packages-under-beta-tag.md) | P1 | implement | TODO | BETA-010-B | npm packages under beta tag |
| [BETA-010-D](../tasks/08_public_beta/BETA-010-D-runtime-binary-qpack-tools.md) | P1 | implement | TODO | BETA-010-C | Runtime binary/QPack tools |
| [BETA-010-E](../tasks/08_public_beta/BETA-010-E-clean-install-tests.md) | P1 | implement | TODO | BETA-010-D | Clean install tests |
| [BETA-010-V](../tasks/08_public_beta/BETA-010-V-verify-create-supported-beta-platform-and-packaging-matrix.md) | P1 | verify | TODO | BETA-010-A, BETA-010-B, BETA-010-C, BETA-010-D, BETA-010-E | Verify Create supported beta platform and packaging matrix |
| [BETA-010-Z](../tasks/08_public_beta/BETA-010-Z-package-evidence-for-create-supported-beta-platform-and-packaging-matrix.md) | P1 | evidence | TODO | BETA-010-V | Package evidence for Create supported beta platform and packaging matrix |
| [BETA-011-A](../tasks/08_public_beta/BETA-011-A-use-semver-prerelease.md) | P1 | implement | TODO | M4A-GATE, BETA-010-Z | Use SemVer prerelease |
| [BETA-011-B](../tasks/08_public_beta/BETA-011-B-publish-next-beta-tag.md) | P1 | implement | TODO | BETA-011-A | Publish `next`/beta tag |
| [BETA-011-C](../tasks/08_public_beta/BETA-011-C-generate-changelog-and-migration-notes.md) | P1 | implement | TODO | BETA-011-B | Generate changelog and migration notes |
| [BETA-011-D](../tasks/08_public_beta/BETA-011-D-create-github-style-release-packet.md) | P1 | implement | TODO | BETA-011-C | Create GitHub-style release packet |
| [BETA-011-E](../tasks/08_public_beta/BETA-011-E-support-yanking-rollback.md) | P1 | implement | TODO | BETA-011-D | Support yanking/rollback |
| [BETA-011-V](../tasks/08_public_beta/BETA-011-V-verify-automate-beta-publishing-and-versioning.md) | P1 | verify | TODO | BETA-011-A, BETA-011-B, BETA-011-C, BETA-011-D, BETA-011-E | Verify Automate beta publishing and versioning |
| [BETA-011-Z](../tasks/08_public_beta/BETA-011-Z-package-evidence-for-automate-beta-publishing-and-versioning.md) | P1 | evidence | TODO | BETA-011-V | Package evidence for Automate beta publishing and versioning |
| [BETA-012-A](../tasks/08_public_beta/BETA-012-A-installation.md) | P1 | implement | TODO | M4A-GATE, BETA-004-Z, BETA-005-Z, BETA-008-Z | Installation |
| [BETA-012-B](../tasks/08_public_beta/BETA-012-B-quickstart.md) | P1 | implement | TODO | BETA-012-A | Quickstart |
| [BETA-012-C](../tasks/08_public_beta/BETA-012-C-architecture.md) | P1 | implement | TODO | BETA-012-B | Architecture |
| [BETA-012-D](../tasks/08_public_beta/BETA-012-D-contracts-treaty.md) | P1 | implement | TODO | BETA-012-C | Contracts/Treaty |
| [BETA-012-E](../tasks/08_public_beta/BETA-012-E-fetch-postgres-auth.md) | P1 | implement | TODO | BETA-012-D | Fetch/Postgres/auth |
| [BETA-012-F](../tasks/08_public_beta/BETA-012-F-deployment.md) | P1 | implement | TODO | BETA-012-E | Deployment |
| [BETA-012-G](../tasks/08_public_beta/BETA-012-G-troubleshooting.md) | P1 | implement | TODO | BETA-012-F | Troubleshooting |
| [BETA-012-H](../tasks/08_public_beta/BETA-012-H-performance-methodology.md) | P1 | implement | TODO | BETA-012-G | Performance methodology |
| [BETA-012-I](../tasks/08_public_beta/BETA-012-I-limitations-non-goals.md) | P1 | implement | TODO | BETA-012-H | Limitations/non-goals |
| [BETA-012-V](../tasks/08_public_beta/BETA-012-V-verify-complete-beta-documentation-and-limitations.md) | P1 | verify | TODO | BETA-012-A, BETA-012-B, BETA-012-C, BETA-012-D, BETA-012-E, BETA-012-F, BETA-012-G, BETA-012-H, BETA-012-I | Verify Complete beta documentation and limitations |
| [BETA-012-Z](../tasks/08_public_beta/BETA-012-Z-package-evidence-for-complete-beta-documentation-and-limitations.md) | P1 | evidence | TODO | BETA-012-V | Package evidence for Complete beta documentation and limitations |
| [BETA-013-A](../tasks/08_public_beta/BETA-013-A-run-at-least-two-hour-mixed-workload-and-at-least-one-million-requests-on-refere.md) | P0 | implement | TODO | BETA-004-Z, BETA-005-Z, BETA-006-Z, BETA-008-Z, BETA-009-Z | Run at least two-hour mixed workload and at least one million requests on reference platform |
| [BETA-013-B](../tasks/08_public_beta/BETA-013-B-include-fetch-db-auth-timeouts-cancellation-worker-replacement-and-reload.md) | P0 | implement | TODO | BETA-013-A | Include fetch, DB, auth, timeouts, cancellation, worker replacement, and reload |
| [BETA-013-C](../tasks/08_public_beta/BETA-013-C-track-rss-heap-slots-tasks-queues-pools-and-errors.md) | P0 | implement | TODO | BETA-013-B | Track RSS, heap, slots, tasks, queues, pools, and errors |
| [BETA-013-D](../tasks/08_public_beta/BETA-013-D-analyze-retained-growth.md) | P0 | implement | TODO | BETA-013-C | Analyze retained growth |
| [BETA-013-V](../tasks/08_public_beta/BETA-013-V-verify-run-beta-soak-and-leak-qualification.md) | P0 | verify | TODO | BETA-013-A, BETA-013-B, BETA-013-C, BETA-013-D | Verify Run beta soak and leak qualification |
| [BETA-013-Z](../tasks/08_public_beta/BETA-013-Z-package-evidence-for-run-beta-soak-and-leak-qualification.md) | P0 | evidence | TODO | BETA-013-V | Package evidence for Run beta soak and leak qualification |
| [BETA-014-A](../tasks/08_public_beta/BETA-014-A-include-cold-start-categories-warm-microbenchmarks-real-db-auth-i-o-cpu-jit-cros.md) | P1 | implement | TODO | BETA-002-Z, BETA-003-Z, BETA-004-Z, BETA-005-Z, BETA-013-Z | Include cold start categories, warm microbenchmarks, real DB/auth/I/O, CPU/JIT crossover, cost-normalized metrics, and limitations |
| [BETA-014-B](../tasks/08_public_beta/BETA-014-B-pin-all-candidates-artifacts.md) | P1 | implement | TODO | BETA-014-A | Pin all candidates/artifacts |
| [BETA-014-C](../tasks/08_public_beta/BETA-014-C-retain-raw-data.md) | P1 | implement | TODO | BETA-014-B | Retain raw data |
| [BETA-014-D](../tasks/08_public_beta/BETA-014-D-have-wording-reviewed.md) | P1 | implement | TODO | BETA-014-C | Have wording reviewed |
| [BETA-014-V](../tasks/08_public_beta/BETA-014-V-verify-publish-canonical-beta-benchmark-report.md) | P1 | verify | TODO | BETA-014-A, BETA-014-B, BETA-014-C, BETA-014-D | Verify Publish canonical beta benchmark report |
| [BETA-014-Z](../tasks/08_public_beta/BETA-014-Z-package-evidence-for-publish-canonical-beta-benchmark-report.md) | P1 | evidence | TODO | BETA-014-V | Package evidence for Publish canonical beta benchmark report |
| [BETA-015-A](../tasks/08_public_beta/BETA-015-A-source-zip.md) | P0 | implement | TODO | BETA-009-Z, BETA-010-Z, BETA-011-Z, BETA-013-Z, BETA-014-Z | Source ZIP |
| [BETA-015-B](../tasks/08_public_beta/BETA-015-B-git-bundle.md) | P0 | implement | TODO | BETA-015-A | Git bundle |
| [BETA-015-C](../tasks/08_public_beta/BETA-015-C-linux-binaries.md) | P0 | implement | TODO | BETA-015-B | Linux binaries |
| [BETA-015-D](../tasks/08_public_beta/BETA-015-D-npm-package-tarballs.md) | P0 | implement | TODO | BETA-015-C | npm package tarballs |
| [BETA-015-E](../tasks/08_public_beta/BETA-015-E-qpack-tools.md) | P0 | implement | TODO | BETA-015-D | QPack tools |
| [BETA-015-F](../tasks/08_public_beta/BETA-015-F-sbom.md) | P0 | implement | TODO | BETA-015-E | SBOM |
| [BETA-015-G](../tasks/08_public_beta/BETA-015-G-checksums.md) | P0 | implement | TODO | BETA-015-F | Checksums |
| [BETA-015-H](../tasks/08_public_beta/BETA-015-H-review-evidence-indexes.md) | P0 | implement | TODO | BETA-015-G | Review/evidence indexes |
| [BETA-015-I](../tasks/08_public_beta/BETA-015-I-known-limitations.md) | P0 | implement | TODO | BETA-015-H | Known limitations |
| [BETA-015-V](../tasks/08_public_beta/BETA-015-V-verify-generate-beta-release-evidence-sbom-and-checksums.md) | P0 | verify | TODO | BETA-015-A, BETA-015-B, BETA-015-C, BETA-015-D, BETA-015-E, BETA-015-F, BETA-015-G, BETA-015-H, BETA-015-I | Verify Generate beta release evidence, SBOM, and checksums |
| [BETA-015-Z](../tasks/08_public_beta/BETA-015-Z-package-evidence-for-generate-beta-release-evidence-sbom-and-checksums.md) | P0 | evidence | TODO | BETA-015-V | Package evidence for Generate beta release evidence, SBOM, and checksums |
| [BETA-016-A](../tasks/08_public_beta/BETA-016-A-fresh-linux-vm-container.md) | P1 | implement | TODO | BETA-011-Z, BETA-012-Z, BETA-015-Z | Fresh Linux VM/container |
| [BETA-016-B](../tasks/08_public_beta/BETA-016-B-install-cli-runtime.md) | P1 | implement | TODO | BETA-016-A | Install CLI/runtime |
| [BETA-016-C](../tasks/08_public_beta/BETA-016-C-scaffold-app.md) | P1 | implement | TODO | BETA-016-B | Scaffold app |
| [BETA-016-D](../tasks/08_public_beta/BETA-016-D-run-tests-dev-build.md) | P1 | implement | TODO | BETA-016-C | Run tests/dev/build |
| [BETA-016-E](../tasks/08_public_beta/BETA-016-E-deploy-proof-service.md) | P1 | implement | TODO | BETA-016-D | Deploy proof service |
| [BETA-016-F](../tasks/08_public_beta/BETA-016-F-use-treaty-client.md) | P1 | implement | TODO | BETA-016-E | Use Treaty client |
| [BETA-016-V](../tasks/08_public_beta/BETA-016-V-verify-run-external-clean-install-and-tutorial-verification.md) | P1 | verify | TODO | BETA-016-A, BETA-016-B, BETA-016-C, BETA-016-D, BETA-016-E, BETA-016-F | Verify Run external clean-install and tutorial verification |
| [BETA-016-Z](../tasks/08_public_beta/BETA-016-Z-package-evidence-for-run-external-clean-install-and-tutorial-verification.md) | P1 | evidence | TODO | BETA-016-V | Package evidence for Run external clean-install and tutorial verification |
| [BETA-017-A](../tasks/08_public_beta/BETA-017-A-repository-organization.md) | P0 | implement | TODO | — | Repository/organization |
| [BETA-017-B](../tasks/08_public_beta/BETA-017-B-license-contribution-model.md) | P0 | implement | TODO | BETA-017-A | License/contribution model |
| [BETA-017-C](../tasks/08_public_beta/BETA-017-C-release-authority.md) | P0 | implement | TODO | BETA-017-B | Release authority |
| [BETA-017-D](../tasks/08_public_beta/BETA-017-D-security-contact.md) | P0 | implement | TODO | BETA-017-C | Security contact |
| [BETA-017-E](../tasks/08_public_beta/BETA-017-E-supported-beta-platforms.md) | P0 | implement | TODO | BETA-017-D | Supported beta platforms |
| [BETA-017-F](../tasks/08_public_beta/BETA-017-F-reverse-proxy-first-statement.md) | P0 | implement | TODO | BETA-017-E | Reverse-proxy-first statement |
| [BETA-017-G](../tasks/08_public_beta/BETA-017-G-public-benchmark-wording.md) | P0 | implement | TODO | BETA-017-F | Public benchmark wording |
| [BETA-017-V](../tasks/08_public_beta/BETA-017-V-verify-resolve-beta-owner-decisions.md) | P0 | verify | TODO | BETA-017-A, BETA-017-B, BETA-017-C, BETA-017-D, BETA-017-E, BETA-017-F, BETA-017-G | Verify Resolve beta owner decisions |
| [BETA-017-Z](../tasks/08_public_beta/BETA-017-Z-package-evidence-for-resolve-beta-owner-decisions.md) | P0 | evidence | TODO | BETA-017-V | Package evidence for Resolve beta owner decisions |
| [BETA-GATE](../gates/BETA-GATE.md) | P0 | gate | TODO | BETA-001-Z, BETA-002-Z, BETA-003-Z, BETA-004-Z, BETA-005-Z, BETA-006-Z, BETA-007-Z, BETA-008-Z, BETA-009-Z, BETA-010-Z, BETA-011-Z, BETA-012-Z, BETA-013-Z, BETA-014-Z, BETA-015-Z, BETA-016-Z, BETA-017-Z | Public Beta Readiness and Release exit gate |
