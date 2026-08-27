//! M28-002-A spike: reqwest 0.12 (rustls-tls-webpki-roots, nothing else).
//!
//! Exercises: client construction (lazy), a loopback plain-HTTP GET,
//! streaming read of a bounded prefix, and early-drop cancellation —
//! then exits. Prints machine-readable markers for the harness.

use std::io::{Read, Write};
use std::net::TcpListener;

fn spawn_mock_origin() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        if let Ok((mut sock, _)) = listener.accept() {
            let mut buf = [0u8; 2048];
            let _n = sock.read(&mut buf);
            let body = "0123456789".repeat(4096); // 40 KiB body
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = sock.write_all(resp.as_bytes());
            let _ = sock.flush();
            // Hold the socket briefly so early client drop visibly cancels.
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });
    port
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .user_agent("velqu-stack-spike/0.1")
        .build()
        .expect("client construction must not require eager I/O");

    let port = spawn_mock_origin();
    let url = format!("http://127.0.0.1:{}/", port);

    let mut resp = client.get(&url).send().await.expect("GET succeeds");
    println!("status={}", resp.status().as_u16());

    // Bounded streaming read: consume exactly 64 KiB-1... our body is 40 KiB,
    // so read a bounded prefix then drop the connection mid-flight.
    let chunk = resp.chunk().await.expect("first chunk").expect("chunk");
    println!("first_chunk_len={}", chunk.len());
    let _prefix = &chunk[..chunk.len().min(16)]; // touch a prefix
    println!("prefix_ok=true");
    // Early drop: cancelling the response future/connection is exercised
    // simply by falling off the end here without draining the body.
    drop(resp);
    drop(client);
    println!("done=true");
}
