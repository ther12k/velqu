/**
 * BWASM-C-001 — browser capability baseline: timer, crypto, logging,
 * restricted fetch (plus the ambient abort/text/url graph entries).
 *
 * Mirrors the native runtime's frozen capability graph
 * (`__velquNativeCapabilities`, q-engine-quickjs prelude): same graph
 * shape, same ids (`runtime:timers` v1, `runtime:crypto` v1,
 * `runtime:console` v1, `runtime:fetch` v1, …), native-parity bounds —
 * implemented over browser primitives with an explicit security and
 * resource policy:
 *
 * - **No ambient authority**: no editor credentials, cookies, storage,
 *   DOM, or unrestricted network. Fetch is default-DENY (an explicit
 *   origin allowlist at construction; empty = everything denied).
 * - **Timer** (runtime:timers): browser scheduling with a bounded
 *   ceiling and cancellation propagation (abort clears the timer and
 *   rejects with a typed cancellation; the work never fires).
 * - **Crypto** (runtime:crypto): WebCrypto random + digest ONLY where
 *   semantics match the native contract (SHA-256/384/512 hex digests,
 *   bounded getRandomValues). Anything else is a typed semantics
 *   mismatch — never a silent substitution.
 * - **Console** (runtime:console): bounded structured logging with
 *   native-parity redaction (auth headers, API-secret prefixes,
 *   key=value secrets), length/argument/count ceilings, correlation
 *   fields, and an injectable bounded host sink. Floods produce typed
 *   structured limit errors.
 * - **Fetch** (runtime:fetch): outbound with default-deny origins,
 *   method allowlist, scheme allowlist (http/https, native parity),
 *   request/response body ceilings (native 16 MiB parity), total
 *   deadline, credentials forced to omit, and redirects DENIED (a
 *   browser fetch cannot expose cross-origin redirect chains for
 *   revalidation; native returns the 3xx — the delta is documented,
 *   the policy outcome is stricter).
 *
 * Availability is introspectable BEFORE handler execution: the graph
 * factory returns frozen descriptors and a describe() summary.
 * Adapters are declared, with versions, in the deployment's
 * `velqu-artifacts.json` manifest (optional `capabilities` field,
 * BWASM-B-002 canonicalization).
 */

import type { CapabilityHandle } from "./capability-treaty";

// ---------------------------------------------------------------------------
// Identities (must equal q-capabilities inventory ids/versions)
// ---------------------------------------------------------------------------

export const BROWSER_TIMERS_ID = "runtime:timers";
export const BROWSER_CRYPTO_ID = "runtime:crypto";
export const BROWSER_CONSOLE_ID = "runtime:console";
export const BROWSER_FETCH_ID = "runtime:fetch";
export const BROWSER_ABORT_ID = "runtime:abort";
export const BROWSER_TEXT_ID = "runtime:text";
export const BROWSER_URL_ID = "runtime:url";

export const BROWSER_CAPABILITY_VERSION = 1;

/** One declared adapter in the deployment manifest. */
export interface CapabilityDescriptor {
  readonly id: string;
  readonly version: number;
  /** Human-readable name; never parsed. */
  readonly name: string;
}

// ---------------------------------------------------------------------------
// Typed errors (structured, machine-readable)
// ---------------------------------------------------------------------------

/** Base class: structured limit/policy errors carry a machine code. */
export class BrowserCapabilityError extends Error {
  readonly code: string;
  constructor(code: string, message: string) {
    super(`[@velqu/browser-runtime:capability:${code}] ${message}`);
    this.name = "BrowserCapabilityError";
    this.code = code;
  }
}

export class TimerDelayTooLarge extends BrowserCapabilityError {
  constructor(ms: number, maxMs: number) {
    super("timer-delay-too-large", `delay ${ms}ms exceeds the ${maxMs}ms ceiling`);
    this.name = "TimerDelayTooLarge";
  }
}

export class TimerCancelled extends BrowserCapabilityError {
  constructor() {
    super("timer-cancelled", "timer cancelled before firing; work discarded");
    this.name = "TimerCancelled";
  }
}

export class CryptoSemanticsMismatch extends BrowserCapabilityError {
  constructor(detail: string) {
    super("crypto-semantics-mismatch", detail);
    this.name = "CryptoSemanticsMismatch";
  }
}

