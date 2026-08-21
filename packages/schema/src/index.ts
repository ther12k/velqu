/**
 * @velqu/schema — Schema IR v2 builders.
 *
 * Each builder returns a value whose RUNTIME representation is exactly the
 * Schema IR JSON node consumed by the Rust runtime (docs/specs/pack-format-v1.md)
 * and whose TYPE carries the inferred TypeScript type. One declaration drives
 * types, runtime validation, Treaty inputs, and OpenAPI (SCHEMA-001).
 */

export const SCHEMA_IR_VERSION = 2 as const;

export type JsonLiteral = null | string | number | boolean | JsonLiteral[] | { readonly [key: string]: JsonLiteral };
export type LiteralValue = JsonLiteral;

export type Schema<T> =
  | { readonly kind: "string"; minLength?: number; maxLength?: number; pattern?: string; format?: string; readonly __t?: T }
  | { readonly kind: "integer"; minimum?: number; maximum?: number; readonly __t?: T }
  | { readonly kind: "number"; minimum?: number; maximum?: number; readonly __t?: T }
  | { readonly kind: "boolean"; readonly __t?: T }
  | { readonly kind: "literal"; readonly value: LiteralValue; readonly __t?: T }
  | { readonly kind: "enum"; readonly values: LiteralValue[]; readonly __t?: T }
  | { readonly kind: "optional"; readonly inner: Schema<unknown>; readonly default?: unknown; readonly __t?: T }
  | { readonly kind: "nullable"; readonly inner: Schema<unknown>; readonly __t?: T }
  | { readonly kind: "array"; readonly items: Schema<unknown>; minItems?: number; maxItems?: number; readonly __t?: T }
  | { readonly kind: "object"; readonly properties: Readonly<Record<string, Schema<unknown>>>; readonly required: readonly string[]; readonly __t?: T }
  | { readonly kind: "union"; readonly members: readonly Schema<unknown>[]; readonly __t?: T }
  | { readonly kind: "transform"; readonly input: Schema<unknown>; readonly output: Schema<unknown>; readonly name: string; readonly __t?: T }
  | { readonly kind: "file"; readonly contentType?: string; readonly maxBytes: number; readonly __t?: T }
  | { readonly kind: "problem"; readonly typeUri?: string; readonly title: string; readonly status: number; readonly detail?: Schema<unknown>; readonly __t?: T }
  | { readonly kind: "fallback"; readonly reason: string; readonly inner?: Schema<unknown>; readonly __t?: T };

/** Infer the TypeScript type carried by a schema. */
export type Infer<S> = S extends { __t?: infer T } ? T : never;

// Clean strips the marker so object inference stays structural.
type StringType = string;

export interface StringOpts {
  minLength?: number;
  maxLength?: number;
  pattern?: string;
  format?: "email" | "uuid";
}
export function s_string(opts: StringOpts = {}): Schema<StringType> {
  return { kind: "string", ...opts } as Schema<StringType>;
}

export interface IntOpts {
  minimum?: number;
  maximum?: number;
}
export function s_integer(opts: IntOpts = {}): Schema<number> {
  return { kind: "integer", ...opts } as Schema<number>;
}
export function s_number(opts: IntOpts = {}): Schema<number> {
  return { kind: "number", ...opts } as Schema<number>;
}
export function s_boolean(): Schema<boolean> {
  return { kind: "boolean" } as Schema<boolean>;
}
export function s_literal<T extends LiteralValue>(value: T): Schema<T> {
  return { kind: "literal", value } as Schema<T>;
}
export function s_enum<T extends LiteralValue>(values: readonly T[]): Schema<T> {
  return { kind: "enum", values: [...values] } as unknown as Schema<T>;
}
export function s_optional<T>(inner: Schema<T>, opts: { default?: T } = {}): Schema<T | undefined> {
  return { kind: "optional", inner, ...(opts.default !== undefined ? { default: opts.default } : {}) } as unknown as Schema<T | undefined>;
}
export function s_nullable<T>(inner: Schema<T>): Schema<T | null> {
  return { kind: "nullable", inner } as unknown as Schema<T | null>;
}
export function s_array<T>(items: Schema<T>, opts: { minItems?: number; maxItems?: number } = {}): Schema<T[]> {
  return { kind: "array", items, ...opts } as unknown as Schema<T[]>;
}

type RequiredKeys<O> = {
  [K in keyof O]: O[K] extends { kind: "optional" } ? never : K;
}[keyof O];

/** Object: every property required unless wrapped in s_optional. */
export function s_object<O extends Readonly<Record<string, Schema<unknown>>>>(properties: O): Schema<ObjectShape<O>> {
  const required = Object.entries(properties)
    .filter(([, v]) => (v as { kind?: string }).kind !== "optional")
    .map(([k]) => k);
  return {
    kind: "object",
    properties: { ...properties },
    required,
  } as unknown as Schema<ObjectShape<O>>;
}

export type ObjectShape<O extends Readonly<Record<string, Schema<unknown>>>> = {
  [K in keyof O as K extends RequiredKeys<O> ? K : never]: Infer<O[K]>;
} & {
  [K in keyof O as K extends RequiredKeys<O> ? never : K]?: Infer<O[K]>;
};

/** Bounded union: at most 4 members (IR limit, enforced by the compiler). */
export function s_union<A, B, C = never, D = never>(
  members: [Schema<A>, Schema<B>, ...([Schema<C>, Schema<D>] extends [never, never] ? [] : [Schema<C>?, Schema<D>?])],
): Schema<A | B | C | D> {
  return { kind: "union", members: members.filter((m) => m !== undefined) } as unknown as Schema<A | B | C | D>;
}

