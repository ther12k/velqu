//! Deterministic fetch fixtures (M28-010-B).
//!
//! Each integration test file includes this module and uses the fixtures
//! it needs; the rest are intentionally available (allow(dead_code)).
#![allow(dead_code)]
//!
//! Hermetic, reproducible fixtures for the fetch executor conformance:
//! canned DNS resolvers (including a rebinding-sequence attacker model),
//! redirect-chain and slow-body HTTP servers, and untrusted-TLS endpoints
//! that must fail verification. No fixture performs live network egress.

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{IpAddr, TcpListener};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// A canned, deterministic DNS resolver: answers from `table`, errors for
/// unknown hosts. Every call re-reads the same table — resolution cannot
/// drift between calls.
pub fn deterministic_resolver(
    table: BTreeMap<&'static str, Vec<IpAddr>>,
) -> impl FnMut(&str) -> Result<Vec<IpAddr>, String> {
    move |host: &str| match table.get(host) {
        Some(addrs) => Ok(addrs.clone()),
        None => Err(format!("nx-domain: {host}")),
    }
}

/// A rebinding attacker resolver: returns `first` on every call, but counts
/// resolutions so tests can prove the executor resolves exactly once and
/// dials the pin set — the second answer is never even consulted.
pub struct RebindingResolver {
    answer: Vec<IpAddr>,
    calls: std::cell::Cell<usize>,
}

impl RebindingResolver {
    pub fn new(answer: Vec<IpAddr>) -> Self {
        RebindingResolver {
            answer,
            calls: std::cell::Cell::new(0),
        }
    }

    pub fn resolve(&self, _host: &str) -> Result<Vec<IpAddr>, String> {
        self.calls.set(self.calls.get() + 1);
        Ok(self.answer.clone())
    }

    pub fn calls(&self) -> usize {
        self.calls.get()
    }
}

/// Spawn a thread-based HTTP server that answers `/hop/N` with `302
/// Location: /hop/N+1` for `chain_len` hops, then `200 ok`. Returns the
/// port and a request counter.
pub fn spawn_redirect_server(chain_len: usize, requests: Arc<AtomicUsize>) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        // The fixture serves the first accepted connection until it closes;
        // the pool under test reuses that single keep-alive connection.
        if let Some(Ok(mut sock)) = listener.incoming().next() {
            let mut buf = [0u8; 2048];
            // Keep-alive: parse each request head independently, resetting
            // the accumulation buffer per request. Connection-level
            // failures end the fixture.
            loop {
                let mut req = String::new();
                loop {
                    match sock.read(&mut buf) {
                        Ok(0) => return,
                        Ok(n) => {
                            req.push_str(&String::from_utf8_lossy(&buf[..n]));
                            if req.contains("\r\n\r\n") {
                                break;
                            }
                        }
                        Err(_) => return,
                    }
                }
                let path = req.split_whitespace().nth(1).unwrap_or("/").to_string();
                requests.fetch_add(1, Ordering::SeqCst);
                let hop: usize = path.trim_start_matches("/hop/").parse().unwrap_or(0);
                let resp = if hop < chain_len {
                    format!(
                        "HTTP/1.1 302 Found\r\nLocation: /hop/{}\r\nContent-Length: 0\r\nConnection: keep-alive\r\n\r\n",
                        hop + 1
                    )
                } else {
                    "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: keep-alive\r\n\r\nok"
                        .to_string()
                };
                if sock.write_all(resp.as_bytes()).is_err() {
                    return;
                }
            }
        }
    });
    port
}

/// Spawn a Tokio slow-body server: full headers immediately, then body
/// chunks one `delay` apart. Returns the port. The caller must drive it on
/// a Tokio runtime (spawn inside a test runtime).
pub async fn spawn_slow_body_server(delay: Duration, chunks: usize) -> u16 {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let (mut sock, _) = match listener.accept().await {
            Ok(s) => s,
            Err(_) => return,
        };
        // Wait for the request head, then answer with a dribbled body.
        let mut buf = [0u8; 1024];
        let _ = tokio::time::timeout(Duration::from_secs(5), sock.read(&mut buf)).await;
        let head = "HTTP/1.1 200 OK\r\nContent-Length: 8\r\nConnection: close\r\n\r\n";
        if sock.write_all(head.as_bytes()).await.is_err() {
            return;
        }
        for i in 0..chunks {
            tokio::time::sleep(delay).await;
            let byte = [b'a' + (i as u8 % 26)];
            if sock.write_all(&byte).await.is_err() {
                return;
            }
        }
        let _ = sock.flush().await;
    });
    port
}

/// Spawn a server that accepts and immediately closes the connection —
/// EOF during any handshake or body read.
pub fn spawn_immediate_close_server() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            drop(stream);
        }
    });
    port
}
