/**
 * @velqu/treaty — Eden-inspired, type-safe client with:
 * - Route-ID dot-navigation: `api.users.get({ id })`
 * - Exact method narrowing: only declared HTTP method exists on the route client
 * - Exact body constraint: `post(body)` must match the contract's `body` schema type
 * - Strict 2xx data vs non-2xx error separation (200 is never in error union)
 * - Status narrowing: `if (r.error.status === 401)` types `r.error.problem`
 * - Dependency-free runtime (zero server/compiler imports)
 */

/** Portable fetch shape; deliberately excludes Bun-only helpers such as preconnect. */
export type TreatyFetch = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;

export interface RouteInfo {
  readonly path: string;
  readonly method: string;
}

// ---------------------------------------------------------------- direct dispatch (unit-local mode)

/** One invocation routed to an in-process dispatcher instead of HTTP. */
export interface DispatchRequest {
  /** full route id, e.g. "greetings.create" */
  routeId: string;
  /** UPPERCASE HTTP method */
  method: string;
  /** path with params substituted (no baseUrl prefix) */
  path: string;
  query?: Record<string, unknown>;
  headers?: Record<string, string>;
  body?: unknown;
}

/** What a dispatcher hands back: the same facts an HTTP response carries. */
export type DispatchOutcome =
  | { kind: "response"; status: number; bodyText: string }
  | { kind: "network"; message: string }
  | { kind: "abort" };

export type DispatchImpl = (req: DispatchRequest) => Promise<DispatchOutcome>;

export interface TreatyOptions {
  baseUrl: string;
  /** route-id -> {path, method}; the published contract's route table */
  contract: Readonly<Record<string, RouteInfo>>;
  fetchImpl?: TreatyFetch;
  /**
   * Direct in-process dispatcher (unit-local mode). When set, NO HTTP
   * transport is used — every invocation goes through this function and
   * results are status-split by the SAME contract machinery as the
   * remote modes (M4A-004-A). Dispatcher throws are contract errors and
   * propagate (fail loud) instead of becoming network errors.
   */
  dispatchImpl?: DispatchImpl;
}

// ---------------------------------------------------------------- status & response splitting

export type SuccessStatus =
  | 200
  | 201
  | 202
  | 203
  | 204
  | 205
  | 206
  | 207
  | 208
  | 226;

export type ExtractSuccessStatuses<Resp> = Extract<keyof Resp & number, SuccessStatus>;
export type ExtractErrorStatuses<Resp> = Exclude<keyof Resp & number, SuccessStatus>;

export type SuccessData<Resp extends Record<number, unknown>> =
  ExtractSuccessStatuses<Resp> extends never
    ? unknown
    : Resp[ExtractSuccessStatuses<Resp>];

// ---------------------------------------------------------------- error types

export interface HttpError<S extends number, P = unknown> {
  readonly status: S;
  readonly problem: P;
}

export interface NetworkError {
  readonly status: 0;
  readonly kind: "network";
  readonly message: string;
}

export interface AbortError {
  readonly status: 0;
  readonly kind: "abort";
}

export type TreatyErrorFor<Resp extends Record<number, unknown>> =
  | NetworkError
  | AbortError
  | (ExtractErrorStatuses<Resp> extends never
      ? never
      : {
          [S in ExtractErrorStatuses<Resp>]: {
            readonly status: S;
            readonly problem: Resp[S];
          };
        }[ExtractErrorStatuses<Resp>]);

// ---------------------------------------------------------------- result

export type TreatyResult<Resp extends Record<number, unknown>> =
  | { readonly data: SuccessData<Resp>; readonly error: null }
  | { readonly data: null; readonly error: TreatyErrorFor<Resp> };

// ---------------------------------------------------------------- path & param extraction

type PathSegments<P extends string> =
  P extends `/${infer Head}/${infer Tail}`
    ? [Head, ...PathSegments<`/${Tail}`>]
    : P extends `/${infer Last}`
      ? [Last]
      : [];

type ExtractParam<Segment> = Segment extends `:${infer N}` ? N : never;
export type ParamNames<P extends string> = ExtractParam<PathSegments<P>[number]>;