export class ConsoleLimitExceeded extends BrowserCapabilityError {
  constructor(detail: string) {
    super("console-limit-exceeded", detail);
    this.name = "ConsoleLimitExceeded";
  }
}

export class FetchPolicyDenied extends BrowserCapabilityError {
  constructor(detail: string) {
    super("fetch-policy-denied", detail);
    this.name = "FetchPolicyDenied";
  }
}

export class FetchLimitExceeded extends BrowserCapabilityError {
  constructor(detail: string) {
    super("fetch-limit-exceeded", detail);
    this.name = "FetchLimitExceeded";
  }
}

// ---------------------------------------------------------------------------
// Native-parity bounds (must equal the q-capabilities constants)
// ---------------------------------------------------------------------------

export const MAX_BROWSER_TIMER_DELAY_MS = 300_000; // MAX_OP_DEADLINE_MS parity
export const MAX_CONSOLE_MSG_LEN = 16_384; // MAX_CONSOLE_MSG_LEN parity
export const MAX_CONSOLE_ARGS = 32; // MAX_CONSOLE_ARGS parity
export const MAX_CONSOLE_RECORDS = 64; // per-invocation record budget (R-004 parity)
export const MAX_FETCH_DEADLINE_MS = 300_000; // MAX_FETCH_DEADLINE_MS parity
export const DEFAULT_FETCH_DEADLINE_MS = 30_000; // DEFAULT_FETCH_DEADLINE_MS parity
export const MAX_FETCH_REQUEST_BODY_BYTES = 16 * 1024 * 1024; // native parity
export const MAX_FETCH_RESPONSE_BODY_BYTES = 16 * 1024 * 1024; // native parity
export const ALLOWED_SCHEMES: ReadonlyArray<string> = ["http", "https"]; // native parity
export const MAX_GET_RANDOM_VALUES_BYTES = 65_536; // WebCrypto identity bound
export const DIGEST_ALGORITHMS: ReadonlyArray<string> = ["SHA-256", "SHA-384", "SHA-512"];

// ---------------------------------------------------------------------------
// Console: redaction (port of q-capabilities::console::redact_sensitive_text)
// ---------------------------------------------------------------------------

const AUTH_PREFIXES = ["Bearer ", "bearer ", "Basic ", "basic "];
const SECRET_PREFIXES = ["sk-live-", "sk-test-", "ghp_", "gho_", "glpat-"];
const KEY_SEPARATORS: ReadonlyArray<readonly [string, string]> = [
  ["password", "="], ["password", ":"],
  ["secret", "="], ["secret", ":"],
  ["api_key", "="], ["api_key", ":"],
  ["apikey", "="], ["apikey", ":"],
  ["auth_token", "="], ["auth_token", ":"],
  ["token", "="], ["token", ":"],
  ["authorization", "="], ["authorization", ":"],
  ["cookie", "="], ["cookie", ":"],
];

/** Mirror of the native redactor: auth headers, API-secret prefixes,
 * key=value secrets → prefix kept + `[REDACTED]`. */
export function redactSensitiveText(text: string): string {
  let out = "";
  let i = 0;
  const len = text.length;
  while (i < len) {
    const remainder = text.slice(i);
    let matched = false;
    for (const prefix of AUTH_PREFIXES) {
      if (remainder.startsWith(prefix)) {
        out += prefix + "[REDACTED]";
        i += prefix.length;
        while (i < len && !/\s/.test(text[i]!) && !",;\"'".includes(text[i]!)) i++;
        matched = true;
        break;
      }
    }
    if (matched) continue;
    for (const prefix of SECRET_PREFIXES) {
      if (remainder.startsWith(prefix)) {
        out += prefix + "[REDACTED]";
        i += prefix.length;
        while (i < len && /[A-Za-z0-9_-]/.test(text[i]!)) i++;
        matched = true;
        break;
      }
    }
    if (matched) continue;
    let keyHit: readonly [string, string] | null = null;
    for (const [key, sep] of KEY_SEPARATORS) {
      if (
        remainder.length > key.length + sep.length &&
        remainder.slice(0, key.length).toLowerCase() === key &&
        remainder.slice(key.length, key.length + sep.length) === sep
      ) {
        keyHit = [key, sep];
        break;
      }
    }
    if (keyHit) {
      const [key, sep] = keyHit;
      out += remainder.slice(0, key.length + sep.length) + "[REDACTED]";
      i += key.length + sep.length;
      let inQuote = false;
      if (i < len && (text[i] === '"' || text[i] === "'")) {
        inQuote = true;
        i++;
      }
      while (i < len && /[A-Za-z0-9_./+=-]/.test(text[i]!)) i++;
      if (inQuote && i < len && (text[i] === '"' || text[i] === "'")) i++;
      continue;
    }
    out += text[i]!;
    i++;
  }
  return out;
}

