//! Console capability: log levels, structured records, and sensitive data redaction (M27-004-B).
//!
//! Enforces bounded message formatting, a closed vocabulary of standard log levels,
//! and automatic redaction of sensitive tokens (authorization headers, API keys, secrets)
//! so console logging never leaks credentials into telemetry sinks.

use std::fmt;

/// Maximum length in bytes for a single console log message.
pub const MAX_CONSOLE_MSG_LEN: usize = 16_384;
/// Maximum number of arguments in a single console call.
pub const MAX_CONSOLE_ARGS: usize = 32;

/// Closed vocabulary of console log levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ConsoleLevel {
    Debug = 0,
    #[default]
    Info = 1,
    Warn = 2,
    Error = 3,
}

impl ConsoleLevel {
    /// Parse from string with fail-closed semantics (None on unknown level).
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "debug" => Some(ConsoleLevel::Debug),
            "info" => Some(ConsoleLevel::Info),
            "warn" | "warning" => Some(ConsoleLevel::Warn),
            "error" => Some(ConsoleLevel::Error),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ConsoleLevel::Debug => "debug",
            ConsoleLevel::Info => "info",
            ConsoleLevel::Warn => "warn",
            ConsoleLevel::Error => "error",
        }
    }
}

impl fmt::Display for ConsoleLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Redact known sensitive patterns from console log messages.
///
/// Replaces:
/// - `Bearer <token>` -> `Bearer [REDACTED]`
/// - `Basic <token>` -> `Basic [REDACTED]`
/// - `sk-live-[A-Za-z0-9_-]+` -> `sk-live-[REDACTED]`
/// - `key=...`, `password=...`, `secret=...`, `token=...` patterns -> `key=[REDACTED]`, etc.
pub fn redact_sensitive_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    let bytes = text.as_bytes();
    let len = bytes.len();

    while i < len {
        let remainder = &text[i..];

        // Check Bearer / Basic auth header values
        if let Some(prefix) = ["Bearer ", "bearer ", "Basic ", "basic "]
            .iter()
            .find(|p| remainder.starts_with(*p))
        {
            out.push_str(prefix);
            out.push_str("[REDACTED]");
            i += prefix.len();
            // skip until whitespace or delimiter
            while i < len
                && !bytes[i].is_ascii_whitespace()
                && bytes[i] != b','
                && bytes[i] != b';'
                && bytes[i] != b'"'
                && bytes[i] != b'\''
            {
                i += 1;
            }
            continue;
        }

        // Check API secret prefixes (e.g. sk-live-..., sk-test-...)
        if let Some(prefix) = ["sk-live-", "sk-test-", "ghp_", "gho_", "glpat-"]
            .iter()
            .find(|p| remainder.starts_with(*p))
        {
            out.push_str(prefix);
            out.push_str("[REDACTED]");
            i += prefix.len();
            while i < len
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'-')
            {
                i += 1;
            }
            continue;
        }

        // Check key=value secret patterns
        if let Some((key, sep)) = [
            ("password", "="),
            ("password", ":"),
            ("secret", "="),
            ("secret", ":"),
            ("api_key", "="),
            ("api_key", ":"),
            ("apikey", "="),
            ("apikey", ":"),
            ("auth_token", "="),
            ("auth_token", ":"),
            ("token", "="),
            ("token", ":"),
        ]
        .iter()
        .find(|(k, s)| {
            if remainder.len() > k.len() + s.len() {
                let prefix = &remainder[..k.len()];
                let after = &remainder[k.len()..k.len() + s.len()];
                prefix.eq_ignore_ascii_case(k) && after == *s
            } else {
                false
            }
        }) {
            out.push_str(&remainder[..key.len() + sep.len()]);
            out.push_str("[REDACTED]");
            i += key.len() + sep.len();
            // Skip optional quote
            let in_quote = if i < len && (bytes[i] == b'"' || bytes[i] == b'\'') {
                let q = bytes[i];
                i += 1;
                Some(q)
            } else {
                None
            };
            while i < len {
                if let Some(q) = in_quote {
                    if bytes[i] == q {
                        i += 1;
                        break;
                    }
                } else if bytes[i].is_ascii_whitespace()
                    || bytes[i] == b','
                    || bytes[i] == b'&'
                    || bytes[i] == b';'
                    || bytes[i] == b'}'
                {
                    break;
                }
                i += 1;
            }
            continue;
        }

        // Default: copy UTF-8 char
        if let Some(c) = remainder.chars().next() {
            out.push(c);
            i += c.len_utf8();
        } else {
            break;
        }
    }

    out
}

