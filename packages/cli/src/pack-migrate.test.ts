import { describe, expect, test } from "bun:test";
import { assessPackMigrate, MIGRATION_DOC } from "./pack-migrate";
import { readFileSync } from "node:fs";

describe("pack migrate guidance (M26-008-B)", () => {
  test("golden v1 fixture reports legacy-supported with rebuild guidance", () => {
    const raw = readFileSync(
      new URL("../../../crates/q-pack/tests/fixtures/v1/minimal.json", import.meta.url),
      "utf8",
    );
    const report = assessPackMigrate(() => raw);
    expect(report.status).toBe("legacy-supported");
    if (report.status === "legacy-supported") {
      expect(report.formatVersion).toBe(1);
      expect(report.guidance.join("\n")).toContain("rebuild from source");
      expect(report.guidance.join("\n")).toContain("deterministic");
    }
  });

  test("unknown version fails closed with actionable message", () => {
    const report = assessPackMigrate(() => JSON.stringify({ kind: "velqu.qpack", formatVersion: 7 }));
    expect(report.status).toBe("unsupported");
    if (report.status === "unsupported") {
      expect(report.formatVersion).toBe(7);
      expect(report.message).toContain("not supported");
      expect(report.message).toContain("fail closed");
      expect(report.message).toContain("rebuild");
      expect(report.message).toContain(MIGRATION_DOC);
    }
  });

  test("non-JSON input reports unreadable without pretending to migrate", () => {
    const report = assessPackMigrate(() => "\u0000VQPK-binary");
    expect(report.status).toBe("unreadable");
    if (report.status === "unreadable") {
      expect(report.message).toContain("rebuild from source");
    }
  });

  test("wrong kind is not a pack", () => {
    const report = assessPackMigrate(() => JSON.stringify({ kind: "other", formatVersion: 1 }));
    expect(report.status).toBe("not-a-pack");
  });
});
