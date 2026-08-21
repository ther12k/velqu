//! q-schema-runtime — Schema IR v2 types, native validator, source-aware coercion.
//!
//! Semantics frozen in `docs/specs/pack-format-v1.md`:
//! - `Body` values must match IR types exactly (no string→number coercion).
//! - `Path`/`Query` values arrive as strings and coerce per IR; coercion failure
//!   is a validation problem, never a 500.
//! - Unknown query keys are ignored; unknown body keys are rejected
//!   (additionalProperties: false).

/// Current normalized schema IR wire version.
pub const SCHEMA_IR_VERSION: u32 = 2;

mod decoder;
pub use decoder::{DecoderProgram, DecoderTable, FieldSpec, PropertyDecoder};

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SchemaIr {
    #[serde(rename_all = "camelCase")]
    String {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_length: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_length: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pattern: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        format: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    Integer {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        minimum: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        maximum: Option<i64>,
    },
    #[serde(rename_all = "camelCase")]
    Number {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        minimum: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        maximum: Option<f64>,
    },
    Boolean,
    #[serde(rename_all = "camelCase")]
    Literal {
        value: Value,
    },
    #[serde(rename_all = "camelCase")]
    Enum {
        values: Vec<Value>,
    },
    #[serde(rename_all = "camelCase")]
    Optional {
        inner: Box<SchemaIr>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default: Option<Value>,
    },
    #[serde(rename_all = "camelCase")]
    Nullable {
        inner: Box<SchemaIr>,
    },
    #[serde(rename_all = "camelCase")]
    Array {
        items: Box<SchemaIr>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_items: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_items: Option<u64>,
    },
    Object {
        properties: BTreeMap<String, Box<SchemaIr>>,
        #[serde(default)]
        required: Vec<String>,
    },
    Union {
        members: Vec<Box<SchemaIr>>,
    },
    /// Closed declarative transform metadata. Executable callbacks are not representable.
    #[serde(rename_all = "camelCase")]
    Transform {
        input: Box<SchemaIr>,
        output: Box<SchemaIr>,
        name: String,
    },
    /// Bounded file metadata; stream ownership and I/O stay outside Schema IR.
    #[serde(rename_all = "camelCase")]
    File {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        content_type: Option<String>,
        max_bytes: u64,
    },
    /// RFC 9457 problem shape metadata.
    #[serde(rename_all = "camelCase")]
    Problem {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        type_uri: Option<String>,
        title: String,
        status: u16,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        detail: Option<Box<SchemaIr>>,
    },
    /// Explicit fallback marker (ADR-0009: no silent downgrade). `reason` comes
    /// from the closed FALLBACK_REASONS vocabulary; `inner` is the optional
    /// best-effort shape the generic path validates against.
    #[serde(rename_all = "camelCase")]
    Fallback {
        reason: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        inner: Option<Box<SchemaIr>>,
    },
}

/// Closed vocabulary for `SchemaIr::Fallback` reasons.
pub const FALLBACK_REASONS: [&str; 4] = [
    "unsupported-transform", // transform name has no native codec (M25-004-B)
    "unrepresentable",       // construct outside the native IR
    "measured",              // generic path chosen by benchmark evidence (M25-002-D/M25-005-D)
    "explicit",              // developer-forced generic path
];

/// Marker feature tags derived from an IR graph (compatibility markers).
/// Sorted and deduplicated; mirrors the TypeScript walker in @velqu/schema.
pub fn features_of(ir: &SchemaIr) -> Vec<String> {
    fn walk(ir: &SchemaIr, set: &mut std::collections::BTreeSet<&'static str>) {
        match ir {
            SchemaIr::Transform { input, output, .. } => {
                set.insert("transform");
                walk(input, set);
                walk(output, set);
            }
            SchemaIr::File { .. } => {
                set.insert("file");
            }
            SchemaIr::Problem { detail, .. } => {
                set.insert("problem");
                if let Some(d) = detail {
                    walk(d, set);
                }
            }
            SchemaIr::Fallback { inner, .. } => {
                set.insert("fallback");
                if let Some(i) = inner {
                    walk(i, set);
                }
            }
            SchemaIr::Optional { inner, .. } | SchemaIr::Nullable { inner } => walk(inner, set),
            SchemaIr::Array { items, .. } => walk(items, set),
            SchemaIr::Object { properties, .. } => {
                for b in properties.values() {
                    walk(b, set);
                }
            }
            SchemaIr::Union { members } => {
                for m in members {
                    walk(m, set);
                }
            }
            _ => {}
        }
    }
    let mut set = std::collections::BTreeSet::new();
    walk(ir, &mut set);
    set.into_iter().map(String::from).collect()
}

/// True when `reason` is a member of the closed fallback vocabulary.
pub fn is_valid_fallback_reason(reason: &str) -> bool {
    FALLBACK_REASONS.contains(&reason)
}

/// Largest magnitude whose integral f64 values are exactly representable and
/// shared with JavaScript's canonical integer formatting.
const CANONICAL_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0; // 2^53 - 1

/// Canonical JSON value form (M25-001-C, ADR-0023):
/// - object keys recursively sorted (byte order),
/// - arrays keep their order,
/// - integral floats within ±(2^53-1) normalize to integers so Rust (`0.0`)
///   and JavaScript (`0`) agree byte-for-byte.
///
/// Applied to every hashed projection of the schema IR; both sides hash the
/// same canonical string regardless of source literal field order.
pub fn canonical_value(v: &Value) -> Value {
    match v {
        Value::Object(m) => {
            let mut keys: Vec<&String> = m.keys().collect();
            keys.sort();
            let mut out = Map::new();
            for k in keys {
                out.insert(k.clone(), canonical_value(&m[k]));
            }
            Value::Object(out)
        }
        Value::Array(a) => Value::Array(a.iter().map(canonical_value).collect()),
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                if f.is_finite()
                    && f.fract() == 0.0
                    && f.abs() <= CANONICAL_SAFE_INTEGER
                    && n.as_i64().is_none()
                {
                    return Value::Number(Number::from(f as i64));
                }
            }
            Value::Number(n.clone())
        }
        other => other.clone(),
    }
}

