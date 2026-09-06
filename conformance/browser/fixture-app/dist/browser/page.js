// packages/browser-runtime/src/dispatcher.ts
var DEFAULT_MAX_BODY_BYTES = 1 << 20;
var MAX_MULTIPART_PARTS = 64;
var MAX_MULTIPART_HEADER_BYTES = 8 << 10;
var MAX_QUERY_PAIRS = 256;
var MAX_HEADER_PAIRS = 128;
class DispatcherError extends Error {
  rejection;
  constructor(rejection) {
    const what = rejection.kind === "aborted" ? `request aborted ${rejection.phase}` : rejection.kind === "unsupported" ? `unsupported: ${rejection.feature}` : `limit: ${rejection.detail}`;
    super(`[@velqu/browser-runtime:dispatcher] ${what}`);
    this.name = "DispatcherError";
    this.rejection = rejection;
  }
}
function normalizeBody(request, contentType, maxBodyBytes) {
  const media = contentType.split(";")[0]?.trim().toLowerCase() ?? "";
  if (request.body !== null && typeof request.body.locked !== "undefined") {}
  return request.arrayBuffer().then((buffer) => {
    if (buffer.byteLength > maxBodyBytes) {
      throw new DispatcherError({
        kind: "limit",
        detail: `request body is ${buffer.byteLength} bytes; dispatcher accepts at most ${maxBodyBytes}`
      });
    }
    const decoder = new TextDecoder("utf-8", { fatal: false });
    const text = decoder.decode(buffer);
    if (media === "application/x-www-form-urlencoded") {
      const record = {};
      for (const [k, v] of new URLSearchParams(text)) {
        record[k] = v;
      }
      return { json: JSON.stringify(record) };
    }
    if (media === "multipart/form-data") {
      const metadata = multipartMetadata(text, contentType);
      return { json: JSON.stringify(metadata) };
    }
    if (media === "application/octet-stream" || media.startsWith("image/") || media.startsWith("audio/") || media.startsWith("video/") || media === "application/wasm") {
      throw new DispatcherError({
        kind: "unsupported",
        feature: "binary request bodies",
        detail: `media type ${media || "(none)"} is not a supported browser-MVP body form (see UNSUPPORTED_SEMANTICS)`
      });
    }
    return media === "application/json" ? { json: text } : { text };
  });
}
function multipartMetadata(text, contentTypeHeader) {
  const match = /boundary=(?:"([^"]+)"|([^;]+))/i.exec(contentContentOf(contentTypeHeader));
  const boundary = match?.[1] ?? match?.[2];
  if (!boundary)
    return { parts: [] };
  const parts = [];
  const delim = `--${boundary}`;
  let cursor = text.indexOf(delim);
  while (cursor !== -1 && parts.length < MAX_MULTIPART_PARTS) {
    const next = text.indexOf(delim, cursor + delim.length);
    if (next === -1)
      break;
    const segment = text.slice(cursor + delim.length, next);
    cursor = next;
    const headerEnd = segment.indexOf(`\r
\r
`);
    if (headerEnd === -1)
      continue;
    const headers = segment.slice(0, headerEnd);
    if (headers.length > MAX_MULTIPART_HEADER_BYTES) {
      throw new DispatcherError({
        kind: "limit",
        detail: `multipart part headers exceed ${MAX_MULTIPART_HEADER_BYTES} bytes`
      });
    }
    const name = /name="([^"]*)"/i.exec(headers)?.[1] ?? "";
    const contentType = /content-type:\s*([^\r\n]+)/i.exec(headers)?.[1]?.trim();
    parts.push(contentType ? { name, contentType } : { name });
  }
  if (cursor !== -1 && parts.length >= MAX_MULTIPART_PARTS) {
    const next = text.indexOf(delim, cursor + delim.length);
    if (next !== -1 && text.slice(cursor + delim.length, next).includes("name=")) {
      throw new DispatcherError({
        kind: "limit",
        detail: `multipart body exceeds ${MAX_MULTIPART_PARTS} parts`
      });
    }
  }
  return { parts };
}
function contentContentOf(header) {
  return header;
}
async function dispatchFetchRequest(kernel, abiVersion, request, options = {}) {
  const maxBodyBytes = options.maxBodyBytes ?? DEFAULT_MAX_BODY_BYTES;
  const signal = options.signal ?? null;
  const isHead = request.method.toUpperCase() === "HEAD";
  if (signal?.aborted)
    throwAbort("before-dispatch");
  const url = new URL(request.url);
  const query = [];
  let queryPairCount = 0;
  url.searchParams.forEach((value, key) => {
    queryPairCount += 1;
    if (query.length < MAX_QUERY_PAIRS)
      query.push([key, value]);
  });
  if (queryPairCount > MAX_QUERY_PAIRS) {
    throw new DispatcherError({
      kind: "limit",
      detail: `query exceeds ${MAX_QUERY_PAIRS} pairs`
    });
  }
  const headers = [];
  request.headers.forEach((value, key) => {
    if (headers.length < MAX_HEADER_PAIRS)
      headers.push([key, value]);
  });
  const contentType = request.headers.get("content-type") ?? "";
  const hasBody = request.method !== "GET" && request.method !== "HEAD";
  const body = hasBody ? await normalizeBody(request, contentType, maxBodyBytes) : {};
  const message = {
    abiVersion,
    method: request.method,
    path: url.pathname,
    query,
    headers,
    ...body.json !== undefined ? { body: body.json } : {},
    ...body.text !== undefined ? { body: body.text } : {}
  };
  if (signal?.aborted)
    throwAbort("during-dispatch");
  let parsed;
  try {
    parsed = JSON.parse(kernel.plan_request(JSON.stringify(message)));
  } catch {
    throw new DispatcherError({
      kind: "protocol",
      feature: "kernel protocol",
      detail: "kernel plan message was not valid JSON"
    });
  }
  if (parsed.kind === "problem") {
    const problem = parsed.problem;
    return materialize(problemResponseBody(problem), isHead);
  }
  const plan = parsed;
  const executor = options.executeHandler ?? defaultExecutor;
  const result = await executor(plan);
  const completion = JSON.stringify({
    abiVersion,
    routeId: plan.routeId,
    result
  });
  let completed;
  try {
    completed = JSON.parse(kernel.complete_invocation(completion));
  } catch {
    throw new DispatcherError({
      kind: "protocol",
      feature: "kernel protocol",
      detail: "kernel completion message was not valid JSON"
    });
  }
  if (completed.kind === "problem") {
    const problem = completed.problem;
    return materialize(problemResponseBody(problem), isHead);
  }
  const response = completed;
  return materialize(new Response(response.body === undefined ? null : JSON.stringify(response.body), {
    status: response.status,
    headers: new Headers(response.headers.map((h) => [h[0], h[1]]))
  }), isHead);
}
async function defaultExecutor(plan) {
  return {
    kind: "response",
    status: plan.defaultStatus,
    headers: [["content-type", "application/json"]],
    body: {}
  };
}
function throwAbort(phase) {
  throw new DOMException("The operation was aborted.", "AbortError");
}
function problemResponseBody(problem) {
  const headers = new Headers;
  headers.set("content-type", "application/problem+json");
  if (problem.allow && problem.allow.length > 0) {
    headers.set("allow", problem.allow.join(", "));
  }
  return new Response(JSON.stringify({ ...problem }), { status: problem.status, headers });
}
async function materialize(response, isHead) {
  if (!isHead)
    return response;
  const headers = new Headers(response.headers);
  headers.delete("content-length");
  headers.delete("content-type");
  return new Response(null, { status: response.status, headers });
}

