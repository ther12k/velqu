//! #1318 / M6-002 — QPack verifier fuzz target.
//!
//! Arbitrary bytes at the pack-verification trust boundary: every input
//! must produce `Ok` (a fully verified pack) or a typed `Err` — never a
//! panic, hang, or unbounded allocation. The verifier is the first code
//! a deployed artifact touches; this is its no-crash contract.

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Byte-level v2 section parsing (header + directory) must be total.
    let _ = q_pack::reject_mixed_mode_bytes(data);
    let _ = q_pack::detect_pack_format_mode(fuzz_to_u32(data));

    // The in-memory verify path (portable, no filesystem): a valid JSON
    // pack prefix that fails verification must yield Malformed/typed
    // errors; malformed bytes must yield Malformed — not a panic.
    let _ = q_pack::QPack::verify_from_slice(data, q_pack::BytecodePolicy::Enforce);
    let _ = q_pack::QPack::verify_from_slice(data, q_pack::BytecodePolicy::Skip);
});

fn fuzz_to_u32(data: &[u8]) -> u32 {
    let mut b = [0u8; 4];
    for (i, v) in data.iter().take(4).enumerate() {
        b[i] = *v;
    }
    u32::from_le_bytes(b)
}
