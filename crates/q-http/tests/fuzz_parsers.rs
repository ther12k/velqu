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
fn invalid_percent_and_utf8_corpus_is_deterministic() {
    assert_eq!(q_http::percent_decode("%"), "%");
    assert_eq!(q_http::percent_decode("%G0"), "%G0");
    assert_eq!(q_http::percent_decode("%0"), "%0");
    assert_eq!(q_http::percent_decode("%FF"), "\u{fffd}");
    assert_eq!(q_http::percent_decode("%C3%28"), "\u{fffd}(");
    assert_eq!(q_http::percent_decode("a+b"), "a b");
}

#[test]
fn differential_query_decode_matches_reference_for_safe_form_inputs() {
    for raw in ["a=1&b=2", "name=Rafi+Z&x=%41", "empty=", "a=1&a=2"] {
        let ours = q_http::parse_query(raw);
        let reference: Vec<(String, String)> = raw
            .split('&')
            .filter(|pair| !pair.is_empty())
            .map(|pair| {
                let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
                (reference_decode(key), reference_decode(value))
            })
            .collect();
        assert_eq!(ours, reference, "reference mismatch for {raw}");
    }
}

fn reference_decode(value: &str) -> String {
    let mut bytes = Vec::with_capacity(value.len());
    let raw = value.as_bytes();
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'+' {
            bytes.push(b' ');
            i += 1;
        } else if i + 2 < raw.len() && raw[i] == b'%' {
            let hi = (raw[i + 1] as char).to_digit(16);
            let lo = (raw[i + 2] as char).to_digit(16);
            if let (Some(hi), Some(lo)) = (hi, lo) {
                bytes.push((hi * 16 + lo) as u8);
                i += 3;
            } else {
                bytes.push(raw[i]);
                i += 1;
            }
        } else {
            bytes.push(raw[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

#[test]
fn bounded_header_and_body_corpus_never_grows_without_limit() {
    let corpus: &[&[u8]] = &[
        b"",
        b"%FF",
        b"\0\xff\n",
        b"{}",
        b"{\"x\":\"y\"}",
        &[0xff; 256],
    ];
    for bytes in corpus {
        let text = String::from_utf8_lossy(bytes);
        let parsed = q_http::parse_query(&text);
        assert!(parsed.len() <= text.len().saturating_add(1));
        assert!(q_http::percent_decode(&text).len() <= text.len() * 4 + 16);
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