// packages/browser-runtime/src/handler-bundle.ts
var HANDLER_ABI_VERSION = 1;

class HandlerBundleError extends Error {
  code;
  constructor(code, message) {
    super(`[@velqu/browser-runtime:handler-bundle:${code}] ${message}`);
    this.name = "HandlerBundleError";
    this.code = code;
  }
}
function defineBrowserHandlers(registrations, expectation) {
  const bundleAbi = expectation.handlerAbiVersion ?? HANDLER_ABI_VERSION;
  if (bundleAbi !== HANDLER_ABI_VERSION) {
    throw new HandlerBundleError("UNKNOWN_ABI_VERSION", `handler bundle declares ABI ${bundleAbi}; this runtime implements ${HANDLER_ABI_VERSION} — rebuild the bundle with a matching @velqu/browser-runtime`);
  }
  const required = [...expectation.requiredHandlerKeys].sort();
  const byKey = new Map;
  for (const reg of registrations) {
    if (typeof reg.handlerKey !== "string" || reg.handlerKey.length === 0 || typeof reg.invoke !== "function") {
      throw new HandlerBundleError("INVALID_REGISTRATION", `registration ${JSON.stringify(reg.handlerKey)} is not well-formed (need handlerKey + invoke)`);
    }
    if (byKey.has(reg.handlerKey)) {
      throw new HandlerBundleError("DUPLICATE_HANDLER_KEY", `handler key ${JSON.stringify(reg.handlerKey)} registered more than once`);
    }
    byKey.set(reg.handlerKey, reg);
  }
  const missing = required.filter((key) => !byKey.has(key));
  if (missing.length > 0) {
    throw new HandlerBundleError("MISSING_HANDLER", `bundle is missing ${missing.length} pack-declared handler(s): ${missing.join(", ")}`);
  }
  const extra = [...byKey.keys()].filter((key) => !required.includes(key));
  if (extra.length > 0) {
    throw new HandlerBundleError("INVALID_REGISTRATION", `bundle registers handler(s) the pack does not declare: ${extra.join(", ")} (undeclared routes cannot be registered silently)`);
  }
  for (const [key, reg] of byKey) {
    const declared = expectation.declaredStatuses[key] ?? [];
    for (const status of reg.statuses) {
      if (!declared.includes(status)) {
        throw new HandlerBundleError("UNDECLARED_STATUS", `handler ${JSON.stringify(key)} registers status ${status}; the pack declares only [${declared.join(", ")}]`);
      }
    }
  }
  const table = new Map([...byKey.entries()].map(([key, reg]) => [key, reg.invoke]));
  return {
    abiVersion: HANDLER_ABI_VERSION,
    keys: required,
    async invoke(handlerKey, ctx) {
      const invoke = table.get(handlerKey);
      if (!invoke) {
        throw new HandlerBundleError("MISSING_HANDLER", `handler ${JSON.stringify(handlerKey)} is not registered`);
      }
      return invoke(ctx);
    }
  };
}
// packages/browser-runtime/src/worker-host.ts
var WORKER_PROTOCOL_VERSION = 1;
var MAX_LOG_LINES_PER_INVOCATION = 64;
var MAX_RESULT_BYTES = 1 << 20;

class WorkerProtocolError extends Error {
  code;
  constructor(code, message) {
    super(`[@velqu/browser-runtime:worker:${code}] ${message}`);
    this.name = "WorkerProtocolError";
    this.code = code;
  }
}

