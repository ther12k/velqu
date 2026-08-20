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

pub const METHOD_GET: usize = 0;
pub const METHOD_POST: usize = 1;
pub const METHOD_PUT: usize = 2;
pub const METHOD_PATCH: usize = 3;
pub const METHOD_DELETE: usize = 4;
pub const METHOD_OPTIONS: usize = 5;
pub const METHOD_HEAD: usize = 6;
pub const METHOD_COUNT: usize = 7;

#[inline]
pub fn method_to_index(method: &str) -> Option<usize> {
    q_pack::method_index(method)
}

pub fn index_to_method(idx: usize) -> &'static str {
    match idx {
        METHOD_GET => "GET",
        METHOD_POST => "POST",
        METHOD_PUT => "PUT",
        METHOD_PATCH => "PATCH",
        METHOD_DELETE => "DELETE",
        METHOD_OPTIONS => "OPTIONS",
        METHOD_HEAD => "HEAD",
        _ => "UNKNOWN",
    }
}

#[derive(Debug, Clone, Default)]
pub struct Terminal {
    pub method_mask: u16,
    pub route_by_method: [Option<usize>; METHOD_COUNT],
}

#[derive(Debug, Clone)]
pub struct StaticEdge {
    pub segment: String,
    pub target_node: usize,
}

#[derive(Debug, Clone, Default)]
pub struct RouterNode {
    pub static_edges: Vec<StaticEdge>,
    pub param_edge: Option<(String, usize)>,
    pub wildcard_edge: Option<usize>,
    pub terminal: Option<Terminal>,
}