export type ConsoleLevel = "debug" | "info" | "warn" | "error";

/** One structured, redacted, bounded record (ConsoleRecord parity). */
export interface ConsoleRecord {
  readonly level: ConsoleLevel;
  readonly message: string;
  readonly correlationId?: string;
  readonly truncated: boolean;
}

/** The bounded host sink: forwarded records land here. */
export interface ConsoleSink {
  push(record: ConsoleRecord): void;
}

export const CONSOLE_LEVELS: ReadonlyArray<ConsoleLevel> = ["debug", "info", "warn", "error"];

/** Bounded ring buffer sink (default host sink). */
export function ringBufferSink(capacity = 128): ConsoleSink & { records(): ReadonlyArray<ConsoleRecord> } {
  const records: ConsoleRecord[] = [];
  return {
    push(record) {
      records.push(record);
      if (records.length > capacity) records.shift();
    },
    records: () => [...records],
  };
}

function formatArgs(args: ReadonlyArray<unknown>): { message: string; truncated: boolean } {
  const parts = args.map((a) => {
    if (typeof a === "string") return a;
    try {
      return JSON.stringify(a) ?? String(a);
    } catch {
      return String(a);
    }
  });
  let message = parts.join(" ");
  let truncated = false;
  const redacted = redactSensitiveText(message);
  if (redacted.length > MAX_CONSOLE_MSG_LEN) {
    message = redacted.slice(0, MAX_CONSOLE_MSG_LEN) + "...[TRUNCATED]";
    truncated = true;
  } else {
    message = redacted;
  }
  return { message, truncated };
}

/**
 * The bounded console object (runtime:console v1). Console-shaped
 * (debug/info/warn/error/log) so handler code reads naturally; every
 * call is redacted, bounded, counted, and forwarded to the sink.
 */
export function createConsoleCapability(options: {
  readonly sink: ConsoleSink;
  readonly correlationId?: () => string | undefined;
}): { console: Console; descriptors: [CapabilityDescriptor] } {
  let count = 0;
  const emit = (level: ConsoleLevel) =>
    (...args: unknown[]) => {
      if (args.length > MAX_CONSOLE_ARGS) {
        throw new ConsoleLimitExceeded(`${args.length} args exceeds the ${MAX_CONSOLE_ARGS}-arg ceiling`);
      }
      count += 1;
      if (count > MAX_CONSOLE_RECORDS) {
        throw new ConsoleLimitExceeded(
          `record budget exhausted (${MAX_CONSOLE_RECORDS} records); further logging refused`,
        );
      }
      const { message, truncated } = formatArgs(args);
      const correlationId = options.correlationId?.();
      options.sink.push({ level, message, truncated, ...(correlationId ? { correlationId } : {}) });
    };
  const consoleShaped = {
    debug: emit("debug"),
    info: emit("info"),
    warn: emit("warn"),
    error: emit("error"),
    log: emit("info"),
  } as unknown as Console;
  return {
    console: Object.freeze(consoleShaped),
    descriptors: [{ id: BROWSER_CONSOLE_ID, version: BROWSER_CAPABILITY_VERSION, name: "bounded console" }],
  };
}

// ---------------------------------------------------------------------------
// Timer (runtime:timers v1)
// ---------------------------------------------------------------------------

export interface TimerDelayInput {
  readonly ms: number;
  /** Cancellation: aborting clears the timer; the work never fires. */
  readonly signal?: AbortSignal;
}

export interface TimerDelayResult {
  readonly elapsedMs: number;
}

