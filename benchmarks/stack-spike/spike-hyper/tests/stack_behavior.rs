//! M28-002-C: behavioral probes of the selected outbound stack
//! (hyper 1 + hyper-util client-legacy pool + hyper-rustls webpki roots).
//!
//! DNS, TLS, and pool semantics are exercised against local mock origins
//! before M28-003 implements the production pool. These tests are the
//! raw evidence for the spike report; they run in this standalone
//! workspace and never touch the Velqu dependency graph.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use http_body_util::{BodyExt, Empty};
use hyper::Request;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;

type StackClient = Client<
    hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>,
    Empty<bytes::Bytes>,
>;

fn build_client() -> StackClient {
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_or_http()
        .enable_http1()
        .wrap_connector(hyper_util::client::legacy::connect::HttpConnector::new());
    Client::builder(TokioExecutor::new()).build(https)
}

/// A mock HTTP/1.1 origin that counts accepted TCP connections and
/// parsed requests, then closes.
fn spawn_counting_origin(counter: Arc<AtomicUsize>, requests: Arc<AtomicUsize>) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut sock) = stream else { break };
            counter.fetch_add(1, Ordering::SeqCst);
            // Serve up to 8 keepalive requests on this connection.
            let mut served = 0;
            while served < 8 {
                let mut buf = [0u8; 4096];
                match sock.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        let req = String::from_utf8_lossy(&buf[..n]).to_string();
                        if req.contains("\r\n\r\n") || req.contains("HTTP/1.1") {
                            requests.fetch_add(1, Ordering::SeqCst);
                        }
                        let resp =
                            "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: keep-alive\r\n\r\nok";
                        if sock.write_all(resp.as_bytes()).is_err() {
                            break;
                        }
                        served += 1;
                    }
                }
            }
        }
    });
    port
}

async fn get(client: &StackClient, uri: &str) -> u16 {
    let req = Request::get(uri)
        .body(Empty::<bytes::Bytes>::new())
        .unwrap();
    let resp = client.request(req).await.expect("request completes");
    resp.status().as_u16()
}

// --- Pool behavior -----------------------------------------------------------

/// Two sequential requests to the same origin reuse one pooled
/// connection (keepalive): exactly 1 accept serves 2 requests.
#[tokio::test]
async fn pool_reuses_connection_for_sequential_same_origin_requests() {
    let accepts = Arc::new(AtomicUsize::new(0));
    let reqs = Arc::new(AtomicUsize::new(0));
    let port = spawn_counting_origin(accepts.clone(), reqs.clone());

    let client = build_client();
    let uri = format!("http://127.0.0.1:{port}/");
    assert_eq!(get(&client, &uri).await, 200);
    assert_eq!(get(&client, &uri).await, 200);
    drop(client); // drop pooled connections before counting
    std::thread::sleep(std::time::Duration::from_millis(50));

    assert_eq!(reqs.load(Ordering::SeqCst), 2, "both requests served");
    assert_eq!(
        accepts.load(Ordering::SeqCst),
        1,
        "pool must reuse the connection for the second request"
    );
}

/// Different origins get distinct pooled connections.
#[tokio::test]
async fn pool_dials_a_separate_connection_per_origin() {
    let accepts_a = Arc::new(AtomicUsize::new(0));
    let reqs_a = Arc::new(AtomicUsize::new(0));
    let port_a = spawn_counting_origin(accepts_a.clone(), reqs_a.clone());
    let accepts_b = Arc::new(AtomicUsize::new(0));
    let reqs_b = Arc::new(AtomicUsize::new(0));
    let port_b = spawn_counting_origin(accepts_b.clone(), reqs_b.clone());

    let client = build_client();
    assert_eq!(get(&client, &format!("http://127.0.0.1:{port_a}/")).await, 200);
    assert_eq!(get(&client, &format!("http://127.0.0.1:{port_b}/")).await, 200);
    drop(client);
    std::thread::sleep(std::time::Duration::from_millis(50));

    assert_eq!(accepts_a.load(Ordering::SeqCst), 1);
    assert_eq!(accepts_b.load(Ordering::SeqCst), 1, "distinct origin, distinct connection");
}

// --- DNS behavior ------------------------------------------------------------

/// The connector resolves hostnames through the system resolver
/// (`localhost` → loopback) — the resolution path M28-008-A will wrap
/// with the ADR-0033 §3 validate-after-resolve pipeline.
#[tokio::test]
async fn dns_hostname_resolution_reaches_loopback_origin() {
    let accepts = Arc::new(AtomicUsize::new(0));
    let reqs = Arc::new(AtomicUsize::new(0));
    let port = spawn_counting_origin(accepts.clone(), reqs.clone());

    let client = build_client();
    // Hostname (not raw IP): forces the connector through DNS resolution.
    let uri = format!("http://localhost:{port}/");
    assert_eq!(get(&client, &uri).await, 200);
    assert_eq!(reqs.load(Ordering::SeqCst), 1);
}

