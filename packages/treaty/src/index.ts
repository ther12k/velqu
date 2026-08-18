/**
 * @q/treaty — Treaty client: object-like route navigation, typed inputs,
 * status-narrowed non-throwing results (TRT-001..003).
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

// ---------------------------------------------------------------- result types

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
export type TreatyError<S extends number = number, P = unknown> = HttpError<S, P> | NetworkError | AbortError;

export type TreatyResult<Resp extends Record<number, unknown>> =
  | { readonly data: Resp[keyof Resp & number]; readonly error: null }
  | { readonly data: null; readonly error: TreatyErrorFor<Resp> };

/** error narrowing: `if (r.error.status === 401)` types problem as Resp[401] */
export type TreatyErrorFor<Resp extends Record<number, unknown>> =
  | NetworkError
  | AbortError
  | { [S in keyof Resp & number]: { readonly status: S; readonly problem: Resp[S] } }[keyof Resp & number];

// ---------------------------------------------------------------- path helpers (type level)

type PathSegments<P extends string> =
  P extends `/${infer Head}/${infer Tail}`
    ? [Head, ...PathSegments<`/${Tail}`>]
    : P extends `/${infer Last}`
      ? [Last]
      : [];

/** Param names from segments (":id" -> "id"). */
export type ParamNames<P extends string> = PathSegments<P>[number] extends `:${infer N}` ? N : never;

export interface RequestOptions<Q = Record<string, never>> {
  query?: Q;
  headers?: Record<string, string>;
  signal?: AbortSignal;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type AnyRouteContract = {
  path: string;
  method: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  responses: Record<number, any>;
};

// ---------------------------------------------------------------- client

/**
 * Build a typed client for a published contract.
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function treaty<Api extends Record<string, AnyRouteContract>>(options: TreatyOptions): any {
  const doFetch = options.fetchImpl ?? fetch;
  const base = options.baseUrl.replace(/\/$/, "");
  return makeProxy(base, doFetch, options.contract, []);
}

function makeProxy(
  base: string,
  doFetch: typeof fetch,
  contract: Readonly<Record<string, RouteInfo>>,
  idSegments: string[],
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
): any {
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
    return { data: parsed as Resp[keyof Resp & number], error: null } as const;
  }
  const err: HttpError<number, unknown> = { status: res.status, problem: parsed };
  return { data: null, error: err } as const;
}

function stripUndefined(q: Record<string, unknown>): Record<string, string> {
  const out: Record<string, string> = {};
  for (const [k, v] of Object.entries(q)) {
    if (v !== undefined) out[k] = String(v);
  }
  return out;
}

// ---------------------------------------------------------------- type-level client

type MethodSuite<R extends AnyRouteContract> = {
  get(opts?: RequestOptions & { query?: QueryOf<R> }): Promise<TreatyResult<R["responses"]>>;
  post<Body>(body: Body, opts?: RequestOptions): Promise<TreatyResult<R["responses"]>>;
  put<Body>(body: Body, opts?: RequestOptions): Promise<TreatyResult<R["responses"]>>;
  patch<Body>(body: Body, opts?: RequestOptions): Promise<TreatyResult<R["responses"]>>;
  delete(opts?: RequestOptions): Promise<TreatyResult<R["responses"]>>;
};

type QueryOf<R extends AnyRouteContract> = R extends { query: infer Q }
  ? Q extends Record<string, never>
    ? Record<string, never>
    : Q
  : Record<string, never>;

/**
 * TreatyClient<Api>: each route id's dot segments become nested callable
 * chains; the final segment is called with path params and returns the
 * method suite.
 */
export type TreatyClient<Api extends Record<string, AnyRouteContract>> = {
  [K in keyof Api & string]: NestedChain<K, Api[K]>;
};

type NestedChain<K extends string, R extends AnyRouteContract> =
  K extends `${infer Head}.${infer Rest}`
    ? { readonly [P in Head]: NestedChain<Rest, R> }
    : ((params?: ParamsForPath<R>) => MethodSuite<R>) & MethodSuite<R>;

type ParamsForPath<R extends AnyRouteContract> = R extends { path: infer P }
  ? P extends string
    ? { [N in ParamNames<P>]: string | number }
    : Record<string, never>
  : Record<string, never>;
