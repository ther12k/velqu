//! q-bridge — native request store backing lazy, generation-checked JS handles.
//!
//! RUN-004/SEC-003: request data materializes into JavaScript only on explicit
//! access through `(slot, generation)` pairs. Settlement (or cancellation)
//! invalidates the slot by bumping its generation; late access with an expired
//! generation fails deterministically instead of touching reused memory.
//! Counters expose laziness evidence (unread fields = 0 materializations).

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

#[derive(Debug, Clone, Default)]
pub struct RequestMeta {
    pub method: String,
    pub path: String,
    /// raw path params as extracted by the router (strings)
    pub params: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub content_type: Option<String>,
    /// Present only when the transport already read a (bounded) body.
    pub body: Option<Vec<u8>>,
}

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

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum BridgeError {
    #[error("request handle expired (slot settled or cancelled): access denied")]
    Expired,
    #[error("request handle does not exist")]
    NoSuchSlot,
}

/// Process-wide laziness/bridge evidence (RUN-004, PERF-004).
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

pub struct RequestStore {
    slots: Mutex<Vec<Slot>>,
    free: Mutex<Vec<usize>>,
    counters: BridgeCounters,
    generation_clock: AtomicU64,
}

impl Default for RequestStore {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestStore {
    pub fn new() -> Self {
        RequestStore {
            slots: Mutex::new(Vec::new()),
            free: Mutex::new(Vec::new()),
            counters: BridgeCounters::default(),
            generation_clock: AtomicU64::new(1),
        }
    }

    pub fn counters(&self) -> &BridgeCounters {
        &self.counters
    }

    pub fn snapshot(&self) -> CountersSnapshot {
        self.counters.snapshot()
    }

    /// Insert a request; returns (slot, generation) — the opaque handle pair.
    pub fn insert(&self, meta: RequestMeta) -> (usize, u64) {
        let generation = self.generation_clock.fetch_add(1, Ordering::SeqCst);
        let mut slots = self.slots.lock().unwrap();
        let mut free = self.free.lock().unwrap();
        let slot = if let Some(idx) = free.pop() {
            slots[idx] = Slot {
                generation,
                meta: Some(meta),
                state: SlotState::Active,
            };
            idx
        } else {
            slots.push(Slot {
                generation,
                meta: Some(meta),
                state: SlotState::Active,
            });
            slots.len() - 1
        };
        drop(slots);
        drop(free);
        self.counters.live_slots.fetch_add(1, Ordering::Relaxed);
        (slot, generation)
    }

    /// Invalidate a handle at settlement/cancellation. Bumping the generation
    /// means any late completion or retained wrapper fails `access` checks.
    pub fn settle(&self, slot: usize, generation: u64) {
        let mut slots = self.slots.lock().unwrap();
        if let Some(s) = slots.get_mut(slot) {
            if s.generation == generation {
                s.state = SlotState::Settled;
                s.meta = None;
                s.generation += 1; // expire outstanding handles
                self.counters.live_slots.fetch_sub(1, Ordering::Relaxed);
                let mut free = self.free.lock().unwrap();
                free.push(slot);
            }
        }
    }

    /// Generation-checked access. `cost_bytes`/`cost_fields` record the
    /// materialization that the caller is about to perform.
    pub fn access<T>(
        &self,
        slot: usize,
        generation: u64,
        cost_fields: u64,
        cost_bytes: u64,
        f: impl FnOnce(&RequestMeta) -> T,
    ) -> Result<T, BridgeError> {
        self.counters.host_calls.fetch_add(1, Ordering::Relaxed);
        let slots = self.slots.lock().unwrap();
        let s = slots.get(slot).ok_or(BridgeError::NoSuchSlot)?;
        if s.generation != generation || s.state != SlotState::Active {
            self.counters
                .expired_accesses
                .fetch_add(1, Ordering::Relaxed);
            return Err(BridgeError::Expired);
        }
        let meta = s.meta.as_ref().expect("active slot has meta");
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

    /// Live slot count (for leak assertions: 0 live slots after settle).
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
    fn access_materializes_and_counts() {
        let store = RequestStore::new();
        let (slot, gen) = store.insert(meta(Some(br#"{"a":1}"#.to_vec())));
        let body = store.access(slot, gen, 1, 7, |m| m.body.clone()).unwrap();
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
        // retained wrapper fails deterministically
        assert_eq!(
            store.access(slot, gen1, 0, 0, |_| ()),
            Err(BridgeError::Expired)
        );
        assert_eq!(store.snapshot().expired_accesses, 1);
        assert_eq!(store.live_slots(), 0);
        // slot reuse gets a new generation; old handle still denied
        let (_slot2, gen2) = store.insert(meta(None));
        assert_ne!(gen1, gen2);
        assert_eq!(
            store.access(slot, gen1, 0, 0, |_| ()),
            Err(BridgeError::Expired)
        );
        // wrong-owner (stale generation on live slot) denied
        assert_eq!(
            store.access(slot, gen2 + 100, 0, 0, |_| ()),
            Err(BridgeError::Expired)
        );
    }

    #[test]
    fn unread_request_costs_nothing() {
        let store = RequestStore::new();
        let (slot, gen) = store.insert(meta(Some(vec![42u8; 1024])));
        // handler never accesses anything: settle immediately
        store.settle(slot, gen);
        let snap = store.snapshot();
        assert_eq!(snap.host_calls, 0);
        assert_eq!(snap.materialized_fields, 0);
        assert_eq!(snap.materialized_bytes, 0);
    }

    #[test]
    fn double_settle_is_idempotent() {
        let store = RequestStore::new();
        let (slot, gen) = store.insert(meta(None));
        store.settle(slot, gen);
        store.settle(slot, gen); // stale settle ignored
        assert_eq!(store.live_slots(), 0);
    }
}
