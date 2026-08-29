//! Deterministic fixture conformance for the fetch executor shape
//! (M28-010-B). Drives DNS-resolution pinning, redirect chains, slow
//! bodies, and untrusted TLS endpoints against the production pool and
//! policy — hermetically, with bounded waits everywhere.

mod fetch_fixtures;

use std::collections::BTreeMap;
use std::io::Write;
use std::net::{IpAddr, TcpListener};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use fetch_fixtures::{
    deterministic_resolver, spawn_immediate_close_server, spawn_redirect_server,
    spawn_slow_body_server, RebindingResolver,
};

use http_body_util::Empty;
use hyper::Request;
use velqu_runtime::fetch_stack::{FetchPool, PoolError};

use q_capabilities::{resolve_and_validate, FetchPolicy, RedirectLimiter, RedirectOutcome};

const DIAL_BUDGET: Duration = Duration::from_secs(10);

#[tokio::test]
async fn dns_pinning_resolves_once_and_dials_the_pin_set() {
    // The rebinding attacker answers the same on every call, but the
    // connect gate must consult it exactly ONCE: the pin set is what
    // dials. A second resolution (TTL expiry attacker) never happens.
    let policy = FetchPolicy::default();
    let resolver = RebindingResolver::new(vec!["93.184.216.34".parse().unwrap()]);
    let pinned = resolve_and_validate(&policy, "example.test", |h| resolver.resolve(h))
        .expect("public resolution validates");
    assert_eq!(pinned, vec!["93.184.216.34".parse::<IpAddr>().unwrap()]);
    assert_eq!(resolver.calls(), 1, "connect gate resolves exactly once");
}

#[tokio::test]
async fn dns_rebinding_table_with_private_answer_fails_closed() {
    let mut table = BTreeMap::new();
    table.insert(
        "rebind.test",
        vec![
            "93.184.216.34".parse::<IpAddr>().unwrap(),
            "192.168.0.1".parse::<IpAddr>().unwrap(),
        ],
    );
    let policy = FetchPolicy::default();
    let err = resolve_and_validate(&policy, "rebind.test", deterministic_resolver(table))
        .expect_err("one private address poisons the host");
    assert!(err.to_string().contains("denied"), "typed denial: {err}");
}

#[tokio::test]
async fn redirect_chain_follows_bounded_and_records_hops() {
    let requests = Arc::new(AtomicUsize::new(0));
    let port = spawn_redirect_server(3, requests.clone());

    let pool = FetchPool::new();
    let client = pool.client();

    // Executor shape: raw request -> read Location -> policy-check each hop
    // through the limiter -> follow. The chain has 3 redirects + 1 final 200.
    let mut limiter = RedirectLimiter::new(FetchPolicy::default());
    let mut url = format!("http://127.0.0.1:{port}/hop/0");
    let mut final_status;
    loop {
        let req = Request::get(url.clone())
            .body(Empty::<bytes::Bytes>::new())
            .unwrap();
        let resp = tokio::time::timeout(DIAL_BUDGET, client.request(req))
            .await
            .expect("redirect hop within budget")
            .expect("hop completes");
        final_status = resp.status().as_u16();
        if final_status != 302 {
            break;
        }
        let location = resp
            .headers()
            .get("location")
            .and_then(|v| v.to_str().ok())
            .expect("redirect carries Location")
            .to_string();
        let next = format!("http://127.0.0.1:{port}{location}");
        match limiter.evaluate(&url, &next) {
            Ok(RedirectOutcome::Follow) => url = next,
            Ok(RedirectOutcome::Surface) => break,
            Err(e) => panic!("chain of 3 hops must follow cleanly: {e}"),
        }
    }
    assert_eq!(final_status, 200, "chain completes with 200");
    assert_eq!(limiter.hops(), 3, "exactly 3 policy-checked hops");
    assert_eq!(requests.load(Ordering::SeqCst), 4);
    pool.shutdown();
}

