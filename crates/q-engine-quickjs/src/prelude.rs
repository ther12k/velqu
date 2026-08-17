//! JS prelude installed before the application bundle evaluates.
//! Defines the handler registration protocol and the lazy request context.

pub const PRELUDE: &str = r#"
"use strict";
globalThis.__velquHandlers = Object.create(null);
globalThis.__velquRegister = function (id, fn) {
  if (typeof id !== "string" || id.length === 0) throw new TypeError("__velquRegister: id must be a non-empty string");
  if (typeof fn !== "function") throw new TypeError("__velquRegister: handler must be a function: " + id);
  if (id in globalThis.__velquHandlers) throw new Error("__velquRegister: duplicate handler id " + id);
  globalThis.__velquHandlers[id] = fn;
};

// Lazy request handle: fields materialize on first access through the native
// bridge (slot, generation are validated by the host; expired handles throw).
// Native accessors return JSON strings; objects are built engine-side.
globalThis.__velquMakeReq = function (slot, gen) {
  const req = {};
  let headers, params, query;
  Object.defineProperty(req, "headers", { enumerable: true, get() { return (headers ??= JSON.parse(globalThis.__velquReqRaw(slot, gen, "headers"))); } });
  Object.defineProperty(req, "params", { enumerable: true, get() { return (params ??= JSON.parse(globalThis.__velquReqRaw(slot, gen, "params"))); } });
  Object.defineProperty(req, "query", { enumerable: true, get() { return (query ??= JSON.parse(globalThis.__velquReqRaw(slot, gen, "query"))); } });
  return req;
};

// ctx: pre.* are host-validated values (native strategy) or undefined for lazy access.
globalThis.__velquMakeCtx = function (slot, gen, pre) {
  const c = {};
  const lazy = (key, fn) => {
    let v, used = false;
    Object.defineProperty(c, key, { enumerable: true, get() { if (!used) { v = fn(); used = true; } return v; } });
  };
  if (pre.params != null) c.params = pre.params; else lazy("params", () => JSON.parse(globalThis.__velquReqRaw(slot, gen, "params")));
  if (pre.query != null) c.query = pre.query; else lazy("query", () => JSON.parse(globalThis.__velquReqRaw(slot, gen, "query")));
  if (pre.headers != null) c.headers = pre.headers; else lazy("headers", () => JSON.parse(globalThis.__velquReqRaw(slot, gen, "headers")));
  if (pre.body !== undefined && pre.body !== null) {
    c.body = pre.body; // native body strategy: already parsed + validated
  } else {
    c.json = () => JSON.parse(globalThis.__velquReqBodyText(slot, gen));
    c.text = () => globalThis.__velquReqBodyText(slot, gen);
    c.bytes = () => {
      const len = globalThis.__velquReqBodyLen(slot, gen);
      const u = new Uint8Array(len);
      if (len > 0) globalThis.__velquFillBytes(slot, gen, u);
      return u;
    };
  }
  c.native = { timer: { delay: (ms) => globalThis.__velquTimerP(ms) } };
  return c;
};

// Timer capability: promise callbacks live in a JS-side op table keyed by
// op id; the host resolves/rejects through the two dispatch functions below.
globalThis.__velquOps = Object.create(null);
globalThis.__velquTimerP = function (ms) {
  return new Promise((resolve, reject) => {
    const opId = globalThis.__velquTimerStart(ms);
    globalThis.__velquOps[opId] = { resolve, reject };
  });
};
globalThis.__velquOpResolve = function (opId, value) {
  const op = globalThis.__velquOps[opId];
  if (op) { delete globalThis.__velquOps[opId]; op.resolve(value); }
};
globalThis.__velquOpReject = function (opId, reason) {
  const op = globalThis.__velquOps[opId];
  if (op) { delete globalThis.__velquOps[opId]; op.reject(new Error(String(reason))); }
};

// Uniform policy+handler runner. Returns a promise that always settles exactly
// once, so host-side cancellation/timeouts cannot double-reply.
globalThis.__velquRun = async function (handlerFn, policyFn, ctx, req) {
  if (policyFn) {
    const r = await policyFn(req);
    if (r && r.__problem) return r;
    Object.defineProperty(ctx, "session", { value: r.session, enumerable: true });
  }
  return await handlerFn(ctx);
};

// Settlement watch: the host reads and clears this table after draining jobs.
// Exactly one entry per invocation id.
globalThis.__velquSettled = Object.create(null);
globalThis.__velquWatch = function (p, id) {
  const key = String(id);
  Promise.resolve(p).then(
    (v) => { globalThis.__velquSettled[key] = { ok: true, v }; },
    (e) => { globalThis.__velquSettled[key] = { ok: false, e }; }
  );
  return p;
};
"#;