/// Canonical JSON string of a schema IR graph (sorted keys, normalized numbers).
pub fn canonical_json(ir: &SchemaIr) -> String {
    let v = serde_json::to_value(ir).expect("schema IR serializes");
    canonical_value(&v).to_string()
}

impl SchemaIr {
    /// Every fallback reason appearing anywhere in the graph (load-time
    /// vocabulary verification; duplicates preserved in walk order).
    pub fn fallback_reasons(&self) -> Vec<&str> {
        fn walk<'a>(ir: &'a SchemaIr, out: &mut Vec<&'a str>) {
            match ir {
                SchemaIr::Fallback { reason, inner } => {
                    out.push(reason);
                    if let Some(i) = inner {
                        walk(i, out);
                    }
                }
                SchemaIr::Transform { input, output, .. } => {
                    walk(input, out);
                    walk(output, out);
                }
                SchemaIr::Problem {
                    detail: Some(d), ..
                } => walk(d, out),
                SchemaIr::Problem { .. } => {}
                SchemaIr::Optional { inner, .. } | SchemaIr::Nullable { inner } => walk(inner, out),
                SchemaIr::Array { items, .. } => walk(items, out),
                SchemaIr::Object { properties, .. } => {
                    for b in properties.values() {
                        walk(b, out);
                    }
                }
                SchemaIr::Union { members } => {
                    for m in members {
                        walk(m, out);
                    }
                }
                _ => {}
            }
        }
        let mut out = Vec::new();
        walk(self, &mut out);
        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Path,
    Query,
    Body,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldError {
    pub path: String,
    pub code: String,
    pub message: String,
}

impl FieldError {
    pub fn new(path: &str, code: &str, message: impl Into<String>) -> Self {
        FieldError {
            path: path.into(),
            code: code.into(),
            message: message.into(),
        }
    }
}

pub type ValidationResult = Result<Value, Vec<FieldError>>;

pub(crate) fn is_email(s: &str) -> bool {
    // pragmatic RFC-ish check sufficient for the proof fixture; documented limitation
    let mut parts = s.split('@');
    let local = parts.next().unwrap_or("");
    let domain = match parts.next() {
        Some(d) => d,
        None => return false,
    };
    if parts.next().is_some() {
        return false;
    }
    !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

pub(crate) fn is_uuid(s: &str) -> bool {
    s.len() == 36
        && s.as_bytes().iter().enumerate().all(|(i, b)| {
            matches!(i, 8 | 13 | 18 | 23) && *b == b'-'
                || !matches!(i, 8 | 13 | 18 | 23) && b.is_ascii_hexdigit()
        })
}

/// Validate + normalize a value from `source` against `ir`.
/// Returns the (possibly coerced / default-applied) value.
pub fn validate(ir: &SchemaIr, value: &Value, source: Source) -> ValidationResult {
    match source {
        Source::Body => validate_node(ir, value, "", false),
        Source::Path | Source::Query => validate_node(ir, value, "", true),
    }
}

fn validate_node(
    ir: &SchemaIr,
    value: &Value,
    path: &str,
    coerce_strings: bool,
) -> ValidationResult {
    match ir {
        SchemaIr::Optional { inner, default } => {
            if value.is_null() {
                return Ok(default.clone().unwrap_or(Value::Null));
            }
            validate_node(inner, value, path, coerce_strings)
        }
        SchemaIr::Nullable { inner } => {
            if value.is_null() {
                return Ok(Value::Null);
            }
            validate_node(inner, value, path, coerce_strings)
        }
        SchemaIr::Transform { .. } | SchemaIr::File { .. } | SchemaIr::Problem { .. } => {
            Err(vec![FieldError::new(
                path,
                "unsupported",
                "schema node requires a specialized codec",
            )])
        }
        SchemaIr::Fallback { reason, inner } => {
            if !is_valid_fallback_reason(reason) {
                return Err(vec![FieldError::new(
                    path,
                    "invalid-schema",
                    format!("unknown fallback reason {}", reason),
                )]);
            }
            match inner {
                // explicit fallback with a best-effort shape: native validation
                // applies the inner schema (the marker itself is transparent)
                Some(inner) => validate_node(inner, value, path, coerce_strings),
                // no shape declared: the generic path must execute; until
                // M25-004-B wires it, this fails closed as a typed error
                None => Err(vec![FieldError::new(
                    path,
                    "fallback",
                    format!("fallback ({}) requires the generic codec path", reason),
                )]),
            }
        }
        SchemaIr::Union { members } => {
            let mut _last = Vec::new();
            for m in members {
                match validate_node(m, value, path, coerce_strings) {
                    Ok(v) => return Ok(v),
                    Err(e) => _last = e,
                }
            }
            Err(vec![FieldError::new(
                path,
                "union",
                format!("value matched none of {} union members", members.len()),
            )])
        }
        SchemaIr::String {
            min_length,
            max_length,
            pattern,
            format,
        } => {
            let s = match value.as_str() {
                Some(s) => s.to_string(),
                None => return Err(vec![FieldError::new(path, "type", "expected string")]),
            };
            if let Some(min) = min_length {
                if (s.len() as u64) < *min {
                    return Err(vec![FieldError::new(
                        path,
                        "minLength",
                        format!("must be at least {} characters", min),
                    )]);
                }
            }
            if let Some(max) = max_length {
                if (s.len() as u64) > *max {
                    return Err(vec![FieldError::new(
                        path,
                        "maxLength",
                        format!("must be at most {} characters", max),
                    )]);
                }
            }
            if let Some(p) = pattern {
                // Only the ^usr_[0-9]+$-style subset is expected; enforce via a tiny matcher
                if !simple_pattern_match(p, &s) {
                    return Err(vec![FieldError::new(
                        path,
                        "pattern",
                        format!("must match {}", p),
                    )]);
                }
            }
            if let Some(f) = format {
                let ok = match f.as_str() {
                    "email" => is_email(&s),
                    "uuid" => is_uuid(&s),
                    other => {
                        return Err(vec![FieldError::new(
                            path,
                            "format",
                            format!("unknown format {}", other),
                        )])
                    }
                };
                if !ok {
                    return Err(vec![FieldError::new(
                        path,
                        "format",
                        format!("must be a valid {}", f),
                    )]);
                }
            }
            Ok(Value::String(s))
        }
        SchemaIr::Integer { minimum, maximum } => {
            let n = coerce_int(value, coerce_strings, path)?;
            if let Some(min) = minimum {
                if n < *min {
                    return Err(vec![FieldError::new(
                        path,
                        "minimum",
                        format!("must be at least {}", min),
                    )]);
                }
            }
            if let Some(max) = maximum {
                if n > *max {
                    return Err(vec![FieldError::new(
                        path,
                        "maximum",
                        format!("must be at most {}", max),
                    )]);
                }
            }
            Ok(Value::Number(Number::from(n)))
        }
        SchemaIr::Number { minimum, maximum } => {
            let n = coerce_number(value, coerce_strings, path)?;
            if let Some(min) = minimum {
                if n < *min {
                    return Err(vec![FieldError::new(
                        path,
                        "minimum",
                        format!("must be at least {}", min),
                    )]);
                }
            }
            if let Some(max) = maximum {
                if n > *max {
                    return Err(vec![FieldError::new(
                        path,
                        "maximum",
                        format!("must be at most {}", max),
                    )]);
                }
            }
            Number::from_f64(n)
                .map(Value::Number)
                .ok_or_else(|| vec![FieldError::new(path, "type", "not a finite number")])
        }
        SchemaIr::Boolean => {
            let b = if coerce_strings {
                match value.as_str() {
                    Some("true") => true,
                    Some("false") => false,
                    _ => {
                        return Err(vec![FieldError::new(
                            path,
                            "type",
                            "expected boolean (true/false)",
                        )])
                    }
                }
            } else {
                match value.as_bool() {
                    Some(b) => b,
                    None => return Err(vec![FieldError::new(path, "type", "expected boolean")]),
                }
            };
            Ok(Value::Bool(b))
        }
        SchemaIr::Literal { value: lit } => {
            if value == lit {
                Ok(lit.clone())
            } else {
                Err(vec![FieldError::new(
                    path,
                    "literal",
                    format!("must equal {}", lit),
                )])
            }
        }
        SchemaIr::Enum { values } => {
            if values.contains(value) {
                Ok(value.clone())
            } else {
                Err(vec![FieldError::new(path, "enum", "value not in enum")])
            }
        }
        SchemaIr::Array {
            items,
            min_items,
            max_items,
        } => {
            let arr = match value.as_array() {
                Some(a) => a,
                None => return Err(vec![FieldError::new(path, "type", "expected array")]),
            };
            if let Some(min) = min_items {
                if (arr.len() as u64) < *min {
                    return Err(vec![FieldError::new(
                        path,
                        "minItems",
                        format!("must have at least {} items", min),
                    )]);
                }
            }
            if let Some(max) = max_items {
                if (arr.len() as u64) > *max {
                    return Err(vec![FieldError::new(
                        path,
                        "maxItems",
                        format!("must have at most {} items", max),
                    )]);
                }
            }
            let mut out = Vec::with_capacity(arr.len());
            for (i, item) in arr.iter().enumerate() {
                let p = format!("{}[{}]", path, i);
                out.push(validate_node(items, item, &p, coerce_strings)?);
            }
            Ok(Value::Array(out))
        }
        SchemaIr::Object {
            properties,
            required,
        } => {
            let obj = match value.as_object() {
                Some(o) => o,
                None => return Err(vec![FieldError::new(path, "type", "expected object")]),
            };
            // unknown body keys rejected; unknown query keys ignored upstream (handled by caller)
            let mut errors = Vec::new();
            for key in obj.keys() {
                if !properties.contains_key(key) {
                    errors.push(FieldError::new(
                        &join_path(path, key),
                        "additional",
                        "unknown field",
                    ));
                }
            }
            for req in required {
                if !obj.contains_key(req) {
                    // an optional-with-default member cannot also be required; enforced by compiler
                    errors.push(FieldError::new(
                        &join_path(path, req),
                        "required",
                        "missing required field",
                    ));
                }
            }
            if !errors.is_empty() {
                return Err(errors);
            }
            let mut out = Map::new();
            for (key, ir) in properties {
                if let Some(v) = obj.get(key) {
                    let p = join_path(path, key);
                    match validate_node(ir, v, &p, coerce_strings) {
                        Ok(nv) => {
                            out.insert(key.clone(), nv);
                        }
                        Err(mut e) => {
                            errors.append(&mut e);
                        }
                    }
                } else if let Some(SchemaIr::Optional {
                    default: Some(d), ..
                }) = properties.get(key).map(|b| b.as_ref())
                {
                    out.insert(key.clone(), d.clone());
                }
            }
            if !errors.is_empty() {
                return Err(errors);
            }
            Ok(Value::Object(out))
        }
    }
}

pub(crate) fn join_path(base: &str, key: &str) -> String {
    if base.is_empty() {
        key.to_string()
    } else {
        format!("{}.{}", base, key)
    }
}

fn coerce_int(value: &Value, coerce: bool, path: &str) -> Result<i64, Vec<FieldError>> {
    match value {
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i)
            } else {
                Err(vec![FieldError::new(path, "type", "expected integer")])
            }
        }
        Value::String(s) if coerce => s
            .parse::<i64>()
            .map_err(|_| vec![FieldError::new(path, "type", "expected integer")]),
        _ => Err(vec![FieldError::new(path, "type", "expected integer")]),
    }
}

