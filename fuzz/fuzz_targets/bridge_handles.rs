//! #1318 / M6-002 — Bridge handle-lifecycle fuzz target (q-bridge).
//!
//! The bridge slab's generation-checked handles are the memory-safety
//! boundary between host and QuickJS. This target drives an op stream
//! derived from arbitrary bytes — insert, stale/foreign access, settle,
//! re-insert — against a small bounded slab. Every op must produce a
//! typed outcome; stale/foreign handles must NEVER grant access, and
//! live slots must stay bounded.

#![no_main]

use libfuzzer_sys::fuzz_target;
use q_bridge::{RequestStore,};
use q_runtime_model::RequestMeta;
use std::cell::RefCell;

thread_local! {
    static STORE: RefCell<Option<RequestStore>> = const { RefCell::new(None) };
}

fn meta_for(seq: u64, data: &[u8]) -> RequestMeta {
    let tag = if data.is_empty() { 0 } else { data[seq as usize % data.len()] };
    RequestMeta {
        method: if tag & 1 == 0 { "GET".into() } else { "POST".into() },
        path: format!("/fuzz/{}", tag % 7),
        param_specs: vec![],
        query: vec![("k".into(), format!("v{}", tag % 5))],
        headers: vec![("x-fuzz".into(), format!("{}", tag))],
        content_type: Some("application/json".into()),
        body: None,
    }
}

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }
    STORE.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_none() {
            *slot = Some(RequestStore::with_capacity(8));
        }
        let store = slot.as_ref().unwrap();

        let mut live: Vec<(usize, u64)> = Vec::new();
        // Deterministic op stream: 2 bytes per op.
        for ops in data.chunks_exact(2) {
            let op = ops[0] % 5;
            let arg = ops[1] as usize;
            match op {
                0 | 1 => {
                    // insert (bounded slab may reject when full: typed)
                    if let Ok(h) = store.try_insert(meta_for(arg as u64, data)) {
                        live.push((h.slot(), h.generation()));
                    }
                }
                2 => {
                    // settle one live handle, if any
                    if let Some((slot, gen)) = arg.checked_rem(live.len().max(1)).and_then(|i| live.get(i)).copied() {
                        let h = store.local_handle(slot, gen);
                        store.settle(h);
                        live.retain(|(s, g)| !(*s == slot && *g == gen));
                    }
                }
                3 => {
                    // STALE/FOREIGN access: random slot/generation pair.
                    // Must be refused (typed error), never grant bytes.
                    let foreign_slot = arg % 8;
                    let foreign_gen = ops[1] as u64 + 10_000;
                    let h = store.local_handle(foreign_slot, foreign_gen);
                    let granted = store.access::<()>(h, 0, 0, |_meta| ()).is_ok();
                    let was_live = live.contains(&(foreign_slot, foreign_gen));
                    assert!(
                        granted == was_live,
                        "access grant ({granted}) does not match liveness ({was_live}) for slot {foreign_slot} gen {foreign_gen}"
                    );
                }
                _ => {
                    // counters snapshot must always be coherent
                    let snap = store.snapshot();
                    assert!(snap.live_slots as usize == store.live_slots());
                }
            }
            // Bounded-slab invariant: capacity 8 => at most 8 live slots.
            assert!(
                store.live_slots() <= 8,
                "live slots {} exceeded capacity",
                store.live_slots()
            );
        }
    });
});
