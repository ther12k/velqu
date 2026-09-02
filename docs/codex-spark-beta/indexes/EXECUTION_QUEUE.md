# Dependency-Safe Execution Queue

All PASS packets are omitted. The first unchecked dependency-ready task is M4A-008-E; future milestone tasks remain TODO until implemented and evidenced.

1. [M4A-008-E — Runtime profiles](tasks/07_m4a_developer_preview/M4A-008-E-runtime-profiles.md) — deps: M4A-008-D — #478
2. [M4A-008-F — Deployment behind reverse proxy](tasks/07_m4a_developer_preview/M4A-008-F-deployment-behind-reverse-proxy.md) — deps: M4A-008-E — #479
3. [M4A-008-G — Limits and non-goals](tasks/07_m4a_developer_preview/M4A-008-G-limits-and-non-goals.md) — deps: M4A-008-F — #480
4. [M4A-008-V — Verify Build documentation and examples](tasks/07_m4a_developer_preview/M4A-008-V-verify-build-documentation-and-examples.md) — deps: M4A-008-A, M4A-008-B, M4A-008-C, M4A-008-D, M4A-008-E, M4A-008-F, M4A-008-G — #481
5. [M4A-008-Z — Package evidence for Build documentation and examples](tasks/07_m4a_developer_preview/M4A-008-Z-package-evidence-for-build-documentation-and-examples.md) — deps: M4A-008-V — #482
6. [M4A-009-A — Feature modules](tasks/07_m4a_developer_preview/M4A-009-A-feature-modules.md) — deps: M4A-004-Z, M4A-007-Z, M28-GATE — #483
7. [M4A-009-B — JWT-like policy reference](tasks/07_m4a_developer_preview/M4A-009-B-jwt-like-policy-reference.md) — deps: M4A-009-A — #484
8. [M4A-009-C — Controlled upstream](tasks/07_m4a_developer_preview/M4A-009-C-controlled-upstream.md) — deps: M4A-009-B — #485
9. [M4A-009-D — Metrics/readiness/shutdown](tasks/07_m4a_developer_preview/M4A-009-D-metrics-readiness-shutdown.md) — deps: M4A-009-C — #486
10. [M4A-009-E — Treaty client](tasks/07_m4a_developer_preview/M4A-009-E-treaty-client.md) — deps: M4A-009-D — #487
11. [M4A-009-V — Verify Build realistic private-alpha proof service](tasks/07_m4a_developer_preview/M4A-009-V-verify-build-realistic-private-alpha-proof-service.md) — deps: M4A-009-A, M4A-009-B, M4A-009-C, M4A-009-D, M4A-009-E — #488
12. [M4A-009-Z — Package evidence for Build realistic private-alpha proof service](tasks/07_m4a_developer_preview/M4A-009-Z-package-evidence-for-build-realistic-private-alpha-proof-service.md) — deps: M4A-009-V — #489
13. [M4A-010-A — Provide clean install packet](tasks/07_m4a_developer_preview/M4A-010-A-provide-clean-install-packet.md) — deps: M4A-003-Z, M4A-008-Z, M4A-009-Z — #490
14. [M4A-010-B — Collect task-based feedback](tasks/07_m4a_developer_preview/M4A-010-B-collect-task-based-feedback.md) — deps: M4A-010-A — #491
15. [M4A-010-C — Classify P0/P1/P2](tasks/07_m4a_developer_preview/M4A-010-C-classify-p0-p1-p2.md) — deps: M4A-010-B — #492
16. [M4A-010-D — Fix beta-blocking findings and publish limitations](tasks/07_m4a_developer_preview/M4A-010-D-fix-beta-blocking-findings-and-publish-limitations.md) — deps: M4A-010-C — #493
17. [M4A-010-V — Verify Run invited developer alpha and close P0/P1 feedback](tasks/07_m4a_developer_preview/M4A-010-V-verify-run-invited-developer-alpha-and-close-p0-p1-feedback.md) — deps: M4A-010-A, M4A-010-B, M4A-010-C, M4A-010-D — #494
18. [M4A-010-Z — Package evidence for Run invited developer alpha and close P0/P1 feedback](tasks/07_m4a_developer_preview/M4A-010-Z-package-evidence-for-run-invited-developer-alpha-and-close-p0-p1-feedback.md) — deps: M4A-010-V — #495
19. [M4A-GATE — M4A — Developer Preview and Private Alpha exit gate](gates/M4A-GATE.md) — deps: M4A-001-Z, M4A-002-Z, M4A-003-Z, M4A-004-Z, M4A-005-Z, M4A-006-Z, M4A-007-Z, M4A-008-Z, M4A-009-Z, M4A-010-Z — #633
20. [BETA-001-V — Verify Make the real-world benchmark harness executable](tasks/08_public_beta/BETA-001-V-verify-make-the-real-world-benchmark-harness-executable.md) — deps: BETA-001-A, BETA-001-B, BETA-001-C, BETA-001-D — #500
21. [BETA-001-Z — Package evidence for Make the real-world benchmark harness executable](tasks/08_public_beta/BETA-001-Z-package-evidence-for-make-the-real-world-benchmark-harness-executable.md) — deps: BETA-001-V — #503
22. [BETA-002-A — Match SQL, pool, JWT, timeouts, logging, responses, compression, and deployment limits](tasks/08_public_beta/BETA-002-A-match-sql-pool-jwt-timeouts-logging-responses-compression-and-deployment-limits.md) — deps: BETA-001-Z — #504
23. [BETA-002-B — Pin versions](tasks/08_public_beta/BETA-002-B-pin-versions.md) — deps: BETA-002-A — #505
24. [BETA-002-C — Add contract-response verification](tasks/08_public_beta/BETA-002-C-add-contract-response-verification.md) — deps: BETA-002-B — #506
25. [BETA-002-D — Document unavoidable differences](tasks/08_public_beta/BETA-002-D-document-unavoidable-differences.md) — deps: BETA-002-C — #507
26. [BETA-002-V — Verify Implement matched competitor candidates](tasks/08_public_beta/BETA-002-V-verify-implement-matched-competitor-candidates.md) — deps: BETA-002-A, BETA-002-B, BETA-002-C, BETA-002-D — #508
27. [BETA-002-Z — Package evidence for Implement matched competitor candidates](tasks/08_public_beta/BETA-002-Z-package-evidence-for-implement-matched-competitor-candidates.md) — deps: BETA-002-V — #509
28. [BETA-003-A — Run 0/1/5/10/25ms I/O, payload matrices, and CPU operation levels](tasks/08_public_beta/BETA-003-A-run-0-1-5-10-25ms-i-o-payload-matrices-and-cpu-operation-levels.md) — deps: BETA-001-Z, M28-GATE, M3-GATE — #510
29. [BETA-003-B — Measure first request through steady state](tasks/08_public_beta/BETA-003-B-measure-first-request-through-steady-state.md) — deps: BETA-003-A — #511
30. [BETA-003-C — Calculate cumulative crossover request counts](tasks/08_public_beta/BETA-003-C-calculate-cumulative-crossover-request-counts.md) — deps: BETA-003-B — #512
31. [BETA-003-D — Report losses honestly](tasks/08_public_beta/BETA-003-D-report-losses-honestly.md) — deps: BETA-003-C — #513
32. [BETA-003-V — Verify Run controlled I/O and CPU/JIT crossover suites](tasks/08_public_beta/BETA-003-V-verify-run-controlled-i-o-and-cpu-jit-crossover-suites.md) — deps: BETA-003-A, BETA-003-B, BETA-003-C, BETA-003-D — #514
33. [BETA-003-Z — Package evidence for Run controlled I/O and CPU/JIT crossover suites](tasks/08_public_beta/BETA-003-Z-package-evidence-for-run-controlled-i-o-and-cpu-jit-crossover-suites.md) — deps: BETA-003-V — #515
34. [BETA-004-A — Use capability ABI](tasks/08_public_beta/BETA-004-A-use-capability-abi.md) — deps: M27-GATE, BETA-001-Z — #516
35. [BETA-004-B — Lazy pool](tasks/08_public_beta/BETA-004-B-lazy-pool.md) — deps: BETA-004-A — #517
36. [BETA-004-C — Parameterized queries/transactions](tasks/08_public_beta/BETA-004-C-parameterized-queries-transactions.md) — deps: BETA-004-B — #518
37. [BETA-004-D — Deadline/cancellation/shutdown](tasks/08_public_beta/BETA-004-D-deadline-cancellation-shutdown.md) — deps: BETA-004-C — #519
38. [BETA-004-E — Pool limits and observability](tasks/08_public_beta/BETA-004-E-pool-limits-and-observability.md) — deps: BETA-004-D — #520
39. [BETA-004-F — No ORM](tasks/08_public_beta/BETA-004-F-no-orm.md) — deps: BETA-004-E — #521
40. [BETA-004-V — Verify Implement optional first-party Postgres capability](tasks/08_public_beta/BETA-004-V-verify-implement-optional-first-party-postgres-capability.md) — deps: BETA-004-A, BETA-004-B, BETA-004-C, BETA-004-D, BETA-004-E, BETA-004-F — #522
41. [BETA-004-Z — Package evidence for Implement optional first-party Postgres capability](tasks/08_public_beta/BETA-004-Z-package-evidence-for-implement-optional-first-party-postgres-capability.md) — deps: BETA-004-V — #523
42. [BETA-005-A — Support one approved JWT algorithm/profile](tasks/08_public_beta/BETA-005-A-support-one-approved-jwt-algorithm-profile.md) — deps: M27-GATE, M25-GATE — #524
43. [BETA-005-B — Key loading/rotation hooks](tasks/08_public_beta/BETA-005-B-key-loading-rotation-hooks.md) — deps: BETA-005-A — #525
44. [BETA-005-C — Expiry/audience/issuer checks](tasks/08_public_beta/BETA-005-C-expiry-audience-issuer-checks.md) — deps: BETA-005-B — #526
45. [BETA-005-D — Typed 401/403 problems](tasks/08_public_beta/BETA-005-D-typed-401-403-problems.md) — deps: BETA-005-C — #527
46. [BETA-005-E — No secret logging](tasks/08_public_beta/BETA-005-E-no-secret-logging.md) — deps: BETA-005-D — #528
47. [BETA-005-V — Verify Implement JWT/auth reference package](tasks/08_public_beta/BETA-005-V-verify-implement-jwt-auth-reference-package.md) — deps: BETA-005-A, BETA-005-B, BETA-005-C, BETA-005-D, BETA-005-E — #529
48. [BETA-005-Z — Package evidence for Implement JWT/auth reference package](tasks/08_public_beta/BETA-005-Z-package-evidence-for-implement-jwt-auth-reference-package.md) — deps: BETA-005-V — #530
49. [BETA-006-A — Request/route/status/duration](tasks/08_public_beta/BETA-006-A-request-route-status-duration.md) — deps: M3-GATE, M28-GATE — #531
50. [BETA-006-B — Worker queues/quarantine/replacements](tasks/08_public_beta/BETA-006-B-worker-queues-quarantine-replacements.md) — deps: BETA-006-A — #532
51. [BETA-006-C — Fetch and DB pools](tasks/08_public_beta/BETA-006-C-fetch-and-db-pools.md) — deps: BETA-006-B — #533
52. [BETA-006-D — Memory/tasks/slots](tasks/08_public_beta/BETA-006-D-memory-tasks-slots.md) — deps: BETA-006-C — #534
53. [BETA-006-E — Optional trace integration or trace IDs](tasks/08_public_beta/BETA-006-E-optional-trace-integration-or-trace-ids.md) — deps: BETA-006-D — #535
54. [BETA-006-F — Redaction](tasks/08_public_beta/BETA-006-F-redaction.md) — deps: BETA-006-E — #536
55. [BETA-006-V — Verify Implement beta observability baseline](tasks/08_public_beta/BETA-006-V-verify-implement-beta-observability-baseline.md) — deps: BETA-006-A, BETA-006-B, BETA-006-C, BETA-006-D, BETA-006-E, BETA-006-F — #537
56. [BETA-006-Z — Package evidence for Implement beta observability baseline](tasks/08_public_beta/BETA-006-Z-package-evidence-for-implement-beta-observability-baseline.md) — deps: BETA-006-V — #538
57. [BETA-007-A — Environment/file configuration](tasks/08_public_beta/BETA-007-A-environment-file-configuration.md) — deps: M27-GATE — #539
58. [BETA-007-B — Validation at startup](tasks/08_public_beta/BETA-007-B-validation-at-startup.md) — deps: BETA-007-A — #540
59. [BETA-007-C — Secret value wrapper/redaction](tasks/08_public_beta/BETA-007-C-secret-value-wrapper-redaction.md) — deps: BETA-007-B — #541
60. [BETA-007-D — Profile-specific settings](tasks/08_public_beta/BETA-007-D-profile-specific-settings.md) — deps: BETA-007-C — #542
61. [BETA-007-E — No dynamic code execution](tasks/08_public_beta/BETA-007-E-no-dynamic-code-execution.md) — deps: BETA-007-D — #543
62. [BETA-007-V — Verify Implement configuration and secret handling](tasks/08_public_beta/BETA-007-V-verify-implement-configuration-and-secret-handling.md) — deps: BETA-007-A, BETA-007-B, BETA-007-C, BETA-007-D, BETA-007-E — #544
63. [BETA-007-Z — Package evidence for Implement configuration and secret handling](tasks/08_public_beta/BETA-007-Z-package-evidence-for-implement-configuration-and-secret-handling.md) — deps: BETA-007-V — #545
64. [BETA-008-A — Trusted proxy configuration](tasks/08_public_beta/BETA-008-A-trusted-proxy-configuration.md) — deps: M3-GATE, BETA-006-Z — #546
65. [BETA-008-B — Forwarded header policy](tasks/08_public_beta/BETA-008-B-forwarded-header-policy.md) — deps: BETA-008-A — #547
66. [BETA-008-C — Liveness/readiness/startup endpoints](tasks/08_public_beta/BETA-008-C-liveness-readiness-startup-endpoints.md) — deps: BETA-008-B — #548
67. [BETA-008-D — Graceful drain and termination](tasks/08_public_beta/BETA-008-D-graceful-drain-and-termination.md) — deps: BETA-008-C — #549
68. [BETA-008-E — Container example](tasks/08_public_beta/BETA-008-E-container-example.md) — deps: BETA-008-D — #550
69. [BETA-008-V — Verify Implement reverse-proxy, drain, and deployment semantics](tasks/08_public_beta/BETA-008-V-verify-implement-reverse-proxy-drain-and-deployment-semantics.md) — deps: BETA-008-A, BETA-008-B, BETA-008-C, BETA-008-D, BETA-008-E — #551
70. [BETA-008-Z — Package evidence for Implement reverse-proxy, drain, and deployment semantics](tasks/08_public_beta/BETA-008-Z-package-evidence-for-implement-reverse-proxy-drain-and-deployment-semantics.md) — deps: BETA-008-V — #552
71. [BETA-009-A — Run fuzz suites for pack/router/schema/bridge/HTTP](tasks/08_public_beta/BETA-009-A-run-fuzz-suites-for-pack-router-schema-bridge-http.md) — deps: M28-GATE, M3-GATE, BETA-004-Z, BETA-005-Z, BETA-007-Z — #553
72. [BETA-009-B — Dependency vulnerability and license scan](tasks/08_public_beta/BETA-009-B-dependency-vulnerability-and-license-scan.md) — deps: BETA-009-A — #554
73. [BETA-009-C — Threat-model review](tasks/08_public_beta/BETA-009-C-threat-model-review.md) — deps: BETA-009-B — #555
74. [BETA-009-D — Chaos tests for upstream/DB/worker poison](tasks/08_public_beta/BETA-009-D-chaos-tests-for-upstream-db-worker-poison.md) — deps: BETA-009-C — #556
75. [BETA-009-E — No known critical/high exploitable issue](tasks/08_public_beta/BETA-009-E-no-known-critical-high-exploitable-issue.md) — deps: BETA-009-D — #557
76. [BETA-009-V — Verify Run beta security and reliability baseline](tasks/08_public_beta/BETA-009-V-verify-run-beta-security-and-reliability-baseline.md) — deps: BETA-009-A, BETA-009-B, BETA-009-C, BETA-009-D, BETA-009-E — #558
77. [BETA-009-Z — Package evidence for Run beta security and reliability baseline](tasks/08_public_beta/BETA-009-Z-package-evidence-for-run-beta-security-and-reliability-baseline.md) — deps: BETA-009-V — #559
78. [BETA-010-A — Linux x86_64 glibc mandatory working assumption](tasks/08_public_beta/BETA-010-A-linux-x86-64-glibc-mandatory-working-assumption.md) — deps: M26-GATE, M4A-002-Z — #560
79. [BETA-010-B — Linux ARM64 glibc when CI is available](tasks/08_public_beta/BETA-010-B-linux-arm64-glibc-when-ci-is-available.md) — deps: BETA-010-A — #561
80. [BETA-010-C — npm packages under beta tag](tasks/08_public_beta/BETA-010-C-npm-packages-under-beta-tag.md) — deps: BETA-010-B — #562
81. [BETA-010-D — Runtime binary/QPack tools](tasks/08_public_beta/BETA-010-D-runtime-binary-qpack-tools.md) — deps: BETA-010-C — #563
82. [BETA-010-E — Clean install tests](tasks/08_public_beta/BETA-010-E-clean-install-tests.md) — deps: BETA-010-D — #564
83. [BETA-010-V — Verify Create supported beta platform and packaging matrix](tasks/08_public_beta/BETA-010-V-verify-create-supported-beta-platform-and-packaging-matrix.md) — deps: BETA-010-A, BETA-010-B, BETA-010-C, BETA-010-D, BETA-010-E — #565
84. [BETA-010-Z — Package evidence for Create supported beta platform and packaging matrix](tasks/08_public_beta/BETA-010-Z-package-evidence-for-create-supported-beta-platform-and-packaging-matrix.md) — deps: BETA-010-V — #566
85. [BETA-011-A — Use SemVer prerelease](tasks/08_public_beta/BETA-011-A-use-semver-prerelease.md) — deps: M4A-GATE, BETA-010-Z — #567
86. [BETA-011-B — Publish `next`/beta tag](tasks/08_public_beta/BETA-011-B-publish-next-beta-tag.md) — deps: BETA-011-A — #568
87. [BETA-011-C — Generate changelog and migration notes](tasks/08_public_beta/BETA-011-C-generate-changelog-and-migration-notes.md) — deps: BETA-011-B — #569
88. [BETA-011-D — Create GitHub-style release packet](tasks/08_public_beta/BETA-011-D-create-github-style-release-packet.md) — deps: BETA-011-C — #570
89. [BETA-011-E — Support yanking/rollback](tasks/08_public_beta/BETA-011-E-support-yanking-rollback.md) — deps: BETA-011-D — #571
90. [BETA-011-V — Verify Automate beta publishing and versioning](tasks/08_public_beta/BETA-011-V-verify-automate-beta-publishing-and-versioning.md) — deps: BETA-011-A, BETA-011-B, BETA-011-C, BETA-011-D, BETA-011-E — #572
91. [BETA-011-Z — Package evidence for Automate beta publishing and versioning](tasks/08_public_beta/BETA-011-Z-package-evidence-for-automate-beta-publishing-and-versioning.md) — deps: BETA-011-V — #573
92. [BETA-012-A — Installation](tasks/08_public_beta/BETA-012-A-installation.md) — deps: M4A-GATE, BETA-004-Z, BETA-005-Z, BETA-008-Z — #574
93. [BETA-012-B — Quickstart](tasks/08_public_beta/BETA-012-B-quickstart.md) — deps: BETA-012-A — #575
94. [BETA-012-C — Architecture](tasks/08_public_beta/BETA-012-C-architecture.md) — deps: BETA-012-B — #576
95. [BETA-012-D — Contracts/Treaty](tasks/08_public_beta/BETA-012-D-contracts-treaty.md) — deps: BETA-012-C — #577
96. [BETA-012-E — Fetch/Postgres/auth](tasks/08_public_beta/BETA-012-E-fetch-postgres-auth.md) — deps: BETA-012-D — #578
97. [BETA-012-F — Deployment](tasks/08_public_beta/BETA-012-F-deployment.md) — deps: BETA-012-E — #579
98. [BETA-012-G — Troubleshooting](tasks/08_public_beta/BETA-012-G-troubleshooting.md) — deps: BETA-012-F — #580
99. [BETA-012-H — Performance methodology](tasks/08_public_beta/BETA-012-H-performance-methodology.md) — deps: BETA-012-G — #581
100. [BETA-012-I — Limitations/non-goals](tasks/08_public_beta/BETA-012-I-limitations-non-goals.md) — deps: BETA-012-H — #582
101. [BETA-012-V — Verify Complete beta documentation and limitations](tasks/08_public_beta/BETA-012-V-verify-complete-beta-documentation-and-limitations.md) — deps: BETA-012-A, BETA-012-B, BETA-012-C, BETA-012-D, BETA-012-E, BETA-012-F, BETA-012-G, BETA-012-H, BETA-012-I — #583
102. [BETA-012-Z — Package evidence for Complete beta documentation and limitations](tasks/08_public_beta/BETA-012-Z-package-evidence-for-complete-beta-documentation-and-limitations.md) — deps: BETA-012-V — #584
103. [BETA-013-A — Run at least two-hour mixed workload and at least one million requests on reference platform](tasks/08_public_beta/BETA-013-A-run-at-least-two-hour-mixed-workload-and-at-least-one-million-requests-on-refere.md) — deps: BETA-004-Z, BETA-005-Z, BETA-006-Z, BETA-008-Z, BETA-009-Z — #585
104. [BETA-013-B — Include fetch, DB, auth, timeouts, cancellation, worker replacement, and reload](tasks/08_public_beta/BETA-013-B-include-fetch-db-auth-timeouts-cancellation-worker-replacement-and-reload.md) — deps: BETA-013-A — #586
105. [BETA-013-C — Track RSS, heap, slots, tasks, queues, pools, and errors](tasks/08_public_beta/BETA-013-C-track-rss-heap-slots-tasks-queues-pools-and-errors.md) — deps: BETA-013-B — #587
106. [BETA-013-D — Analyze retained growth](tasks/08_public_beta/BETA-013-D-analyze-retained-growth.md) — deps: BETA-013-C — #588
107. [BETA-013-V — Verify Run beta soak and leak qualification](tasks/08_public_beta/BETA-013-V-verify-run-beta-soak-and-leak-qualification.md) — deps: BETA-013-A, BETA-013-B, BETA-013-C, BETA-013-D — #589
108. [BETA-013-Z — Package evidence for Run beta soak and leak qualification](tasks/08_public_beta/BETA-013-Z-package-evidence-for-run-beta-soak-and-leak-qualification.md) — deps: BETA-013-V — #590
109. [BETA-014-A — Include cold start categories, warm microbenchmarks, real DB/auth/I/O, CPU/JIT crossover, cost-normalized metrics, and limitations](tasks/08_public_beta/BETA-014-A-include-cold-start-categories-warm-microbenchmarks-real-db-auth-i-o-cpu-jit-cros.md) — deps: BETA-002-Z, BETA-003-Z, BETA-004-Z, BETA-005-Z, BETA-013-Z — #591
110. [BETA-014-B — Pin all candidates/artifacts](tasks/08_public_beta/BETA-014-B-pin-all-candidates-artifacts.md) — deps: BETA-014-A — #592
111. [BETA-014-C — Retain raw data](tasks/08_public_beta/BETA-014-C-retain-raw-data.md) — deps: BETA-014-B — #593
112. [BETA-014-D — Have wording reviewed](tasks/08_public_beta/BETA-014-D-have-wording-reviewed.md) — deps: BETA-014-C — #594
113. [BETA-014-V — Verify Publish canonical beta benchmark report](tasks/08_public_beta/BETA-014-V-verify-publish-canonical-beta-benchmark-report.md) — deps: BETA-014-A, BETA-014-B, BETA-014-C, BETA-014-D — #595
114. [BETA-014-Z — Package evidence for Publish canonical beta benchmark report](tasks/08_public_beta/BETA-014-Z-package-evidence-for-publish-canonical-beta-benchmark-report.md) — deps: BETA-014-V — #596
115. [BETA-015-A — Source ZIP](tasks/08_public_beta/BETA-015-A-source-zip.md) — deps: BETA-009-Z, BETA-010-Z, BETA-011-Z, BETA-013-Z, BETA-014-Z — #597
116. [BETA-015-B — Git bundle](tasks/08_public_beta/BETA-015-B-git-bundle.md) — deps: BETA-015-A — #598
117. [BETA-015-C — Linux binaries](tasks/08_public_beta/BETA-015-C-linux-binaries.md) — deps: BETA-015-B — #599
118. [BETA-015-D — npm package tarballs](tasks/08_public_beta/BETA-015-D-npm-package-tarballs.md) — deps: BETA-015-C — #600
119. [BETA-015-E — QPack tools](tasks/08_public_beta/BETA-015-E-qpack-tools.md) — deps: BETA-015-D — #601
120. [BETA-015-F — SBOM](tasks/08_public_beta/BETA-015-F-sbom.md) — deps: BETA-015-E — #602
121. [BETA-015-G — Checksums](tasks/08_public_beta/BETA-015-G-checksums.md) — deps: BETA-015-F — #603
122. [BETA-015-H — Review/evidence indexes](tasks/08_public_beta/BETA-015-H-review-evidence-indexes.md) — deps: BETA-015-G — #604
123. [BETA-015-I — Known limitations](tasks/08_public_beta/BETA-015-I-known-limitations.md) — deps: BETA-015-H — #605
124. [BETA-015-V — Verify Generate beta release evidence, SBOM, and checksums](tasks/08_public_beta/BETA-015-V-verify-generate-beta-release-evidence-sbom-and-checksums.md) — deps: BETA-015-A, BETA-015-B, BETA-015-C, BETA-015-D, BETA-015-E, BETA-015-F, BETA-015-G, BETA-015-H, BETA-015-I — #606
125. [BETA-015-Z — Package evidence for Generate beta release evidence, SBOM, and checksums](tasks/08_public_beta/BETA-015-Z-package-evidence-for-generate-beta-release-evidence-sbom-and-checksums.md) — deps: BETA-015-V — #607
126. [BETA-016-A — Fresh Linux VM/container](tasks/08_public_beta/BETA-016-A-fresh-linux-vm-container.md) — deps: BETA-011-Z, BETA-012-Z, BETA-015-Z — #608
127. [BETA-016-B — Install CLI/runtime](tasks/08_public_beta/BETA-016-B-install-cli-runtime.md) — deps: BETA-016-A — #609
128. [BETA-016-C — Scaffold app](tasks/08_public_beta/BETA-016-C-scaffold-app.md) — deps: BETA-016-B — #610
129. [BETA-016-D — Run tests/dev/build](tasks/08_public_beta/BETA-016-D-run-tests-dev-build.md) — deps: BETA-016-C — #611
130. [BETA-016-E — Deploy proof service](tasks/08_public_beta/BETA-016-E-deploy-proof-service.md) — deps: BETA-016-D — #612
131. [BETA-016-F — Use Treaty client](tasks/08_public_beta/BETA-016-F-use-treaty-client.md) — deps: BETA-016-E — #613
132. [BETA-016-V — Verify Run external clean-install and tutorial verification](tasks/08_public_beta/BETA-016-V-verify-run-external-clean-install-and-tutorial-verification.md) — deps: BETA-016-A, BETA-016-B, BETA-016-C, BETA-016-D, BETA-016-E, BETA-016-F — #614
133. [BETA-016-Z — Package evidence for Run external clean-install and tutorial verification](tasks/08_public_beta/BETA-016-Z-package-evidence-for-run-external-clean-install-and-tutorial-verification.md) — deps: BETA-016-V — #615
134. [BETA-GATE — Public Beta Readiness and Release exit gate](gates/BETA-GATE.md) — deps: BETA-001-Z, BETA-002-Z, BETA-003-Z, BETA-004-Z, BETA-005-Z, BETA-006-Z, BETA-007-Z, BETA-008-Z, BETA-009-Z, BETA-010-Z, BETA-011-Z, BETA-012-Z, BETA-013-Z, BETA-014-Z, BETA-015-Z, BETA-016-Z, BETA-017-Z — #625
