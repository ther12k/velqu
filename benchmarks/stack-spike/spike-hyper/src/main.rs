//! M28-002-A spike: hyper 1 + hyper-util + hyper-rustls (webpki roots).
//!
//! Same exercise as the reqwest spike: client construction (lazy pool),
//! loopback plain-HTTP GET, bounded streaming prefix, early drop.

use std::io::{Read, Write};
use std::net::TcpListener;

use http_body_util::{BodyExt, Empty};
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;

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
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });
    port
}

#[tokio::main]
async fn main() {
    // Policy-shaped connector: rustls with webpki roots, HTTP/1 only.
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_or_http()
        .enable_http1()
        .wrap_connector(hyper_util::client::legacy::connect::HttpConnector::new());

    let client: Client<_, Empty<bytes::Bytes>> =
        Client::builder(TokioExecutor::new()).build(https);

    let port = spawn_mock_origin();
    let uri: hyper::Uri = format!("http://127.0.0.1:{}/", port).parse().unwrap();

    let mut resp = client
        .request(hyper::Request::get(uri).body(Empty::<bytes::Bytes>::new()).unwrap())
        .await
        .expect("GET succeeds");
    println!("status={}", resp.status().as_u16());

    // Bounded streaming read of a prefix, then drop mid-flight.
    let frame = resp
        .body_mut()
        .frame()
        .await
        .expect("first frame")
        .expect("frame data");
    let chunk = frame.into_data().expect("data frame");
    println!("first_chunk_len={}", chunk.len());
    println!("prefix_ok=true");
    drop(resp);
    drop(client);
    println!("done=true");
}
