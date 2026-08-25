/**
 * BETA-001-B: candidate version pin manifest tests.
 * versions.json is the single pin source for the real-world harness; it must
 * agree with compose.yaml (Postgres image) and the CI Bun pin, and every
 * registry version must be exact (no ranges).
 */
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";

const versions = JSON.parse(readFileSync(import.meta.dir + "/versions.json", "utf8"));
const compose = readFileSync(import.meta.dir + "/compose.yaml", "utf8");
const verifyYml = readFileSync(import.meta.dir + "/../../.github/workflows/verify.yml", "utf8");

const EXACT = /^\d+\.\d+\.\d+(-[\w.]+)?$/;

describe("real-world version pins", () => {
  test("manifest format is v1", () => {
    expect(versions.format).toBe("velqu-realworld-versions-v1");
  });

  test("postgres image pin matches compose.yaml exactly", () => {
    expect(versions.postgresImage).toMatch(/^postgres:\d+\.\d+(\.\d+)?-alpine\d+\.\d+$/);
    expect(compose).toContain(`image: ${versions.postgresImage}`);
  });

  test("bun pin matches the CI workflow pin", () => {
    expect(versions.bun).toMatch(EXACT);
    expect(verifyYml).toContain(`bun-version: ${versions.bun}`);
  });

  test("node LTS major is a positive integer (SPEC requires Node 22 LTS)", () => {
    expect(Number.isInteger(versions.nodeLtsMajor)).toBe(true);
    expect(versions.nodeLtsMajor).toBe(22);
  });

  test("every candidate and driver version is exact — no ranges or prefixes", () => {
    for (const [name, v] of Object.entries({ ...versions.candidates, ...versions.drivers })) {
      if (name === "velqu") continue; // workspace source, pinned by commit
      expect(typeof v).toBe("string");
      expect((v as string)).toMatch(EXACT);
    }
  });

  test("all four SPEC candidates are pinned", () => {
    for (const name of ["velqu", "elysia", "hono", "fastify"]) {
      expect(versions.candidates[name]).toBeTruthy();
    }
  });

  test("elysia pin matches the existing baseline package.json", () => {
    const baseline = JSON.parse(readFileSync(import.meta.dir + "/../../baselines/elysia2/package.json", "utf8"));
    expect(versions.candidates.elysia).toBe(baseline.dependencies.elysia);
  });
});
