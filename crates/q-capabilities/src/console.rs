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
            ("authorization", "="),
            ("authorization", ":"),
            ("cookie", "="),
            ("cookie", ":"),
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
            // Assignment-form Authorization may contain a scheme followed by
            // a second token; both belong to the redacted value.
            if in_quote.is_none() && key.eq_ignore_ascii_case("authorization") {
                while i < len && bytes[i].is_ascii_whitespace() {
                    i += 1;
                }
                if text[i..].starts_with("Bearer ") || text[i..].starts_with("Basic ") {
                    let scheme_start = i;
                    while i < len && !bytes[i].is_ascii_whitespace() {
                        i += 1;
                    }
                    let scheme = &text[scheme_start..i];
                    while i < len && bytes[i].is_ascii_whitespace() {
                        i += 1;
                    }
                    while i < len
                        && !bytes[i].is_ascii_whitespace()
                        && bytes[i] != b','
                        && bytes[i] != b';'
                        && bytes[i] != b'}'
                    {
                        i += 1;
                    }
                    out.truncate(out.len().saturating_sub("[REDACTED]".len()));
                    out.push(' ');
                    out.push_str(scheme);
                    out.push_str(" [REDACTED]");
                }
            }
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
                    // Authorization assignments commonly contain a scheme
                    // and a credential separated by a space; consume both.
                    let rest = &text[i..];
                    if rest.starts_with(" ") {
                        let after_space = rest.trim_start();
                        if after_space.starts_with("Bearer ") || after_space.starts_with("Basic ") {
                            i += rest.len() - after_space.len();
                            while i < len
                                && !bytes[i].is_ascii_whitespace()
                                && bytes[i] != b','
                                && bytes[i] != b';'
                                && bytes[i] != b'}'
                            {
                                i += 1;
                            }
                        }
                    }
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

/// Default capacity for the bounded log sink buffer.
pub const DEFAULT_LOG_SINK_CAP: usize = 1024;

/// Bounded log sink that never blocks the worker and never grows without limit (M27-004-C).
/// Excess logs increment the `dropped` counter instead of allocating unboundedly.
#[derive(Debug)]
pub struct BoundedLogSink {
    buffer: std::sync::Mutex<std::collections::VecDeque<ConsoleRecord>>,
    capacity: usize,
    enqueued: std::sync::atomic::AtomicU64,
    dropped: std::sync::atomic::AtomicU64,
    drained: std::sync::atomic::AtomicU64,
}

impl BoundedLogSink {
    /// Create with a fixed capacity.
    pub fn new(capacity: usize) -> Self {
        let cap = if capacity == 0 {
            DEFAULT_LOG_SINK_CAP
        } else {
            capacity
        };
        BoundedLogSink {
            buffer: std::sync::Mutex::new(std::collections::VecDeque::with_capacity(cap)),
            capacity: cap,
            enqueued: std::sync::atomic::AtomicU64::new(0),
            dropped: std::sync::atomic::AtomicU64::new(0),
            drained: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Non-blocking enqueue. If capacity is exhausted, the record is dropped
    /// and the `dropped` count incremented (fail-safe logging: log load cannot starve requests).
    pub fn try_push(&self, record: ConsoleRecord) -> bool {
        let mut buf = self.buffer.lock().unwrap();
        if buf.len() < self.capacity {
            buf.push_back(record);
            self.enqueued
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            true
        } else {
            self.dropped
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            false
        }
    }

    /// Drain all queued records into a Vec.
    pub fn drain(&self) -> Vec<ConsoleRecord> {
        let mut buf = self.buffer.lock().unwrap();
        let records: Vec<ConsoleRecord> = buf.drain(..).collect();
        self.drained
            .fetch_add(records.len() as u64, std::sync::atomic::Ordering::Relaxed);
        records
    }

    /// Snapshot buffer and dropped-log statistics.
    pub fn stats(&self) -> LogSinkStats {
        LogSinkStats {
            enqueued: self.enqueued.load(std::sync::atomic::Ordering::Relaxed),
            dropped: self.dropped.load(std::sync::atomic::Ordering::Relaxed),
            drained: self.drained.load(std::sync::atomic::Ordering::Relaxed),
            buffered: self.buffer.lock().unwrap().len(),
        }
    }
}

impl Default for BoundedLogSink {
    fn default() -> Self {
        Self::new(DEFAULT_LOG_SINK_CAP)
    }
}

/// Statistics snapshot for a `BoundedLogSink`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogSinkStats {
    pub enqueued: u64,
    pub dropped: u64,
    pub drained: u64,
    pub buffered: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_log_sink_drops_on_overflow_without_blocking() {
        let sink = BoundedLogSink::new(3);
        assert!(sink.try_push(ConsoleRecord::new(ConsoleLevel::Info, "msg 1", None, None)));
        assert!(sink.try_push(ConsoleRecord::new(ConsoleLevel::Info, "msg 2", None, None)));
        assert!(sink.try_push(ConsoleRecord::new(ConsoleLevel::Info, "msg 3", None, None)));
        // 4th push exceeds capacity: dropped
        assert!(!sink.try_push(ConsoleRecord::new(ConsoleLevel::Info, "msg 4", None, None)));

        let stats = sink.stats();
        assert_eq!(stats.enqueued, 3);
        assert_eq!(stats.dropped, 1);
        assert_eq!(stats.buffered, 3);

        // Drain records
        let drained = sink.drain();
        assert_eq!(drained.len(), 3);
        assert_eq!(drained[0].message, "msg 1");
        assert_eq!(drained[2].message, "msg 3");

        let stats_after = sink.stats();
        assert_eq!(stats_after.drained, 3);
        assert_eq!(stats_after.buffered, 0);

        // Can push again after drain
        assert!(sink.try_push(ConsoleRecord::new(ConsoleLevel::Info, "msg 5", None, None)));
        assert_eq!(sink.stats().buffered, 1);
    }

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
    fn redact_header_and_cookie_assignment_values() {
        let msg = "authorization=secret-value; cookie=session=private-cookie";
        let redacted = redact_sensitive_text(msg);
        assert!(!redacted.contains("secret-value"));
        assert!(!redacted.contains("private-cookie"));
        assert!(redacted.contains("authorization=[REDACTED]"));
        assert!(redacted.contains("cookie=[REDACTED]"));
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