class WorkerHost {
  worker;
  sessionId;
  factory;
  pending = new Map;
  nextInvocationId = 1;
  terminated = false;
  staleMessagesDropped = 0;
  hardRecoveries = 0;
  constructor(sessionId, factory) {
    this.sessionId = sessionId;
    this.factory = factory;
    this.worker = this.spawn();
  }
  spawn() {
    const worker = this.factory();
    worker.addEventListener("message", (event) => this.onMessage(event.data));
    return worker;
  }
  get isTerminated() {
    return this.terminated;
  }
  execute(plan, signal) {
    if (this.terminated) {
      this.terminated = false;
      this.worker = this.spawn();
    }
    const invocationId = this.nextInvocationId++;
    return new Promise((resolve, reject) => {
      const pending = {
        invocationId,
        plan,
        resolve,
        reject,
        timer: null,
        logLines: 0
      };
      this.pending.set(invocationId, pending);
      const abort = () => {
        if (!this.pending.has(invocationId))
          return;
        this.pending.delete(invocationId);
        this.worker.postMessage({
          v: WORKER_PROTOCOL_VERSION,
          type: "cancel",
          sessionId: this.sessionId,
          invocationId
        });
        reject(new DOMException("The operation was aborted.", "AbortError"));
      };
      if (signal?.aborted) {
        abort();
        return;
      }
      signal?.addEventListener("abort", abort, { once: true });
      pending.timer = setTimeout(() => {
        if (!this.pending.has(invocationId))
          return;
        this.pending.delete(invocationId);
        this.hardTerminate(`deadline ${plan.deadlineMs}ms exceeded`);
        reject(new DOMException("The operation was aborted.", "AbortError"));
      }, plan.deadlineMs);
      let message;
      try {
        message = JSON.stringify({
          v: WORKER_PROTOCOL_VERSION,
          type: "invoke",
          sessionId: this.sessionId,
          invocationId,
          plan
        });
      } catch (cause) {
        this.pending.delete(invocationId);
        reject(new WorkerProtocolError("UNCLONEABLE_PAYLOAD", String(cause)));
        return;
      }
      if (message.length > MAX_RESULT_BYTES) {
        this.pending.delete(invocationId);
        reject(new WorkerProtocolError("OVERSIZED_PAYLOAD", "invoke message exceeds 1 MiB"));
        return;
      }
      this.worker.postMessage(JSON.parse(message));
    });
  }
  hardTerminate(reason) {
    this.hardRecoveries += 1;
    this.worker.terminate();
    this.terminated = true;
    for (const pending of this.pending.values()) {
      if (pending.timer)
        clearTimeout(pending.timer);
      pending.reject(new WorkerProtocolError("FATAL", `worker terminated: ${reason}`));
    }
    this.pending.clear();
  }
  dispose() {
    this.hardTerminate("disposed");
  }
  onMessage(data) {
    if (typeof data !== "object" || data === null)
      return;
    const msg = data;
    if (msg.v !== WORKER_PROTOCOL_VERSION)
      return;
    if (msg.type !== "result" && msg.type !== "log" && msg.type !== "fatal") {
      return;
    }
    if (typeof msg.sessionId !== "string" || msg.type !== "fatal" && typeof msg.invocationId !== "number" || msg.type === "result" && (typeof msg.result !== "object" || msg.result === null) || msg.type === "log" && !Array.isArray(msg.lines)) {
      this.staleMessagesDropped += 1;
      return;
    }
    switch (msg.type) {
      case "result": {
        if (msg.sessionId !== this.sessionId) {
          this.staleMessagesDropped += 1;
          return;
        }
        const invocationId = msg.invocationId;
        const pending = this.pending.get(invocationId);
        if (!pending) {
          this.staleMessagesDropped += 1;
          return;
        }
        this.pending.delete(invocationId);
        if (pending.timer)
          clearTimeout(pending.timer);
        const result = msg.result;
        if (this.oversized(result)) {
          pending.reject(new WorkerProtocolError("OVERSIZED_PAYLOAD", "result exceeds 1 MiB"));
          return;
        }
        pending.resolve(result);
        return;
      }
      case "log": {
        if (msg.sessionId !== this.sessionId) {
          this.staleMessagesDropped += 1;
          return;
        }
        const invocationId = msg.invocationId;
        const pending = this.pending.get(invocationId);
        if (!pending) {
          this.staleMessagesDropped += 1;
          return;
        }
        pending.logLines += msg.lines.length;
        if (pending.logLines > MAX_LOG_LINES_PER_INVOCATION) {
          this.hardTerminate(`log volume exceeded ${MAX_LOG_LINES_PER_INVOCATION} lines`);
        }
        return;
      }
      case "fatal": {
        if (msg.sessionId !== this.sessionId)
          return;
        this.hardTerminate(typeof msg.detail === "string" ? msg.detail : "worker reported fatal");
        return;
      }
      default:
        return;
    }
  }
  oversized(result) {
    try {
      return JSON.stringify(result).length > MAX_RESULT_BYTES;
    } catch {
      return true;
    }
  }
}
// packages/browser-runtime/src/kv.ts
var MAX_KV_VALUE_BYTES = 1024 * 1024;
// packages/browser-runtime/src/capabilities.ts
var BROWSER_TIMERS_ID = "runtime:timers";
var BROWSER_CRYPTO_ID = "runtime:crypto";
var BROWSER_CONSOLE_ID = "runtime:console";
var BROWSER_FETCH_ID = "runtime:fetch";
var BROWSER_ABORT_ID = "runtime:abort";
var BROWSER_TEXT_ID = "runtime:text";
var BROWSER_URL_ID = "runtime:url";
var BROWSER_CAPABILITY_VERSION = 1;

class BrowserCapabilityError extends Error {
  code;
  constructor(code, message) {
    super(`[@velqu/browser-runtime:capability:${code}] ${message}`);
    this.name = "BrowserCapabilityError";
    this.code = code;
  }
}

class TimerDelayTooLarge extends BrowserCapabilityError {
  constructor(ms, maxMs) {
    super("timer-delay-too-large", `delay ${ms}ms exceeds the ${maxMs}ms ceiling`);
    this.name = "TimerDelayTooLarge";
  }
}

class TimerCancelled extends BrowserCapabilityError {
  constructor() {
    super("timer-cancelled", "timer cancelled before firing; work discarded");
    this.name = "TimerCancelled";
  }
}

class CryptoSemanticsMismatch extends BrowserCapabilityError {
  constructor(detail) {
    super("crypto-semantics-mismatch", detail);
    this.name = "CryptoSemanticsMismatch";
  }
}

class ConsoleLimitExceeded extends BrowserCapabilityError {
  constructor(detail) {
    super("console-limit-exceeded", detail);
    this.name = "ConsoleLimitExceeded";
  }
}

class FetchPolicyDenied extends BrowserCapabilityError {
  constructor(detail) {
    super("fetch-policy-denied", detail);
    this.name = "FetchPolicyDenied";
  }
}

