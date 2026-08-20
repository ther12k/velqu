# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M24-001-V; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M24-001-V — Verify Freeze ingress ownership and backpressure design](tasks/01_m24_zero_copy_ingress/M24-001-V-verify-freeze-ingress-ownership-and-backpressure-design.md) — deps: M24-001-A, M24-001-B, M24-001-C, M24-001-D — #64
2. [M24-001-Z — Package evidence for Freeze ingress ownership and backpressure design](tasks/01_m24_zero_copy_ingress/M24-001-Z-package-evidence-for-freeze-ingress-ownership-and-backpressure-design.md) — deps: M24-001-V — #65
3. [M24-004-A — Store capture start/end ranges against the URI path](tasks/01_m24_zero_copy_ingress/M24-004-A-store-capture-start-end-ranges-against-the-uri-path.md) — deps: M24-002-Z, M24-003-Z — #78
4. [M24-004-B — Bind route-specific parameter names after RouteId selection](tasks/01_m24_zero_copy_ingress/M24-004-B-bind-route-specific-parameter-names-after-routeid-selection.md) — deps: M24-004-A — #79
5. [M24-004-C — Validate numeric/UUID formats directly from bytes where possible](tasks/01_m24_zero_copy_ingress/M24-004-C-validate-numeric-uuid-formats-directly-from-bytes-where-possible.md) — deps: M24-004-B — #80
6. [M24-004-D — Materialize JS strings lazily](tasks/01_m24_zero_copy_ingress/M24-004-D-materialize-js-strings-lazily.md) — deps: M24-004-C — #81
7. [M24-004-V — Verify Capture path parameters as byte ranges](tasks/01_m24_zero_copy_ingress/M24-004-V-verify-capture-path-parameters-as-byte-ranges.md) — deps: M24-004-A, M24-004-B, M24-004-C, M24-004-D — #82
8. [M24-004-Z — Package evidence for Capture path parameters as byte ranges](tasks/01_m24_zero_copy_ingress/M24-004-Z-package-evidence-for-capture-path-parameters-as-byte-ranges.md) — deps: M24-004-V — #83
9. [M24-005-A — Compile header-name IDs into RoutePlan](tasks/01_m24_zero_copy_ingress/M24-005-A-compile-header-name-ids-into-routeplan.md) — deps: M24-003-Z — #84
10. [M24-005-B — Read header values by ID on demand](tasks/01_m24_zero_copy_ingress/M24-005-B-read-header-values-by-id-on-demand.md) — deps: M24-005-A — #85
11. [M24-005-C — Define duplicate header behavior and byte/string conversion](tasks/01_m24_zero_copy_ingress/M24-005-C-define-duplicate-header-behavior-and-byte-string-conversion.md) — deps: M24-005-B — #86
12. [M24-005-D — Keep full Headers escape hatch explicit and costed](tasks/01_m24_zero_copy_ingress/M24-005-D-keep-full-headers-escape-hatch-explicit-and-costed.md) — deps: M24-005-C — #87
13. [M24-005-V — Verify Implement declared-header lazy access](tasks/01_m24_zero_copy_ingress/M24-005-V-verify-implement-declared-header-lazy-access.md) — deps: M24-005-A, M24-005-B, M24-005-C, M24-005-D — #88
14. [M24-005-Z — Package evidence for Implement declared-header lazy access](tasks/01_m24_zero_copy_ingress/M24-005-Z-package-evidence-for-implement-declared-header-lazy-access.md) — deps: M24-005-V — #89
15. [M24-006-A — Compile query/cookie field IDs](tasks/01_m24_zero_copy_ingress/M24-006-A-compile-query-cookie-field-ids.md) — deps: M24-003-Z, M24-004-Z — #90
16. [M24-006-B — Provide repeated-key policy](tasks/01_m24_zero_copy_ingress/M24-006-B-provide-repeated-key-policy.md) — deps: M24-006-A — #91
17. [M24-006-C — Define percent decoding and invalid-byte behavior](tasks/01_m24_zero_copy_ingress/M24-006-C-define-percent-decoding-and-invalid-byte-behavior.md) — deps: M24-006-B — #92
18. [M24-006-D — Cache decoded fields per request slot](tasks/01_m24_zero_copy_ingress/M24-006-D-cache-decoded-fields-per-request-slot.md) — deps: M24-006-C — #93
19. [M24-006-V — Verify Implement lazy query and cookie decoding](tasks/01_m24_zero_copy_ingress/M24-006-V-verify-implement-lazy-query-and-cookie-decoding.md) — deps: M24-006-A, M24-006-B, M24-006-C, M24-006-D — #94
20. [M24-006-Z — Package evidence for Implement lazy query and cookie decoding](tasks/01_m24_zero_copy_ingress/M24-006-Z-package-evidence-for-implement-lazy-query-and-cookie-decoding.md) — deps: M24-006-V — #95
21. [M24-007-A — Drive body behavior from RoutePlan, not HTTP method](tasks/01_m24_zero_copy_ingress/M24-007-A-drive-body-behavior-from-routeplan-not-http-method.md) — deps: M24-001-Z, M24-003-Z — #96
22. [M24-007-B — Use Bytes and avoid Bytes-to-Vec copies](tasks/01_m24_zero_copy_ingress/M24-007-B-use-bytes-and-avoid-bytes-to-vec-copies.md) — deps: M24-007-A — #97
23. [M24-007-C — Enforce content length and streaming limits](tasks/01_m24_zero_copy_ingress/M24-007-C-enforce-content-length-and-streaming-limits.md) — deps: M24-007-B — #98
24. [M24-007-D — Cache one decoded representation and reject incompatible second reads](tasks/01_m24_zero_copy_ingress/M24-007-D-cache-one-decoded-representation-and-reject-incompatible-second-reads.md) — deps: M24-007-C — #99
25. [M24-007-V — Verify Implement bounded read-once body admission](tasks/01_m24_zero_copy_ingress/M24-007-V-verify-implement-bounded-read-once-body-admission.md) — deps: M24-007-A, M24-007-B, M24-007-C, M24-007-D — #100
