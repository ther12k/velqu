/**
 * Schema Conformance Suite (SCHEMA-001..005):
 * - Schema IR v2 builders producing exact JSON nodes
 * - source-aware coercion (path/query vs body)
 * - validation problems producing 422 with failing field paths
 */

import { describe, expect, test } from "bun:test";
import { s, s_transform, s_file, s_problem, SCHEMA_IR_VERSION, MAX_FILE_BYTES, type Schema, type Infer } from "@velqu/schema";

describe("Schema IR v1 builders (SCHEMA-001)", () => {
  test("string schema options and IR structure", () => {
    const Str = s.string({ minLength: 1, maxLength: 60, format: "email" });
    const ir = Str as any;
    expect(ir.kind).toBe("string");
    expect(ir.minLength).toBe(1);
    expect(ir.maxLength).toBe(60);
    expect(ir.format).toBe("email");
  });

  test("integer and number schemas", () => {
    const Int = s.integer({ minimum: 0, maximum: 100 });
    const ir = Int as any;
    expect(ir.kind).toBe("integer");
    expect(ir.minimum).toBe(0);
    expect(ir.maximum).toBe(100);
  });

  test("boolean, literal, enum, optional with default, nullable", () => {
    const Bool = s.boolean();
    expect((Bool as any).kind).toBe("boolean");

    const Lit = s.literal("active");
    expect((Lit as any).kind).toBe("literal");
    expect((Lit as any).value).toBe("active");

    const Enum = s.enum(["a", "b", "c"] as const);
    expect((Enum as any).kind).toBe("enum");
    expect((Enum as any).values).toEqual(["a", "b", "c"]);

    const Opt = s.optional(s.integer(), { default: 10 });
    expect((Opt as any).kind).toBe("optional");
    expect((Opt as any).default).toBe(10);

    const Nullable = s.nullable(s.string());
    expect((Nullable as any).kind).toBe("nullable");
  });

  test("object required inference vs optional fields", () => {
    const Obj = s.object({
      name: s.string(),
      age: s.optional(s.integer()),
      role: s.literal("admin"),
    });
    const ir = Obj as any;
    expect(ir.kind).toBe("object");
    expect(ir.required).toEqual(["name", "role"]);
    expect(Object.keys(ir.properties)).toEqual(["name", "age", "role"]);
  });

  test("array schema", () => {
    const Arr = s.array(s.string(), { minItems: 1, maxItems: 10 });
    const ir = Arr as any;
    expect(ir.kind).toBe("array");
    expect(ir.minItems).toBe(1);
    expect(ir.maxItems).toBe(10);
    expect(ir.items.kind).toBe("string");
  });

  test("bounded union schema (max 4 members)", () => {
    const Union = s.union([s.string(), s.integer()]);
    const ir = Union as any;
    expect(ir.kind).toBe("union");
    expect(ir.members.length).toBe(2);
  });
});

describe("Schema IR v2 nodes (SCHEMA-001, IR v2)", () => {
  test("version constant is 2 and exported", () => {
    expect(SCHEMA_IR_VERSION).toBe(2);
  });

  test("transform builder emits declarative input/output/name node", () => {
    const ir = s_transform(s.string(), s.integer(), "parse-count") as any;
    expect(ir).toEqual({ kind: "transform", input: { kind: "string" }, output: { kind: "integer" }, name: "parse-count" });
    expect(Object.keys(ir)).toEqual(["kind", "input", "output", "name"]);
  });

  test("transform rejects unbounded or malformed names", () => {
    expect(() => s_transform(s.string(), s.string(), "")).toThrow();
    expect(() => s_transform(s.string(), s.string(), "has spaces")).toThrow();
    expect(() => s_transform(s.string(), s.string(), "x".repeat(65))).toThrow();
    expect(() => s_transform(s.string(), s.string(), "injection; drop table")).toThrow();
  });

  test("transform carries no executable surface (data only)", () => {
    const ir = s_transform(s.string(), s.string(), "trim") as any;
    for (const v of Object.values(ir)) {
      expect(typeof v === "function").toBe(false);
    }
  });

  test("file builder emits bounded metadata node in canonical field order", () => {
    const ir = s_file({ contentType: "text/csv", maxBytes: 4096 }) as any;
    expect(ir).toEqual({ kind: "file", contentType: "text/csv", maxBytes: 4096 });
    expect(Object.keys(ir)).toEqual(["kind", "contentType", "maxBytes"]);

    const bare = s_file({ maxBytes: 1 }) as any;
    expect(bare).toEqual({ kind: "file", maxBytes: 1 });
    expect("contentType" in bare).toBe(false);
  });

  test("file bounds: maxBytes integer in [1, 16 MiB], contentType 1..128", () => {
    expect(() => s_file({ maxBytes: 0 })).toThrow();
    expect(() => s_file({ maxBytes: -1 })).toThrow();
    expect(() => s_file({ maxBytes: 1.5 })).toThrow();
    expect(() => s_file({ maxBytes: MAX_FILE_BYTES + 1 })).toThrow();
    expect(() => s_file({ maxBytes: Number.MAX_SAFE_INTEGER })).toThrow();
    expect(() => s_file({ maxBytes: 1, contentType: "" })).toThrow();
    expect(() => s_file({ maxBytes: 1, contentType: "x".repeat(129) })).toThrow();
    expect(s_file({ maxBytes: MAX_FILE_BYTES }) as any).toEqual({ kind: "file", maxBytes: MAX_FILE_BYTES });
  });

  test("problem builder emits RFC 9457 metadata in canonical field order", () => {
    const ir = s_problem({ typeUri: "https://example.com/probs/oos", title: "Out of stock", status: 409, detail: s.string() }) as any;
    expect(ir).toEqual({
      kind: "problem",
      typeUri: "https://example.com/probs/oos",
      title: "Out of stock",
      status: 409,
      detail: { kind: "string" },
    });
    expect(Object.keys(ir)).toEqual(["kind", "typeUri", "title", "status", "detail"]);

    const minimal = s_problem({ title: "Boom", status: 500 }) as any;
    expect(minimal).toEqual({ kind: "problem", title: "Boom", status: 500 });
  });

  test("problem bounds: title 1..128, status 400..599, typeUri <= 2048", () => {
    expect(() => s_problem({ title: "", status: 400 })).toThrow();
    expect(() => s_problem({ title: "x".repeat(129), status: 400 })).toThrow();
    expect(() => s_problem({ title: "x", status: 399 })).toThrow();
    expect(() => s_problem({ title: "x", status: 600 })).toThrow();
    expect(() => s_problem({ title: "x", status: 404.5 })).toThrow();
    expect(() => s_problem({ title: "x", status: 400, typeUri: "u".repeat(2049) })).toThrow();
  });

  test("v2 nodes compose inside objects, arrays, and unions", () => {
    const Upload = s.object({
      payload: s_file({ maxBytes: 1024 }),
      tags: s.array(s.string()),
      normalized: s.nullable(s.integer()),
      failure: s_problem({ title: "Rejected", status: 422 }),
    }) as any;
    expect(Upload.kind).toBe("object");
    // only s.optional members leave the required list; nullable stays required
    expect(Upload.required).toEqual(["payload", "tags", "normalized", "failure"]);
    expect(Upload.properties.payload.kind).toBe("file");
    expect(Upload.properties.failure.kind).toBe("problem");
    expect(Upload.properties.normalized.kind).toBe("nullable");

    const Union = s.union([s.string(), s_transform(s.string(), s.integer(), "len")]) as any;
    expect(Union.members.map((m: any) => m.kind)).toEqual(["string", "transform"]);
  });
});