class FetchLimitExceeded extends BrowserCapabilityError {
  constructor(detail) {
    super("fetch-limit-exceeded", detail);
    this.name = "FetchLimitExceeded";
  }
}
var MAX_BROWSER_TIMER_DELAY_MS = 300000;
var MAX_CONSOLE_MSG_LEN = 16384;
var MAX_CONSOLE_ARGS = 32;
var MAX_CONSOLE_RECORDS = 64;
var MAX_FETCH_DEADLINE_MS = 300000;
var DEFAULT_FETCH_DEADLINE_MS = 30000;
var MAX_FETCH_REQUEST_BODY_BYTES = 16 * 1024 * 1024;
var MAX_FETCH_RESPONSE_BODY_BYTES = 16 * 1024 * 1024;
var ALLOWED_SCHEMES = ["http", "https"];
var MAX_GET_RANDOM_VALUES_BYTES = 65536;
var DIGEST_ALGORITHMS = ["SHA-256", "SHA-384", "SHA-512"];
var AUTH_PREFIXES = ["Bearer ", "bearer ", "Basic ", "basic "];
var SECRET_PREFIXES = ["sk-live-", "sk-test-", "ghp_", "gho_", "glpat-"];
var KEY_SEPARATORS = [
  ["password", "="],
  ["password", ":"],
  ["secret", "="],
  ["secret", ":"],
  ["api_key", "="],
  ["api_key", ":"],
  ["apikey", "="],
  ["apikey", ":"],
  ["auth_token", "="],
  ["auth_token", ":"],
  ["token", "="],
  ["token", ":"],
  ["authorization", "="],
  ["authorization", ":"],
  ["cookie", "="],
  ["cookie", ":"]
];
function redactSensitiveText(text) {
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
        while (i < len && !/\s/.test(text[i]) && !`,;"'`.includes(text[i]))
          i++;
        matched = true;
        break;
      }
    }
    if (matched)
      continue;
    for (const prefix of SECRET_PREFIXES) {
      if (remainder.startsWith(prefix)) {
        out += prefix + "[REDACTED]";
        i += prefix.length;
        while (i < len && /[A-Za-z0-9_-]/.test(text[i]))
          i++;
        matched = true;
        break;
      }
    }
    if (matched)
      continue;
    let keyHit = null;
    for (const [key, sep] of KEY_SEPARATORS) {
      if (remainder.length > key.length + sep.length && remainder.slice(0, key.length).toLowerCase() === key && remainder.slice(key.length, key.length + sep.length) === sep) {
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
      while (i < len && /[A-Za-z0-9_./+=-]/.test(text[i]))
        i++;
      if (inQuote && i < len && (text[i] === '"' || text[i] === "'"))
        i++;
      continue;
    }
    out += text[i];
    i++;
  }
  return out;
}
function ringBufferSink(capacity = 128) {
  const records = [];
  return {
    push(record) {
      records.push(record);
      if (records.length > capacity)
        records.shift();
    },
    records: () => [...records]
  };
}
function formatArgs(args) {
  const parts = args.map((a) => {
    if (typeof a === "string")
      return a;
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
function createConsoleCapability(options) {
  let count = 0;
  const emit = (level) => (...args) => {
    if (args.length > MAX_CONSOLE_ARGS) {
      throw new ConsoleLimitExceeded(`${args.length} args exceeds the ${MAX_CONSOLE_ARGS}-arg ceiling`);
    }
    count += 1;
    if (count > MAX_CONSOLE_RECORDS) {
      throw new ConsoleLimitExceeded(`record budget exhausted (${MAX_CONSOLE_RECORDS} records); further logging refused`);
    }
    const { message, truncated } = formatArgs(args);
    const correlationId = options.correlationId?.();
    options.sink.push({ level, message, truncated, ...correlationId ? { correlationId } : {} });
  };
  const consoleShaped = {
    debug: emit("debug"),
    info: emit("info"),
    warn: emit("warn"),
    error: emit("error"),
    log: emit("info")
  };
  return {
    console: Object.freeze(consoleShaped),
    descriptors: [{ id: BROWSER_CONSOLE_ID, version: BROWSER_CAPABILITY_VERSION, name: "bounded console" }]
  };
}
function createTimerCapability(options) {
  const maxDelayMs = Math.min(options?.maxDelayMs ?? MAX_BROWSER_TIMER_DELAY_MS, MAX_BROWSER_TIMER_DELAY_MS);
  const delay = (msOrInput, signalArg) => {
    const ms = typeof msOrInput === "number" ? msOrInput : msOrInput.ms;
    const signal = typeof msOrInput === "number" ? signalArg : msOrInput.signal;
    if (typeof ms !== "number" || !Number.isFinite(ms) || ms < 0) {
      return Promise.reject(new BrowserCapabilityError("timer-invalid-delay", `delay must be a finite ms >= 0, got ${String(ms)}`));
    }
    if (ms > maxDelayMs) {
      return Promise.reject(new TimerDelayTooLarge(ms, maxDelayMs));
    }
    if (signal?.aborted) {
      return Promise.reject(new TimerCancelled);
    }
    return new Promise((resolve, reject) => {
      const started = Date.now();
      let settled = false;
      const timer = setTimeout(() => {
        if (settled)
          return;
        settled = true;
        if (signal)
          signal.removeEventListener("abort", onAbort);
        resolve({ elapsedMs: Date.now() - started });
      }, ms);
      const onAbort = () => {
        if (settled)
          return;
        settled = true;
        clearTimeout(timer);
        reject(new TimerCancelled);
      };
      signal?.addEventListener("abort", onAbort, { once: true });
    });
  };
  return {
    timers: Object.freeze({ delay }),
    descriptors: [{ id: BROWSER_TIMERS_ID, version: BROWSER_CAPABILITY_VERSION, name: "bounded browser timer" }]
  };
}
function toHex(bytes) {
  let hex = "";
  for (const b of bytes)
    hex += b.toString(16).padStart(2, "0");
  return hex;
}
function createCryptoCapability() {
  const call = async (input) => {
    if (input.op === "getRandomValues") {
      if (!Number.isInteger(input.length) || input.length < 0 || input.length > MAX_GET_RANDOM_VALUES_BYTES) {
        throw new CryptoSemanticsMismatch(`getRandomValues length ${input.length} outside 0..=${MAX_GET_RANDOM_VALUES_BYTES}`);
      }
      return crypto.getRandomValues(new Uint8Array(input.length));
    }
    if (input.op === "digest") {
      if (!DIGEST_ALGORITHMS.includes(input.algorithm)) {
        throw new CryptoSemanticsMismatch(`digest algorithm "${input.algorithm}" is not pinned for browser/native parity (allowed: ${DIGEST_ALGORITHMS.join(", ")})`);
      }
      const digest = await crypto.subtle.digest(input.algorithm, input.data);
      const bytes = new Uint8Array(digest);
      return input.encoding === "bytes" ? bytes : toHex(bytes);
    }
    throw new CryptoSemanticsMismatch(`unknown crypto op "${input.op}"`);
  };
  return {
    crypto: Object.freeze({
      call,
      digestHex: (algorithm, data) => call({ op: "digest", algorithm, data }),
      getRandomValues: (length) => call({ op: "getRandomValues", length })
    }),
    descriptors: [{ id: BROWSER_CRYPTO_ID, version: BROWSER_CAPABILITY_VERSION, name: "pinned WebCrypto subset" }]
  };
}
function createFetchCapability(options) {
  const allowedOrigins = options.policy.allowedOrigins.map((o) => o.replace(/\/+$/, ""));
  const allowedMethods = options.policy.allowedMethods ?? ["GET", "HEAD"];
  const deadlineMs = Math.min(options.policy.deadlineMs ?? DEFAULT_FETCH_DEADLINE_MS, MAX_FETCH_DEADLINE_MS);
  const maxRequestBytes = options.policy.maxRequestBytes ?? MAX_FETCH_REQUEST_BODY_BYTES;
  const maxResponseBytes = options.policy.maxResponseBytes ?? MAX_FETCH_RESPONSE_BODY_BYTES;
  const doFetch = options.fetchImpl ?? fetch;
  const call = async (input) => {
    let url;
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
      throw new FetchPolicyDenied(`origin "${origin}" is not in the allowlist (default deny; allowed: ${allowedOrigins.length} origin(s))`);
    }
    const method = (input.method ?? "GET").toUpperCase();
    if (!allowedMethods.includes(method)) {
      throw new FetchPolicyDenied(`method "${method}" not allowed (${allowedMethods.join("/")})`);
    }
    if (input.body && input.body.byteLength > maxRequestBytes) {
      throw new FetchLimitExceeded(`request body ${input.body.byteLength}B exceeds ${maxRequestBytes}B`);
    }
    const headers = new Headers(input.headers);
    headers.delete("cookie");
    headers.delete("authorization");
    const deadlineController = new AbortController;
    const deadline = new Promise((_, reject) => {
      deadlineController.signal.addEventListener("abort", () => {
        reject(new FetchLimitExceeded(`deadline ${deadlineMs}ms exceeded`));
      }, { once: true });
    });
    const deadlineTimer = setTimeout(() => deadlineController.abort(), deadlineMs);
    const request = new Request(url.href, {
      method,
      headers,
      ...input.body ? { body: input.body } : {},
      credentials: "omit",
      redirect: "error",
      signal: deadlineController.signal
    });
    let response;
    try {
      response = await Promise.race([doFetch(request), deadline]);
    } catch (cause) {
      if (cause instanceof FetchLimitExceeded)
        throw cause;
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
    if (response.status >= 300 && response.status < 400) {
      throw new FetchPolicyDenied(`redirect status ${response.status} denied by policy`);
    }
    const reader = response.body?.getReader();
    const chunks = [];
    let total = 0;
    let truncated = false;
    if (reader) {
      for (;; ) {
        const { done, value } = await reader.read();
        if (done)
          break;
        total += value.byteLength;
        if (total > maxResponseBytes) {
          truncated = true;
          reader.cancel();
          break;
        }
        chunks.push(value);
      }
    }
    const bodyBytes = new Uint8Array(Math.min(total, maxResponseBytes));
    let offset = 0;
    for (const chunk of chunks) {
      bodyBytes.set(chunk.subarray(0, Math.min(chunk.byteLength, bodyBytes.byteLength - offset)), offset);
      offset += chunk.byteLength;
      if (offset >= bodyBytes.byteLength)
        break;
    }
    const responseHeaders = {};
    response.headers.forEach((v, k) => {
      responseHeaders[k] = v;
    });
    return Object.freeze({
      status: response.status,
      headers: Object.freeze(responseHeaders),
      bodyBytes,
      truncated
    });
  };
  return {
    fetch: Object.freeze({
      call,
      policySummary: () => `default-deny origins (allowlist: ${allowedOrigins.length}), methods: ${allowedMethods.join("/")}, ` + `deadline ≤ ${deadlineMs}ms, bodies ≤ ${maxRequestBytes}/${maxResponseBytes}B, credentials: omit, redirects: denied`
    }),
    descriptors: [{ id: BROWSER_FETCH_ID, version: BROWSER_CAPABILITY_VERSION, name: "restricted outbound fetch" }]
  };
}
function createBrowserCapabilityGraph(options) {
  const timer = createTimerCapability({ maxDelayMs: options.timerMaxDelayMs });
  const cryptoCap = createCryptoCapability();
  const consoleCap = createConsoleCapability({
    sink: options.consoleSink,
    correlationId: options.consoleCorrelationId
  });
  const fetchCap = createFetchCapability({ policy: options.fetchPolicy, fetchImpl: options.fetchImpl });
  const abort = Object.freeze({
    AbortController: globalThis.AbortController,
    AbortSignal: globalThis.AbortSignal
  });
  const text = Object.freeze({
    TextEncoder: globalThis.TextEncoder,
    TextDecoder: globalThis.TextDecoder
  });
  const url = Object.freeze({ URL: globalThis.URL, URLSearchParams: globalThis.URLSearchParams });
  const descriptors = [
    ...timer.descriptors,
    ...cryptoCap.descriptors,
    ...consoleCap.descriptors,
    ...fetchCap.descriptors,
    { id: BROWSER_ABORT_ID, version: BROWSER_CAPABILITY_VERSION, name: "abort primitives" },
    { id: BROWSER_TEXT_ID, version: BROWSER_CAPABILITY_VERSION, name: "text encoding" },
    { id: BROWSER_URL_ID, version: BROWSER_CAPABILITY_VERSION, name: "url parsing" }
  ];
  return {
    graph: Object.freeze({
      timers: timer.timers,
      crypto: cryptoCap.crypto,
      console: consoleCap.console,
      fetch: fetchCap.fetch,
      abort,
      text,
      url
    }),
    descriptors,
    describe: () => descriptors.map((d) => `${d.id}@${d.version} (${d.name})`).join("; ")
  };
}
// packages/browser-runtime/src/artifact-loader.ts
var ARTIFACT_MANIFEST_VERSION = 1;
class ArtifactManifestError extends Error {
  artifact;
  reason;
  constructor(artifact, reason, detail) {
    super(`[@velqu/browser-runtime:artifact:${reason}] ${artifact}: ${detail}`);
    this.name = "ArtifactManifestError";
    this.artifact = artifact;
    this.reason = reason;
  }
}
async function sha256Hex(bytes) {
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return [...new Uint8Array(digest)].map((b) => b.toString(16).padStart(2, "0")).join("");
}
function canonicalManifestJson(manifest) {
  const roles = Object.keys(manifest.artifacts).sort();
  const ordered = {
    formatVersion: manifest.formatVersion,
    target: manifest.target,
    handlerAbiVersion: manifest.handlerAbiVersion,
    kernelAbiVersion: manifest.kernelAbiVersion,
    ...Array.isArray(manifest.capabilities) ? {
      capabilities: [...manifest.capabilities].map((c) => ({ id: c.id, version: c.version })).sort((a, b) => a.id < b.id ? -1 : a.id > b.id ? 1 : a.version - b.version)
    } : {},
    appId: manifest.appId,
    packSha256: manifest.packSha256,
    artifacts: Object.fromEntries(roles.map((role) => {
      const a = manifest.artifacts[role];
      return [role, { mediaType: a.mediaType, sha256: a.sha256, bytes: a.bytes, url: a.url }];
    }))
  };
  return JSON.stringify(ordered);
}
async function buildIdOf(manifest) {
  return sha256Hex(new TextEncoder().encode(canonicalManifestJson(manifest)));
}
async function loadArtifacts(manifestJson, readArtifact) {
  let manifest;
  try {
    manifest = JSON.parse(manifestJson);
  } catch {
    throw new ArtifactManifestError("browser-manifest.json", "tampered", "manifest is not valid JSON");
  }
  if (manifest.formatVersion !== ARTIFACT_MANIFEST_VERSION) {
    throw new ArtifactManifestError("browser-manifest.json", "unsupported-version", `manifest formatVersion ${String(manifest.formatVersion)} unsupported (loader implements ${ARTIFACT_MANIFEST_VERSION})`);
  }
  if (manifest.target !== "browser-wasm") {
    throw new ArtifactManifestError("browser-manifest.json", "unsupported-version", `manifest target ${String(manifest.target)} is not browser-wasm`);
  }
  const { artifacts, buildId, ...rest } = manifest;
  const recomputed = await buildIdOf({ ...rest, artifacts });
  if (recomputed !== buildId) {
    throw new ArtifactManifestError("browser-manifest.json", "tampered", `manifest buildId mismatch (declared ${buildId.slice(0, 12)}…, canonical ${recomputed.slice(0, 12)}…)`);
  }
  const bytes = {};
  for (const role of Object.keys(manifest.artifacts)) {
    const entry = manifest.artifacts[role];
    let raw;
    try {
      raw = await readArtifact(entry.url);
    } catch {
      throw new ArtifactManifestError(role, "missing", `artifact file missing at ${entry.url}`);
    }
    if (raw.byteLength !== entry.bytes) {
      throw new ArtifactManifestError(role, "truncated", `expected ${entry.bytes} bytes, read ${raw.byteLength}`);
    }
    const digest = await sha256Hex(raw);
    if (digest !== entry.sha256) {
      throw new ArtifactManifestError(role, "tampered", `sha256 mismatch (expected ${entry.sha256.slice(0, 12)}…, got ${digest.slice(0, 12)}…)`);
    }
    bytes[role] = raw;
  }
  if (bytes.pack) {
    const packDigest = await sha256Hex(bytes.pack);
    if (packDigest !== manifest.packSha256) {
      throw new ArtifactManifestError("pack", "cross-build", "pack bytes do not match the manifest's packSha256 (mixed deployment)");
    }
  }
  return { buildId, manifest, bytes };
}

// packages/browser-runtime/src/service-worker.ts
function resolveAgainst(baseUrl, url) {
  const base = baseUrl.replace(/\/+$/, "");
  return `${base}/${url.replace(/^\/+/, "")}`;
}
async function bootstrapServiceWorker(options) {
  const nav = globalThis;
  if (!nav.navigator?.serviceWorker) {
    return {
      kind: "injected-fetch-fallback",
      reason: "ServiceWorker is unavailable in this environment — use the injected-fetch fallback (runtime.fetch directly in the page); see UNSUPPORTED_SEMANTICS"
    };
  }
  try {
    const registration = await nav.navigator.serviceWorker.register(options.scriptUrl, {
      scope: options.scope
    });
    return { kind: "service-worker", scope: options.scope, registration };
  } catch (cause) {
    return {
      kind: "injected-fetch-fallback",
      reason: `ServiceWorker registration failed: ${String(cause)}`
    };
  }
}
async function loadArtifactsWithFallback(env) {
  let networkError;
  try {
    const response = await env.fetch(env.manifestUrl);
    if (!response.ok) {
      throw new Error(`manifest fetch → HTTP ${response.status}`);
    }
    const loaded = await loadArtifacts(await response.text(), async (url) => new Uint8Array(await (await env.fetch(resolveAgainst(env.baseUrl, url))).arrayBuffer()));
    return { source: "network", ...loaded };
  } catch (cause) {
    networkError = cause;
  }
  const prefix = `velqu:${env.appId}:`;
  const candidates = (await env.caches.keys()).filter((n) => n.startsWith(prefix)).sort().reverse();
  for (const name of candidates) {
    const cache = await env.caches.open(name);
    if (!cache)
      continue;
    const manifestResponse = await cache.match(env.manifestUrl);
    if (!manifestResponse)
      continue;
    try {
      const loaded = await loadArtifacts(await manifestResponse.text(), async (url) => {
        const hit = await cache.match(resolveAgainst(env.baseUrl, url));
        if (!hit)
          throw new Error(`cache miss: ${url}`);
        return new Uint8Array(await hit.arrayBuffer());
      });
      return { source: "cache", ...loaded };
    } catch {}
  }
  throw networkError instanceof Error ? networkError : new Error("no verified cached build available (last-known-good fallback exhausted)");
}

// packages/browser-runtime/src/index.ts
class BrowserRuntimeError extends Error {
  code;
  status;
  problem;
  constructor(code, message, options) {
    super(`[@velqu/browser-runtime:${code}] ${message}`, { cause: options?.cause });
    this.name = "BrowserRuntimeError";
    this.code = code;
    this.status = options?.status ?? 500;
    this.problem = options?.problem;
  }
}
function parseKernelMessage(raw, what) {
  let parsed;
  try {
    parsed = JSON.parse(raw);
  } catch (cause) {
    throw new BrowserRuntimeError("KERNEL_PROTOCOL", `kernel ${what} was not valid JSON`, {
      cause
    });
  }
  return parsed;
}
function isProblem(x) {
  return typeof x === "object" && x !== null && x.kind === "problem" && typeof x.problem === "object";
}
function createBrowserRuntime(options) {
  const expectedAbi = options.expectedAbiVersion ?? 1;
  const kernelAbi = options.kernel.kernel_abi_version();
  if (kernelAbi !== expectedAbi) {
    throw new BrowserRuntimeError("KERNEL_ABI_MISMATCH", `kernel reports ABI ${kernelAbi}, runtime contract expects ${expectedAbi}`);
  }
  let instance;
  try {
    instance = new options.kernel(options.packBytes);
  } catch (cause) {
    let problem;
    if (cause instanceof Error) {
      try {
        const parsed = JSON.parse(cause.message);
        if (typeof parsed?.problemId === "string")
          problem = parsed;
      } catch {
        problem = undefined;
      }
    }
    throw new BrowserRuntimeError("ARTIFACT_REJECTED", "kernel rejected the artifact", {
      problem,
      cause
    });
  }
  let state = "ready";
  const requireReady = () => {
    if (state === "disposed") {
      throw new BrowserRuntimeError("RUNTIME_DISPOSED", "fetch on a disposed runtime");
    }
  };
  return {
    get state() {
      return state;
    },
    abiVersion: kernelAbi,
    fetch(request) {
      requireReady();
      return dispatchFetchRequest(instance, kernelAbi, request, {
        maxBodyBytes: options.maxBodyBytes ?? DEFAULT_MAX_BODY_BYTES,
        signal: options.signal ?? null,
        ...options.executeHandler ? { executeHandler: options.executeHandler } : {}
      }).catch((cause) => {
        if (cause instanceof BrowserRuntimeError)
          throw cause;
        if (cause instanceof DispatcherError) {
          const code = cause.rejection.kind === "protocol" ? "KERNEL_PROTOCOL" : cause.rejection.kind === "unsupported" ? "REQUEST_INVALID" : "REQUEST_INVALID";
          throw new BrowserRuntimeError(code, cause.message, { cause });
        }
        if (cause instanceof DOMException && cause.name === "AbortError")
          throw cause;
        throw new BrowserRuntimeError("KERNEL_PROTOCOL", String(cause), { cause });
      });
    },
    authorizeCapability(name) {
      requireReady();
      const raw = instance.authorize_capability(name);
      const parsed = parseKernelMessage(raw, "capability authorization");
      if (isProblem(parsed))
        return parsed.problem;
      if (typeof parsed.authorized !== "boolean") {
        throw new BrowserRuntimeError("KERNEL_PROTOCOL", "capability authorization returned neither a decision nor a problem");
      }
      return { authorized: parsed.authorized };
    },
    dispose() {
      if (state === "disposed")
        return;
      state = "disposed";
      instance.dispose();
    }
  };
}

// conformance/browser/fixture-app/dist/browser/kernel.js
var exports = {};

class WasmKernel {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    WasmKernelFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_wasmkernel_free(ptr, 0);
  }
  authorize_capability(name) {
    let deferred2_0;
    let deferred2_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      const ptr0 = passStringToWasm0(name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
      const len0 = WASM_VECTOR_LEN;
      wasm.wasmkernel_authorize_capability(retptr, this.__wbg_ptr, ptr0, len0);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      deferred2_0 = r0;
      deferred2_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_export3(deferred2_0, deferred2_1, 1);
    }
  }
  complete_invocation(completion_json) {
    let deferred2_0;
    let deferred2_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      const ptr0 = passStringToWasm0(completion_json, wasm.__wbindgen_export, wasm.__wbindgen_export2);
      const len0 = WASM_VECTOR_LEN;
      wasm.wasmkernel_complete_invocation(retptr, this.__wbg_ptr, ptr0, len0);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      deferred2_0 = r0;
      deferred2_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_export3(deferred2_0, deferred2_1, 1);
    }
  }
  dispose() {
    const ptr = this.__destroy_into_raw();
    wasm.wasmkernel_dispose(ptr);
  }
  constructor(pack_bytes) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      const ptr0 = passArray8ToWasm0(pack_bytes, wasm.__wbindgen_export);
      const len0 = WASM_VECTOR_LEN;
      wasm.wasmkernel_new(retptr, ptr0, len0);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
      if (r2) {
        throw takeObject(r1);
      }
      this.__wbg_ptr = r0 >>> 0;
      WasmKernelFinalization.register(this, this.__wbg_ptr, this);
      return this;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  plan_request(request_json) {
    let deferred2_0;
    let deferred2_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      const ptr0 = passStringToWasm0(request_json, wasm.__wbindgen_export, wasm.__wbindgen_export2);
      const len0 = WASM_VECTOR_LEN;
      wasm.wasmkernel_plan_request(retptr, this.__wbg_ptr, ptr0, len0);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      deferred2_0 = r0;
      deferred2_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_export3(deferred2_0, deferred2_1, 1);
    }
  }
}
if (Symbol.dispose)
  WasmKernel.prototype[Symbol.dispose] = WasmKernel.prototype.free;
