# Publisher Key Provisioning Record — TEMPLATE

<!--
  M8-003 / #1321. This is the vessel required by
  docs/production/operational/trusted-publishers.json keyLifecycle step 3:
  "archive the provisioning record (who created the key, on what host,
  from what entropy) with the release evidence."

  Instantiate ONE copy per provisioned key under
  docs/production/evidence/ (name: publisher-key-<id>.md) when the
  owner provisions and ratifies. The instantiated record + the registry
  entry moving to status "active" together constitute the ratification.

  HARD RULES (from the signing protocol and owner review 2026-09-20):
  - Public material only. NO private keys, NO passphrases, NO
    secret-key exports (--export-secret-keys output), in this record,
    the repository, the issue tracker, or any transcript/chat.
  - Use the FULL 40-hex-char OpenPGP primary fingerprint everywhere.
    Short key IDs are ambiguous and must not appear as the identifier
    of record (the registry pins primary keys; subkey signatures are
    accepted only through the pinned primary fingerprint).
  - The bring-up/dev key (registry id "velqu-official-release-2026",
    status "provisioning-pending") stays non-production until this
    record exists for an owner-authorized identity and the registry
    says "active".
-->

## 1. Authorized owner

| Field | Value |
|---|---|
| Approving identity (name/role) | <!-- who authorizes this publisher identity --> |
| Decision reference | <!-- issue comment / OWNER_DECISIONS.md entry / ADR id --> |
| Decision date (UTC) | <!-- YYYY-MM-DD --> |

## 2. Key identity

| Field | Value |
|---|---|
| Registry id | <!-- must match trusted-publishers.json publishers[].id --> |
| Full primary fingerprint (40 hex) | <!-- e.g. 7F4FBB5D…E3FC700-length; never an abbreviated ID --> |
| Type | <!-- e.g. openpgp-ed25519 --> |
| Created / expires | <!-- dates; rotation policy reference --> |

## 3. Public-key artifact

| Field | Value |
|---|---|
| Exported public key file | <!-- e.g. keys/velqu-release-2026.pub.asc, committed alongside --> |
| Export command | <!-- e.g. gpg --export --armor <fingerprint> (public export ONLY) --> |
| SHA-256 of the export | <!-- lets verifiers pin the exact bytes --> |

## 4. Release scope

<!-- Exactly which Velqu release artifacts this key is authorized to
sign: runtime binaries, QPack tool, source archives, git bundles,
SBOM, indexes, npm tarballs — per the Signed Manifest Architecture in
RELEASE_SIGNING_AND_DISASTER_RECOVERY.md. State exclusions. -->

## 5. Custody and lifecycle

| Field | Value |
|---|---|
| Created by / on what host / entropy source | <!-- keyLifecycle requirement; state what is disclosable --> |
| Signing access arrangement | <!-- where the key lives, who can operate it; no secrets here --> |
| Backup arrangement | <!-- offline copy location class; no secret material inline --> |
| Expiry / rotation | <!-- schedule and trigger conditions --> |
| Revocation arrangement | <!-- how a revocation is published; registry revokedKeys path --> |

## 6. Verification instructions for independent verifiers

<!-- How a third party checks a release against this record: import
the public-key artifact, confirm its fingerprint matches §2 and the
trusted-publishers.json active entry, then run the release verifier
per RELEASE_SIGNING_AND_DISASTER_RECOVERY.md. -->
