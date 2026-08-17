/**
 * @q/schema — Schema IR v1 builders.
 *
 * Each builder returns a value whose RUNTIME representation is exactly the
 * Schema IR JSON node consumed by the Rust runtime (docs/specs/pack-format-v1.md)
 * and whose TYPE carries the inferred TypeScript type. One declaration drives
 * types, runtime validation, Treaty inputs, and OpenAPI (SCHEMA-001).
 */

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
  | { readonly kind: "union"; readonly members: readonly Schema<unknown>[]; readonly __t?: T };

export type LiteralValue = string | number | boolean;

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
