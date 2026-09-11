//! #1318 / M6-002 — Router resolution fuzz target.
//!
//! A fixed valid route table is built once; arbitrary (method, path)
//! inputs are resolved against it. `resolve` is a total function on the
//! hot dispatch path: any input yields a match, a 405-class result, or
//! a miss — never a panic or an unbounded walk.

#![no_main]

use libfuzzer_sys::fuzz_target;
use q_pack::{PathSegment, RouteEntry, SegKind};
use q_router::Router;
use std::collections::BTreeMap;
use std::sync::OnceLock;

fn st(v: &str) -> PathSegment {
    PathSegment { kind: SegKind::Static, value: v.into() }
}
fn pm(v: &str) -> PathSegment {
    PathSegment { kind: SegKind::Param, value: v.into() }
}
fn wc() -> PathSegment {
    PathSegment { kind: SegKind::Wildcard, value: String::new() }
}

fn entry(id: &str, method: &str, segs: Vec<PathSegment>, path: &str) -> RouteEntry {
    RouteEntry {
        id: id.into(),
        module_id: "m".into(),
        method: method.into(),
        path: path.into(),
        path_segments: segs,
        handler: "h".into(),
        policy: None,
        params: None,
        query: None,
        body: None,
        headers: None,
        responses: BTreeMap::from([(
            "200".into(),
            q_pack::ResponseDecl { schema: None, strategy: q_pack::Strategy::Js, problem: None },
        )]),
        validation_strategy: q_pack::Strategy::Native,
        native_liveness: None,
        security: vec![],
        capabilities: vec![],
        deadline_ms: 5000,
        plan: None,
    }
}

fn router() -> &'static Router {
    static ROUTER: OnceLock<Router> = OnceLock::new();
    ROUTER.get_or_init(|| {
        let routes = vec![
            entry("a.get", "GET", vec![st("greet"), st("ping")], "/greet/ping"),
            entry("b.get", "GET", vec![st("users"), pm("id")], "/users/:id"),
            entry("c.post", "POST", vec![st("users"), pm("id"), st("notes")], "/users/:id/notes"),
            entry("d.get", "GET", vec![st("files"), wc()], "/files/*"),
            entry("e.del", "DELETE", vec![st("items"), pm("a"), st("rev"), pm("b")], "/items/:a/rev/:b"),
        ];
        Router::build(&routes).expect("fixed valid route table")
    })
}

fuzz_target!(|data: &[u8]| {
    let router = router();
    // Interpret the first byte as a method selector, the rest as a path.
    let (method_byte, rest): (u8, &[u8]) = match data.split_first() {
        Some((b, r)) => (*b, r),
        None => (0u8, &[]),
    };
    let method = match method_byte % 8 {
        0 => "GET",
        1 => "POST",
        2 => "PUT",
        3 => "DELETE",
        4 => "PATCH",
        5 => "HEAD",
        6 => "OPTIONS",
        _ => "\u{0}WEIRD",
    };
    // Paths arrive as arbitrary bytes; force to-lossy UTF-8 like ingress
    // does before any routing decision, and also exercise the raw form.
    let lossy = String::from_utf8_lossy(rest);
    let _ = router.resolve(method, &lossy);
    if let Ok(raw) = std::str::from_utf8(rest) {
        let _ = router.resolve(method, raw);
    }
});