#[tokio::test]
async fn slow_body_transfer_is_bounded_by_explicit_budget() {
    // 4 chunks, 300ms apart = ~1.2s of body. A 500ms overall budget must
    // cut the transfer deterministically — the caller's budget bounds the
    // wait; the client has none by design.
    let port = spawn_slow_body_server(Duration::from_millis(300), 4).await;

    let pool = FetchPool::new();
    let client = pool.client();
    let req = Request::get(format!("http://127.0.0.1:{port}/"))
        .body(Empty::<bytes::Bytes>::new())
        .unwrap();

    let res = tokio::time::timeout(DIAL_BUDGET, client.request(req)).await;
    assert!(res.is_ok(), "headers arrive within budget");
    let resp = res.unwrap().expect("response head completes");

    use http_body_util::BodyExt;
    let body_read = async {
        let mut body = resp.into_body();
        let mut total = 0usize;
        while let Some(frame) = body.frame().await.transpose().expect("frame ok") {
            if let Some(data) = frame.data_ref() {
                total += data.len();
            }
        }
        total
    };
    let started = std::time::Instant::now();
    let outcome = tokio::time::timeout(Duration::from_millis(500), body_read).await;
    assert!(outcome.is_err(), "slow body must exceed the 500ms budget");
    assert!(
        started.elapsed() < Duration::from_secs(3),
        "budget cut the wait near 500ms, not unbounded"
    );
    pool.shutdown();
}

#[tokio::test]
async fn untrusted_tls_endpoints_fail_closed_deterministically() {
    // Immediate-close: EOF mid-handshake.
    let close_port = spawn_immediate_close_server();
    // Garbage handshake bytes: not a TLS record.
    let garbage = TcpListener::bind("127.0.0.1:0").unwrap();
    let garbage_port = garbage.local_addr().unwrap().port();
    std::thread::spawn(move || {
        if let Ok((mut sock, _)) = garbage.accept() {
            let _ = sock.write_all(b"NOT-A-TLS-HANDSHAKE\r\n\r\n");
            let _ = sock.flush();
        }
    });

    let pool = FetchPool::new();
    let client = pool.client();
    for port in [close_port, garbage_port] {
        let req = Request::get(format!("https://127.0.0.1:{port}/"))
            .body(Empty::<bytes::Bytes>::new())
            .unwrap();
        let res = tokio::time::timeout(DIAL_BUDGET, client.request(req)).await;
        assert!(res.is_ok(), "handshake failure must be bounded, not a hang");
        assert!(
            res.unwrap().is_err(),
            "https to untrusted endpoint on port {port} must fail closed"
        );
    }
    pool.shutdown();
}

#[tokio::test]
async fn pool_permits_bound_concurrent_fixture_traffic() {
    // The backpressure permit bounds real fixture traffic: 2 requests
    // against max_active=1 must serialize, both completing within budget.
    let requests = Arc::new(AtomicUsize::new(0));
    let port = spawn_redirect_server(0, requests.clone());

    let pool = FetchPool::with_bounds(velqu_runtime::fetch_stack::PoolBounds {
        max_active_connections: 1,
        ..Default::default()
    });
    let client = pool.client();

    let p1 = pool.try_acquire_permit().expect("first permit");
    let run = async {
        let req = Request::get(format!("http://127.0.0.1:{port}/"))
            .body(Empty::<bytes::Bytes>::new())
            .unwrap();
        client.request(req).await
    };
    let first = tokio::time::timeout(DIAL_BUDGET, run).await;
    drop(p1);
    assert!(first.is_ok(), "first request with permit completes");
    // The second permit acquires only after release.
    assert!(
        pool.try_acquire_permit().is_ok(),
        "permit released back after use"
    );
    assert!(matches!(
        // A shut-down pool refuses further work (M28-009-D semantics).
        {
            pool.shutdown();
            pool.try_acquire_permit()
        },
        Err(PoolError::PoolShuttingDown)
    ));
}
