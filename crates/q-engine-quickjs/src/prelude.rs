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
// M24-008-A: shared prototypes stabilize hidden classes across requests. The
// request-specific lazy accessors remain an explicit fallback for fields whose
// route plan needs dynamic names; stale slot/generation checks stay native.
const __velquRequestPrototype = Object.create(null);
const __velquContextPrototype = Object.create(null);
globalThis.__velquRequestPrototype = __velquRequestPrototype;
globalThis.__velquContextPrototype = __velquContextPrototype;

// Explicit compatibility fallback; native lazy fields remain default path.
__velquContextPrototype.webRequest = function () {
  const slot = this.__velquSlot, gen = this.__velquGeneration;
  return {
    method: "GET",
    url: "" + slot,
    headers: JSON.parse(globalThis.__velquReqRaw(slot, gen, "headers")),
    query: JSON.parse(globalThis.__velquReqRaw(slot, gen, "query")),
    text: () => globalThis.__velquReqBodyText(slot, gen)
  };
};

// Console capability (M27-004-B): structured log methods with formatting.
function __velquFormatArgs(args) {
  var parts = [];
  for (var i = 0; i < args.length && i < 32; i++) {
    var a = args[i];
    if (typeof a === "object" && a !== null) {
      try { parts.push(JSON.stringify(a)); } catch (e) { parts.push(String(a)); }
    } else {
      parts.push(String(a));
    }
  }
  return parts.join(" ");
}

globalThis.console = {
  debug: function () { if (typeof globalThis.__velquConsoleLog === "function") globalThis.__velquConsoleLog("debug", __velquFormatArgs(arguments)); },
  log: function () { if (typeof globalThis.__velquConsoleLog === "function") globalThis.__velquConsoleLog("info", __velquFormatArgs(arguments)); },
  info: function () { if (typeof globalThis.__velquConsoleLog === "function") globalThis.__velquConsoleLog("info", __velquFormatArgs(arguments)); },
  warn: function () { if (typeof globalThis.__velquConsoleLog === "function") globalThis.__velquConsoleLog("warn", __velquFormatArgs(arguments)); },
  error: function () { if (typeof globalThis.__velquConsoleLog === "function") globalThis.__velquConsoleLog("error", __velquFormatArgs(arguments)); }
};

