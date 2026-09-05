//! Exports one validate call so the linker keeps the schema runtime.
use q_schema_runtime::{validate, SchemaIr, Source};

#[no_mangle]
pub extern "C" fn probe_validate(len: i32, ptr: *const u8) -> i32 {
    // Minimal ABI: JSON {schema, value} in, error count out. Kept
    // trivial on purpose — the point is to measure the linked size of
    // the schema runtime, not to be a real ABI.
    if ptr.is_null() {
        return -1;
    }
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
    let Ok(doc) = serde_json::from_slice::<serde_json::Value>(bytes) else {
        return -2;
    };
    let Some(schema_json) = doc.get("schema") else {
        return -3;
    };
    let Ok(ir) = serde_json::from_value::<SchemaIr>(schema_json.clone()) else {
        return -4;
    };
    let value = doc.get("value").cloned().unwrap_or(serde_json::Value::Null);
    let result = validate(&ir, &value, Source::Body);
    match result {
        Ok(_) => 0,
        Err(errors) => errors.len() as i32,
    }
}
