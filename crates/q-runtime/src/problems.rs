//! RFC 9457-compatible problem registry (frozen URNs; see pack-format spec).

use serde_json::{json, Value};

pub fn registry(problem_id: &str) -> (&'static str, &'static str, u16) {
    match problem_id {
        "validation" => ("https://velqu.dev/problems/validation", "Validation failed", 422),
        "unauthorized" => ("https://velqu.dev/problems/unauthorized", "Unauthorized", 401),
        "not-found" => ("https://velqu.dev/problems/not-found", "Not Found", 404),
        "method" => ("https://velqu.dev/problems/method", "Method Not Allowed", 405),
        "body" => ("https://velqu.dev/problems/body", "Unsupported body", 415),
        "limit" => ("https://velqu.dev/problems/limit", "Payload too large", 413),
        "timeout" => ("https://velqu.dev/problems/timeout", "Handler deadline exceeded", 504),
        _ => ("https://velqu.dev/problems/internal", "Internal Server Error", 500),
    }
}

/// Build a problem body. `errors` carries field-level validation failures.
pub fn body(
    problem_id: &str,
    status_override: Option<u16>,
    detail: Option<&str>,
    errors: &[q_engine::FieldErrorOut],
    instance: &str,
) -> Value {
    let (type_uri, title, status) = registry(problem_id);
    let mut v = json!({
        "type": type_uri,
        "title": title,
        "status": status_override.unwrap_or(status),
        "instance": instance,
    });
    if let Some(d) = detail {
        v["detail"] = json!(d);
    }
    if !errors.is_empty() {
        v["errors"] = serde_json::to_value(errors).unwrap_or(Value::Null);
    }
    v
}

/// Redaction check used by tests and the response path: no secret may cross.
pub fn is_redacted(text: &str, secrets: &[&str]) -> bool {
    secrets.iter().all(|s| !text.contains(s))
}
