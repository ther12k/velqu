# Contributing to Velqu

Thanks for helping improve Velqu. Contributions are welcome through the public
GitHub repository at <https://github.com/ther12k/velqu>.

## Before opening work

- Search existing issues before opening a duplicate.
- Open an issue for bugs, security-impacting behavior, or material design work.
- Keep one focused change per pull request.
- Do not add features outside the authorized milestone scope.
- Do not include credentials, private user data, or generated build output.

## Pull requests

1. Create a branch from `master`.
2. Explain behavior, scope, and any compatibility impact.
3. Link the issue with `Closes #<number>` when applicable.
4. Include tests and evidence for behavior, performance, or security claims.
5. Keep commits focused and reviewable.
6. Wait for review before merging.

Repository packet work follows the branch, worktree, PR, and squash-merge rules
in `docs/codex-spark-beta/WORKFLOW.md`.

## Local verification

Run the checks relevant to your change. For TypeScript or package changes:

```bash
bun test
bun run typecheck
```

For Rust changes, run the affected package tests and formatting checks:

```bash
cargo test -p <package>
cargo fmt --check
```

Run `./scripts/verify` before milestone checkpoint changes that touch code,
benchmarks, or release evidence.

## Licensing

Velqu is released under the MIT License in [`LICENSE`](LICENSE). By submitting
a contribution, you represent that you have the right to submit it and agree
that it may be distributed under the repository's MIT License. Contributions
must not include code copied from incompatible licenses or material without
appropriate permission and attribution.

## Security

Do not report vulnerabilities in public issues. Follow the repository's private
disclosure process once `OD-BETA-004` is accepted and documented. Until then,
limit public reports to non-sensitive reproducible information and avoid
publishing exploit details.