/// A structured console log record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsoleRecord {
    pub level: ConsoleLevel,
    pub message: String,
    pub route_id: Option<String>,
    pub invocation_id: Option<u64>,
}

impl ConsoleRecord {
    /// Create a new record with sensitive tokens redacted and length bounded.
    pub fn new(
        level: ConsoleLevel,
        raw_message: &str,
        route_id: Option<String>,
        invocation_id: Option<u64>,
    ) -> Self {
        let redacted = redact_sensitive_text(raw_message);
        let message = if redacted.len() > MAX_CONSOLE_MSG_LEN {
            let mut truncated = redacted[..MAX_CONSOLE_MSG_LEN].to_string();
            truncated.push_str("...[TRUNCATED]");
            truncated
        } else {
            redacted
        };
        ConsoleRecord {
            level,
            message,
            route_id,
            invocation_id,
        }
    }

    /// Render structured JSON format.
    pub fn to_json_value(&self) -> serde_json::Value {
        serde_json::json!({
            "level": self.level.as_str(),
            "event": "console.log",
            "message": self.message,
            "routeId": self.route_id,
            "invocationId": self.invocation_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn console_levels_parse_and_display() {
        assert_eq!(ConsoleLevel::parse("debug"), Some(ConsoleLevel::Debug));
        assert_eq!(ConsoleLevel::parse("info"), Some(ConsoleLevel::Info));
        assert_eq!(ConsoleLevel::parse("warn"), Some(ConsoleLevel::Warn));
        assert_eq!(ConsoleLevel::parse("warning"), Some(ConsoleLevel::Warn));
        assert_eq!(ConsoleLevel::parse("error"), Some(ConsoleLevel::Error));
        assert_eq!(ConsoleLevel::parse("DEBUG"), Some(ConsoleLevel::Debug));
        assert_eq!(ConsoleLevel::parse("invalid"), None);
        assert_eq!(ConsoleLevel::parse(""), None);

        assert_eq!(ConsoleLevel::Debug.as_str(), "debug");
        assert_eq!(ConsoleLevel::Info.as_str(), "info");
        assert_eq!(ConsoleLevel::Warn.as_str(), "warn");
        assert_eq!(ConsoleLevel::Error.as_str(), "error");
        assert_eq!(ConsoleLevel::default(), ConsoleLevel::Info);
    }

    #[test]
    fn redact_bearer_and_basic_tokens() {
        let text = "Headers: Authorization: Bearer secret-token-12345, Content-Type: text/plain";
        let redacted = redact_sensitive_text(text);
        assert_eq!(
            redacted,
            "Headers: Authorization: Bearer [REDACTED], Content-Type: text/plain"
        );

        let basic = "Auth: Basic dXNlcjpwYXNz; user=admin";
        assert_eq!(
            redact_sensitive_text(basic),
            "Auth: Basic [REDACTED]; user=admin"
        );
    }

    #[test]
    fn redact_api_keys_and_passwords() {
        let msg = "connecting with api_key=sk-live-abcdef123456 and password=supersecret now";
        let redacted = redact_sensitive_text(msg);
        assert!(!redacted.contains("sk-live-abcdef123456"));
        assert!(!redacted.contains("supersecret"));
        assert!(redacted.contains("api_key=[REDACTED]"));
        assert!(redacted.contains("password=[REDACTED]"));
    }

    #[test]
    fn redact_prefix_tokens() {
        let msg = "GitHub token ghp_1234567890abcdef and sk-live-999000";
        let redacted = redact_sensitive_text(msg);
        assert!(!redacted.contains("ghp_1234567890abcdef"));
        assert!(!redacted.contains("sk-live-999000"));
        assert!(redacted.contains("ghp_[REDACTED]"));
        assert!(redacted.contains("sk-live-[REDACTED]"));
    }

    #[test]
    fn message_length_is_bounded() {
        let huge = "a".repeat(MAX_CONSOLE_MSG_LEN + 100);
        let record = ConsoleRecord::new(ConsoleLevel::Info, &huge, None, None);
        assert!(record.message.len() <= MAX_CONSOLE_MSG_LEN + 15);
        assert!(record.message.ends_with("...[TRUNCATED]"));
    }

    #[test]
    fn console_record_json_serialization() {
        let record = ConsoleRecord::new(
            ConsoleLevel::Warn,
            "User login failed for secret=pass123",
            Some("auth.login".into()),
            Some(42),
        );
        let json = record.to_json_value();
        assert_eq!(json["level"], "warn");
        assert_eq!(json["event"], "console.log");
        assert_eq!(json["routeId"], "auth.login");
        assert_eq!(json["invocationId"], 42);
        assert_eq!(json["message"], "User login failed for secret=[REDACTED]");
    }
}
