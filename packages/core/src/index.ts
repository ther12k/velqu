/**
 * @velqu/core — static authoring primitives. Pure data constructors: no side
 * effects, no I/O, no server runtime code. The compiler reads the literal
 * arguments of these calls WITHOUT executing handlers or service factories
 * (COMP-002); `handle`/`check`/factories never run at build time.
 */

import type { Infer, Schema } from "@velqu/schema";

// ---------------------------------------------------------------- problems

export type ProblemId =
  | "validation"
  | "unauthorized"
  | "not-found"
  | "method"
  | "body"
  | "limit"
  | "timeout"
  | "internal";

export interface FieldIssue {
  readonly path: string;
  readonly code: string;
  readonly message: string;
}

/** M25-007-B: raw Response escape value (see unsupported-transformations §5). */
export interface RawResponseValue {
  readonly __velquRaw: true;
  readonly status: number;
  readonly headers: Record<string, string>;
  readonly body: unknown;
}

/** RFC 9457-compatible problem value (wire shape frozen in pack-format-v1). */
export interface ProblemValue<S extends number = number> {
  readonly __problem: true;
  readonly problem: ProblemId;
  readonly status: S;
  readonly detail?: string;
  readonly errors?: readonly FieldIssue[];
}

// ---------------------------------------------------------------- results

export interface OkValue<S extends number, T> {
  readonly __ok: true;
  readonly status: S;
  readonly value: T;
}

export class Status<S extends number> {
  constructor(readonly status: S) {}
  value<T>(value: T): OkValue<S, T> {
    return { __ok: true, status: this.status, value };
  }
  /** M25-007-B: raw Response escape hatch — crosses status/headers/body
   * AS-IS, bypassing declared response validation. Only routes declaring
   * the `raw-response` capability may return it. */
  raw(body: unknown, opts: { status?: S; headers?: Record<string, string> } = {}): RawResponseValue {
    return { __velquRaw: true, status: opts.status ?? this.status, headers: opts.headers ?? {}, body };
  }
  problem(
    problem: ProblemId,
    opts: { detail?: string; errors?: readonly FieldIssue[]; fields?: Readonly<Record<string, unknown>> } = {},
  ): ProblemValue<S> {
    // M25-006-A: RFC 9457 extension members — `fields` spreads onto the
    // problem object so custom members survive end-to-end.
    const { fields, ...rest } = opts;
    return { __problem: true, problem, status: this.status, ...rest, ...(fields ?? {}) };
  }
}

/** `status(201).value(user)` / `status(404).problem("not-found")` */
export function status<S extends number>(code: S): Status<S> {
  return new Status(code);
}

// ---------------------------------------------------------------- context types (handler-facing)

export interface TimerCapability {
  /** resolves after `ms` milliseconds (native operation) */
  delay(ms: number): Promise<number>;
}

export interface HandlerCtx<P, Q, B, Sess> {
  params: P;
  query: Q;
  headers: Readonly<Record<string, string>>;
  /** present when a policy provided a session (SCHEMA-004) */
  session: Sess;
  /** pre-validated body (native strategy); absent when the route declares none */
  body: B;
  /** lazy body accessors (js body strategy) */
  json(): unknown;
  text(): string;
  bytes(): Uint8Array;
  native: { timer: TimerCapability };
}

/** Policy check sees the lazy request (header access before validation). */
export interface PolicyRequest {
  readonly headers: Readonly<Record<string, string>>;
}

// ---------------------------------------------------------------- route

export type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

export interface RouteDecl<
  Id extends string,
  M extends HttpMethod,
  P = unknown,
  Q = unknown,
  B = unknown,
  Resp extends Record<number, unknown> = Record<number, unknown>,
  Sess = undefined,
> {
  readonly __route: true;
  readonly id: Id;
  readonly method: M;
  readonly path: string;
  readonly params?: Schema<P>;
  readonly query?: Schema<Q>;
  readonly body?: Schema<B>;
  readonly policy?: PolicyDecl<Sess>;
  /** status -> body schema; keys are the ONLY statuses this route may return */
  readonly response: { [K in keyof Resp & number]: Schema<Resp[K]> };
  /** method syntax keeps handler assignability bivariant for module composition */
  handle(ctx: HandlerCtx<P, Q, B, Sess>): Promise<HandlerResult<Resp>> | HandlerResult<Resp>;
}

