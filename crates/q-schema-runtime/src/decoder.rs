//! Direct decoder programs keyed by SchemaId (M25-003-A / M25-003-B).
//!
//! Compiles an object SchemaIr into a specialized, direct field decoder program
//! that fuses field extraction, byte-range slicing, string/byte coercion, bounds
//! validation, and default application into a single pass without intermediate
//! generic AST trees.

use std::collections::BTreeMap;

use serde_json::{Map, Number, Value};

use crate::{
    is_email, is_uuid, is_uuid_bytes, is_valid_fallback_reason, join_path, simple_pattern_match,
    FieldError, SchemaIr, Source, ValidationResult,
};

/// A compiled field-level specification for direct decoding.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldSpec {
    String {
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
    },
    Integer {
        minimum: Option<i64>,
        maximum: Option<i64>,
    },
    Number {
        minimum: Option<f64>,
        maximum: Option<f64>,
    },
    Boolean,
    Literal {
        value: Value,
    },
    Enum {
        values: Vec<Value>,
    },
    Optional {
        inner: Box<FieldSpec>,
        default: Option<Value>,
    },
    Nullable {
        inner: Box<FieldSpec>,
    },
    Array {
        items: Box<FieldSpec>,
        min_items: Option<u64>,
        max_items: Option<u64>,
    },
    Union {
        members: Vec<FieldSpec>,
    },
    Fallback {
        reason: String,
        inner: Option<Box<FieldSpec>>,
    },
    Unsupported {
        kind: &'static str,
    },
}

impl FieldSpec {
    pub fn compile(ir: &SchemaIr) -> Self {
        match ir {
            SchemaIr::String {
                min_length,
                max_length,
                pattern,
                format,
            } => FieldSpec::String {
                min_length: *min_length,
                max_length: *max_length,
                pattern: pattern.clone(),
                format: format.clone(),
            },
            SchemaIr::Integer { minimum, maximum } => FieldSpec::Integer {
                minimum: *minimum,
                maximum: *maximum,
            },
            SchemaIr::Number { minimum, maximum } => FieldSpec::Number {
                minimum: *minimum,
                maximum: *maximum,
            },
            SchemaIr::Boolean => FieldSpec::Boolean,
            SchemaIr::Literal { value } => FieldSpec::Literal {
                value: value.clone(),
            },
            SchemaIr::Enum { values } => FieldSpec::Enum {
                values: values.clone(),
            },
            SchemaIr::Optional { inner, default } => FieldSpec::Optional {
                inner: Box::new(FieldSpec::compile(inner)),
                default: default.clone(),
            },
            SchemaIr::Nullable { inner } => FieldSpec::Nullable {
                inner: Box::new(FieldSpec::compile(inner)),
            },
            SchemaIr::Array {
                items,
                min_items,
                max_items,
            } => FieldSpec::Array {
                items: Box::new(FieldSpec::compile(items)),
                min_items: *min_items,
                max_items: *max_items,
            },
            SchemaIr::Union { members } => FieldSpec::Union {
                members: members.iter().map(|m| FieldSpec::compile(m)).collect(),
            },
            SchemaIr::Fallback { reason, inner } => FieldSpec::Fallback {
                reason: reason.clone(),
                inner: inner.as_ref().map(|i| Box::new(FieldSpec::compile(i))),
            },
            SchemaIr::Transform { .. } => FieldSpec::Unsupported { kind: "transform" },
            SchemaIr::File { .. } => FieldSpec::Unsupported { kind: "file" },
            SchemaIr::Problem { .. } => FieldSpec::Unsupported { kind: "problem" },
            SchemaIr::Object { .. } => FieldSpec::Unsupported { kind: "object" },
        }
    }

