# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M24-001-V; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M24-001-V — Verify Freeze ingress ownership and backpressure design](tasks/01_m24_zero_copy_ingress/M24-001-V-verify-freeze-ingress-ownership-and-backpressure-design.md) — deps: M24-001-A, M24-001-B, M24-001-C, M24-001-D — #64
2. [M24-001-Z — Package evidence for Freeze ingress ownership and backpressure design](tasks/01_m24_zero_copy_ingress/M24-001-Z-package-evidence-for-freeze-ingress-ownership-and-backpressure-design.md) — deps: M24-001-V — #65
3. [M24-002-D — Bypass request-object creation for policy-free routes that need no request fields](tasks/01_m24_zero_copy_ingress/M24-002-D-bypass-request-object-creation-for-policy-free-routes-that-need-no-request-field.md) — deps: M24-002-C — #69
4. [M24-002-V — Verify Route before request materialization](tasks/01_m24_zero_copy_ingress/M24-002-V-verify-route-before-request-materialization.md) — deps: M24-002-A, M24-002-B, M24-002-C, M24-002-D — #70
5. [M24-002-Z — Package evidence for Route before request materialization](tasks/01_m24_zero_copy_ingress/M24-002-Z-package-evidence-for-route-before-request-materialization.md) — deps: M24-002-V — #71
6. [M24-003-A — Move request slots into each QuickJS worker](tasks/01_m24_zero_copy_ingress/M24-003-A-move-request-slots-into-each-quickjs-worker.md) — deps: M24-001-Z, M24-002-Z — #72
7. [M24-003-B — Use slot plus generation handles](tasks/01_m24_zero_copy_ingress/M24-003-B-use-slot-plus-generation-handles.md) — deps: M24-003-A — #73
8. [M24-003-C — Invalidate at settlement, timeout, cancellation, quarantine, and shutdown](tasks/01_m24_zero_copy_ingress/M24-003-C-invalidate-at-settlement-timeout-cancellation-quarantine-and-shutdown.md) — deps: M24-003-B — #74
9. [M24-003-D — Reject stale or cross-worker handles deterministically](tasks/01_m24_zero_copy_ingress/M24-003-D-reject-stale-or-cross-worker-handles-deterministically.md) — deps: M24-003-C — #75
10. [M24-003-V — Verify Implement worker-local generation-checked request slab](tasks/01_m24_zero_copy_ingress/M24-003-V-verify-implement-worker-local-generation-checked-request-slab.md) — deps: M24-003-A, M24-003-B, M24-003-C, M24-003-D — #76
11. [M24-003-Z — Package evidence for Implement worker-local generation-checked request slab](tasks/01_m24_zero_copy_ingress/M24-003-Z-package-evidence-for-implement-worker-local-generation-checked-request-slab.md) — deps: M24-003-V — #77
12. [M24-004-A — Store capture start/end ranges against the URI path](tasks/01_m24_zero_copy_ingress/M24-004-A-store-capture-start-end-ranges-against-the-uri-path.md) — deps: M24-002-Z, M24-003-Z — #78
13. [M24-004-B — Bind route-specific parameter names after RouteId selection](tasks/01_m24_zero_copy_ingress/M24-004-B-bind-route-specific-parameter-names-after-routeid-selection.md) — deps: M24-004-A — #79
14. [M24-004-C — Validate numeric/UUID formats directly from bytes where possible](tasks/01_m24_zero_copy_ingress/M24-004-C-validate-numeric-uuid-formats-directly-from-bytes-where-possible.md) — deps: M24-004-B — #80
15. [M24-004-D — Materialize JS strings lazily](tasks/01_m24_zero_copy_ingress/M24-004-D-materialize-js-strings-lazily.md) — deps: M24-004-C — #81
16. [M24-004-V — Verify Capture path parameters as byte ranges](tasks/01_m24_zero_copy_ingress/M24-004-V-verify-capture-path-parameters-as-byte-ranges.md) — deps: M24-004-A, M24-004-B, M24-004-C, M24-004-D — #82
17. [M24-004-Z — Package evidence for Capture path parameters as byte ranges](tasks/01_m24_zero_copy_ingress/M24-004-Z-package-evidence-for-capture-path-parameters-as-byte-ranges.md) — deps: M24-004-V — #83
18. [M24-005-A — Compile header-name IDs into RoutePlan](tasks/01_m24_zero_copy_ingress/M24-005-A-compile-header-name-ids-into-routeplan.md) — deps: M24-003-Z — #84
19. [M24-005-B — Read header values by ID on demand](tasks/01_m24_zero_copy_ingress/M24-005-B-read-header-values-by-id-on-demand.md) — deps: M24-005-A — #85
20. [M24-005-C — Define duplicate header behavior and byte/string conversion](tasks/01_m24_zero_copy_ingress/M24-005-C-define-duplicate-header-behavior-and-byte-string-conversion.md) — deps: M24-005-B — #86
21. [M24-005-D — Keep full Headers escape hatch explicit and costed](tasks/01_m24_zero_copy_ingress/M24-005-D-keep-full-headers-escape-hatch-explicit-and-costed.md) — deps: M24-005-C — #87
22. [M24-005-V — Verify Implement declared-header lazy access](tasks/01_m24_zero_copy_ingress/M24-005-V-verify-implement-declared-header-lazy-access.md) — deps: M24-005-A, M24-005-B, M24-005-C, M24-005-D — #88
23. [M24-005-Z — Package evidence for Implement declared-header lazy access](tasks/01_m24_zero_copy_ingress/M24-005-Z-package-evidence-for-implement-declared-header-lazy-access.md) — deps: M24-005-V — #89
24. [M24-006-A — Compile query/cookie field IDs](tasks/01_m24_zero_copy_ingress/M24-006-A-compile-query-cookie-field-ids.md) — deps: M24-003-Z, M24-004-Z — #90
25. [M24-006-B — Provide repeated-key policy](tasks/01_m24_zero_copy_ingress/M24-006-B-provide-repeated-key-policy.md) — deps: M24-006-A — #91
