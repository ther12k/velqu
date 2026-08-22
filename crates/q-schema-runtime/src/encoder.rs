//! Generated per-status response encoders keyed by SchemaId (M25-005-A).
//!
//! Compiles a representable object SchemaIr into a direct encoder program
//! that fuses response validation and JSON serialization into ONE traversal:
//! typed field errors surface exactly where the reference validator produces
//! them, and JSON bytes are emitted in declared property order with no
//! intermediate `Value` allocation and no second serialization pass.
//!
//! Schemas the direct encoder cannot represent (nested object properties,
//! unions, transforms, files, problems, fallback markers without inner)
//! compile to `None`; the runtime keeps the reference
//! validate-then-serialize path for those routes instead of failing closed —
//! an unrepresentable contract is not a contract violation.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::{
    is_email, is_uuid, join_path, simple_pattern_match, FieldError, FieldErrorCode, FieldSpec,
    SchemaIr, MAX_VALIDATE_DEPTH,
};

/// One declared property, pre-resolved for the fixed-order read pass.
#[derive(Debug, Clone, PartialEq)]
struct PropertyEncoder {
    name: String,
    spec: FieldSpec,
    /// Hoisted `Optional` default: emitted verbatim when the property is
    /// absent (the reference validator inserts it unvalidated).
    default: Option<Value>,
}

/// A compiled direct encoder program for one declared response schema.
#[derive(Debug, Clone, PartialEq)]
pub struct EncoderProgram {
    /// Fixed declared property order, byte-sorted at compile time
    /// (M25-005-B): every response reads exactly these properties in
    /// exactly this order, regardless of the handler value's key order.
    properties: Vec<PropertyEncoder>,
    /// Required names in schema declaration order — the missing-required
    /// pass iterates this list so typed-error ordering matches the
    /// reference validator exactly.
    required: Vec<String>,
}

/// True when every leaf this spec can reach emits via the direct encoder.
/// Nested objects, unions, transforms, files, problems, and fallback markers
/// without inner keep the reference path (`M25-005-C` refines the latter).
fn encodable(spec: &FieldSpec) -> bool {
    match spec {
        FieldSpec::String { .. }
        | FieldSpec::Integer { .. }
        | FieldSpec::Number { .. }
        | FieldSpec::Boolean
        | FieldSpec::Literal { .. }
        | FieldSpec::Enum { .. } => true,
        FieldSpec::Optional { inner, .. } | FieldSpec::Nullable { inner } => encodable(inner),
        FieldSpec::Array { items, .. } => encodable(items),
        FieldSpec::Fallback {
            inner: Some(inner), ..
        } => encodable(inner),
        FieldSpec::Fallback { inner: None, .. } | FieldSpec::Unsupported { .. } => false,
        // M25-005-C: unions encode via scratch-buffer retry; every member
        // must be encodable or the whole program keeps the reference path
        FieldSpec::Union { members } => !members.is_empty() && members.iter().all(encodable),
    }
}

impl EncoderProgram {
    /// Compile an object SchemaIr into a direct encoder program.
    /// Returns `None` when any declared property is not directly encodable.
    pub fn compile(ir: &SchemaIr) -> Option<Self> {
        let SchemaIr::Object {
            properties,
            required,
        } = ir
        else {
            // string-kind and other non-object responses keep the
            // reference validate-then-serialize path
            return None;
        };
        let mut specs = BTreeMap::new();
        for (key, node) in properties {
            let spec = FieldSpec::compile(node);
            if !encodable(&spec) {
                return None;
            }
            specs.insert(key.clone(), spec);
        }
        // fixed read order: properties arrive byte-sorted from the BTreeMap
        // and are frozen into the program as a plain vector — no runtime
        // map iteration, no per-property required-list scan
        let fixed: Vec<PropertyEncoder> = specs
            .into_iter()
            .map(|(name, spec)| {
                let default = match &spec {
                    FieldSpec::Optional {
                        default: Some(d), ..
                    } => Some(d.clone()),
                    _ => None,
                };
                PropertyEncoder {
                    name,
                    spec,
                    default,
                }
            })
            .collect();
        Some(EncoderProgram {
            properties: fixed,
            required: required.clone(),
        })
    }

    /// One traversal: validate `value` against the declared contract and
    /// append the canonical JSON bytes to `out`. On error `out` may hold
    /// partial output the caller must discard. The emitted bytes equal
    /// `serde_json::to_vec` of the reference validator's normalized output.
    pub fn encode(&self, value: &Value, out: &mut Vec<u8>) -> Result<(), Vec<FieldError>> {
        self.encode_object(value, "", 0, out)
    }

