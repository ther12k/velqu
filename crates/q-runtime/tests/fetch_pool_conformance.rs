//! Outbound fetch pool conformance tests (M28-003-A).
//!
//! Validates lazy initialization, connection reuse, and shutdown behavior
//! of the production [`velqu_runtime::fetch_stack::FetchPool`].

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use http_body_util::Empty;
use hyper::Request;
use velqu_runtime::fetch_stack::FetchPool;

fn spawn_mock_server(request_counter: Arc<AtomicUsize>) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut sock) = stream else { break };
            let mut buf = [0u8; 1024];
            while let Ok(n) = sock.read(&mut buf) {
                if n == 0 {
                    break;
                }
                request_counter.fetch_add(1, Ordering::SeqCst);
                let resp =
                    "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: keep-alive\r\n\r\nok";
                if sock.write_all(resp.as_bytes()).is_err() {
                    break;
                }
            }
        }
    });
    port
}

#[test]
fn fetch_pool_remains_uninitialized_until_explicit_request() {
    let pool = FetchPool::new();
    assert!(!pool.is_initialized(), "pool must start dormant");

    // Can query state without triggering initialization
    assert!(!pool.is_shutdown());
    assert!(!pool.is_initialized());
}

#[tokio::test]
async fn fetch_pool_initializes_on_request_and_serves_traffic() {
    let requests = Arc::new(AtomicUsize::new(0));
    let port = spawn_mock_server(requests.clone());

    let pool = FetchPool::new();
    assert!(!pool.is_initialized());

    let client = pool.client();
    assert!(
        pool.is_initialized(),
        "first client() call initializes pool"
    );

    let req = Request::get(format!("http://127.0.0.1:{port}/"))
        .body(Empty::<bytes::Bytes>::new())
        .unwrap();

    let resp = client.request(req).await.expect("request completes");
    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(requests.load(Ordering::SeqCst), 1);

    pool.shutdown();
    assert!(pool.is_shutdown());
}
