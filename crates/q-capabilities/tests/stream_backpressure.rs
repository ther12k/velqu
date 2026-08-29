//! M28-006-B — downstream backpressure propagation integration tests.
//!
//! Proves the parent guardrails at the buffer/pump boundary:
//! - a slow downstream suspends the producer (task suspension, never
//!   unbounded buffering),
//! - buffered memory stays at the buffer capacity regardless of body size,
//! - streaming errors are typed on the async path,
//! - large streaming loads keep bytes ordered and totals exact.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use q_capabilities::{
    BoundedStream, StreamError, DEFAULT_STREAM_BUFFER_BYTES, MAX_FETCH_RESPONSE_BODY_BYTES,
    MAX_STREAM_CHUNK_BYTES,
};

const CHUNK_LEN: usize = 32 * 1024;
const TOTAL_BODY_BYTES: u64 = 8 * 1024 * 1024; // 8 MiB streamed body

// Compile-time: every streamed chunk fits the per-chunk ceiling.
const _: () = assert!(MAX_STREAM_CHUNK_BYTES >= CHUNK_LEN);

/// Deterministic chunk payload: byte i of the body is `i mod 251`, so any
/// reordering or loss changes the checksum.
fn payload_byte(pos: u64) -> u8 {
    (pos % 251) as u8
}

