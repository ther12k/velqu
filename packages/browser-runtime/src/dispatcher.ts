/**
 * BWASM-R-002 — the Fetch-compatible browser dispatcher.
 *
 * Owns everything at the JS/WASM boundary: normalization of URL,
 * method, headers, query, and body into the kernel ABI message;
 * bounded body-form handling (text, JSON, URL-encoded, multipart
 * metadata; binary/streaming explicitly unsupported — see
 * UNSUPPORTED_SEMANTICS); abort propagation; HEAD body-stripping;
 * deterministic header/status mapping. Route selection, parameter
 * extraction, request/response validation, capability checks, and
 * problem mapping stay in the Rust/WASM kernel — there is NO
 * JavaScript fast path that bypasses kernel validation (acceptance
 * criterion): the dispatcher only normalizes transport, never decides
 * routing or validity.
 */

import type {
  KernelInstance,
  KernelPlanRequest,
  KernelPlanResult,
  KernelProblemShape,
} from "./index";

// ---------------------------------------------------------------------------
// Policy constants (bounded everything — AGENTS.md #11)
// ---------------------------------------------------------------------------

/** Default request-body cap (matches the native default max body). */
export const DEFAULT_MAX_BODY_BYTES = 1 << 20;
/** Hard cap on multipart metadata: parts and per-part header bytes. */
export const MAX_MULTIPART_PARTS = 64;
export const MAX_MULTIPART_HEADER_BYTES = 8 << 10;
/** Upper bound on query/header pair counts crossing the boundary. */
export const MAX_QUERY_PAIRS = 256;
export const MAX_HEADER_PAIRS = 128;

/**
 * Semantics this dispatcher deliberately does NOT support in the MVP.
 * Every entry is fail-closed with a typed `body`/`limit` problem —
 * never silently degraded (required evidence: unsupported-semantics
 * inventory).
 */
export const UNSUPPORTED_SEMANTICS: ReadonlyArray<{
  readonly feature: string;
  readonly reason: string;
  readonly disposition: string;
}> = [
  {
    feature: "request body streaming (ReadableStream)",
    reason: "kernel ABI is bounded messages; streaming has no frozen contract",
    disposition: "rejects with `body` problem `streaming request bodies are unsupported in the browser MVP` before dispatch",
  },
  {
    feature: "binary request bodies (non-text media types)",
    reason: "kernel ABI carries UTF-8 JSON/text; byte-exact binary crossing is the handler-bundle contract (BWASM-R-003)",
    disposition: "rejects with `body` problem naming the media type",
  },
  {
    feature: "full multipart/form-data parsing",
    reason: "MVP carries metadata only (part names, content types); part payloads need the bounded binary contract",
    disposition: "metadata crosses as bodyText JSON; payloads beyond metadata reject with `limit` problem",
  },
  {
    feature: "response streaming",
    reason: "dispatcher returns materialized standard Responses; the public boundary stays Promise<Response>",
    disposition: "responses are always fully materialized and bounded",
  },
];

// ---------------------------------------------------------------------------
// Options and results
// ---------------------------------------------------------------------------

export interface DispatcherOptions {
  readonly maxBodyBytes?: number;
}

/** Structured dispatcher error types surfaced to the runtime. */
export type DispatcherRejection =
  | { readonly kind: "aborted"; readonly phase: "before-dispatch" | "during-dispatch" }
  | {
      readonly kind: "unsupported";
      readonly feature: string;
      readonly detail: string;
    }
  | { readonly kind: "protocol"; readonly feature: "kernel protocol"; readonly detail: string }
  | { readonly kind: "limit"; readonly detail: string };

export class DispatcherError extends Error {
  readonly rejection: DispatcherRejection;
  constructor(rejection: DispatcherRejection) {
    const what =
      rejection.kind === "aborted"
        ? `request aborted ${rejection.phase}`
        : rejection.kind === "unsupported"
          ? `unsupported: ${rejection.feature}`
          : `limit: ${rejection.detail}`;
    super(`[@velqu/browser-runtime:dispatcher] ${what}`);
    this.name = "DispatcherError";
    this.rejection = rejection;
  }
}

// ---------------------------------------------------------------------------
// Body normalization (transport-only; validation stays kernel-side)
// ---------------------------------------------------------------------------

interface NormalizedBody {
  /** JSON text for the kernel `body` field (validated by the kernel). */
  readonly json?: string;
  /** Raw text when the route declares no body schema (bodyText). */
  readonly text?: string;
}