/** Required keys in an object type (optional query/header fields stay optional). */
type RequiredKeys<T> = T extends object
  ? { [K in keyof T]-?: {} extends Pick<T, K> ? never : K }[keyof T]
  : never;

/** Exact caller-selected query/header options for a route. */
export type RequestOptions<
  Q = Record<string, never>,
  H = Record<string, string>,
> = {
  signal?: AbortSignal;
} & ([RequiredKeys<Q>] extends [never]
  ? { query?: Q }
  : { query: Q })
  & ([RequiredKeys<H>] extends [never]
    ? { headers?: H }
    : { headers: H });

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type AnyRouteContract = {
  readonly path: string;
  readonly method: string;
  readonly params?: unknown;
  readonly query?: unknown;
  readonly body?: unknown;
  readonly headers?: unknown;
  readonly responses: Record<number, unknown>;
};

// ---------------------------------------------------------------- method narrowing

type QueryOf<R extends AnyRouteContract> = R extends { query: infer Q }
  ? Q extends Record<string, never>
    ? Record<string, never>
    : Q
  : Record<string, never>;

/** Header input is exact when a generated contract declares a header schema. */
type HeadersOf<R extends AnyRouteContract> = R extends { headers: infer H }
  ? H extends Record<string, never>
    ? Record<string, never>
    : H
  : Record<string, string>;

type BodyOf<R extends AnyRouteContract> = R extends { body: infer B }
  ? [B] extends [undefined]
    ? never
    : [B] extends [Record<string, never>]
      ? never
      : B
  : never;

type GetMethod<R extends AnyRouteContract> = {
  get(opts?: RequestOptions<QueryOf<R>, HeadersOf<R>>): Promise<TreatyResult<R["responses"]>>;
};

type PostMethod<R extends AnyRouteContract> = {
  post(
    ...args: [BodyOf<R>] extends [never]
      ? [opts?: RequestOptions<QueryOf<R>, HeadersOf<R>>]
      : [body: BodyOf<R>, opts?: RequestOptions<QueryOf<R>, HeadersOf<R>>]
  ): Promise<TreatyResult<R["responses"]>>;
};

type PutMethod<R extends AnyRouteContract> = {
  put(
    ...args: [BodyOf<R>] extends [never]
      ? [opts?: RequestOptions<QueryOf<R>, HeadersOf<R>>]
      : [body: BodyOf<R>, opts?: RequestOptions<QueryOf<R>, HeadersOf<R>>]
  ): Promise<TreatyResult<R["responses"]>>;
};

type PatchMethod<R extends AnyRouteContract> = {
  patch(
    ...args: [BodyOf<R>] extends [never]
      ? [opts?: RequestOptions<QueryOf<R>, HeadersOf<R>>]
      : [body: BodyOf<R>, opts?: RequestOptions<QueryOf<R>, HeadersOf<R>>]
  ): Promise<TreatyResult<R["responses"]>>;
};

type DeleteMethod<R extends AnyRouteContract> = {
  delete(opts?: RequestOptions<QueryOf<R>, HeadersOf<R>>): Promise<TreatyResult<R["responses"]>>;
};

type HeadMethod<R extends AnyRouteContract> = {
  head(opts?: RequestOptions<QueryOf<R>, HeadersOf<R>>): Promise<TreatyResult<R["responses"]>>;
};

export type MethodSuiteFor<R extends AnyRouteContract> =
  (Uppercase<R["method"]> extends "GET" ? GetMethod<R> : unknown) &
  (Uppercase<R["method"]> extends "POST" ? PostMethod<R> : unknown) &
  (Uppercase<R["method"]> extends "PUT" ? PutMethod<R> : unknown) &
  (Uppercase<R["method"]> extends "PATCH" ? PatchMethod<R> : unknown) &
  (Uppercase<R["method"]> extends "DELETE" ? DeleteMethod<R> : unknown) &
  (Uppercase<R["method"]> extends "HEAD" ? HeadMethod<R> : unknown);

