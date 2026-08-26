//! URL and URLSearchParams implementation based on the WHATWG URL standard (M27-005-A).
//!
//! Provides bounded, conformant URL parsing and manipulation for runtime capabilities,
//! fetch, and application handlers.

use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};
use std::fmt;
use url::Url;

/// The WHATWG path percent-encode set: CONTROLS + ' ' + '"' + '#' + '<' + '>' + '?' + '`' + '{' + '}'
const PATH_PERCENT_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}');

/// Encode a single path segment per the WHATWG path percent-encode set (M27-005-C).
pub fn encode_path_segment(segment: &str) -> String {
    utf8_percent_encode(segment, PATH_PERCENT_ENCODE_SET).to_string()
}

/// Decode a percent-encoded path segment to UTF-8 text with lossy fallback on invalid sequences.
pub fn decode_path_segment(encoded: &str) -> String {
    percent_decode_str(encoded).decode_utf8_lossy().to_string()
}

/// Normalize an IDNA/IPv4/IPv6 host string using WHATWG parsing rules (M27-005-C).
pub fn normalize_host(host_input: &str) -> Result<String, UrlError> {
    if host_input.is_empty() {
        return Err(UrlError::EmptyInput);
    }
    if host_input.len() > MAX_URL_LEN {
        return Err(UrlError::InputTooLong {
            len: host_input.len(),
            max: MAX_URL_LEN,
        });
    }
    let dummy_url = format!("http://{host_input}/");
    let parsed = Url::parse(&dummy_url).map_err(|e| UrlError::InvalidUrl(e.to_string()))?;
    Ok(parsed.host_str().unwrap_or("").to_string())
}

/// Maximum length in bytes for a URL string input to prevent unbounded parser CPU/memory (M27-005-D).
pub const MAX_URL_LEN: usize = 8_192;
/// Maximum length in bytes for a URLSearchParams query string input (M27-005-D).
pub const MAX_SEARCH_PARAMS_LEN: usize = 16_384;
/// Maximum number of search parameter key-value pairs (M27-005-D).
pub const MAX_SEARCH_PARAMS_COUNT: usize = 1_024;
/// Maximum number of path segments in a URL path (M27-005-D).
pub const MAX_URL_PATH_SEGMENTS: usize = 256;