export function createTimerCapability(options?: { maxDelayMs?: number }): {
  timers: { delay(ms: number | TimerDelayInput, signal?: AbortSignal): Promise<TimerDelayResult> };
  descriptors: [CapabilityDescriptor];
} {
  const maxDelayMs = Math.min(options?.maxDelayMs ?? MAX_BROWSER_TIMER_DELAY_MS, MAX_BROWSER_TIMER_DELAY_MS);
  const delay = (msOrInput: number | TimerDelayInput, signalArg?: AbortSignal): Promise<TimerDelayResult> => {
    const ms = typeof msOrInput === "number" ? msOrInput : msOrInput.ms;
    const signal = typeof msOrInput === "number" ? signalArg : msOrInput.signal;
    if (typeof ms !== "number" || !Number.isFinite(ms) || ms < 0) {
      return Promise.reject(new BrowserCapabilityError("timer-invalid-delay", `delay must be a finite ms >= 0, got ${String(ms)}`));
    }
    if (ms > maxDelayMs) {
      return Promise.reject(new TimerDelayTooLarge(ms, maxDelayMs));
    }
    if (signal?.aborted) {
      return Promise.reject(new TimerCancelled());
    }
    return new Promise<TimerDelayResult>((resolve, reject) => {
      const started = Date.now();
      let settled = false;
      const timer = setTimeout(() => {
        if (settled) return;
        settled = true;
        if (signal) signal.removeEventListener("abort", onAbort);
        resolve({ elapsedMs: Date.now() - started });
      }, ms);
      const onAbort = () => {
        if (settled) return;
        settled = true;
        clearTimeout(timer); // the work is discarded — it never fires
        reject(new TimerCancelled());
      };
      signal?.addEventListener("abort", onAbort, { once: true });
    });
  };
  return {
    timers: Object.freeze({ delay }),
    descriptors: [{ id: BROWSER_TIMERS_ID, version: BROWSER_CAPABILITY_VERSION, name: "bounded browser timer" }],
  };
}

// ---------------------------------------------------------------------------
// Crypto (runtime:crypto v1) — WebCrypto where semantics match
// ---------------------------------------------------------------------------

export type CryptoDigestInput = {
  readonly op: "digest";
  readonly algorithm: "SHA-256" | "SHA-384" | "SHA-512";
  readonly data: Uint8Array;
  /** Format of the returned digest; hex (native digest parity) default. */
  readonly encoding?: "hex" | "bytes";
} | {
  readonly op: "getRandomValues";
  /** Number of random bytes; ≤ 65_536 (WebCrypto identity bound). */
  readonly length: number;
};

function toHex(bytes: Uint8Array): string {
  let hex = "";
  for (const b of bytes) hex += b.toString(16).padStart(2, "0");
  return hex;
}

export function createCryptoCapability(): {
  crypto: {
    call(input: CryptoDigestInput): Promise<string | Uint8Array>;
    /** Console-shaped helpers for handler ergonomics. */
    digestHex(algorithm: "SHA-256" | "SHA-384" | "SHA-512", data: Uint8Array): Promise<string>;
    getRandomValues(length: number): Promise<Uint8Array>;
  };
  descriptors: [CapabilityDescriptor];
} {
  const call = async (input: CryptoDigestInput): Promise<string | Uint8Array> => {
    if (input.op === "getRandomValues") {
      if (!Number.isInteger(input.length) || input.length < 0 || input.length > MAX_GET_RANDOM_VALUES_BYTES) {
        throw new CryptoSemanticsMismatch(
          `getRandomValues length ${input.length} outside 0..=${MAX_GET_RANDOM_VALUES_BYTES}`,
        );
      }
      return crypto.getRandomValues(new Uint8Array(input.length));
    }
    if (input.op === "digest") {
      if (!DIGEST_ALGORITHMS.includes(input.algorithm)) {
        // No silent substitution: an algorithm whose semantics we have
        // not pinned against the native contract is rejected by name.
        throw new CryptoSemanticsMismatch(
          `digest algorithm "${input.algorithm}" is not pinned for browser/native parity (allowed: ${DIGEST_ALGORITHMS.join(", ")})`,
        );
      }
      const digest = await crypto.subtle.digest(input.algorithm, input.data as BufferSource);
      const bytes = new Uint8Array(digest);
      return input.encoding === "bytes" ? bytes : toHex(bytes);
    }
    throw new CryptoSemanticsMismatch(`unknown crypto op "${(input as { op: string }).op}"`);
  };
  return {
    crypto: Object.freeze({
      call,
      digestHex: (algorithm, data) => call({ op: "digest", algorithm, data }) as Promise<string>,
      getRandomValues: (length: number) => call({ op: "getRandomValues", length }) as Promise<Uint8Array>,
    }),
    descriptors: [{ id: BROWSER_CRYPTO_ID, version: BROWSER_CAPABILITY_VERSION, name: "pinned WebCrypto subset" }],
  };
}