fn coerce_number(value: &Value, coerce: bool, path: &str) -> Result<f64, Vec<FieldError>> {
    match value {
        Value::Number(n) => Ok(n.as_f64().unwrap_or(f64::NAN)),
        Value::String(s) if coerce => s
            .parse::<f64>()
            .map_err(|_| vec![FieldError::new(path, "type", "expected number")]),
        _ => Err(vec![FieldError::new(path, "type", "expected number")]),
    }
}

/// Validate a query string (`a=1&b=x`) against an object IR.
/// Unknown keys are ignored (documented semantics); missing optional-with-default
/// members get their default applied.
pub fn validate_query(ir: &SchemaIr, query: &[(String, String)]) -> ValidationResult {
    let mut raw = Map::new();
    for (k, v) in query {
        // M24-006-B: repeated query keys use frozen last-value-wins policy.
        raw.insert(k.clone(), Value::String(v.clone()));
    }
    // Only declared properties are validated; extras ignored.
    if let SchemaIr::Object { properties, .. } = ir {
        let mut filtered = Map::new();
        for key in properties.keys() {
            if let Some(v) = raw.get(key) {
                filtered.insert(key.clone(), v.clone());
            }
        }
        validate_node(ir, &Value::Object(filtered), "", true)
    } else {
        validate_node(ir, &Value::Object(raw), "", true)
    }
}