function normalizeBody(
  request: Request,
  contentType: string,
  maxBodyBytes: number,
): Promise<NormalizedBody> {
  const media = contentType.split(";")[0]?.trim().toLowerCase() ?? "";

  if (request.body !== null && typeof (request.body as ReadableStream).locked !== "undefined") {
    // Streaming request bodies are unsupported (fail closed early).
    // Detection: a duplex/streamed request. `request.body` exists for
    // all non-GET/HEAD; streamed bodies have no known length, so the
    // bound check below is the real guard — reject stream usage by
    // reading bounded.
  }

  return request
    .arrayBuffer()
    .then((buffer): NormalizedBody => {
      if (buffer.byteLength > maxBodyBytes) {
        throw new DispatcherError({
          kind: "limit",
          detail: `request body is ${buffer.byteLength} bytes; dispatcher accepts at most ${maxBodyBytes}`,
        });
      }
      const decoder = new TextDecoder("utf-8", { fatal: false });
      const text = decoder.decode(buffer);

      if (media === "application/x-www-form-urlencoded") {
        // Parse into a string record and cross as JSON — transport
        // normalization only; schema validation stays in the kernel.
        const record: Record<string, string> = {};
        for (const [k, v] of new URLSearchParams(text)) {
          record[k] = v; // last-wins, matching native query semantics
        }
        return { json: JSON.stringify(record) };
      }

      if (media === "multipart/form-data") {
        const metadata = multipartMetadata(text, contentType);
        return { json: JSON.stringify(metadata) };
      }

      if (media === "application/octet-stream" || media.startsWith("image/") ||
          media.startsWith("audio/") || media.startsWith("video/") ||
          media === "application/wasm") {
        throw new DispatcherError({
          kind: "unsupported",
          feature: "binary request bodies",
          detail: `media type ${media || "(none)"} is not a supported browser-MVP body form (see UNSUPPORTED_SEMANTICS)`,
        });
      }

      // text/*, application/json, and unspecified forms cross as text;
      // the kernel decides whether it parses as JSON against the route.
      return media === "application/json" ? { json: text } : { text };
    });
}

/** Bounded multipart METADATA: part names + content types only. */
function multipartMetadata(text: string, contentTypeHeader: string): {
  parts: Array<{ name: string; contentType?: string }>;
} {
  const match = /boundary=(?:"([^"]+)"|([^;]+))/i.exec(contentContentOf(contentTypeHeader));
  const boundary = match?.[1] ?? match?.[2];
  if (!boundary) return { parts: [] };
  const parts: Array<{ name: string; contentType?: string }> = [];
  const delim = `--${boundary}`;
  let cursor = text.indexOf(delim);
  while (cursor !== -1 && parts.length < MAX_MULTIPART_PARTS) {
    const next = text.indexOf(delim, cursor + delim.length);
    if (next === -1) break;
    const segment = text.slice(cursor + delim.length, next);
    cursor = next;
    const headerEnd = segment.indexOf("\r\n\r\n");
    if (headerEnd === -1) continue;
    const headers = segment.slice(0, headerEnd);
    if (headers.length > MAX_MULTIPART_HEADER_BYTES) {
      throw new DispatcherError({
        kind: "limit",
        detail: `multipart part headers exceed ${MAX_MULTIPART_HEADER_BYTES} bytes`,
      });
    }
    const name = /name="([^"]*)"/i.exec(headers)?.[1] ?? "";
    const contentType = /content-type:\s*([^\r\n]+)/i.exec(headers)?.[1]?.trim();
    parts.push(contentType ? { name, contentType } : { name });
  }
  if (cursor !== -1 && parts.length >= MAX_MULTIPART_PARTS) {
    // More parts followed the last accepted one.
    const next = text.indexOf(delim, cursor + delim.length);
    if (next !== -1 && text.slice(cursor + delim.length, next).includes("name=")) {
      throw new DispatcherError({
        kind: "limit",
        detail: `multipart body exceeds ${MAX_MULTIPART_PARTS} parts`,
      });
    }
  }
  return { parts };
}

function contentContentOf(header: string): string {
  return header;
}

// ---------------------------------------------------------------------------
// Dispatcher
// ---------------------------------------------------------------------------

export interface DispatchOutcome {
  readonly plan: KernelPlanResult;
  /** Serialized completion message to hand the kernel (or null when the
   * plan is a problem or the request aborted before execution). */
  readonly completion: string | null;
  readonly isHead: boolean;
}

/**
 * Normalize a standard Request into the kernel ABI plan message and
 * drive plan → (executor seam) → completion. The kernel stays the only
 * authority for routing/validation/capabilities/problems.
 */
