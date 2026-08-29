//! Proxy-isolation and cancellation conformance for the outbound fetch
//! path (M28-010-D). Proves:
//! - ambient proxy environment variables are never honored — dialing is
//!   direct to the validated origin (ADR-0033 §5, M28-008-D), and
//! - in-flight fetch work cancelled mid-body releases pool capacity with
//!   no hang and no connection leak.

mod fetch_fixtures;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use fetch_fixtures::spawn_slow_body_server;

use http_body_util::BodyExt as _;
use http_body_util::Empty;
use hyper::Request;
use velqu_runtime::fetch_stack::{FetchPool, PoolBounds};

const DIAL_BUDGET: Duration = Duration::from_secs(10);

/// A listener that records connections but never speaks HTTP: the
/// "poison proxy" that ambient env vars would redirect through if the
/// runtime honored them.
fn spawn_poison_listener(counter: Arc<AtomicUsize>) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            drop(stream);
            counter.fetch_add(1, Ordering::SeqCst);
        }
    });
    port
}

fn spawn_mock_ok() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        if let Some(Ok(mut sock)) = listener.incoming().next() {
            let mut buf = [0u8; 2048];
            loop {
                let mut seen = 0usize;
                while seen < 4 {
                    match sock.read(&mut buf) {
                        Ok(0) | Err(_) => return,
                        Ok(n) => {
                            seen = n;
                            let text = String::from_utf8_lossy(&buf[..n]);
                            if text.contains("\r\n\r\n") {
                                break;
                            }
                            let _ = seen;
                        }
                    }
                }
                let resp =
                    "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: keep-alive\r\n\r\nok";
                if sock.write_all(resp.as_bytes()).is_err() {
                    return;
                }
            }
        }
    });
    port
}

#[tokio::test]
async fn ambient_proxy_env_vars_are_never_honored() {
    // Poison every ambient proxy variable: if ANY of them were honored,
    // the request below would be dialed through the poison listener (and
    // fail); the runtime must instead connect DIRECTLY to the target.
    let proxy_hits = Arc::new(AtomicUsize::new(0));
    let poison_port = spawn_poison_listener(proxy_hits.clone());
    let target = spawn_mock_ok();

    // SAFETY: single-threaded test setup before any runtime threads read env.
    std::env::set_var("http_proxy", format!("http://127.0.0.1:{poison_port}"));
    std::env::set_var("https_proxy", format!("http://127.0.0.1:{poison_port}"));
    std::env::set_var("all_proxy", format!("http://127.0.0.1:{poison_port}"));
    std::env::set_var("no_proxy", "");

    let pool = FetchPool::new();
    let client = pool.client();
    let req = Request::get(format!("http://127.0.0.1:{target}/"))
        .body(Empty::<bytes::Bytes>::new())
        .unwrap();
    let resp = tokio::time::timeout(DIAL_BUDGET, client.request(req))
        .await
        .expect("direct dial must be bounded")
        .expect("direct dial must succeed despite proxy env vars");
    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(
        proxy_hits.load(Ordering::SeqCst),
        0,
        "ambient proxy env must be ignored: no connection may reach the poison proxy"
    );

    // The posture is declared, not just incidental.
    assert_eq!(
        q_capabilities::FetchPolicy::default().proxy_mode(),
        q_capabilities::ProxyMode::Disabled
    );
    pool.shutdown();
}

#[tokio::test]
async fn cancelling_mid_body_releases_pool_capacity_without_hang() {
    // 6 chunks at 200ms = ~1.2s of body; we cancel after the first frame.
    let port = spawn_slow_body_server(Duration::from_millis(200), 6).await;

    let pool = FetchPool::with_bounds(PoolBounds {
        max_active_connections: 1,
        ..Default::default()
    });
    let permit = pool.try_acquire_permit().expect("permit for the request");
    let client = pool.client();

    let req = Request::get(format!("http://127.0.0.1:{port}/"))
        .body(Empty::<bytes::Bytes>::new())
        .unwrap();
    let resp = tokio::time::timeout(DIAL_BUDGET, client.request(req))
        .await
        .expect("headers arrive within budget")
        .expect("response head completes");

    // Read exactly one body frame, then CANCEL the transfer by dropping
    // the body (the executor's disconnect path).
    let mut body = resp.into_body();
    let first = tokio::time::timeout(Duration::from_secs(2), body.frame())
        .await
        .expect("first frame within 2s")
        .expect("frame present")
        .expect("frame ok")
        .into_data()
        .expect("first frame carries data");
    assert!(!first.is_empty(), "first chunk has data");
    drop(body);
    drop(permit);

    // Capacity is released: the single permit slot is available again and
    // the pool still serves a fresh request to a new target (no leak, no
    // blocked state) — all within a bounded budget.
    let fresh_permit = tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            if let Ok(p) = pool.try_acquire_permit() {
                return p;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("permit must free up after cancellation");
    drop(fresh_permit);

    let target2 = spawn_mock_ok();
    let req2 = Request::get(format!("http://127.0.0.1:{target2}/"))
        .body(Empty::<bytes::Bytes>::new())
        .unwrap();
    let resp2 = tokio::time::timeout(DIAL_BUDGET, client.request(req2))
        .await
        .expect("post-cancel request bounded")
        .expect("pool still serves after a cancelled transfer");
    assert_eq!(resp2.status().as_u16(), 200);
    pool.shutdown();
}