/** Transform names are declarative identifiers, never executed by Schema IR. */
const TRANSFORM_NAME_RE = /^[A-Za-z0-9_.:-]{1,64}$/;

/** Declarative transform: input/output schema pair + stable name. No callbacks. */
export function s_transform<I, O>(input: Schema<I>, output: Schema<O>, name: string): Schema<O> {
  if (!TRANSFORM_NAME_RE.test(name)) {
    throw new Error("s_transform: name must match [A-Za-z0-9_.:-]{1,64}");
  }
  return { kind: "transform", input, output, name } as unknown as Schema<O>;
}

/** Hard transport bound for file payloads (matches bounded-body limits). */
export const MAX_FILE_BYTES = 16 * 1024 * 1024;

export interface FileOpts {
  contentType?: string;
  maxBytes: number;
}

/** Bounded file metadata. The value stays an opaque byte payload owned by transport. */
export function s_file(opts: FileOpts): Schema<Uint8Array> {
  if (!Number.isSafeInteger(opts.maxBytes) || opts.maxBytes < 1 || opts.maxBytes > MAX_FILE_BYTES) {
    throw new Error(`s_file: maxBytes must be an integer in [1, ${MAX_FILE_BYTES}]`);
  }
  if (opts.contentType !== undefined && (opts.contentType.length === 0 || opts.contentType.length > 128)) {
    throw new Error("s_file: contentType must be 1..128 characters");
  }
  return { kind: "file", ...(opts.contentType !== undefined ? { contentType: opts.contentType } : {}), maxBytes: opts.maxBytes } as unknown as Schema<Uint8Array>;
}

export interface ProblemOpts {
  typeUri?: string;
  title: string;
  status: number;
  detail?: Schema<unknown>;
}

/** RFC 9457 problem metadata. `detail` is a declarative schema, not free-form. */
export function s_problem<T>(opts: ProblemOpts): Schema<T> {
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
  } as unknown as Schema<T>;
}

/** Closed fallback vocabulary (M25-001-B). Mirrors FALLBACK_REASONS in q-schema-runtime. */
export const FALLBACK_REASONS = ["unsupported-transform", "unrepresentable", "measured", "explicit"] as const;
export type FallbackReason = (typeof FALLBACK_REASONS)[number];

export type FallbackT = { readonly kind: "fallback"; readonly reason: FallbackReason; readonly inner?: Schema<unknown> };

/**
 * Explicit fallback marker (ADR-0009: no silent downgrade). `inner` is the
 * optional best-effort shape the native path validates against until the
 * generic codec path lands (M25-004-B).
 */
export function s_fallback<T = unknown>(reason: FallbackReason, inner?: Schema<T>): Schema<T> {
  if (!(FALLBACK_REASONS as readonly string[]).includes(reason)) {
    throw new Error(`s_fallback: reason must be one of ${FALLBACK_REASONS.join(", ")}`);
  }
  return { kind: "fallback", reason, ...(inner !== undefined ? { inner } : {}) } as unknown as Schema<T>;
}

/**
 * Compatibility markers (M25-001-B): feature tags derived from an IR graph.
 * Sorted, deduplicated; mirrors `features_of` in q-schema-runtime. Feeds the
 * pack schema manifest, which q-pack verifies fail-closed.
 */
export const FEATURE_TAGS = ["fallback", "file", "problem", "transform"] as const;
export type FeatureTag = (typeof FEATURE_TAGS)[number];

type IrLike = { kind?: string; [k: string]: unknown };

export function featuresOf(ir: Schema<unknown> | IrLike): FeatureTag[] {
  const seen = new Set<FeatureTag>();
  const walk = (node: IrLike): void => {
    switch (node.kind) {
      case "transform":
        seen.add("transform");
        walk(node.input as IrLike);
        walk(node.output as IrLike);
        break;
      case "file":
        seen.add("file");
        break;
      case "problem":
        seen.add("problem");
        if (node.detail !== undefined) walk(node.detail as IrLike);
        break;
      case "fallback":
        seen.add("fallback");
        if (node.inner !== undefined) walk(node.inner as IrLike);
        break;
      case "optional":
      case "nullable":
        walk(node.inner as IrLike);
        break;
      case "array":
        walk(node.items as IrLike);
        break;
      case "object":
        for (const p of Object.values(node.properties as Record<string, IrLike>)) walk(p);
        break;
      case "union":
        for (const m of node.members as IrLike[]) walk(m);
        break;
      default:
        break;
    }
  };
  walk(ir as IrLike);
  return [...seen].sort() as FeatureTag[];
}

/**
 * Canonical JSON form (M25-001-C, ADR-0023): every object's keys sorted
 * recursively (code-unit order), arrays keep order, integral floats already
 * stringify as integers in JS. Mirrors `q_schema_runtime::canonical_value` /
 * `canonical_json` byte-for-byte; one canonical string feeds hashes and
 * semantic diff on both sides of the boundary.
 */
export function canonicalValue<T = unknown>(v: unknown): T {
  if (Array.isArray(v)) return v.map(canonicalValue) as unknown as T;
  if (v && typeof v === "object") {
    const o = v as Record<string, unknown>;
    const out: Record<string, unknown> = {};
    for (const k of Object.keys(o).sort()) out[k] = canonicalValue(o[k]);
    return out as unknown as T;
  }
  return v as unknown as T;
}

export function canonicalJson(ir: unknown): string {
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