// ---------------------------------------------------------------------------
// Fetch (runtime:fetch v1) — default-deny, bounded, credential-free
// ---------------------------------------------------------------------------

export interface BrowserFetchPolicy {
  /**
   * Explicit origin allowlist (scheme + host + optional port).
   * DEFAULT-DENY: an empty list denies every request. Prefix matching
   * is NOT applied — origins match exactly.
   */
  readonly allowedOrigins: ReadonlyArray<string>;
  /** Allowed methods (default GET + HEAD). */
  readonly allowedMethods?: ReadonlyArray<string>;
  /** Total deadline ceiling per request, including body read (default 30s, max 300s). */
  readonly deadlineMs?: number;
  /** Request body ceiling (default/parity 16 MiB). */
  readonly maxRequestBytes?: number;
  /** Response body ceiling (default/parity 16 MiB). */
  readonly maxResponseBytes?: number;
}

export interface BrowserFetchInput {
  readonly url: string;
  readonly method?: string;
  readonly headers?: Readonly<Record<string, string>>;
  readonly body?: Uint8Array;
  readonly signal?: AbortSignal;
}

export interface BrowserFetchResult {
  readonly status: number;
  readonly headers: Readonly<Record<string, string>>;
  readonly bodyBytes: Uint8Array;
  readonly truncated: boolean;
}

/**
 * The restricted outbound fetch (runtime:fetch v1). Every request is
 * checked against the policy BEFORE any I/O; credentials are forced to
 * omit; redirects are denied (a browser fetch cannot hand back the 3xx
 * chain for per-hop revalidation, so the policy outcome is stricter
 * than native's Manual — the delta is documented, not hidden).
 */
