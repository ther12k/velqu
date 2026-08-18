//! q-router — native route table consumed from pre-compiled pack segments.
//!
//! The router performs ZERO parsing or compilation at runtime: `PathSegment`
//! arrays come verbatim from the pack (built by the compiler). `Router::build`
//! rejects duplicate and canonically-equivalent routes (COMP-004) and is run
//! once at startup.

use q_pack::{PathSegment, RouteEntry, SegKind};

#[derive(Debug, thiserror::Error)]
pub enum RouterError {
    #[error("route collision: {method} {a} and {b} are canonically equivalent")]
    Collision {
        method: String,
        a: String,
        b: String,
    },
    #[error("route {route}: wildcard must be terminal ({path})")]
    NonTerminalWildcard { route: String, path: String },
    #[error("route {route}: empty path segment in {path}")]
    EmptySegment { route: String, path: String },
}

#[derive(Debug, Clone)]
pub struct CompiledRoute {
    pub index: usize,
    pub method: String,
    pub segments: Vec<PathSegment>,
    pub has_params: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchResult {
    /// route index into the pack's routes vec + extracted path params
    Found {
        route_index: usize,
        params: Vec<(String, String)>,
        head: bool,
    },
    NotFound,
    /// 405: methods allowed for the matched path (sorted, includes HEAD when GET present)
    MethodNotAllowed {
        allow: Vec<String>,
    },
}

#[derive(Debug, Default)]
pub struct Router {
    routes: Vec<CompiledRoute>,
    /// first-static-segment bucket index: HashMap<String, Vec<usize>> + fallback list
    static_index: std::collections::HashMap<String, Vec<usize>>,
    fallback: Vec<usize>,
}

impl Router {
    /// Build from pack routes; rejects collisions and malformed segments.
    pub fn build(routes: &[RouteEntry]) -> Result<Router, RouterError> {
        let mut compiled = Vec::with_capacity(routes.len());
        for (i, r) in routes.iter().enumerate() {
            for (si, seg) in r.path_segments.iter().enumerate() {
                match seg.kind {
                    SegKind::Wildcard if si != r.path_segment_count() - 1 => {
                        return Err(RouterError::NonTerminalWildcard {
                            route: r.id.clone(),
                            path: r.path.clone(),
                        });
                    }
                    SegKind::Static if seg.value.is_empty() => {
                        return Err(RouterError::EmptySegment {
                            route: r.id.clone(),
                            path: r.path.clone(),
                        });
                    }
                    _ => {}
                }
            }
            compiled.push(CompiledRoute {
                index: i,
                method: r.method.clone(),
                segments: r.path_segments.clone(),
                has_params: r.path_segments.iter().any(|s| s.kind != SegKind::Static),
            });
        }
        // canonical collision detection: same method + same segment shape
        // (static values equal; param names ignored) = equivalent route.
        let mut seen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for c in &compiled {
            if c.segments.iter().any(|s| s.kind == SegKind::Wildcard) {
                continue; // wildcards bucket separately; equivalence is "rest of path"
            }
            let mut key = String::with_capacity(32);
            key.push_str(&c.method);
            for seg in &c.segments {
                match seg.kind {
                    SegKind::Static => {
                        key.push('/');
                        key.push_str(&seg.value);
                    }
                    _ => key.push_str("/*"),
                }
            }
            if let Some(prev) = seen.insert(key.clone(), c.index) {
                return Err(RouterError::Collision {
                    method: c.method.clone(),
                    a: routes[prev].path.clone(),
                    b: routes[c.index].path.clone(),
                });
            }
        }
        // build match structures: bucket by leading static segment when present
        let mut static_index: std::collections::HashMap<String, Vec<usize>> =
            std::collections::HashMap::new();
        let mut fallback = Vec::new();
        for c in &compiled {
            match c.segments.first() {
                Some(PathSegment {
                    kind: SegKind::Static,
                    value,
                }) if c.segments.len() > 1 || !c.has_params => {
                    static_index.entry(value.clone()).or_default().push(c.index);
                }
                _ => fallback.push(c.index),
            }
        }
        Ok(Router {
            routes: compiled,
            static_index,
            fallback,
        })
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    /// Match a method + already-split path segments.
    /// HEAD maps onto GET (head=true); 405 carries `Allow`.
    pub fn match_path(&self, method: &str, segments: &[&str]) -> MatchResult {
        let method = method.to_ascii_uppercase();
        let head = method == "HEAD";
        let eff_method = if head { "GET" } else { method.as_str() };

        let candidates: Vec<usize> = match segments.first() {
            Some(first) => {
                let mut v = self.static_index.get(*first).cloned().unwrap_or_default();
                v.extend(self.fallback.iter().copied());
                v
            }
            None => self.static_index.get("").cloned().unwrap_or_default(),
        };

        #[allow(clippy::type_complexity)]
        let mut best: Option<(usize, Vec<(String, String)>, u32)> = None;
        for idx in candidates {
            let c = &self.routes[idx];
            if c.method != eff_method {
                continue;
            }
            let mut params = Vec::new();
            let mut specificity = 0u32;
            let mut ok = true;
            if c.segments
                .last()
                .is_some_and(|s| s.kind == SegKind::Wildcard)
            {
                // terminal wildcard: prefix segments must match
                if segments.len() < c.segments.len() - 1 {
                    continue;
                }
                for (cs, ps) in c
                    .segments
                    .iter()
                    .take(c.segments.len() - 1)
                    .zip(segments.iter())
                {
                    match cs.kind {
                        SegKind::Static => {
                            if cs.value != *ps {
                                ok = false;
                                break;
                            }
                            specificity += 2;
                        }
                        _ => {
                            params.push((cs.value.clone(), (*ps).to_string()));
                            specificity += 1;
                        }
                    }
                }
            } else {
                if c.segments.len() != segments.len() {
                    continue;
                }
                for (cs, ps) in c.segments.iter().zip(segments.iter()) {
                    match cs.kind {
                        SegKind::Static => {
                            if cs.value != *ps {
                                ok = false;
                                break;
                            }
                            specificity += 2;
                        }
                        SegKind::Param => {
                            if ps.is_empty() {
                                ok = false;
                                break;
                            }
                            params.push((cs.value.clone(), (*ps).to_string()));
                            specificity += 1;
                        }
                        SegKind::Wildcard => {
                            unreachable!("non-terminal wildcards rejected at build")
                        }
                    }
                }
            }
            if !ok {
                continue;
            }
            if best.as_ref().is_none_or(|(_, _, s)| specificity > *s) {
                best = Some((idx, params, specificity));
            }
        }
        if let Some((idx, params, _)) = best {
            return MatchResult::Found {
                route_index: idx,
                params,
                head,
            };
        }
        MatchResult::NotFound
    }

    /// Full match including 405 handling: returns MethodNotAllowed with the
    /// sorted Allow list when the path matches under other method(s).
    pub fn resolve(&self, method: &str, path: &str) -> MatchResult {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let direct = self.match_path(method, &segments);
        if !matches!(direct, MatchResult::NotFound) {
            return direct;
        }
        // any method matches this path shape? → 405
        let mut allow: Vec<String> = Vec::new();
        for m in ["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"] {
            if !matches!(self.match_path(m, &segments), MatchResult::NotFound) {
                allow.push(m.to_string());
            }
        }
        if allow.is_empty() {
            MatchResult::NotFound
        } else {
            let mut with_head = allow.clone();
            if allow.contains(&"GET".to_string()) && !with_head.contains(&"HEAD".to_string()) {
                with_head.push("HEAD".to_string());
            }
            with_head.sort();
            MatchResult::MethodNotAllowed { allow: with_head }
        }
    }
}

trait SegmentCount {
    fn path_segment_count(&self) -> usize;
}
impl SegmentCount for RouteEntry {
    fn path_segment_count(&self) -> usize {
        self.path_segments.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use q_pack::*;
    use std::collections::BTreeMap;

    fn route(id: &str, method: &str, path_segs: Vec<PathSegment>, path: &str) -> RouteEntry {
        RouteEntry {
            id: id.into(),
            module_id: "m".into(),
            method: method.into(),
            path: path.into(),
            path_segments: path_segs,
            handler: "h".into(),
            policy: None,
            params: None,
            query: None,
            body: None,
            headers: None,
            responses: BTreeMap::from([(
                "200".into(),
                ResponseDecl {
                    schema: None,
                    strategy: Strategy::Js,
                    problem: None,
                },
            )]),
            validation_strategy: Strategy::Native,
            native_liveness: None,
            security: vec![],
            capabilities: vec![],
            deadline_ms: 5000,
        }
    }

    fn st(v: &str) -> PathSegment {
        PathSegment {
            kind: SegKind::Static,
            value: v.into(),
        }
    }
    fn pm(v: &str) -> PathSegment {
        PathSegment {
            kind: SegKind::Param,
            value: v.into(),
        }
    }
    fn wc() -> PathSegment {
        PathSegment {
            kind: SegKind::Wildcard,
            value: String::new(),
        }
    }

    #[test]
    fn static_param_wildcard_matching() {
        let routes = vec![
            route(
                "health",
                "GET",
                vec![st("health"), st("live")],
                "/health/live",
            ),
            route(
                "hello",
                "GET",
                vec![st("hello"), pm("name")],
                "/hello/:name",
            ),
            route("files", "GET", vec![st("files"), wc()], "/files/*"),
        ];
        let r = Router::build(&routes).unwrap();
        assert!(matches!(
            r.resolve("GET", "/health/live"),
            MatchResult::Found { .. }
        ));
        match r.resolve("GET", "/hello/Rafi") {
            MatchResult::Found { params, .. } => {
                assert_eq!(params, vec![("name".to_string(), "Rafi".to_string())])
            }
            other => panic!("{other:?}"),
        }
        assert!(matches!(
            r.resolve("GET", "/files/a/b/c"),
            MatchResult::Found { .. }
        ));
        assert!(matches!(r.resolve("GET", "/nope"), MatchResult::NotFound));
    }

    #[test]
    fn method_not_allowed_has_allow_header_set() {
        let routes = vec![route("only-get", "GET", vec![st("x")], "/x")];
        let r = Router::build(&routes).unwrap();
        match r.resolve("POST", "/x") {
            MatchResult::MethodNotAllowed { allow } => {
                assert_eq!(allow, vec!["GET".to_string(), "HEAD".to_string()])
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn head_maps_to_get() {
        let routes = vec![route("g", "GET", vec![st("x")], "/x")];
        let r = Router::build(&routes).unwrap();
        match r.resolve("HEAD", "/x") {
            MatchResult::Found { head, .. } => assert!(head),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn static_preferred_over_param() {
        let routes = vec![
            route("param", "GET", vec![st("a"), pm("id")], "/a/:id"),
            route("static", "GET", vec![st("a"), st("special")], "/a/special"),
        ];
        let r = Router::build(&routes).unwrap();
        match r.resolve("GET", "/a/special") {
            MatchResult::Found { route_index, .. } => assert_eq!(route_index, 1),
            other => panic!("{other:?}"),
        }
        assert!(matches!(
            r.resolve("GET", "/a/other"),
            MatchResult::Found { route_index: 0, .. }
        ));
    }

    #[test]
    fn canonical_collision_rejected() {
        let routes = vec![
            route("a", "GET", vec![st("u"), pm("id")], "/u/:id"),
            route("b", "GET", vec![st("u"), pm("uid")], "/u/:uid"),
        ];
        assert!(matches!(
            Router::build(&routes),
            Err(RouterError::Collision { .. })
        ));
    }

    #[test]
    fn same_path_different_method_ok() {
        let routes = vec![
            route("c", "POST", vec![st("u")], "/u"),
            route("g", "GET", vec![st("u"), pm("id")], "/u/:id"),
        ];
        assert!(Router::build(&routes).is_ok());
    }

    #[test]
    fn non_terminal_wildcard_rejected() {
        let routes = vec![route("w", "GET", vec![wc(), st("x")], "/*/x")];
        assert!(matches!(
            Router::build(&routes),
            Err(RouterError::NonTerminalWildcard { .. })
        ));
    }
}
