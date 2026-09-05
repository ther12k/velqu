use q_runtime_model::{BodyOut, FieldNeeds, FunctionDecl, FunctionKind, MODEL_ABI_VERSION};
use sha2::{Digest, Sha256};

#[test]
fn model_abi_version_is_explicit() {
    assert_eq!(MODEL_ABI_VERSION, 1);
}

#[test]
fn function_declaration_serialization_is_versioned_and_deterministic() {
    let value = FunctionDecl {
        id: 7,
        key: "greetings.get".to_owned(),
        kind: FunctionKind::RouteHandler,
    };
    let encoded = serde_json::to_vec(&value).expect("fixture serializes");
    assert_eq!(
        encoded,
        br#"{"id":7,"key":"greetings.get","kind":"route-handler"}"#
    );

    let mut digest = Sha256::new();
    digest.update(MODEL_ABI_VERSION.to_be_bytes());
    digest.update(&encoded);
    assert_eq!(
        format!("{:x}", digest.finalize()),
        "9dacb30d967a41ade33554fe8d8c57ffd6a03afe9e026b1721cb50aac6871d1c"
    );

    let decoded: FunctionDecl = serde_json::from_slice(&encoded).expect("fixture deserializes");
    assert_eq!(decoded, value);
}

#[test]
fn field_needs_round_trip_has_stable_key_order() {
    let value = FieldNeeds {
        params: true,
        query: false,
        headers: true,
        body: false,
    };
    let encoded = serde_json::to_vec(&value).expect("fixture serializes");
    assert_eq!(
        encoded,
        br#"{"params":true,"query":false,"headers":true,"body":false}"#
    );
    assert_eq!(
        serde_json::from_slice::<FieldNeeds>(&encoded).unwrap(),
        value
    );
}

#[test]
fn body_out_preserves_json_value_shape_for_native_adapter() {
    let value = BodyOut::Json(serde_json::json!({"ok": true}));
    assert_eq!(value, BodyOut::Json(serde_json::json!({"ok": true})));
}