    fn encode_object(
        &self,
        value: &Value,
        path: &str,
        depth: usize,
        out: &mut Vec<u8>,
    ) -> Result<(), Vec<FieldError>> {
        if depth > MAX_VALIDATE_DEPTH {
            return Err(vec![FieldError::typed(
                path,
                FieldErrorCode::Depth,
                format!("maximum nesting depth {} exceeded", MAX_VALIDATE_DEPTH),
            )]);
        }
        let obj = match value.as_object() {
            Some(o) => o,
            None => {
                return Err(vec![FieldError::typed(
                    path,
                    FieldErrorCode::Type,
                    "expected object",
                )])
            }
        };
        let mut errors = Vec::new();
        for key in obj.keys() {
            // self.properties is byte-sorted — binary search, no map
            if self
                .properties
                .binary_search_by(|probe| probe.name.as_str().cmp(key.as_str()))
                .is_err()
            {
                errors.push(FieldError::typed(
                    &join_path(path, key),
                    FieldErrorCode::Additional,
                    "unknown field",
                ));
            }
        }
        for req in &self.required {
            if !obj.contains_key(req) {
                errors.push(FieldError::typed(
                    &join_path(path, req),
                    FieldErrorCode::Required,
                    "missing required field",
                ));
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }

        // The read pass walks the frozen declared order (byte-sorted,
        // matching the reference validator's normalized output insertion
        // order). Absent optional-without-default keys are omitted; absent
        // optional-with-default keys emit the default.
        out.push(b'{');
        let mut first = true;
        for prop in &self.properties {
            let (v, is_default) = match obj.get(prop.name.as_str()) {
                Some(v) => (v, false),
                None => match &prop.default {
                    Some(d) => (d, true),
                    None => continue,
                },
            };
            if !first {
                out.push(b',');
            }
            first = false;
            // leaf byte parity: delegate key and scalar writing to
            // serde_json itself so escaping/number formatting can never
            // drift from the reference serialization
            let _ = serde_json::to_writer(&mut *out, prop.name.as_str());
            out.push(b':');
            let p = join_path(path, &prop.name);
            let walked = if is_default {
                // defaults are schema-declared literals: emitted verbatim
                // (reference inserts the default unvalidated)
                let _ = serde_json::to_writer(&mut *out, v);
                Ok(())
            } else {
                encode_spec(&prop.spec, v, &p, depth + 1, out)
            };
            if let Err(mut e) = walked {
                errors.append(&mut e);
            }
        }
        out.push(b'}');
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Encode a value against a field spec, emitting JSON bytes (strict
/// Source::Body semantics — no string coercion).
fn encode_spec(
    spec: &FieldSpec,
    value: &Value,
    path: &str,
    depth: usize,
    out: &mut Vec<u8>,
) -> Result<(), Vec<FieldError>> {
    if depth > MAX_VALIDATE_DEPTH {
        return Err(vec![FieldError::typed(
            path,
            FieldErrorCode::Depth,
            format!("maximum nesting depth {} exceeded", MAX_VALIDATE_DEPTH),
        )]);
    }
    match spec {
        FieldSpec::Optional { inner, default } => {
            if value.is_null() {
                // reference: null on optional resolves to the declared
                // default, else null
                match default {
                    Some(d) => {
                        let _ = serde_json::to_writer(&mut *out, d);
                    }
                    None => out.extend_from_slice(b"null"),
                }
                Ok(())
            } else {
                encode_spec(inner, value, path, depth + 1, out)
            }
        }
        FieldSpec::Nullable { inner } => {
            if value.is_null() {
                out.extend_from_slice(b"null");
                Ok(())
            } else {
                encode_spec(inner, value, path, depth + 1, out)
            }
        }
        FieldSpec::String {
            min_length,
            max_length,
            pattern,
            format,
        } => {
            let s = match value.as_str() {
                Some(s) => s,
                None => {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::Type,
                        "expected string",
                    )])
                }
            };
            if let Some(min) = min_length {
                if (s.len() as u64) < *min {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::MinLength,
                        format!("must be at least {} characters", min),
                    )]);
                }
            }
            if let Some(max) = max_length {
                if (s.len() as u64) > *max {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::MaxLength,
                        format!("must be at most {} characters", max),
                    )]);
                }
            }
            if let Some(p) = pattern {
                if !simple_pattern_match(p, s) {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::Pattern,
                        format!("must match {}", p),
                    )]);
                }
            }
            if let Some(f) = format {
                let ok = match f.as_str() {
                    "email" => is_email(s),
                    "uuid" => is_uuid(s),
                    other => {
                        return Err(vec![FieldError::typed(
                            path,
                            FieldErrorCode::Format,
                            format!("unknown format {}", other),
                        )])
                    }
                };
                if !ok {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::Format,
                        format!("must be a valid {}", f),
                    )]);
                }
            }
            let _ = serde_json::to_writer(&mut *out, s);
            Ok(())
        }
        FieldSpec::Integer { minimum, maximum } => {
            let n = match value.as_i64() {
                Some(n) => n,
                None => {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::Type,
                        "expected integer",
                    )])
                }
            };
            if let Some(min) = minimum {
                if n < *min {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::Minimum,
                        format!("must be at least {}", min),
                    )]);
                }
            }
            if let Some(max) = maximum {
                if n > *max {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::Maximum,
                        format!("must be at most {}", max),
                    )]);
                }
            }
            let _ = serde_json::to_writer(&mut *out, &n);
            Ok(())
        }
        FieldSpec::Number { minimum, maximum } => {
            let n = match value.as_f64() {
                Some(n) if n.is_finite() => n,
                Some(_) => {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::Type,
                        "not a finite number",
                    )])
                }
                None => {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::Type,
                        "expected number",
                    )])
                }
            };
            if let Some(min) = minimum {
                if n < *min {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::Minimum,
                        format!("must be at least {}", min),
                    )]);
                }
            }
            if let Some(max) = maximum {
                if n > *max {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::Maximum,
                        format!("must be at most {}", max),
                    )]);
                }
            }
            // reference normalizes through Number::from_f64 (integer inputs
            // become float form, e.g. 3 -> 3.0) — mirror it for byte parity
            let normalized = serde_json::Number::from_f64(n).ok_or_else(|| {
                vec![FieldError::typed(
                    path,
                    FieldErrorCode::Type,
                    "not a finite number",
                )]
            })?;
            let _ = serde_json::to_writer(&mut *out, &normalized);
            Ok(())
        }
        FieldSpec::Boolean => match value.as_bool() {
            Some(b) => {
                let _ = serde_json::to_writer(&mut *out, &b);
                Ok(())
            }
            None => Err(vec![FieldError::typed(
                path,
                FieldErrorCode::Type,
                "expected boolean",
            )]),
        },
        FieldSpec::Literal { value: lit } => {
            if value == lit {
                let _ = serde_json::to_writer(&mut *out, lit);
                Ok(())
            } else {
                Err(vec![FieldError::typed(
                    path,
                    FieldErrorCode::Literal,
                    format!("must equal {}", lit),
                )])
            }
        }
        FieldSpec::Enum { values } => {
            if values.contains(value) {
                let _ = serde_json::to_writer(&mut *out, value);
                Ok(())
            } else {
                Err(vec![FieldError::typed(
                    path,
                    FieldErrorCode::Enum,
                    "value not in enum",
                )])
            }
        }
        FieldSpec::Array {
            items,
            min_items,
            max_items,
        } => {
            let arr = match value.as_array() {
                Some(a) => a,
                None => {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::Type,
                        "expected array",
                    )])
                }
            };
            if let Some(min) = min_items {
                if (arr.len() as u64) < *min {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::MinItems,
                        format!("must have at least {} items", min),
                    )]);
                }
            }
            if let Some(max) = max_items {
                if (arr.len() as u64) > *max {
                    return Err(vec![FieldError::typed(
                        path,
                        FieldErrorCode::MaxItems,
                        format!("must have at most {} items", max),
                    )]);
                }
            }
            out.push(b'[');
            let mut errors = Vec::new();
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                let p = format!("{}[{}]", path, i);
                if let Err(mut e) = encode_spec(items, item, &p, depth + 1, out) {
                    errors.append(&mut e);
                }
            }
            out.push(b']');
            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors)
            }
        }
        FieldSpec::Union { members } => {
            // M25-005-C: first-match-wins, mirroring the reference
            // validator's member order. Each attempt encodes into a
            // scratch buffer so a failed member's partial bytes never
            // reach the response; only the winning member's bytes append.
            for member in members {
                let mut scratch = Vec::new();
                if encode_spec(member, value, path, depth + 1, &mut scratch).is_ok() {
                    out.extend_from_slice(&scratch);
                    return Ok(());
                }
            }
            Err(vec![FieldError::typed(
                path,
                FieldErrorCode::Union,
                format!("value matched none of {} union members", members.len()),
            )])
        }
        // compile() rejects these; reachable only through a programming error
        FieldSpec::Fallback { .. } | FieldSpec::Unsupported { .. } => Err(vec![FieldError::typed(
            path,
            FieldErrorCode::Unsupported,
            "schema node requires a specialized codec",
        )]),
    }
}

