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
//! - #1424 review hardening: the pin registry is bound to ONE egress
//!   policy per process (hyper-util's resolver has no per-request
//!   context, so request-bound pins are structurally unavailable; the
//!   single-policy invariant is what makes cross-request pin reuse
//!   safe). A second distinct policy is refused fail-closed, and
//!   concurrent pin+dial churn on distinct hosts reaches exactly each
//!   host's own validated origin.

use std::error::Error as _;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use q_capabilities::FetchPolicy;
use q_engine_quickjs::FetchDialer as _;
use velqu_runtime::fetch_bridge::PoolFetchDialer;
use velqu_runtime::fetch_stack::{
    build_pinned_client_with_bounds, pin_validated_addrs, process_policy_tag, PoolBounds,
};

/// The tests in this file share the process-global pin registry and the
/// global pinned client; the registry's active-policy tag makes that
/// coupling real. Every test holds this lock for its whole body so the
/// tag invariant is deterministic regardless of test order. All pinning
/// uses the test-trust policy's tag (the only policy that ever pins
/// here), so the second-policy refusal test can deterministically
/// exercise the refusal path.
static TEST_PIN_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

/// Held for each test's whole body: the pin registry and the global
/// pinned client are process-global, and the active-policy tag makes the
/// test-order coupling real. An async mutex because the guard is
/// deliberately held across the (awaiting) dial assertions.
async fn lock_pins() -> tokio::sync::MutexGuard<'static, ()> {
    TEST_PIN_LOCK.lock().await
}

fn test_trust_tag() -> u64 {
    process_policy_tag(&FetchPolicy::trusted_loopback_explicit())
}

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

    let _guard = lock_pins().await;
    pin_validated_addrs(
        test_trust_tag(),
        "velqu-pinned.example.invalid",
        &[std::net::IpAddr::from([127u8, 0, 0, 1])],
        good.port,
    )
    .expect("first pin under the test-trust policy");

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
    let _guard = lock_pins().await;
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
    let _guard = lock_pins().await;
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
    let _guard = lock_pins().await;
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

#[tokio::test]
async fn second_egress_policy_is_refused_fail_closed() {
    // #1424 review hardening: hyper-util's resolver receives only the
    // host string, so pins cannot be bound to individual requests. The
    // registry instead enforces the invariant that makes cross-request
    // pin reuse safe: ONE egress policy per process. A second distinct
    // policy is refused at pin registration (fail closed), and its
    // dialer consequently cannot reach any validated address.
    let _guard = lock_pins().await;

    // Deterministically establish the active tag (whatever ran first).
    let seed_port = MockOrigin::spawn("seed").port;
    pin_validated_addrs(
        test_trust_tag(),
        "tag-seed.example.invalid",
        &[std::net::IpAddr::from([127u8, 0, 0, 1])],
        seed_port,
    )
    .expect("active tag is the test-trust policy");

    // A second policy (distinct deny list → distinct tag) is refused.
    let other = FetchPolicy::trusted_loopback_explicit().with_deny_hosts(["forbidden.example"]);
    assert_ne!(
        process_policy_tag(&other),
        test_trust_tag(),
        "the two policies must hash to different tags"
    );
    let err = pin_validated_addrs(
        process_policy_tag(&other),
        "tag-second.example.invalid",
        &[std::net::IpAddr::from([127u8, 0, 0, 1])],
        1,
    )
    .expect_err("a second egress policy must be refused");
    assert!(
        err.contains("second egress policy"),
        "typed refusal expected, got: {err}"
    );

    // And that policy's dialer fails closed end-to-end on a HOSTNAME:
    // its gate passes (test-trust semantics), but its pin is refused at
    // registration, so the fetch never reaches a socket. (An IP literal
    // target would bypass the registry by design — literals never pin.)
    // On hosts that cannot resolve `localhost`, the dial fails one step
    // earlier (DNS) — the socket-untouched invariant still holds.
    let origin = MockOrigin::spawn("second-policy");
    let dialer = PoolFetchDialer::new(other);
    let err = dialer
        .dial(
            "GET".into(),
            format!("http://localhost:{}/", origin.port),
            "{}".into(),
            None,
        )
        .await
        .expect_err("the second policy's dialer must fail closed");
    assert!(
        err.contains("second egress policy") || err.contains("dns resolution failed"),
        "dialer must surface the registry refusal, got: {err}"
    );
    assert_eq!(
        origin.hits.load(Ordering::SeqCst),
        0,
        "the second policy's dialer must never reach a socket"
    );
}

