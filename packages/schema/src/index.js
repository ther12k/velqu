/**
 * @velqu/schema — Schema IR v2 builders.
 *
 * Each builder returns a value whose RUNTIME representation is exactly the
 * Schema IR JSON node consumed by the Rust runtime (docs/specs/pack-format-v1.md)
 * and whose TYPE carries the inferred TypeScript type. One declaration drives
 * types, runtime validation, Treaty inputs, and OpenAPI (SCHEMA-001).
 */
export const SCHEMA_IR_VERSION = 2;
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
/** Declarative transform: input/output schema pair + stable name. No callbacks. */
export function s_transform(input, output, name) {
    if (!/^[A-Za-z0-9_.:-]{1,64}$/.test(name)) {
        throw new Error("s_transform: name must match [A-Za-z0-9_.:-]{1,64}");
    }
    return { kind: "transform", input, output, name };
}
/** Hard transport bound for file payloads (matches bounded-body limits). */
export const MAX_FILE_BYTES = 16 * 1024 * 1024;
/** Bounded file metadata. The value stays an opaque byte payload owned by transport. */
export function s_file(opts) {
    if (!Number.isSafeInteger(opts.maxBytes) || opts.maxBytes < 1 || opts.maxBytes > MAX_FILE_BYTES) {
        throw new Error(`s_file: maxBytes must be an integer in [1, ${MAX_FILE_BYTES}]`);
    }
    if (opts.contentType !== undefined && (opts.contentType.length === 0 || opts.contentType.length > 128)) {
        throw new Error("s_file: contentType must be 1..128 characters");
    }
    return { kind: "file", ...(opts.contentType !== undefined ? { contentType: opts.contentType } : {}), maxBytes: opts.maxBytes };
}
/** RFC 9457 problem metadata. `detail` is a declarative schema, not free-form. */
export function s_problem(opts) {
    if (opts.title.length === 0 || opts.title.length > 128) {
        throw new Error("s_problem: title must be 1..128 characters");
    }
    if (!Number.isSafeInteger(opts.status) || opts.status < 400 || opts.status > 599) {
        throw new Error("s_problem: status must be an integer in [400, 599]");
    }
    if (opts.typeUri !== undefined && opts.typeUri.length > 2048) {
        throw new Error("s_problem: typeUri must be at most 2048 characters");
    }
    return {
        kind: "problem",
        ...(opts.typeUri !== undefined ? { typeUri: opts.typeUri } : {}),
        title: opts.title,
        status: opts.status,
        ...(opts.detail !== undefined ? { detail: opts.detail } : {}),
    };
}
/** Closed fallback vocabulary (M25-001-B). Mirrors FALLBACK_REASONS in q-schema-runtime. */
export const FALLBACK_REASONS = ["unsupported-transform", "unrepresentable", "measured", "explicit"];
/** Explicit fallback marker (ADR-0009: no silent downgrade). */
export function s_fallback(reason, inner) {
    if (!FALLBACK_REASONS.includes(reason)) {
        throw new Error(`s_fallback: reason must be one of ${FALLBACK_REASONS.join(", ")}`);
    }
    return { kind: "fallback", reason, ...(inner !== undefined ? { inner } : {}) };
}
/** Compatibility markers (M25-001-B): feature tags derived from an IR graph. */
export const FEATURE_TAGS = ["fallback", "file", "problem", "transform"];
export function featuresOf(ir) {
    const seen = new Set();
    const walk = (node) => {
        switch (node.kind) {
            case "transform":
                seen.add("transform");
                walk(node.input);
                walk(node.output);
                break;
            case "file":
                seen.add("file");
                break;
            case "problem":
                seen.add("problem");
                if (node.detail !== undefined)
                    walk(node.detail);
                break;
            case "fallback":
                seen.add("fallback");
                if (node.inner !== undefined)
                    walk(node.inner);
                break;
            case "optional":
            case "nullable":
                walk(node.inner);
                break;
            case "array":
                walk(node.items);
                break;
            case "object":
                for (const p of Object.values(node.properties))
                    walk(p);
                break;
            case "union":
                for (const m of node.members)
                    walk(m);
                break;
            default:
                break;
        }
    };
    walk(ir);
    return [...seen].sort();
}
/** Canonical JSON form (M25-001-C): keys sorted recursively; mirrors `q_schema_runtime::canonical_value`. */
export function canonicalValue(v) {
    if (Array.isArray(v))
        return v.map(canonicalValue);
    if (v && typeof v === "object") {
        const o = v;
        const out = {};
        for (const k of Object.keys(o).sort())
            out[k] = canonicalValue(o[k]);
        return out;
    }
    return v;
}
export function canonicalJson(ir) {
    return JSON.stringify(canonicalValue(ir));
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
    transform: s_transform,
    file: s_file,
    problem: s_problem,
    fallback: s_fallback,
};