/// A dense table of direct encoder programs keyed by SchemaId index.
#[derive(Debug, Clone, Default)]
pub struct EncoderTable {
    programs: Vec<Option<EncoderProgram>>,
}

impl EncoderTable {
    /// Compile a table of direct encoders from a slice of SchemaIr
    /// definitions (same dense SchemaId ordering as the decoder table).
    pub fn from_schemas(schemas: &[SchemaIr]) -> Self {
        let programs = schemas.iter().map(EncoderProgram::compile).collect();
        EncoderTable { programs }
    }

    /// Access the compiled encoder program for a schema id. `None` means
    /// the schema is not directly encodable — keep the reference path.
    pub fn get(&self, schema_id: u32) -> Option<&EncoderProgram> {
        self.programs
            .get(schema_id as usize)
            .and_then(|o| o.as_ref())
    }
}

#[cfg(test)]
mod m25_005_a_tests {
    use super::*;
    use crate::{validate, Source};
    use serde_json::json;
    use std::collections::BTreeMap;

    fn obj(props: Vec<(&str, SchemaIr)>, required: Vec<&str>) -> SchemaIr {
        SchemaIr::Object {
            properties: props
                .into_iter()
                .map(|(k, v)| (k.to_string(), Box::new(v)))
                .collect::<BTreeMap<_, _>>(),
            required: required.into_iter().map(String::from).collect(),
        }
    }