// URLSearchParams implementation (M27-005-A/D)
function URLSearchParams(init) {
  this._entries = [];
  var MAX_SEARCH_PARAMS_LEN = 16384;
  var MAX_SEARCH_PARAMS_COUNT = 1024;
  if (typeof init === "string") {
    var str = init.charAt(0) === "?" ? init.slice(1) : init;
    if (str.length > MAX_SEARCH_PARAMS_LEN) throw new RangeError("URLSearchParams input exceeds maximum length limit");
    if (str.length > 0) {
      var pairs = str.split("&");
      if (pairs.length > MAX_SEARCH_PARAMS_COUNT) throw new RangeError("URLSearchParams entry count exceeds maximum limit");
      for (var i = 0; i < pairs.length; i++) {
        var eq = pairs[i].indexOf("=");
        if (eq === -1) {
          this._entries.push([decodeURIComponent(pairs[i].split("+").join(" ")), ""]);
        } else {
          this._entries.push([
            decodeURIComponent(pairs[i].slice(0, eq).split("+").join(" ")),
            decodeURIComponent(pairs[i].slice(eq + 1).split("+").join(" "))
          ]);
        }
      }
    }
  } else if (Array.isArray(init)) {
    if (init.length > MAX_SEARCH_PARAMS_COUNT) throw new RangeError("URLSearchParams entry count exceeds maximum limit");
    for (var i = 0; i < init.length; i++) {
      this._entries.push([String(init[i][0]), String(init[i][1])]);
    }
  } else if (init && typeof init === "object") {
    var keys = Object.keys(init);
    if (keys.length > MAX_SEARCH_PARAMS_COUNT) throw new RangeError("URLSearchParams entry count exceeds maximum limit");
    for (var i = 0; i < keys.length; i++) {
      this._entries.push([keys[i], String(init[keys[i]])]);
    }
  }
}
URLSearchParams.prototype.append = function(name, value) {
  this._entries.push([String(name), String(value)]);
};
URLSearchParams.prototype.get = function(name) {
  var n = String(name);
  for (var i = 0; i < this._entries.length; i++) {
    if (this._entries[i][0] === n) return this._entries[i][1];
  }
  return null;
};
URLSearchParams.prototype.getAll = function(name) {
  var n = String(name), res = [];
  for (var i = 0; i < this._entries.length; i++) {
    if (this._entries[i][0] === n) res.push(this._entries[i][1]);
  }
  return res;
};
URLSearchParams.prototype.has = function(name, value) {
  var n = String(name);
  for (var i = 0; i < this._entries.length; i++) {
    if (this._entries[i][0] === n) {
      if (value === undefined || this._entries[i][1] === String(value)) return true;
    }
  }
  return false;
};
URLSearchParams.prototype.set = function(name, value) {
  var n = String(name), v = String(value), replaced = false, res = [];
  for (var i = 0; i < this._entries.length; i++) {
    if (this._entries[i][0] === n) {
      if (!replaced) { res.push([n, v]); replaced = true; }
    } else {
      res.push(this._entries[i]);
    }
  }
  if (!replaced) res.push([n, v]);
  this._entries = res;
};
URLSearchParams.prototype.delete = function(name, value) {
  var n = String(name);
  this._entries = this._entries.filter(function(e) {
    if (e[0] !== n) return true;
    return value !== undefined && e[1] !== String(value);
  });
};
URLSearchParams.prototype.sort = function() {
  this._entries.sort(function(a, b) {
    return a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0;
  });
};
URLSearchParams.prototype.toString = function() {
  var parts = [];
  for (var i = 0; i < this._entries.length; i++) {
    parts.push(encodeURIComponent(this._entries[i][0]) + "=" + encodeURIComponent(this._entries[i][1]));
  }
  return parts.join("&");
};
URLSearchParams.prototype.forEach = function(cb, thisArg) {
  for (var i = 0; i < this._entries.length; i++) {
    cb.call(thisArg, this._entries[i][1], this._entries[i][0], this);
  }
};
URLSearchParams.prototype.entries = function*() {
  for (var i = 0; i < this._entries.length; i++) yield [this._entries[i][0], this._entries[i][1]];
};
URLSearchParams.prototype.keys = function*() {
  for (var i = 0; i < this._entries.length; i++) yield this._entries[i][0];
};
URLSearchParams.prototype.values = function*() {
  for (var i = 0; i < this._entries.length; i++) yield this._entries[i][1];
};
URLSearchParams.prototype[Symbol.iterator] = URLSearchParams.prototype.entries;
globalThis.URLSearchParams = URLSearchParams;

// URL implementation (M27-005-A)
function URL(url, base) {
  if (typeof globalThis.__velquUrlParse !== "function") throw new TypeError("URL parser native unavailable");
  var raw = globalThis.__velquUrlParse(String(url), base !== undefined ? String(base) : undefined);
  var data = JSON.parse(raw);
  this.href = data.href;
  this.origin = data.origin;
  this.protocol = data.protocol;
  this.username = data.username;
  this.password = data.password;
  this.host = data.host;
  this.hostname = data.hostname;
  this.port = data.port;
  this.pathname = data.pathname;
  this.search = data.search;
  this.hash = data.hash;
  this.searchParams = new URLSearchParams(data.search);
}
URL.canParse = function(url, base) {
  try { new URL(url, base); return true; } catch (e) { return false; }
};
URL.prototype.toString = function() { return this.href; };
URL.prototype.toJSON = function() { return this.href; };
globalThis.URL = URL;