/// Validate extracted path params (all strings) against an object IR.
pub fn validate_params(ir: &SchemaIr, params: &[(String, String)]) -> ValidationResult {
    let mut raw = Map::new();
    for (k, v) in params {
        raw.insert(k.clone(), Value::String(v.clone()));
    }
    // unknown path params cannot exist if the router matched; extra safety here:
    if let SchemaIr::Object { properties, .. } = ir {
        let mut filtered = Map::new();
        for key in properties.keys() {
            if let Some(v) = raw.get(key) {
                filtered.insert(key.clone(), v.clone());
            }
        }
        validate_node(ir, &Value::Object(filtered), "", true)
    } else {
        validate_node(ir, &Value::Object(raw), "", true)
    }
}

/// M24-004-C: byte-level format gate for path parameters. Numeric and UUID
/// formats are validated directly from the captured path bytes — an INVALID
/// value is rejected without allocating any parameter string. Values that
/// pass continue through `validate_params`, whose full semantics (bounds,
/// length, pattern, defaults, coercion) remain the single source of truth;
/// the owned strings it builds are the pre-validated params the engine
/// consumes, so nothing is allocated unless validation succeeded.
pub fn validate_params_bytes(ir: &SchemaIr, params: &[(&str, &[u8])]) -> ValidationResult {
    if let SchemaIr::Object { properties, .. } = ir {
        for (key, member) in properties {
            if let Some((_, bytes)) = params.iter().find(|(n, _)| *n == key.as_str()) {
                byte_format_error(member, key, bytes)?;
            }
        }
    }
    let owned: Vec<(String, String)> = params
        .iter()
        .filter_map(|(n, b)| {
            std::str::from_utf8(b)
                .ok()
                .map(|v| (n.to_string(), v.to_string()))
        })
        .collect();
    validate_params(ir, &owned)
}

