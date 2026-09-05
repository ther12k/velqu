# Dependency-Safe Execution Queue

All PASS packets are omitted. The first unchecked dependency-ready task is BETA-016-B; future milestone tasks remain TODO until implemented and evidenced.

1. [BETA-016-B — Install CLI/runtime](tasks/08_public_beta/BETA-016-B-install-cli-runtime.md) — deps: BETA-016-A — #609
2. [BETA-016-C — Scaffold app](tasks/08_public_beta/BETA-016-C-scaffold-app.md) — deps: BETA-016-B — #610
3. [BETA-016-D — Run tests/dev/build](tasks/08_public_beta/BETA-016-D-run-tests-dev-build.md) — deps: BETA-016-C — #611
4. [BETA-016-E — Deploy proof service](tasks/08_public_beta/BETA-016-E-deploy-proof-service.md) — deps: BETA-016-D — #612
5. [BETA-016-F — Use Treaty client](tasks/08_public_beta/BETA-016-F-use-treaty-client.md) — deps: BETA-016-E — #613
6. [BETA-016-V — Verify Run external clean-install and tutorial verification](tasks/08_public_beta/BETA-016-V-verify-run-external-clean-install-and-tutorial-verification.md) — deps: BETA-016-A, BETA-016-B, BETA-016-C, BETA-016-D, BETA-016-E, BETA-016-F — #614
7. [BETA-016-Z — Package evidence for Run external clean-install and tutorial verification](tasks/08_public_beta/BETA-016-Z-package-evidence-for-run-external-clean-install-and-tutorial-verification.md) — deps: BETA-016-V — #615
8. [BETA-GATE — Public Beta Readiness and Release exit gate](gates/BETA-GATE.md) — deps: BETA-001-Z, BETA-002-Z, BETA-003-Z, BETA-004-Z, BETA-005-Z, BETA-006-Z, BETA-007-Z, BETA-008-Z, BETA-009-Z, BETA-010-Z, BETA-011-Z, BETA-012-Z, BETA-013-Z, BETA-014-Z, BETA-015-Z, BETA-016-Z, BETA-017-Z — #625