// TextEncoder implementation (M27-006-A)
function TextEncoder() {
  this.encoding = "utf-8";
}
TextEncoder.prototype.encode = function(input) {
  if (typeof globalThis.__velquTextEncodeLen !== "function" || typeof globalThis.__velquTextEncodeFill !== "function") {
    throw new TypeError("TextEncoder native unavailable");
  }
  var str = input !== undefined ? String(input) : "";
  var len = globalThis.__velquTextEncodeLen(str);
  var u = new Uint8Array(len);
  if (len > 0) globalThis.__velquTextEncodeFill(str, u);
  return u;
};
TextEncoder.prototype.encodeInto = function(source, destination) {
  if (typeof globalThis.__velquTextEncodeInto !== "function") throw new TypeError("TextEncoder native unavailable");
  var res = globalThis.__velquTextEncodeInto(String(source), destination);
  return { read: res[0], written: res[1] };
};
globalThis.TextEncoder = TextEncoder;

// TextDecoder implementation (M27-006-A)
function TextDecoder(label, options) {
  var enc = label !== undefined ? String(label).trim().toLowerCase() : "utf-8";
  if (enc !== "utf-8" && enc !== "utf8" && enc !== "unicode-1-1-utf-8") {
    throw new RangeError("The encoding label provided ('" + label + "') is invalid.");
  }
  this.encoding = "utf-8";
  this.fatal = options && options.fatal ? Boolean(options.fatal) : false;
  this.ignoreBOM = options && options.ignoreBOM ? Boolean(options.ignoreBOM) : false;
}
TextDecoder.prototype.decode = function(input, options) {
  if (typeof globalThis.__velquTextDecode !== "function") throw new TypeError("TextDecoder native unavailable");
  var buf;
  if (input === undefined) {
    buf = new Uint8Array(0);
  } else if (input instanceof Uint8Array) {
    buf = input;
  } else if (ArrayBuffer.isView(input)) {
    buf = new Uint8Array(input.buffer, input.byteOffset, input.byteLength);
  } else if (input instanceof ArrayBuffer) {
    buf = new Uint8Array(input);
  } else {
    throw new TypeError("Failed to execute 'decode' on 'TextDecoder': The provided value is not of type '(ArrayBuffer or ArrayBufferView)'");
  }
  return globalThis.__velquTextDecode(buf, this.fatal, this.ignoreBOM);
};
globalThis.TextDecoder = TextDecoder;

// AbortSignal implementation (M27-007-A)
function AbortSignal() {
  this._aborted = false;
  this._reason = undefined;
  this._listeners = [];
  this.onabort = null;
}
Object.defineProperty(AbortSignal.prototype, "aborted", {
  get: function() { return this._aborted; },
  enumerable: true
});
Object.defineProperty(AbortSignal.prototype, "reason", {
  get: function() { return this._reason; },
  enumerable: true
});
AbortSignal.prototype.addEventListener = function (type, listener, options) {
  if (type !== "abort" || typeof listener !== "function") return;
  if (this._listeners.length >= 1024) throw new RangeError("AbortSignal listener limit exceeded (1024)");
  var once = options && typeof options === "object" && options.once;
  var self = this;
  var wrap = once ? function (evt) { self.removeEventListener("abort", wrap); listener.call(self, evt); } : listener;
  wrap._orig = listener;
  if (this._aborted) {
    var evt = { type: "abort", target: self, currentTarget: self };
    if (typeof queueMicrotask === "function") {
      queueMicrotask(function () { wrap(evt); });
    } else {
      wrap(evt);
    }
    return;
  }
  this._listeners.push(wrap);
};
AbortSignal.prototype.removeEventListener = function (type, listener) {
  if (type !== "abort" || typeof listener !== "function") return;
  this._listeners = this._listeners.filter(function (l) { return l !== listener && l._orig !== listener; });
};
AbortSignal.prototype.dispatchEvent = function (evt) {
  if (evt && evt.type === "abort") {
    var ls = this._listeners.slice();
    this._listeners = [];
    for (var i = 0; i < ls.length; i++) {
      try { ls[i].call(this, evt); } catch (e) { if (console && console.error) console.error(e); }
    }
    if (typeof this.onabort === "function") {
      try { this.onabort.call(this, evt); } catch (e) { if (console && console.error) console.error(e); }
    }
  }
  return true;
};
AbortSignal.prototype.throwIfAborted = function () {
  if (this._aborted) throw this._reason;
};
AbortSignal.abort = function (reason) {
  var s = new AbortSignal();
  s._aborted = true;
  s._reason = reason !== undefined ? reason : new Error("This operation was aborted");
  return s;
};
AbortSignal.timeout = function (ms) {
  var ctrl = new AbortController();
  var delay = Number(ms) || 0;
  if (typeof globalThis.__velquTimerP === "function") {
    globalThis.__velquTimerP(delay).then(function () {
      ctrl.abort(new Error("TimeoutError: The operation timed out"));
    });
  } else {
    setTimeout(function () {
      ctrl.abort(new Error("TimeoutError: The operation timed out"));
    }, delay);
  }
  return ctrl.signal;
};
AbortSignal.any = function (signals) {
  var ctrl = new AbortController();
  var sigs = Array.from(signals || []);
  var cleanups = [];
  function cleanupAll() {
    for (var j = 0; j < cleanups.length; j++) cleanups[j]();
    cleanups = [];
  }
  for (var i = 0; i < sigs.length; i++) {
    var s = sigs[i];
    if (s.aborted) {
      cleanupAll();
      ctrl.abort(s.reason);
      return ctrl.signal;
    }
    (function (sig) {
      var onAbort = function () {
        cleanupAll();
        ctrl.abort(sig.reason);
      };
      sig.addEventListener("abort", onAbort, { once: true });
      cleanups.push(function () {
        sig.removeEventListener("abort", onAbort);
      });
    })(s);
  }
  return ctrl.signal;
};
globalThis.AbortSignal = AbortSignal;