    /// Golden corpus: valid values must encode to EXACTLY the reference
    /// validator's normalized serialization (byte-for-byte), including
    /// values whose keys arrive out of declared order.
    #[test]
    fn encoder_matches_reference_serialization_on_golden_corpus() {
        let corpus: Vec<(SchemaIr, Value)> = vec![
            // flat scalars, handler key order reversed from schema order
            (
                obj(
                    vec![
                        (
                            "id",
                            SchemaIr::String {
                                min_length: Some(1),
                                max_length: None,
                                pattern: Some("^usr_[0-9]+$".into()),
                                format: None,
                            },
                        ),
                        (
                            "age",
                            SchemaIr::Integer {
                                minimum: Some(0),
                                maximum: Some(150),
                            },
                        ),
                        (
                            "score",
                            SchemaIr::Number {
                                minimum: None,
                                maximum: Some(100.0),
                            },
                        ),
                        ("active", SchemaIr::Boolean),
                    ],
                    vec!["id", "age", "score", "active"],
                ),
                json!({"active": true, "score": 2.5, "age": 30, "id": "usr_1"}),
            ),
            // optional present / absent-with-default / absent-without-default
            (
                obj(
                    vec![
                        (
                            "name",
                            SchemaIr::String {
                                min_length: None,
                                max_length: None,
                                pattern: None,
                                format: None,
                            },
                        ),
                        (
                            "nick",
                            SchemaIr::Optional {
                                inner: Box::new(SchemaIr::String {
                                    min_length: None,
                                    max_length: None,
                                    pattern: None,
                                    format: None,
                                }),
                                default: Some(json!("anon")),
                            },
                        ),
                        (
                            "phone",
                            SchemaIr::Optional {
                                inner: Box::new(SchemaIr::String {
                                    min_length: None,
                                    max_length: None,
                                    pattern: None,
                                    format: None,
                                }),
                                default: None,
                            },
                        ),
                    ],
                    vec!["name"],
                ),
                json!({"name": "Ada"}),
            ),
            // optional present-null resolves to the declared default
            (
                obj(
                    vec![
                        (
                            "name",
                            SchemaIr::String {
                                min_length: None,
                                max_length: None,
                                pattern: None,
                                format: None,
                            },
                        ),
                        (
                            "nick",
                            SchemaIr::Optional {
                                inner: Box::new(SchemaIr::String {
                                    min_length: None,
                                    max_length: None,
                                    pattern: None,
                                    format: None,
                                }),
                                default: Some(json!("anon")),
                            },
                        ),
                    ],
                    vec!["name"],
                ),
                json!({"name": "Ada", "nick": null}),
            ),
            // nullable present-null and present-value
            (
                obj(
                    vec![
                        (
                            "a",
                            SchemaIr::Nullable {
                                inner: Box::new(SchemaIr::Integer {
                                    minimum: None,
                                    maximum: None,
                                }),
                            },
                        ),
                        (
                            "b",
                            SchemaIr::Nullable {
                                inner: Box::new(SchemaIr::Integer {
                                    minimum: None,
                                    maximum: None,
                                }),
                            },
                        ),
                    ],
                    vec!["a", "b"],
                ),
                json!({"a": null, "b": 7}),
            ),
            // arrays with bounds, nested arrays of scalars
            (
                obj(
                    vec![
                        (
                            "tags",
                            SchemaIr::Array {
                                items: Box::new(SchemaIr::String {
                                    min_length: None,
                                    max_length: None,
                                    pattern: None,
                                    format: None,
                                }),
                                min_items: Some(1),
                                max_items: Some(3),
                            },
                        ),
                        (
                            "matrix",
                            SchemaIr::Array {
                                items: Box::new(SchemaIr::Array {
                                    items: Box::new(SchemaIr::Integer {
                                        minimum: None,
                                        maximum: None,
                                    }),
                                    min_items: None,
                                    max_items: None,
                                }),
                                min_items: None,
                                max_items: None,
                            },
                        ),
                    ],
                    vec!["tags", "matrix"],
                ),
                json!({"tags": ["x", "y"], "matrix": [[1, 2], [3]]}),
            ),
            // literals and enums
            (
                obj(
                    vec![
                        (
                            "kind",
                            SchemaIr::Literal {
                                value: json!("user"),
                            },
                        ),
                        (
                            "role",
                            SchemaIr::Enum {
                                values: vec![json!("admin"), json!("member")],
                            },
                        ),
                    ],
                    vec!["kind", "role"],
                ),
                json!({"kind": "user", "role": "admin"}),
            ),
            // formats: email + uuid
            (
                obj(
                    vec![
                        (
                            "email",
                            SchemaIr::String {
                                min_length: None,
                                max_length: None,
                                pattern: None,
                                format: Some("email".into()),
                            },
                        ),
                        (
                            "uuid",
                            SchemaIr::String {
                                min_length: None,
                                max_length: None,
                                pattern: None,
                                format: Some("uuid".into()),
                            },
                        ),
                    ],
                    vec!["email", "uuid"],
                ),
                json!({"email": "ada@example.org", "uuid": "123e4567-e89b-12d3-a456-426614174000"}),
            ),
            // unicode + escapes round-trip byte-identically
            (
                obj(
                    vec![(
                        "note",
                        SchemaIr::String {
                            min_length: None,
                            max_length: None,
                            pattern: None,
                            format: None,
                        },
                    )],
                    vec!["note"],
                ),
                json!({"note": "Ada \"The Ada\" ☃\n\tbar\\baz"}),
            ),
            // integer input under a Number schema normalizes to float form
            (
                obj(
                    vec![(
                        "n",
                        SchemaIr::Number {
                            minimum: None,
                            maximum: None,
                        },
                    )],
                    vec!["n"],
                ),
                json!({"n": 3}),
            ),
        ];

        for (ir, value) in corpus {
            let reference = validate(&ir, &value, Source::Body)
                .unwrap_or_else(|e| panic!("corpus value must be valid: {:?} ({:?})", value, e));
            let expected = serde_json::to_vec(&reference).unwrap();
            let program = EncoderProgram::compile(&ir)
                .unwrap_or_else(|| panic!("corpus schema must be encodable: {:?}", ir));
            let mut out = Vec::new();
            program
                .encode(&value, &mut out)
                .unwrap_or_else(|e| panic!("encode must succeed: {:?} ({:?})", value, e));
            assert_eq!(
                out, expected,
                "encoder bytes must equal reference serialization for {:?}",
                value
            );
        }
    }

