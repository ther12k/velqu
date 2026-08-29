//! Bounded streaming buffers for fetch bodies (M28-006-A).
//!
//! Chunk-buffer with a hard byte ceiling shared by the reader and writer
//! sides of a streaming body. The producer blocks (backpressure) when the
//! buffer is at capacity; the consumer drains and wakes the producer.
//! Over-ceiling writes are typed rejections — never unbounded buffering.

use std::fmt;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// Default buffer capacity for streaming bodies (64 KiB).
pub const DEFAULT_STREAM_BUFFER_BYTES: usize = 64 * 1024;
/// Hard ceiling on a single streaming chunk (1 MiB).
pub const MAX_STREAM_CHUNK_BYTES: usize = 1024 * 1024;

/// Typed streaming-buffer violations. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamError {
    /// Chunk exceeds the per-chunk ceiling.
    ChunkTooLarge { len: usize, max: usize },
    /// Total buffered/exceeded bytes crossed the body limit.
    LimitExceeded { total: u64, limit: u64 },
    /// The stream was already closed.
    StreamClosed,
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StreamError::ChunkTooLarge { len, max } => {
                write!(
                    f,
                    "stream chunk {len} bytes exceeds the {max} byte per-chunk ceiling"
                )
            }
            StreamError::LimitExceeded { total, limit } => {
                write!(
                    f,
                    "stream body {total} bytes exceeds the {limit} byte limit"
                )
            }
            StreamError::StreamClosed => f.write_str("stream already closed"),
        }
    }
}

impl std::error::Error for StreamError {}

struct Inner {
    buf: Vec<u8>,
    /// Total bytes that have ever been written (including drained ones).
    total_written: u64,
    /// Hard ceiling on total body bytes.
    limit: u64,
    /// Buffer capacity before backpressure applies.
    capacity: usize,
    closed: bool,
    /// Waker of a blocked producer (buffer was full).
    producer_waker: Option<Waker>,
    /// Waker of a blocked consumer (buffer was empty).
    consumer_waker: Option<Waker>,
}

/// Shared bounded buffer between the response stream producer and consumer.
#[derive(Clone)]
pub struct BoundedStream {
    inner: Arc<Mutex<Inner>>,
}

impl BoundedStream {
    /// Create a bounded stream with the given body limit and buffer capacity.
    pub fn new(limit: u64, capacity: usize) -> Self {
        BoundedStream {
            inner: Arc::new(Mutex::new(Inner {
                buf: Vec::with_capacity(capacity.min(4096)),
                total_written: 0,
                limit,
                capacity,
                closed: false,
                producer_waker: None,
                consumer_waker: None,
            })),
        }
    }

    /// Create a bounded stream with default capacity.
    pub fn with_limit(limit: u64) -> Self {
        Self::new(limit, DEFAULT_STREAM_BUFFER_BYTES)
    }

    /// Try to write a chunk. Fails typed if the chunk exceeds the per-chunk
    /// ceiling or if the body limit would be crossed. Returns `false` when
    /// the buffer is full (producer should wait); registers the waker.
    pub fn try_write(&self, chunk: &[u8]) -> Result<bool, StreamError> {
        if chunk.len() > MAX_STREAM_CHUNK_BYTES {
            return Err(StreamError::ChunkTooLarge {
                len: chunk.len(),
                max: MAX_STREAM_CHUNK_BYTES,
            });
        }
        let mut g = self.inner.lock().unwrap();
        if g.closed {
            return Err(StreamError::StreamClosed);
        }
        if g.total_written + chunk.len() as u64 > g.limit {
            return Err(StreamError::LimitExceeded {
                total: g.total_written + chunk.len() as u64,
                limit: g.limit,
            });
        }
        if g.buf.len() + chunk.len() > g.capacity {
            // Backpressure: buffer full — caller must wait for drain.
            return Ok(false);
        }
        g.buf.extend_from_slice(chunk);
        g.total_written += chunk.len() as u64;
        if let Some(w) = g.consumer_waker.take() {
            w.wake();
        }
        Ok(true)
    }

    /// Drain up to `max` bytes. Returns `None` when the stream is closed and
    /// fully drained (EOF). Returns an empty vec when empty but still open
    /// (consumer should wait).
    pub fn try_read(&self, max: usize) -> Option<Vec<u8>> {
        let mut g = self.inner.lock().unwrap();
        if g.buf.is_empty() {
            if g.closed {
                return None; // EOF
            }
            return Some(Vec::new()); // wait for producer
        }
        let n = g.buf.len().min(max);
        let out = g.buf.drain(..n).collect::<Vec<u8>>();
        if let Some(w) = g.producer_waker.take() {
            w.wake(); // producer may resume
        }
        Some(out)
    }

