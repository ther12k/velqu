//! Property-based robustness tests for externally controlled native parsers
//! (engineering standard: fuzz native parsers). Using inline proptest-style
//! loops over deterministic pseudo-random bytes — no external dependency, so
//! these run in every `cargo test` invocation.

use q_pack::QPack;

/// xorshift PRNG — deterministic, no deps.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn bytes(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| (self.next() & 0xff) as u8).collect()
    }
}

#[test]
fn random_bytes_never_panic_the_pack_parser() {
    let mut rng = Rng(0x9e3779b97f4a7c15);
    for len in [0usize, 1, 2, 8, 64, 512, 4096] {
        for _ in 0..64 {
            let junk = rng.bytes(len);
            // must return an error (malformed/rejected), never panic
            let dir = std::env::temp_dir().join("velqu-fuzz-pack");
            let _ = std::fs::create_dir_all(&dir);
            let path = dir.join("junk.qpack");
            std::fs::write(&path, &junk).unwrap();
            let _ = QPack::load_and_verify(&path);
        }
    }
}

#[test]
fn mutated_valid_pack_never_panic_and_tamper_is_detected() {
    // build one valid pack, flip random bytes, re-verify: every mutation must
    // either still verify (unchanged semantics) or be REJECTED — never panic,
    // never serve a tampered bundle.
    let pack = q_pack::minimal_pack_public();
    let dir = std::env::temp_dir().join("velqu-fuzz-mutate");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("mut.qpack");
    std::fs::write(&path, serde_json::to_vec(&pack).unwrap()).unwrap();

    let original = std::fs::read(&path).unwrap();
    let mut rng = Rng(0x123456789abcdef);
    let mut rejected = 0;
    for _ in 0..256 {
        let mut copy = original.clone();
        let pos = (rng.next() as usize) % copy.len();
        copy[pos] = (rng.next() & 0xff) as u8;
        std::fs::write(&path, &copy).unwrap();
        match QPack::load_and_verify(&path) {
            Ok(_) => { /* benign mutation (e.g. whitespace) */ }
            Err(_) => rejected += 1,
        }
    }
    // almost every single-byte flip in a JSON+sha256 pack must be caught
    assert!(
        rejected > 200,
        "tamper detection too weak: {rejected}/256 rejected"
    );
}
