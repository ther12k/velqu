#!/usr/bin/env bash
# #1321 / M8-003 — Automated release packet verification and signature check.
#
# Verifies a Velqu distribution release packet:
#   1. Validates presence and structure of SHA256SUMS.txt manifest.
#   2. If a signature is present or --require-signature is specified:
#      - Verifies the GPG detached signature (SHA256SUMS.txt.asc) over SHA256SUMS.txt
#        via machine-readable status ([GNUPG:] VALIDSIG / GOODSIG).
#      - Asserts that the signing key fingerprint matches an authorized publisher
#        key (from docs/production/operational/trusted-publishers.json or --trusted-key).
#      - Fails closed on untrusted keys, BADSIG, ERRSIG, or missing signature.
#   3. Verifies SHA-256 checksums of every shipped artifact listed in the manifest.
#   4. Asserts manifest completeness: no extraneous unlisted files exist in the packet
#      directory (except SHA256SUMS.txt and its detached signature).
#
# Usage: scripts/verify-release-packet.sh [OPTIONS]
# Options:
#   --packet-dir <path>         Path to release packet directory (default: release)
#   --require-signature         Enforce that SHA256SUMS.txt.asc exists and is verified
#   --trusted-key <fpr>         Explicitly trust this key fingerprint
#   --trusted-keys-file <path>  JSON file containing trusted publisher keys
#                               (default: docs/production/operational/trusted-publishers.json)
#   --dry-run                   Display verification plan and discovered files without failing
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PACKET_DIR="$ROOT/release"
REQUIRE_SIGNATURE=0
DRY_RUN=0
EXPLICIT_TRUSTED_KEYS=()
TRUSTED_KEYS_FILE="$ROOT/docs/production/operational/trusted-publishers.json"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --packet-dir)
      PACKET_DIR="$(cd "$2" && pwd)"
      shift 2
      ;;
    --require-signature)
      REQUIRE_SIGNATURE=1
      shift
      ;;
    --trusted-key)
      EXPLICIT_TRUSTED_KEYS+=("$(echo "$2" | tr '[:lower:]' '[:upper:]' | tr -d ' ')")
      shift 2
      ;;
    --trusted-keys-file)
      TRUSTED_KEYS_FILE="$(cd "$(dirname "$2")" && pwd)/$(basename "$2")"
      shift 2
      ;;
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    -h|--help)
      echo "Usage: scripts/verify-release-packet.sh [--packet-dir <dir>] [--require-signature] [--trusted-key <fpr>] [--trusted-keys-file <file>] [--dry-run]"
      exit 0
      ;;
    *)
      echo "ERROR: unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

echo "=== Velqu Release Packet Verifier (#1321 / M8-003) ==="
echo "Packet Directory: $PACKET_DIR"
echo "Require Signature: $([[ $REQUIRE_SIGNATURE -eq 1 ]] && echo 'YES (fail closed if unsigned)' || echo 'NO (optional)')"

# Temp files (gpg status capture, parsed manifest paths) are cleaned up on exit.
TMP_FILES=()
trap '[[ ${#TMP_FILES[@]} -gt 0 ]] && rm -f "${TMP_FILES[@]}" || true' EXIT

# Load trusted publisher key fingerprints
TRUSTED_FINGERPRINTS=()
for k in "${EXPLICIT_TRUSTED_KEYS[@]}"; do
  TRUSTED_FINGERPRINTS+=("$k")
done

