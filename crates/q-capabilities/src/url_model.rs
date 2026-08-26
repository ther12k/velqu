//! URL and URLSearchParams implementation based on the WHATWG URL standard (M27-005-A).
//!
//! Provides bounded, conformant URL parsing and manipulation for runtime capabilities,
//! fetch, and application handlers.

use std::fmt;
use url::Url;

/// Maximum length in bytes for a URL string input to prevent unbounded parser CPU/memory.
pub const MAX_URL_LEN: usize = 8_192;

/// Typed URL parse errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UrlError {
    EmptyInput,
    InputTooLong { len: usize, max: usize },
    InvalidUrl(String),
    InvalidBase(String),
}

impl fmt::Display for UrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UrlError::EmptyInput => f.write_str("URL input is empty"),
            UrlError::InputTooLong { len, max } => {
                write!(f, "URL length {len} exceeds maximum allowed limit {max}")
            }
            UrlError::InvalidUrl(err) => write!(f, "invalid URL: {err}"),
            UrlError::InvalidBase(err) => write!(f, "invalid base URL: {err}"),
        }
    }
}

impl std::error::Error for UrlError {}

/// Fully parsed WHATWG URL model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedUrl {
    pub href: String,
    pub origin: String,
    pub protocol: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub hostname: String,
    pub port: String,
    pub pathname: String,
    pub search: String,
    pub hash: String,
    pub search_params: Vec<(String, String)>,
}

impl ParsedUrl {
    /// Parse a URL string with an optional base URL.
    pub fn parse(input: &str, base: Option<&str>) -> Result<Self, UrlError> {
        if input.is_empty() {
            return Err(UrlError::EmptyInput);
        }
        if input.len() > MAX_URL_LEN {
            return Err(UrlError::InputTooLong {
                len: input.len(),
                max: MAX_URL_LEN,
            });
        }
        if let Some(b) = base {
            if b.len() > MAX_URL_LEN {
                return Err(UrlError::InputTooLong {
                    len: b.len(),
                    max: MAX_URL_LEN,
                });
            }
        }

        let parsed = match base {
            Some(base_str) => {
                let base_url =
                    Url::parse(base_str).map_err(|e| UrlError::InvalidBase(e.to_string()))?;
                base_url
                    .join(input)
                    .map_err(|e| UrlError::InvalidUrl(e.to_string()))?
            }
            None => Url::parse(input).map_err(|e| UrlError::InvalidUrl(e.to_string()))?,
        };

        Self::from_url(parsed)
    }

    /// Check if a URL string can be parsed without throwing.
    pub fn can_parse(input: &str, base: Option<&str>) -> bool {
        Self::parse(input, base).is_ok()
    }

    fn from_url(url: Url) -> Result<Self, UrlError> {
        let href = url.to_string();
        let origin = url.origin().unicode_serialization();
        let protocol = format!("{}:", url.scheme());
        let username = url.username().to_string();
        let password = url.password().unwrap_or("").to_string();
        let hostname = url.host_str().unwrap_or("").to_string();
        let port = url.port().map(|p| p.to_string()).unwrap_or_default();
        let host = if port.is_empty() {
            hostname.clone()
        } else {
            format!("{hostname}:{port}")
        };
        let pathname = url.path().to_string();
        let search = url.query().map(|q| format!("?{q}")).unwrap_or_default();
        let hash = url.fragment().map(|f| format!("#{f}")).unwrap_or_default();
        let search_params = url
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        Ok(ParsedUrl {
            href,
            origin,
            protocol,
            username,
            password,
            host,
            hostname,
            port,
            pathname,
            search,
            hash,
            search_params,
        })
    }
}

/// Parsed URL search parameters.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedSearchParams {
    params: Vec<(String, String)>,
}

impl ParsedSearchParams {
    /// Parse from query string (with or without leading '?').
    pub fn parse(query: &str) -> Self {
        let clean = query.strip_prefix('?').unwrap_or(query);
        if clean.is_empty() {
            return ParsedSearchParams { params: Vec::new() };
        }
        let pairs = url::form_urlencoded::parse(clean.as_bytes())
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        ParsedSearchParams { params: pairs }
    }