fn fnv1a(bytes: &[u8], mut hash: u64) -> u64 {
    for b in bytes {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[tokio::test(flavor = "multi_thread")]
async fn slow_consumer_suspends_producer_and_stays_bounded() {
    let stream = BoundedStream::new(MAX_FETCH_RESPONSE_BODY_BYTES, DEFAULT_STREAM_BUFFER_BYTES);
    let stream = Arc::new(stream);
    let written = Arc::new(AtomicU64::new(0));

    // Producer: streams the full 8 MiB body in 32 KiB chunks, suspending
    // inside write_chunk whenever the consumer falls behind.
    let producer_stream = stream.clone();
    let producer_written = written.clone();
    let producer = tokio::spawn(async move {
        let mut pos: u64 = 0;
        while pos < TOTAL_BODY_BYTES {
            let len = CHUNK_LEN.min((TOTAL_BODY_BYTES - pos) as usize);
            let chunk: Vec<u8> = (pos..pos + len as u64).map(payload_byte).collect();
            producer_stream.write_chunk(chunk).await.unwrap();
            producer_written.fetch_add(len as u64, Ordering::SeqCst);
            pos += len as u64;
        }
        producer_stream.close();
    });

    // Phase 1 — the consumer is not reading at all. The producer must stall
    // at the buffer capacity (64 KiB) and cannot finish no matter how long
    // it is given: backpressure is structural, not timing-dependent.
    tokio::time::sleep(Duration::from_millis(150)).await;
    let stalled_at = written.load(Ordering::SeqCst);
    assert!(
        stalled_at as usize <= DEFAULT_STREAM_BUFFER_BYTES,
        "producer wrote {stalled_at} bytes with no consumer; capacity is {}",
        DEFAULT_STREAM_BUFFER_BYTES
    );
    assert!(
        stalled_at >= CHUNK_LEN as u64,
        "producer made no initial progress"
    );
    let mut producer_join = Box::pin(producer);
    let still_running = tokio::time::timeout(Duration::from_millis(150), &mut *producer_join).await;
    assert!(
        still_running.is_err(),
        "producer finished while the consumer was fully stopped"
    );

    // Phase 2 — consumer drains slowly (1 ms per read). The producer resumes
    // only as the consumer frees capacity, and the pump completes.
    let consumer_stream = stream.clone();
    let consumer = tokio::spawn(async move {
        let mut received: Vec<u8> = Vec::new();
        let mut hash: u64 = 0xcbf29ce484222325;
        while let Some(chunk) = consumer_stream
            .read_chunk(DEFAULT_STREAM_BUFFER_BYTES)
            .await
        {
            hash = fnv1a(&chunk, hash);
            received.extend_from_slice(&chunk);
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
        (received.len() as u64, hash)
    });

    let (received_len, rx_hash) = tokio::time::timeout(Duration::from_secs(30), consumer)
        .await
        .expect("consumer pump timed out")
        .expect("consumer task panicked");
    tokio::time::timeout(Duration::from_secs(30), producer_join)
        .await
        .expect("producer never resumed after drain")
        .expect("producer task panicked");

    assert_eq!(received_len, TOTAL_BODY_BYTES);

    // Reference checksum over the same deterministic payload.
    let mut reference_hash: u64 = 0xcbf29ce484222325;
    let mut pos: u64 = 0;
    while pos < TOTAL_BODY_BYTES {
        let len = CHUNK_LEN.min((TOTAL_BODY_BYTES - pos) as usize);
        let chunk: Vec<u8> = (pos..pos + len as u64)
            .map(payload_byte)
            .collect::<Vec<u8>>();
        reference_hash = fnv1a(&chunk, reference_hash);
        pos += len as u64;
    }
    assert_eq!(rx_hash, reference_hash, "streamed bytes were reordered");

    // Memory profile: buffered memory is bounded by capacity, independent
    // of the 8 MiB body size.
    assert_eq!(stream.total_written(), TOTAL_BODY_BYTES);
    assert!(
        stream.max_buffered() <= DEFAULT_STREAM_BUFFER_BYTES as u64,
        "peak buffered {} exceeded capacity {}",
        stream.max_buffered(),
        DEFAULT_STREAM_BUFFER_BYTES
    );
    println!(
        "m28-006-b profile: body={}B chunk={}B capacity={}B peak_buffered={}B",
        TOTAL_BODY_BYTES,
        CHUNK_LEN,
        DEFAULT_STREAM_BUFFER_BYTES,
        stream.max_buffered()
    );
}

#[tokio::test]
async fn typed_errors_propagate_through_async_write_path() {
    let waker_stream = BoundedStream::with_limit(4);
    waker_stream.write_chunk(b"abcd".to_vec()).await.unwrap();
    let err = waker_stream.write_chunk(b"X".to_vec()).await.unwrap_err();
    assert!(matches!(
        err,
        StreamError::LimitExceeded { total: 5, limit: 4 }
    ));

    let closed = BoundedStream::with_limit(64);
    closed.close();
    assert!(matches!(
        closed.write_chunk(b"X".to_vec()).await,
        Err(StreamError::StreamClosed)
    ));
}

#[tokio::test]
async fn streaming_load_profile_peak_stays_at_capacity_bound() {
    // Fast consumer: measures the best-case profile. Peak buffered must stay
    // at the capacity bound while streaming the full body.
    let stream = BoundedStream::new(MAX_FETCH_RESPONSE_BODY_BYTES, DEFAULT_STREAM_BUFFER_BYTES);
    let total_chunks = TOTAL_BODY_BYTES / CHUNK_LEN as u64;
    for i in 0..total_chunks {
        let pos = i * CHUNK_LEN as u64;
        let chunk: Vec<u8> = (pos..pos + CHUNK_LEN as u64).map(payload_byte).collect();
        stream.write_chunk(chunk).await.unwrap();
        let _ = stream.read_chunk(CHUNK_LEN * 2).await.unwrap();
    }
    stream.close();
    assert_eq!(stream.total_written(), TOTAL_BODY_BYTES);
    assert_eq!(stream.read_chunk(1).await, None, "EOF after close+drain");
    assert!(stream.max_buffered() > 0);
    assert!(stream.max_buffered() <= DEFAULT_STREAM_BUFFER_BYTES as u64);
    println!(
        "m28-006-b fast-consumer profile: body={}B peak_buffered={}B capacity={}B",
        TOTAL_BODY_BYTES,
        stream.max_buffered(),
        DEFAULT_STREAM_BUFFER_BYTES
    );
}
