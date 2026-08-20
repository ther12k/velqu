# Next 25 Dependency-Safe Tasks

G0 is closed and M24-001-A is accepted (ADR-0021). The next dependency-ready implementation task is M24-001-B; this short queue lists the first 25 unchecked packets.

1. [M24-001-B — Specify body ownership, queue admission, disconnect cancellation, and request-slot lifecycle](tasks/01_m24_zero_copy_ingress/M24-001-B-specify-body-ownership-queue-admission-disconnect-cancellation-and-request-slot.md) — deps: M24-001-A
2. [M24-001-C — Define no-copy and bounded-copy boundaries](tasks/01_m24_zero_copy_ingress/M24-001-C-define-no-copy-and-bounded-copy-boundaries.md) — deps: M24-001-B
3. [M24-001-D — Define overload responses and metrics](tasks/01_m24_zero_copy_ingress/M24-001-D-define-overload-responses-and-metrics.md) — deps: M24-001-C
4. [M24-001-V — Verify Freeze ingress ownership and backpressure design](tasks/01_m24_zero_copy_ingress/M24-001-V-verify-freeze-ingress-ownership-and-backpressure-design.md) — deps: M24-001-A, M24-001-B, M24-001-C, M24-001-D
5. [M24-001-Z — Package evidence for Freeze ingress ownership and backpressure design](tasks/01_m24_zero_copy_ingress/M24-001-Z-package-evidence-for-freeze-ingress-ownership-and-backpressure-design.md) — deps: M24-001-V
6. [M24-002-A — Keep Method, Uri, HeaderMap, and body stream in native forms](tasks/01_m24_zero_copy_ingress/M24-002-A-keep-method-uri-headermap-and-body-stream-in-native-forms.md) — deps: M24-001-Z
7. [M24-002-B — Match RouteId using method/path before creating request metadata](tasks/01_m24_zero_copy_ingress/M24-002-B-match-routeid-using-method-path-before-creating-request-metadata.md) — deps: M24-002-A
8. [M24-002-C — Read FieldNeeds from RoutePlan](tasks/01_m24_zero_copy_ingress/M24-002-C-read-fieldneeds-from-routeplan.md) — deps: M24-002-B
9. [M24-002-D — Bypass request-object creation for policy-free routes that need no request fields](tasks/01_m24_zero_copy_ingress/M24-002-D-bypass-request-object-creation-for-policy-free-routes-that-need-no-request-field.md) — deps: M24-002-C
10. [M24-002-V — Verify Route before request materialization](tasks/01_m24_zero_copy_ingress/M24-002-V-verify-route-before-request-materialization.md) — deps: M24-002-A, M24-002-B, M24-002-C, M24-002-D
11. [M24-002-Z — Package evidence for Route before request materialization](tasks/01_m24_zero_copy_ingress/M24-002-Z-package-evidence-for-route-before-request-materialization.md) — deps: M24-002-V
12. [M24-003-A — Move request slots into each QuickJS worker](tasks/01_m24_zero_copy_ingress/M24-003-A-move-request-slots-into-each-quickjs-worker.md) — deps: M24-001-Z, M24-002-Z
13. [M24-003-B — Use slot plus generation handles](tasks/01_m24_zero_copy_ingress/M24-003-B-use-slot-plus-generation-handles.md) — deps: M24-003-A
14. [M24-003-C — Invalidate at settlement, timeout, cancellation, quarantine, and shutdown](tasks/01_m24_zero_copy_ingress/M24-003-C-invalidate-at-settlement-timeout-cancellation-quarantine-and-shutdown.md) — deps: M24-003-B
15. [M24-003-D — Reject stale or cross-worker handles deterministically](tasks/01_m24_zero_copy_ingress/M24-003-D-reject-stale-or-cross-worker-handles-deterministically.md) — deps: M24-003-C
16. [M24-003-V — Verify Implement worker-local generation-checked request slab](tasks/01_m24_zero_copy_ingress/M24-003-V-verify-implement-worker-local-generation-checked-request-slab.md) — deps: M24-003-A, M24-003-B, M24-003-C, M24-003-D
17. [M24-003-Z — Package evidence for Implement worker-local generation-checked request slab](tasks/01_m24_zero_copy_ingress/M24-003-Z-package-evidence-for-implement-worker-local-generation-checked-request-slab.md) — deps: M24-003-V
18. [M24-004-A — Store capture start/end ranges against the URI path](tasks/01_m24_zero_copy_ingress/M24-004-A-store-capture-start-end-ranges-against-the-uri-path.md) — deps: M24-002-Z, M24-003-Z
19. [M24-004-B — Bind route-specific parameter names after RouteId selection](tasks/01_m24_zero_copy_ingress/M24-004-B-bind-route-specific-parameter-names-after-routeid-selection.md) — deps: M24-004-A
20. [M24-004-C — Validate numeric/UUID formats directly from bytes where possible](tasks/01_m24_zero_copy_ingress/M24-004-C-validate-numeric-uuid-formats-directly-from-bytes-where-possible.md) — deps: M24-004-B
21. [M24-004-D — Materialize JS strings lazily](tasks/01_m24_zero_copy_ingress/M24-004-D-materialize-js-strings-lazily.md) — deps: M24-004-C
22. [M24-004-V — Verify Capture path parameters as byte ranges](tasks/01_m24_zero_copy_ingress/M24-004-V-verify-capture-path-parameters-as-byte-ranges.md) — deps: M24-004-A, M24-004-B, M24-004-C, M24-004-D
23. [M24-004-Z — Package evidence for Capture path parameters as byte ranges](tasks/01_m24_zero_copy_ingress/M24-004-Z-package-evidence-for-capture-path-parameters-as-byte-ranges.md) — deps: M24-004-V
24. [M24-005-A — Compile header-name IDs into RoutePlan](tasks/01_m24_zero_copy_ingress/M24-005-A-compile-header-name-ids-into-routeplan.md) — deps: M24-003-Z
25. [M24-005-B — Read header values by ID on demand](tasks/01_m24_zero_copy_ingress/M24-005-B-read-header-values-by-id-on-demand.md) — deps: M24-005-A