// AbortController implementation (M27-007-A)
function AbortController() {
  this.signal = new AbortSignal();
}
AbortController.prototype.abort = function(reason) {
  if (this.signal._aborted) return;
  this.signal._aborted = true;
  this.signal._reason = reason !== undefined ? reason : new Error("This operation was aborted");
  var evt = { type: "abort", target: this.signal, currentTarget: this.signal };
  this.signal.dispatchEvent(evt);
};
globalThis.AbortController = AbortController;

// Crypto capability (M27-008-A/B): getRandomValues and randomUUID
function __velquIsIntegerTypedArray(arr) {
  return arr instanceof Int8Array ||
         arr instanceof Uint8Array ||
         arr instanceof Uint8ClampedArray ||
         arr instanceof Int16Array ||
         arr instanceof Uint16Array ||
         arr instanceof Int32Array ||
         arr instanceof Uint32Array ||
         (typeof BigInt64Array !== "undefined" && arr instanceof BigInt64Array) ||
         (typeof BigUint64Array !== "undefined" && arr instanceof BigUint64Array);
}

globalThis.crypto = {
  getRandomValues: function(array) {
    if (typeof globalThis.__velquCryptoGetRandomValues !== "function") throw new TypeError("crypto.getRandomValues native unavailable");
    if (!array || !ArrayBuffer.isView(array) || array instanceof DataView || !__velquIsIntegerTypedArray(array)) {
      throw new TypeError("Failed to execute 'getRandomValues' on 'Crypto': parameter 1 is not an Integer Array");
    }
    var byteLen = array.byteLength;
    if (byteLen > 65536) {
      throw new RangeError("QuotaExceededError: The requested length exceeds 65536 bytes");
    }
    var u = new Uint8Array(array.buffer, array.byteOffset, array.byteLength);
    globalThis.__velquCryptoGetRandomValues(u);
    return array;
  },
  randomUUID: function() {
    if (typeof globalThis.__velquCryptoRandomUUID !== "function") throw new TypeError("crypto.randomUUID native unavailable");
    return globalThis.__velquCryptoRandomUUID();
  }
};

