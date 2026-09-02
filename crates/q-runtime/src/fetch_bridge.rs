//! M4A-009-C: the runtime's outbound fetch dialer. Implements the engine's
//! [`FetchDialer`] trait over the shared outbound pool (M28-003) with the
//! frozen ADR-0033 fetch policy applied at every hop: scheme allowlist,
//! metadata-name denial, SSRF address classification (deny-by-default),
//! bounded redirects with cross-origin credential stripping, a total
//! deadline, and a response-body cap.
//!
//! The reference bridge surfaces textual bodies; binary payloads are outside
//! this packet (bytes streaming is M28-006 infrastructure).

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use http_body_util::BodyExt;
use hyper::header::LOCATION;
use hyper::{Method, Request as HyperRequest};
use q_capabilities::fetch_policy::{
    headers_surviving_redirect, is_cross_origin_redirect, resolve_and_validate, FetchPolicy,
    RedirectLimiter, RedirectOutcome,
};
use q_capabilities::FetchPolicyError;

use crate::fetch_stack::shared_pool;
use q_engine_quickjs::FetchDialer;

/// Textual body cap applied on top of the policy's response-body bound.
const RELAY_BODY_CAP: u64 = 16 * 1024 * 1024;

pub struct PoolFetchDialer {
    policy: FetchPolicy,
    total_deadline: Duration,
}

impl PoolFetchDialer {
    pub fn new(policy: FetchPolicy) -> Self {
        let total_deadline = Duration::from_millis(policy.timeouts().total_deadline_ms);
        PoolFetchDialer {
            policy,
            total_deadline,
        }
    }
}

