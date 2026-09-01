use std::path::PathBuf;

use velqu_runtime::source_map::{mapper_for, mapper_for_sidecar, SourcemapMapper};

fn pack_without_map() -> q_pack::QPack {
    q_pack::QPack {
        format_version: 1,
        kind: "velqu.qpack".into(),
        runtime_abi: 1,
        engine: q_pack::EngineRef {
            name: "quickjs-ng".into(),
            version: "0.15.1".into(),
            binding: "rquickjs".into(),
            rquickjs: "0.6".into(),
            build_hash: "hash".into(),
        },
        schema_ir_version: 1,
        contract_version: 1,
        contract_hash: "hash".into(),
        built_by: q_pack::BuiltBy::default(),
        app_id: "test".into(),
        modules: vec![],
        entry: "app".into(),
        bundle_form: None,
        bundle: "".into(),
        source_map: None,
        bundle_bytecode: None,
        execution_mode: None,
        bundle_prelude: None,
        decoded_bytecode: None,
        routes: vec![],
        schemas: Default::default(),
        policies: Default::default(),
        capabilities: vec![],
        capability_hash: String::new(),
        capability_inventory: None,
        capability_inventory_sha256: None,
        functions: vec![],
        schema_manifest: vec![],
        header_name_table: vec![],
        query_name_table: vec![],
        cookie_name_table: vec![],
        policy_manifest: vec![],
        router: None,
        handler_table: Default::default(),
        integrity: q_pack::Integrity {
            algorithm: "sha256".into(),
            bundle_sha256: String::new(),
            routes_sha256: String::new(),
            bytecode_sha256: None,
        },
    }
}

#[test]
fn valid_sidecar_is_loaded_only_for_symbolization_and_bound_to_pack() {
    let pack_bytes = b"verified-pack";
    let hash = q_pack::sources_sidecar::SourcesSidecar::pack_sha256_of(pack_bytes);
    let path = std::env::temp_dir().join(format!("velqu-source-map-{}.json", std::process::id()));
    let text = format!(
        r#"{{"formatVersion":1,"packSha256":"{hash}","sourceMap":"{{\"version\":3,\"sources\":[\"app.ts\"],\"names\":[],\"mappings\":\"AAAA\"}}"}}"#
    );
    std::fs::write(&path, text).unwrap();
    let mapper = mapper_for_sidecar(&path, pack_bytes).expect("bound sidecar");
    assert!(mapper.map(1, 0).is_some());
    std::fs::remove_file(path).ok();
}

#[test]
fn mismatched_sidecar_fails_closed_without_affecting_default_identity_mapper() {
    let default_mapper = mapper_for(&pack_without_map());
    assert!(default_mapper.map(1, 0).is_none());

    let path: PathBuf =
        std::env::temp_dir().join(format!("velqu-source-map-bad-{}.json", std::process::id()));
    std::fs::write(
        &path,
        r#"{"formatVersion":1,"packSha256":"00","sourceMap":"{}"}"#,
    )
    .unwrap();
    let result = mapper_for_sidecar(&path, b"verified-pack");
    assert!(result.is_err());
    assert!(result.err().unwrap().contains("packSha256 mismatch"));
    std::fs::remove_file(path).ok();
}

#[test]
fn invalid_embedded_map_falls_back_to_identity() {
    let mut pack = pack_without_map();
    pack.source_map = Some("not-json".into());
    assert!(mapper_for(&pack).map(1, 0).is_none());
    assert!(SourcemapMapper::parse("not-json").is_err());
}