#[derive(Debug, Clone)]
pub struct CompiledRoute {
    pub index: usize,
    pub route_id: q_engine::RouteId,
    pub method: String,
    pub segments: Vec<PathSegment>,
    pub param_names: Vec<String>,
    pub has_params: bool,
    pub plan: Option<q_pack::RoutePlanDecl>,
    pub handler_id: Option<q_engine::HandlerId>,
    pub policy_id: Option<q_engine::PolicyId>,
    pub policy_handler_id: Option<q_engine::HandlerId>,
    pub params_schema_id: Option<q_engine::SchemaId>,
    pub query_schema_id: Option<q_engine::SchemaId>,
    pub headers_schema_id: Option<q_engine::SchemaId>,
    pub body_schema_id: Option<q_engine::SchemaId>,
    pub default_status: u16,
    pub allowed_statuses: Vec<u16>,
    pub response_strategy: q_engine::ResponseStrategy,
    pub deadline_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchResult {
    /// Matched route identity (G0-r1): `route_id` is the canonical numeric
    /// RouteId that directly indexes the dense route/plan vector;
    /// `route_index` is retained as the internal Vec position (equal value).
    /// M24-004-A: captures are byte ranges into the resolved path — no owned
    /// parameter string exists until `materialize_params` is called.
    Found {
        route_id: q_engine::RouteId,
        route_index: usize,
        param_ranges: Vec<(u32, u32)>,
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
    nodes: Vec<RouterNode>,
}

impl Router {
    /// Build from pack routes into an in-memory terminal automaton (M2.3-r2); rejects collisions.
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
            let (
                handler_id,
                policy_id,
                policy_handler_id,
                params_schema_id,
                query_schema_id,
                headers_schema_id,
                body_schema_id,
                default_status,
                allowed_statuses,
                response_strategy,
                deadline_ms,
            ) = if let Some(p) = &r.plan {
                let strategy = match p.response_strategy {
                    q_pack::Strategy::Native => q_engine::ResponseStrategy::Native,
                    q_pack::Strategy::Js => q_engine::ResponseStrategy::Js,
                };
                (
                    Some(q_engine::HandlerId(p.handler_id)),
                    p.policy_id.map(q_engine::PolicyId),
                    p.policy_handler_id.map(q_engine::HandlerId),
                    p.params_schema_id.map(q_engine::SchemaId),
                    p.query_schema_id.map(q_engine::SchemaId),
                    p.headers_schema_id.map(q_engine::SchemaId),
                    p.body_schema_id.map(q_engine::SchemaId),
                    p.default_status,
                    p.allowed_statuses.clone(),
                    strategy,
                    p.deadline_ms,
                )
            } else {
                let default_status = r
                    .responses
                    .contains_key("200")
                    .then_some(200)
                    .or_else(|| r.responses.keys().next().and_then(|k| k.parse().ok()))
                    .unwrap_or(200);
                let allowed_statuses: Vec<u16> =
                    r.responses.keys().filter_map(|k| k.parse().ok()).collect();
                let response_strategy = match r.responses.get(&default_status.to_string()) {
                    Some(decl) => match decl.strategy {
                        q_pack::Strategy::Native => q_engine::ResponseStrategy::Native,
                        q_pack::Strategy::Js => q_engine::ResponseStrategy::Js,
                    },
                    None => q_engine::ResponseStrategy::Js,
                };
                (
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    default_status,
                    allowed_statuses,
                    response_strategy,
                    r.deadline_ms,
                )
            };

            let param_names = r
                .path_segments
                .iter()
                .filter_map(|s| match s.kind {
                    SegKind::Param => Some(s.value.clone()),
                    _ => None,
                })
                .collect();

            compiled.push(CompiledRoute {
                index: i,
                route_id: q_engine::RouteId(i as u32),
                method: r.method.clone(),
                segments: r.path_segments.clone(),
                param_names,
                has_params: r.path_segments.iter().any(|s| s.kind != SegKind::Static),
                plan: r.plan.clone(),
                handler_id,
                policy_id,
                policy_handler_id,
                params_schema_id,
                query_schema_id,
                headers_schema_id,
                body_schema_id,
                default_status,
                allowed_statuses,
                response_strategy,
                deadline_ms,
            });
        }

        // Build terminal automaton
        let mut nodes = vec![RouterNode::default()];
        for c in &compiled {
            let mut curr = 0;
            for seg in &c.segments {
                match seg.kind {
                    SegKind::Static => {
                        let existing = nodes[curr]
                            .static_edges
                            .iter()
                            .find(|e| e.segment == seg.value)
                            .map(|e| e.target_node);
                        if let Some(t) = existing {
                            curr = t;
                        } else {
                            let next = nodes.len();
                            nodes.push(RouterNode::default());
                            nodes[curr].static_edges.push(StaticEdge {
                                segment: seg.value.clone(),
                                target_node: next,
                            });
                            curr = next;
                        }
                    }
                    SegKind::Param => {
                        if let Some((_, t)) = nodes[curr].param_edge {
                            curr = t;
                        } else {
                            let next = nodes.len();
                            nodes.push(RouterNode::default());
                            nodes[curr].param_edge = Some((seg.value.clone(), next));
                            curr = next;
                        }
                    }
                    SegKind::Wildcard => {
                        if let Some(t) = nodes[curr].wildcard_edge {
                            curr = t;
                        } else {
                            let next = nodes.len();
                            nodes.push(RouterNode::default());
                            nodes[curr].wildcard_edge = Some(next);
                            curr = next;
                        }
                    }
                }
            }
            let terminal = nodes[curr].terminal.get_or_insert_with(Terminal::default);
            let method_upper = c.method.to_ascii_uppercase();
            let Some(m_idx) = method_to_index(&method_upper) else {
                continue;
            };
            if (terminal.method_mask & (1 << m_idx)) != 0 {
                let prev_idx = terminal.route_by_method[m_idx].unwrap();
                return Err(RouterError::Collision {
                    method: c.method.clone(),
                    a: routes[prev_idx].path.clone(),
                    b: routes[c.index].path.clone(),
                });
            }
            terminal.method_mask |= 1 << m_idx;
            terminal.route_by_method[m_idx] = Some(c.index);
        }

        Ok(Router {
            routes: compiled,
            nodes,
        })
    }