// Fetch API implementation (M28-004-A, M28-004-B: lazy native-backed objects)
const __velquHeadersPrototype = Object.create(null);
__velquHeadersPrototype.get = function(name) {
  var k = String(name).toLowerCase();
  return k in this._map ? this._map[k] : null;
};
__velquHeadersPrototype.has = function(name) {
  var k = String(name).toLowerCase();
  return k in this._map;
};
__velquHeadersPrototype.set = function(name, value) {
  var k = String(name).toLowerCase();
  this._map[k] = String(value);
};
__velquHeadersPrototype.append = function(name, value) {
  var k = String(name).toLowerCase();
  var v = String(value);
  if (k in this._map) {
    this._map[k] = this._map[k] + ", " + v;
  } else {
    this._map[k] = v;
  }
};
__velquHeadersPrototype.delete = function(name) {
  var k = String(name).toLowerCase();
  delete this._map[k];
};
__velquHeadersPrototype.forEach = function(cb, thisArg) {
  for (var k in this._map) {
    cb.call(thisArg, this._map[k], k, this);
  }
};
__velquHeadersPrototype.entries = function* () {
  for (var k in this._map) {
    yield [k, this._map[k]];
  }
};
__velquHeadersPrototype.keys = function* () {
  for (var k in this._map) {
    yield k;
  }
};
__velquHeadersPrototype.values = function* () {
  for (var k in this._map) {
    yield this._map[k];
  }
};
__velquHeadersPrototype[Symbol.iterator] = __velquHeadersPrototype.entries;

function Headers(init) {
  var h = Object.create(__velquHeadersPrototype);
  h._map = Object.create(null);
  if (!init) return h;
  if (init instanceof Headers || (init && init._map)) {
    for (var k in init._map) h._map[k] = init._map[k];
  } else if (Array.isArray(init)) {
    for (var i = 0; i < init.length; i++) {
      if (Array.isArray(init[i]) && init[i].length >= 2) {
        h.append(init[i][0], init[i][1]);
      }
    }
  } else if (typeof init === "object") {
    for (var key in init) {
      if (Object.prototype.hasOwnProperty.call(init, key)) {
        h.set(key, init[key]);
      }
    }
  }
  return h;
}
Headers.prototype = __velquHeadersPrototype;
globalThis.Headers = Headers;

function Request(input, init) {
  init = init || {};
  if (input instanceof Request) {
    this.url = input.url;
    this.method = init.method ? String(init.method).toUpperCase() : input.method;
    this._headersInit = init.headers || input.headers;
    this.body = init.body !== undefined ? init.body : input.body;
    this.signal = init.signal || input.signal;
  } else {
    this.url = String(input);
    this.method = init.method ? String(init.method).toUpperCase() : "GET";
    this._headersInit = init.headers;
    this.body = init.body !== undefined ? init.body : null;
    this.signal = init.signal || null;
  }
  // Lazy headers materialization
  var _h = null;
  Object.defineProperty(this, "headers", {
    enumerable: true,
    get: function() {
      if (_h === null) _h = new Headers(this._headersInit);
      return _h;
    }
  });
}
Request.prototype.clone = function() {
  return new Request(this.url, {
    method: this.method,
    headers: new Headers(this.headers),
    body: this.body,
    signal: this.signal
  });
};
globalThis.Request = Request;

