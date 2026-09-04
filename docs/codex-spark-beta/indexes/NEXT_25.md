# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is BETA-011-A; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [BETA-011-A — Use SemVer prerelease](tasks/08_public_beta/BETA-011-A-use-semver-prerelease.md) — deps: M4A-GATE, BETA-010-Z — #567
2. [BETA-011-B — Publish `next`/beta tag](tasks/08_public_beta/BETA-011-B-publish-next-beta-tag.md) — deps: BETA-011-A — #568
3. [BETA-011-C — Generate changelog and migration notes](tasks/08_public_beta/BETA-011-C-generate-changelog-and-migration-notes.md) — deps: BETA-011-B — #569
4. [BETA-011-D — Create GitHub-style release packet](tasks/08_public_beta/BETA-011-D-create-github-style-release-packet.md) — deps: BETA-011-C — #570
5. [BETA-011-E — Support yanking/rollback](tasks/08_public_beta/BETA-011-E-support-yanking-rollback.md) — deps: BETA-011-D — #571
6. [BETA-011-V — Verify Automate beta publishing and versioning](tasks/08_public_beta/BETA-011-V-verify-automate-beta-publishing-and-versioning.md) — deps: BETA-011-A, BETA-011-B, BETA-011-C, BETA-011-D, BETA-011-E — #572
7. [BETA-011-Z — Package evidence for Automate beta publishing and versioning](tasks/08_public_beta/BETA-011-Z-package-evidence-for-automate-beta-publishing-and-versioning.md) — deps: BETA-011-V — #573
8. [BETA-012-A — Installation](tasks/08_public_beta/BETA-012-A-installation.md) — deps: M4A-GATE, BETA-004-Z, BETA-005-Z, BETA-008-Z — #574
9. [BETA-012-B — Quickstart](tasks/08_public_beta/BETA-012-B-quickstart.md) — deps: BETA-012-A — #575
10. [BETA-012-C — Architecture](tasks/08_public_beta/BETA-012-C-architecture.md) — deps: BETA-012-B — #576
11. [BETA-012-D — Contracts/Treaty](tasks/08_public_beta/BETA-012-D-contracts-treaty.md) — deps: BETA-012-C — #577
12. [BETA-012-E — Fetch/Postgres/auth](tasks/08_public_beta/BETA-012-E-fetch-postgres-auth.md) — deps: BETA-012-D — #578
13. [BETA-012-F — Deployment](tasks/08_public_beta/BETA-012-F-deployment.md) — deps: BETA-012-E — #579
14. [BETA-012-G — Troubleshooting](tasks/08_public_beta/BETA-012-G-troubleshooting.md) — deps: BETA-012-F — #580
15. [BETA-012-H — Performance methodology](tasks/08_public_beta/BETA-012-H-performance-methodology.md) — deps: BETA-012-G — #581
16. [BETA-012-I — Limitations/non-goals](tasks/08_public_beta/BETA-012-I-limitations-non-goals.md) — deps: BETA-012-H — #582
17. [BETA-012-V — Verify Complete beta documentation and limitations](tasks/08_public_beta/BETA-012-V-verify-complete-beta-documentation-and-limitations.md) — deps: BETA-012-A, BETA-012-B, BETA-012-C, BETA-012-D, BETA-012-E, BETA-012-F, BETA-012-G, BETA-012-H, BETA-012-I — #583
18. [BETA-012-Z — Package evidence for Complete beta documentation and limitations](tasks/08_public_beta/BETA-012-Z-package-evidence-for-complete-beta-documentation-and-limitations.md) — deps: BETA-012-V — #584
19. [BETA-013-A — Run at least two-hour mixed workload and at least one million requests on reference platform](tasks/08_public_beta/BETA-013-A-run-at-least-two-hour-mixed-workload-and-at-least-one-million-requests-on-refere.md) — deps: BETA-004-Z, BETA-005-Z, BETA-006-Z, BETA-008-Z, BETA-009-Z — #585
20. [BETA-013-B — Include fetch, DB, auth, timeouts, cancellation, worker replacement, and reload](tasks/08_public_beta/BETA-013-B-include-fetch-db-auth-timeouts-cancellation-worker-replacement-and-reload.md) — deps: BETA-013-A — #586
21. [BETA-013-C — Track RSS, heap, slots, tasks, queues, pools, and errors](tasks/08_public_beta/BETA-013-C-track-rss-heap-slots-tasks-queues-pools-and-errors.md) — deps: BETA-013-B — #587
22. [BETA-013-D — Analyze retained growth](tasks/08_public_beta/BETA-013-D-analyze-retained-growth.md) — deps: BETA-013-C — #588
23. [BETA-013-V — Verify Run beta soak and leak qualification](tasks/08_public_beta/BETA-013-V-verify-run-beta-soak-and-leak-qualification.md) — deps: BETA-013-A, BETA-013-B, BETA-013-C, BETA-013-D — #589
24. [BETA-013-Z — Package evidence for Run beta soak and leak qualification](tasks/08_public_beta/BETA-013-Z-package-evidence-for-run-beta-soak-and-leak-qualification.md) — deps: BETA-013-V — #590
25. [BETA-014-A — Include cold start categories, warm microbenchmarks, real DB/auth/I/O, CPU/JIT crossover, cost-normalized metrics, and limitations](tasks/08_public_beta/BETA-014-A-include-cold-start-categories-warm-microbenchmarks-real-db-auth-i-o-cpu-jit-cros.md) — deps: BETA-002-Z, BETA-003-Z, BETA-004-Z, BETA-005-Z, BETA-013-Z — #591
