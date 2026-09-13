import { describe, expect, test, beforeAll, afterAll } from "bun:test";
import { join } from "node:path";
import { mkdtempSync, writeFileSync, rmSync, mkdirSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";

const root = join(import.meta.dir, "..");
const verifierBin = join(root, "scripts", "verify-release-packet.sh");

let testGpgHome: string;
let trustedFingerprint: string;
let untrustedFingerprint: string;

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
});

afterAll(() => {
  if (testGpgHome) {
    try {
      rmSync(testGpgHome, { recursive: true, force: true });
    } catch {}
  }
});

function createMockPacket(packetDir: string) {
  mkdirSync(packetDir, { recursive: true });
  writeFileSync(join(packetDir, "SOURCE-COMMIT.txt"), "4dab05b0a90ef1e0e5152f000624445958d9b68f\n");
  writeFileSync(join(packetDir, "velqu-runtime"), "#!/bin/sh\necho runtime binary\n");
  writeFileSync(join(packetDir, "sbom.cdx.json"), '{"bomFormat":"CycloneDX","specVersion":"1.5"}\n');

  // Generate manifest
  const genManifest = Bun.spawnSync(
    ["bash", "-c", "find . -type f ! -name SHA256SUMS.txt ! -name 'SHA256SUMS.txt.asc' | sed 's|^\\./||' | LC_ALL=C sort | while IFS= read -r f; do sha256sum \"$f\"; done > SHA256SUMS.txt"],
    { cwd: packetDir, stdout: "pipe", stderr: "pipe" },
  );
  expect(genManifest.exitCode).toBe(0);
}

function signManifest(packetDir: string, signerKey: string) {
  const signProc = Bun.spawnSync(
    ["gpg", "--batch", "--yes", "--armor", "--default-key", signerKey, "--detach-sign", "--output", "SHA256SUMS.txt.asc", "SHA256SUMS.txt"],
    { cwd: packetDir, env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
  );
  expect(signProc.exitCode).toBe(0);
}

describe("verify-release-packet verification suite (#1321 / M8-003)", () => {
  test("clean release packet with valid trusted signature passes", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-clean-"));
    try {
      createMockPacket(packetDir);
      signManifest(packetDir, trustedFingerprint);

      const proc = Bun.spawnSync(
        ["bash", verifierBin, "--packet-dir", packetDir, "--require-signature", "--trusted-key", trustedFingerprint],
        { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
      );
      const stdout = new TextDecoder().decode(proc.stdout);
      expect(proc.exitCode).toBe(0);
      expect(stdout).toContain("SIGNATURE-OK: authentic signature from trusted publisher");
      expect(stdout).toContain("CHECKSUMS-OK: 3 files verified");
      expect(stdout).toContain("RELEASE PACKET VERIFICATION PASSED");
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

      const proc = Bun.spawnSync(
        ["bash", verifierBin, "--packet-dir", packetDir, "--require-signature", "--trusted-key", trustedFingerprint],
        { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
      );
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

      const proc = Bun.spawnSync(
        ["bash", verifierBin, "--packet-dir", packetDir, "--require-signature", "--trusted-key", trustedFingerprint],
        { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
      );
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

      const proc = Bun.spawnSync(
        ["bash", verifierBin, "--packet-dir", packetDir, "--require-signature", "--trusted-key", trustedFingerprint],
        { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
      );
      const stderr = new TextDecoder().decode(proc.stderr);
      expect(proc.exitCode).toBe(1);
      expect(stderr).toContain("is NOT authorized in trusted publishers list");
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  });

  test("fail-closed on corrupted signature file", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-corrupt-sig-"));
    try {
      createMockPacket(packetDir);
      signManifest(packetDir, trustedFingerprint);

      // Corrupt signature
      writeFileSync(join(packetDir, "SHA256SUMS.txt.asc"), "-----BEGIN PGP SIGNATURE-----\ncorrupted garbage\n-----END PGP SIGNATURE-----\n");

      const proc = Bun.spawnSync(
        ["bash", verifierBin, "--packet-dir", packetDir, "--require-signature", "--trusted-key", trustedFingerprint],
        { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
      );
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

      const proc = Bun.spawnSync(
        ["bash", verifierBin, "--packet-dir", packetDir, "--require-signature"],
        { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
      );
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

      const proc = Bun.spawnSync(
        ["bash", verifierBin, "--packet-dir", packetDir, "--require-signature", "--trusted-key", trustedFingerprint],
        { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
      );
      const stderr = new TextDecoder().decode(proc.stderr);
      expect(proc.exitCode).toBe(1);
      expect(stderr).toContain("packet directory contains unlisted files not covered by SHA256SUMS.txt");
      expect(stderr).toContain("unlisted-secret.key");
    } finally {
      rmSync(packetDir, { recursive: true, force: true });
    }
  });

  test("dry-run displays plan without executing verification", () => {
    const packetDir = mkdtempSync(join(tmpdir(), "mock-packet-dry-run-"));
    try {
      createMockPacket(packetDir);
      signManifest(packetDir, trustedFingerprint);

      const proc = Bun.spawnSync(
        ["bash", verifierBin, "--packet-dir", packetDir, "--dry-run", "--trusted-key", trustedFingerprint],
        { env: { ...process.env, GNUPGHOME: testGpgHome }, stdout: "pipe", stderr: "pipe" },
      );
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