/// A host that cannot resolve fails typed (connect error), never hangs.
#[tokio::test]
async fn dns_unresolvable_host_fails_typed_and_fast() {
    let client = build_client();
    // RFC 6761 reserved invalid TLD — never resolvable.
    let started = std::time::Instant::now();
    let req = Request::get("http://nonexistent.invalid/")
        .body(Empty::<bytes::Bytes>::new())
        .unwrap();
    let result = client.request(req).await;
    let elapsed = started.elapsed();
    assert!(result.is_err(), "unresolvable host must fail");
    assert!(
        elapsed < std::time::Duration::from_secs(10),
        "resolution failure must be fast, took {elapsed:?}"
    );
}

// --- TLS behavior --------------------------------------------------------------

/// A real rustls TLS server presenting a SELF-SIGNED certificate: the
/// webpki-roots-only client must reject the unknown CA fail-closed —
/// hostname/root validation is mandatory and the policy-shaped
/// connector exposes no bypass knob (ADR-0033 §6).
#[tokio::test]
async fn tls_self_signed_certificate_is_rejected_fail_closed() {
    use rcgen::generate_simple_self_signed;
    use rustls::ServerConfig;
    use rustls_pki_types::pem::PemObject;
    use rustls_pki_types::{CertificateDer, PrivateKeyDer};
    use tokio::net::TcpListener as TlsListener;
    use tokio_rustls::TlsAcceptor;

    let cert = generate_simple_self_signed(vec!["127.0.0.1".to_string()]).unwrap();
    let cert_der = CertificateDer::from_pem_slice(cert.cert.pem().as_bytes()).unwrap();
    let key_der = PrivateKeyDer::from_pem_slice(cert.key_pair.serialize_pem().as_bytes()).unwrap();

    let listener = TlsListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let server_config =
        ServerConfig::builder().with_no_client_auth().with_single_cert(vec![cert_der], key_der).unwrap();
    let acceptor = TlsAcceptor::from(std::sync::Arc::new(server_config));

    let server = tokio::spawn(async move {
        if let Ok((sock, _)) = listener.accept().await {
            let _ = acceptor.accept(sock).await; // handshake may or may not complete
        }
    });

    let client = build_client();
    let req = Request::get(format!("https://127.0.0.1:{port}/"))
        .body(Empty::<bytes::Bytes>::new())
        .unwrap();
    let started = std::time::Instant::now();
    let result = client.request(req).await;
    let elapsed = started.elapsed();

    assert!(
        result.is_err(),
        "self-signed cert must be rejected by the webpki-roots client"
    );
    assert!(
        elapsed < std::time::Duration::from_secs(10),
        "TLS rejection must be fast, took {elapsed:?}"
    );
    server.abort();
}

// --- Backpressure / streaming ---------------------------------------------------

/// A large streaming body can be consumed in bounded chunks and
/// dropped mid-stream (backpressure + cancellation semantics that
/// M28-006 builds on).
#[tokio::test]
async fn streaming_body_supports_bounded_prefix_and_early_drop() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = std::thread::spawn(move || {
        if let Ok((mut sock, _)) = listener.accept() {
            let mut buf = [0u8; 2048];
            let _ = sock.read(&mut buf);
            let _ = sock.write_all(
                b"HTTP/1.1 200 OK\r\nContent-Length: 1048576\r\nConnection: close\r\n\r\n",
            );
            // Stream 1 MiB in chunks; the client drops early.
            let chunk = [b'x'; 8192];
            for _ in 0..128 {
                if sock.write_all(&chunk).is_err() {
                    break; // client dropped — server observes the cancel
                }
            }
        }
    });

    let client = build_client();
    let req = Request::get(format!("http://127.0.0.1:{port}/"))
        .body(Empty::<bytes::Bytes>::new())
        .unwrap();
    let mut resp = client.request(req).await.expect("response");
    assert_eq!(resp.status().as_u16(), 200);
    // Read exactly one bounded frame, then drop mid-stream.
    let frame = resp.body_mut().frame().await.expect("frame").expect("data");
    let chunk = frame.into_data().expect("data frame");
    assert!(!chunk.is_empty() && chunk.len() <= 8192 + 1, "bounded chunk");
    drop(resp);
    drop(client);
    server.join().unwrap();
}