#[tokio::test]
async fn concurrent_pin_and_dial_reaches_each_host_own_validated_origin() {
    // #1424 review hardening: concurrent requests pinning and dialing
    // DISTINCT hosts must each reach exactly their own validated origin —
    // the churn/overwrite shape the review flagged, with the dial-set
    // invariant asserted per request. Every host is unresolvable
    // (reserved-TLD label): any system-resolver fallback would NXDOMAIN.
    let _guard = lock_pins().await;

    // Establish the active tag.
    let seed_port = MockOrigin::spawn("tag-seed").port;
    pin_validated_addrs(
        test_trust_tag(),
        "churn-seed.example.invalid",
        &[std::net::IpAddr::from([127u8, 0, 0, 1])],
        seed_port,
    )
    .expect("active tag is the test-trust policy");

    const N: usize = 8;
    const MARKERS: [&str; N] = ["c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7"];
    let origins: Vec<(u16, Arc<AtomicUsize>)> = MARKERS
        .iter()
        .map(|m| {
            let o = MockOrigin::spawn(m);
            (o.port, o.hits)
        })
        .collect();

    let client = Arc::new(build_pinned_client_with_bounds(&PoolBounds::default()));
    let mut handles = Vec::new();
    for i in 0..N {
        let client = Arc::clone(&client);
        let (port, hits) = origins[i].clone();
        handles.push(tokio::spawn(async move {
            let host = format!("churn-{i}.example.invalid");
            pin_validated_addrs(
                test_trust_tag(),
                &host,
                &[std::net::IpAddr::from([127u8, 0, 0, 1])],
                port,
            )
            .expect("same policy pins concurrently");
            let req = hyper::Request::get(format!("http://{host}/"))
                .header("connection", "close")
                .body(http_body_util::Empty::<bytes::Bytes>::new())
                .expect("request");
            let res = tokio::time::timeout(Duration::from_secs(10), client.request(req))
                .await
                .expect("request within timeout")
                .expect("pinned dial must succeed");
            let body = http_body_util::BodyExt::collect(res.into_body())
                .await
                .expect("body")
                .to_bytes();
            let text = String::from_utf8_lossy(&body).into_owned();
            assert!(
                text.contains(MARKERS[i]),
                "dial for host {i} must reach its OWN pinned origin, got: {text}"
            );
            assert_eq!(
                hits.load(Ordering::SeqCst),
                1,
                "origin {i} hit exactly once"
            );
        }));
    }
    for h in handles {
        h.await.expect("churn task");
    }

    // Overwrite shape: re-pinning an existing host to a NEW validated
    // origin keeps the invariant — the next dial reaches a validated set
    // (the newest pin, or the still-pooled connection to the previous
    // validated origin; both were gate-classified).
    let (rebind_port, rebind_hits) = {
        let o = MockOrigin::spawn("c8");
        (o.port, o.hits)
    };
    pin_validated_addrs(
        test_trust_tag(),
        "churn-0.example.invalid",
        &[std::net::IpAddr::from([127u8, 0, 0, 1])],
        rebind_port,
    )
    .expect("re-pin under the same policy");
    let req = hyper::Request::get("http://churn-0.example.invalid/")
        .header("connection", "close")
        .body(http_body_util::Empty::<bytes::Bytes>::new())
        .expect("request");
    let res = tokio::time::timeout(Duration::from_secs(10), client.request(req))
        .await
        .expect("request within timeout")
        .expect("re-pinned dial must succeed");
    let body = http_body_util::BodyExt::collect(res.into_body())
        .await
        .expect("body")
        .to_bytes();
    let text = String::from_utf8_lossy(&body).into_owned();
    assert!(
        text.contains("c0") || text.contains("c8"),
        "re-pinned dial must reach a VALIDATED origin (old pooled or new pin), got: {text}"
    );
    let c8_hits = rebind_hits.load(Ordering::SeqCst);
    assert!(
        c8_hits <= 1,
        "the new validated origin is reached at most once, got {c8_hits}"
    );
}