    /// Load router from pack. In numeric mode with precompiled automaton,
    /// loads nodes and compiled routes directly with ZERO runtime path parsing or collision scans.
    pub fn from_pack(pack: &q_pack::QPack) -> Result<Router, RouterError> {
        if let Some(ref serialized) = pack.router {
            let mut compiled = Vec::with_capacity(pack.routes.len());
            for (i, r) in pack.routes.iter().enumerate() {
                let (
                    handler_id,
                    policy_id,
                    policy_handler_id,
                    params_schema_id,
                    query_schema_id,
                    headers_schema_id,
                    body_schema_id,
                    default_status,
                    allowed_statuses,
                    response_strategy,
                    deadline_ms,
                ) = if let Some(p) = &r.plan {
                    let strategy = match p.response_strategy {
                        q_pack::Strategy::Native => q_engine::ResponseStrategy::Native,
                        q_pack::Strategy::Js => q_engine::ResponseStrategy::Js,
                    };
                    (
                        Some(q_engine::HandlerId(p.handler_id)),
                        p.policy_id.map(q_engine::PolicyId),
                        p.policy_handler_id.map(q_engine::HandlerId),
                        p.params_schema_id.map(q_engine::SchemaId),
                        p.query_schema_id.map(q_engine::SchemaId),
                        p.headers_schema_id.map(q_engine::SchemaId),
                        p.body_schema_id.map(q_engine::SchemaId),
                        p.default_status,
                        p.allowed_statuses.clone(),
                        strategy,
                        p.deadline_ms,
                    )
                } else {
                    let default_status = r
                        .responses
                        .contains_key("200")
                        .then_some(200)
                        .or_else(|| r.responses.keys().next().and_then(|k| k.parse().ok()))
                        .unwrap_or(200);
                    let allowed_statuses: Vec<u16> =
                        r.responses.keys().filter_map(|k| k.parse().ok()).collect();
                    let response_strategy = match r.responses.get(&default_status.to_string()) {
                        Some(decl) => match decl.strategy {
                            q_pack::Strategy::Native => q_engine::ResponseStrategy::Native,
                            q_pack::Strategy::Js => q_engine::ResponseStrategy::Js,
                        },
                        None => q_engine::ResponseStrategy::Js,
                    };
                    (
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        default_status,
                        allowed_statuses,
                        response_strategy,
                        r.deadline_ms,
                    )
                };

                let param_names = r
                    .path_segments
                    .iter()
                    .filter_map(|s| match s.kind {
                        SegKind::Param => Some(s.value.clone()),
                        _ => None,
                    })
                    .collect();

                compiled.push(CompiledRoute {
                    index: i,
                    route_id: q_engine::RouteId(i as u32),
                    method: r.method.clone(),
                    segments: r.path_segments.clone(),
                    param_names,
                    has_params: r.path_segments.iter().any(|s| s.kind != SegKind::Static),
                    plan: r.plan.clone(),
                    handler_id,
                    policy_id,
                    policy_handler_id,
                    params_schema_id,
                    query_schema_id,
                    headers_schema_id,
                    body_schema_id,
                    default_status,
                    allowed_statuses,
                    response_strategy,
                    deadline_ms,
                });
            }

            let nodes: Vec<RouterNode> = serialized
                .nodes
                .iter()
                .map(|sn| RouterNode {
                    static_edges: sn
                        .static_edges
                        .iter()
                        .map(|se| StaticEdge {
                            segment: se.segment.clone(),
                            target_node: se.target_node,
                        })
                        .collect(),
                    param_edge: sn.param_edge.map(|target| (String::new(), target)),
                    wildcard_edge: sn.wildcard_edge,
                    terminal: sn.terminal.as_ref().map(|st| Terminal {
                        method_mask: st.method_mask,
                        route_by_method: st.route_by_method,
                    }),
                })
                .collect();

            Ok(Router {
                routes: compiled,
                nodes,
            })
        } else {
            Router::build(&pack.routes)
        }
    }