    pub fn from_pairs(pairs: Vec<(String, String)>) -> Self {
        ParsedSearchParams { params: pairs }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.params
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    pub fn get_all(&self, name: &str) -> Vec<&str> {
        self.params
            .iter()
            .filter(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
            .collect()
    }

    pub fn has(&self, name: &str, value: Option<&str>) -> bool {
        match value {
            Some(v) => self.params.iter().any(|(k, val)| k == name && val == v),
            None => self.params.iter().any(|(k, _)| k == name),
        }
    }

    pub fn append(&mut self, name: &str, value: &str) {
        self.params.push((name.to_string(), value.to_string()));
    }

    pub fn set(&mut self, name: &str, value: &str) {
        let mut first = true;
        self.params.retain_mut(|(k, v)| {
            if k == name {
                if first {
                    *v = value.to_string();
                    first = false;
                    true
                } else {
                    false
                }
            } else {
                true
            }
        });
        if first {
            self.params.push((name.to_string(), value.to_string()));
        }
    }

    pub fn delete(&mut self, name: &str, value: Option<&str>) {
        match value {
            Some(v) => self.params.retain(|(k, val)| !(k == name && val == v)),
            None => self.params.retain(|(k, _)| k != name),
        }
    }

    pub fn sort(&mut self) {
        // Stable sort by name (UTF-16 code units / UTF-8 byte order)
        self.params.sort_by(|a, b| a.0.cmp(&b.0));
    }

    pub fn to_query_string(&self) -> String {
        if self.params.is_empty() {
            return String::new();
        }
        let mut serializer = url::form_urlencoded::Serializer::new(String::new());
        for (k, v) in &self.params {
            serializer.append_pair(k, v);
        }
        serializer.finish()
    }

    pub fn entries(&self) -> &[(String, String)] {
        &self.params
    }

    pub fn len(&self) -> usize {
        self.params.len()
    }

    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_urls() {
        let url = ParsedUrl::parse(
            "https://user:pass@example.com:8080/path/to/page?query=1&foo=bar#section",
            None,
        )
        .unwrap();
        assert_eq!(url.protocol, "https:");
        assert_eq!(url.username, "user");
        assert_eq!(url.password, "pass");
        assert_eq!(url.hostname, "example.com");
        assert_eq!(url.port, "8080");
        assert_eq!(url.host, "example.com:8080");
        assert_eq!(url.pathname, "/path/to/page");
        assert_eq!(url.search, "?query=1&foo=bar");
        assert_eq!(url.hash, "#section");
        assert_eq!(url.origin, "https://example.com:8080");
    }

    #[test]
    fn parse_with_base_url() {
        let url = ParsedUrl::parse("sub/path?id=123", Some("https://example.org/api/v1/")).unwrap();
        assert_eq!(url.href, "https://example.org/api/v1/sub/path?id=123");
        assert_eq!(url.pathname, "/api/v1/sub/path");
    }

    #[test]
    fn can_parse_checks() {
        assert!(ParsedUrl::can_parse("https://example.com", None));
        assert!(ParsedUrl::can_parse(
            "/relative",
            Some("https://example.com")
        ));
        assert!(!ParsedUrl::can_parse("invalid url without base", None));
        assert!(!ParsedUrl::can_parse("", None));
    }

    #[test]
    fn url_length_limit_is_enforced() {
        let huge = format!("https://example.com/{}", "a".repeat(MAX_URL_LEN));
        assert_eq!(
            ParsedUrl::parse(&huge, None),
            Err(UrlError::InputTooLong {
                len: huge.len(),
                max: MAX_URL_LEN
            })
        );
    }

    #[test]
    fn search_params_operations() {
        let mut sp = ParsedSearchParams::parse("?a=1&b=2&a=3");
        assert_eq!(sp.get("a"), Some("1"));
        assert_eq!(sp.get_all("a"), vec!["1", "3"]);
        assert!(sp.has("b", None));
        assert!(sp.has("a", Some("3")));
        assert!(!sp.has("a", Some("4")));

        sp.append("c", "4");
        assert_eq!(sp.get("c"), Some("4"));

        sp.set("a", "10");
        assert_eq!(sp.get_all("a"), vec!["10"]);

        sp.delete("b", None);
        assert_eq!(sp.get("b"), None);

        sp.sort();
        assert_eq!(sp.to_query_string(), "a=10&c=4");
    }

    #[test]
    fn search_params_encoding_and_decoding() {
        let sp = ParsedSearchParams::parse("key=hello+world%26more%3Dyes");
        assert_eq!(sp.get("key"), Some("hello world&more=yes"));
        assert_eq!(sp.to_query_string(), "key=hello+world%26more%3Dyes");
    }
}
