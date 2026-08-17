/**
 * @q/schema — Schema IR v1 builders.
 *
 * Each builder returns a value whose RUNTIME representation is exactly the
 * Schema IR JSON node consumed by the Rust runtime (docs/specs/pack-format-v1.md)
 * and whose TYPE carries the inferred TypeScript type. One declaration drives
 * types, runtime validation, Treaty inputs, and OpenAPI (SCHEMA-001).
 */
export function s_string(opts = {}) {
    return { kind: "string", ...opts };
}
export function s_integer(opts = {}) {
    return { kind: "integer", ...opts };
}
export function s_number(opts = {}) {
    return { kind: "number", ...opts };
}
export function s_boolean() {
    return { kind: "boolean" };
}
export function s_literal(value) {
    return { kind: "literal", value };
}
export function s_enum(values) {
    return { kind: "enum", values: [...values] };
}
export function s_optional(inner, opts = {}) {
    return { kind: "optional", inner, ...(opts.default !== undefined ? { default: opts.default } : {}) };
}
export function s_nullable(inner) {
    return { kind: "nullable", inner };
}
export function s_array(items, opts = {}) {
    return { kind: "array", items, ...opts };
}
/** Object: every property required unless wrapped in s_optional. */
export function s_object(properties) {
    const required = Object.entries(properties)
        .filter(([, v]) => v.kind !== "optional")
        .map(([k]) => k);
    return {
        kind: "object",
        properties: { ...properties },
        required,
    };
}
/** Bounded union: at most 4 members (IR limit, enforced by the compiler). */
export function s_union(members) {
    return { kind: "union", members: members.filter((m) => m !== undefined) };
}
/** Convenience namespace so `s.string()` reads naturally. */
export const s = {
    string: s_string,
    integer: s_integer,
    number: s_number,
    boolean: s_boolean,
    literal: s_literal,
    enum: s_enum,
    optional: s_optional,
    nullable: s_nullable,
    array: s_array,
    object: s_object,
    union: s_union,
};
