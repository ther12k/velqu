# Release Artifact Signing and Disaster Recovery Protocol (M8-003)

This document establishes the artifact signing methodology, verification protocol, trusted publisher key management, key revocation drill, and disaster recovery rollback procedures for Velqu releases.

---

## 1. Signed Manifest Release Architecture

To maintain cryptographic provenance and supply-chain integrity without inflating binary overhead, Velqu employs the **Signed Manifest Architecture**:

```
[Release Directory]
  ├── velqu-runtime (Linux x86_64 binary)
  ├── velqu-bytecode (QPack tool binary)
  ├── source-*.zip (Source archive)
  ├── velqu-*.bundle (Git history bundle)
  ├── sbom.cdx.json (CycloneDX 1.5 SBOM)
  ├── REVIEW_INDEX.json & EVIDENCE_INDEX.json
  └── npm-tarballs/*.tgz (Packages)
            │
            ▼ SHA-256 Checksum calculation
       SHA256SUMS.txt (Unified Checksum Manifest)
            │
            ▼ OpenPGP Detached Signature (GPG / ed25519)
       SHA256SUMS.txt.asc (ASCII-Armored Detached Signature)
```

### Architectural Contract:
- **Manifest Scope**: `SHA256SUMS.txt` covers every shipped release file (binaries, source archive, git bundle, SBOM, indexes, and package tarballs) sorted deterministically (`LC_ALL=C sort`).
- **Signature Scope**: The detached signature `SHA256SUMS.txt.asc` authenticates the exact bytes of `SHA256SUMS.txt`. The manifest checksums in turn authenticate each individual artifact.
- **Precision Notice**: The release packet uses a **signed checksum manifest**. Individual files do not carry separate detached signatures; verifying `SHA256SUMS.txt.asc` against `SHA256SUMS.txt` transitively proves the authenticity and integrity of all contained artifacts.

---

## 2. Release Generation and Signing Procedure

Release generation is executed via `scripts/release-packet`:

```bash
# Generate release packet with automated OpenPGP signing:
./scripts/release-packet --sign --key <AUTHORIZED_KEY_FINGERPRINT>

# Or via environment variables:
VELQU_SIGN_RELEASE=1 VELQU_GPG_KEY=<AUTHORIZED_KEY_FINGERPRINT> ./scripts/release-packet
```

### Invariants:
1. Working tree must be strictly clean (`git status --porcelain` must be empty).
2. The release runtime binary (`target/release/velqu-runtime`) must exist and be built from the exact HEAD commit.
3. Every artifact is checksummed into `SHA256SUMS.txt`.
4. `gpg --batch --yes --armor --detach-sign` creates `SHA256SUMS.txt.asc`.
5. The packet immediately executes self-verification via `scripts/verify-release-packet.sh --require-signature`. If signing or verification fails, the release script aborts fail-closed with a non-zero exit code. **There is zero silent fallback to unsigned releases.**

---

## 3. Consumer & Operator Verification Protocol

Consumers and operators verify the authenticity of a Velqu release packet using `scripts/verify-release-packet.sh`:

```bash
# Standard verification against the official trusted publishers registry:
scripts/verify-release-packet.sh --packet-dir release/ --require-signature

# Explicit trusted key verification:
scripts/verify-release-packet.sh --packet-dir release/ --require-signature --trusted-key <FINGERPRINT>
```

### Verification Checks Performed:
1. **Signature Validity**: Validates `SHA256SUMS.txt.asc` over `SHA256SUMS.txt` using machine-readable GPG status output (`[GNUPG:] VALIDSIG` and `[GNUPG:] GOODSIG`). BADSIG or ERRSIG fails immediately.
2. **Authorized Publisher Trust**: Asserts that the signer key fingerprint exists in `docs/production/operational/trusted-publishers.json` with status `active`. Unrecognized or revoked keys fail closed.
3. **Artifact Integrity**: Executes `sha256sum -c SHA256SUMS.txt` verifying every artifact's byte hash matches the signed manifest.
4. **Packet Completeness**: Checks that no unlisted or extraneous files exist in the release directory.

---

## 4. Compromised Key Revocation Protocol

If an authorized publisher signing key is compromised or suspected of exposure:

```
[Key Compromise Detected]
          │
          ▼
1. Emergency Owner Notice & Revocation Decision
          │
          ▼
2. Issue GPG Revocation Certificate (`gpg --import <key>.rev`)
          │
          ▼
3. Update `docs/production/operational/trusted-publishers.json`:
   - Move compromised fingerprint to `revokedKeys` with timestamp and reason.
   - Set status to `revoked`.
          │
          ▼
4. Publish Revocation to Public Keyservers (`keyserver.ubuntu.com`, `keys.openpgp.org`)
          │
          ▼
5. Authorize New Replacement Key (Owner action)
          │
          ▼
6. Re-sign Affected Release Manifests with Replacement Key
```

### Invariant:
Any verifier running `scripts/verify-release-packet.sh` automatically rejects releases signed by a revoked key listed in `trusted-publishers.json`.

---

## 5. Disaster Recovery (DR) and Rollback Drill

### Rollback Scenarios and Procedures:
1. **Corrupted or Tampered Release**:
   - `scripts/verify-release-packet.sh` fails in CI or staging canary.
   - The deployment pipeline automatically halts promotion (zero traffic shifted).
   - The release packet is purged from distribution mirrors.
2. **Post-Promotion Critical Flaw (Rollback to N-1)**:
   - Operators execute rollback procedure defined in `CANARY_PROGRAM.md`.
   - Previous signed release packet (N-1) remains archived with immutable checksums and signatures.
   - Upstream proxies switch traffic to N-1 instances.
   - QPack bytecode remains compatible or is rebuilt using N-1 `q-bytecode-tool`.
3. **Drill Frequency**:
   - Signature verification, key revocation, and binary rollback must be rehearsed and verified before every major Release Candidate (RC) and General Availability (GA) milestone.
