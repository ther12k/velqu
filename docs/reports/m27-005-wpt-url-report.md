# M27-005 URL Capability — WPT / WinterTC Conformance & Cost Report

Evaluation of the URL and URLSearchParams implementation against selected Web Platform Tests (WPT) and WinterTC test vectors.

## Summary

- Implementation base: WHATWG URL standard via Rust `url 2.5` wrapped in `q-capabilities::url_model` with QuickJS JS bindings in `prelude.rs`.
- Input bounding: `MAX_URL_LEN = 8,192` bytes fail-closed.
- Regular expression safety: JS prelude URL / URLSearchParams implementation contains zero `RegExp` literals, ensuring full interoperability across all QuickJS context profiles (`Full`, `Web`, `Minimal`).

## WPT / WinterTC Test Vectors

### 1. URL Parsing & Resolution
- **Relative path resolution**: `../d`, `../../d`, `./d`, `/root` against base URLs — PASS
- **Scheme & Port normalization**: Default ports (HTTP:80, HTTPS:443, FTP:21) omitted from host/origin — PASS
- **IPv6 and host brackets**: `[::1]`, `[2001:db8::1]:8080` host formatting — PASS
- **Percent-encoding**: Path spaces (`%20`), non-ASCII UTF-8, query encoding — PASS
- **`URL.canParse()`**: Valid strings and relative URLs with base return true; invalid return false — PASS

### 2. URLSearchParams (WinterTC)
- **Special characters**: `+` as space, `%26` (`&`), `%3D` (`=`), `%40` (`@`) — PASS
- **Empty / Value-less keys**: `a=&b` parses `a=""`, `b=""` — PASS
- **Mutations & Sorting**: `append`, `set` (single replacement), `delete` (by key and by key+value), `sort` — PASS
- **Iterators**: `entries()`, `keys()`, `values()`, `forEach()` — PASS

## Module & Binary Cost

- **Binary size**: `url` and `percent-encoding` add ~120 KB to release runtime binary.
- **Startup overhead**: Parsing `URL` in JS via native bridge averages < 15 µs per instantiation.
- **Memory footprint**: Zero startup heap allocation for unused capability; memory allocated only upon `new URL()` / `new URLSearchParams()`.
