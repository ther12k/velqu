//! Bounded streaming buffers for fetch bodies (M28-006-A).
//!
//! Chunk-buffer with a hard byte ceiling shared by the reader and writer
//! sides of a streaming body. The producer blocks (backpressure) when the
//! buffer is at capacity; the consumer drains and wakes the producer.
//! Over-ceiling writes are typed rejections — never unbounded buffering.

use std::fmt;
use std::future::Future;
use std::pin::Pin;
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
    /// High-water mark of buffered bytes observed since creation.
    max_buffered: u64,
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
                max_buffered: 0,
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
    /// the buffer is full (the producer must retry later — no waker is
    /// registered; async producers use [`BoundedStream::poll_write`]).
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
        g.max_buffered = g.max_buffered.max(g.buf.len() as u64);
        if let Some(w) = g.consumer_waker.take() {
            w.wake();
        }
        Ok(true)
    }

    /// Poll-style write for async producers: writes when buffer space
    /// allows; when the buffer is full the producer waker is registered and
    /// `Poll::Pending` is returned, suspending the producer task until the
    /// consumer drains. This is the backpressure propagation primitive —
    /// downstream slowness upstreams as task suspension, never as unbounded
    /// buffering. Single-producer/single-consumer contract: a second
    /// concurrent writer would overwrite the registered producer waker.
    pub fn poll_write(&self, cx: &mut Context<'_>, chunk: &[u8]) -> Poll<Result<(), StreamError>> {
        if chunk.len() > MAX_STREAM_CHUNK_BYTES {
            return Poll::Ready(Err(StreamError::ChunkTooLarge {
                len: chunk.len(),
                max: MAX_STREAM_CHUNK_BYTES,
            }));
        }
        let mut g = self.inner.lock().unwrap();
        if g.closed {
            return Poll::Ready(Err(StreamError::StreamClosed));
        }
        if g.total_written + chunk.len() as u64 > g.limit {
            return Poll::Ready(Err(StreamError::LimitExceeded {
                total: g.total_written + chunk.len() as u64,
                limit: g.limit,
            }));
        }
        if g.buf.len() + chunk.len() > g.capacity {
            // Downstream is slower than the producer: park until drained.
            g.producer_waker = Some(cx.waker().clone());
            return Poll::Pending;
        }
        g.buf.extend_from_slice(chunk);
        g.total_written += chunk.len() as u64;
        g.max_buffered = g.max_buffered.max(g.buf.len() as u64);
        if let Some(w) = g.consumer_waker.take() {
            w.wake();
        }
        Poll::Ready(Ok(()))
    }

    /// Async producer write: resolves once `chunk` is fully buffered.
    /// Suspends (backpressure) while the consumer is slower than the chunk
    /// inflow. Typed failures surface as `Err` on first poll.
    pub fn write_chunk(&self, chunk: Vec<u8>) -> WriteChunk<'_> {
        WriteChunk {
            stream: self,
            chunk: Some(chunk),
        }
    }

    /// Async consumer read: resolves with up to `max` bytes, or `None` at
    /// EOF (stream closed and fully drained). Suspends while empty-but-open.
    pub fn read_chunk(&self, max: usize) -> ReadChunk<'_> {
        ReadChunk { stream: self, max }
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

    /// Configured buffer capacity (the backpressure threshold).
    pub fn capacity(&self) -> usize {
        self.inner.lock().unwrap().capacity
    }

    /// High-water mark of buffered bytes observed since creation. For a
    /// correctly driven pump this never exceeds [`BoundedStream::capacity`].
    pub fn max_buffered(&self) -> u64 {
        self.inner.lock().unwrap().max_buffered
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

/// Future produced by [`BoundedStream::write_chunk`]: resolves once the
/// chunk is fully buffered (or with a typed error).
pub struct WriteChunk<'a> {
    stream: &'a BoundedStream,
    chunk: Option<Vec<u8>>,
}