    /// Direct decode from borrowed byte slices (e.g. from path parameter byte ranges).
    /// Avoids UTF-8 String allocations for integer, number, boolean, and UUID validation.
    pub fn decode_bytes(&self, bytes: &[u8], path: &str) -> Result<Value, Vec<FieldError>> {
        match self {
            FieldSpec::Integer { minimum, maximum } => {
                let Ok(s) = std::str::from_utf8(bytes) else {
                    return Err(vec![FieldError::new(path, "type", "expected integer")]);
                };
                let n = s
                    .parse::<i64>()
                    .map_err(|_| vec![FieldError::new(path, "type", "expected integer")])?;
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
            FieldSpec::Number { minimum, maximum } => {
                let Ok(s) = std::str::from_utf8(bytes) else {
                    return Err(vec![FieldError::new(path, "type", "expected number")]);
                };
                let n = s
                    .parse::<f64>()
                    .map_err(|_| vec![FieldError::new(path, "type", "expected number")])?;
                if !n.is_finite() {
                    return Err(vec![FieldError::new(path, "type", "not a finite number")]);
                }
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
            FieldSpec::Boolean => match bytes {
                b"true" => Ok(Value::Bool(true)),
                b"false" => Ok(Value::Bool(false)),
                _ => Err(vec![FieldError::new(
                    path,
                    "type",
                    "expected boolean (true/false)",
                )]),
            },
            FieldSpec::String {
                format: Some(ref f),
                ..
            } if f == "uuid" => {
                if !is_uuid_bytes(bytes) {
                    return Err(vec![FieldError::new(
                        path,
                        "format",
                        "must be a valid uuid",
                    )]);
                }
                let s = std::str::from_utf8(bytes).map_err(|_| {
                    vec![FieldError::new(path, "type", "expected valid utf-8 string")]
                })?;
                Ok(Value::String(s.to_string()))
            }
            _ => {
                let s = std::str::from_utf8(bytes).map_err(|_| {
                    vec![FieldError::new(path, "type", "expected valid utf-8 string")]
                })?;
                self.decode_str(s, path, true)
            }
        }
    }

    /// Decode a single raw string value with source-appropriate coercion and bounds validation.
    pub fn decode_str(
        &self,
        raw_str: &str,
        path: &str,
        coerce_strings: bool,
    ) -> Result<Value, Vec<FieldError>> {
        match self {
            FieldSpec::String {
                min_length,
                max_length,
                pattern,
                format,
            } => {
                let s = raw_str;
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
                    if !simple_pattern_match(p, s) {
                        return Err(vec![FieldError::new(
                            path,
                            "pattern",
                            format!("must match {}", p),
                        )]);
                    }
                }
                if let Some(f) = format {
                    let ok = match f.as_str() {
                        "email" => is_email(s),
                        "uuid" => is_uuid(s),
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
                Ok(Value::String(s.to_string()))
            }
            FieldSpec::Integer { minimum, maximum } => {
                let n = if coerce_strings {
                    raw_str
                        .parse::<i64>()
                        .map_err(|_| vec![FieldError::new(path, "type", "expected integer")])?
                } else {
                    return Err(vec![FieldError::new(path, "type", "expected integer")]);
                };
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
            FieldSpec::Number { minimum, maximum } => {
                let n = if coerce_strings {
                    raw_str
                        .parse::<f64>()
                        .map_err(|_| vec![FieldError::new(path, "type", "expected number")])?
                } else {
                    return Err(vec![FieldError::new(path, "type", "expected number")]);
                };
                if !n.is_finite() {
                    return Err(vec![FieldError::new(path, "type", "not a finite number")]);
                }
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
            FieldSpec::Boolean => {
                if coerce_strings {
                    match raw_str {
                        "true" => Ok(Value::Bool(true)),
                        "false" => Ok(Value::Bool(false)),
                        _ => Err(vec![FieldError::new(
                            path,
                            "type",
                            "expected boolean (true/false)",
                        )]),
                    }
                } else {
                    Err(vec![FieldError::new(path, "type", "expected boolean")])
                }
            }
            FieldSpec::Literal { value } => match value {
                Value::String(s) if s == raw_str => Ok(value.clone()),
                Value::Number(n) if coerce_strings && n.to_string() == raw_str => Ok(value.clone()),
                Value::Bool(b) if coerce_strings && b.to_string() == raw_str => Ok(value.clone()),
                _ => Err(vec![FieldError::new(
                    path,
                    "literal",
                    format!("must equal {}", value),
                )]),
            },
            FieldSpec::Enum { values } => {
                for val in values {
                    match val {
                        Value::String(s) if s == raw_str => return Ok(val.clone()),
                        Value::Number(n) if coerce_strings && n.to_string() == raw_str => {
                            return Ok(val.clone())
                        }
                        Value::Bool(b) if coerce_strings && b.to_string() == raw_str => {
                            return Ok(val.clone())
                        }
                        _ => {}
                    }
                }
                Err(vec![FieldError::new(path, "enum", "value not in enum")])
            }
            FieldSpec::Optional { inner, default } => {
                if raw_str.is_empty() || raw_str == "null" {
                    Ok(default.clone().unwrap_or(Value::Null))
                } else {
                    inner.decode_str(raw_str, path, coerce_strings)
                }
            }
            FieldSpec::Nullable { inner } => {
                if raw_str == "null" {
                    Ok(Value::Null)
                } else {
                    inner.decode_str(raw_str, path, coerce_strings)
                }
            }
            FieldSpec::Array {
                items,
                min_items,
                max_items,
            } => {
                let parts: Vec<&str> = if raw_str.is_empty() {
                    Vec::new()
                } else {
                    raw_str.split(',').collect()
                };
                if let Some(min) = min_items {
                    if (parts.len() as u64) < *min {
                        return Err(vec![FieldError::new(
                            path,
                            "minItems",
                            format!("must have at least {} items", min),
                        )]);
                    }
                }
                if let Some(max) = max_items {
                    if (parts.len() as u64) > *max {
                        return Err(vec![FieldError::new(
                            path,
                            "maxItems",
                            format!("must have at most {} items", max),
                        )]);
                    }
                }
                let mut out = Vec::with_capacity(parts.len());
                for (i, part) in parts.iter().enumerate() {
                    let item_path = format!("{}[{}]", path, i);
                    out.push(items.decode_str(part, &item_path, coerce_strings)?);
                }
                Ok(Value::Array(out))
            }
            FieldSpec::Union { members } => {
                let mut last_err = Vec::new();
                for m in members {
                    match m.decode_str(raw_str, path, coerce_strings) {
                        Ok(v) => return Ok(v),
                        Err(e) => last_err = e,
                    }
                }
                Err(if last_err.is_empty() {
                    vec![FieldError::new(
                        path,
                        "union",
                        format!("value matched none of {} union members", members.len()),
                    )]
                } else {
                    last_err
                })
            }
            FieldSpec::Fallback { reason, inner } => {
                if !is_valid_fallback_reason(reason) {
                    return Err(vec![FieldError::new(
                        path,
                        "invalid-schema",
                        format!("unknown fallback reason {}", reason),
                    )]);
                }
                match inner {
                    Some(inner) => inner.decode_str(raw_str, path, coerce_strings),
                    None => Err(vec![FieldError::new(
                        path,
                        "fallback",
                        format!("fallback ({}) requires the generic codec path", reason),
                    )]),
                }
            }
            FieldSpec::Unsupported { .. } => Err(vec![FieldError::new(
                path,
                "unsupported",
                "schema node requires a specialized codec",
            )]),
        }
    }
}

/// A property specification inside a `DecoderProgram`.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDecoder {
    pub name: String,
    pub required: bool,
    pub spec: FieldSpec,
}

/// Direct decoder program compiled for an object schema.
#[derive(Debug, Clone, PartialEq)]
pub struct DecoderProgram {
    pub properties: BTreeMap<String, PropertyDecoder>,
    pub required: Vec<String>,
    pub allow_unknown: bool,
    pub source: Source,
}

impl DecoderProgram {
    /// Compile a `SchemaIr` into a direct decoder program.
    pub fn compile(ir: &SchemaIr, source: Source) -> Self {
        match ir {
            SchemaIr::Object {
                properties,
                required,
            } => {
                let allow_unknown = matches!(source, Source::Query);
                let decoders = properties
                    .iter()
                    .map(|(k, v)| {
                        let is_req = required.contains(k);
                        (
                            k.clone(),
                            PropertyDecoder {
                                name: k.clone(),
                                required: is_req,
                                spec: FieldSpec::compile(v),
                            },
                        )
                    })
                    .collect();
                DecoderProgram {
                    properties: decoders,
                    required: required.clone(),
                    allow_unknown,
                    source,
                }
            }
            SchemaIr::Fallback {
                inner: Some(inner), ..
            } => Self::compile(inner, source),
            _ => DecoderProgram {
                properties: BTreeMap::new(),
                required: Vec::new(),
                allow_unknown: true,
                source,
            },
        }
    }

    /// Fast byte-level pre-check and extraction for path parameter bytes.
    pub fn decode_params_bytes(&self, params: &[(&str, &[u8])]) -> ValidationResult {
        let mut errors: Vec<FieldError> = Vec::new();

        // 1. Unknown field checks (if not allow_unknown)
        if !self.allow_unknown {
            for (name, _) in params {
                if !self.properties.contains_key(*name) {
                    errors.push(FieldError::new(
                        &join_path("", name),
                        "additional",
                        "unknown field",
                    ));
                }
            }
        }

        // 2. Missing required field checks
        for req in &self.required {
            if !params.iter().any(|(n, _)| n == req) {
                errors.push(FieldError::new(
                    &join_path("", req),
                    "required",
                    "missing required field",
                ));
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        // 3. Direct decode per declared property
        let mut out = Map::new();
        for (key, prop) in &self.properties {
            if let Some((_, bytes)) = params.iter().find(|(n, _)| *n == key.as_str()) {
                match prop.spec.decode_bytes(bytes, &join_path("", key)) {
                    Ok(val) => {
                        out.insert(key.clone(), val);
                    }
                    Err(mut e) => {
                        errors.append(&mut e);
                    }
                }
            } else if let FieldSpec::Optional {
                default: Some(d), ..
            } = &prop.spec
            {
                out.insert(key.clone(), d.clone());
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Value::Object(out))
        }
    }

    /// Fast zero-copy decode directly from raw path bytes and (start, end) byte ranges.
    pub fn decode_params_ranges(
        &self,
        path_bytes: &[u8],
        param_names: &[&str],
        ranges: &[(u32, u32)],
    ) -> ValidationResult {
        let params: Vec<(&str, &[u8])> = param_names
            .iter()
            .zip(ranges)
            .filter_map(|(name, (start, end))| {
                let s = *start as usize;
                let e = *end as usize;
                if s <= e && e <= path_bytes.len() {
                    Some((*name, &path_bytes[s..e]))
                } else {
                    None
                }
            })
            .collect();
        self.decode_params_bytes(&params)
    }

    /// Direct decode for query key-value pairs (last-value-wins for repeated keys).
    pub fn decode_query_pairs(&self, query: &[(String, String)]) -> ValidationResult {
        let mut errors: Vec<FieldError> = Vec::new();

        // Repeated keys use last-value-wins policy
        let mut latest_query: BTreeMap<&str, &str> = BTreeMap::new();
        for (k, v) in query {
            latest_query.insert(k.as_str(), v.as_str());
        }

        // 1. Unknown field checks (if not allow_unknown)
        if !self.allow_unknown {
            for key in latest_query.keys() {
                if !self.properties.contains_key(*key) {
                    errors.push(FieldError::new(
                        &join_path("", key),
                        "additional",
                        "unknown field",
                    ));
                }
            }
        }

        // 2. Missing required field checks
        for req in &self.required {
            if !latest_query.contains_key(req.as_str()) {
                errors.push(FieldError::new(
                    &join_path("", req),
                    "required",
                    "missing required field",
                ));
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        // 3. Direct decode per declared property
        let mut out = Map::new();
        for (key, prop) in &self.properties {
            if let Some(raw_val) = latest_query.get(key.as_str()) {
                match prop.spec.decode_str(raw_val, &join_path("", key), true) {
                    Ok(val) => {
                        out.insert(key.clone(), val);
                    }
                    Err(mut e) => {
                        errors.append(&mut e);
                    }
                }
            } else if let FieldSpec::Optional {
                default: Some(d), ..
            } = &prop.spec
            {
                out.insert(key.clone(), d.clone());
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Value::Object(out))
        }
    }

    /// Direct decode for HTTP request headers (case-insensitive name lookup, unknown headers ignored).
    pub fn decode_headers(&self, headers: &[(&str, &str)]) -> ValidationResult {
        let mut errors: Vec<FieldError> = Vec::new();

        // Build lowercase lookup table for headers
        let mut lower_headers: BTreeMap<String, &str> = BTreeMap::new();
        for (k, v) in headers {
            lower_headers.insert(k.to_ascii_lowercase(), *v);
        }

        // 1. Missing required field checks
        for req in &self.required {
            let req_lower = req.to_ascii_lowercase();
            if !lower_headers.contains_key(&req_lower) {
                errors.push(FieldError::new(
                    &join_path("", req),
                    "required",
                    "missing required field",
                ));
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        // 2. Direct decode per declared property
        let mut out = Map::new();
        for (key, prop) in &self.properties {
            let key_lower = key.to_ascii_lowercase();
            if let Some(raw_val) = lower_headers.get(&key_lower) {
                match prop.spec.decode_str(raw_val, &join_path("", key), true) {
                    Ok(val) => {
                        out.insert(key.clone(), val);
                    }
                    Err(mut e) => {
                        errors.append(&mut e);
                    }
                }
            } else if let FieldSpec::Optional {
                default: Some(d), ..
            } = &prop.spec
            {
                out.insert(key.clone(), d.clone());
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Value::Object(out))
        }
    }
}

/// A dense table of direct decoder programs keyed by SchemaId index.
#[derive(Debug, Clone, Default)]
pub struct DecoderTable {
    programs: Vec<Option<DecoderProgram>>,
}

impl DecoderTable {
    /// Compile a table of direct decoders from a slice of SchemaIr definitions.
    pub fn from_schemas(schemas: &[SchemaIr]) -> Self {
        let programs = schemas
            .iter()
            .map(|ir| match ir {
                SchemaIr::Object { .. } | SchemaIr::Fallback { inner: Some(_), .. } => {
                    Some(DecoderProgram::compile(ir, Source::Path))
                }
                _ => None,
            })
            .collect();
        DecoderTable { programs }
    }

    /// Access the compiled decoder program for a schema id.
    pub fn get(&self, schema_id: u32) -> Option<&DecoderProgram> {
        self.programs
            .get(schema_id as usize)
            .and_then(|o| o.as_ref())
    }

    /// Decode path parameters using the decoder program keyed by `schema_id`.
    pub fn decode_params(&self, schema_id: u32, params: &[(&str, &[u8])]) -> ValidationResult {
        if let Some(decoder) = self.get(schema_id) {
            decoder.decode_params_bytes(params)
        } else {
            Err(vec![FieldError::new(
                "",
                "invalid-schema",
                format!("unknown schema id {schema_id}"),
            )])
        }
    }

    /// Decode path parameter ranges directly from path bytes using the decoder program keyed by `schema_id`.
    pub fn decode_params_ranges(
        &self,
        schema_id: u32,
        path_bytes: &[u8],
        param_names: &[&str],
        ranges: &[(u32, u32)],
    ) -> ValidationResult {
        if let Some(decoder) = self.get(schema_id) {
            decoder.decode_params_ranges(path_bytes, param_names, ranges)
        } else {
            Err(vec![FieldError::new(
                "",
                "invalid-schema",
                format!("unknown schema id {schema_id}"),
            )])
        }
    }

    /// Decode query parameters using the decoder program keyed by `schema_id`.
    pub fn decode_query(&self, schema_id: u32, query: &[(String, String)]) -> ValidationResult {
        if let Some(decoder) = self.get(schema_id) {
            decoder.decode_query_pairs(query)
        } else {
            Err(vec![FieldError::new(
                "",
                "invalid-schema",
                format!("unknown schema id {schema_id}"),
            )])
        }
    }

    /// Decode HTTP headers using the decoder program keyed by `schema_id`.
    pub fn decode_headers(&self, schema_id: u32, headers: &[(&str, &str)]) -> ValidationResult {
        if let Some(decoder) = self.get(schema_id) {
            decoder.decode_headers(headers)
        } else {
            Err(vec![FieldError::new(
                "",
                "invalid-schema",
                format!("unknown schema id {schema_id}"),
            )])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn test_param_schema() -> SchemaIr {
        SchemaIr::Object {
            properties: BTreeMap::from([
                (
                    "id".to_string(),
                    Box::new(SchemaIr::Integer {
                        minimum: Some(1),
                        maximum: Some(100),
                    }),
                ),
                (
                    "slug".to_string(),
                    Box::new(SchemaIr::String {
                        min_length: Some(2),
                        max_length: Some(20),
                        pattern: None,
                        format: None,
                    }),
                ),
                (
                    "tag".to_string(),
                    Box::new(SchemaIr::Optional {
                        inner: Box::new(SchemaIr::String {
                            min_length: None,
                            max_length: None,
                            pattern: None,
                            format: None,
                        }),
                        default: Some(json!("default-tag")),
                    }),
                ),
                (
                    "uuid".to_string(),
                    Box::new(SchemaIr::Optional {
                        inner: Box::new(SchemaIr::String {
                            min_length: None,
                            max_length: None,
                            pattern: None,
                            format: Some("uuid".to_string()),
                        }),
                        default: None,
                    }),
                ),
            ]),
            required: vec!["id".into(), "slug".into()],
        }
    }

    #[test]
    fn decoder_program_decodes_valid_params_bytes() {
        let ir = test_param_schema();
        let prog = DecoderProgram::compile(&ir, Source::Path);
        let params: Vec<(&str, &[u8])> = vec![
            ("id", b"42"),
            ("slug", b"my-post"),
            ("uuid", b"123e4567-e89b-12d3-a456-426614174000"),
        ];
        let res = prog.decode_params_bytes(&params).unwrap();
        assert_eq!(
            res,
            json!({
                "id": 42,
                "slug": "my-post",
                "tag": "default-tag",
                "uuid": "123e4567-e89b-12d3-a456-426614174000",
            })
        );
    }

    #[test]
    fn decoder_program_decodes_ranges_directly() {
        let ir = test_param_schema();
        let prog = DecoderProgram::compile(&ir, Source::Path);
        let path = b"/posts/99/rust-guide";
        let names = vec!["id", "slug"];
        let ranges = vec![(7u32, 9u32), (10u32, 20u32)];
        let res = prog.decode_params_ranges(path, &names, &ranges).unwrap();
        assert_eq!(
            res,
            json!({
                "id": 99,
                "slug": "rust-guide",
                "tag": "default-tag",
            })
        );
    }

    #[test]
    fn decoder_program_rejects_invalid_param_fields() {
        let ir = test_param_schema();
        let prog = DecoderProgram::compile(&ir, Source::Path);

        // missing required
        let res = prog.decode_params_bytes(&[("slug", b"post")]);
        assert_eq!(
            res,
            Err(vec![FieldError::new(
                "id",
                "required",
                "missing required field"
            )])
        );

        // unknown field
        let res = prog.decode_params_bytes(&[("id", b"10"), ("slug", b"post"), ("extra", b"x")]);
        assert_eq!(
            res,
            Err(vec![FieldError::new(
                "extra",
                "additional",
                "unknown field"
            )])
        );

        // bounds failure
        let res = prog.decode_params_bytes(&[("id", b"0"), ("slug", b"post")]);
        assert_eq!(
            res,
            Err(vec![FieldError::new("id", "minimum", "must be at least 1")])
        );

        // format failure
        let res =
            prog.decode_params_bytes(&[("id", b"10"), ("slug", b"post"), ("uuid", b"bad-uuid")]);
        assert_eq!(
            res,
            Err(vec![FieldError::new(
                "uuid",
                "format",
                "must be a valid uuid"
            )])
        );
    }

    #[test]
    fn decoder_program_decodes_query_with_last_value_wins() {
        let ir = test_param_schema();
        let prog = DecoderProgram::compile(&ir, Source::Query);
        let query = vec![
            ("id".to_string(), "10".to_string()),
            ("id".to_string(), "20".to_string()), // last value wins -> 20
            ("slug".to_string(), "alpha".to_string()),
            ("unknown_ignored".to_string(), "skip_me".to_string()),
        ];
        let res = prog.decode_query_pairs(&query).unwrap();
        assert_eq!(
            res,
            json!({
                "id": 20,
                "slug": "alpha",
                "tag": "default-tag",
            })
        );
    }

    #[test]
    fn decoder_program_decodes_headers_case_insensitively() {
        let header_schema = SchemaIr::Object {
            properties: BTreeMap::from([
                (
                    "x-api-key".to_string(),
                    Box::new(SchemaIr::String {
                        min_length: Some(4),
                        max_length: None,
                        pattern: None,
                        format: None,
                    }),
                ),
                (
                    "x-rate-limit".to_string(),
                    Box::new(SchemaIr::Optional {
                        inner: Box::new(SchemaIr::Integer {
                            minimum: Some(0),
                            maximum: None,
                        }),
                        default: Some(json!(100)),
                    }),
                ),
            ]),
            required: vec!["x-api-key".into()],
        };

        let prog = DecoderProgram::compile(&header_schema, Source::Query);

        // Header names arrived with mixed casing
        let headers = vec![
            ("X-Api-Key", "secret-key-123"),
            ("User-Agent", "Mozilla"), // unknown header ignored
        ];
        let res = prog.decode_headers(&headers).unwrap();
        assert_eq!(
            res,
            json!({
                "x-api-key": "secret-key-123",
                "x-rate-limit": 100,
            })
        );

        // Missing required header
        let res_err = prog.decode_headers(&[("X-Other", "value")]);
        assert_eq!(
            res_err,
            Err(vec![FieldError::new(
                "x-api-key",
                "required",
                "missing required field"
            )])
        );
    }

    #[test]
    fn decoder_table_indexes_and_dispatches_by_schema_id() {
        let ir1 = test_param_schema();
        let ir2 = SchemaIr::Object {
            properties: BTreeMap::from([(
                "count".to_string(),
                Box::new(SchemaIr::Integer {
                    minimum: Some(0),
                    maximum: None,
                }),
            )]),
            required: vec!["count".into()],
        };
        let table = DecoderTable::from_schemas(&[ir1, ir2]);

        // Schema 0: test_param_schema
        let res0 = table
            .decode_params(0, &[("id", b"5"), ("slug", b"abc")])
            .unwrap();
        assert_eq!(res0, json!({"id": 5, "slug": "abc", "tag": "default-tag"}));

        // Schema 1: count schema
        let res1 = table.decode_params(1, &[("count", b"99")]).unwrap();
        assert_eq!(res1, json!({"count": 99}));

        // Schema 2: Out of bounds -> error
        let res2 = table.decode_params(2, &[]);
        assert!(res2.is_err());
    }

    #[test]
    fn decoder_program_malformed_byte_ranges_rejects_cleanly() {
        let ir = test_param_schema();
        let prog = DecoderProgram::compile(&ir, Source::Path);

        // Inverted ranges or out of bounds range returns missing required field error
        let path = b"/posts/123/rust";
        let names = vec!["id", "slug"];
        let inverted_ranges = vec![(10u32, 5u32), (200u32, 300u32)];
        let res = prog.decode_params_ranges(path, &names, &inverted_ranges);
        assert!(res.is_err());

        // Non-UTF8 byte slice returns type error
        let bad_utf8: &[u8] = &[0xff, 0xfe, 0xfd];
        let res_utf8 = prog.decode_params_bytes(&[("id", b"1"), ("slug", bad_utf8)]);
        assert_eq!(
            res_utf8,
            Err(vec![FieldError::new(
                "slug",
                "type",
                "expected valid utf-8 string"
            )])
        );
    }

    #[test]
    fn decoder_program_query_arrays_comma_separated() {
        let ir = SchemaIr::Object {
            properties: BTreeMap::from([
                (
                    "tags".to_string(),
                    Box::new(SchemaIr::Array {
                        items: Box::new(SchemaIr::String {
                            min_length: Some(1),
                            max_length: Some(10),
                            pattern: None,
                            format: None,
                        }),
                        min_items: Some(1),
                        max_items: Some(5),
                    }),
                ),
                (
                    "ids".to_string(),
                    Box::new(SchemaIr::Array {
                        items: Box::new(SchemaIr::Integer {
                            minimum: Some(0),
                            maximum: Some(100),
                        }),
                        min_items: None,
                        max_items: None,
                    }),
                ),
            ]),
            required: vec!["tags".into()],
        };

        let prog = DecoderProgram::compile(&ir, Source::Query);
        let query = vec![
            ("tags".to_string(), "rust,wasm,fast".to_string()),
            ("ids".to_string(), "1,2,3,4".to_string()),
        ];
        let res = prog.decode_query_pairs(&query).unwrap();
        assert_eq!(
            res,
            json!({
                "tags": ["rust", "wasm", "fast"],
                "ids": [1, 2, 3, 4],
            })
        );
    }

    #[test]
    fn decoder_program_matches_reference_validator_on_mixed_corpus() {
        let ir = test_param_schema();
        let prog = DecoderProgram::compile(&ir, Source::Query);

        let cases = vec![
            vec![
                ("id".to_string(), "5".to_string()),
                ("slug".to_string(), "abc".to_string()),
            ],
            vec![
                ("id".to_string(), "99".to_string()),
                ("slug".to_string(), "valid-slug".to_string()),
                ("tag".to_string(), "custom".to_string()),
            ],
            vec![
                ("id".to_string(), "0".to_string()),
                ("slug".to_string(), "abc".to_string()),
            ], // invalid min
            vec![("id".to_string(), "5".to_string())], // missing required slug
            vec![
                ("id".to_string(), "abc".to_string()),
                ("slug".to_string(), "def".to_string()),
            ], // invalid type
        ];

        for query in cases {
            let res_prog = prog.decode_query_pairs(&query);
            let res_ref = crate::validate_query(&ir, &query);
            assert_eq!(res_prog.is_ok(), res_ref.is_ok(), "query: {query:?}");
            if let (Err(e1), Err(e2)) = (&res_prog, &res_ref) {
                assert_eq!(e1.len(), e2.len(), "error counts match");
                assert_eq!(e1[0].code, e2[0].code, "error code match: {e1:?} vs {e2:?}");
            }
        }
    }
}
