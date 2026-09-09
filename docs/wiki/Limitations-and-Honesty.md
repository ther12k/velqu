# Limitations and Honesty

Velqu's claim policy is part of the architecture. These statements are
binding on all documentation, issues, and marketing:

## Claim prohibitions

- **No hostile-code sandboxing claims.** Same-process QuickJS (and the
  browser Worker bridge) executes *trusted application code only*.
  Worker isolation is a process boundary, not a security sandbox.
  Untrusted-handler deployments require a separate preview origin
  (ADR-0038).
- **No PostgreSQL-parity claims** for the browser target. Browser
  persistence is namespaced IndexedDB KV; PostgreSQL remains a
  native-runtime capability.
- **No native-performance-parity claims** without the separately
  required matched evidence.
- **No "zero hosting" framing.** Browser-WASM deployment means static
  HTTPS artifacts with no Velqu *application server* — you still need a
  static host.

## Recorded limitations

The running list lives in
[docs/beta/KNOWN-LIMITATIONS.md](https://github.com/ther12k/velqu/blob/master/docs/beta/KNOWN-LIMITATIONS.md)
(items 1–24, including the Browser-WASM preview boundaries 19–24).
Highlights:

- Beta deployment target is Linux x86_64; a pack runs only on the exact
  runtime build it was compiled against (rebuild both on upgrade).
- Browser evidence lanes cover Chromium; other browsers are documented
  but untested.
- Browser preview assets not listed in the build manifest (custom pages,
  images) have documented Service-Worker boundary behavior.
- npm publication and public release channels are owner decisions
  ([docs/open-decisions.md](https://github.com/ther12k/velqu/blob/master/docs/open-decisions.md)).

## Governance

- Open vs decided owner decisions:
  [docs/open-decisions.md](https://github.com/ther12k/velqu/blob/master/docs/open-decisions.md)
- Beta governance (release authority, risk register, evidence standard):
  [docs/beta/governance/](https://github.com/ther12k/velqu/tree/master/docs/beta/governance)
- Material design changes require an ADR under
  [docs/okf/decisions/](https://github.com/ther12k/velqu/tree/master/docs/okf/decisions).
