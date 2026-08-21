/**
 * Golden corpus round-trip (M25-001-A): every committed wire node must be
 * produced byte-identically by the @velqu/schema builders and stay stable
 * under JSON serialization. Rust parity is asserted by the companion test
 * `m25_001_a_tests::golden_corpus_round_trips` in q-schema-runtime.
 */

import { describe, expect, test } from "bun:test";
import { s, s_transform, s_file, s_problem } from "@velqu/schema";

const corpus: Record<string, unknown> = {
  "transform.json": s_transform(
    s.string({ maxLength: 64 }),
    s.integer({ minimum: 0 }),
    "parse-count",
  ),
  "file.json": s_file({ maxBytes: 4096 }),
  "file-content-type.json": s_file({ contentType: "text/csv", maxBytes: 1_048_576 }),
  "problem.json": s_problem({
    typeUri: "https://example.com/probs/out-of-stock",
    title: "Out of stock",
    status: 409,
    detail: s.string({ maxLength: 256 }),
  }),
  "problem-minimal.json": s_problem({ title: "Internal error", status: 500 }),
  "nested-composition.json": s.object({
    upload: s_file({ contentType: "application/octet-stream", maxBytes: 8192 }),
    tags: s.array(s.string({ minLength: 1 }), { minItems: 1, maxItems: 8 }),
    limit: s.optional(s.integer({ minimum: 1, maximum: 100 }), { default: 10 }),
    nickname: s.nullable(s.string({ maxLength: 30 })),
    normalized: s_transform(s.string(), s.integer({ minimum: 0 }), "len"),
    failure: s_problem({ typeUri: "https://example.com/probs/rejected", title: "Rejected", status: 422 }),
    mode: s.enum(["fast", "slow"] as const),
    origin: s.literal("api"),
    either: s.union([s.string(), s.integer()]),
    score: s.number({ minimum: 0, maximum: 1 }),
    active: s.boolean(),
    contact: s.string({ format: "email" }),
    id: s.string({ format: "uuid" }),
    code: s.string({ pattern: "^usr_[0-9]+$" }),
  }),
};

describe("Schema IR v2 golden corpus (M25-001-A)", () => {
  for (const [file, built] of Object.entries(corpus)) {
    test(`${file}: builders reproduce the committed wire node exactly`, async () => {
      const golden = await import(`./golden/${file}`, { with: { type: "json" } });
      expect(JSON.parse(JSON.stringify(built))).toEqual(golden.default);
    });
  }

  test("serialization is key-order stable (wire form is canonical)", async () => {
    const golden = await import("./golden/transform.json", { with: { type: "json" } });
    const built = corpus["transform.json"] as Record<string, unknown>;
    expect(JSON.stringify(built)).toBe(JSON.stringify(golden.default));
  });
});