    /// Response mismatch evidence: every invalid value must produce the
    /// SAME typed code+path as the reference validator.
    #[test]
    fn encoder_rejects_mismatches_with_reference_parity() {
        let ir = obj(
            vec![
                (
                    "id",
                    SchemaIr::String {
                        min_length: Some(1),
                        max_length: None,
                        pattern: Some("^usr_[0-9]+$".into()),
                        format: None,
                    },
                ),
                (
                    "age",
                    SchemaIr::Integer {
                        minimum: Some(0),
                        maximum: Some(150),
                    },
                ),
                (
                    "email",
                    SchemaIr::String {
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        format: Some("email".into()),
                    },
                ),
                (
                    "kind",
                    SchemaIr::Literal {
                        value: json!("user"),
                    },
                ),
                (
                    "role",
                    SchemaIr::Enum {
                        values: vec![json!("admin")],
                    },
                ),
                (
                    "tags",
                    SchemaIr::Array {
                        items: Box::new(SchemaIr::String {
                            min_length: None,
                            max_length: None,
                            pattern: None,
                            format: None,
                        }),
                        min_items: Some(1),
                        max_items: None,
                    },
                ),
            ],
            vec!["id", "age", "email", "kind", "role", "tags"],
        );
        let program = EncoderProgram::compile(&ir).unwrap();

        let mismatches: Vec<Value> = vec![
            json!({"id": "usr_1", "age": 30, "email": "a@b.c", "kind": "user", "role": "admin", "tags": ["t"], "extra": 1}),
            json!({"age": 30, "email": "a@b.c", "kind": "user", "role": "admin", "tags": ["t"]}),
            json!({"id": 7, "age": 30, "email": "a@b.c", "kind": "user", "role": "admin", "tags": ["t"]}),
            json!({"id": "usr_1", "age": -1, "email": "a@b.c", "kind": "user", "role": "admin", "tags": ["t"]}),
            json!({"id": "usr_1", "age": 30, "email": "not-an-email", "kind": "user", "role": "admin", "tags": ["t"]}),
            json!({"id": "usr_1", "age": 30, "email": "a@b.c", "kind": "admin", "role": "admin", "tags": ["t"]}),
            json!({"id": "usr_1", "age": 30, "email": "a@b.c", "kind": "user", "role": "nope", "tags": ["t"]}),
            json!({"id": "usr_1", "age": 30, "email": "a@b.c", "kind": "user", "role": "admin", "tags": []}),
            json!({"id": "usr_1", "age": 30, "email": "a@b.c", "kind": "user", "role": "admin", "tags": [1]}),
            json!("not an object"),
        ];

        for value in mismatches {
            let ref_err = validate(&ir, &value, Source::Body).expect_err("reference must reject");
            let mut out = Vec::new();
            let enc_err = program
                .encode(&value, &mut out)
                .expect_err("encoder must reject");
            let pairs = |errs: &Vec<FieldError>| -> Vec<(String, String)> {
                errs.iter()
                    .map(|e| (e.path.clone(), e.code.clone()))
                    .collect()
            };
            assert_eq!(
                pairs(&enc_err),
                pairs(&ref_err),
                "typed mismatch parity failed for {:?}",
                value
            );
        }
    }

