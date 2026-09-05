fn main() {
    let pack = q_pack::minimal_pack_public();
    let bytes = serde_json::to_vec(&pack).unwrap();
    std::fs::write(std::env::args().nth(1).unwrap(), bytes).unwrap();
}