export function createFetchCapability(options: {
  readonly policy: BrowserFetchPolicy;
  /** Injectable fetch (tests; defaults to global fetch). */
  readonly fetchImpl?: typeof fetch;
}): {
  fetch: {
    call(input: BrowserFetchInput): Promise<BrowserFetchResult>;
    /** The policy summary — introspectable before execution. */
    policySummary(): string;
  };
  descriptors: [CapabilityDescriptor];
} {
  const allowedOrigins = options.policy.allowedOrigins.map((o) => o.replace(/\/+$/, ""));
  const allowedMethods = options.policy.allowedMethods ?? ["GET", "HEAD"];
  const deadlineMs = Math.min(options.policy.deadlineMs ?? DEFAULT_FETCH_DEADLINE_MS, MAX_FETCH_DEADLINE_MS);
  const maxRequestBytes = options.policy.maxRequestBytes ?? MAX_FETCH_REQUEST_BODY_BYTES;
  const maxResponseBytes = options.policy.maxResponseBytes ?? MAX_FETCH_RESPONSE_BODY_BYTES;
  const doFetch = options.fetchImpl ?? fetch;

  const call = async (input: BrowserFetchInput): Promise<BrowserFetchResult> => {
    // scheme + origin policy (default deny)
    let url: URL;
    try {
      url = new URL(input.url);
    } catch {
      throw new FetchPolicyDenied(`unparseable url: ${JSON.stringify(input.url).slice(0, 80)}`);
    }
    if (!ALLOWED_SCHEMES.includes(url.protocol.replace(":", ""))) {
      throw new FetchPolicyDenied(`scheme "${url.protocol}" not allowed (${ALLOWED_SCHEMES.join("/")})`);
    }
    const origin = url.origin;
    if (!allowedOrigins.includes(origin)) {
      throw new FetchPolicyDenied(
        `origin "${origin}" is not in the allowlist (default deny; allowed: ${allowedOrigins.length} origin(s))`,
      );
    }
    // method policy
    const method = (input.method ?? "GET").toUpperCase();
    if (!allowedMethods.includes(method)) {
      throw new FetchPolicyDenied(`method "${method}" not allowed (${allowedMethods.join("/")})`);
    }
    // body ceiling before I/O
    if (input.body && input.body.byteLength > maxRequestBytes) {
      throw new FetchLimitExceeded(`request body ${input.body.byteLength}B exceeds ${maxRequestBytes}B`);
    }
    const headers = new Headers(input.headers as HeadersInit);
    // ambient credential stripping: cookies are never attached (forced
    // omit at the Request level AND no credential headers pass through).
    headers.delete("cookie");
    headers.delete("authorization");

    const deadlineController = new AbortController();
    const deadline = new Promise<never>((_, reject) => {
      deadlineController.signal.addEventListener("abort", () => {
        reject(new FetchLimitExceeded(`deadline ${deadlineMs}ms exceeded`));
      }, { once: true });
    });
    const deadlineTimer = setTimeout(() => deadlineController.abort(), deadlineMs);
    const request = new Request(url.href, {
      method,
      headers,
      ...(input.body ? { body: input.body as BodyInit } : {}),
      credentials: "omit", // policy: never ambient credentials
      redirect: "error", // policy: redirects denied (see module doc)
      signal: deadlineController.signal,
    });
    let response: Response;
    try {
      // Race the policy deadline ourselves (injected transports may
      // ignore the abort signal); the signal stays for real fetches.
      response = await Promise.race([doFetch(request), deadline]);
    } catch (cause) {
      if (cause instanceof FetchLimitExceeded) throw cause;
      if (cause instanceof DOMException && cause.name === "TimeoutError") {
        throw new FetchLimitExceeded(`deadline ${deadlineMs}ms exceeded`);
      }
      if (cause instanceof DOMException && cause.name === "AbortError") {
        throw new FetchPolicyDenied("request aborted by redirect/abort policy");
      }
      if (cause instanceof TypeError && /redirect/i.test(cause.message)) {
        throw new FetchPolicyDenied("redirects are denied by policy (redirect: error)");
      }
      throw cause;
    } finally {
      clearTimeout(deadlineTimer);
    }
    // Redirect enforcement at the adapter level: with redirect:"error"
    // real browsers reject before this point, but a 3xx from any
    // transport is still policy-denied (never followed, never escaped).
    if (response.status >= 300 && response.status < 400) {
      throw new FetchPolicyDenied(`redirect status ${response.status} denied by policy`);
    }
    // bounded response materialization
    const reader = response.body?.getReader();
    const chunks: Uint8Array[] = [];
    let total = 0;
    let truncated = false;
    if (reader) {
      for (;;) {
        const { done, value } = await reader.read();
        if (done) break;
        total += value!.byteLength;
        if (total > maxResponseBytes) {
          truncated = true;
          void reader.cancel();
          break;
        }
        chunks.push(value!);
      }
    }
    const bodyBytes = new Uint8Array(Math.min(total, maxResponseBytes));
    let offset = 0;
    for (const chunk of chunks) {
      bodyBytes.set(chunk.subarray(0, Math.min(chunk.byteLength, bodyBytes.byteLength - offset)), offset);
      offset += chunk.byteLength;
      if (offset >= bodyBytes.byteLength) break;
    }
    const responseHeaders: Record<string, string> = {};
    response.headers.forEach((v, k) => {
      responseHeaders[k] = v;
    });
    return Object.freeze({
      status: response.status,
      headers: Object.freeze(responseHeaders),
      bodyBytes,
      truncated,
    });
  };

  return {
    fetch: Object.freeze({
      call,
      policySummary: () =>
        `default-deny origins (allowlist: ${allowedOrigins.length}), methods: ${allowedMethods.join("/")}, ` +
        `deadline ≤ ${deadlineMs}ms, bodies ≤ ${maxRequestBytes}/${maxResponseBytes}B, credentials: omit, redirects: denied`,
    }),
    descriptors: [{ id: BROWSER_FETCH_ID, version: BROWSER_CAPABILITY_VERSION, name: "restricted outbound fetch" }],
  };
}

// ---------------------------------------------------------------------------
// The frozen browser capability graph (native graph parity)
// ---------------------------------------------------------------------------