    /// Mapping deadline evidence: encode recursion is bounded by
    /// MAX_VALIDATE_DEPTH with a typed depth problem — no unbounded stack
    /// work during response conversion.
    #[test]
    fn encoder_depth_is_bounded() {
        // a schema nested past the bound (mirrors the M25-004-C decoder
        // test): both the schema and the value carry the same depth
        let levels = MAX_VALIDATE_DEPTH + 8;
        let int = SchemaIr::Integer {
            minimum: None,
            maximum: None,
        };
        let mut deep = int.clone();
        let mut value = json!(1);
        for _ in 0..levels {
            deep = SchemaIr::Array {
                items: Box::new(deep),
                min_items: None,
                max_items: None,
            };
            value = json!([value]);
        }
        let ir = obj(vec![("deep", deep)], vec!["deep"]);
        let program = EncoderProgram::compile(&ir).unwrap();
        let body = json!({ "deep": value });

        let mut out = Vec::new();
        let err = program
            .encode(&body, &mut out)
            .expect_err("deep must reject");
        assert_eq!(err[0].code, "depth");

        // the reference validator agrees (parity on the bound)
        let ref_err = validate(&ir, &body, Source::Body).expect_err("reference must reject");
        assert_eq!(ref_err[0].code, "depth");
    }

    /// Schemas the direct encoder cannot represent compile to None — the
    /// runtime keeps the reference validate-then-serialize path for them.
    #[test]
    fn unrepresentable_schemas_compile_to_none() {
        let nested_object = obj(
            vec![(
                "inner",
                obj(
                    vec![(
                        "x",
                        SchemaIr::Integer {
                            minimum: None,
                            maximum: None,
                        },
                    )],
                    vec!["x"],
                ),
            )],
            vec!["inner"],
        );
        assert!(EncoderProgram::compile(&nested_object).is_none());

        let union_member = obj(
            vec![(
                "u",
                SchemaIr::Union {
                    members: vec![
                        Box::new(SchemaIr::Integer {
                            minimum: None,
                            maximum: None,
                        }),
                        Box::new(SchemaIr::Boolean),
                    ],
                },
            )],
            vec!["u"],
        );
        // M25-005-C: all-encodable unions compile
        assert!(EncoderProgram::compile(&union_member).is_some());
        // ...but a union carrying an unencodable member keeps the
        // reference path (a value could match ONLY that member)
        let union_unencodable = obj(
            vec![(
                "u",
                SchemaIr::Union {
                    members: vec![
                        Box::new(SchemaIr::Integer {
                            minimum: None,
                            maximum: None,
                        }),
                        Box::new(obj(
                            vec![(
                                "x",
                                SchemaIr::Integer {
                                    minimum: None,
                                    maximum: None,
                                },
                            )],
                            vec!["x"],
                        )),
                    ],
                },
            )],
            vec!["u"],
        );
        assert!(EncoderProgram::compile(&union_unencodable).is_none());

        let transform = obj(
            vec![(
                "t",
                SchemaIr::Transform {
                    input: Box::new(SchemaIr::String {
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        format: None,
                    }),
                    output: Box::new(SchemaIr::String {
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        format: None,
                    }),
                    name: "upper".into(),
                },
            )],
            vec![],
        );
        assert!(EncoderProgram::compile(&transform).is_none());

        let file = obj(
            vec![(
                "f",
                SchemaIr::File {
                    content_type: None,
                    max_bytes: 1024,
                },
            )],
            vec![],
        );
        assert!(EncoderProgram::compile(&file).is_none());

        let fallback_no_inner = obj(
            vec![(
                "fb",
                SchemaIr::Fallback {
                    reason: "explicit".into(),
                    inner: None,
                },
            )],
            vec![],
        );
        assert!(EncoderProgram::compile(&fallback_no_inner).is_none());

        // fallback WITH inner is transparent — still encodable
        let fallback_inner = obj(
            vec![(
                "fb",
                SchemaIr::Fallback {
                    reason: "explicit".into(),
                    inner: Some(Box::new(SchemaIr::Integer {
                        minimum: None,
                        maximum: None,
                    })),
                },
            )],
            vec!["fb"],
        );
        assert!(EncoderProgram::compile(&fallback_inner).is_some());

        // non-object top level keeps the reference path (string responses)
        assert!(EncoderProgram::compile(&SchemaIr::Boolean).is_none());
    }

