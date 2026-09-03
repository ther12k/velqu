# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is BETA-005-A; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [BETA-005-A — Support one approved JWT algorithm/profile](tasks/08_public_beta/BETA-005-A-support-one-approved-jwt-algorithm-profile.md) — deps: M27-GATE, M25-GATE — #524
2. [BETA-005-B — Key loading/rotation hooks](tasks/08_public_beta/BETA-005-B-key-loading-rotation-hooks.md) — deps: BETA-005-A — #525
3. [BETA-005-C — Expiry/audience/issuer checks](tasks/08_public_beta/BETA-005-C-expiry-audience-issuer-checks.md) — deps: BETA-005-B — #526
4. [BETA-005-D — Typed 401/403 problems](tasks/08_public_beta/BETA-005-D-typed-401-403-problems.md) — deps: BETA-005-C — #527
5. [BETA-005-E — No secret logging](tasks/08_public_beta/BETA-005-E-no-secret-logging.md) — deps: BETA-005-D — #528
6. [BETA-005-V — Verify Implement JWT/auth reference package](tasks/08_public_beta/BETA-005-V-verify-implement-jwt-auth-reference-package.md) — deps: BETA-005-A, BETA-005-B, BETA-005-C, BETA-005-D, BETA-005-E — #529
7. [BETA-005-Z — Package evidence for Implement JWT/auth reference package](tasks/08_public_beta/BETA-005-Z-package-evidence-for-implement-jwt-auth-reference-package.md) — deps: BETA-005-V — #530
8. [BETA-006-A — Request/route/status/duration](tasks/08_public_beta/BETA-006-A-request-route-status-duration.md) — deps: M3-GATE, M28-GATE — #531
9. [BETA-006-B — Worker queues/quarantine/replacements](tasks/08_public_beta/BETA-006-B-worker-queues-quarantine-replacements.md) — deps: BETA-006-A — #532
10. [BETA-006-C — Fetch and DB pools](tasks/08_public_beta/BETA-006-C-fetch-and-db-pools.md) — deps: BETA-006-B — #533
11. [BETA-006-D — Memory/tasks/slots](tasks/08_public_beta/BETA-006-D-memory-tasks-slots.md) — deps: BETA-006-C — #534
12. [BETA-006-E — Optional trace integration or trace IDs](tasks/08_public_beta/BETA-006-E-optional-trace-integration-or-trace-ids.md) — deps: BETA-006-D — #535
13. [BETA-006-F — Redaction](tasks/08_public_beta/BETA-006-F-redaction.md) — deps: BETA-006-E — #536
14. [BETA-006-V — Verify Implement beta observability baseline](tasks/08_public_beta/BETA-006-V-verify-implement-beta-observability-baseline.md) — deps: BETA-006-A, BETA-006-B, BETA-006-C, BETA-006-D, BETA-006-E, BETA-006-F — #537
15. [BETA-006-Z — Package evidence for Implement beta observability baseline](tasks/08_public_beta/BETA-006-Z-package-evidence-for-implement-beta-observability-baseline.md) — deps: BETA-006-V — #538
16. [BETA-007-A — Environment/file configuration](tasks/08_public_beta/BETA-007-A-environment-file-configuration.md) — deps: M27-GATE — #539
17. [BETA-007-B — Validation at startup](tasks/08_public_beta/BETA-007-B-validation-at-startup.md) — deps: BETA-007-A — #540
18. [BETA-007-C — Secret value wrapper/redaction](tasks/08_public_beta/BETA-007-C-secret-value-wrapper-redaction.md) — deps: BETA-007-B — #541
19. [BETA-007-D — Profile-specific settings](tasks/08_public_beta/BETA-007-D-profile-specific-settings.md) — deps: BETA-007-C — #542
20. [BETA-007-E — No dynamic code execution](tasks/08_public_beta/BETA-007-E-no-dynamic-code-execution.md) — deps: BETA-007-D — #543
21. [BETA-007-V — Verify Implement configuration and secret handling](tasks/08_public_beta/BETA-007-V-verify-implement-configuration-and-secret-handling.md) — deps: BETA-007-A, BETA-007-B, BETA-007-C, BETA-007-D, BETA-007-E — #544
22. [BETA-007-Z — Package evidence for Implement configuration and secret handling](tasks/08_public_beta/BETA-007-Z-package-evidence-for-implement-configuration-and-secret-handling.md) — deps: BETA-007-V — #545
23. [BETA-008-A — Trusted proxy configuration](tasks/08_public_beta/BETA-008-A-trusted-proxy-configuration.md) — deps: M3-GATE, BETA-006-Z — #546
24. [BETA-008-B — Forwarded header policy](tasks/08_public_beta/BETA-008-B-forwarded-header-policy.md) — deps: BETA-008-A — #547
25. [BETA-008-C — Liveness/readiness/startup endpoints](tasks/08_public_beta/BETA-008-C-liveness-readiness-startup-endpoints.md) — deps: BETA-008-B — #548