function Response(body, init) {
  init = init || {};
  this.status = init.status !== undefined ? Number(init.status) : 200;
  this.statusText = init.statusText !== undefined ? String(init.statusText) : "OK";
  this.ok = this.status >= 200 && this.status < 300;
  this.url = init.url !== undefined ? String(init.url) : "";
  this.bodyUsed = false;
  this._body = body !== undefined ? body : null;
  this._headersInit = init.headers;
  // Lazy headers materialization
  var _h = null;
  Object.defineProperty(this, "headers", {
    enumerable: true,
    get: function() {
      if (_h === null) {
        _h = this._headersInit instanceof Headers ? this._headersInit : new Headers(this._headersInit);
      }
      return _h;
    }
  });
}
Response.json = function(data, init) {
  init = init || {};
  var headers = new Headers(init.headers);
  if (!headers.has("content-type")) {
    headers.set("content-type", "application/json");
  }
  init.headers = headers;
  return new Response(JSON.stringify(data), init);
};
// M28-006-D: maximum body helper sizes. Materializing helpers fail closed
// above the native limit before any derived copy is made. The guard keeps
// the plain Web surface runnable outside the host (dev tooling); in the
// production worker the limit binding is always installed.
var __velquBodyHelperCheck = function(helper, b) {
  if (typeof globalThis.__velquBodyHelperLimit !== "function") return;
  var max = globalThis.__velquBodyHelperLimit();
  var n;
  if (b === null || b === undefined) {
    n = 0;
  } else if (b instanceof Uint8Array || b instanceof ArrayBuffer) {
    n = b.byteLength;
  } else {
    // Measure UTF-8 size via the native encode bridge; its over-ceiling
    // throw means the body is over the (same) helper limit too.
    try {
      n = globalThis.__velquTextEncodeLen(String(b));
    } catch (e) {
      n = max + 1;
    }
  }
  if (n > max) {
    throw new TypeError("Response." + helper + ": body of " + n + " bytes exceeds the maximum helper size of " + max + " bytes");
  }
};
Response.prototype.text = function() {
  __velquBodyHelperCheck("text", this._body);
  if (this.bodyUsed) return Promise.reject(new TypeError("Body has already been consumed"));
  this.bodyUsed = true;
  var b = this._body;
  if (b === null || b === undefined) return Promise.resolve("");
  if (typeof b === "string") return Promise.resolve(b);
  if (b instanceof Uint8Array || b instanceof ArrayBuffer) {
    return Promise.resolve(new TextDecoder().decode(b));
  }
  return Promise.resolve(String(b));
};
Response.prototype.json = function() {
  return this.text().then(function(t) {
    return JSON.parse(t);
  });
};
Response.prototype.arrayBuffer = function() {
  __velquBodyHelperCheck("arrayBuffer", this._body);
  if (this.bodyUsed) return Promise.reject(new TypeError("Body has already been consumed"));
  this.bodyUsed = true;
  var b = this._body;
  if (b === null || b === undefined) return Promise.resolve(new ArrayBuffer(0));
  if (b instanceof ArrayBuffer) return Promise.resolve(b);
  if (b instanceof Uint8Array) return Promise.resolve(b.buffer.slice(b.byteOffset, b.byteOffset + b.byteLength));
  var enc = new TextEncoder().encode(String(b));
  return Promise.resolve(enc.buffer.slice(enc.byteOffset, enc.byteOffset + enc.byteLength));
};
Response.prototype.bytes = function() {
  __velquBodyHelperCheck("bytes", this._body);
  if (this.bodyUsed) return Promise.reject(new TypeError("Body has already been consumed"));
  this.bodyUsed = true;
  var b = this._body;
  if (b === null || b === undefined) return Promise.resolve(new Uint8Array(0));
  if (b instanceof Uint8Array) return Promise.resolve(b);
  if (b instanceof ArrayBuffer) return Promise.resolve(new Uint8Array(b));
  return Promise.resolve(new TextEncoder().encode(String(b)));
};
Response.prototype.clone = function() {
  if (this.bodyUsed) {
    throw new TypeError("Failed to execute 'clone' on 'Response': Response body is already used");
  }
  return new Response(this._body, {
    status: this.status,
    statusText: this.statusText,
    headers: new Headers(this.headers),
    url: this.url
  });
};
globalThis.Response = Response;