    /// M25-005-B: the read pass walks the FIXED declared order. With
    /// multiple per-property failures the typed-error sequence follows the
    /// declared (byte-sorted) property order, not the handler value's key
    /// insertion order — deterministic output contracts.
    #[test]
    fn encoder_reads_properties_in_declared_fixed_order() {
        let ir = obj(
            vec![
                (
                    "beta",
                    SchemaIr::Integer {
                        minimum: None,
                        maximum: None,
                    },
                ),
                (
                    "alpha",
                    SchemaIr::Integer {
                        minimum: None,
                        maximum: None,
                    },
                ),
                (
                    "gamma",
                    SchemaIr::Integer {
                        minimum: None,
                        maximum: None,
                    },
                ),
            ],
            vec!["beta", "alpha", "gamma"],
        );
        let program = EncoderProgram::compile(&ir).unwrap();

        // value keys arrive REVERSED from declared order; every property
        // fails its type check — errors must come back in declared order
        let value = json!({"gamma": "bad", "beta": "bad", "alpha": "bad"});
        let mut out = Vec::new();
        let err = program.encode(&value, &mut out).unwrap_err();
        let paths: Vec<&str> = err.iter().map(|e| e.path.as_str()).collect();
        assert_eq!(paths, vec!["alpha", "beta", "gamma"]);

        // valid value with reversed key order encodes in declared order
        let value = json!({"gamma": 3, "beta": 2, "alpha": 1});
        let mut out = Vec::new();
        program.encode(&value, &mut out).unwrap();
        assert_eq!(out, b"{\"alpha\":1,\"beta\":2,\"gamma\":3}");
    }

    /// M25-005-B: compilation is deterministic — the same schema compiles
    /// to an equal program and encodes identical bytes every time.
    #[test]
    fn encoder_program_is_deterministic_across_compiles() {
        let ir = obj(
            vec![
                (
                    "a",
                    SchemaIr::Integer {
                        minimum: None,
                        maximum: None,
                    },
                ),
                (
                    "b",
                    SchemaIr::Optional {
                        inner: Box::new(SchemaIr::String {
                            min_length: None,
                            max_length: None,
                            pattern: None,
                            format: None,
                        }),
                        default: Some(json!("dflt")),
                    },
                ),
            ],
            vec!["a"],
        );
        let first = EncoderProgram::compile(&ir).unwrap();
        let second = EncoderProgram::compile(&ir).unwrap();
        assert_eq!(first, second, "compilation must be deterministic");

        let value = json!({"a": 1});
        let mut bytes_a = Vec::new();
        let mut bytes_b = Vec::new();
        first.encode(&value, &mut bytes_a).unwrap();
        second.encode(&value, &mut bytes_b).unwrap();
        assert_eq!(bytes_a, bytes_b);
        assert_eq!(bytes_a, b"{\"a\":1,\"b\":\"dflt\"}");
    }

    /// M25-005-C: unions encode first-match-wins with reference parity,
    /// and a failed member's partial bytes never reach the output.
    #[test]
    fn unions_encode_via_first_matching_member_with_parity() {
        let string_spec = SchemaIr::String {
            min_length: None,
            max_length: None,
            pattern: None,
            format: None,
        };
        let ir = obj(
            vec![(
                "u",
                SchemaIr::Union {
                    members: vec![
                        // first member matches arrays of integers and can
                        // emit partial bytes before a later item fails —
                        // the scratch buffer must discard them
                        Box::new(SchemaIr::Array {
                            items: Box::new(SchemaIr::Integer {
                                minimum: None,
                                maximum: None,
                            }),
                            min_items: None,
                            max_items: None,
                        }),
                        Box::new(string_spec.clone()),
                    ],
                },
            )],
            vec!["u"],
        );
        let program = EncoderProgram::compile(&ir).unwrap();

        // first member matches
        let value = json!({"u": [1, 2]});
        let reference = validate(&ir, &value, Source::Body).unwrap();
        let mut out = Vec::new();
        program.encode(&value, &mut out).unwrap();
        assert_eq!(out, serde_json::to_vec(&reference).unwrap());

        // first member fails mid-array (partial `[1,` written) -> the
        // second member (mixed int/string array) matches the whole value;
        // output must contain ONLY the winning bytes
        let ir_mixed = obj(
            vec![(
                "u",
                SchemaIr::Union {
                    members: vec![
                        Box::new(SchemaIr::Array {
                            items: Box::new(SchemaIr::Integer {
                                minimum: None,
                                maximum: None,
                            }),
                            min_items: None,
                            max_items: None,
                        }),
                        Box::new(SchemaIr::Array {
                            items: Box::new(SchemaIr::Union {
                                members: vec![
                                    Box::new(SchemaIr::Integer {
                                        minimum: None,
                                        maximum: None,
                                    }),
                                    Box::new(SchemaIr::String {
                                        min_length: None,
                                        max_length: None,
                                        pattern: None,
                                        format: None,
                                    }),
                                ],
                            }),
                            min_items: None,
                            max_items: None,
                        }),
                    ],
                },
            )],
            vec!["u"],
        );
        let program_mixed = EncoderProgram::compile(&ir_mixed).unwrap();
        let value = json!({"u": [1, "x"]});
        let reference = validate(&ir_mixed, &value, Source::Body).unwrap();
        let mut out = Vec::new();
        program_mixed.encode(&value, &mut out).unwrap();
        assert_eq!(out, serde_json::to_vec(&reference).unwrap());
        // the winning member's bytes are the mixed array, NOT the
        // partially-written integer-only prefix duplicated
        assert_eq!(out, b"{\"u\":[1,\"x\"]}");

        // no member matches: typed union error, same code+path as the
        // reference validator
        let value = json!({"u": true});
        let mut out = Vec::new();
        let err = program.encode(&value, &mut out).unwrap_err();
        let ref_err = validate(&ir, &value, Source::Body).unwrap_err();
        assert_eq!(err[0].code, ref_err[0].code);
        assert_eq!(err[0].code, "union");
        assert_eq!(err[0].path, ref_err[0].path);
    }

