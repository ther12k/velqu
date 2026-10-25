//! #1422 — completed connection tasks must be reaped DURING serving.
//!
//! The accept loop historically joined the connection JoinSet only after
//! shutdown, so every finished connection's task entry stayed retained
//! until process exit — unbounded growth under cumulative connection
//! churn, independent of `max_queue`. This suite drives sequential
//! short-lived connections and asserts the reaped counter catches up to
//! the spawned counter WHILE the server is still serving (before any
//! shutdown signal), that serving continues afterwards, and that drain
//! accounting does not double-count already-reaped connections.

use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;

use q_http::{HttpHost, Limits, PlainResponse};

fn plain_ok() -> PlainResponse {
    PlainResponse {
        status: 200,
        headers: vec![],
        body: b"{\"ok\":true}".to_vec(),
        head_only: false,
    }
}

async fn one_closed_connection(addr: std::net::SocketAddr) {
    let mut sock = TcpStream::connect(addr).await.expect("connect");
    sock.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .await
        .expect("write request");
    let mut buf = Vec::new();
    sock.read_to_end(&mut buf).await.expect("read to EOF");
    assert!(
        buf.starts_with(b"HTTP/1.1 200"),
        "unexpected response: {}",
        String::from_utf8_lossy(&buf[..buf.len().min(64)])
    );
}

#[tokio::test]
async fn completed_connections_are_reaped_while_serving() {
    let host = HttpHost::new(Limits::default());
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local addr");
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let serve_host = host.clone();
    let serve = tokio::spawn(async move {
        serve_host
            .serve(
                listener,
                |_req| async move { (Ok(plain_ok()), "reap-test".to_string(), "reap-test") },
                shutdown_rx,
                Duration::from_secs(10),
            )
            .await
    });

    // sequential connection churn: each connection completes and closes
    // before the next opens — the exact shape that accumulated retained
    // task entries before the fix
    const N: u64 = 64;
    for _ in 0..N {
        one_closed_connection(addr).await;
    }
    assert_eq!(
        host.connections_spawned(),
        N,
        "every accept spawns one task"
    );

    // reaping must happen while still serving — before any shutdown
    // signal. Poll with a generous bound so a loaded runner cannot flake;
    // the assertion is about WHICH PHASE reaps, not raw speed.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while host.connections_reaped() < N {
        assert!(
            tokio::time::Instant::now() < deadline,
            "only {} of {} connections reaped while serving (retention bug)",
            host.connections_reaped(),
            N
        );
        tokio::time::sleep(Duration::from_millis(25)).await;
    }

    // the server is still healthy after churn: one more connection works
    one_closed_connection(addr).await;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while host.connections_reaped() < N + 1 {
        assert!(
            tokio::time::Instant::now() < deadline,
            "post-churn connection was not reaped while serving"
        );
        tokio::time::sleep(Duration::from_millis(25)).await;
    }

    // shutdown accounting: all N+1 connections were reaped during serving,
    // so the drain reports zero completions and zero aborts — the reaped
    // gauge and ServeDrain never double-count the same connection.
    shutdown_tx.send(true).expect("signal shutdown");
    let drain = serve.await.expect("join serve").expect("serve ok");
    assert_eq!(
        (drain.completed, drain.aborted),
        (0, 0),
        "already-reaped connections must not appear in drain accounting"
    );
    assert_eq!(drain.reaped_while_serving, N + 1);
    assert_eq!(host.connections_spawned(), N + 1);
    assert_eq!(host.connections_reaped(), N + 1);
}

#[tokio::test]
async fn keep_alive_connection_counts_once_until_close() {
    // a keep-alive connection stays in the live set while open and is
    // reaped exactly once after the peer closes it
    let host = HttpHost::new(Limits::default());
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local addr");
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let serve_host = host.clone();
    let serve = tokio::spawn(async move {
        serve_host
            .serve(
                listener,
                |_req| async move { (Ok(plain_ok()), "reap-test".to_string(), "reap-test") },
                shutdown_rx,
                Duration::from_secs(10),
            )
            .await
    });

    let mut sock = TcpStream::connect(addr).await.expect("connect");
    sock.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
        .await
        .expect("write request 1");
    let mut buf = vec![0u8; 1024];
    let n = sock.read(&mut buf).await.expect("read response 1");
    assert!(buf[..n].starts_with(b"HTTP/1.1 200"));
    // second request on the SAME connection (keep-alive)
    sock.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .await
        .expect("write request 2");
    let mut tail = Vec::new();
    sock.read_to_end(&mut tail).await.expect("read to EOF");
    assert!(tail.starts_with(b"HTTP/1.1 200"));

    assert_eq!(host.connections_spawned(), 1);
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while host.connections_reaped() < 1 {
        assert!(
            tokio::time::Instant::now() < deadline,
            "closed keep-alive connection was not reaped while serving"
        );
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    assert_eq!(host.connections_reaped(), 1);

    shutdown_tx.send(true).expect("signal shutdown");
    let drain = serve.await.expect("join serve").expect("serve ok");
    assert_eq!((drain.completed, drain.aborted), (0, 0));
    assert_eq!(drain.reaped_while_serving, 1);
}