impl Future for WriteChunk<'_> {
    type Output = Result<(), StreamError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let chunk = self
            .chunk
            .as_deref()
            .expect("WriteChunk polled after completion");
        match self.stream.poll_write(cx, chunk) {
            Poll::Ready(result) => {
                self.chunk = None;
                Poll::Ready(result)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Future produced by [`BoundedStream::read_chunk`]: resolves with buffered
/// bytes (`Some`) or EOF (`None`).
pub struct ReadChunk<'a> {
    stream: &'a BoundedStream,
    max: usize,
}

impl Future for ReadChunk<'_> {
    type Output = Option<Vec<u8>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.stream.poll_read(cx, self.max)
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

    use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
    use std::task::Wake;

    struct CountingWaker(AtomicUsize);
    impl Wake for CountingWaker {
        fn wake(self: std::sync::Arc<Self>) {
            self.0.fetch_add(1, AtomicOrdering::SeqCst);
        }
    }

    #[test]
    fn poll_write_pends_when_full_and_wakes_after_drain() {
        let s = BoundedStream::new(1 << 20, 16);
        assert!(s.try_write(b"0123456789abcdef").unwrap()); // buffer at capacity
        let counter = std::sync::Arc::new(CountingWaker(AtomicUsize::new(0)));
        let waker = Waker::from(counter.clone());
        let mut cx = Context::from_waker(&waker);
        // Full buffer: producer side must suspend, not error and not buffer.
        assert!(matches!(s.poll_write(&mut cx, b"X"), Poll::Pending));
        assert_eq!(counter.0.load(AtomicOrdering::SeqCst), 0);
        // Consumer drain wakes the parked producer exactly once.
        let out = s.try_read(16).unwrap();
        assert_eq!(out.len(), 16);
        assert_eq!(counter.0.load(AtomicOrdering::SeqCst), 1);
        // With capacity free the same write now completes.
        assert!(matches!(s.poll_write(&mut cx, b"X"), Poll::Ready(Ok(()))));
    }

    #[test]
    fn write_chunk_future_resolves_after_capacity_frees() {
        let s = BoundedStream::new(1 << 20, 8);
        let mut cx = Context::from_waker(Waker::noop());
        let mut fut = std::pin::pin!(s.write_chunk(b"abcdefgh".to_vec()));
        assert!(matches!(fut.as_mut().poll(&mut cx), Poll::Ready(Ok(()))));
        let mut blocked = std::pin::pin!(s.write_chunk(b"Z".to_vec()));
        assert!(matches!(blocked.as_mut().poll(&mut cx), Poll::Pending));
        let _ = s.try_read(8);
        assert!(matches!(
            blocked.as_mut().poll(&mut cx),
            Poll::Ready(Ok(()))
        ));
    }

    #[test]
    fn poll_write_typed_errors_fail_closed() {
        let mut cx = Context::from_waker(Waker::noop());
        let big = BoundedStream::with_limit(u64::MAX);
        let oversized = vec![0u8; MAX_STREAM_CHUNK_BYTES + 1];
        assert!(matches!(
            big.poll_write(&mut cx, &oversized),
            Poll::Ready(Err(StreamError::ChunkTooLarge { .. }))
        ));
        let s = BoundedStream::with_limit(4);
        assert!(matches!(
            s.poll_write(&mut cx, b"abcd"),
            Poll::Ready(Ok(()))
        ));
        assert!(matches!(
            s.poll_write(&mut cx, b"X"),
            Poll::Ready(Err(StreamError::LimitExceeded { total: 5, limit: 4 }))
        ));
        s.close();
        assert!(matches!(
            s.poll_write(&mut cx, b"X"),
            Poll::Ready(Err(StreamError::StreamClosed))
        ));
    }

    #[test]
    fn max_buffered_tracks_peak_and_never_exceeds_capacity() {
        let s = BoundedStream::new(1 << 20, 16);
        assert_eq!(s.capacity(), 16);
        s.try_write(&[0u8; 10]).unwrap();
        assert_eq!(s.max_buffered(), 10);
        let drained = s.try_read(4).unwrap();
        assert_eq!(drained.len(), 4);
        s.try_write(&[0u8; 8]).unwrap(); // 6 buffered + 8 = 14
        assert_eq!(s.buffered(), 14);
        assert_eq!(s.max_buffered(), 14);
        assert!(s.max_buffered() <= s.capacity() as u64);
    }
}
