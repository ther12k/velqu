/**
 * @velqu/treaty — Eden-inspired, type-safe client with:
 * - Route-ID dot-navigation: `api.users.get({ id })`
 * - Exact method narrowing: only declared HTTP method exists on the route client
 * - Exact body constraint: `post(body)` must match the contract's `body` schema type
 * - Strict 2xx data vs non-2xx error separation (200 is never in error union)
 * - Status narrowing: `if (r.error.status === 401)` types `r.error.problem`
 * - Dependency-free runtime (zero server/compiler imports)
 */

export type TreatyFetch = typeof fetch;

export interface RouteInfo {
  readonly path: string;
  readonly method: string;
}

export interface TreatyOptions {
  baseUrl: string;
  /** route-id -> {path, method}; the published contract's route table */
  contract: Readonly<Record<string, RouteInfo>>;
  fetchImpl?: TreatyFetch;
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

export interface RequestOptions<Q = Record<string, never>> {
  query?: Q;
  headers?: Record<string, string>;
  signal?: AbortSignal;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type AnyRouteContract = {
  readonly path: string;
  readonly method: string;
  readonly params?: unknown;
  readonly query?: unknown;
  readonly body?: unknown;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  readonly responses: Record<number, any>;
};

// ---------------------------------------------------------------- method narrowing

type QueryOf<R extends AnyRouteContract> = R extends { query: infer Q }
  ? Q extends Record<string, never>
    ? Record<string, never>
    : Q
  : Record<string, never>;

type BodyOf<R extends AnyRouteContract> = R extends { body: infer B }
  ? [B] extends [undefined]
    ? never
    : [B] extends [Record<string, never>]
      ? never
      : B
  : never;

type GetMethod<R extends AnyRouteContract> = {
  get(opts?: RequestOptions<QueryOf<R>>): Promise<TreatyResult<R["responses"]>>;
};

type PostMethod<R extends AnyRouteContract> = {
  post(
    ...args: [BodyOf<R>] extends [never]
      ? [opts?: RequestOptions<QueryOf<R>>]
      : [body: BodyOf<R>, opts?: RequestOptions<QueryOf<R>>]
  ): Promise<TreatyResult<R["responses"]>>;
};

type PutMethod<R extends AnyRouteContract> = {
  put(
    ...args: [BodyOf<R>] extends [never]
      ? [opts?: RequestOptions<QueryOf<R>>]
      : [body: BodyOf<R>, opts?: RequestOptions<QueryOf<R>>]
  ): Promise<TreatyResult<R["responses"]>>;
};

type PatchMethod<R extends AnyRouteContract> = {
  patch(
    ...args: [BodyOf<R>] extends [never]
      ? [opts?: RequestOptions<QueryOf<R>>]
      : [body: BodyOf<R>, opts?: RequestOptions<QueryOf<R>>]
  ): Promise<TreatyResult<R["responses"]>>;
};

type DeleteMethod<R extends AnyRouteContract> = {
  delete(opts?: RequestOptions<QueryOf<R>>): Promise<TreatyResult<R["responses"]>>;
};

type HeadMethod<R extends AnyRouteContract> = {
  head(opts?: RequestOptions<QueryOf<R>>): Promise<TreatyResult<R["responses"]>>;
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
  const doFetch = options.fetchImpl ?? fetch;
  const base = options.baseUrl.replace(/\/$/, "");
  return makeProxy(base, doFetch, options.contract, []) as TreatyClient<Api>;
}

function makeProxy(
  base: string,
  doFetch: typeof fetch,
  contract: Readonly<Record<string, RouteInfo>>,
  idSegments: string[],
): unknown {
  return new Proxy(function () {} as unknown as object, {
    get(_t, prop: string) {
      if (prop === "then") return undefined; // not a thenable
      const id = idSegments.join(".");
      const methodUpper = prop.toUpperCase();
      if (contract[id] && ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD"].includes(methodUpper)) {
        const info = contract[id];
        const routeUrl = `${base}/${info.path.replace(/^\//, "")}`;
        return (bodyOrOpts?: unknown, maybeOpts?: RequestOptions) => {
          if (methodUpper === "GET" || methodUpper === "HEAD" || methodUpper === "DELETE") {
            return request(doFetch, routeUrl, methodUpper, (bodyOrOpts ?? {}) as RequestOptions);
          }
          return request(doFetch, routeUrl, methodUpper, { ...(maybeOpts ?? {}), body: bodyOrOpts });
        };
      }
      const next = [...idSegments, prop];
      return makeProxy(base, doFetch, contract, next);
    },
    apply(_t, _thisArg, args: [Record<string, string | number>?]) {
      const id = idSegments.join(".");
      const info = contract[id];
      if (!info) {
        throw new Error(`treaty: unknown route id "${id}" (not in published contract)`);
      }
      const params = args[0] ?? {};
      const path = info.path
        .split("/")
        .filter(Boolean)
        .map((seg) => (seg.startsWith(":") ? String(params[seg.slice(1)] ?? "") : seg))
        .join("/");
      const routeUrl = `${base}/${path}`;
      return {
        get: (opts?: RequestOptions & { query?: Record<string, unknown> }) => request(doFetch, routeUrl, "GET", opts),
        post: (body?: unknown, opts?: RequestOptions) => request(doFetch, routeUrl, "POST", { ...opts, body }),
        put: (body?: unknown, opts?: RequestOptions) => request(doFetch, routeUrl, "PUT", { ...opts, body }),
        patch: (body?: unknown, opts?: RequestOptions) => request(doFetch, routeUrl, "PATCH", { ...opts, body }),
        delete: (opts?: RequestOptions) => request(doFetch, routeUrl, "DELETE", opts),
        head: (opts?: RequestOptions) => request(doFetch, routeUrl, "HEAD", opts),
      };
    },
  });
}

async function request<Resp extends Record<number, unknown>>(
  doFetch: typeof fetch,
  url: string,
  method: string,
  opts: RequestOptions & { body?: unknown } = {},
): Promise<TreatyResult<Resp>> {
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
  const text = await res.text();
  let parsed: unknown = undefined;
  try {
    parsed = text ? JSON.parse(text) : undefined;
  } catch {
    parsed = text;
  }
  if (res.ok) {
    return { data: parsed as SuccessData<Resp>, error: null } as const;
  }
  const err = { status: res.status as ExtractErrorStatuses<Resp>, problem: parsed };
  return { data: null, error: err as unknown as TreatyErrorFor<Resp> } as const;
}

function stripUndefined(q: Record<string, unknown>): Record<string, string> {
  const out: Record<string, string> = {};
  for (const [k, v] of Object.entries(q)) {
    if (v !== undefined) out[k] = String(v);
  }
  return out;
}