export interface BrowserCapabilityGraph {
  /** Frozen graph entries, keyed by short name (native graph parity). */
  readonly graph: Readonly<{
    timers: unknown;
    crypto: unknown;
    console: unknown;
    fetch: unknown;
    abort: unknown;
    text: unknown;
    url: unknown;
  }>;
  /** Declared adapters — written to the deployment manifest. */
  readonly descriptors: ReadonlyArray<CapabilityDescriptor>;
  /** Human-readable availability summary (introspection pre-execution). */
  readonly describe: () => string;
}

/**
 * Install the browser capability baseline. Availability is decided HERE
 * (at installation, before any handler runs) and is introspectable via
 * `descriptors` / `describe()`.
 */
export function createBrowserCapabilityGraph(options: {
  readonly consoleSink: ConsoleSink;
  readonly consoleCorrelationId?: () => string | undefined;
  readonly fetchPolicy: BrowserFetchPolicy;
  readonly fetchImpl?: typeof fetch;
  readonly timerMaxDelayMs?: number;
}): BrowserCapabilityGraph {
  const timer = createTimerCapability({ maxDelayMs: options.timerMaxDelayMs });
  const cryptoCap = createCryptoCapability();
  const consoleCap = createConsoleCapability({
    sink: options.consoleSink,
    correlationId: options.consoleCorrelationId,
  });
  const fetchCap = createFetchCapability({ policy: options.fetchPolicy, fetchImpl: options.fetchImpl });

  const abort = Object.freeze({
    AbortController: globalThis.AbortController,
    AbortSignal: globalThis.AbortSignal,
  });
  const text = Object.freeze({
    TextEncoder: globalThis.TextEncoder,
    TextDecoder: globalThis.TextDecoder,
  });
  const url = Object.freeze({ URL: globalThis.URL, URLSearchParams: globalThis.URLSearchParams });

  const descriptors: ReadonlyArray<CapabilityDescriptor> = [
    ...timer.descriptors,
    ...cryptoCap.descriptors,
    ...consoleCap.descriptors,
    ...fetchCap.descriptors,
    { id: BROWSER_ABORT_ID, version: BROWSER_CAPABILITY_VERSION, name: "abort primitives" },
    { id: BROWSER_TEXT_ID, version: BROWSER_CAPABILITY_VERSION, name: "text encoding" },
    { id: BROWSER_URL_ID, version: BROWSER_CAPABILITY_VERSION, name: "url parsing" },
  ];

  return {
    graph: Object.freeze({
      timers: timer.timers,
      crypto: cryptoCap.crypto,
      console: consoleCap.console,
      fetch: fetchCap.fetch,
      abort,
      text,
      url,
    }),
    descriptors,
    describe: () => descriptors.map((d) => `${d.id}@${d.version} (${d.name})`).join("; "),
  };
}

// ---------------------------------------------------------------------------
// CapabilityHandle adapters (for the R-005 registry, where a deployment
// wants the baseline routed through kernel authorization)
// ---------------------------------------------------------------------------

/** The graph as registry-installable handles (call(input) contract). */
export function baselineHandles(graph: BrowserCapabilityGraph): ReadonlyArray<CapabilityHandle> {
  const g = graph.graph as {
    timers: { delay(input: TimerDelayInput): Promise<TimerDelayResult> };
    crypto: { call(input: CryptoDigestInput): Promise<string | Uint8Array> };
    console: Record<string, (...args: unknown[]) => void>;
    fetch: { call(input: BrowserFetchInput): Promise<BrowserFetchResult> };
  };
  return [
    { id: BROWSER_TIMERS_ID, version: BROWSER_CAPABILITY_VERSION, call: (input) => g.timers.delay(input as TimerDelayInput) },
    { id: BROWSER_CRYPTO_ID, version: BROWSER_CAPABILITY_VERSION, call: (input) => g.crypto.call(input as CryptoDigestInput) },
    {
      id: BROWSER_CONSOLE_ID,
      version: BROWSER_CAPABILITY_VERSION,
      call: async (input) => {
        const { level, args } = input as { level: ConsoleLevel; args: unknown[] };
        (g.console[level ?? "info"] as ((...a: unknown[]) => void) | undefined)?.(...(args ?? []));
        return { forwarded: true };
      },
    },
    { id: BROWSER_FETCH_ID, version: BROWSER_CAPABILITY_VERSION, call: (input) => g.fetch.call(input as BrowserFetchInput) },
  ];
}
