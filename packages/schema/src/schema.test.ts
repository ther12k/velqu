/**
 * #1427 — Schema builder typing and runtime IR conformance.
 *
 * Proves that s.optional() preserves its "optional" discriminator in the type
 * system so RequiredKeys<O> recognizes optional properties during object type
 * inference, while maintaining full assignability to Schema<unknown> and
 * generating identical runtime Schema IR.
 */
import { describe, it, expect, expectTypeOf } from "bun:test";
import { s, type Infer, type Schema, type OptionalSchema } from "./index";

describe("s.optional typing (#1427)", () => {
  it("infers optional properties on object schemas so omitted fields are valid", () => {
    const UserSchema = s.object({
      name: s.string({ minLength: 1 }),
      nickname: s.optional(s.string()),
    });

    type InferredUser = Infer<typeof UserSchema>;

    // Type-level assertions
    expectTypeOf<InferredUser>().toMatchTypeOf<{ name: string; nickname?: string | undefined }>();

    // Value-level assignability (was TS2322 before #1427)
    const omitted: InferredUser = { name: "Ada" };
    expect(omitted.name).toBe("Ada");
    expect(omitted.nickname).toBeUndefined();

    const explicitUndefined: InferredUser = { name: "Ada", nickname: undefined };
    expect(explicitUndefined.nickname).toBeUndefined();

    const provided: InferredUser = { name: "Ada", nickname: "Adie" };
    expect(provided.nickname).toBe("Adie");
  });

  it("handles s.optional with defaults", () => {
    const QuerySchema = s.object({
      page: s.optional(s.integer({ minimum: 1 }), { default: 1 }),
      limit: s.optional(s.integer({ maximum: 100 })),
    });

    type InferredQuery = Infer<typeof QuerySchema>;

    const empty: InferredQuery = {};
    expect(empty.page).toBeUndefined();
    expect(empty.limit).toBeUndefined();

    const partial: InferredQuery = { page: 2 };
    expect(partial.page).toBe(2);
  });

  it("nested object schemas with optional fields infer correctly", () => {
    const NestedSchema = s.object({
      id: s.string(),
      profile: s.object({
        bio: s.optional(s.string({ maxLength: 500 })),
      }),
    });

    type InferredNested = Infer<typeof NestedSchema>;

    const valid: InferredNested = {
      id: "usr_1",
      profile: {},
    };
    expect(valid.id).toBe("usr_1");
    expect(valid.profile.bio).toBeUndefined();
  });

  it("OptionalSchema is assignable to Schema<unknown> and Schema<T | undefined>", () => {
    const opt: OptionalSchema<string> = s.optional(s.string());
    const asSchema: Schema<string | undefined> = opt;
    const asUnknown: Schema<unknown> = opt;

    expect(asSchema.kind).toBe("optional");
    expect(asUnknown.kind).toBe("optional");
  });

  it("runtime Schema IR representation is preserved", () => {
    const obj = s.object({
      requiredField: s.string(),
      optionalField: s.optional(s.string()),
    });

    const ir = obj as unknown as {
      kind: string;
      required: string[];
      properties: Record<string, { kind: string }>;
    };

    expect(ir.kind).toBe("object");
    expect(ir.required).toEqual(["requiredField"]);
    expect(ir.properties.requiredField.kind).toBe("string");
    expect(ir.properties.optionalField.kind).toBe("optional");
  });
});