if [[ -f "$TRUSTED_KEYS_FILE" ]]; then
  FILE_KEYS=$(python3 -c "
import json, sys
try:
    doc = json.load(open(sys.argv[1]))
    for p in doc.get('publishers', []):
        if p.get('status') == 'active' and 'fingerprint' in p:
            print(p['fingerprint'].strip().upper().replace(' ', ''))
except Exception as e:
    pass
" "$TRUSTED_KEYS_FILE")
  while IFS= read -r line; do
    if [[ -n "$line" ]]; then
      TRUSTED_FINGERPRINTS+=("$line")
    fi
  done <<< "$FILE_KEYS"
fi

if [[ $DRY_RUN -eq 1 ]]; then
  echo "--- Dry-run Verification Plan ---"
  MANIFEST="$PACKET_DIR/SHA256SUMS.txt"
  SIGNATURE="$PACKET_DIR/SHA256SUMS.txt.asc"
  if [[ -f "$MANIFEST" ]]; then
    echo "Manifest file: $MANIFEST ($(wc -l < "$MANIFEST") entries)"
  else
    echo "Manifest file: $MANIFEST (not present yet)"
  fi
  if [[ -f "$SIGNATURE" ]]; then
    echo "Signature file: $SIGNATURE (present)"
  else
    echo "Signature file: $SIGNATURE (not present)"
  fi
  echo "Trusted Fingerprints configured: ${#TRUSTED_FINGERPRINTS[@]}"
  for fpr in "${TRUSTED_FINGERPRINTS[@]}"; do
    echo "  - $fpr"
  done
  echo "Dry-run complete."
  exit 0
fi

if [[ ! -d "$PACKET_DIR" ]]; then
  echo "ERROR: release packet directory does not exist: $PACKET_DIR" >&2
  exit 2
fi

MANIFEST="$PACKET_DIR/SHA256SUMS.txt"
if [[ ! -f "$MANIFEST" ]]; then
  echo "ERROR: checksum manifest missing: $MANIFEST" >&2
  exit 1
fi

SIGNATURE="$PACKET_DIR/SHA256SUMS.txt.asc"
SIG_EXISTS=0
if [[ -f "$SIGNATURE" ]]; then
  SIG_EXISTS=1
fi

if [[ $REQUIRE_SIGNATURE -eq 1 && $SIG_EXISTS -eq 0 ]]; then
  echo "ERROR: release signature missing ($SIGNATURE) but --require-signature was specified." >&2
  exit 1
fi

# Step 1: Signature verification
if [[ $SIG_EXISTS -eq 1 ]]; then
  echo "Verifying detached OpenPGP signature: $(basename "$SIGNATURE")..."
  if ! command -v gpg >/dev/null 2>&1; then
    echo "ERROR: gpg binary is required to verify release packet signatures." >&2
    exit 1
  fi

  # Machine-readable status goes to its own file (--status-file); gpg's human
  # output is kept separate. The gpg EXIT CODE is preserved: a non-zero exit
  # is a verification failure even when VALIDSIG (cryptographic validity
  # alone) is present — VALIDSIG can accompany EXPSIG/EXPKEYSIG/REVKEYSIG.
  GPG_STATUS_FILE=$(mktemp)
  GPG_LOG_FILE=$(mktemp)
  TMP_FILES+=("$GPG_STATUS_FILE" "$GPG_LOG_FILE")
  if gpg --batch --status-file "$GPG_STATUS_FILE" --verify "$SIGNATURE" "$MANIFEST" >"$GPG_LOG_FILE" 2>&1; then
    GPG_FAILED=0
  else
    GPG_FAILED=1
  fi

  # Reject disqualifying statuses explicitly, with actionable diagnostics.
  if grep -q '^\[GNUPG:\] REVKEYSIG' "$GPG_STATUS_FILE"; then
    KEY_ID=$(grep '^\[GNUPG:\] REVKEYSIG' "$GPG_STATUS_FILE" | head -n1 | awk '{print $3}')
    echo "ERROR: signature made by REVOKED key $KEY_ID — rejected regardless of fingerprint allowlisting." >&2
    exit 1
  fi
  if grep -q '^\[GNUPG:\] EXPKEYSIG' "$GPG_STATUS_FILE"; then
    KEY_ID=$(grep '^\[GNUPG:\] EXPKEYSIG' "$GPG_STATUS_FILE" | head -n1 | awk '{print $3}')
    echo "ERROR: signature made by EXPIRED key $KEY_ID — rejected (no GOODSIG)." >&2
    exit 1
  fi
  if grep -q '^\[GNUPG:\] EXPSIG' "$GPG_STATUS_FILE"; then
    echo "ERROR: signature itself has EXPIRED — rejected." >&2
    exit 1
  fi
  if grep -q '^\[GNUPG:\] NO_PUBKEY' "$GPG_STATUS_FILE"; then
    KEY_ID=$(grep '^\[GNUPG:\] NO_PUBKEY' "$GPG_STATUS_FILE" | head -n1 | awk '{print $3}')
    echo "ERROR: signing public key $KEY_ID is not present in the local keyring." >&2
    exit 1
  fi
  if grep -qE '^\[GNUPG:\] (BADSIG|ERRSIG|UNEXPECTED)' "$GPG_STATUS_FILE"; then
    echo "ERROR: invalid or corrupted signature detected in $SIGNATURE" >&2
    grep -E '^\[GNUPG:\] (BADSIG|ERRSIG|UNEXPECTED)' "$GPG_STATUS_FILE" >&2
    exit 1
  fi
  if [[ $GPG_FAILED -ne 0 ]]; then
    echo "ERROR: gpg exited non-zero while verifying $SIGNATURE:" >&2
    cat "$GPG_LOG_FILE" >&2
    exit 1
  fi

  # A normal acceptance requires BOTH GOODSIG (key/signature status accepted)
  # and VALIDSIG (cryptographic validity) for the same signature.
  if ! grep -q '^\[GNUPG:\] GOODSIG' "$GPG_STATUS_FILE"; then
    echo "ERROR: no [GNUPG:] GOODSIG status — signature key/signature status not acceptable (expired or revoked variants are rejected)." >&2
    cat "$GPG_LOG_FILE" >&2
    exit 1
  fi
  if ! grep -q '^\[GNUPG:\] VALIDSIG' "$GPG_STATUS_FILE"; then
    echo "ERROR: no [GNUPG:] VALIDSIG status — signature is not cryptographically valid." >&2
    cat "$GPG_LOG_FILE" >&2
    exit 1
  fi

  # VALIDSIG fields: $3 = fingerprint of the key that MADE the signature
  # (a signing SUBKEY when subkeys are used); the LAST field = PRIMARY key
  # fingerprint. The trusted-publishers registry pins PRIMARY fingerprints,
  # so the primary field is authoritative for the trust decision.
  SIG_SIGNING_FPR=$(grep '^\[GNUPG:\] VALIDSIG' "$GPG_STATUS_FILE" | head -n1 | awk '{print $3}' | tr '[:lower:]' '[:upper:]')
  SIG_PRIMARY_FPR=$(grep '^\[GNUPG:\] VALIDSIG' "$GPG_STATUS_FILE" | head -n1 | awk '{print $NF}' | tr '[:lower:]' '[:upper:]')
  echo "Signature by signing-key $SIG_SIGNING_FPR (primary key $SIG_PRIMARY_FPR)"

  # Verify the PRIMARY key fingerprint is an authorized publisher.
  IS_TRUSTED=0
  for tfpr in "${TRUSTED_FINGERPRINTS[@]}"; do
    if [[ "$SIG_PRIMARY_FPR" == "$tfpr" ]]; then
      IS_TRUSTED=1
      break
    fi
  done

  if [[ $IS_TRUSTED -eq 0 ]]; then
    echo "ERROR: signer primary key $SIG_PRIMARY_FPR is NOT authorized in trusted publishers list." >&2
    echo "Configured trusted keys:" >&2
    for tfpr in "${TRUSTED_FINGERPRINTS[@]}"; do
      echo "  - $tfpr" >&2
    done
    exit 1
  fi

  echo "✓ SIGNATURE-OK: authentic signature from trusted publisher (primary key $SIG_PRIMARY_FPR)"
else
  echo "Notice: packet is unsigned (SHA256SUMS.txt.asc absent). Skipped signature check."
fi

# Step 2: Checksum verification
echo "Verifying SHA-256 artifact checksums from $(basename "$MANIFEST")..."
(
  cd "$PACKET_DIR"
  sha256sum -c "$(basename "$MANIFEST")"
)
CHECKSUM_COUNT=$(wc -l < "$MANIFEST")
echo "✓ CHECKSUMS-OK: $CHECKSUM_COUNT files verified identically against manifest"

# Step 3: Completeness check (ensure no unexpected unlisted files).
# The manifest is parsed into a set of literal paths and compared with
# fixed-string whole-line matching — filenames are never treated as regex
# (a manifest entry "velqu-runtime" must NOT cover an unlisted
# "velqu.runtime").
echo "Verifying packet completeness (no unlisted extraneous files)..."
MANIFEST_PATHS_FILE=$(mktemp)
TMP_FILES+=("$MANIFEST_PATHS_FILE")
sed -E 's/^[0-9a-fA-F]{64}[[:space:]]+//' "$MANIFEST" | sed 's|^\./||' > "$MANIFEST_PATHS_FILE"
UNLISTED_FILES=()
while IFS= read -r f; do
  rel="${f#$PACKET_DIR/}"
  if [[ "$rel" == "SHA256SUMS.txt" || "$rel" == "SHA256SUMS.txt.asc" ]]; then
    continue
  fi
  if ! grep -Fxq "$rel" "$MANIFEST_PATHS_FILE"; then
    UNLISTED_FILES+=("$rel")
  fi
done < <(find "$PACKET_DIR" -type f | LC_ALL=C sort)

if [[ ${#UNLISTED_FILES[@]} -gt 0 ]]; then
  echo "ERROR: packet directory contains unlisted files not covered by SHA256SUMS.txt:" >&2
  for uf in "${UNLISTED_FILES[@]}"; do
    echo "  - $uf" >&2
  done
  exit 1
fi

echo "✓ PACKET-COMPLETE: all files accounted for in manifest"
echo "=== RELEASE PACKET VERIFICATION PASSED ==="
exit 0
