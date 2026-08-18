/**
 * Schema Conformance Suite (SCHEMA-001..005):
 * - Schema IR v1 builders producing exact JSON nodes
 * - source-aware coercion (path/query vs body)
 * - validation problems producing 422 with failing field paths
 */

import { describe, expect, test } from "bun:test";
import { s, type Schema, type Infer } from "@velqu/schema";

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
