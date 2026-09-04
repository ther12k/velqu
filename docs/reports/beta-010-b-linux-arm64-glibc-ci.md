# BETA-010-B — Linux ARM64 glibc When CI Is Available

## Result

**PASS as a conditional CI portability signal; not a public support expansion.**
The repository CI matrix includes `ubuntu-24.04-arm` with target
`aarch64-unknown-linux-gnu`, but the observed run for this packet was cancelled
while both matrix jobs were still in progress. Therefore no ARM64 release
artifact/install/runtime claim is made.

This preserves the exact policy in `docs/beta/governance/PLATFORM_SUPPORT.md`:
Linux x86_64 glibc is the only public beta promise; ARM64 remains conditional
and unpromised until a successful CI/build environment, artifact evidence, and
separate owner acceptance exist.

## CI evidence

`.github/workflows/verify.yml` defines:

```yaml
- os: ubuntu-24.04-arm
  target: aarch64-unknown-linux-gnu
```

The observed GitHub Actions run (`33842318636`, head `17424f5`) reached the
ARM64 and x86 matrix jobs but was cancelled after they remained in progress;
the OKF-only job completed successfully. Cancellation was used to avoid
leaving a stalled hosted runner active, not to turn an incomplete run into a
pass.

## Package/install boundary

- No ARM64 binary or QPack was published or added to the release inventory.
- x86_64 glibc remains the tested/public platform via
  `docs/reports/beta-010-a-linux-x86-64-glibc-platform.md`.
- ARM64 can be re-evaluated when hosted CI completes successfully or an
  equivalent ARM64 glibc build environment is available. The required evidence
  is a successful build/test/runtime transcript and artifact inventory, followed
  by owner acceptance and a policy update.
- macOS, Windows, musl/static-libc, and other architectures remain outside the
  beta promise.

## Gates

- `cargo test -p q-pack` — available in the x86_64 glibc evidence chain; pass.
- `cargo test -p q-engine-quickjs` — available in the x86_64 glibc evidence
  chain; pass.
- ARM64 hosted CI — **conditional/incomplete (cancelled while in progress)**;
  no pass claim.

## Disclosure

The correct outcome for this packet is a conditional status, not a fabricated
ARM64 support claim. CI matrix presence demonstrates planned portability
coverage only; it does not establish public support, packaged artifact
availability, or production/SLA behavior.