    /// M25-005-C: optional/null combinations normalize exactly like the
    /// reference validator (optional absorbs null into its default before
    /// nullable ever sees it; nested optional defaults apply on absence).
    #[test]
    fn optional_null_combinations_match_reference() {
        let int = SchemaIr::Integer {
            minimum: None,
            maximum: None,
        };
        let str_spec = SchemaIr::String {
            min_length: None,
            max_length: None,
            pattern: None,
            format: None,
        };
        let cases: Vec<(SchemaIr, Value)> = vec![
            // Optional<Nullable<Integer>> with default: null resolves to
            // the DEFAULT (reference Optional branch), never to bare null
            (
                obj(
                    vec![(
                        "v",
                        SchemaIr::Optional {
                            inner: Box::new(SchemaIr::Nullable {
                                inner: Box::new(int.clone()),
                            }),
                            default: Some(json!(9)),
                        },
                    )],
                    vec![],
                ),
                json!({"v": null}),
            ),
            // same shape, absent key -> default
            (
                obj(
                    vec![(
                        "v",
                        SchemaIr::Optional {
                            inner: Box::new(SchemaIr::Nullable {
                                inner: Box::new(int.clone()),
                            }),
                            default: Some(json!(9)),
                        },
                    )],
                    vec![],
                ),
                json!({}),
            ),
            // same shape, present value
            (
                obj(
                    vec![(
                        "v",
                        SchemaIr::Optional {
                            inner: Box::new(SchemaIr::Nullable {
                                inner: Box::new(int.clone()),
                            }),
                            default: Some(json!(9)),
                        },
                    )],
                    vec![],
                ),
                json!({"v": 4}),
            ),
            // Nullable<Optional<Integer>> with default: null -> null (the
            // nullable branch sees the null first)
            (
                obj(
                    vec![(
                        "v",
                        SchemaIr::Nullable {
                            inner: Box::new(SchemaIr::Optional {
                                inner: Box::new(int.clone()),
                                default: Some(json!(9)),
                            }),
                        },
                    )],
                    vec![],
                ),
                json!({"v": null}),
            ),
            // Optional<Optional<String>> nested defaults collapse to the
            // outer default on absence
            (
                obj(
                    vec![(
                        "v",
                        SchemaIr::Optional {
                            inner: Box::new(SchemaIr::Optional {
                                inner: Box::new(str_spec.clone()),
                                default: Some(json!("inner")),
                            }),
                            default: Some(json!("outer")),
                        },
                    )],
                    vec![],
                ),
                json!({}),
            ),
            // null under Optional<Optional<String>> resolves outer default
            (
                obj(
                    vec![(
                        "v",
                        SchemaIr::Optional {
                            inner: Box::new(SchemaIr::Optional {
                                inner: Box::new(str_spec.clone()),
                                default: Some(json!("inner")),
                            }),
                            default: Some(json!("outer")),
                        },
                    )],
                    vec![],
                ),
                json!({"v": null}),
            ),
        ];
        for (ir, value) in cases {
            let reference = validate(&ir, &value, Source::Body)
                .unwrap_or_else(|e| panic!("case must be valid: {:?} ({:?})", value, e));
            let program = EncoderProgram::compile(&ir).unwrap();
            let mut out = Vec::new();
            program.encode(&value, &mut out).unwrap();
            assert_eq!(
                out,
                serde_json::to_vec(&reference).unwrap(),
                "combination parity failed for {:?}",
                value
            );
        }
    }

    /// The table is dense by SchemaId and mirrors the decoder table's
    /// construction from the same schema vector.
    #[test]
    fn encoder_table_is_dense_by_schema_id() {
        let flat = obj(
            vec![(
                "a",
                SchemaIr::Integer {
                    minimum: None,
                    maximum: None,
                },
            )],
            vec!["a"],
        );
        let schemas = vec![
            flat.clone(),
            SchemaIr::Fallback {
                reason: "explicit".into(),
                inner: None,
            },
            SchemaIr::Boolean,
        ];
        let table = EncoderTable::from_schemas(&schemas);
        assert!(table.get(0).is_some(), "flat object is encodable");
        assert!(table.get(1).is_none(), "fallback-without-inner is not");
        assert!(table.get(2).is_none(), "non-object top level is not");
        assert!(table.get(999).is_none(), "unknown id is not");
    }
}
