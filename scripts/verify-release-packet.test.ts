import { describe, expect, test, beforeAll, afterAll } from "bun:test";
import { join } from "node:path";
import { mkdtempSync, writeFileSync, rmSync, mkdirSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";

const root = join(import.meta.dir, "..");
const verifierBin = join(root, "scripts", "verify-release-packet.sh");

let testGpgHome: string;
let trustedFingerprint: string;
let untrustedFingerprint: string;
let revokedFingerprint: string;
let revokedRevCert: string;

function getFingerprint(gpgHome: string, keyName: string): string {
  const proc = Bun.spawnSync(["gpg", "--batch", "--with-colons", "--fingerprint", keyName], {
    env: { ...process.env, GNUPGHOME: gpgHome },
    stdout: "pipe",
    stderr: "pipe",
  });
  const stdout = new TextDecoder().decode(proc.stdout);
  for (const line of stdout.split("\n")) {
    if (line.startsWith("fpr:")) {
      const parts = line.split(":");
      return parts[9]!.trim().toUpperCase();
    }
  }
  throw new Error(`Failed to extract fingerprint for ${keyName} from:\n${stdout}`);
}

beforeAll(() => {
  testGpgHome = mkdtempSync(join(tmpdir(), "velqu-gpg-test-"));

  // Generate trusted test key
  const genTrusted = Bun.spawnSync(
    ["gpg", "--batch", "--passphrase", "", "--quick-generate-key", "Velqu Test Trusted <trusted@velqu.test>", "default", "default"],
    { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
  );
  if (genTrusted.exitCode !== 0) {
    throw new Error(`Failed to generate trusted test key: ${new TextDecoder().decode(genTrusted.stderr)}`);
  }
  trustedFingerprint = getFingerprint(testGpgHome, "trusted@velqu.test");

  // Generate untrusted test key
  const genUntrusted = Bun.spawnSync(
    ["gpg", "--batch", "--passphrase", "", "--quick-generate-key", "Velqu Untrusted Signer <untrusted@attacker.test>", "default", "default"],
    { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
  );
  if (genUntrusted.exitCode !== 0) {
    throw new Error(`Failed to generate untrusted test key: ${new TextDecoder().decode(genUntrusted.stderr)}`);
  }
  untrustedFingerprint = getFingerprint(testGpgHome, "untrusted@attacker.test");

  // Generate a key that will later be revoked (keep its revocation certificate)
  const genRevoked = Bun.spawnSync(
    ["gpg", "--batch", "--passphrase", "", "--quick-generate-key", "Velqu Revoked Signer <revoked@velqu.test>", "default", "default"],
    { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
  );
  if (genRevoked.exitCode !== 0) {
    throw new Error(`Failed to generate revoked test key: ${new TextDecoder().decode(genRevoked.stderr)}`);
  }
  revokedFingerprint = getFingerprint(testGpgHome, "revoked@velqu.test");
  revokedRevCert = join(testGpgHome, "openpgp-revocs.d", `${revokedFingerprint}.rev`);
});

afterAll(() => {
  if (testGpgHome) {
    try {
      rmSync(testGpgHome, { recursive: true, force: true });
    } catch {}
  }
});

/**
 * Builds a mock release packet mirroring scripts/release-packet semantics:
 * the top-level manifest excludes ONLY ./SHA256SUMS.txt and
 * ./SHA256SUMS.txt.asc (path-based); nested SHA256SUMS.txt files ARE
 * checksummed into the top-level manifest.
 */
function createMockPacket(packetDir: string, opts?: { nestedTarballs?: boolean }) {
  mkdirSync(packetDir, { recursive: true });
  writeFileSync(join(packetDir, "SOURCE-COMMIT.txt"), "4dab05b0a90ef1e0e5152f000624445958d9b68f\n");
  writeFileSync(join(packetDir, "velqu-runtime"), "#!/bin/sh\necho runtime binary\n");
  writeFileSync(join(packetDir, "sbom.cdx.json"), '{"bomFormat":"CycloneDX","specVersion":"1.5"}\n');

  if (opts?.nestedTarballs) {
    // Mirrors scripts/npm-package-tarballs.sh: a nested directory with its
    // own SHA256SUMS.txt over the tarballs.
    const tarballsDir = join(packetDir, "npm-tarballs");
    mkdirSync(tarballsDir, { recursive: true });
    writeFileSync(join(tarballsDir, "velqu-core-0.1.0.tgz"), "fake core tarball\n");
    writeFileSync(join(tarballsDir, "velqu-cli-0.1.0.tgz"), "fake cli tarball\n");
    const nested = Bun.spawnSync(
      ["bash", "-c", "find . -type f ! -name SHA256SUMS.txt | sed 's|^\\./||' | LC_ALL=C sort | while IFS= read -r f; do sha256sum \"$f\"; done > SHA256SUMS.txt"],
      { cwd: tarballsDir, stdout: "pipe", stderr: "pipe" },
    );
    expect(nested.exitCode).toBe(0);
  }

  // Top-level manifest: path-based exclusion matching the FIXED release-packet.
  const genManifest = Bun.spawnSync(
    ["bash", "-c", "find . -type f ! -path './SHA256SUMS.txt' ! -path './SHA256SUMS.txt.asc' | sed 's|^\\./||' | LC_ALL=C sort | while IFS= read -r f; do sha256sum \"$f\"; done > SHA256SUMS.txt"],
    { cwd: packetDir, stdout: "pipe", stderr: "pipe" },
  );
  expect(genManifest.exitCode).toBe(0);
}

function signManifest(packetDir: string, signerKey: string) {
  const signProc = Bun.spawnSync(
    ["gpg", "--batch", "--yes", "--armor", "--local-user", signerKey, "--detach-sign", "--output", "SHA256SUMS.txt.asc", "SHA256SUMS.txt"],
    { cwd: packetDir, env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
  );
  expect(signProc.exitCode).toBe(0);
}

function verifyPacket(packetDir: string, args: string[]) {
  return Bun.spawnSync(
    ["bash", verifierBin, "--packet-dir", packetDir, ...args],
    { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
  );
}

describe("verify-release-packet verification suite (#1321 / M8-003)", () => {
  test("clean release packet with valid trusted signature passes", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-clean-"));
    try {
      createMockPacket(packetDir);
      signManifest(packetDir, trustedFingerprint);

      const proc = verifyPacket(packetDir, ["--require-signature", "--trusted-key", trustedFingerprint]);
      const stdout = new TextDecoder().decode(proc.stdout);
      expect(proc.exitCode).toBe(0);
      expect(stdout).toContain("SIGNATURE-OK: authentic signature from trusted publisher");
      expect(stdout).toContain("CHECKSUMS-OK: 3 files verified");
      expect(stdout).toContain("RELEASE PACKET VERIFICATION PASSED");
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  });

  test("nested npm-tarballs SHA256SUMS.txt is checksummed into the top-level manifest and verifies", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-nested-"));
    try {
      createMockPacket(packetDir, { nestedTarballs: true });
      signManifest(packetDir, trustedFingerprint);

      const proc = verifyPacket(packetDir, ["--require-signature", "--trusted-key", trustedFingerprint]);
      const stdout = new TextDecoder().decode(proc.stdout);
      // The nested manifest itself must be listed in the top-level manifest
      // (6 files: 3 top-level artifacts + 2 tarballs + nested SHA256SUMS.txt).
      expect(proc.exitCode).toBe(0);
      expect(stdout).toContain("npm-tarballs/SHA256SUMS.txt: OK");
      expect(stdout).toContain("CHECKSUMS-OK: 6 files verified");
      expect(stdout).toContain("PACKET-COMPLETE");
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  });

  test("fail-closed on tampered artifact (checksum mismatch)", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-tamper-art-"));
    try {
      createMockPacket(packetDir);
      signManifest(packetDir, trustedFingerprint);

      // Tamper artifact
      writeFileSync(join(packetDir, "velqu-runtime"), "MALICIOUS PAYLOAD\n");

      const proc = verifyPacket(packetDir, ["--require-signature", "--trusted-key", trustedFingerprint]);
      expect(proc.exitCode).toBe(1);
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  });

  test("fail-closed on tampered manifest (signature mismatch)", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-tamper-man-"));
    try {
      createMockPacket(packetDir);
      signManifest(packetDir, trustedFingerprint);

      // Tamper manifest
      const manifestPath = join(packetDir, "SHA256SUMS.txt");
      writeFileSync(manifestPath, readFileSync(manifestPath, "utf8") + "0000000000000000000000000000000000000000000000000000000000000000  hacked.bin\n");

      const proc = verifyPacket(packetDir, ["--require-signature", "--trusted-key", trustedFingerprint]);
      const stderr = new TextDecoder().decode(proc.stderr);
      expect(proc.exitCode).toBe(1);
      expect(stderr).toContain("invalid or corrupted signature detected");
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  });

  test("fail-closed on untrusted signer key", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-untrusted-"));
    try {
      createMockPacket(packetDir);
      // Signed with untrusted key
      signManifest(packetDir, untrustedFingerprint);

      const proc = verifyPacket(packetDir, ["--require-signature", "--trusted-key", trustedFingerprint]);
      const stderr = new TextDecoder().decode(proc.stderr);
      expect(proc.exitCode).toBe(1);
      expect(stderr).toContain("is NOT authorized in trusted publishers list");
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  });

  test("fail-closed on REVOKED signing key (VALIDSIG alone must not pass)", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-revoked-"));
    try {
      createMockPacket(packetDir);
      // Sign while the key is still valid...
      signManifest(packetDir, revokedFingerprint);

      // ...then revoke it in the keyring via its revocation certificate.
      // GnuPG stores the cert with a protective leading colon; strip it.
      const revRaw = readFileSync(revokedRevCert, "utf8");
      const revClean = revRaw.replace(/^:-----/m, "-----");
      const revPath = join(packetDir, "clean-rev.asc");
      writeFileSync(revPath, revClean);
      const importProc = Bun.spawnSync(["gpg", "--batch", "--import", revPath], {
        env: { ...process.env, GNUPGHOME: testGpgHome },
        stdout: "pipe",
        stderr: "pipe",
      });
      expect(importProc.exitCode).toBe(0);
      rmSync(revPath, { force: true });

      const proc = verifyPacket(packetDir, ["--require-signature", "--trusted-key", revokedFingerprint]);
      const stderr = new TextDecoder().decode(proc.stderr);
      expect(proc.exitCode).toBe(1);
      expect(stderr).toContain("REVOKED key");
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  }, 20_000);

  test("fail-closed on EXPIRED signing key (no GOODSIG)", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-expired-"));
    try {
      // Dedicated short-lived key in the shared test keyring home.
      const genProc = Bun.spawnSync(
        ["gpg", "--batch", "--passphrase", "", "--quick-generate-key", "Velqu Expiring Signer <expiring@velqu.test>", "default", "default", "seconds=4"],
        { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
      );
      expect(genProc.exitCode).toBe(0);
      const expiringFpr = getFingerprint(testGpgHome, "expiring@velqu.test");

      createMockPacket(packetDir);
      // Sign while the key is still valid...
      signManifest(packetDir, expiringFpr);
      // ...then let the key expire before verification.
      const sleepProc = Bun.spawnSync(["sleep", "6"], { stdout: "pipe", stderr: "pipe" });
      expect(sleepProc.exitCode).toBe(0);

      const proc = verifyPacket(packetDir, ["--require-signature", "--trusted-key", expiringFpr]);
      const stderr = new TextDecoder().decode(proc.stderr);
      expect(proc.exitCode).toBe(1);
      // Either the explicit EXPKEYSIG rejection or the missing-GOODSIG guard.
      expect(stderr.includes("EXPIRED key") || stderr.includes("no [GNUPG:] GOODSIG status")).toBe(true);
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  }, 30_000);

  test("fail-closed on corrupted signature file", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-corrupt-sig-"));
    try {
      createMockPacket(packetDir);
      signManifest(packetDir, trustedFingerprint);

      // Corrupt signature
      writeFileSync(join(packetDir, "SHA256SUMS.txt.asc"), "-----BEGIN PGP SIGNATURE-----\ncorrupted garbage\n-----END PGP SIGNATURE-----\n");

      const proc = verifyPacket(packetDir, ["--require-signature", "--trusted-key", trustedFingerprint]);
      expect(proc.exitCode).toBe(1);
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  });

  test("fail-closed on missing signature when --require-signature is specified", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-missing-sig-"));
    try {
      createMockPacket(packetDir);
      // No signature created

      const proc = verifyPacket(packetDir, ["--require-signature"]);
      const stderr = new TextDecoder().decode(proc.stderr);
      expect(proc.exitCode).toBe(1);
      expect(stderr).toContain("release signature missing");
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  });

  test("fail-closed on extraneous unlisted file in release directory", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-unlisted-file-"));
    try {
      createMockPacket(packetDir);
      signManifest(packetDir, trustedFingerprint);

      // Add unlisted file
      writeFileSync(join(packetDir, "unlisted-secret.key"), "should not be here\n");

      const proc = verifyPacket(packetDir, ["--require-signature", "--trusted-key", trustedFingerprint]);
      const stderr = new TextDecoder().decode(proc.stderr);
      expect(proc.exitCode).toBe(1);
      expect(stderr).toContain("packet directory contains unlisted files not covered by SHA256SUMS.txt");
      expect(stderr).toContain("unlisted-secret.key");
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  });

  test("completeness matches filenames literally: velqu.runtime is NOT covered by manifest's velqu-runtime", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-literal-names-"));
    try {
      createMockPacket(packetDir);
      signManifest(packetDir, trustedFingerprint);

      // Manifest records "velqu-runtime". Add an unlisted sibling whose name
      // differs only by regex-meaningful characters: with regex matching, the
      // dot in "velqu.runtime" would match the dash in "velqu-runtime" and
      // the file would slip through; literal matching must reject it.
      writeFileSync(join(packetDir, "velqu.runtime"), "impostor binary\n");

      const proc = verifyPacket(packetDir, ["--require-signature", "--trusted-key", trustedFingerprint]);
      const stderr = new TextDecoder().decode(proc.stderr);
      expect(proc.exitCode).toBe(1);
      expect(stderr).toContain("packet directory contains unlisted files not covered by SHA256SUMS.txt");
      expect(stderr).toContain("velqu.runtime");
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  });

  test("dry-run displays plan without executing verification", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-dry-run-"));
    try {
      createMockPacket(packetDir);
      signManifest(packetDir, trustedFingerprint);

      const proc = verifyPacket(packetDir, ["--dry-run", "--trusted-key", trustedFingerprint]);
      const stdout = new TextDecoder().decode(proc.stdout);
      expect(proc.exitCode).toBe(0);
      expect(stdout).toContain("--- Dry-run Verification Plan ---");
      expect(stdout).toContain("Manifest file:");
      expect(stdout).toContain("Trusted Fingerprints configured:");
      expect(stdout).toContain(trustedFingerprint);
      expect(stdout).toContain("Dry-run complete.");
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  });
});
