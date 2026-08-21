//! Minimized malformed-input regression corpus replay.

#[test]
fn minimized_ingress_inputs_remain_total() {
    const CORPUS: &[&str] = &["%", "%G0", "%FF", "%C3%28", "a+b", "&&", "=", "\0"];
    for input in CORPUS {
        let pairs = q_http::parse_query(input);
        let decoded = q_http::percent_decode(input);
        assert!(pairs.len() <= input.len() + 1);
        assert!(decoded.len() <= input.len() * 4 + 16);
    }
}