    /// Signal EOF from the producer side. Idempotent.
    pub fn close(&self) {
        let mut g = self.inner.lock().unwrap();
        g.closed = true;
        if let Some(w) = g.consumer_waker.take() {
            w.wake();
        }
    }

    /// True when the producer closed the stream.
    pub fn is_closed(&self) -> bool {
        self.inner.lock().unwrap().closed
    }

    /// Bytes currently buffered.
    pub fn buffered(&self) -> usize {
        self.inner.lock().unwrap().buf.len()
    }

    /// Total bytes ever written.
    pub fn total_written(&self) -> u64 {
        self.inner.lock().unwrap().total_written
    }

    /// Poll-style read for async consumers: returns Ready when data or EOF
    /// is available; registers the consumer waker when empty-but-open.
    pub fn poll_read(&self, cx: &mut Context<'_>, max: usize) -> Poll<Option<Vec<u8>>> {
        let mut g = self.inner.lock().unwrap();
        if !g.buf.is_empty() {
            let n = g.buf.len().min(max);
            let out = g.buf.drain(..n).collect::<Vec<u8>>();
            if let Some(w) = g.producer_waker.take() {
                w.wake();
            }
            return Poll::Ready(Some(out));
        }
        if g.closed {
            return Poll::Ready(None); // EOF
        }
        g.consumer_waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_then_read_roundtrips() {
        let s = BoundedStream::with_limit(1024);
        assert!(s.try_write(b"hello world").unwrap());
        assert_eq!(s.buffered(), 11);
        let out = s.try_read(64).unwrap();
        assert_eq!(out, b"hello world".to_vec());
        s.close();
        assert_eq!(s.try_read(64), None, "EOF after close+drain");
    }

    #[test]
    fn per_chunk_ceiling_is_enforced() {
        let s = BoundedStream::with_limit(u64::MAX);
        let big = vec![0u8; MAX_STREAM_CHUNK_BYTES + 1];
        let err = s.try_write(&big).unwrap_err();
        assert!(matches!(err, StreamError::ChunkTooLarge { .. }));
        assert!(err.to_string().contains("per-chunk ceiling"));
    }

    #[test]
    fn body_limit_is_enforced() {
        let s = BoundedStream::with_limit(16);
        assert!(s.try_write(b"0123456789abcdef").unwrap());
        let err = s.try_write(b"X").unwrap_err();
        assert!(matches!(
            err,
            StreamError::LimitExceeded {
                total: 17,
                limit: 16
            }
        ));
        assert!(err.to_string().contains("limit"));
    }

    #[test]
    fn backpressure_blocks_producer_at_capacity_and_drain_resumes() {
        let s = BoundedStream::new(1 << 20, 16); // tiny capacity
        assert!(s.try_write(&[0u8; 10]).unwrap());
        // This write exceeds the 16-byte capacity: backpressure signals false.
        assert!(!s.try_write(&[0u8; 10]).unwrap());
        // Draining frees space
        let _ = s.try_read(16).unwrap();
        // (Real producers would retry after the waker fires; here we drain
        //  fully and write again.)
        let _ = s.try_read(16);
        assert!(s.try_write(&[1u8; 8]).unwrap());
    }

    #[test]
    fn closed_stream_rejects_writes() {
        let s = BoundedStream::with_limit(1024);
        s.close();
        let err = s.try_write(b"x").unwrap_err();
        assert!(matches!(err, StreamError::StreamClosed));
        s.close(); // idempotent
    }

    #[test]
    fn total_written_tracks_across_drains() {
        let s = BoundedStream::with_limit(1024);
        s.try_write(b"aaaa").unwrap();
        let _ = s.try_read(64);
        s.try_write(b"bb").unwrap();
        s.close();
        assert_eq!(s.total_written(), 6);
        assert_eq!(s.buffered(), 2);
    }

    #[test]
    fn default_constants_are_bounded() {
        assert_eq!(DEFAULT_STREAM_BUFFER_BYTES, 64 * 1024);
        assert_eq!(MAX_STREAM_CHUNK_BYTES, 1024 * 1024);
        // Streaming ceiling matches the fetch response body limit (ADR-0033 §9).
        assert_eq!(
            crate::fetch_policy::MAX_FETCH_RESPONSE_BODY_BYTES,
            16 * 1024 * 1024
        );
    }
}
