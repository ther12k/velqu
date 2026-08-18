//! Property-based robustness tests for the query/percent-decode parser and
//! header accounting (externally controlled inputs — engineering standard).

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
    fn asciiish(&mut self, n: usize) -> String {
        // biased toward URI-relevant characters incl. %, =, &, +, hex
        const CHARSET: &[u8] = b"%=&+abcxyz019?#/ \t\x00\xff";
        (0..n)
            .map(|_| CHARSET[(self.next() as usize) % CHARSET.len()] as char)
            .collect()
    }
}

#[test]
fn query_parser_never_panics_on_arbitrary_input() {
    let mut rng = Rng(0xdeadbeefcafebabe);
    for _ in 0..20_000 {
        let n = (rng.next() % 64) as usize;
        let s = rng.asciiish(n);
        let _ = q_http::parse_query(&s);
    }
}

#[test]
fn percent_decode_never_panics_and_always_returns_utf8() {
    let mut rng = Rng(0x0123456789abcdef);
    for _ in 0..20_000 {
        let n = (rng.next() % 32) as usize;
        let s = rng.asciiish(n);
        let out = q_http::percent_decode(&s);
        // must be valid UTF-8 by construction (from_utf8_lossy) and finite
        assert!(out.len() < s.len() * 4 + 16);
    }
}

#[test]
fn query_parser_semantics_hold_on_valid_pairs() {
    // invariant: round-trip of simple keys survives parsing
    let pairs = q_http::parse_query("a=1&b=2&c=3");
    assert_eq!(pairs.len(), 3);
    assert_eq!(pairs[0], ("a".into(), "1".into()));
    // malformed percent escapes decode literally (documented behavior)
    assert_eq!(q_http::percent_decode("100%"), "100%");
    assert_eq!(q_http::percent_decode("%41%42"), "AB");
    assert_eq!(q_http::percent_decode("a+b"), "a b");
}