exports.WasmKernel = WasmKernel;
function kernel_abi_version() {
  const ret = wasm.kernel_abi_version();
  return ret >>> 0;
}
exports.kernel_abi_version = kernel_abi_version;
function __wbg_get_imports() {
  const import0 = {
    __proto__: null,
    __wbg_Error_8c4e43fe74559d73: function(arg0, arg1) {
      const ret = Error(getStringFromWasm0(arg0, arg1));
      return addHeapObject(ret);
    },
    __wbg___wbindgen_throw_be289d5034ed271b: function(arg0, arg1) {
      throw new Error(getStringFromWasm0(arg0, arg1));
    }
  };
  return {
    __proto__: null,
    "./q_browser_kernel_bg.js": import0
  };
}
var WasmKernelFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {}, unregister: () => {} } : new FinalizationRegistry((ptr) => wasm.__wbg_wasmkernel_free(ptr >>> 0, 1));
function addHeapObject(obj) {
  if (heap_next === heap.length)
    heap.push(heap.length + 1);
  const idx = heap_next;
  heap_next = heap[idx];
  heap[idx] = obj;
  return idx;
}
function dropObject(idx) {
  if (idx < 132)
    return;
  heap[idx] = heap_next;
  heap_next = idx;
}
var cachedDataViewMemory0 = null;
function getDataViewMemory0() {
  if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer) {
    cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
  }
  return cachedDataViewMemory0;
}
function getStringFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return decodeText(ptr, len);
}
var cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
  if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
    cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8ArrayMemory0;
}
function getObject(idx) {
  return heap[idx];
}
var heap = new Array(128).fill(undefined);
heap.push(undefined, null, true, false);
var heap_next = heap.length;
function passArray8ToWasm0(arg, malloc) {
  const ptr = malloc(arg.length * 1, 1) >>> 0;
  getUint8ArrayMemory0().set(arg, ptr / 1);
  WASM_VECTOR_LEN = arg.length;
  return ptr;
}
function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === undefined) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr2 = malloc(buf.length, 1) >>> 0;
    getUint8ArrayMemory0().subarray(ptr2, ptr2 + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr2;
  }
  let len = arg.length;
  let ptr = malloc(len, 1) >>> 0;
  const mem = getUint8ArrayMemory0();
  let offset = 0;
  for (;offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 127)
      break;
    mem[ptr + offset] = code;
  }
  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
    const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
    const ret = cachedTextEncoder.encodeInto(arg, view);
    offset += ret.written;
    ptr = realloc(ptr, len, offset, 1) >>> 0;
  }
  WASM_VECTOR_LEN = offset;
  return ptr;
}
function takeObject(idx) {
  const ret = getObject(idx);
  dropObject(idx);
  return ret;
}
var cachedTextDecoder = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
function decodeText(ptr, len) {
  return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}
