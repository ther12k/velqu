//! Regenerates the committed golden v1 compatibility fixture
//! (`tests/fixtures/v1/minimal.json`) from the current in-repo model.
//!
//! Run from repo root:
//!   cargo run -p q-pack --example gen-fixture > crates/q-pack/tests/fixtures/v1/minimal.json
//!
//! After regenerating, run `cargo test -p q-pack` — the legacy_v1 adapter
//! tests pin that the committed fixture keeps verifying.

fn main() {
    let mut pack = q_pack::minimal_pack_public();
    pack.app_id = "fixture".into();
    print!("{}", serde_json::to_string_pretty(&pack).unwrap());
}