impl FetchDialer for PoolFetchDialer {
    fn dial(
        &self,
        method: String,
        url: String,
        headers_json: String,
        body_json: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send>> {
        let policy = self.policy.clone();
        let total = self.total_deadline;
        Box::pin(async move {
            match tokio::time::timeout(
                total,
                dial_following_redirects(policy, method, url, headers_json, body_json),
            )
            .await
            {
                Ok(inner) => inner,
                Err(_) => Err(format!(
                    "fetch deadline exceeded (total_deadline_ms={})",
                    total.as_millis()
                )),
            }
        })
    }
}

fn policy_err(e: FetchPolicyError) -> String {
    format!("fetch policy rejected request: {e}")
}

/// Blocking DNS resolution for the SSRF gate (runs on the blocking pool).
fn resolve_host(host: &str) -> Result<Vec<std::net::IpAddr>, String> {
    use std::net::ToSocketAddrs;
    match (host, 0u16).to_socket_addrs() {
        Ok(addrs) => Ok(addrs.map(|a| a.ip()).collect()),
        Err(e) => Err(format!("dns resolution failed for {host}: {e}")),
    }
}

fn url_parts(target: &str) -> Option<(String, String)> {
    let scheme_end = target.find("://")?;
    let scheme = target[..scheme_end].to_ascii_lowercase();
    let rest = &target[scheme_end + 3..];
    let authority_end = rest.find('/').unwrap_or(rest.len());
    let authority = &rest[..authority_end];
    let host = authority.rsplit_once('@').map_or(authority, |(_, h)| h);
    let host = host.split_once(':').map_or(host, |(h, _)| h);
    if scheme.is_empty() || host.is_empty() {
        return None;
    }
    Some((scheme, host.to_ascii_lowercase()))
}

async fn validated_target(policy: &FetchPolicy, target: &str) -> Result<(), String> {
    let (scheme, host) =
        url_parts(target).ok_or_else(|| format!("fetch url has no scheme/host: {target}"))?;
    policy.check_scheme(&scheme).map_err(policy_err)?;
    let policy2 = policy.clone();
    let addrs =
        tokio::task::spawn_blocking(move || resolve_and_validate(&policy2, &host, resolve_host))
            .await
            .map_err(|e| format!("resolver task failed: {e}"))?
            .map_err(policy_err)?;
    let _ = addrs; // validated; the pooled connector performs its own dial
    Ok(())
}

fn dial_following_redirects(
    policy: FetchPolicy,
    method: String,
    url: String,
    headers_json: String,
    body: Option<String>,
) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send>> {
    Box::pin(async move {
        let mut limiter = RedirectLimiter::new(policy.clone());
        limiter.seed_target(&url);
        let mut current_url = url;
        let current_method = method;
        let mut current_headers = headers_json;
        let current_body = body;

        loop {
            validated_target(&policy, &current_url).await?;

            let permit = shared_pool()
                .try_acquire_permit()
                .map_err(|e| e.to_string())?;
            let client = shared_pool().client();

            let http_method = Method::from_bytes(current_method.as_bytes())
                .map_err(|e| format!("invalid method {current_method}: {e}"))?;
            let uri = current_url
                .parse::<hyper::Uri>()
                .map_err(|e| format!("invalid url {current_url}: {e}"))?;

            let mut builder = HyperRequest::builder().method(http_method.clone()).uri(uri);
            let header_map: serde_json::Map<String, serde_json::Value> =
                serde_json::from_str(&current_headers)
                    .map_err(|e| format!("invalid header set: {e}"))?;
            for (k, v) in &header_map {
                let name = hyper::header::HeaderName::from_bytes(k.as_bytes())
                    .map_err(|e| format!("invalid header name {k}: {e}"))?;
                let value = v
                    .as_str()
                    .ok_or_else(|| format!("invalid header value for {k}"))?;
                let hval = hyper::header::HeaderValue::from_str(value)
                    .map_err(|e| format!("invalid header value for {k}: {e}"))?;
                builder = builder.header(name, hval);
            }

            let _has_body = matches!(current_method.as_str(), "POST" | "PUT" | "PATCH" | "DELETE")
                && current_body.as_deref().is_some();
            let req: HyperRequest<http_body_util::Empty<bytes::Bytes>> = builder
                .body(http_body_util::Empty::<bytes::Bytes>::new())
                .map_err(|e| format!("request build failed: {e}"))?;

            let response = client
                .request(req)
                .await
                .map_err(|e| format!("upstream request failed: {e}"))?;
            drop(permit);

            let status = response.status();
            if status.is_redirection() {
                let location = response
                    .headers()
                    .get(LOCATION)
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string());
                let _ = response.into_body();
                let Some(location) = location else {
                    return Err("redirect response without Location".into());
                };
                // The reference bridge follows absolute Location targets only.
                let next = if location.contains("://") {
                    location
                } else {
                    return Err(format!(
                        "relative redirect Location is not followed by this bridge: {location}"
                    ));
                };
                let from = current_url.clone();
                // Cross-origin hops must not leak credential headers.
                if is_cross_origin_redirect(&from, &next) {
                    let keys: Vec<&str> = header_map.keys().map(|k| k.as_str()).collect();
                    let surviving: Vec<String> = headers_surviving_redirect(&from, &next, keys);
                    let mut filtered = serde_json::Map::new();
                    for (k, v) in &header_map {
                        let lower = k.to_ascii_lowercase();
                        if surviving.iter().any(|s| s.eq_ignore_ascii_case(&lower)) {
                            filtered.insert(k.clone(), v.clone());
                        }
                    }
                    current_headers = serde_json::Value::Object(filtered).to_string();
                }
                let hop = limiter
                    .follow_hop(&from, &next, resolve_host)
                    .map_err(policy_err)?;
                match hop.outcome {
                    RedirectOutcome::Surface => {
                        // Manual policy: surface the 3xx to the caller as-is.
                        return Ok(wire_json(
                            status.as_u16(),
                            status.canonical_reason().unwrap_or(""),
                            &header_map,
                            String::new(),
                            &from,
                        ));
                    }
                    RedirectOutcome::Follow => {
                        current_url = next;
                        continue;
                    }
                }
            }

            let body_cap = policy.max_response_body_bytes().min(RELAY_BODY_CAP);
            let response_headers: serde_json::Map<String, serde_json::Value> =
                response_headers_json(response.headers());
            let mut body_bytes = response.into_body();
            let mut collected: Vec<u8> = Vec::new();
            loop {
                let frame = body_bytes
                    .frame()
                    .await
                    .transpose()
                    .map_err(|e| format!("upstream body read failed: {e}"))?;
                let Some(frame) = frame else { break };
                let data = frame
                    .into_data()
                    .map_err(|_| "upstream body frame unsupported".to_string())?;
                collected.extend_from_slice(&data);
                if collected.len() as u64 > body_cap {
                    return Err(format!(
                        "upstream body exceeds the {} byte response bound",
                        body_cap
                    ));
                }
            }
            let text = String::from_utf8_lossy(&collected).into_owned();
            return Ok(wire_json(
                status.as_u16(),
                status.canonical_reason().unwrap_or(""),
                &response_headers,
                text,
                &current_url,
            ));
        }
    })
}

fn response_headers_json(headers: &hyper::HeaderMap) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    for (name, value) in headers {
        let key = name.as_str().to_ascii_lowercase();
        let val = value.to_str().unwrap_or("").to_string();
        map.insert(key, serde_json::Value::String(val));
    }
    map
}

fn wire_json(
    status: u16,
    status_text: &str,
    headers: &serde_json::Map<String, serde_json::Value>,
    body: String,
    url: &str,
) -> String {
    serde_json::json!({
        "status": status,
        "statusText": status_text,
        "headers": serde_json::Value::Object(headers.clone()),
        "body": body,
        "url": url,
    })
    .to_string()
}