    /// Load directly from a serialized automaton + routes (no pack object).
    /// This is the pure form used by the reference-vs-serialized property test.
    pub fn from_serialized(
        routes: &[RouteEntry],
        serialized: &q_pack::SerializedRouter,
    ) -> Result<Router, RouterError> {
        let mut compiled = Vec::with_capacity(routes.len());
        for (i, r) in routes.iter().enumerate() {
            let strategy = match r
                .plan
                .as_ref()
                .map(|p| p.response_strategy)
                .unwrap_or(q_pack::Strategy::Js)
            {
                q_pack::Strategy::Native => q_engine::ResponseStrategy::Native,
                q_pack::Strategy::Js => q_engine::ResponseStrategy::Js,
            };
            let default_status = r
                .responses
                .contains_key("200")
                .then_some(200)
                .or_else(|| r.responses.keys().next().and_then(|k| k.parse().ok()))
                .unwrap_or(200);
            let (
                handler_id,
                policy_id,
                policy_handler_id,
                params_schema_id,
                query_schema_id,
                headers_schema_id,
                body_schema_id,
                plan_status,
                plan_statuses,
                plan_deadline,
            ) = match &r.plan {
                Some(p) => (
                    Some(q_engine::HandlerId(p.handler_id)),
                    p.policy_id.map(q_engine::PolicyId),
                    p.policy_handler_id.map(q_engine::HandlerId),
                    p.params_schema_id.map(q_engine::SchemaId),
                    p.query_schema_id.map(q_engine::SchemaId),
                    p.headers_schema_id.map(q_engine::SchemaId),
                    p.body_schema_id.map(q_engine::SchemaId),
                    p.default_status,
                    p.allowed_statuses.clone(),
                    p.deadline_ms,
                ),
                None => (
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    default_status,
                    r.responses.keys().filter_map(|k| k.parse().ok()).collect(),
                    r.deadline_ms,
                ),
            };
            let param_names = r
                .path_segments
                .iter()
                .filter_map(|s| match s.kind {
                    SegKind::Param => Some(s.value.clone()),
                    _ => None,
                })
                .collect();
            compiled.push(CompiledRoute {
                index: i,
                route_id: q_engine::RouteId(i as u32),
                method: r.method.clone(),
                segments: r.path_segments.clone(),
                param_names,
                has_params: r.path_segments.iter().any(|s| s.kind != SegKind::Static),
                plan: r.plan.clone(),
                handler_id,
                policy_id,
                policy_handler_id,
                params_schema_id,
                query_schema_id,
                headers_schema_id,
                body_schema_id,
                default_status: plan_status,
                allowed_statuses: plan_statuses,
                response_strategy: strategy,
                deadline_ms: plan_deadline,
            });
        }
        let nodes: Vec<RouterNode> = serialized
            .nodes
            .iter()
            .map(|sn| RouterNode {
                static_edges: sn
                    .static_edges
                    .iter()
                    .map(|se| StaticEdge {
                        segment: se.segment.clone(),
                        target_node: se.target_node,
                    })
                    .collect(),
                param_edge: sn.param_edge.map(|target| (String::new(), target)),
                wildcard_edge: sn.wildcard_edge,
                terminal: sn.terminal.as_ref().map(|st| Terminal {
                    method_mask: st.method_mask,
                    route_by_method: st.route_by_method,
                }),
            })
            .collect();
        Ok(Router {
            routes: compiled,
            nodes,
        })
    }

