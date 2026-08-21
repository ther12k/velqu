//! Direct decoder programs keyed by SchemaId (M25-003-A).
//!
//! Compiles an object SchemaIr into a specialized, direct field decoder program
//! that fuses field extraction, string/byte coercion, bounds validation, and
//! default application into a single pass without intermediate generic AST trees.

use std::collections::BTreeMap;

use serde_json::{Map, Number, Value};

use crate::{
    is_email, is_uuid, is_valid_fallback_reason, join_path, simple_pattern_match, FieldError,
    SchemaIr, Source, ValidationResult,
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

    /// Decode a single raw string value (e.g. from params, query, or headers)
    /// with source-appropriate coercion and bounds validation.
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
                let s = match std::str::from_utf8(bytes) {
                    Ok(s) => s,
                    Err(_) => {
                        errors.push(FieldError::new(
                            &join_path("", key),
                            "type",
                            "expected valid utf-8 string",
                        ));
                        continue;
                    }
                };
                match prop.spec.decode_str(s, &join_path("", key), true) {
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
}
