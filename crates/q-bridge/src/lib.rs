//! q-bridge — native request storage backing lazy, generation-checked JS handles.
//!
//! A RequestStore is deliberately not thread-safe: the production instance is
//! created inside one QuickJS worker and accessed through that worker's
//! `Rc<RefCell<_>>`. The public counters remain atomic so a read-only snapshot
//! can be observed by the host without sharing request bytes or slab state.

use std::cell::RefCell;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
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
    query_cache: RefCell<Option<String>>,
    cookie_cache: RefCell<Option<String>>,
    body_mode: RefCell<Option<&'static str>>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BridgeError {
    #[error("request handle expired (slot settled or cancelled): access denied")]
    Expired,
    #[error("request handle belongs to another worker: access denied")]
    ForeignWorker,
    #[error("request handle does not exist")]
    NoSuchSlot,
    #[error("request slab capacity reached")]
    Capacity,
    #[error("incompatible second body representation read")]
    IncompatibleBodyRead,
}

/// Monotonic worker-identity clock: every slab (one per QuickJS worker) gets a
/// distinct id, so a handle minted by worker A can never be confused with the
/// same (slot, generation) pair in worker B's slab.
static WORKER_ID_CLOCK: AtomicU32 = AtomicU32::new(1);

/// Opaque, typed request capability. Fields are private: a handle is minted
/// only by the owning slab (`try_insert`) or reconstructed for this worker's
/// numeric JS pair (`local_handle`). Worker identity is validated before any
/// slot lookup; the JS ABI only ever carries the (slot, generation) numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestHandle {
    worker_id: u32,
    slot: usize,
    generation: u64,
}

impl RequestHandle {
    #[inline]
    pub fn worker_id(&self) -> u32 {
        self.worker_id
    }

    #[inline]
    pub fn slot(&self) -> usize {
        self.slot
    }

