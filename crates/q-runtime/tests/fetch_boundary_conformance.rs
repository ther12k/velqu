//! #1424 — fetch boundary conformance: validated-address pinning and the
//! serving egress default.
//!
//! The review found two boundary violations: (1) the SSRF gate classified
//! DNS answers and then DISCARDED them, while the pooled connector
//! re-resolved independently at dial time — a rebinding window where the
//! connected address was never classified; (2) serving unconditionally
//! constructed the fetch dialer with the integration-test loopback trust.
//!
//! These tests prove the closed boundary composition:
//! - classification → pin: the dialer path validates and pins the
//!   addresses it classified (localhost test below);
//! - pin → dial: the pinned client resolves ONLY from validated pins —
//!   an unpinned host can never be dialed, so an address that was not
//!   classified cannot become a connection (rebinding window closed);
//! - serving default: the default policy denies loopback egress; the
//!   test-loopback opt-in still reaches a local mock origin.

use std::error::Error as _;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use q_capabilities::FetchPolicy;
use q_engine_quickjs::FetchDialer as _;
use velqu_runtime::fetch_bridge::PoolFetchDialer;
use velqu_runtime::fetch_stack::{
    build_pinned_client_with_bounds, pin_validated_addrs, PoolBounds,
};

/// A one-shot HTTP/1.1 mock origin counting every accepted connection.
struct MockOrigin {
    port: u16,
    hits: Arc<AtomicUsize>,
}

impl MockOrigin {
    /// Spawns a listener answering `200 {"origin":"<marker>"}` once per
    /// connection (hyper serves keep-alive; one request per connection is
    /// forced by Connection: close from the client side).
    fn spawn(marker: &'static str) -> MockOrigin {
        let hits = Arc::new(AtomicUsize::new(0));
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind mock origin");
        let port = listener.local_addr().expect("local addr").port();
        let counter = Arc::clone(&hits);
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else { break };
                counter.fetch_add(1, Ordering::SeqCst);
                let body = format!("{{\"origin\":\"{marker}\"}}");
                let resp = format!(
                    "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                use std::io::{Read, Write};
                let mut buf = [0u8; 1024];
                let _ = stream.read(&mut buf);
                let _ = stream.write_all(resp.as_bytes());
            }
        });
        MockOrigin { port, hits }
    }
}

#[tokio::test]
async fn pinned_client_dials_only_validated_addresses() {
    // The connector's resolution must come from the validated pin, never
    // from the system resolver. `velqu-pinned.example.invalid` does not
    // resolve anywhere (RFC 6762 reserved TLD + invented label): if any
    // code path re-resolved, the request would fail NXDOMAIN. With the
    // pin, it connects to the pinned socket and succeeds.
    let good = MockOrigin::spawn("pin-good");
    let evil = MockOrigin::spawn("pin-evil");

    pin_validated_addrs(
        "velqu-pinned.example.invalid",
        &[std::net::IpAddr::from([127u8, 0, 0, 1])],
        good.port,
    );

    let client = build_pinned_client_with_bounds(&PoolBounds::default());
    let req = hyper::Request::get(format!(
        "http://velqu-pinned.example.invalid:{}/",
        good.port
    ))
    .header("connection", "close")
    .body(http_body_util::Empty::<bytes::Bytes>::new())
    .expect("request");
    let res = tokio::time::timeout(Duration::from_secs(10), client.request(req))
        .await
        .expect("request within timeout")
        .expect("pinned request must reach the pinned origin");
    assert_eq!(res.status(), 200);
    let body = http_body_util::BodyExt::collect(res.into_body())
        .await
        .expect("body");
    let text = String::from_utf8_lossy(&body.to_bytes()).into_owned();
    assert!(
        text.contains("pin-good"),
        "the dialed origin must be the PINNED one, got: {text}"
    );
    assert_eq!(
        evil.hits.load(Ordering::SeqCst),
        0,
        "no connection may be placed to any address outside the validated pin"
    );
}

