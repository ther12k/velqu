# M8-003 — Release Signing and Disaster Recovery Drill (evidence)

Issue #1321 / task M8-003 ("Finalize release signing, rollback, and
disaster recovery"). This report records the first executed end-to-end
drill of the signing and rollback tooling: `scripts/release-packet --sign`,
`scripts/verify-release-packet.sh` (positive and all four designed
negative paths), and the §5 Disaster Recovery rollback-to-N-1 procedure
from `docs/production/operational/RELEASE_SIGNING_AND_DISASTER_RECOVERY.md`.

Raw transcript: `docs/production/evidence/raw/m8-003-signing-dr-drill-transcript.txt`
(sha256 `9512183e85546acc264f3f371f7f375ce89a514956ec148c31eb3c031b497504`).

## Drill environment (deliberately non-production)

- Drill executed 2026-09-16 in a throwaway git worktree of master
  `b51927ed` (branch `drill/m8-003`, deleted after evidence capture);
  the live checkout was left untouched so the running 72 h soak evidence
  was never disturbed.
- Signing key: a dedicated **DRILL key** generated in an isolated
  `GNUPGHOME` (`/tmp/velqu-drill-gnupg`), primary ed25519 fingerprint
  `90FB7F4E84D6E00026A9D5E17EE7DA2F0B04B412`, uid
  `Velqu M8-003 DRILL KEY (NOT authorized for production) <drill@velqu.dev>`.
  The key was NEVER added to `docs/production/operational/trusted-publishers.json`
  and was destroyed with the keyring after the drill. Production packet
  signing remains blocked on the owner-ratified `active` publisher key
  per the registry's `keyLifecycle` policy — this drill validates the
  TOOLING, not a production signing event.
- Two consecutive packets were built and signed with the real builder:
  - **N-1** at commit `b51927ed802d558cf594dd6820edaf764f7f14c0`
    (25 files: runtime + bytecode binaries, source zip, git bundle,
    SBOM, provenance, indexes, npm tarballs, checksum manifest + signature);
    manifest sha256 `14aa3711e0a7ad34d119e32929c66ee34f1bf5cce79f93d51bb9a1414b7f6870`,
    signature sha256 `4c8b91cc1af38975837ebea458ec3cb8f29045c010f1eff52c9e265196fc5650`.
  - **N** at drill commit `f72ec83b426379867bdb20c56219f561e017636f`
    (marker CHANGELOG entry on the throwaway branch); manifest sha256
    `376731a780c1d41389ee830c2750283366112fbbc48b81d2836f45583b241442`,
    signature sha256 `eed29d3fae038f24c292cdf87dc95190f7beb4cc08448c7086832c263c4e4ceb`.

## Drill matrix and results

| # | Scenario | Expected | Result |
|---|---|---|---|
| 1 | Build + sign N-1 and N via `scripts/release-packet --key <fpr>`; builder's built-in verification | PASS | PASS — 25 files verified, packet complete, both commits |
| 2 | Promote N: verifier `--require-signature --trusted-key <drill>` | PASS | PASS — checksums + completeness + signature OK; runtime boots, `/health/live` → `{"status":"ok"}` |
| 3 | §5.1 tampered release: one bit flipped in `velqu-runtime` copy | checksum FAIL, nonzero exit | REJECTED (exit 1) — `velqu-runtime: FAILED` |
| 4 | Untrusted signer: valid manifest re-signed by a second, non-registry key | fail closed | REJECTED (exit 1) — `signer primary key F72FE2… is NOT authorized in trusted publishers list` |
| 5 | Registry revocation: drill fingerprint placed in `revokedKeys` (copy of registry), verifier invoked WITH `--trusted-key <revoked>` | rejection overrides explicit trust | REJECTED (exit 1) — `key 90FB7F… is REVOKED … rejected even when named via --trusted-key` |
| 6 | Missing signature with `--require-signature` | fail closed | REJECTED (exit 1) — `release signature missing … but --require-signature was specified` |
| 7 | §5.2 rollback to N-1: archived N-1 packet re-verified, binary swapped, runtime booted | signature + checksums PASS, service restored | PASS — full verification OK, `/health/live` → `{"status":"ok"}`, `/hello/Rafi` → `{"message":"Hello Rafi"}`, `SOURCE-COMMIT.txt` confirms `b51927ed…` |

## Acceptance mapping (M8-003)

- "Artifacts/checksums/SBOM/provenance are signed." — tooling proven:
  builder signs the unified manifest covering binaries, SBOM,
  provenance, indexes, and npm tarballs; verification is fail-closed.
  **Remaining for PASS**: the production release set signed by the
  owner-ratified `active` publisher key at RC/GA (registry
  `keyLifecycle` policy; the current registry entry is honestly marked
  `provisioning-pending` and must never sign a real packet).
- "Rollback to previous runtime/package is rehearsed." — **done** (this
  drill, scenario 7; yank/rollback of npm packages was previously
  rehearsed in beta-011-e). Rehearsed again before every RC/GA per §5.3.
- "Compromised-key and bad-release procedures are documented." — **done**
  (`RELEASE_SIGNING_AND_DISASTER_RECOVERY.md` §4 key-compromise ladder,
  registry-authoritative revocation; bad-release/yank in the same doc
  plus beta-011-e evidence).

## Disposition

Task M8-003 moves **TODO → IN_PROGRESS** with this drill as evidence.
PASS remains gated on the owner-key production signing event, which by
policy cannot be performed by the implementation agent.
