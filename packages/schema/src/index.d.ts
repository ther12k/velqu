/**
 * @q/schema — Schema IR v1 builders.
 *
 * Each builder returns a value whose RUNTIME representation is exactly the
 * Schema IR JSON node consumed by the Rust runtime (docs/specs/pack-format-v1.md)
 * and whose TYPE carries the inferred TypeScript type. One declaration drives
 * types, runtime validation, Treaty inputs, and OpenAPI (SCHEMA-001).
 */
export type Schema<T> = {
    readonly kind: "string";
    minLength?: number;
    maxLength?: number;
    pattern?: string;
    format?: string;
    readonly __t?: T;
} | {
    readonly kind: "integer";
    minimum?: number;
    maximum?: number;
    readonly __t?: T;
} | {
    readonly kind: "number";
    minimum?: number;
    maximum?: number;
    readonly __t?: T;
} | {
    readonly kind: "boolean";
    readonly __t?: T;
} | {
    readonly kind: "literal";
    readonly value: LiteralValue;
    readonly __t?: T;
} | {
    readonly kind: "enum";
    readonly values: LiteralValue[];
    readonly __t?: T;
} | {
    readonly kind: "optional";
    readonly inner: Schema<unknown>;
    readonly default?: unknown;
    readonly __t?: T;
} | {
    readonly kind: "nullable";
    readonly inner: Schema<unknown>;
    readonly __t?: T;
} | {
    readonly kind: "array";
    readonly items: Schema<unknown>;
    minItems?: number;
    maxItems?: number;
    readonly __t?: T;
} | {
    readonly kind: "object";
    readonly properties: Readonly<Record<string, Schema<unknown>>>;
    readonly required: readonly string[];
    readonly __t?: T;
} | {
    readonly kind: "union";
    readonly members: readonly Schema<unknown>[];
    readonly __t?: T;
};
export type LiteralValue = string | number | boolean;
/** Infer the TypeScript type carried by a schema. */
export type Infer<S> = S extends {
    __t?: infer T;
} ? T : never;
type StringType = string;
export interface StringOpts {
    minLength?: number;
    maxLength?: number;
    pattern?: string;
    format?: "email" | "uuid";
}
export declare function s_string(opts?: StringOpts): Schema<StringType>;
export interface IntOpts {
    minimum?: number;
    maximum?: number;
}
export declare function s_integer(opts?: IntOpts): Schema<number>;
export declare function s_number(opts?: IntOpts): Schema<number>;
export declare function s_boolean(): Schema<boolean>;
export declare function s_literal<T extends LiteralValue>(value: T): Schema<T>;
export declare function s_enum<T extends LiteralValue>(values: readonly T[]): Schema<T>;
export declare function s_optional<T>(inner: Schema<T>, opts?: {
    default?: T;
}): Schema<T | undefined>;
export declare function s_nullable<T>(inner: Schema<T>): Schema<T | null>;
export declare function s_array<T>(items: Schema<T>, opts?: {
    minItems?: number;
    maxItems?: number;
}): Schema<T[]>;
type RequiredKeys<O> = {
    [K in keyof O]: O[K] extends {
        kind: "optional";
    } ? never : K;
}[keyof O];
/** Object: every property required unless wrapped in s_optional. */
export declare function s_object<O extends Readonly<Record<string, Schema<unknown>>>>(properties: O): Schema<ObjectShape<O>>;
export type ObjectShape<O extends Readonly<Record<string, Schema<unknown>>>> = {
    [K in keyof O as K extends RequiredKeys<O> ? K : never]: Infer<O[K]>;
} & {
    [K in keyof O as K extends RequiredKeys<O> ? never : K]?: Infer<O[K]>;
};
/** Bounded union: at most 4 members (IR limit, enforced by the compiler). */
export declare function s_union<A, B, C = never, D = never>(members: [Schema<A>, Schema<B>, ...([Schema<C>, Schema<D>] extends [never, never] ? [] : [Schema<C>?, Schema<D>?])]): Schema<A | B | C | D>;
/** Convenience namespace so `s.string()` reads naturally. */
export declare const s: {
    string: typeof s_string;
    integer: typeof s_integer;
    number: typeof s_number;
    boolean: typeof s_boolean;
    literal: typeof s_literal;
    enum: typeof s_enum;
    optional: typeof s_optional;
    nullable: typeof s_nullable;
    array: typeof s_array;
    object: typeof s_object;
    union: typeof s_union;
};
export {};
