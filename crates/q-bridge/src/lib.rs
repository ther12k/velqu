//! q-bridge — native request storage backing lazy, generation-checked JS handles.
//!
//! A RequestStore is deliberately not thread-safe: the production instance is
//! created inside one QuickJS worker and accessed through that worker's
//! `Rc<RefCell<_>>`. The public counters remain atomic so a read-only snapshot
//! can be observed by the host without sharing request bytes or slab state.

use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub use q_engine::RequestMeta;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotState {
    Active,
    Settled,
}

struct Slot {
    generation: u64,
    meta: Option<RequestMeta>,
    state: SlotState,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BridgeError {
    #[error("request handle expired (slot settled or cancelled): access denied")]
    Expired,
    #[error("request handle does not exist")]
    NoSuchSlot,
    #[error("request slab capacity reached")]
    Capacity,
}

/// Worker-to-host laziness and slab accounting. Request bytes remain worker
/// local; only these bounded scalar counters are observable across the seam.
#[derive(Debug, Default)]
pub struct BridgeCounters {
    pub host_calls: AtomicU64,
    pub materialized_fields: AtomicU64,
    pub materialized_bytes: AtomicU64,
    pub expired_accesses: AtomicU64,
    pub live_slots: AtomicU64,
}

impl BridgeCounters {
    pub fn snapshot(&self) -> CountersSnapshot {
        CountersSnapshot {
            host_calls: self.host_calls.load(Ordering::Relaxed),
            materialized_fields: self.materialized_fields.load(Ordering::Relaxed),
            materialized_bytes: self.materialized_bytes.load(Ordering::Relaxed),
            expired_accesses: self.expired_accesses.load(Ordering::Relaxed),
            live_slots: self.live_slots.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct CountersSnapshot {
    pub host_calls: u64,
    pub materialized_fields: u64,
    pub materialized_bytes: u64,
    pub expired_accesses: u64,
    pub live_slots: u64,
}

/// A bounded slab owned by one worker. The RefCell is only an implementation
/// convenience for synchronous native callbacks; production code wraps this
/// value in an Rc that never leaves the worker thread.
pub struct RequestStore {
    slots: RefCell<Vec<Slot>>,
    free: RefCell<Vec<usize>>,
    counters: Arc<BridgeCounters>,
    generation_clock: RefCell<u64>,
    capacity: usize,
}

impl Default for RequestStore {
    fn default() -> Self {
        Self::with_capacity(usize::MAX)
    }
}

impl RequestStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_counters(capacity, Arc::new(BridgeCounters::default()))
    }

    pub fn with_capacity_and_counters(capacity: usize, counters: Arc<BridgeCounters>) -> Self {
        RequestStore {
            slots: RefCell::new(Vec::with_capacity(capacity.min(1024))),
            free: RefCell::new(Vec::new()),
            counters,
            generation_clock: RefCell::new(1),
            capacity,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn counters(&self) -> &BridgeCounters {
        self.counters.as_ref()
    }

    pub fn counters_arc(&self) -> Arc<BridgeCounters> {
        Arc::clone(&self.counters)
    }

    pub fn snapshot(&self) -> CountersSnapshot {
        self.counters.snapshot()
    }

    /// Insert a request, returning an opaque slot/generation pair. This helper
    /// is retained for local tests and compatibility; worker admission uses the
    /// fallible form so a full slab cannot grow or panic.
    pub fn insert(&self, meta: RequestMeta) -> (usize, u64) {
        self.try_insert(meta)
            .expect("request slab capacity reached")
    }

    pub fn try_insert(&self, meta: RequestMeta) -> Result<(usize, u64), BridgeError> {
        let mut generation = self.generation_clock.borrow_mut();
        let current = *generation;
        *generation = generation.wrapping_add(1).max(1);
        let mut slots = self.slots.borrow_mut();
        let mut free = self.free.borrow_mut();
        let slot = if let Some(idx) = free.pop() {
            slots[idx] = Slot {
                generation: current,
                meta: Some(meta),
                state: SlotState::Active,
            };
            idx
        } else {
            if slots.len() >= self.capacity {
                return Err(BridgeError::Capacity);
            }
            slots.push(Slot {
                generation: current,
                meta: Some(meta),
                state: SlotState::Active,
            });
            slots.len() - 1
        };
        self.counters.live_slots.fetch_add(1, Ordering::Relaxed);
        Ok((slot, current))
    }

    /// Invalidate a handle at settlement/cancellation. Generation changes
    /// before the slot is returned to the free list, so retained handles fail.
    pub fn settle(&self, slot: usize, generation: u64) {
        let mut slots = self.slots.borrow_mut();
        let Some(s) = slots.get_mut(slot) else {
            return;
        };
        if s.generation != generation || s.state != SlotState::Active {
            return;
        }
        s.state = SlotState::Settled;
        s.meta = None;
        s.generation = s.generation.wrapping_add(1).max(1);
        self.counters.live_slots.fetch_sub(1, Ordering::Relaxed);
        self.free.borrow_mut().push(slot);
    }

    /// Generation-checked access. The closure returns owned data before the
    /// local borrow ends, and materialization accounting is scalar only.
    pub fn access<T>(
        &self,
        slot: usize,
        generation: u64,
        cost_fields: u64,
        cost_bytes: u64,
        f: impl FnOnce(&RequestMeta) -> T,
    ) -> Result<T, BridgeError> {
        self.counters.host_calls.fetch_add(1, Ordering::Relaxed);
        let slots = self.slots.borrow();
        let s = slots.get(slot).ok_or(BridgeError::NoSuchSlot)?;
        if s.generation != generation || s.state != SlotState::Active {
            self.counters
                .expired_accesses
                .fetch_add(1, Ordering::Relaxed);
            return Err(BridgeError::Expired);
        }
        let meta = s.meta.as_ref().expect("active slot has metadata");
        let out = f(meta);
        drop(slots);
        self.counters
            .materialized_fields
            .fetch_add(cost_fields, Ordering::Relaxed);
        self.counters
            .materialized_bytes
            .fetch_add(cost_bytes, Ordering::Relaxed);
        Ok(out)
    }

    pub fn live_slots(&self) -> usize {
        self.counters.live_slots.load(Ordering::Relaxed) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta(body: Option<Vec<u8>>) -> RequestMeta {
        RequestMeta {
            method: "GET".into(),
            path: "/x".into(),
            params: vec![],
            query: vec![("a".into(), "1".into())],
            headers: vec![("authorization".into(), "Bearer x".into())],
            content_type: Some("application/json".into()),
            body,
        }
    }

    #[test]
    fn bounded_slab_rejects_growth() {
        let store = RequestStore::with_capacity(1);
        let first = store.try_insert(meta(None)).unwrap();
        assert_eq!(store.try_insert(meta(None)), Err(BridgeError::Capacity));
        store.settle(first.0, first.1);
        assert!(store.try_insert(meta(None)).is_ok());
    }

    #[test]
    fn access_materializes_and_counts() {
        let store = RequestStore::new();
        let (slot, generation) = store.insert(meta(Some(br#"{"a":1}"#.to_vec())));
        let body = store
            .access(slot, generation, 1, 7, |m| m.body.clone())
            .unwrap();
        assert_eq!(body.unwrap(), br#"{"a":1}"#.to_vec());
        let snap = store.snapshot();
        assert_eq!(snap.host_calls, 1);
        assert_eq!(snap.materialized_fields, 1);
        assert_eq!(snap.materialized_bytes, 7);
        assert_eq!(snap.live_slots, 1);
    }

    #[test]
    fn settlement_expires_handle_and_reuse_is_isolated() {
        let store = RequestStore::new();
        let (slot, gen1) = store.insert(meta(None));
        store.settle(slot, gen1);
        assert_eq!(
            store.access(slot, gen1, 0, 0, |_| ()),
            Err(BridgeError::Expired)
        );
        assert_eq!(store.snapshot().expired_accesses, 1);
        assert_eq!(store.live_slots(), 0);
        let (slot2, gen2) = store.insert(meta(None));
        assert_eq!(slot, slot2);
        assert_ne!(gen1, gen2);
        assert_eq!(
            store.access(slot, gen1, 0, 0, |_| ()),
            Err(BridgeError::Expired)
        );
        assert_eq!(
            store.access(slot, gen2 + 100, 0, 0, |_| ()),
            Err(BridgeError::Expired)
        );
    }

    #[test]
    fn stale_handle_corpus_never_reads_or_leaks() {
        let store = RequestStore::with_capacity(2);
        let (slot, generation) = store.insert(meta(Some(vec![7; 32])));
        store.settle(slot, generation);
        let before = store.snapshot();
        for stale_generation in [0, 1, generation, generation.wrapping_add(1), u64::MAX] {
            for stale_slot in [slot, slot + 1, usize::MAX] {
                let result = store.access(stale_slot, stale_generation, 1, 32, |m| m.body.clone());
                assert!(matches!(
                    result,
                    Err(BridgeError::Expired | BridgeError::NoSuchSlot)
                ));
            }
        }
        let after = store.snapshot();
        assert_eq!(after.live_slots, 0);
        assert_eq!(after.materialized_fields, before.materialized_fields);
        assert_eq!(after.materialized_bytes, before.materialized_bytes);
    }

    #[test]
    fn unread_request_costs_nothing() {
        let store = RequestStore::new();
        let (slot, generation) = store.insert(meta(Some(vec![42u8; 1024])));
        store.settle(slot, generation);
        let snap = store.snapshot();
        assert_eq!(snap.host_calls, 0);
        assert_eq!(snap.materialized_fields, 0);
        assert_eq!(snap.materialized_bytes, 0);
    }

    #[test]
    fn double_settle_is_idempotent() {
        let store = RequestStore::new();
        let (slot, generation) = store.insert(meta(None));
        store.settle(slot, generation);
        store.settle(slot, generation);
        assert_eq!(store.live_slots(), 0);
    }
}