globalThis.fetch = function(input, init) {
  var req = input instanceof Request ? input : new Request(input, init);
  // Explicit scheme validation (ADR-0033 §1)
  var urlStr = String(req.url);
  var colonIdx = urlStr.indexOf(":");
  if (colonIdx !== -1) {
    var scheme = urlStr.slice(0, colonIdx).toLowerCase();
    if (scheme !== "http" && scheme !== "https") {
      return Promise.reject(new TypeError("fetch: scheme '" + scheme + "' is not allowed (http/https only, fail closed)"));
    }
  }
  if (req.signal && req.signal.aborted) {
    return Promise.reject(req.signal.reason);
  }
  return new Promise(function(resolve, reject) {
    if (req.signal && typeof req.signal.addEventListener === "function") {
      req.signal.addEventListener("abort", function() {
        reject(req.signal.reason);
      }, { once: true });
    }
    // Host-bound native fetch bridge
    if (typeof globalThis.__velquFetchBridge === "function") {
      var headerObj = {};
      req.headers.forEach(function(v, k) { headerObj[k] = v; });
      var bodyStr = null;
      if (typeof req.body === "string") bodyStr = req.body;
      else if (req.body) bodyStr = JSON.stringify(req.body);
      globalThis.__velquFetchBridge(req.method, req.url, JSON.stringify(headerObj), bodyStr)
        .then(function(resJson) {
          var parsed = JSON.parse(resJson);
          var resp = new Response(parsed.body, {
            status: parsed.status,
            statusText: parsed.statusText,
            headers: parsed.headers,
            url: parsed.url || req.url
          });
          resolve(resp);
        })
        .catch(reject);
    } else {
      // Default fallback mock response when bridge is not yet active
      resolve(new Response("", { status: 200, statusText: "OK", url: req.url }));
    }
  });
};

// Stable capability graph; operation authorization remains native per call.
const __velquNativeCapabilities = Object.freeze({
  timer: Object.freeze({ delay: (ms, options) => globalThis.__velquTimerP(ms, options) }),
  console: Object.freeze(globalThis.console),
  url: Object.freeze({ URL: globalThis.URL, URLSearchParams: globalThis.URLSearchParams }),
  text: Object.freeze({ TextEncoder: globalThis.TextEncoder, TextDecoder: globalThis.TextDecoder }),
  abort: Object.freeze({ AbortController: globalThis.AbortController, AbortSignal: globalThis.AbortSignal }),
  crypto: Object.freeze(globalThis.crypto),
  fetch: Object.freeze({
    fetch: globalThis.fetch,
    Headers: globalThis.Headers,
    Request: globalThis.Request,
    Response: globalThis.Response,
  })
});
globalThis.__velquNativeCapabilities = __velquNativeCapabilities;
__velquContextPrototype.native = __velquNativeCapabilities;

globalThis.__velquMakeReq = function (slot, gen) {
  const req = Object.create(__velquRequestPrototype);
  let headers, params, query, signal;
  Object.defineProperty(req, "headers", { enumerable: true, get() { return (headers ??= JSON.parse(globalThis.__velquReqRaw(slot, gen, "headers"))); } });
  Object.defineProperty(req, "params", { enumerable: true, get() { return (params ??= globalThis.__velquMakeLazyParams(slot, gen)); } });
  Object.defineProperty(req, "query", { enumerable: true, get() { return (query ??= JSON.parse(globalThis.__velquReqRaw(slot, gen, "query"))); } });
  Object.defineProperty(req, "text", { enumerable: true, value: () => globalThis.__velquReqBodyText(slot, gen) });
  Object.defineProperty(req, "signal", { enumerable: true, get() { return (signal ??= new AbortController().signal); } });
  return req;
};

// M24-004-D: params builds as an object of per-key lazy getters — touching
// ctx.params.k materializes exactly one value; untouched keys allocate nothing.
globalThis.__velquMakeLazyParams = function (slot, gen) {
  const obj = {};
  const names = JSON.parse(globalThis.__velquReqParamNames(slot, gen));
  for (var i = 0; i < names.length; i++) {
    (function (n) {
      let v, used = false;
      Object.defineProperty(obj, n, {
        enumerable: true,
        get: function () {
          if (!used) { const raw = globalThis.__velquReqParam(slot, gen, n); v = (raw === undefined || raw === null) ? undefined : raw; used = true; }
          return v;
        }
      });
    })(names[i]);
  }
  return obj;
};