/** What a handler may return: a declared-status value or a typed problem. */
export type HandlerResult<Resp extends Record<number, unknown> = Record<number, unknown>> =
  | { [K in keyof Resp & number]?: Resp[K] | OkValue<K, Resp[K]> }
  | ProblemValue<number>
  | Resp[keyof Resp];

// Input type extraction used by Treaty: path params of `P`.
export type RouteParams<R> = R extends { readonly params?: Schema<infer P> } ? P : Record<string, never>;
export type RouteQuery<R> = R extends { readonly query?: Schema<infer Q> } ? Q : Record<string, never>;
export type RouteBody<R> = R extends { readonly body?: Schema<infer B> } ? B : undefined;
export type RouteResponses<R> = R extends { readonly response: infer Resp } ? Resp : never;
export type RoutePath<R> = R extends { readonly path: infer Pa extends string } ? Pa : never;
export type RouteMethod<R> = R extends { readonly method: infer M extends HttpMethod } ? M : never;
export type RouteSession<R> = R extends { readonly policy?: PolicyDecl<infer S> } ? S : undefined;

/** Route constructor. `def` is read statically by the compiler. */
export function route<
  Id extends string,
  M extends HttpMethod,
  P,
  Q,
  B,
  Resp extends Record<number, unknown>,
  Sess,
>(def: {
  id: Id;
  method: M;
  path: string;
  params?: Schema<P>;
  query?: Schema<Q>;
  body?: Schema<B>;
  policy?: PolicyDecl<Sess>;
  response: { [K in keyof Resp & number]: Schema<Resp[K]> };
  handle: (ctx: HandlerCtx<P, Q, B, Sess>) => Promise<HandlerResult<Resp>> | HandlerResult<Resp>;
}): RouteDecl<Id, M, P, Q, B, Resp, Sess> {
  return { __route: true, ...def };
}

// ---------------------------------------------------------------- policy

export interface PolicyDecl<Sess> {
  readonly __policy: true;
  readonly id: string;
  readonly header: string;
  /** statuses this policy can produce; flow into the route response union */
  readonly declares: { readonly [S in number]: ProblemId };
  readonly provides: "session";
  check(req: PolicyRequest): Promise<{ session: Sess } | ProblemValue<number>>;
}

export function definePolicy<Sess>(def: {
  id: string;
  header: string;
  declares: { readonly [S in number]: ProblemId };
  provides: "session";
  check: (req: PolicyRequest) => Promise<{ session: Sess } | ProblemValue<number>>;
}): PolicyDecl<Sess> {
  return { __policy: true, ...def };
}

// ---------------------------------------------------------------- service

/** Lazy service: the factory runs on first `resolve()` at runtime — never at
 *  compile time and never during unrelated cold starts (G-005/C5). */
export interface ServiceDecl<T> {
  readonly __service: true;
  readonly id: string;
  readonly factory: () => T;
}

export function defineService<T>(id: string, factory: () => T): ServiceDecl<T> {
  return { __service: true, id, factory };
}

// ---------------------------------------------------------------- module & app

export type AnyRoute = RouteDecl<string, HttpMethod, unknown, unknown, unknown, Record<number, unknown>, unknown>;

export interface ModuleDecl<Id extends string, Routes extends readonly AnyRoute[]> {
  readonly __module: true;
  readonly id: Id;
  readonly routes: Routes;
}

export function defineModule<Id extends string, Routes extends readonly AnyRoute[]>(
  def: { id: Id; routes: Routes & readonly AnyRoute[] },
): ModuleDecl<Id, Routes> {
  return { __module: true, ...def };
}

export type AnyModule = ModuleDecl<string, readonly AnyRoute[]>;

export interface AppDecl<Id extends string, Modules extends readonly AnyModule[]> {
  readonly __app: true;
  readonly id: Id;
  readonly modules: Modules;
}

export function defineApp<Id extends string, Modules extends readonly AnyModule[]>(
  def: { id: Id; modules: Modules & readonly AnyModule[] },
): AppDecl<Id, Modules> {
  return { __app: true, ...def };
}

/** Flatten an app's routes into a tuple union for contract extraction. */
export type AppRoutes<A> = A extends { readonly modules: readonly (infer Ms)[] }
  ? Ms extends AnyModule
    ? Ms["routes"][number]
    : never
  : never;