var cachedTextEncoder = new TextEncoder;
if (!("encodeInto" in cachedTextEncoder)) {
  cachedTextEncoder.encodeInto = function(arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
      read: arg.length,
      written: buf.length
    };
  };
}
var WASM_VECTOR_LEN = 0;
var wasm = null;
function initKernelSync(verifiedBytes) {
  if (wasm)
    throw new Error("[velqu:kernel] kernel already initialized");
  wasm = new WebAssembly.Instance(new WebAssembly.Module(verifiedBytes), __wbg_get_imports()).exports;
}

// conformance/browser/fixture-app/dist/browser/page.js
var base = new URL(".", new URL(import.meta.url));
var statusEl = document.getElementById("velqu-status");
var setStatus = (text) => {
  if (statusEl)
    statusEl.textContent = text;
};
try {
  let VelquKernel = function(packBytes) {
    return new WasmKernel(packBytes);
  };
  const capabilities = createBrowserCapabilityGraph({
    consoleSink: ringBufferSink(128),
    fetchPolicy: { allowedOrigins: [] }
  });
  const capabilityLog = capabilities.graph.console;
  setStatus("loading verified artifacts…");
  const loaded = await loadArtifactsWithFallback({
    fetch: (url) => fetch(url),
    caches,
    manifestUrl: new URL("velqu-artifacts.json", base).href,
    baseUrl: base.href,
    appId: "app"
  });
  setStatus("initializing kernel…");
  initKernelSync(loaded.bytes.kernelWasm);
  const sessionId = crypto.randomUUID();
  const host = new WorkerHost(sessionId, () => {
    const worker = new Worker(new URL("worker.js", base).href, { type: "module" });
    worker.postMessage({ type: "velqu-session", sessionId });
    return worker;
  });
  VelquKernel.kernel_abi_version = kernel_abi_version;
  const runtime = createBrowserRuntime({
    packBytes: loaded.bytes.pack,
    kernel: VelquKernel,
    executeHandler: (plan) => host.execute(plan)
  });
  const probeRes = null;
  const probeBody = null;
  const swOutcome = await bootstrapServiceWorker({
    scriptUrl: new URL("service-worker.js", base).href,
    scope: "/"
  });
  setStatus("ready — build " + loaded.buildId.slice(0, 12) + " (" + loaded.source + ")" + " · worker " + runtime.state + " · sw: " + swOutcome.kind);
  const info = document.getElementById("velqu-info");
  if (info) {
    info.textContent = JSON.stringify({
      buildId: loaded.buildId,
      source: loaded.source,
      appId: loaded.manifest.appId,
      handlerAbiVersion: loaded.manifest.handlerAbiVersion,
      kernelAbiVersion: loaded.manifest.kernelAbiVersion,
      serviceWorker: swOutcome.kind,
      capabilities: capabilities.descriptors,
      probe: null,
      reason: swOutcome.kind === "injected-fetch-fallback" ? swOutcome.reason : undefined
    }, null, 2);
  }
} catch (cause) {
  setStatus("failed: " + String(cause && cause.message ? cause.message : cause));
}