// M24-005-B: headers builds as per-key lazy getters over the plan-declared
// names — ctx.headers.k materializes exactly one value.
globalThis.__velquMakeLazyHeaders = function (slot, gen) {
  const obj = {};
  const names = JSON.parse(globalThis.__velquReqHeaderNames(slot, gen));
  for (var i = 0; i < names.length; i++) {
    (function (n) {
      let v, used = false;
      Object.defineProperty(obj, n, {
        enumerable: true,
        get: function () {
          if (!used) { const raw = globalThis.__velquReqHeader(slot, gen, n); v = (raw === undefined || raw === null) ? undefined : raw; used = true; }
          return v;
        }
      });
    })(names[i]);
  }
  return obj;
};

// ctx: pre.* are host-validated values (native strategy) or undefined for lazy access.
globalThis.__velquMakeCtx = function (slot, gen, pre) {
  const c = Object.create(__velquContextPrototype);
  Object.defineProperty(c, "__velquSlot", { value: slot });
  Object.defineProperty(c, "__velquGeneration", { value: gen });
  const requestless = slot === -1;
  const lazy = (key, fn) => {
    let v, used = false;
    Object.defineProperty(c, key, { enumerable: true, get() { if (!used) { v = fn(); used = true; } return v; } });
  };
  if (pre.routePlan != null) c.routePlan = pre.routePlan;
  if (!requestless) {
    // M25-007-B: the full request handle — whole-field header/query/param
    // access through the store (declared set unless the route declared
    // the full-request capability, which materializes everything)
    lazy("request", () => globalThis.__velquMakeReq(slot, gen));
    lazy("signal", () => new AbortController().signal);
    if (pre.params != null) c.params = pre.params; else lazy("params", () => globalThis.__velquMakeLazyParams(slot, gen));
    if (pre.query != null) c.query = pre.query; else lazy("query", () => JSON.parse(globalThis.__velquReqRaw(slot, gen, "query")));
    if (pre.headers != null) c.headers = pre.headers; else lazy("headers", () => globalThis.__velquMakeLazyHeaders(slot, gen));
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
  }
  return c;
};

// Timer capability: promise callbacks live in a JS-side op table keyed by
// op id; the host resolves/rejects through the two dispatch functions below.
globalThis.__velquOps = Object.create(null);
globalThis.__velquTimerP = function (ms, options) {
  return new Promise((resolve, reject) => {
    if (options && options.signal && options.signal.aborted) {
      return reject(options.signal.reason);
    }
    const opId = globalThis.__velquTimerStart(ms);
    globalThis.__velquOps[opId] = { resolve, reject };
    if (options && options.signal && typeof options.signal.addEventListener === "function") {
      options.signal.addEventListener("abort", function() {
        globalThis.__velquOpReject(opId, options.signal.reason);
      }, { once: true });
    }
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

// Sync-or-Promise policy+handler runner: synchronous handlers return directly
// (no Promise allocation, no settlement-table entry, no job-queue drain).
// Async handlers (or thenable policies) take the Promise path naturally.
globalThis.__velquIsThenable = function (v) {
  return v !== null && v !== undefined &&
    (typeof v === "object" || typeof v === "function") &&
    typeof v.then === "function";
};

globalThis.__velquRun = function (handlerFn, policyFn, ctx, req) {
  if (!policyFn) return handlerFn(ctx);

  var pr = policyFn(req);
  if (globalThis.__velquIsThenable(pr)) {
    return pr.then(function (r) {
      if (r && r.__problem) return r;
      Object.defineProperty(ctx, "session", { value: r.session, enumerable: true });
      return handlerFn(ctx);
    });
  }
  if (pr && pr.__problem) return pr;
  Object.defineProperty(ctx, "session", { value: pr.session, enumerable: true });
  return handlerFn(ctx);
};

// Settlement watch: the host reads and clears this table after draining jobs.
// Exactly one entry per invocation id. Only registered for thenable results.
globalThis.__velquSettled = Object.create(null);
globalThis.__velquWatch = function (p, id) {
  var key = String(id);
  Promise.resolve(p).then(
    (v) => { globalThis.__velquSettled[key] = { ok: true, v }; },
    (e) => { globalThis.__velquSettled[key] = { ok: false, e }; }
  );
  return p;
};
"#;