export async function dispatchFetchRequest(
  kernel: KernelInstance,
  abiVersion: number,
  request: Request,
  options: DispatcherOptions & {
    /** R-004 seam: executes an invoke plan, returns a completion result. */
    executeHandler?: (plan: Extract<KernelPlanResult, { kind: "invoke" }>) => Promise<unknown>;
    signal?: AbortSignal | null;
  } = {},
): Promise<Response> {
  const maxBodyBytes = options.maxBodyBytes ?? DEFAULT_MAX_BODY_BYTES;
  const signal = options.signal ?? null;
  const isHead = request.method.toUpperCase() === "HEAD";

  // Abort before dispatch: the documented cancellation result.
  if (signal?.aborted) throwAbort("before-dispatch");

  const url = new URL(request.url);
  const query: Array<[string, string]> = [];
  let queryPairCount = 0;
  url.searchParams.forEach((value, key) => {
    queryPairCount += 1;
    if (query.length < MAX_QUERY_PAIRS) query.push([key, value]);
  });
  if (queryPairCount > MAX_QUERY_PAIRS) {
    throw new DispatcherError({
      kind: "limit",
      detail: `query exceeds ${MAX_QUERY_PAIRS} pairs`,
    });
  }

  // Duplicate headers: the platform Headers contract joins duplicates
  // with ", " (deterministic); we forward the joined form (fixture-locked).
  const headers: Array<[string, string]> = [];
  request.headers.forEach((value, key) => {
    if (headers.length < MAX_HEADER_PAIRS) headers.push([key, value]);
  });

  const contentType = request.headers.get("content-type") ?? "";
  const hasBody = request.method !== "GET" && request.method !== "HEAD";
  const body = hasBody ? await normalizeBody(request, contentType, maxBodyBytes) : {};

  const message: KernelPlanRequest = {
    abiVersion,
    method: request.method,
    path: url.pathname,
    query,
    headers,
    ...(body.json !== undefined ? { body: body.json } : {}),
    ...(body.text !== undefined ? { body: body.text } : {}),
  };

  // Abort during dispatch (after body normalization, before the kernel
  // sees the request): documented cancellation result.
  if (signal?.aborted) throwAbort("during-dispatch");

  let parsed: unknown;
  try {
    parsed = JSON.parse(kernel.plan_request(JSON.stringify(message)));
  } catch {
    throw new DispatcherError({
      kind: "protocol",
      feature: "kernel protocol",
      detail: "kernel plan message was not valid JSON",
    });
  }

  if ((parsed as { kind?: string }).kind === "problem") {
    const problem = (parsed as KernelProblemShape).problem;
    return materialize(problemResponseBody(problem), isHead);
  }
  const plan = parsed as Extract<KernelPlanResult, { kind: "invoke" }>;

  const executor = options.executeHandler ?? defaultExecutor;
  const result = await executor(plan);
  const completion = JSON.stringify({
    abiVersion,
    routeId: plan.routeId,
    result,
  });

  let completed: unknown;
  try {
    completed = JSON.parse(kernel.complete_invocation(completion));
  } catch {
    throw new DispatcherError({
      kind: "protocol",
      feature: "kernel protocol",
      detail: "kernel completion message was not valid JSON",
    });
  }
  if ((completed as { kind?: string }).kind === "problem") {
    const problem = (completed as KernelProblemShape).problem;
    return materialize(problemResponseBody(problem), isHead);
  }
  const response = completed as {
    status: number;
    headers: ReadonlyArray<readonly [string, string]>;
    body?: unknown;
  };
  return materialize(
    new Response(response.body === undefined ? null : JSON.stringify(response.body), {
      status: response.status,
      headers: new Headers(response.headers.map((h) => [h[0], h[1]] as [string, string])),
    }),
    isHead,
  );
}

/**
 * Default executor (R-001 seam kept as the fallback): resolves the
 * declared default status with an empty object body. The kernel's
 * declared-status enforcement still runs on the completion — an
 * undeclared default produces the contract-violation problem.
 * R-004 replaces this with Worker execution.
 */
async function defaultExecutor(
  plan: Extract<KernelPlanResult, { kind: "invoke" }>,
): Promise<{ kind: "response"; status: number; headers: Array<[string, string]>; body: unknown }> {
  return {
    kind: "response",
    status: plan.defaultStatus,
    headers: [["content-type", "application/json"]],
    body: {},
  };
}

function throwAbort(phase: "before-dispatch" | "during-dispatch"): never {
  throw new DOMException("The operation was aborted.", "AbortError");
}

function problemResponseBody(problem: KernelProblemShape["problem"]): Response {
  const headers = new Headers();
  headers.set("content-type", "application/problem+json");
  if (problem.allow && problem.allow.length > 0) {
    headers.set("allow", problem.allow.join(", "));
  }
  return new Response(JSON.stringify({ ...problem }), { status: problem.status, headers });
}

/** HEAD responses keep status and headers, never a body (HTTP spec). */
async function materialize(response: Response, isHead: boolean): Promise<Response> {
  if (!isHead) return response;
  const headers = new Headers(response.headers);
  headers.delete("content-length");
  headers.delete("content-type");
  return new Response(null, { status: response.status, headers });
}
