//! Outbound fetch stack wiring (M28-002-B).
//!
//! Links the stack selected in M28-002-A (hyper 1 + hyper-util
//! client-legacy + hyper-rustls with webpki roots) into the release
//! binary and exposes one dormant, reachable constructor used by the
//! `--fetch-stack-info` diagnostic flag. M28-003 replaces this module's
//! internals with the real bounded pool; the policy (ADR-0033) still
//! gates every dial.

use hyper_rustls::HttpsConnectorBuilder;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;

/// Human-readable identity of the selected outbound stack.
pub const STACK_ID: &str =
    "hyper1+hyper-util(client-legacy)+hyper-rustls(webpki-roots,ring,http1,tls12)";

/// Build the policy-shaped HTTPS connector: webpki roots, HTTP/1 only,
/// https-or-http (scheme enforcement happens in the ADR-0033 policy
/// layer before any dial).
fn build_connector(
) -> hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector> {
    HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_or_http()
        .enable_http1()
        .wrap_connector(hyper_util::client::legacy::connect::HttpConnector::new())
}

/// Construct the shared outbound client. Dormant until M28-003 wires
/// the bounded pool; reachable so the linker keeps the stack in the
/// release binary (the M28-002-B cost measurement depends on this).
pub fn build_client() -> Client<
    hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>,
    http_body_util::Empty<bytes::Bytes>,
> {
    Client::builder(TokioExecutor::new()).build(build_connector())
}

/// Diagnostic summary for `--fetch-stack-info`.
pub fn describe() -> String {
    let _client = build_client(); // prove the stack constructs
    format!("outbound fetch stack: {STACK_ID} (constructs ok; dialing gated by ADR-0033 policy, wired in M28-003)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_constructs_lazily_and_names_itself() {
        let described = describe();
        assert!(described.contains("hyper1+hyper-util"));
        assert!(described.contains("webpki-roots"));
        assert!(described.contains("M28-003"));
    }
}