    /// Serialize this router's automaton (used by the property test to round-trip
    /// a reference-built router through the serialized form).
    pub fn to_serialized(&self) -> q_pack::SerializedRouter {
        q_pack::SerializedRouter {
            nodes: self
                .nodes
                .iter()
                .map(|n| q_pack::SerializedRouterNode {
                    static_edges: n
                        .static_edges
                        .iter()
                        .map(|e| q_pack::SerializedStaticEdge {
                            segment: e.segment.clone(),
                            target_node: e.target_node,
                        })
                        .collect(),
                    param_edge: n.param_edge.as_ref().map(|(_, t)| *t),
                    wildcard_edge: n.wildcard_edge,
                    terminal: n.terminal.as_ref().map(|t| q_pack::SerializedTerminal {
                        method_mask: t.method_mask,
                        route_by_method: t.route_by_method,
                    }),
                })
                .collect(),
        }
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    /// Retrieve the precompiled numeric route plan for a matched route index
    #[inline]
    pub fn route_plan(&self, route_index: usize) -> Option<&q_pack::RoutePlanDecl> {
        self.routes.get(route_index).and_then(|r| r.plan.as_ref())
    }

    /// Retrieve the compiled route entry for a matched route index
    #[inline]
    pub fn compiled_route(&self, route_index: usize) -> Option<&CompiledRoute> {
        self.routes.get(route_index)
    }

    fn search_route(
        &self,
        node_idx: usize,
        seg_idx: usize,
        segments: &[&str],
        seg_ranges: &[(u32, u32)],
        eff_method_idx: usize,
        captures: &mut Vec<(u32, u32)>,
    ) -> Option<(usize, usize)> {
        let node = &self.nodes[node_idx];
        if seg_idx == segments.len() {
            if let Some(ref t) = node.terminal {
                if let Some(r_idx) = t.route_by_method[eff_method_idx] {
                    return Some((node_idx, r_idx));
                }
            }
            if let Some(w_target) = node.wildcard_edge {
                if let Some(ref t) = self.nodes[w_target].terminal {
                    if let Some(r_idx) = t.route_by_method[eff_method_idx] {
                        return Some((w_target, r_idx));
                    }
                }
            }
            return None;
        }

        let curr_seg = segments[seg_idx];

        // 1. Try static edge for this method
        if let Some(edge) = node.static_edges.iter().find(|e| e.segment == curr_seg) {
            if let Some(found) = self.search_route(
                edge.target_node,
                seg_idx + 1,
                segments,
                seg_ranges,
                eff_method_idx,
                captures,
            ) {
                return Some(found);
            }
        }

        // 2. Try param edge if static edge didn't match for this method.
        //    M24-004-A: record the segment's byte range — no owned string.
        if let Some((_, target)) = node.param_edge {
            captures.push(seg_ranges[seg_idx]);
            if let Some(found) = self.search_route(
                target,
                seg_idx + 1,
                segments,
                seg_ranges,
                eff_method_idx,
                captures,
            ) {
                return Some(found);
            }
            captures.pop();
        }

        // 3. Try wildcard edge if param edge didn't match for this method
        if let Some(w_target) = node.wildcard_edge {
            if let Some(ref t) = self.nodes[w_target].terminal {
                if let Some(r_idx) = t.route_by_method[eff_method_idx] {
                    return Some((w_target, r_idx));
                }
            }
        }

        None
    }

    fn collect_available_methods(
        &self,
        node_idx: usize,
        seg_idx: usize,
        segments: &[&str],
        mask: &mut u16,
    ) {
        let node = &self.nodes[node_idx];
        if seg_idx == segments.len() {
            if let Some(ref t) = node.terminal {
                *mask |= t.method_mask;
            }
            if let Some(w_target) = node.wildcard_edge {
                if let Some(ref t) = self.nodes[w_target].terminal {
                    *mask |= t.method_mask;
                }
            }
            return;
        }

        let curr_seg = segments[seg_idx];
        if let Some(edge) = node.static_edges.iter().find(|e| e.segment == curr_seg) {
            self.collect_available_methods(edge.target_node, seg_idx + 1, segments, mask);
        }
        if let Some((_, target)) = node.param_edge {
            self.collect_available_methods(target, seg_idx + 1, segments, mask);
        }
        if let Some(w_target) = node.wildcard_edge {
            if let Some(ref t) = self.nodes[w_target].terminal {
                *mask |= t.method_mask;
            }
        }
    }

    /// Match a method + already-split path segments. M24-004-A: the segments
    /// are rejoined into a synthetic path so captures can be expressed as
    /// byte ranges against it; the request hot path uses `resolve`, which
    /// computes ranges against the original URI path with no join.
    pub fn match_path(&self, method: &str, segments: &[&str]) -> MatchResult {
        let mut joined = String::new();
        for seg in segments {
            joined.push('/');
            joined.push_str(seg);
        }
        self.resolve(method, &joined)
    }

    /// Full match including 405 handling: one single traversal into the
    /// terminal automaton. Captures are returned as byte ranges into `path`.
    pub fn resolve(&self, method: &str, path: &str) -> MatchResult {
        let method_upper = method.to_ascii_uppercase();
        let head = method_upper == "HEAD";
        let eff_method_idx = if head {
            METHOD_GET
        } else {
            match method_to_index(&method_upper) {
                Some(idx) => idx,
                None => return MatchResult::NotFound,
            }
        };

        // Split while tracking each segment's byte range in the original path
        // (segments stay on '/' boundaries, hence char boundaries).
        let mut segments: Vec<&str> = Vec::new();
        let mut seg_ranges: Vec<(u32, u32)> = Vec::new();
        let bytes = path.as_bytes();
        let mut start = 0usize;
        for (i, b) in bytes.iter().enumerate() {
            if *b == b'/' {
                if i > start {
                    segments.push(&path[start..i]);
                    seg_ranges.push((start as u32, i as u32));
                }
                start = i + 1;
            }
        }
        if start < bytes.len() {
            segments.push(&path[start..]);
            seg_ranges.push((start as u32, bytes.len() as u32));
        }

        let mut captures: Vec<(u32, u32)> = Vec::new();
        if let Some((_, route_index)) =
            self.search_route(0, 0, &segments, &seg_ranges, eff_method_idx, &mut captures)
        {
            let route = &self.routes[route_index];
            return MatchResult::Found {
                route_id: route.route_id,
                route_index,
                param_ranges: captures,
                head,
            };
        }

        // No route matched for this method: check if any method matches this path shape for 405
        let mut method_mask = 0u16;
        self.collect_available_methods(0, 0, &segments, &mut method_mask);
        if method_mask != 0 {
            let mut allow = Vec::new();
            for m_idx in 0..METHOD_COUNT {
                if (method_mask & (1 << m_idx)) != 0 {
                    allow.push(index_to_method(m_idx).to_string());
                }
            }
            if (method_mask & (1 << METHOD_GET)) != 0 && !allow.contains(&"HEAD".to_string()) {
                allow.push("HEAD".to_string());
            }
            allow.sort();
            MatchResult::MethodNotAllowed { allow }
        } else {
            MatchResult::NotFound
        }
    }

    /// Materialize named parameter strings from capture ranges. This is the
    /// ONLY allocation path for parameter values; call it when validation or
    /// JavaScript access actually needs them. Policy (explicit, tested):
    /// values are the raw path bytes between the recorded range boundaries —
    /// percent sequences are NOT decoded here; decoding/byte-level validation
    /// belongs to the schema layer (M24-004-C/D).
    pub fn materialize_params(
        &self,
        route_index: usize,
        path: &str,
        ranges: &[(u32, u32)],
    ) -> Vec<(String, String)> {
        let Some(route) = self.routes.get(route_index) else {
            return Vec::new();
        };
        route
            .param_names
            .iter()
            .zip(ranges)
            .map(|(name, (start, end))| {
                let start = *start as usize;
                let end = *end as usize;
                let value = path.get(start..end).unwrap_or_default();
                (name.clone(), value.to_string())
            })
            .collect()
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
            plan: None,
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
            MatchResult::Found {
                route_index,
                param_ranges,
                ..
            } => {
                // M24-004-A: capture is a byte range into the path; the owned
                // string exists only after materialize_params
                assert_eq!(param_ranges, vec![(7, 11)]);
                assert_eq!(
                    r.materialize_params(route_index, "/hello/Rafi", &param_ranges),
                    vec![("name".to_string(), "Rafi".to_string())]
                );
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

    /// M24-004-A: captures are Copy byte ranges; the owned parameter string
    /// exists only through materialize_params, and its values match the
    /// previous allocate-at-match behavior exactly (reference parity).
    #[test]
    fn capture_ranges_defer_string_allocation_and_match_reference_values() {
        let routes = vec![route(
            "two",
            "GET",
            vec![st("u"), pm("a"), pm("b")],
            "/u/:a/:b",
        )];
        let r = Router::build(&routes).unwrap();
        let path = "/u/first/second";
        match r.resolve("GET", path) {
            MatchResult::Found {
                route_index,
                param_ranges,
                ..
            } => {
                // ranges point into the ORIGINAL path bytes — no strings here
                assert_eq!(param_ranges, vec![(3, 8), (9, 15)]);
                assert_eq!(
                    r.materialize_params(route_index, path, &param_ranges),
                    vec![
                        ("a".to_string(), "first".to_string()),
                        ("b".to_string(), "second".to_string())
                    ]
                );
                // materializing against a different path yields that path's
                // bytes at the same offsets (ranges are path-relative facts)
                assert_eq!(
                    r.materialize_params(route_index, "/u/XXXXX/YYYYYY", &param_ranges),
                    vec![
                        ("a".to_string(), "XXXXX".to_string()),
                        ("b".to_string(), "YYYYYY".to_string())
                    ]
                );
            }
            other => panic!("{other:?}"),
        }
    }

    /// M24-004-A encoding corpus: multibyte UTF-8, percent sequences, empty
    /// and trailing segments. Policy is explicit: captured values are the RAW
    /// path bytes — percent sequences are not decoded at capture or
    /// materialization (decoding/byte validation is schema-layer scope), so
    /// behavior is consistent for any input and never panics.
    #[test]
    fn capture_ranges_encoding_corpus_is_raw_and_panic_free() {
        let routes = vec![route("p", "GET", vec![pm("v")], "/:v")];
        let r = Router::build(&routes).unwrap();
        let corpus = [
            "/hello%20world",
            "/100%2Fdone",
            "/%ZZinvalid",
            "/日本語パス",
            "/emoji-🚀-here",
            "/trailing/",
            "//leading-empty",
            "/a%2",
        ];
        for path in corpus {
            match r.resolve("GET", path) {
                MatchResult::Found {
                    route_index,
                    param_ranges,
                    ..
                } => {
                    let (start, end) = param_ranges[0];
                    let slice = &path[start as usize..end as usize];
                    let materialized = r.materialize_params(route_index, path, &param_ranges);
                    assert_eq!(materialized.len(), 1);
                    assert_eq!(&materialized[0].0, "v");
                    assert_eq!(&materialized[0].1, slice, "raw-bytes policy for {path}");
                    // percent sequences survive untouched (no decode policy)
                    if path.contains("%20") {
                        assert!(materialized[0].1.contains("%20"));
                    }
                }
                other => panic!("corpus path {path} must match the param route, got {other:?}"),
            }
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

    /// M2.3-r3: Static route for another method must not shadow a valid parameter route
    #[test]
    fn static_route_for_other_method_does_not_shadow_parameter_route() {
        let routes = vec![
            route("get_user", "GET", vec![st("users"), pm("id")], "/users/:id"),
            route("post_me", "POST", vec![st("users"), st("me")], "/users/me"),
        ];
        let r = Router::build(&routes).unwrap();

        // GET /users/me should match get_user with id="me" (NOT return 405 because of POST /users/me)
        match r.resolve("GET", "/users/me") {
            MatchResult::Found {
                route_index,
                param_ranges,
                ..
            } => {
                assert_eq!(route_index, 0);
                assert_eq!(
                    r.materialize_params(route_index, "/users/me", &param_ranges),
                    vec![("id".to_string(), "me".to_string())]
                );
            }
            other => panic!("expected match for GET /users/me, got {other:?}"),
        }

        // POST /users/me matches post_me
        match r.resolve("POST", "/users/me") {
            MatchResult::Found { route_index, .. } => assert_eq!(route_index, 1),
            other => panic!("expected match for POST /users/me, got {other:?}"),
        }
    }

    /// M2.3-r3: Static route for another method must not shadow a valid wildcard route
    #[test]
    fn static_route_for_other_method_does_not_shadow_wildcard_route() {
        let routes = vec![
            route("get_wild", "GET", vec![st("files"), wc()], "/files/*"),
            route(
                "post_static",
                "POST",
                vec![st("files"), st("upload")],
                "/files/upload",
            ),
        ];
        let r = Router::build(&routes).unwrap();

        // GET /files/upload should match get_wild
        match r.resolve("GET", "/files/upload") {
            MatchResult::Found { route_index, .. } => assert_eq!(route_index, 0),
            other => panic!("expected match for GET /files/upload, got {other:?}"),
        }
    }

    /// M2.3-r3: Parameter route for another method must not shadow a valid wildcard route
    #[test]
    fn parameter_route_for_other_method_does_not_shadow_wildcard_route() {
        let routes = vec![
            route("get_wild", "GET", vec![st("api"), wc()], "/api/*"),
            route(
                "post_param",
                "POST",
                vec![st("api"), pm("item")],
                "/api/:item",
            ),
        ];
        let r = Router::build(&routes).unwrap();

        match r.resolve("GET", "/api/abc") {
            MatchResult::Found { route_index, .. } => assert_eq!(route_index, 0),
            other => panic!("expected match for GET /api/abc, got {other:?}"),
        }
    }

    /// M2.3-r3: Routes with same shape but different methods preserve route-specific parameter names
    #[test]
    fn same_shape_different_methods_preserve_route_specific_parameter_names() {
        let routes = vec![
            route("get_user", "GET", vec![st("users"), pm("id")], "/users/:id"),
            route(
                "post_user",
                "POST",
                vec![st("users"), pm("userId")],
                "/users/:userId",
            ),
        ];
        let r = Router::build(&routes).unwrap();

        match r.resolve("GET", "/users/123") {
            MatchResult::Found {
                route_index,
                param_ranges,
                ..
            } => {
                assert_eq!(route_index, 0);
                assert_eq!(
                    r.materialize_params(route_index, "/users/123", &param_ranges),
                    vec![("id".to_string(), "123".to_string())]
                );
            }
            other => panic!("expected match for GET, got {other:?}"),
        }

        match r.resolve("POST", "/users/123") {
            MatchResult::Found {
                route_index,
                param_ranges,
                ..
            } => {
                assert_eq!(route_index, 1);
                assert_eq!(
                    r.materialize_params(route_index, "/users/123", &param_ranges),
                    vec![("userId".to_string(), "123".to_string())]
                );
            }
            other => panic!("expected match for POST, got {other:?}"),
        }
    }

    /// G0-r1: REAL property test — over many GENERATED route tables, the
    /// serialized-automaton matcher must agree with the reference-built
    /// matcher on every dimension: matched route, RouteId, captured params,
    /// 404 vs 405, Allow sets, and HEAD semantics. Also proves the serialized
    /// graph passes SerializedRouter::verify_against.
    #[test]
    fn compiled_and_reference_routers_are_property_equivalent() {
        // Deterministic LCG so failures reproduce exactly
        struct Lcg(u64);
        impl Lcg {
            fn next(&mut self) -> u64 {
                self.0 = self
                    .0
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                self.0 >> 33
            }
            fn below(&mut self, n: u64) -> u64 {
                self.next() % n
            }
        }
        const METHODS: [&str; 6] = ["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"];
        const WORDS: [&str; 9] = [
            "a", "b", "users", "items", "x1", "health", "live", "api", "v2",
        ];

        let mut rng = Lcg(0xC0FFEE);

        for table in 0..60 {
            let n_routes = 1 + rng.below(12) as usize;
            let mut routes: Vec<RouteEntry> = Vec::new();
            let mut shapes: std::collections::BTreeSet<String> = Default::default();
            let mut guard = 0;
            while routes.len() < n_routes && guard < 200 {
                guard += 1;
                let method = METHODS[rng.below(METHODS.len() as u64) as usize];
                let depth = 1 + rng.below(3) as usize;
                let mut segs: Vec<PathSegment> = Vec::new();
                for d in 0..depth {
                    match rng.below(10) {
                        0..=5 => segs.push(st(WORDS[rng.below(WORDS.len() as u64) as usize])),
                        6..=8 => segs.push(pm(&format!("p{d}"))),
                        _ => {
                            if d == depth - 1 {
                                segs.push(wc());
                            } else {
                                segs.push(st("w"));
                            }
                        }
                    }
                }
                // canonical shape (param names erased) for collision detection
                let mut shape = method.to_string();
                for s in &segs {
                    match s.kind {
                        SegKind::Static => shape.push('/'),
                        _ => shape.push('*'),
                    }
                }
                // ensure wildcard is terminal-only (build requirement)
                if segs
                    .iter()
                    .position(|s| s.kind == SegKind::Wildcard)
                    .is_some_and(|p| p + 1 != segs.len())
                {
                    continue;
                }
                if !shapes.insert(shape) {
                    continue; // canonically-equivalent duplicate
                }
                let path = segs
                    .iter()
                    .map(|s| match s.kind {
                        SegKind::Static => format!("/{}", s.value),
                        SegKind::Param => format!("/:{}", s.value),
                        SegKind::Wildcard => "/*".into(),
                    })
                    .collect::<String>();
                routes.push(route(&format!("r{}", routes.len()), method, segs, &path));
            }
            if routes.is_empty() {
                continue;
            }

            // Reference: built from segments. Serialized: compiler-equivalent
            // emission via to_serialized on the reference build, then loaded
            // through from_serialized with NO rebuild.
            let reference = Router::build(&routes).unwrap();
            let serialized = reference.to_serialized();
            serialized
                .verify_against(&routes)
                .unwrap_or_else(|e| panic!("table {table}: serialized graph rejected: {e}"));
            let compiled = Router::from_serialized(&routes, &serialized).expect("from_serialized");

            // Probe paths: every declared route's own path (with param values
            // substituted), wrong-method probes on the same shape, and misses.
            let mut probes: Vec<(String, String)> = Vec::new();
            for (i, r) in routes.iter().enumerate() {
                let mut probe = String::new();
                let mut mi = 0;
                for s in &r.path_segments {
                    match s.kind {
                        SegKind::Static => probe.push_str(&format!("/{}", s.value)),
                        SegKind::Param => {
                            mi += 1;
                            probe.push_str(&format!("/v{i}_{mi}"));
                        }
                        SegKind::Wildcard => probe.push_str("/deep/sub"),
                    }
                }
                probes.push((r.method.clone(), probe.clone()));
                let other = METHODS[(i + 1) % METHODS.len()];
                if other != r.method {
                    probes.push((other.to_string(), probe));
                }
            }
            probes.push(("GET".into(), "/definitely/not/declared".into()));
            probes.push(("POST".into(), format!("/{}", WORDS[rng.below(9) as usize])));

            for (method, path) in probes {
                let expect = reference.resolve(&method, &path);
                let got = compiled.resolve(&method, &path);
                assert_eq!(
                    normalize(expect, &routes, &path, &reference),
                    normalize(got, &routes, &path, &compiled),
                    "table {table}: method {method} path {path} diverged"
                );
            }
        }
    }

    type NormalizedMatch = (
        Option<u32>,
        Vec<(String, String)>,
        bool,
        Option<Vec<String>>,
    );

    fn normalize(
        m: MatchResult,
        _routes: &[RouteEntry],
        path: &str,
        router: &Router,
    ) -> NormalizedMatch {
        match m {
            MatchResult::Found {
                route_id,
                route_index,
                param_ranges,
                head,
                ..
            } => (
                Some(route_id.0),
                router.materialize_params(route_index, path, &param_ranges),
                head,
                None,
            ),
            MatchResult::MethodNotAllowed { allow } => (None, vec![], false, Some(allow)),
            MatchResult::NotFound => (None, vec![], false, None),
        }
    }
}