// ---------------------------------------------------------------- client shape

type ParamsForPath<R extends AnyRouteContract> = R extends { path: infer P }
  ? P extends string
    ? [ParamNames<P>] extends [never]
      ? Record<string, never>
      : { [N in ParamNames<P>]: string | number }
    : Record<string, never>
  : Record<string, never>;

export type CallableOrDirect<R extends AnyRouteContract> =
  [ParamNames<R["path"]>] extends [never]
    ? ((params?: Record<string, never>) => MethodSuiteFor<R>) & MethodSuiteFor<R>
    : (params: ParamsForPath<R>) => MethodSuiteFor<R>;

type FirstSegment<K extends string> = K extends `${infer Head}.${string}` ? Head : K;

type SubApi<Api extends Record<string, AnyRouteContract>, Prefix extends string> = {
  [K in keyof Api as K extends `${Prefix}.${infer Rest}` ? Rest : never]: Api[K];
};

export type TreatyClient<Api extends Record<string, AnyRouteContract>> = {
  [Head in FirstSegment<keyof Api & string>]:
    [keyof SubApi<Api, Head>] extends [never]
      ? (Head extends keyof Api ? CallableOrDirect<Api[Head]> : never)
      : TreatyClient<SubApi<Api, Head>> & (Head extends keyof Api ? CallableOrDirect<Api[Head]> : unknown);
};

// ---------------------------------------------------------------- implementation

export function treaty<Api extends Record<string, AnyRouteContract>>(
  options: TreatyOptions,
): TreatyClient<Api> {
  const doFetch: TreatyFetch = options.fetchImpl ?? fetch;
  const base = options.baseUrl.replace(/\/$/, "");
  const dispatch = options.dispatchImpl ?? null;
  return makeProxy(base, doFetch, options.contract, [], dispatch) as TreatyClient<Api>;
}