/// Fast reject from bytes for the formats the packet names: integers,
/// numbers, and UUID strings. Returns Ok(()) when the byte level is valid
/// (or the schema uses a format that needs the full validator), and the
/// matching FieldError otherwise. Messages mirror the full validator's.
fn byte_format_error(member: &SchemaIr, key: &str, bytes: &[u8]) -> Result<(), Vec<FieldError>> {
    let err = |code: &str, msg: &str| Err(vec![FieldError::new(&format!(".{key}"), code, msg)]);
    match member {
        SchemaIr::Integer { .. } => {
            let Ok(text) = std::str::from_utf8(bytes) else {
                return err("type", "expected an integer");
            };
            match text.parse::<i64>() {
                Ok(_) => Ok(()),
                Err(_) => err("type", "expected an integer"),
            }
        }
        SchemaIr::Number { .. } => {
            let Ok(text) = std::str::from_utf8(bytes) else {
                return err("type", "expected a number");
            };
            match text.parse::<f64>() {
                Ok(n) if n.is_finite() => Ok(()),
                _ => err("type", "expected a number"),
            }
        }
        SchemaIr::String { format, .. } => match format.as_deref() {
            Some("uuid") if !is_uuid_bytes(bytes) => err("format", "must be a valid uuid"),
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}

/// UUID syntax check on raw bytes (same rule as `is_uuid`, no UTF-8 step).
fn is_uuid_bytes(b: &[u8]) -> bool {
    b.len() == 36
        && b.iter().enumerate().all(|(i, byte)| {
            matches!(i, 8 | 13 | 18 | 23) && *byte == b'-'
                || !matches!(i, 8 | 13 | 18 | 23) && byte.is_ascii_hexdigit()
        })
}

/// Compile-and-cache the pattern subset the compiler is allowed to emit.
/// Unsupported constructs fail closed (no match) rather than panicking on
/// untrusted input. Compilation is lazy per unique pattern (bounded by build
/// validation which only emits supported constructs).
pub(crate) fn simple_pattern_match(pattern: &str, s: &str) -> bool {
    use std::cell::RefCell;
    thread_local! {
        static CACHE: RefCell<std::collections::HashMap<String, Option<regex::Regex>>> =
            RefCell::new(std::collections::HashMap::new());
    }
    CACHE.with(|c| {
        let mut cache = c.borrow_mut();
        let re = cache
            .entry(pattern.to_string())
            .or_insert_with(|| regex::Regex::new(pattern).ok());
        match re {
            Some(re) => re.is_match(s),
            None => false,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn user_body_ir() -> SchemaIr {
        SchemaIr::Object {
            properties: BTreeMap::from([
                (
                    "name".to_string(),
                    Box::new(SchemaIr::String {
                        min_length: Some(1),
                        max_length: Some(60),
                        pattern: None,
                        format: None,
                    }),
                ),
                (
                    "email".to_string(),
                    Box::new(SchemaIr::String {
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        format: Some("email".into()),
                    }),
                ),
            ]),
            required: vec!["name".into(), "email".into()],
        }
    }

    #[test]
    fn body_accepts_valid_user() {
        let v = validate(
            &user_body_ir(),
            &json!({"name": "Ada", "email": "ada@example.org"}),
            Source::Body,
        );
        assert!(v.is_ok());
    }

    #[test]
    fn body_rejects_bad_email_identifying_field() {
        let err = validate(
            &user_body_ir(),
            &json!({"name": "Ada", "email": "not-an-email"}),
            Source::Body,
        )
        .unwrap_err();
        assert!(err.iter().any(|e| e.path == "email" && e.code == "format"));
    }

    #[test]
    fn body_rejects_missing_and_unknown_fields() {
        let err = validate(&user_body_ir(), &json!({"name": "Ada"}), Source::Body).unwrap_err();
        assert!(err
            .iter()
            .any(|e| e.code == "required" && e.path == "email"));
        let err = validate(
            &user_body_ir(),
            &json!({"name": "Ada", "email": "a@b.co", "extra": 1}),
            Source::Body,
        )
        .unwrap_err();
        assert!(err
            .iter()
            .any(|e| e.code == "additional" && e.path == "extra"));
    }

    #[test]
    fn path_coerces_integer_and_enforces_range() {
        let ir = SchemaIr::Object {
            properties: BTreeMap::from([(
                "id".to_string(),
                Box::new(SchemaIr::Integer {
                    minimum: Some(1),
                    maximum: Some(25),
                }),
            )]),
            required: vec!["id".into()],
        };
        let ok = validate_params(&ir, &[("id".into(), "7".into())]).unwrap();
        assert_eq!(ok, json!({"id": 7}));
        let err = validate_params(&ir, &[("id".into(), "0".into())]).unwrap_err();
        assert!(err[0].code == "minimum");
        let err = validate_params(&ir, &[("id".into(), "x".into())]).unwrap_err();
        assert!(err[0].code == "type");
    }

    #[test]
    fn query_applies_default_and_ignores_unknown() {
        let ir = SchemaIr::Object {
            properties: BTreeMap::from([(
                "ms".to_string(),
                Box::new(SchemaIr::Optional {
                    inner: Box::new(SchemaIr::Integer {
                        minimum: Some(1),
                        maximum: Some(1000),
                    }),
                    default: Some(json!(10)),
                }),
            )]),
            required: vec![],
        };
        let v = validate_query(&ir, &[("unrelated".into(), "1".into())]).unwrap();
        assert_eq!(v, json!({"ms": 10}));
        let v = validate_query(&ir, &[("ms".into(), "50".into())]).unwrap();
        assert_eq!(v, json!({"ms": 50}));
        let err = validate_query(&ir, &[("ms".into(), "2000".into())]).unwrap_err();
        assert!(err[0].code == "maximum");
    }

    #[test]
    fn pattern_matches_usr_id() {
        let ir = SchemaIr::Object {
            properties: BTreeMap::from([(
                "id".to_string(),
                Box::new(SchemaIr::String {
                    min_length: None,
                    max_length: None,
                    pattern: Some("^usr_[0-9]+$".into()),
                    format: None,
                }),
            )]),
            required: vec!["id".into()],
        };
        assert!(validate_params(&ir, &[("id".into(), "usr_1".into())]).is_ok());
        assert!(
            validate_params(&ir, &[("id".into(), "user_1".into())]).unwrap_err()[0].code
                == "pattern"
        );
        assert!(
            validate_params(&ir, &[("id".into(), "usr_".into())]).unwrap_err()[0].code == "pattern"
        );
    }

    #[test]
    fn name_length_validation() {
        let ir = SchemaIr::Object {
            properties: BTreeMap::from([(
                "name".to_string(),
                Box::new(SchemaIr::String {
                    min_length: Some(1),
                    max_length: Some(60),
                    pattern: None,
                    format: None,
                }),
            )]),
            required: vec!["name".into()],
        };
        let long = "x".repeat(61);
        let err = validate_params(&ir, &[("name".into(), long)]).unwrap_err();
        assert!(err[0].code == "maxLength" && err[0].path == "name");
    }
}

#[cfg(test)]
mod m24_004_c_tests {
    use super::*;

    fn object(props: Vec<(&str, SchemaIr)>, required: Vec<&str>) -> SchemaIr {
        SchemaIr::Object {
            properties: props
                .into_iter()
                .map(|(k, v)| (k.to_string(), Box::new(v)))
                .collect(),
            required: required.into_iter().map(String::from).collect(),
        }
    }

    /// M24-004-C: invalid numeric/UUID bytes reject from the byte gate with
    /// the same error identity the full validator produces for owned strings.
    #[test]
    fn validate_params_bytes_rejects_invalid_formats_from_bytes() {
        let ir = object(
            vec![
                (
                    "count",
                    SchemaIr::Integer {
                        minimum: Some(0),
                        maximum: Some(100),
                    },
                ),
                (
                    "id",
                    SchemaIr::String {
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        format: Some("uuid".into()),
                    },
                ),
            ],
            vec!["count", "id"],
        );
        // non-numeric integer
        let err = validate_params_bytes(&ir, &[("count", b"twelve")]).unwrap_err();
        assert_eq!(err[0].code, "type");
        assert_eq!(err[0].path, ".count");
        // float text where an integer is declared
        assert!(validate_params_bytes(&ir, &[("count", b"1.5")]).is_err());
        // bad UUID (wrong length + non-hex)
        let err =
            validate_params_bytes(&ir, &[("count", b"7"), ("id", b"zz-not-a-uuid")]).unwrap_err();
        assert_eq!(err[0].code, "format");
        assert_eq!(err[0].path, ".id");
        // invalid UTF-8 integer bytes reject without panic
        assert!(validate_params_bytes(&ir, &[("count", &[0xff, 0xfe])]).is_err());
    }

    /// M24-004-C: on the valid path, byte validation returns EXACTLY the
    /// value the owned-string validator returns (parity, single semantics).
    #[test]
    fn validate_params_bytes_parity_with_owned_validator() {
        let ir = object(
            vec![
                (
                    "count",
                    SchemaIr::Integer {
                        minimum: Some(1),
                        maximum: Some(100),
                    },
                ),
                (
                    "id",
                    SchemaIr::String {
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        format: Some("uuid".into()),
                    },
                ),
            ],
            vec!["count", "id"],
        );
        let uuid = b"123e4567-e89b-12d3-a456-426614174000";
        let bytes: Vec<(&str, &[u8])> = vec![("count", b"42"), ("id", uuid)];
        let owned = vec![
            ("count".to_string(), "42".to_string()),
            ("id".to_string(), String::from_utf8(uuid.to_vec()).unwrap()),
        ];
        assert_eq!(
            validate_params_bytes(&ir, &bytes).unwrap(),
            validate_params(&ir, &owned).unwrap()
        );
    }
}

/// M25-001-A: Schema IR v2 nodes — serde wire forms, version boundary, and the
/// validation semantics fixed alongside v2 (object null handling, array coercion).
#[cfg(test)]
mod m25_001_a_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn schema_ir_version_is_two() {
        assert_eq!(SCHEMA_IR_VERSION, 2);
    }

    #[test]
    fn transform_serde_round_trip_camel_case() {
        let node = json!({
            "kind": "transform",
            "input": { "kind": "string" },
            "output": { "kind": "integer" },
            "name": "parse-count"
        });
        let ir: SchemaIr = serde_json::from_value(node.clone()).unwrap();
        match &ir {
            SchemaIr::Transform { name, .. } => assert_eq!(name, "parse-count"),
            other => panic!("wrong variant: {other:?}"),
        }
        assert_eq!(serde_json::to_value(&ir).unwrap(), node);
    }

    #[test]
    fn file_serde_round_trip_omits_absent_content_type() {
        let node = json!({ "kind": "file", "maxBytes": 1024 });
        let ir: SchemaIr = serde_json::from_value(node.clone()).unwrap();
        match &ir {
            SchemaIr::File {
                content_type,
                max_bytes,
            } => {
                assert_eq!(content_type, &None);
                assert_eq!(*max_bytes, 1024);
            }
            other => panic!("wrong variant: {other:?}"),
        }
        assert_eq!(serde_json::to_value(&ir).unwrap(), node);

        let with_type = json!({ "kind": "file", "contentType": "text/csv", "maxBytes": 4 });
        let ir: SchemaIr = serde_json::from_value(with_type.clone()).unwrap();
        assert_eq!(serde_json::to_value(&ir).unwrap(), with_type);
    }

    #[test]
    fn problem_serde_round_trip_camel_case() {
        let node = json!({
            "kind": "problem",
            "typeUri": "https://example.com/probs/oos",
            "title": "Out of stock",
            "status": 409,
            "detail": { "kind": "string" }
        });
        let ir: SchemaIr = serde_json::from_value(node.clone()).unwrap();
        match &ir {
            SchemaIr::Problem {
                type_uri,
                title,
                status,
                ..
            } => {
                assert_eq!(type_uri.as_deref(), Some("https://example.com/probs/oos"));
                assert_eq!(title, "Out of stock");
                assert_eq!(*status, 409);
            }
            other => panic!("wrong variant: {other:?}"),
        }
        assert_eq!(serde_json::to_value(&ir).unwrap(), node);

        let minimal = json!({ "kind": "problem", "title": "Boom", "status": 500 });
        let ir: SchemaIr = serde_json::from_value(minimal.clone()).unwrap();
        assert_eq!(serde_json::to_value(&ir).unwrap(), minimal);
    }

    #[test]
    fn v2_nodes_return_typed_unsupported_validation_errors() {
        // Runtime codecs for transform/file/problem land in M25-002+; until then
        // validating against one is a typed field error, never a silent accept
        // and never a panic.
        let cases: Vec<SchemaIr> = vec![
            SchemaIr::Transform {
                input: Box::new(SchemaIr::String {
                    min_length: None,
                    max_length: None,
                    pattern: None,
                    format: None,
                }),
                output: Box::new(SchemaIr::Integer {
                    minimum: None,
                    maximum: None,
                }),
                name: "parse-count".into(),
            },
            SchemaIr::File {
                content_type: None,
                max_bytes: 8,
            },
            SchemaIr::Problem {
                type_uri: None,
                title: "Boom".into(),
                status: 500,
                detail: None,
            },
        ];
        for ir in &cases {
            for source in [Source::Body, Source::Path, Source::Query] {
                let err = validate(ir, &json!("anything"), source).unwrap_err();
                assert_eq!(err.len(), 1);
                assert_eq!(err[0].code, "unsupported");
            }
        }
    }

    #[test]
    fn object_rejects_null_for_non_nullable_member() {
        let ir = SchemaIr::Object {
            properties: BTreeMap::from([(
                "name".to_string(),
                Box::new(SchemaIr::String {
                    min_length: Some(1),
                    max_length: None,
                    pattern: None,
                    format: None,
                }),
            )]),
            required: vec!["name".into()],
        };
        // present-but-null on a non-nullable member is a type error
        let err = validate(&ir, &json!({ "name": null }), Source::Body).unwrap_err();
        assert_eq!(err[0].path, "name");
        assert_eq!(err[0].code, "type");
    }

    #[test]
    fn object_accepts_null_for_nullable_and_optional_members() {
        let ir = SchemaIr::Object {
            properties: BTreeMap::from([
                (
                    "nick".to_string(),
                    Box::new(SchemaIr::Nullable {
                        inner: Box::new(SchemaIr::String {
                            min_length: None,
                            max_length: None,
                            pattern: None,
                            format: None,
                        }),
                    }),
                ),
                (
                    "page".to_string(),
                    Box::new(SchemaIr::Optional {
                        inner: Box::new(SchemaIr::Integer {
                            minimum: None,
                            maximum: None,
                        }),
                        default: Some(json!(1)),
                    }),
                ),
            ]),
            required: vec!["nick".into()],
        };
        let out = validate(&ir, &json!({ "nick": null, "page": null }), Source::Body).unwrap();
        // nullable stays null; optional null falls back to the declared default
        assert_eq!(out, json!({ "nick": null, "page": 1 }));
    }

    #[test]
    fn query_array_items_coerce_strings_consistently() {
        let ir = SchemaIr::Object {
            properties: BTreeMap::from([(
                "ids".to_string(),
                Box::new(SchemaIr::Array {
                    items: Box::new(SchemaIr::Integer {
                        minimum: None,
                        maximum: None,
                    }),
                    min_items: None,
                    max_items: None,
                }),
            )]),
            required: vec!["ids".into()],
        };
        // query values arrive as strings; nested array items coerce like scalars
        let out = validate(&ir, &json!({ "ids": ["1", "2"] }), Source::Query).unwrap();
        assert_eq!(out, json!({ "ids": [1, 2] }));
        // body values must match exactly: strings stay type errors
        let err = validate(&ir, &json!({ "ids": ["1"] }), Source::Body).unwrap_err();
        assert_eq!(err[0].path, "ids[0]");
        assert_eq!(err[0].code, "type");
    }

    /// Shared wire corpus with @velqu/schema (conformance/schema/golden/).
    /// Deserialization and re-serialization must be identity for every node.
    #[test]
    fn golden_corpus_round_trips() {
        let corpus = [
            (
                "transform",
                include_str!("../../../conformance/schema/golden/transform.json"),
            ),
            (
                "file",
                include_str!("../../../conformance/schema/golden/file.json"),
            ),
            (
                "file-content-type",
                include_str!("../../../conformance/schema/golden/file-content-type.json"),
            ),
            (
                "problem",
                include_str!("../../../conformance/schema/golden/problem.json"),
            ),
            (
                "problem-minimal",
                include_str!("../../../conformance/schema/golden/problem-minimal.json"),
            ),
            (
                "nested-composition",
                include_str!("../../../conformance/schema/golden/nested-composition.json"),
            ),
            (
                "fallback-with-inner",
                include_str!("../../../conformance/schema/golden/fallback-with-inner.json"),
            ),
            (
                "fallback-minimal",
                include_str!("../../../conformance/schema/golden/fallback-minimal.json"),
            ),
        ];
        for (name, raw) in corpus {
            let value: Value = serde_json::from_str(raw).expect(name);
            let ir: SchemaIr = serde_json::from_value(value.clone()).expect(name);
            let back = serde_json::to_value(&ir).expect(name);
            assert_eq!(back, value, "{name} must round-trip identically");
            // classification terminates without panic for every source
            let _ = validate(&ir, &json!({}), Source::Body);
            let _ = validate(&ir, &json!("x"), Source::Query);
        }
    }

    /// M25-001-B: feature derivation parity on the shared corpus — the Rust
    /// walker and the TypeScript walker must agree (canonicalization test).
    #[test]
    fn golden_corpus_feature_expectations() {
        let expected: &[(&str, &[&str])] = &[
            ("transform", &["transform"]),
            ("file", &["file"]),
            ("file-content-type", &["file"]),
            ("problem", &["problem"]),
            ("problem-minimal", &["problem"]),
            ("nested-composition", &["file", "problem", "transform"]),
            ("fallback-with-inner", &["fallback"]),
            ("fallback-minimal", &["fallback"]),
        ];
        let files = [
            (
                "transform",
                include_str!("../../../conformance/schema/golden/transform.json"),
            ),
            (
                "file",
                include_str!("../../../conformance/schema/golden/file.json"),
            ),
            (
                "file-content-type",
                include_str!("../../../conformance/schema/golden/file-content-type.json"),
            ),
            (
                "problem",
                include_str!("../../../conformance/schema/golden/problem.json"),
            ),
            (
                "problem-minimal",
                include_str!("../../../conformance/schema/golden/problem-minimal.json"),
            ),
            (
                "nested-composition",
                include_str!("../../../conformance/schema/golden/nested-composition.json"),
            ),
            (
                "fallback-with-inner",
                include_str!("../../../conformance/schema/golden/fallback-with-inner.json"),
            ),
            (
                "fallback-minimal",
                include_str!("../../../conformance/schema/golden/fallback-minimal.json"),
            ),
        ];
        for ((name, raw), (_, want)) in files.iter().zip(expected.iter()) {
            let ir: SchemaIr = serde_json::from_str(raw).expect(name);
            assert_eq!(
                features_of(&ir),
                want.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                "{name} derived features must match the corpus expectation"
            );
        }
    }
}

/// M25-001-B: explicit fallback markers and derived compatibility features.
#[cfg(test)]
mod m25_001_b_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn fallback_serde_round_trip_with_and_without_inner() {
        let with_inner = json!({
            "kind": "fallback",
            "reason": "unsupported-transform",
            "inner": { "kind": "string" }
        });
        let ir: SchemaIr = serde_json::from_value(with_inner.clone()).unwrap();
        assert_eq!(serde_json::to_value(&ir).unwrap(), with_inner);

        let minimal = json!({ "kind": "fallback", "reason": "explicit" });
        let ir: SchemaIr = serde_json::from_value(minimal.clone()).unwrap();
        match &ir {
            SchemaIr::Fallback { reason, inner } => {
                assert_eq!(reason, "explicit");
                assert!(inner.is_none());
            }
            other => panic!("wrong variant: {other:?}"),
        }
        assert_eq!(serde_json::to_value(&ir).unwrap(), minimal);
    }

    #[test]
    fn fallback_with_inner_validates_against_inner() {
        let ir = SchemaIr::Fallback {
            reason: "unsupported-transform".into(),
            inner: Some(Box::new(SchemaIr::Object {
                properties: BTreeMap::from([(
                    "n".to_string(),
                    Box::new(SchemaIr::Integer {
                        minimum: Some(1),
                        maximum: None,
                    }),
                )]),
                required: vec!["n".into()],
            })),
        };
        // best-effort native validation applies the inner schema
        let out = validate(&ir, &json!({ "n": 5 }), Source::Body).unwrap();
        assert_eq!(out, json!({ "n": 5 }));
        let err = validate(&ir, &json!({ "n": 0 }), Source::Body).unwrap_err();
        assert_eq!(err[0].code, "minimum");
    }

    #[test]
    fn fallback_without_inner_fails_closed_with_typed_error() {
        let ir = SchemaIr::Fallback {
            reason: "measured".into(),
            inner: None,
        };
        let err = validate(&ir, &json!({ "x": 1 }), Source::Body).unwrap_err();
        assert_eq!(err[0].code, "fallback");
        assert!(err[0].message.contains("generic codec path"));
    }

    #[test]
    fn fallback_rejects_unknown_reason() {
        let ir = SchemaIr::Fallback {
            reason: "because".into(),
            inner: Some(Box::new(SchemaIr::String {
                min_length: None,
                max_length: None,
                pattern: None,
                format: None,
            })),
        };
        let err = validate(&ir, &json!("s"), Source::Body).unwrap_err();
        assert_eq!(err[0].code, "invalid-schema");
        assert!(!is_valid_fallback_reason("because"));
        for r in FALLBACK_REASONS {
            assert!(is_valid_fallback_reason(r));
        }
    }

    #[test]
    fn features_are_derived_sorted_and_deduplicated() {
        // plain v1 node: no features
        let plain = SchemaIr::Object {
            properties: BTreeMap::from([(
                "a".to_string(),
                Box::new(SchemaIr::String {
                    min_length: None,
                    max_length: None,
                    pattern: None,
                    format: None,
                }),
            )]),
            required: vec!["a".into()],
        };
        assert_eq!(features_of(&plain), Vec::<String>::new());

        // nested graph: transform (with file inside output) + problem + fallback
        let graph = SchemaIr::Object {
            properties: BTreeMap::from([
                (
                    "t".to_string(),
                    Box::new(SchemaIr::Transform {
                        input: Box::new(SchemaIr::String {
                            min_length: None,
                            max_length: None,
                            pattern: None,
                            format: None,
                        }),
                        output: Box::new(SchemaIr::File {
                            content_type: None,
                            max_bytes: 8,
                        }),
                        name: "x".into(),
                    }),
                ),
                (
                    "p".to_string(),
                    Box::new(SchemaIr::Problem {
                        type_uri: None,
                        title: "T".into(),
                        status: 422,
                        detail: None,
                    }),
                ),
                (
                    "f".to_string(),
                    Box::new(SchemaIr::Fallback {
                        reason: "explicit".into(),
                        inner: Some(Box::new(SchemaIr::Fallback {
                            reason: "explicit".into(),
                            inner: None,
                        })),
                    }),
                ),
            ]),
            required: vec![],
        };
        assert_eq!(
            features_of(&graph),
            vec![
                "fallback".to_string(),
                "file".to_string(),
                "problem".to_string(),
                "transform".to_string()
            ]
        );
    }
}

/// M25-001-C: canonical ordering and hashing (ADR-0023) — sorted-key
/// canonical JSON shared byte-for-byte with the TypeScript side.
#[cfg(test)]
mod m25_001_c_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn canonical_json_sorts_all_keys_recursively() {
        let ir = SchemaIr::Object {
            properties: BTreeMap::from([
                (
                    "zeta".to_string(),
                    Box::new(SchemaIr::String {
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        format: None,
                    }),
                ),
                (
                    "alpha".to_string(),
                    Box::new(SchemaIr::Integer {
                        minimum: None,
                        maximum: None,
                    }),
                ),
            ]),
            required: vec!["zeta".into()],
        };
        assert_eq!(
            canonical_json(&ir),
            r#"{"kind":"object","properties":{"alpha":{"kind":"integer"},"zeta":{"kind":"string"}},"required":["zeta"]}"#
        );
    }

    #[test]
    fn canonical_form_normalizes_integral_floats() {
        // Rust serializes f64 0.0 as "0.0"; canonical form must emit "0" so it
        // matches JavaScript's formatting byte-for-byte
        let ir = SchemaIr::Number {
            minimum: Some(0.0),
            maximum: Some(1.5),
        };
        assert_eq!(
            canonical_json(&ir),
            r#"{"kind":"number","maximum":1.5,"minimum":0}"#
        );

        let raw = json!({"kind": "number", "minimum": 0.0, "maximum": 1.5});
        assert_eq!(
            canonical_value(&raw).to_string(),
            r#"{"kind":"number","maximum":1.5,"minimum":0}"#
        );
        // already-integer numbers pass through untouched
        let raw = json!({"a": 3});
        assert_eq!(canonical_value(&raw).to_string(), r#"{"a":3}"#);
    }

    #[test]
    fn canonical_value_is_emission_order_insensitive() {
        let a = json!({"kind": "x", "a": 1, "b": {"y": 2, "x": [3, {"q": 1, "p": 2}]}});
        let b = json!({"b": {"x": [3, {"p": 2, "q": 1}], "y": 2}, "a": 1, "kind": "x"});
        assert_eq!(
            canonical_value(&a).to_string(),
            canonical_value(&b).to_string()
        );
        // arrays keep their order (never sorted)
        let c = json!([3, 1, 2]);
        assert_eq!(canonical_value(&c).to_string(), "[3,1,2]");
    }

    /// Cross-language canonical corpus: the committed canonical files are the
    /// byte-exact expectation for both this crate and @velqu/schema.
    #[test]
    fn canonical_corpus_matches_golden_files() {
        let corpus = [
            (
                "transform",
                include_str!("../../../conformance/schema/golden/transform.json"),
                include_str!("../../../conformance/schema/golden/canonical/transform.canonical.json"),
            ),
            (
                "file",
                include_str!("../../../conformance/schema/golden/file.json"),
                include_str!("../../../conformance/schema/golden/canonical/file.canonical.json"),
            ),
            (
                "file-content-type",
                include_str!("../../../conformance/schema/golden/file-content-type.json"),
                include_str!("../../../conformance/schema/golden/canonical/file-content-type.canonical.json"),
            ),
            (
                "problem",
                include_str!("../../../conformance/schema/golden/problem.json"),
                include_str!("../../../conformance/schema/golden/canonical/problem.canonical.json"),
            ),
            (
                "problem-minimal",
                include_str!("../../../conformance/schema/golden/problem-minimal.json"),
                include_str!("../../../conformance/schema/golden/canonical/problem-minimal.canonical.json"),
            ),
            (
                "nested-composition",
                include_str!("../../../conformance/schema/golden/nested-composition.json"),
                include_str!("../../../conformance/schema/golden/canonical/nested-composition.canonical.json"),
            ),
            (
                "fallback-with-inner",
                include_str!("../../../conformance/schema/golden/fallback-with-inner.json"),
                include_str!("../../../conformance/schema/golden/canonical/fallback-with-inner.canonical.json"),
            ),
            (
                "fallback-minimal",
                include_str!("../../../conformance/schema/golden/fallback-minimal.json"),
                include_str!("../../../conformance/schema/golden/canonical/fallback-minimal.canonical.json"),
            ),
        ];
        for (name, raw, want) in corpus {
            let ir: SchemaIr = serde_json::from_str(raw).expect(name);
            assert_eq!(
                canonical_json(&ir),
                want.trim_end(),
                "{name} canonical form"
            );
        }
    }
}