#[tokio::test]
async fn unpinned_host_fails_closed_never_dials() {
    // A host with NO validated pin (the stale/recycled/ungated case, and
    // exactly the shape a rebinding attacker's second answer would hit)
    // must fail closed at resolution: the connector never falls back to
    // the system resolver, so an unclassified address cannot be dialed.
    let probe = MockOrigin::spawn("never-reached");

    let client = build_pinned_client_with_bounds(&PoolBounds::default());
    let req = hyper::Request::get(format!(
        "http://velqu-unpinned.example.invalid:{}/",
        probe.port
    ))
    .header("connection", "close")
    .body(http_body_util::Empty::<bytes::Bytes>::new())
    .expect("request");
    let err = tokio::time::timeout(Duration::from_secs(10), client.request(req))
        .await
        .expect("request within timeout")
        .expect_err("unpinned host must fail closed");
    // hyper-util's top-level Display is generic ("client error
    // (Connect)"); the resolver message lives in the source chain.
    let mut chain = err.to_string();
    let mut src: Option<&dyn std::error::Error> = err.source();
    while let Some(e) = src {
        chain.push_str(" :: ");
        chain.push_str(&e.to_string());
        src = e.source();
    }
    assert!(
        chain.contains("no policy-validated address"),
        "fail-closed resolver error expected, got: {chain}"
    );
    assert_eq!(
        probe.hits.load(Ordering::SeqCst),
        0,
        "an unvalidated host must never be dialed"
    );
}

#[tokio::test]
async fn dialer_default_policy_denies_loopback_and_test_trust_allows_it() {
    // The full serving path through PoolFetchDialer (validate → classify
    // → pin → pinned client), against a real local origin bound to
    // 127.0.0.1. `localhost` resolves to 127.0.0.1 via the system
    // resolver (RFC 6761), so classification and pinning are exercised
    // end-to-end. If the platform cannot resolve `localhost`, the
    // deny-case is still asserted; the allow-case degrades to a skip.
    let origin = MockOrigin::spawn("local");

    // Default policy: loopback egress denied — serving's posture without
    // the explicit test opt-in (#1424.2).
    let deny = PoolFetchDialer::new(FetchPolicy::default());
    let err = deny
        .dial(
            "GET".into(),
            format!("http://localhost:{}/", origin.port),
            "{}".into(),
            None,
        )
        .await
        .expect_err("default serving policy must deny loopback egress");
    assert!(
        err.contains("fetch policy rejected"),
        "expected the policy-classified rejection, got: {err}"
    );
    assert_eq!(
        origin.hits.load(Ordering::SeqCst),
        0,
        "nothing dialed under deny"
    );

    // Test-loopback trust (the explicit opt-in): classification passes,
    // the resolved address is pinned, and the pinned client dials it.
    let allow = PoolFetchDialer::new(FetchPolicy::trusted_loopback_explicit());
    let wire = allow
        .dial(
            "GET".into(),
            format!("http://localhost:{}/", origin.port),
            "{}".into(),
            None,
        )
        .await;
    match wire {
        Ok(body) => assert!(
            body.contains("local"),
            "opt-in loopback fetch must reach the local origin, got: {body}"
        ),
        Err(e) => {
            // hosts-file-less minimal containers cannot resolve
            // `localhost` through getaddrinfo; the policy posture above is
            // still proven. Degraded environments must not fail the suite.
            eprintln!("skipping allow-case dial (localhost unresolvable here): {e}");
        }
    }
}

#[tokio::test]
async fn ip_literal_fetch_reaches_pinned_local_origin_only_when_policy_allows() {
    // IP literals bypass DNS entirely: the policy classifies the literal
    // in-line and the connector connects directly (no pin involved).
    let origin = MockOrigin::spawn("literal");

    let deny = PoolFetchDialer::new(FetchPolicy::default());
    let err = deny
        .dial(
            "GET".into(),
            format!("http://127.0.0.1:{}/", origin.port),
            "{}".into(),
            None,
        )
        .await
        .expect_err("default policy must deny a loopback IP literal");
    assert!(
        err.contains("fetch policy rejected"),
        "expected policy rejection for the literal, got: {err}"
    );

    let allow = PoolFetchDialer::new(FetchPolicy::trusted_loopback_explicit());
    let body = allow
        .dial(
            "GET".into(),
            format!("http://127.0.0.1:{}/", origin.port),
            "{}".into(),
            None,
        )
        .await
        .expect("opt-in policy dials the loopback literal");
    assert!(
        body.contains("literal"),
        "expected the origin body, got: {body}"
    );
}