function makeProxy(
  base: string,
  doFetch: TreatyFetch,
  contract: Readonly<Record<string, RouteInfo>>,
  idSegments: string[],
  dispatch: DispatchImpl | null,
): unknown {
  return new Proxy(function () {} as unknown as object, {
    get(_t, prop: string) {
      if (prop === "then") return undefined; // not a thenable
      const id = idSegments.join(".");
      const methodUpper = prop.toUpperCase();
      if (contract[id]) {
        const info = contract[id];
        const expectedMethod = info.method.toUpperCase();
        if (methodUpper === expectedMethod) {
          const pathPart = info.path.replace(/^\//, "");
          const routeUrl = `${base}/${pathPart}`;
          return (bodyOrOpts?: unknown, maybeOpts?: RequestOptions) => {
            const opts =
              methodUpper === "GET" || methodUpper === "HEAD" || methodUpper === "DELETE"
                ? ((bodyOrOpts ?? {}) as RequestOptions)
                : { ...(maybeOpts ?? {}), body: bodyOrOpts };
            return request(doFetch, dispatch, id, routeUrl, pathPart, methodUpper, opts);
          };
        }
        if (["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD"].includes(methodUpper)) {
          return () => {
            throw new Error(
              `treaty: method "${methodUpper}" is not allowed on route "${id}" (declared method: "${info.method}")`,
            );
          };
        }
      }
      const next = [...idSegments, prop];
      return makeProxy(base, doFetch, contract, next, dispatch);
    },
    apply(_t, _thisArg, args: [Record<string, string | number>?]) {
      const id = idSegments.join(".");
      const info = contract[id];
      if (!info) {
        throw new Error(`treaty: unknown route id "${id}" (not in published contract)`);
      }
      const params = args[0] ?? {};
      const pathSegs = info.path.split("/").filter(Boolean);
      for (const seg of pathSegs) {
        if (seg.startsWith(":")) {
          const paramName = seg.slice(1);
          const val = params[paramName];
          if (val === undefined || val === null || val === "") {
            throw new Error(
              `treaty: missing required path parameter "${paramName}" for route "${id}"`,
            );
          }
        }
      }
      const path = pathSegs
        .map((seg) => (seg.startsWith(":") ? encodeURIComponent(String(params[seg.slice(1)])) : seg))
        .join("/");
      const routeUrl = `${base}/${path}`;
      const declaredMethod = info.method.toLowerCase();
      const invoke = (method: string, opts: RequestOptions & { body?: unknown }) =>
        request(doFetch, dispatch, id, routeUrl, path, method, opts);
      const methodMap: Record<string, Function> = {
        get: (opts?: RequestOptions) => invoke("GET", opts ?? {}),
        post: (body?: unknown, opts?: RequestOptions) => invoke("POST", { ...opts, body }),
        put: (body?: unknown, opts?: RequestOptions) => invoke("PUT", { ...opts, body }),
        patch: (body?: unknown, opts?: RequestOptions) => invoke("PATCH", { ...opts, body }),
        delete: (opts?: RequestOptions) => invoke("DELETE", opts ?? {}),
        head: (opts?: RequestOptions) => invoke("HEAD", opts ?? {}),
      };
      return new Proxy({} as any, {
        get(_target, prop: string) {
          const m = prop.toLowerCase();
          if (m === declaredMethod && methodMap[m]) {
            return methodMap[m];
          }
          if (["get", "post", "put", "patch", "delete", "head"].includes(m)) {
            return () => {
              throw new Error(
                `treaty: method "${prop.toUpperCase()}" is not allowed on route "${id}" (declared method: "${info.method}")`,
              );
            };
          }
          return undefined;
        },
      });
    },
  });
}

async function request<Resp extends Record<number, unknown>>(
  doFetch: TreatyFetch,
  dispatch: DispatchImpl | null,
  routeId: string,
  url: string,
  pathPart: string,
  method: string,
  opts: RequestOptions & { body?: unknown } = {},
): Promise<TreatyResult<Resp>> {
  let status: number;
  let text: string;
  if (dispatch) {
    // Direct in-process dispatcher: no HTTP. Dispatcher throws are
    // contract errors and propagate — they never masquerade as network
    // failures (fail loud).
    const outcome = await dispatch({
      routeId,
      method,
      path: `/${pathPart.replace(/^\//, "")}`,
      query: opts.query,
      headers: opts.headers,
      body: opts.body,
    });
    if (outcome.kind === "abort") {
      return { data: null, error: { status: 0, kind: "abort" } } as const;
    }
    if (outcome.kind === "network") {
      return { data: null, error: { status: 0, kind: "network", message: outcome.message } } as const;
    }
    status = outcome.status;
    text = outcome.bodyText;
  } else {
    const qs = opts.query ? `?${new URLSearchParams(stripUndefined(opts.query))}` : "";
    let res: Response;
    try {
      res = await doFetch(url + qs, {
        method,
        headers: {
          ...(opts.body !== undefined && opts.body !== null ? { "content-type": "application/json" } : {}),
          ...(opts.headers ?? {}),
        },
        body: opts.body !== undefined && opts.body !== null ? JSON.stringify(opts.body) : undefined,
        signal: opts.signal,
      });
    } catch (e) {
      if (e instanceof Error && e.name === "AbortError") {
        return { data: null, error: { status: 0, kind: "abort" } } as const;
      }
      return {
        data: null,
        error: { status: 0, kind: "network", message: e instanceof Error ? e.message : "network failure" },
      } as const;
    }
    status = res.status;
    text = await res.text();
  }
  let parsed: unknown = undefined;
  try {
    parsed = text ? JSON.parse(text) : undefined;
  } catch {
    parsed = text;
  }
  if (status >= 200 && status <= 299) {
    return { data: parsed as SuccessData<Resp>, error: null } as const;
  }
  const err = { status: status as ExtractErrorStatuses<Resp>, problem: parsed };
  return { data: null, error: err as unknown as TreatyErrorFor<Resp> } as const;
}

function stripUndefined(q: Record<string, unknown>): Record<string, string> {
  const out: Record<string, string> = {};
  for (const [k, v] of Object.entries(q)) {
    if (v !== undefined) out[k] = String(v);
  }
  return out;
}