    #[inline]
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// The numeric pair JavaScript already exchanges (prelude ABI).
    #[inline]
    pub fn js_pair(&self) -> (usize, u64) {
        (self.slot, self.generation)
    }
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
    worker_id: u32,
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
            worker_id: WORKER_ID_CLOCK.fetch_add(1, Ordering::Relaxed),
            slots: RefCell::new(Vec::with_capacity(capacity.min(1024))),
            free: RefCell::new(Vec::new()),
            counters,
            generation_clock: RefCell::new(1),
            capacity,
        }
    }

    pub fn worker_id(&self) -> u32 {
        self.worker_id
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

    /// Insert a request, returning the typed capability. This helper is
    /// retained for local tests and compatibility; worker admission uses the
    /// fallible form so a full slab cannot grow or panic.
    pub fn insert(&self, meta: RequestMeta) -> RequestHandle {
        self.try_insert(meta)
            .expect("request slab capacity reached")
    }

    pub fn try_insert(&self, meta: RequestMeta) -> Result<RequestHandle, BridgeError> {
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
                query_cache: RefCell::new(None),
                cookie_cache: RefCell::new(None),
                body_mode: RefCell::new(None),
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
                query_cache: RefCell::new(None),
                cookie_cache: RefCell::new(None),
                body_mode: RefCell::new(None),
            });
            slots.len() - 1
        };
        self.counters.live_slots.fetch_add(1, Ordering::Relaxed);
        Ok(RequestHandle {
            worker_id: self.worker_id,
            slot,
            generation: current,
        })
    }

    /// Reconstruct this worker's handle from the numeric pair JavaScript
    /// carries. The pair came from this slab originally; slot/generation
    /// validity is still enforced by `access`/`settle`.
    pub fn local_handle(&self, slot: usize, generation: u64) -> RequestHandle {
        RequestHandle {
            worker_id: self.worker_id,
            slot,
            generation,
        }
    }

    /// Invalidate a handle at settlement/cancellation. Generation changes
    /// before the slot is returned to the free list, so retained handles fail.
    /// A handle minted by another worker's slab is denied before slot lookup.
    pub fn settle(&self, handle: RequestHandle) {
        if handle.worker_id != self.worker_id {
            return;
        }
        let mut slots = self.slots.borrow_mut();
        let Some(s) = slots.get_mut(handle.slot) else {
            return;
        };
        if s.generation != handle.generation || s.state != SlotState::Active {
            return;
        }
        s.state = SlotState::Settled;
        s.meta = None;
        *s.query_cache.get_mut() = None;
        *s.cookie_cache.get_mut() = None;
        *s.body_mode.get_mut() = None;
        s.generation = s.generation.wrapping_add(1).max(1);
        self.counters.live_slots.fetch_sub(1, Ordering::Relaxed);
        self.free.borrow_mut().push(handle.slot);
    }

    /// Worker-owned terminal sweep for quarantine/shutdown: settles every
    /// Active slot in one bounded pass (the slab is capacity-bounded), so no
    /// slot can survive its worker even if no PendingInvocation tracks it.
    /// Already-settled slots are skipped, so overlap with per-handle settles
    /// is idempotent. Returns the number of slots this pass settled.
    pub fn settle_all(&self) -> usize {
        let mut slots = self.slots.borrow_mut();
        let mut free = self.free.borrow_mut();
        let mut settled = 0usize;
        for (idx, s) in slots.iter_mut().enumerate() {
            if s.state != SlotState::Active {
                continue;
            }
            s.state = SlotState::Settled;
            s.meta = None;
            s.generation = s.generation.wrapping_add(1).max(1);
            self.counters.live_slots.fetch_sub(1, Ordering::Relaxed);
            free.push(idx);
            settled += 1;
        }
        settled
    }

    /// Worker- and generation-checked access. The closure returns owned data
    /// before the local borrow ends, and materialization accounting is scalar
    /// only. A foreign-worker handle is denied before any slot is touched.
    pub fn access<T>(
        &self,
        handle: RequestHandle,
        cost_fields: u64,
        cost_bytes: u64,
        f: impl FnOnce(&RequestMeta) -> T,
    ) -> Result<T, BridgeError> {
        self.counters.host_calls.fetch_add(1, Ordering::Relaxed);
        if handle.worker_id != self.worker_id {
            // M24-003-D: cross-worker handles get a dedicated deterministic
            // denial, decided before any slot of this slab is inspected.
            self.counters
                .expired_accesses
                .fetch_add(1, Ordering::Relaxed);
            return Err(BridgeError::ForeignWorker);
        }
        let slots = self.slots.borrow();
        let s = slots.get(handle.slot).ok_or(BridgeError::NoSuchSlot)?;
        if s.generation != handle.generation || s.state != SlotState::Active {
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

    /// Return cached query JSON, computing it once while slot remains active.
    /// Cache is slot-local and cleared before generation reuse.
    pub fn cached_query(
        &self,
        handle: RequestHandle,
        build: impl FnOnce(&RequestMeta) -> String,
    ) -> Result<String, BridgeError> {
        self.counters.host_calls.fetch_add(1, Ordering::Relaxed);
        if handle.worker_id != self.worker_id {
            self.counters
                .expired_accesses
                .fetch_add(1, Ordering::Relaxed);
            return Err(BridgeError::ForeignWorker);
        }
        let slots = self.slots.borrow();
        let slot = match slots.get(handle.slot) {
            Some(slot) => slot,
            None => return Err(BridgeError::NoSuchSlot),
        };
        if slot.generation != handle.generation || slot.state != SlotState::Active {
            self.counters
                .expired_accesses
                .fetch_add(1, Ordering::Relaxed);
            return Err(BridgeError::Expired);
        }
        if let Some(value) = slot.query_cache.borrow().as_ref() {
            return Ok(value.clone());
        }
        let value = build(slot.meta.as_ref().expect("active slot has metadata"));
        *slot.query_cache.borrow_mut() = Some(value.clone());
        self.counters
            .materialized_fields
            .fetch_add(1, Ordering::Relaxed);
        self.counters
            .materialized_bytes
            .fetch_add(value.len() as u64, Ordering::Relaxed);
        Ok(value)
    }

    #[cfg(test)]
    pub fn query_cache_value(&self, handle: RequestHandle) -> Option<String> {
        let slots = self.slots.borrow();
        slots
            .get(handle.slot)
            .and_then(|slot| slot.query_cache.borrow().clone())
    }

    /// Permit one decoded body representation per request slot. Reusing same
    /// representation is allowed; switching representation fails closed.
    pub fn body_mode(&self, handle: RequestHandle, mode: &'static str) -> Result<(), BridgeError> {
        if handle.worker_id != self.worker_id {
            return Err(BridgeError::ForeignWorker);
        }
        let slots = self.slots.borrow();
        let slot = slots.get(handle.slot).ok_or(BridgeError::NoSuchSlot)?;
        if slot.generation != handle.generation || slot.state != SlotState::Active {
            return Err(BridgeError::Expired);
        }
        let mut current = slot.body_mode.borrow_mut();
        if let Some(existing) = *current {
            if existing != mode {
                return Err(BridgeError::IncompatibleBodyRead);
            }
        } else {
            *current = Some(mode);
        }
        Ok(())
    }

    pub fn live_slots(&self) -> usize {
        self.counters.live_slots.load(Ordering::Relaxed) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta(body: Option<bytes::Bytes>) -> RequestMeta {
        RequestMeta {
            method: "GET".into(),
            path: "/x".into(),
            param_specs: vec![],
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
        store.settle(first);
        assert!(store.try_insert(meta(None)).is_ok());
    }

    #[test]
    fn settle_all_is_bounded_and_idempotent_with_handle_settles() {
        let store = RequestStore::with_capacity(4);
        let a = store.insert(meta(None));
        let b = store.insert(meta(None));
        let c = store.insert(meta(None));
        assert_eq!(store.live_slots(), 3);
        // one slot settles through its own handle first (the single owner path)
        store.settle(a);
        // the terminal sweep settles the remaining Active slots exactly once
        assert_eq!(store.settle_all(), 2);
        assert_eq!(store.live_slots(), 0);
        // a repeated sweep — and late per-handle settles — are checked no-ops
        assert_eq!(store.settle_all(), 0);
        store.settle(b);
        store.settle(c);
        assert_eq!(store.live_slots(), 0, "no double decrement of live_slots");
        // settled slots are reusable with fresh generations
        let d = store.insert(meta(None));
        assert_ne!(d.generation(), b.generation());
        assert_eq!(store.settle_all(), 1);
        assert_eq!(store.live_slots(), 0);
    }

    #[test]
    fn access_materializes_and_counts() {
        let store = RequestStore::new();
        let handle = store.insert(meta(Some(bytes::Bytes::from_static(br#"{"a":1}"#))));
        let body = store.access(handle, 1, 7, |m| m.body.clone()).unwrap();
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
        let handle = store.insert(meta(None));
        store.settle(handle);
        assert_eq!(
            store.access(handle, 0, 0, |_| ()),
            Err(BridgeError::Expired)
        );
        assert_eq!(store.snapshot().expired_accesses, 1);
        assert_eq!(store.live_slots(), 0);
        let handle2 = store.insert(meta(None));
        assert_eq!(handle.slot(), handle2.slot());
        assert_ne!(handle.generation(), handle2.generation());
        assert_eq!(
            store.access(handle, 0, 0, |_| ()),
            Err(BridgeError::Expired)
        );
        // wrong-owner (stale generation on live slot) denied
        let stale = store.local_handle(handle2.slot(), handle2.generation() + 100);
        assert_eq!(store.access(stale, 0, 0, |_| ()), Err(BridgeError::Expired));
    }

    #[test]
    fn typed_handle_from_foreign_worker_is_denied_before_slot_lookup() {
        let store_a = RequestStore::new();
        let store_b = RequestStore::new();
        assert_ne!(store_a.worker_id(), store_b.worker_id());
        let handle_a = store_a.insert(meta(Some(bytes::Bytes::from(vec![1, 2, 3]))));
        // worker A's minted handle presented to worker B's slab: the dedicated
        // deterministic ForeignWorker denial fires before any slot of store_b
        // is inspected — even when store_b has a live slot at the same index
        let decoy = store_b.insert(meta(Some(bytes::Bytes::from(vec![9; 8]))));
        assert_eq!(decoy.slot(), handle_a.slot());
        assert_eq!(
            store_b.access(handle_a, 1, 3, |m| m.body.clone()),
            Err(BridgeError::ForeignWorker)
        );
        // the decoy slot was neither read nor settled by the denial
        assert_eq!(store_b.snapshot().materialized_fields, 0);
        assert_eq!(store_b.live_slots(), 1);
        // settle from the wrong worker must not touch either slab
        store_b.settle(handle_a);
        assert_eq!(store_b.live_slots(), 1);
        assert_eq!(store_a.live_slots(), 1);
        // the true owner still works and settles exactly once
        assert!(store_a.access(handle_a, 0, 0, |_| ()).is_ok());
        store_a.settle(handle_a);
        store_b.settle(decoy);
        assert_eq!(store_a.live_slots(), 0);
        assert_eq!(store_b.live_slots(), 0);
    }

    /// M24-003-D: bounded fuzz corpus over arbitrary (worker, slot,
    /// generation) triples across two slabs. No forged operation may read
    /// request bytes, settle a live slot, or perturb the valid handles'
    /// accounting.
    #[test]
    fn fuzzed_handle_triples_fail_closed_without_side_effects() {
        let store = RequestStore::with_capacity(2);
        let foreign = RequestStore::with_capacity(2);
        let live = store.insert(meta(Some(bytes::Bytes::from(vec![7; 64]))));
        let baseline = store.snapshot();
        let foreign_baseline = foreign.snapshot();

        // deterministic LCG: bounded corpus, no external rand dependency
        let mut state: u64 = 0x9E3779B97F4A7C15;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for _ in 0..2048 {
            let worker_id = (next() % 3) as u32; // 0/1 → almost surely foreign; 2 → maybe local
            let slot = (next() % 4) as usize; // spans live slot, settled, and holes
            let generation = next();
            let forged = RequestHandle {
                worker_id,
                slot,
                generation,
            };
            let outcome = store.access(forged, 1, 64, |m| m.body.clone());
            // a forged triple is only valid if it happens to be EXACTLY the
            // live handle; everything else fails deterministically
            if forged != live {
                assert!(
                    matches!(
                        outcome,
                        Err(BridgeError::ForeignWorker
                            | BridgeError::Expired
                            | BridgeError::NoSuchSlot)
                    ),
                    "forged triple {forged:?} must fail closed"
                );
            } else {
                assert!(outcome.is_ok(), "the exact live handle must still work");
            }
            // forged settles never free another worker's or generation's slot
            store.settle(forged);
            assert_eq!(
                store.live_slots(),
                1,
                "live slot must survive forged settles"
            );
            let _ = foreign.access(forged, 0, 0, |_| ());
            foreign.settle(forged);
        }

        let after = store.snapshot();
        assert_eq!(after.live_slots, 1);
        // only the exact-live-handle hits may have materialized anything
        assert_eq!(after.materialized_bytes % 64, 0);
        assert_eq!(
            foreign.snapshot().materialized_fields,
            foreign_baseline.materialized_fields
        );
        assert_eq!(foreign.snapshot().live_slots, 0);
        // the honest handle still reads and settles exactly once
        let body = store.access(live, 1, 64, |m| m.body.clone()).unwrap();
        assert_eq!(body.unwrap(), vec![7u8; 64]);
        store.settle(live);
        assert_eq!(store.live_slots(), 0);
        let _ = baseline;
    }

    #[test]
    fn stale_handle_corpus_never_reads_or_leaks() {
        let store = RequestStore::with_capacity(2);
        let handle = store.insert(meta(Some(bytes::Bytes::from(vec![7; 32]))));
        store.settle(handle);
        let before = store.snapshot();
        for stale_generation in [
            0,
            1,
            handle.generation(),
            handle.generation().wrapping_add(1),
            u64::MAX,
        ] {
            for stale_slot in [handle.slot(), handle.slot() + 1, usize::MAX] {
                let forged = store.local_handle(stale_slot, stale_generation);
                let result = store.access(forged, 1, 32, |m| m.body.clone());
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
    fn query_cache_materializes_once_and_expires_with_slot() {
        let store = RequestStore::with_capacity(1);
        let handle = store.insert(meta(None));
        let first = store.cached_query(handle, |m| m.path.to_string()).unwrap();
        let second = store.cached_query(handle, |_| "wrong".into()).unwrap();
        assert_eq!(first, second);
        assert_eq!(store.snapshot().materialized_fields, 1);
        store.settle(handle);
        assert!(store.query_cache_value(handle).is_none());
        assert!(matches!(
            store.cached_query(handle, |_| "stale".into()),
            Err(BridgeError::Expired)
        ));
    }

    #[test]
    fn unread_request_costs_nothing() {
        let store = RequestStore::new();
        let handle = store.insert(meta(Some(bytes::Bytes::from(vec![42u8; 1024]))));
        store.settle(handle);
        let snap = store.snapshot();
        assert_eq!(snap.host_calls, 0);
        assert_eq!(snap.materialized_fields, 0);
        assert_eq!(snap.materialized_bytes, 0);
    }

    #[test]
    fn double_settle_is_idempotent() {
        let store = RequestStore::new();
        let handle = store.insert(meta(None));
        store.settle(handle);
        store.settle(handle);
        assert_eq!(store.live_slots(), 0);
    }
}