/// Typed URL parse errors (M27-005-D).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UrlError {
    EmptyInput,
    InputTooLong { len: usize, max: usize },
    ParamsTooLong { len: usize, max: usize },
    TooManyParams { count: usize, max: usize },
    TooManyPathSegments { count: usize, max: usize },
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
            UrlError::ParamsTooLong { len, max } => {
                write!(
                    f,
                    "URLSearchParams input length {len} exceeds maximum limit {max}"
                )
            }
            UrlError::TooManyParams { count, max } => {
                write!(
                    f,
                    "URLSearchParams entry count {count} exceeds maximum limit {max}"
                )
            }
            UrlError::TooManyPathSegments { count, max } => {
                write!(
                    f,
                    "URL path segments count {count} exceeds maximum limit {max}"
                )
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
        if let Some(segments) = url.path_segments() {
            let count = segments.count();
            if count > MAX_URL_PATH_SEGMENTS {
                return Err(UrlError::TooManyPathSegments {
                    count,
                    max: MAX_URL_PATH_SEGMENTS,
                });
            }
        }
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
    /// Parse from query string with fail-closed limit enforcement (M27-005-D).
    pub fn try_parse(query: &str) -> Result<Self, UrlError> {
        let clean = query.strip_prefix('?').unwrap_or(query);
        if clean.is_empty() {
            return Ok(ParsedSearchParams { params: Vec::new() });
        }
        if clean.len() > MAX_SEARCH_PARAMS_LEN {
            return Err(UrlError::ParamsTooLong {
                len: clean.len(),
                max: MAX_SEARCH_PARAMS_LEN,
            });
        }
        let mut pairs = Vec::new();
        for (k, v) in url::form_urlencoded::parse(clean.as_bytes()) {
            if pairs.len() >= MAX_SEARCH_PARAMS_COUNT {
                return Err(UrlError::TooManyParams {
                    count: pairs.len() + 1,
                    max: MAX_SEARCH_PARAMS_COUNT,
                });
            }
            pairs.push((k.into_owned(), v.into_owned()));
        }
        Ok(ParsedSearchParams { params: pairs })
    }

    /// Parse from query string (with or without leading '?').
    pub fn parse(query: &str) -> Self {
        Self::try_parse(query).unwrap_or_default()
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

    /// WPT URL resolution test vectors (Web Platform Tests suite subset).
    #[test]
    fn wpt_relative_url_resolution_vectors() {
        let base = "http://example.org/a/b/c?orig=1#hash";

        // Relative path
        let u1 = ParsedUrl::parse("../d", Some(base)).unwrap();
        assert_eq!(u1.href, "http://example.org/a/d");
        assert_eq!(u1.pathname, "/a/d");

        // Root-relative path
        let u2 = ParsedUrl::parse("/root", Some(base)).unwrap();
        assert_eq!(u2.href, "http://example.org/root");

        // Query only
        let u3 = ParsedUrl::parse("?new=2", Some(base)).unwrap();
        assert_eq!(u3.href, "http://example.org/a/b/c?new=2");

        // Hash only
        let u4 = ParsedUrl::parse("#newhash", Some(base)).unwrap();
        assert_eq!(u4.href, "http://example.org/a/b/c?orig=1#newhash");

        // Protocol-relative
        let u5 = ParsedUrl::parse("//other.com/path", Some(base)).unwrap();
        assert_eq!(u5.href, "http://other.com/path");
    }

    /// WPT default port and special scheme normalizations.
    #[test]
    fn wpt_special_schemes_and_default_ports() {
        let u1 = ParsedUrl::parse("http://example.com:80/path", None).unwrap();
        assert_eq!(u1.port, "");
        assert_eq!(u1.host, "example.com");
        assert_eq!(u1.href, "http://example.com/path");

        let u2 = ParsedUrl::parse("https://example.com:443/path", None).unwrap();
        assert_eq!(u2.port, "");
        assert_eq!(u2.host, "example.com");

        let u3 = ParsedUrl::parse("http://example.com:8080/path", None).unwrap();
        assert_eq!(u3.port, "8080");
        assert_eq!(u3.host, "example.com:8080");
    }

    /// WPT IPv6 and host serialization.
    #[test]
    fn wpt_ipv6_host_parsing() {
        let u = ParsedUrl::parse("http://[2001:db8::1]:8080/test", None).unwrap();
        assert_eq!(u.hostname, "[2001:db8::1]");
        assert_eq!(u.host, "[2001:db8::1]:8080");
        assert_eq!(u.origin, "http://[2001:db8::1]:8080");

        let u_local = ParsedUrl::parse("http://[::1]/", None).unwrap();
        assert_eq!(u_local.hostname, "[::1]");
        assert_eq!(u_local.origin, "http://[::1]");
    }

    /// WinterTC URLSearchParams compliance test cases.
    #[test]
    fn wintertc_urlsearchparams_vectors() {
        // Empty values vs absent values
        let sp = ParsedSearchParams::parse("a=&b");
        assert_eq!(sp.get("a"), Some(""));
        assert_eq!(sp.get("b"), Some(""));

        // Special characters in keys and values
        let sp2 = ParsedSearchParams::parse("a+b=c%26d&email=user%2Btag%40example.com");
        assert_eq!(sp2.get("a b"), Some("c&d"));
        assert_eq!(sp2.get("email"), Some("user+tag@example.com"));

        // Delete with specific value
        let mut sp3 = ParsedSearchParams::parse("tag=rust&tag=js&tag=rust");
        sp3.delete("tag", Some("rust"));
        assert_eq!(sp3.get_all("tag"), vec!["js"]);
    }

    /// M27-005-C: host and path encoding behavior tests.
    #[test]
    fn host_and_path_encoding_behavior() {
        // IDNA Punycode normalization
        assert_eq!(
            normalize_host("xn--bcher-kva.example").unwrap(),
            "xn--bcher-kva.example"
        );

        // Path segment encoding
        assert_eq!(encode_path_segment("hello world"), "hello%20world");
        assert_eq!(encode_path_segment("foo#bar?baz"), "foo%23bar%3Fbaz");
        assert_eq!(encode_path_segment("normal-path_123"), "normal-path_123");

        // Path segment decoding
        assert_eq!(decode_path_segment("hello%20world"), "hello world");
        assert_eq!(decode_path_segment("foo%23bar%3Fbaz"), "foo#bar?baz");
        assert_eq!(decode_path_segment("%C3%BCber"), "über");

        // URL parser canonicalizes percent-encoded path and IDNA host
        let u = ParsedUrl::parse("http://example.com/foo bar/baz#frag", None).unwrap();
        assert_eq!(u.pathname, "/foo%20bar/baz");
        assert_eq!(u.href, "http://example.com/foo%20bar/baz#frag");
    }

    /// M27-005-D: explicit parser limits enforcement.
    #[test]
    fn url_and_search_params_parser_limits_enforced() {
        // Query string length limit
        let huge_query = format!("key={}", "v".repeat(MAX_SEARCH_PARAMS_LEN + 10));
        assert_eq!(
            ParsedSearchParams::try_parse(&huge_query),
            Err(UrlError::ParamsTooLong {
                len: huge_query.len(),
                max: MAX_SEARCH_PARAMS_LEN,
            })
        );

        // Query param entry count limit
        let mut pairs_query = String::new();
        for i in 0..=MAX_SEARCH_PARAMS_COUNT {
            if i > 0 {
                pairs_query.push('&');
            }
            pairs_query.push_str(&format!("k{i}=v"));
        }
        assert!(matches!(
            ParsedSearchParams::try_parse(&pairs_query),
            Err(UrlError::TooManyParams { .. })
        ));

        // URL path segments count limit
        let mut deep_path = String::from("http://example.com");
        for _ in 0..=MAX_URL_PATH_SEGMENTS {
            deep_path.push_str("/seg");
        }
        assert!(matches!(
            ParsedUrl::parse(&deep_path, None),
            Err(UrlError::TooManyPathSegments { .. })
        ));
    }
}
