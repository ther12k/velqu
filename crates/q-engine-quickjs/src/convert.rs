//! JS value conversion between the QuickJS world and serde_json.
//! Conversion cost is the measurable core of bridge strategy B (PERF-004).

use rquickjs::{Array, Coerced, Ctx, IntoJs, Object, TypedArray, Value};
use serde_json::{Map, Number, Value as Json};

/// serde_json::Value -> fresh JS value (recursive object construction).
pub fn json_to_js<'js>(ctx: &Ctx<'js>, v: &Json) -> rquickjs::Result<Value<'js>> {
    match v {
        Json::Null => ().into_js(ctx),
        Json::Bool(b) => b.into_js(ctx),
        Json::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                    (i as i32).into_js(ctx)
                } else {
                    (i as f64).into_js(ctx)
                }
            } else {
                n.as_f64().unwrap_or(f64::NAN).into_js(ctx)
            }
        }
        Json::String(s) => s.as_str().into_js(ctx),
        Json::Array(items) => {
            let arr = Array::new(ctx.clone())?;
            for (i, item) in items.iter().enumerate() {
                arr.set(i, json_to_js(ctx, item)?)?;
            }
            Ok(arr.into_value())
        }
        Json::Object(map) => {
            let obj = Object::new(ctx.clone())?;
            for (k, val) in map {
                obj.set(k.as_str(), json_to_js(ctx, val)?)?;
            }
            Ok(obj.into_value())
        }
    }
}

/// Read a JS object into a JSON object map.
pub fn js_object_to_json(o: &Object<'_>) -> rquickjs::Result<Json> {
    let mut map = Map::new();
    for key in o.keys::<String>() {
        let key = key?;
        let val: Value = o.get(key.as_str())?;
        map.insert(key, any_js_to_json(&val)?);
    }
    Ok(Json::Object(map))
}

/// Convert any JS value (primitives, arrays, plain objects) to JSON.
pub fn any_js_to_json(v: &Value<'_>) -> rquickjs::Result<Json> {
    if v.is_undefined() || v.is_null() {
        return Ok(Json::Null);
    }
    if let Some(b) = v.as_bool() {
        return Ok(Json::Bool(b));
    }
    if v.is_number() {
        if let Some(i) = v.as_int() {
            return Ok(Json::Number(Number::from(i)));
        }
        let f = v.as_float().unwrap_or(f64::NAN);
        if f.is_finite() && f.fract() == 0.0 && f >= (i64::MIN as f64) && f <= (i64::MAX as f64) {
            return Ok(Json::Number(Number::from(f as i64)));
        }
        return Ok(Number::from_f64(f).map(Json::Number).unwrap_or(Json::Null));
    }
    if v.is_string() {
        let s: Coerced<String> = v.clone().get()?;
        return Ok(Json::String(s.0));
    }
    if v.is_array() {
        let arr: Array = v.clone().get()?;
        let mut out = Vec::with_capacity(arr.len());
        for item in arr.iter::<Value>() {
            out.push(any_js_to_json(&item?)?);
        }
        return Ok(Json::Array(out));
    }
    if v.is_object() {
        if let Some(o) = v.as_object() {
            return js_object_to_json(o);
        }
    }
    // functions, symbols, etc. cannot be serialized
    Err(rquickjs::Error::FromJs {
        from: "function/symbol",
        to: "json",
        message: None,
    })
}

/// Engine-side JSON.stringify (strategy A response path). Returns None when
/// JSON.stringify drops the value (undefined/function) — same semantics.
pub fn engine_stringify<'js>(ctx: &Ctx<'js>, v: &Value<'js>) -> rquickjs::Result<Option<String>> {
    let json: Object = ctx.globals().get("JSON")?;
    let stringify: rquickjs::Function = json.get("stringify")?;
    let out: Option<Coerced<String>> = stringify.call((v.clone(),))?;
    Ok(out.map(|s| s.0))
}

/// Read a JS Uint8Array as bytes (pre-serialized response path).
pub fn js_to_bytes(v: &Value<'_>) -> Option<Vec<u8>> {
    let ta: TypedArray<'_, u8> = v.clone().get().ok()?;
    Some(ta.as_bytes()?.to_vec())
}
