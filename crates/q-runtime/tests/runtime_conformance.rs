//! Runtime conformance (M1): spawns the ACTUAL q-runtime binary with a
//! fixture pack implementing the frozen benchmark contract, then drives
//! black-box HTTP requests against it. This is runtime-local evidence —
//! unit-local engine tests do not substitute (TRT-005).

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use q_pack::*;
use serde_json::{json, Value};

// ---------------------------------------------------------------- fixture pack

fn fixture_pack() -> q_pack::QPack {
    use q_pack::*;
    use q_schema_runtime::SchemaIr as S;
    use std::collections::BTreeMap;

    let seg = |parts: &[(&str, &str)]| -> Vec<PathSegment> {
        parts
            .iter()
            .map(|(kind, value)| PathSegment {
                kind: match *kind {
                    "s" => SegKind::Static,
                    "p" => SegKind::Param,
                    _ => SegKind::Wildcard,
                },
                value: value.to_string(),
            })
            .collect()
    };
    let mut schemas = BTreeMap::new();
    schemas.insert(
        "sch:fallback.body".to_string(),
        // M25-004-B: explicit fallback marker with NO inner shape — native
        // decode fails closed; only the js strategy lets the raw body cross.
        S::Fallback {
            reason: "explicit".into(),
            inner: None,
        },
    );
    schemas.insert(
        "sch:hello.params".to_string(),
        S::Object {
            properties: BTreeMap::from([(
                "name".into(),
                Box::new(S::String {
                    min_length: Some(1),
                    max_length: Some(60),
                    pattern: None,
                    format: None,
                }),
            )]),
            required: vec!["name".into()],
        },
    );
    schemas.insert(
        "sch:users.create.body".to_string(),
        S::Object {
            properties: BTreeMap::from([
                (
                    "name".into(),
                    Box::new(S::String {
                        min_length: Some(1),
                        max_length: Some(60),
                        pattern: None,
                        format: None,
                    }),
                ),
                (
                    "email".into(),
                    Box::new(S::String {
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        format: Some("email".into()),
                    }),
                ),
            ]),
            required: vec!["name".into(), "email".into()],
        },
    );
    schemas.insert(
        "sch:users.get.params".to_string(),
        S::Object {
            properties: BTreeMap::from([(
                "id".into(),
                Box::new(S::String {
                    min_length: None,
                    max_length: None,
                    pattern: Some("^usr_[0-9]+$".into()),
                    format: None,
                }),
            )]),
            required: vec!["id".into()],
        },
    );
    schemas.insert(
        "sch:async.query".to_string(),
        S::Object {
            properties: BTreeMap::from([(
                "ms".into(),
                Box::new(S::Optional {
                    inner: Box::new(S::Integer {
                        minimum: Some(1),
                        maximum: Some(1000),
                    }),
                    default: Some(json!(10)),
                }),
            )]),
            required: vec![],
        },
    );

    let route = |id: &str,
                 method: &str,
                 path: &str,
                 segments: Vec<PathSegment>,
                 handler: &str,
                 idx: u32,
                 policy_idx: Option<u32>,
                 def_status: u16,
                 statuses: Vec<u16>,
                 needs: FieldNeeds| {
        let plan = RoutePlanDecl {
            route_id: idx,
            handler_id: idx,
            policy_id: policy_idx.map(|_| 0),
            policy_handler_id: policy_idx,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            header_name_ids: vec![],
            query_name_ids: vec![],
            cookie_name_ids: vec![],
            default_status: def_status,
            allowed_statuses: statuses,
            field_needs: needs,
            response_strategy: Strategy::Js,
            validation_fallback_reason: None,
            response_fallback_reason: Some("explicit".into()),
            deadline_ms: 5000,
        };
        RouteEntry {
            id: id.into(),
            module_id: id.split('.').next().unwrap().into(),
            method: method.into(),
            path: path.into(),
            path_segments: segments,
            handler: handler.into(),
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
            plan: Some(plan),
        }
    };

    let mut routes = vec![
        {
            let mut r = route(
                "health.live",
                "GET",
                "/health/live",
                seg(&[("s", "health"), ("s", "live")]),
                "health.live",
                0,
                None,
                200,
                vec![200],
                FieldNeeds::default(),
            );
            r.native_liveness = Some(LivenessSpec {
                status: 200,
                content_type: "application/json".into(),
                body: "{\"status\":\"ok\"}".into(),
            });
            r
        },
        route(
            "js.text",
            "GET",
            "/js-text",
            seg(&[("s", "js-text")]),
            "js.text",
            1,
            None,
            200,
            vec![200],
            FieldNeeds::default(),
        ),
        route(
            "js.json",
            "GET",
            "/js-json",
            seg(&[("s", "js-json")]),
            "js.json",
            2,
            None,
            200,
            vec![200],
            FieldNeeds::default(),
        ),
        {
            let mut r = route(
                "hello.get",
                "GET",
                "/hello/:name",
                seg(&[("s", "hello"), ("p", "name")]),
                "hello.get",
                3,
                None,
                200,
                vec![200, 422],
                FieldNeeds {
                    params: true,
                    query: false,
                    headers: false,
                    body: false,
                },
            );
            r.params = Some(SourceBinding {
                schema: Some("sch:hello.params".into()),
                coerce: Some("path".into()),
                content_type: None,
                limit_bytes: 0,
            });
            r.responses.insert(
                "422".into(),
                ResponseDecl {
                    schema: None,
                    strategy: Strategy::Js,
                    problem: Some("validation".into()),
                },
            );
            r
        },
        {
            let mut r = route(
                "users.create",
                "POST",
                "/users",
                seg(&[("s", "users")]),
                "users.create",
                4,
                None,
                201,
                vec![201, 422],
                FieldNeeds {
                    params: false,
                    query: false,
                    headers: false,
                    body: true,
                },
            );
            r.body = Some(SourceBinding {
                schema: Some("sch:users.create.body".into()),
                coerce: None,
                content_type: Some("application/json".into()),
                limit_bytes: 65_536,
            });
            r.responses = BTreeMap::from([
                (
                    "201".into(),
                    ResponseDecl {
                        schema: None,
                        strategy: Strategy::Js,
                        problem: None,
                    },
                ),
                (
                    "422".into(),
                    ResponseDecl {
                        schema: None,
                        strategy: Strategy::Js,
                        problem: Some("validation".into()),
                    },
                ),
            ]);
            r
        },
        {
            let mut r = route(
                "users.get",
                "GET",
                "/users/:id",
                seg(&[("s", "users"), ("p", "id")]),
                "users.get",
                5,
                Some(10),
                200,
                vec![200, 401, 404],
                FieldNeeds {
                    params: true,
                    query: false,
                    headers: true,
                    body: false,
                },
            );
            r.policy = Some("auth.session".into());
            r.params = Some(SourceBinding {
                schema: Some("sch:users.get.params".into()),
                coerce: Some("path".into()),
                content_type: None,
                limit_bytes: 0,
            });
            r.responses = BTreeMap::from([
                (
                    "200".into(),
                    ResponseDecl {
                        schema: None,
                        strategy: Strategy::Js,
                        problem: None,
                    },
                ),
                (
                    "401".into(),
                    ResponseDecl {
                        schema: None,
                        strategy: Strategy::Js,
                        problem: Some("unauthorized".into()),
                    },
                ),
                (
                    "404".into(),
                    ResponseDecl {
                        schema: None,
                        strategy: Strategy::Js,
                        problem: Some("not-found".into()),
                    },
                ),
            ]);
            r.security = vec![SecurityReq {
                scheme: "bearer".into(),
                header: "authorization".into(),
                problem_status: 401,
            }];
            r
        },
        {
            let mut r = route(
                "async.timer",
                "GET",
                "/async",
                seg(&[("s", "async")]),
                "async.timer",
                6,
                None,
                200,
                vec![200],
                FieldNeeds {
                    params: false,
                    query: true,
                    headers: false,
                    body: false,
                },
            );
            r.query = Some(SourceBinding {
                schema: Some("sch:async.query".into()),
                coerce: Some("query".into()),
                content_type: None,
                limit_bytes: 0,
            });
            r.capabilities = vec!["timer".into()];
            r
        },
        {
            let mut r = route(
                "async.cancel",
                "GET",
                "/cancel",
                seg(&[("s", "cancel")]),
                "async.cancel",
                7,
                None,
                200,
                vec![200],
                FieldNeeds {
                    params: false,
                    query: true,
                    headers: false,
                    body: false,
                },
            );
            r.query = Some(SourceBinding {
                schema: Some("sch:cancel.query".into()),
                coerce: Some("query".into()),
                content_type: None,
                limit_bytes: 0,
            });
            r.capabilities = vec!["timer".into()];
            r
        },
        route(
            "throw.redacted",
            "GET",
            "/throw",
            seg(&[("s", "throw")]),
            "throw.redacted",
            8,
            None,
            200,
            vec![200],
            FieldNeeds::default(),
        ),
        {
            let mut r = route(
                "poison.chain",
                "GET",
                "/poison",
                seg(&[("s", "poison")]),
                "poison.chain",
                9,
                None,
                200,
                vec![200],
                FieldNeeds::default(),
            );
            r.deadline_ms = 200;
            if let Some(ref mut p) = r.plan {
                p.deadline_ms = 200;
            }
            r
        },
        {
            // M25-004-B: body schema is an explicit fallback marker WITHOUT
            // inner — the compiler routes this to validationStrategy "js" and
            // the runtime must hand the raw parsed JSON to the handler.
            let mut r = route(
                "fallback.echo",
                "POST",
                "/fallback",
                seg(&[("s", "fallback")]),
                "fallback.echo",
                10,
                None,
                200,
                vec![200],
                FieldNeeds {
                    params: false,
                    query: false,
                    headers: false,
                    body: true,
                },
            );
            r.body = Some(SourceBinding {
                schema: Some("sch:fallback.body".into()),
                coerce: None,
                content_type: Some("application/json".into()),
                limit_bytes: 65_536,
            });
            r.validation_strategy = Strategy::Js;
            if let Some(ref mut p) = r.plan {
                p.validation_fallback_reason = Some("explicit".into());
            }
            r
        },
        {
            // M25-004-D: a short route deadline must bound the pre-invocation
            // body read. The route shares the fallback.echo RouteHandler —
            // a client that sends headers and then stalls the body stream
            // must get the 504 timeout problem at the deadline while the
            // dropped read cancels the transfer.
            let mut r = route(
                "deadline.body",
                "POST",
                "/deadline-body",
                seg(&[("s", "deadline-body")]),
                "fallback.echo",
                11,
                None,
                200,
                vec![200],
                FieldNeeds {
                    params: false,
                    query: false,
                    headers: false,
                    body: true,
                },
            );
            r.body = Some(SourceBinding {
                schema: Some("sch:fallback.body".into()),
                coerce: None,
                content_type: Some("application/json".into()),
                limit_bytes: 65_536,
            });
            r.validation_strategy = Strategy::Js;
            r.deadline_ms = 200;
            if let Some(ref mut p) = r.plan {
                p.deadline_ms = 200;
                p.handler_id = 10;
                p.validation_fallback_reason = Some("explicit".into());
            }
            r
        },
    ];

    // timer schema reuse for cancel needs a wider maximum
    schemas.insert(
        "sch:cancel.query".to_string(),
        S::Object {
            properties: BTreeMap::from([(
                "ms".into(),
                Box::new(S::Optional {
                    inner: Box::new(S::Integer {
                        minimum: Some(1),
                        maximum: Some(5000),
                    }),
                    default: Some(json!(1000)),
                }),
            )]),
            required: vec![],
        },
    );
    routes
        .iter_mut()
        .find(|r| r.id == "async.cancel")
        .unwrap()
        .query
        .as_mut()
        .unwrap()
        .schema = Some("sch:cancel.query".into());

    let bundle = FIXTURE_BUNDLE.to_string();
    let mut handler_table = BTreeMap::new();
    for r in &routes {
        handler_table.insert(r.handler.clone(), r.handler.clone());
    }
    handler_table.insert("auth.session.check".into(), "auth.session.check".into());

    let functions = vec![
        FunctionDecl {
            id: 0,
            key: "health.live".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "js.text".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 2,
            key: "js.json".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 3,
            key: "hello.get".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 4,
            key: "users.create".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 5,
            key: "users.get".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 6,
            key: "async.timer".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 7,
            key: "async.cancel".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 8,
            key: "throw.redacted".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 9,
            key: "poison.chain".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 10,
            key: "fallback.echo".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 11,
            key: "auth.session.check".into(),
            kind: FunctionKind::PolicyHandler,
        },
    ];

    let mut pack = QPack {
        bundle_prelude: None,
        decoded_bytecode: None,
        header_name_table: Vec::new(),
        query_name_table: Vec::new(),
        cookie_name_table: Vec::new(),
        format_version: q_pack::PACK_FORMAT_VERSION,
        kind: "velqu.qpack".into(),
        runtime_abi: q_pack::RUNTIME_ABI,
        engine: q_pack::EngineRef {
            name: q_pack::ENGINE_NAME.into(),
            version: q_pack::ENGINE_VERSION.into(),
            binding: q_pack::ENGINE_BINDING.into(),
            rquickjs: q_pack::RQUICKJS_VERSION.into(),
            build_hash: q_pack::runtime_build_hash(),
        },
        schema_ir_version: q_pack::SCHEMA_IR_VERSION,
        contract_version: 1,
        contract_hash: String::new(),
        built_by: q_pack::BuiltBy {
            compiler: "manual-fixture".into(),
            typescript: String::new(),
            bun: String::new(),
        },
        app_id: "proof-fixture".into(),
        modules: vec![
            "health".into(),
            "hello".into(),
            "users".into(),
            "async".into(),
        ],
        entry: "app.js".into(),
        bundle_form: None,
        execution_mode: None,
        bundle,
        source_map: None,
        bundle_bytecode: None,
        routes,
        schemas,
        policies: BTreeMap::from([(
            "auth.session".to_string(),
            PolicyEntry {
                id: "auth.session".into(),
                // M2.2.1-r3: policy id != handler-table key; the runtime must
                // resolve the policy through THIS field (fail-closed verified
                // in QPack::verify and tested below)
                handler: "auth.session.check".into(),
                declared_statuses: vec![401],
                provides: Some("session".into()),
            },
        )]),
        capabilities: vec!["timer".into()],
        capability_hash: q_pack::capability_hash(&["timer".to_string()]),
        capability_inventory: None,
        capability_inventory_sha256: None,
        functions,
        schema_manifest: vec![],
        policy_manifest: vec![],
        router: None,
        handler_table,
        integrity: Integrity {
            algorithm: "sha256".into(),
            bundle_sha256: String::new(),
            routes_sha256: String::new(),
            bytecode_sha256: None,
        },
    };
    finalize_numeric(&mut pack);
    {
        use sha2::{Digest, Sha256};
        pack.contract_hash = pack.public_contract_sha256()[..32].to_string();
        pack.integrity.bundle_sha256 = hex(&Sha256::digest(pack.bundle.as_bytes()));
        pack.integrity.routes_sha256 =
            hex(&Sha256::digest(pack.routes_canonical_json().as_bytes()));
    }
    pack
}

/// Mirror of the compiler's numeric current-pack finalization: empty handler
/// table, dense complete schema manifest, serialized router automaton.
fn finalize_numeric(pack: &mut QPack) {
    pack.execution_mode = Some("numeric".into());
    pack.handler_table.clear();
    // Dense policy manifest: sorted policy keys, resolve handler IDs through
    // the dense function manifest.
    pack.policy_manifest = pack
        .policies
        .keys()
        .enumerate()
        .map(|(i, key)| {
            let handler = &pack.policies[key].handler;
            let handler_id = pack
                .functions
                .iter()
                .position(|f| f.key == *handler)
                .expect("policy handler in function manifest") as u32;
            q_pack::PolicyDecl {
                id: i as u32,
                key: key.clone(),
                handler_id,
            }
        })
        .collect();
    for route in pack.routes.iter_mut() {
        if let (Some(policy), Some(plan)) = (&route.policy, route.plan.as_mut()) {
            let pd = pack
                .policy_manifest
                .iter()
                .find(|p| p.key == *policy)
                .expect("route policy in policy manifest");
            plan.policy_id = Some(pd.id);
            plan.policy_handler_id = Some(pd.handler_id);
        }
    }
    // M24-005-A: canonical header-name table + per-plan ids, mirroring the
    // compiler emission (security scheme headers; headers-binding schemas).
    let mut names: Vec<String> = Vec::new();
    for route in pack.routes.iter_mut() {
        let mut route_names: Vec<String> = route
            .security
            .iter()
            .map(|sec| sec.header.clone())
            .collect();
        if let Some(binding) = &route.headers {
            if let Some(key) = &binding.schema {
                if let Some(q_schema_runtime::SchemaIr::Object { properties, .. }) =
                    pack.schemas.get(key)
                {
                    route_names.extend(properties.keys().cloned());
                }
            }
        }
        route_names.sort();
        route_names.dedup();
        if let Some(plan) = route.plan.as_mut() {
            plan.header_name_ids = route_names
                .iter()
                .map(|n| match names.binary_search(n) {
                    Ok(pos) => pos as u32,
                    Err(pos) => {
                        names.insert(pos, n.clone());
                        pos as u32
                    }
                })
                .collect();
        }
    }
    pack.header_name_table = names;
    let mut query_names = Vec::new();
    for route in pack.routes.iter_mut() {
        let mut route_names = route
            .query
            .as_ref()
            .and_then(|b| b.schema.as_ref())
            .and_then(|key| pack.schemas.get(key))
            .and_then(|ir| match ir {
                q_schema_runtime::SchemaIr::Object { properties, .. } => {
                    Some(properties.keys().cloned().collect::<Vec<_>>())
                }
                _ => None,
            })
            .unwrap_or_default();
        route_names.sort();
        route_names.dedup();
        if let Some(plan) = route.plan.as_mut() {
            plan.query_name_ids = route_names
                .iter()
                .map(|n| match query_names.binary_search(n) {
                    Ok(pos) => pos as u32,
                    Err(pos) => {
                        query_names.insert(pos, n.clone());
                        pos as u32
                    }
                })
                .collect();
            plan.cookie_name_ids.clear();
        }
    }
    pack.query_name_table = query_names;
    pack.cookie_name_table.clear();
    pack.schema_manifest = pack
        .schemas
        .keys()
        .enumerate()
        .map(|(i, k)| q_pack::SchemaDecl {
            id: i as u32,
            key: k.clone(),
            features: q_schema_runtime::features_of(&pack.schemas[k]),
            ir: pack.schemas[k].clone(),
        })
        .collect();
    // Bind plan schema IDs from each route's declared schema keys
    let schema_id = |key: &str| -> Option<u32> {
        pack.schema_manifest
            .iter()
            .find(|s| s.key == key)
            .map(|s| s.id)
    };
    for route in pack.routes.iter_mut() {
        let Some(ref mut plan) = route.plan else {
            continue;
        };
        plan.params_schema_id = route
            .params
            .as_ref()
            .and_then(|b| b.schema.as_deref())
            .and_then(schema_id);
        plan.query_schema_id = route
            .query
            .as_ref()
            .and_then(|b| b.schema.as_deref())
            .and_then(schema_id);
        plan.body_schema_id = route
            .body
            .as_ref()
            .and_then(|b| b.schema.as_deref())
            .and_then(schema_id);
        plan.headers_schema_id = route
            .headers
            .as_ref()
            .and_then(|b| b.schema.as_deref())
            .and_then(schema_id);
    }
    let method_index = |m: &str| -> usize {
        match m.to_ascii_uppercase().as_str() {
            "GET" => 0,
            "POST" => 1,
            "PUT" => 2,
            "PATCH" => 3,
            "DELETE" => 4,
            "OPTIONS" => 5,
            "HEAD" => 6,
            _ => 0,
        }
    };
    let mut nodes: Vec<q_pack::SerializedRouterNode> =
        vec![q_pack::SerializedRouterNode::default()];
    for (r_idx, route) in pack.routes.iter().enumerate() {
        let mut curr = 0usize;
        for seg in &route.path_segments {
            match seg.kind {
                q_pack::SegKind::Static => {
                    if let Some(existing) = nodes[curr]
                        .static_edges
                        .iter()
                        .find(|e| e.segment == seg.value)
                        .map(|e| e.target_node)
                    {
                        curr = existing;
                    } else {
                        let next = nodes.len();
                        nodes.push(q_pack::SerializedRouterNode::default());
                        nodes[curr].static_edges.push(q_pack::SerializedStaticEdge {
                            segment: seg.value.clone(),
                            target_node: next,
                        });
                        curr = next;
                    }
                }
                q_pack::SegKind::Param => {
                    if let Some(next) = nodes[curr].param_edge {
                        curr = next;
                    } else {
                        let next = nodes.len();
                        nodes.push(q_pack::SerializedRouterNode::default());
                        nodes[curr].param_edge = Some(next);
                        curr = next;
                    }
                }
                q_pack::SegKind::Wildcard => {
                    if let Some(next) = nodes[curr].wildcard_edge {
                        curr = next;
                    } else {
                        let next = nodes.len();
                        nodes.push(q_pack::SerializedRouterNode::default());
                        nodes[curr].wildcard_edge = Some(next);
                        curr = next;
                    }
                }
            }
        }
        let terminal = nodes[curr]
            .terminal
            .get_or_insert_with(q_pack::SerializedTerminal::default);
        let m_idx = method_index(&route.method);
        terminal.method_mask |= 1 << m_idx;
        terminal.route_by_method[m_idx] = Some(r_idx);
    }
    pack.router = Some(q_pack::SerializedRouter { nodes });
}

#[allow(dead_code)]
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

const FIXTURE_BUNDLE: &str = r#"
"use strict";
var __users = null;
var __nextUser = 1;
function users() {
  // lazy in-memory service: first use seeds the fixture user (C5)
  if (__users === null) {
    __users = new Map();
    __users.set("usr_1", { id: "usr_1", name: "Ada", email: "ada@example.org" });
  }
  return __users;
}

async function health_live() { return { status: "ok" }; }
async function js_text() { return "plain"; }
async function js_json() { return { ok: true }; }
async function hello_get(ctx) {
  return { message: "Hello " + ctx.params.name };
}
async function users_create(ctx) {
  // deterministic in a fresh process: first created user is usr_1
  const id = "usr_" + (__nextUser++);
  const u = { id, name: ctx.body.name, email: ctx.body.email };
  users().set(id, u);
  return { __ok: true, status: 201, value: u };
}
async function users_get(ctx) {
  const u = users().get(ctx.params.id);
  if (!u) return { __problem: true, problem: "not-found", status: 404, detail: "user not found" };
  return u;
}
async function auth_session(req) {
  const token = req.headers.authorization;
  if (token !== "Bearer q-demo-token") {
    return { __problem: true, problem: "unauthorized", status: 401 };
  }
  return { session: { userId: "usr_1" } };
}async function async_timer(ctx) {
  const waited = await ctx.native.timer.delay(ctx.query.ms);
  return { waited };
}
async function async_cancel(ctx) {
  const waited = await ctx.native.timer.delay(ctx.query.ms);
  return { cancelled: false, waited };
}
async function throw_redacted() {
  throw new Error("secret-boom");
}
function poison_chain(ctx) {
  const again = () => { Promise.resolve().then(again); };
  again();
  return { ok: true };
}
async function fallback_echo(ctx) {
  // M25-004-B generic fallback: receives the RAW parsed body (no native
  // validation) and echoes it back so the test can prove what crossed.
  return ctx.body;
}

globalThis.__velquFunctionManifest = [
  ["health.live", 0, health_live],
  ["js.text", 0, js_text],
  ["js.json", 0, js_json],
  ["hello.get", 0, hello_get],
  ["users.create", 0, users_create],
  ["users.get", 0, users_get],
  ["async.timer", 0, async_timer],
  ["async.cancel", 0, async_cancel],
  ["throw.redacted", 0, throw_redacted],
  ["poison.chain", 0, poison_chain],
  ["fallback.echo", 0, fallback_echo],
  ["auth.session.check", 1, auth_session]
];
globalThis.__velquFunctions = globalThis.__velquFunctionManifest.map(function(e) { return e[2]; });
"#;

// ---------------------------------------------------------------- harness

struct Server {
    child: Child,
    #[allow(dead_code)]
    port: u16,
    log_lines: std::sync::mpsc::Receiver<String>,
    /// The ready line consumed by start()'s readiness wait — retained so
    /// tests can inspect startup fields without racing the reader.
    ready_line: String,
}

impl Server {
    fn start(pack_path: &std::path::Path, port: u16) -> Server {
        let bin = env!("CARGO_BIN_EXE_velqu-runtime");
        let mut child = Command::new(bin)
            .arg("--pack")
            .arg(pack_path)
            .arg("--port")
            .arg(port.to_string())
            .arg("--log")
            .arg("full")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn velqu-runtime");
        let stdout = child.stdout.take().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        let _ = tx.send(l);
                    }
                    Err(_) => break,
                }
            }
        });
        // wait for the explicit ready line (bounded)
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut ready_line_for_test: Option<String> = None;
        loop {
            if ready_line_for_test.is_none() {
                if let Ok(line) = rx.try_recv() {
                    if line.contains("\"event\":\"ready\"") {
                        ready_line_for_test = Some(line);
                        break;
                    }
                }
            } else {
                break;
            }
            if Instant::now() > deadline {
                panic!("server did not become ready in time");
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        Server {
            child,
            port,
            log_lines: rx,
            ready_line: ready_line_for_test.unwrap_or_default(),
        }
    }

    fn drain_logs(&self) -> Vec<String> {
        let mut out = Vec::new();
        while let Ok(l) = self.log_lines.try_recv() {
            out.push(l);
        }
        out
    }

    fn stop(mut self) -> std::process::ExitStatus {
        self.child.kill().expect("kill server");
        self.child.wait().expect("reap server")
    }
}

struct Resp {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Resp {
    fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
    fn json(&self) -> Value {
        serde_json::from_slice(&self.body).unwrap_or(Value::Null)
    }
    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

fn http(port: u16, req: &str, body: Option<&[u8]>) -> Resp {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    let mut raw = req.to_string();
    raw.push_str("connection: close\r\n");
    if let Some(b) = body {
        raw = format!("{}content-length: {}\r\n\r\n", raw, b.len());
        stream.write_all(raw.as_bytes()).unwrap();
        stream.write_all(b).unwrap();
    } else {
        raw.push_str("\r\n");
        stream.write_all(raw.as_bytes()).unwrap();
    }
    let mut buf = Vec::new();
    // tolerate RST-after-response (HEAD/keep-alive edge cases): parse what arrived
    let _ = stream.read_to_end(&mut buf);
    parse_http(&buf)
}

fn parse_http(raw: &[u8]) -> Resp {
    let split = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .expect("header terminator");
    let head = String::from_utf8_lossy(&raw[..split]).into_owned();
    let mut lines = head.lines();
    let status_line = lines.next().unwrap_or_default();
    let status: u16 = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let mut headers = Vec::new();
    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            headers.push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    let mut body = raw[split + 4..].to_vec();
    if let Some(len) = headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, v)| v.parse::<usize>().ok())
    {
        body.truncate(len);
    }
    Resp {
        status,
        headers,
        body,
    }
}

fn write_pack(dir: &std::path::Path) -> PathBuf {
    let pack = fixture_pack();
    let path = dir.join("app.qpack");
    std::fs::write(&path, serde_json::to_vec(&pack).unwrap()).unwrap();
    path
}

#[test]
fn fingerprint_flag_reports_exact_tuple_and_verifies_without_serving() {
    // M26-009-C: --fingerprint prints the runtime's enforced tuple and,
    // with --pack, runs FULL verification without ever binding a port.
    let dir = temp_dir("fingerprint");
    let pack_path = write_pack(&dir);
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");

    // compatible pack: exit 0, JSON carries the tuple + verdict
    let out = Command::new(bin)
        .arg("--fingerprint")
        .arg("--pack")
        .arg(&pack_path)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let j: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(j["event"], "runtime.fingerprint");
    assert_eq!(j["runtime"]["engineName"], "quickjs-ng");
    assert_eq!(j["runtime"]["engineVersion"], "0.15.1");
    assert_eq!(j["runtime"]["rquickjs"], "0.12.2");
    assert_eq!(j["runtime"]["runtimeAbi"], 1);
    assert_eq!(j["runtime"]["pointerWidth"], 64);
    assert_eq!(j["pack"]["verdict"], "compatible");

    // mismatched pack: exit 2 with the actionable diagnostic, still no port
    let mut pack = q_pack::QPack::load_and_verify(&pack_path).unwrap();
    pack.engine.version = "9.9.9".into();
    let bad = dir.join("bad.qpack");
    std::fs::write(&bad, serde_json::to_vec(&pack).unwrap()).unwrap();
    let out = Command::new(bin)
        .arg("--fingerprint")
        .arg("--pack")
        .arg(&bad)
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(2),
        "mismatched fingerprint must exit 2"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("engine mismatch"), "stderr: {stderr}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn fingerprint_reports_sidecar_binding_key() {
    // M26-009-D: --fingerprint prints packSha256 — the binding key for
    // the deployment mode's debug sidecar — and it equals sha256 of the
    // exact pack bytes on disk.
    let dir = temp_dir("fp-sidecar");
    let pack_path = write_pack(&dir);
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let out = Command::new(bin)
        .arg("--fingerprint")
        .arg("--pack")
        .arg(&pack_path)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let j: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&out.stdout).trim()).unwrap();
    assert_eq!(j["pack"]["verdict"], "compatible");
    assert_eq!(j["pack"]["sidecar"], "<pack>.sources.json");
    let want = {
        use sha2::{Digest, Sha256};
        let bytes = std::fs::read(&pack_path).unwrap();
        let h = Sha256::digest(&bytes);
        h.iter().map(|b| format!("{b:02x}")).collect::<String>()
    };
    assert_eq!(j["pack"]["packSha256"].as_str().unwrap(), want);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn source_sidecar_never_affects_serving() {
    // ADR-0027: a present-but-garbage <pack>.sources.json sidecar changes
    // nothing about serving — the runtime has no load path to it.
    let dir = temp_dir("sidecar");
    let pack_path = write_pack(&dir);
    std::fs::write(
        dir.join("app.qpack.sources.json"),
        b"{\"formatVersion\":99,\"packSha256\":\"garbage\"}",
    )
    .unwrap();
    let port = free_port();
    let server = Server::start(&pack_path, port);
    let r = http(port, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200, "sidecar must not influence serving");
    assert_eq!(r.header("x-velqu-stage"), Some("native"));
    drop(server);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn debug_dump_pack() {
    let dir = std::path::PathBuf::from("/tmp/velqu-debug");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    write_pack(&dir);
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("velqu-m1-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn free_port() -> u16 {
    static NEXT: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(19000);
    for _ in 0..100 {
        let p = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if std::net::TcpListener::bind(("127.0.0.1", p)).is_ok() {
            return p;
        }
    }
    std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

/// M2.2.1-r4.1: poisoning the engine flips readiness while liveness stays
/// green, and dynamic JS routes fail closed with 503 at the HTTP boundary.
#[test]
fn poisoned_runtime_marks_readiness_false() {
    let dir = temp_dir("readiness");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);

    // healthy: readiness 200 before poisoning (GET and HEAD)
    let r = http(port, "GET /health/ready HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200, "ready before poison");
    let r_head = http(port, "HEAD /health/ready HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r_head.status, 200);
    assert_eq!(r_head.body.len(), 0, "HEAD /health/ready must emit no body");

    // poison the engine: unquiescable sync microtask chain → 504 timeout
    let r = http(port, "GET /poison HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 504, "poison route times out, got {}", r.status);

    // liveness stays 200 (process + listener alive)
    let r = http(port, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200, "liveness stays green after quarantine");

    // readiness flips to 503 (GET and HEAD both tested; HEAD has no body)
    let r = http(port, "GET /health/ready HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 503, "readiness fails after quarantine");
    assert!(r.text().contains("engine quarantined"));
    let r_head_503 = http(port, "HEAD /health/ready HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r_head_503.status, 503);
    assert_eq!(
        r_head_503.body.len(),
        0,
        "HEAD /health/ready 503 must emit no body"
    );

    // dynamic JS routes fail closed at the boundary with 503 + retry-after
    let r = http(port, "GET /js-text HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 503, "dynamic JS routes 503 after quarantine");
    assert_eq!(r.header("retry-after"), Some("1"));
    assert!(r.text().contains("engine quarantined"));

    server.stop();
}

/// M2.2.1-r3: the route declares policy "auth.session" whose
/// PolicyEntry.handler is "auth.session.check" (a DIFFERENT key). The runtime
/// must resolve the policy through the entry's handler field: enforcement
/// active (401 without token, 200 with) instead of the old fail-closed
/// "policy not in cache" engine failure.
#[test]
fn policy_id_resolves_declared_handler_key() {
    let dir = temp_dir("policy-resolve");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);

    // no credentials → policy ran and rejected
    let r = http(port, "GET /users/usr_1 HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(
        r.status, 401,
        "policy handler must execute via entry.handler"
    );
    assert_eq!(r.json()["type"], "https://velqu.dev/problems/unauthorized");

    // valid credentials → policy passed, session injected, business handler ran
    let r = http(
        port,
        "GET /users/usr_1 HTTP/1.1\r\nhost: t\r\nauthorization: Bearer q-demo-token\r\n",
        None,
    );
    assert_eq!(r.status, 200, "body: {}", r.text());
    assert!(r.text().contains("\"Ada\""), "business handler executed");

    server.stop();
}

// ---------------------------------------------------------------- tests

#[test]
fn full_runtime_conformance() {
    let dir = temp_dir("conf");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);

    // C0: native liveness, exact bytes, JS never entered (stage=native header)
    let r = http(port, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"status\":\"ok\"}");
    assert_eq!(r.header("x-velqu-stage"), Some("native"));

    // HEAD on C0: same status, no body
    let r = http(port, "HEAD /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert!(r.body.is_empty(), "HEAD must not carry a body");

    // C1: JS text
    let r = http(port, "GET /js-text HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "plain");

    // C2: JS JSON
    let r = http(port, "GET /js-json HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"ok\":true}");

    // C3: validated path param + happy path
    let r = http(port, "GET /hello/Rafi HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"message\":\"Hello Rafi\"}");

    // C3 negative: name too long → 422 identifying field
    let long = "x".repeat(61);
    let r = http(
        port,
        &format!("GET /hello/{} HTTP/1.1\r\nhost: t\r\n", long),
        None,
    );
    assert_eq!(r.status, 422, "body: {}", r.text());
    let j = r.json();
    assert_eq!(
        j["type"],
        "https://velqu.dev/problems/validation",
        "body: {}",
        r.text()
    );
    assert_eq!(j["errors"][0]["path"], "name");
    assert_eq!(j["errors"][0]["code"], "maxLength");

    // POST /users happy → 201 exact bytes
    let body = br#"{"name":"Ada","email":"ada@example.org"}"#;
    let r = http(
        port,
        "POST /users HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(body),
    );
    assert_eq!(r.status, 201, "body: {}", r.text());
    assert_eq!(
        r.text(),
        "{\"id\":\"usr_1\",\"name\":\"Ada\",\"email\":\"ada@example.org\"}"
    );

    // malformed JSON → 422
    let r = http(
        port,
        "POST /users HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(br#"{name:"#),
    );
    assert_eq!(r.status, 422);
    assert_eq!(r.json()["status"], 422);

    // schema-invalid email → 422 identifying field
    let r = http(
        port,
        "POST /users HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(br#"{"name":"Ada","email":"not-an-email"}"#),
    );
    assert_eq!(r.status, 422);
    assert_eq!(r.json()["errors"][0]["path"], "email");

    // C4: policy 401 without credentials
    let r = http(port, "GET /users/usr_1 HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 401);
    assert_eq!(r.json()["type"], "https://velqu.dev/problems/unauthorized");

    // C4: valid credentials → 200 exact bytes (lazy service first use = C5 too)
    let r = http(
        port,
        "GET /users/usr_1 HTTP/1.1\r\nhost: t\r\nauthorization: Bearer q-demo-token\r\n",
        None,
    );
    assert_eq!(r.status, 200, "body: {}", r.text());
    assert_eq!(
        r.text(),
        "{\"id\":\"usr_1\",\"name\":\"Ada\",\"email\":\"ada@example.org\"}"
    );

    // C4: unknown user → typed 404
    let r = http(
        port,
        "GET /users/usr_999 HTTP/1.1\r\nhost: t\r\nauthorization: Bearer q-demo-token\r\n",
        None,
    );
    assert_eq!(r.status, 404);
    assert_eq!(r.json()["type"], "https://velqu.dev/problems/not-found");

    // async: native timer through a promise; default ms=10 (query coercion)
    let r = http(port, "GET /async HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"waited\":10}");
    let r = http(port, "GET /async?ms=50 HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.text(), "{\"waited\":50}");

    // throw → redacted 500; body must not leak the secret or any stack
    let r = http(port, "GET /throw HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 500);
    let text = r.text();
    assert!(text.contains("\"status\":500"));
    assert!(!text.contains("secret-boom"), "redaction failed: {text}");
    assert!(!text.contains("at "), "stack leaked: {text}");

    // 404 unknown path (JSON problem)
    let r = http(
        port,
        "GET /definitely/not/here HTTP/1.1\r\nhost: t\r\n",
        None,
    );
    assert_eq!(r.status, 404);
    assert_eq!(r.json()["title"], "Not Found");

    // 405 with Allow (GET-only route hit with POST)
    let r = http(port, "POST /js-text HTTP/1.1\r\nhost: t\r\n", Some(b""));
    assert_eq!(r.status, 405);
    let allow = r.header("allow").expect("Allow header").to_string();
    assert!(
        allow.contains("GET") && allow.contains("HEAD"),
        "allow: {allow}"
    );

    // logs: stage evidence — C0 served natively; JS routes via engine
    std::thread::sleep(Duration::from_millis(100));
    let logs = server.drain_logs();
    let native = logs
        .iter()
        .filter(|l| l.contains("/health/live") && l.contains("\"stage\":\"native\""))
        .count();
    assert!(
        native >= 2,
        "C0 must be served at the native stage: {logs:?}"
    );
    let engine_stage = logs
        .iter()
        .filter(|l| l.contains("\"stage\":\"engine\""))
        .count();
    assert!(engine_stage >= 5, "JS routes must log engine stage");
    let redacted_logs = logs.iter().any(|l| l.contains("secret-boom"));
    assert!(
        !redacted_logs,
        "internal error detail goes to stderr, not stdout logs"
    );

    server.stop();
}

/// M25-004-B: a body schema that is an explicit fallback marker WITHOUT inner
/// keeps the QuickJS/generic path — the raw parsed JSON crosses to the handler
/// (echoed back verbatim), never a fail-closed 422.
#[test]
fn js_fallback_body_routes_raw_json_to_handler() {
    let dir = temp_dir("fallback-body");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);
    wait_tcp(port, Duration::from_secs(10));

    // 1. Arbitrary JSON (schema-less under the fallback marker) crosses intact
    let body = br#"{"anything":"goes","nested":[1,2,3]}"#;
    let r = http(
        port,
        "POST /fallback HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(body),
    );
    assert_eq!(r.status, 200, "body: {}", r.text());
    assert_eq!(r.json()["anything"], "goes");
    assert_eq!(r.json()["nested"][2], 3);

    // 2. Even shape-mismatched values cross (no native validation ran)
    let r = http(
        port,
        "POST /fallback HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(br#"[1,2,3]"#),
    );
    assert_eq!(r.status, 200, "array body must cross raw: {}", r.text());
    assert_eq!(r.json().as_array().unwrap().len(), 3);

    // 3. Malformed JSON still rejects 422 at admission (parse precedes strategy)
    let r = http(
        port,
        "POST /fallback HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(b"{not json"),
    );
    assert_eq!(r.status, 422);
    assert_eq!(r.header("content-type"), Some("application/problem+json"));
    assert_eq!(r.json()["detail"], "malformed JSON body");

    server.stop();
}

/// M25-004-C: deeply nested inputs fail boundedly. Parse-level nesting is
/// capped by serde_json's recursion limit (128) — anything deeper rejects 422
/// at admission on every body route, including the js-fallback route.
#[test]
fn deeply_nested_body_fails_boundedly() {
    let dir = temp_dir("deep-body");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);
    wait_tcp(port, Duration::from_secs(10));

    let deep = format!("{}1{}", "[".repeat(200), "]".repeat(200));
    let r = http(
        port,
        "POST /fallback HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(deep.as_bytes()),
    );
    assert_eq!(r.status, 422, "body: {}", r.text());
    assert_eq!(r.json()["detail"], "malformed JSON body");
    assert_eq!(r.json()["type"], "https://velqu.dev/problems/validation");

    let r = http(
        port,
        "POST /users HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(deep.as_bytes()),
    );
    assert_eq!(r.status, 422);

    server.stop();
}

/// M25-004-D: the route deadline bounds the whole pipeline from route
/// match, not just the handler. A client that sends POST headers declaring
/// a body and then stalls the stream must receive the 504 `timeout`
/// problem at the route deadline (200ms here) — the bounded read is
/// cancelled instead of holding the request open — and a prompt body on
/// the same route still reaches the handler under the anchored deadline.
#[test]
fn body_read_deadline_cancels_stalled_transfer() {
    let dir = temp_dir("deadline-body");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);
    wait_tcp(port, Duration::from_secs(10));

    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    // headers declare a 32-byte JSON body that never arrives
    stream
        .write_all(
            b"POST /deadline-body HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\ncontent-length: 32\r\n\r\n",
        )
        .unwrap();
    let started = Instant::now();
    let mut buf = Vec::new();
    // the connection closes once the deadline response is written (request
    // body left unread); tolerate a client-side read timeout and parse
    // whatever arrived, like the http() helper does
    let _ = stream.read_to_end(&mut buf);
    let elapsed = started.elapsed();
    let r = parse_http(&buf);
    assert_eq!(
        r.status,
        504,
        "stalled body read must settle at the route deadline: {}",
        r.text()
    );
    assert_eq!(r.json()["type"], "https://velqu.dev/problems/timeout");
    // 200ms deadline — the response must arrive at the deadline, not at the
    // 5s client read timeout or a transport-level abort
    assert!(
        elapsed < Duration::from_secs(2),
        "504 arrived after {:?}, deadline is 200ms",
        elapsed
    );

    // control: a prompt body on the same route settles normally — the
    // anchored deadline leaves the handler its full remaining budget
    let r = http(
        port,
        "POST /deadline-body HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(br#"{"prompt":true}"#),
    );
    assert_eq!(
        r.status,
        200,
        "prompt body must still reach the handler: {}",
        r.text()
    );
    assert_eq!(r.json()["prompt"], true);

    server.stop();
}

#[test]
fn tampered_pack_fails_before_ready() {
    let dir = temp_dir("tamper");
    let pack_path = write_pack(&dir);
    // mutate the bundle without recomputing integrity
    let raw: Value = serde_json::from_slice(&std::fs::read(&pack_path).unwrap()).unwrap();
    let mut tampered = raw.clone();
    tampered["bundle"] = json!("__velquRegister('health.live', function(){ return {evil:true} });");
    let tampered_path = dir.join("tampered.qpack");
    std::fs::write(&tampered_path, serde_json::to_vec(&tampered).unwrap()).unwrap();

    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let out = Command::new(bin)
        .arg("--pack")
        .arg(&tampered_path)
        .arg("--port")
        .arg(free_port().to_string())
        .output()
        .expect("run tampered pack");
    assert_ne!(
        out.status.code(),
        Some(0),
        "tampered pack must exit non-zero"
    );
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("sha256 mismatch") || err.contains("integrity"),
        "stderr: {err}"
    );
}

#[test]
fn client_abort_leaves_server_healthy() {
    let dir = temp_dir("abort");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);

    // start a long /cancel request, then drop the connection mid-flight
    {
        let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
        s.write_all(b"GET /cancel?ms=1000 HTTP/1.1\r\nhost: t\r\n\r\n")
            .unwrap();
        std::thread::sleep(Duration::from_millis(20));
        drop(s);
    }
    // server must remain healthy: liveness answers within 1s
    let t0 = Instant::now();
    let r = http(port, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert!(
        t0.elapsed() < Duration::from_secs(1),
        "server unhealthy after abort"
    );

    // and JS still works afterwards
    let r = http(port, "GET /js-json HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    server.stop();
}

/// M3-003-D: the ready line exposes the service profile and startup
/// worker count; an explicit bad profile fails closed before serving.
#[test]
fn ready_line_reports_service_profile_and_bad_profile_fails_closed() {
    let dir = temp_dir("svcprofile");
    let pack_path = write_pack(&dir);

    // Default (no flag): serverless, one startup worker.
    let port = free_port();
    let server = Server::start(&pack_path, port);
    let ready = &server.ready_line;
    assert!(
        ready.contains("\"serviceProfile\":\"serverless\"")
            && ready.contains("\"startupWorkers\":1"),
        "default ready line must declare serverless/1: {ready}"
    );
    server.stop();

    // Unknown profile: fail closed BEFORE serving (exit 2, no ready line).
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_velqu-runtime"))
        .arg("--pack")
        .arg(&pack_path)
        .arg("--port")
        .arg(port.to_string())
        .arg("--service-profile")
        .arg("bogus")
        .output()
        .expect("spawn runtime");
    assert_eq!(output.status.code(), Some(2), "bad profile exits 2");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unknown service profile"),
        "fail-closed error names the problem: {stderr}"
    );
}

#[test]
fn graceful_shutdown_exits_zero() {
    let dir = temp_dir("shutdown");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let mut server = Server::start(&pack_path, port);
    // Allow the server's signal listener to establish
    std::thread::sleep(Duration::from_millis(50));
    // M3-007-A: one full JS invocation before shutdown — the ownership
    // registry must show the admission reached its terminal transition
    // (registered == settled) and the shutdown report carries the
    // no-orphan invariant (pending == 0) for the engine-backed route.
    let r = http(port, "GET /js-text HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200, "js route served before shutdown");
    // SIGTERM
    unsafe {
        libc::kill(server.child.id() as i32, libc::SIGTERM);
    }
    let status = server.child.wait().unwrap();
    assert!(
        status.success(),
        "graceful shutdown must exit 0, got {status:?}"
    );
    // M28-009-C: quiescence includes the outbound fetch pool — the
    // shutdown.complete event must report the pool drain. The log reader
    // thread may still be pumping the pipe after wait(): poll bounded.
    let mut complete: Option<String> = None;
    for _ in 0..50 {
        let logs = server.drain_logs();
        if let Some(line) = logs
            .iter()
            .find(|l| l.contains("\"event\":\"shutdown.complete\""))
        {
            complete = Some(line.clone());
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    let complete = complete.unwrap_or_else(|| panic!("shutdown.complete event missing"));
    assert!(
        complete.contains("\"fetchPool\":{\"initialized\":false,\"drained\":true}"),
        "fetch pool drain not reported: {complete}"
    );
    // M3-007-A: no orphan invocation — every admission settled; the
    // single pre-shutdown request is visible as exactly one lifecycle.
    assert!(
        complete.contains("\"invocations\":{\"pending\":0,\"registered\":1,\"settled\":1}"),
        "ownership invariant (pending=0, 1 settled invocation) not reported: {complete}"
    );
}

#[test]
fn source_mapped_exception_identifies_original_location() {
    use sourcemap::SourceMapBuilder;
    // generated bundle with the throw on line 2
    let bundle = "async function thrower() {\n  throw new Error(\"origin-boom\");\n}\nglobalThis.__velquFunctionManifest = [[\"t\", 0, thrower]];\nglobalThis.__velquFunctions = [thrower];\n";
    let mut b = SourceMapBuilder::new(None);
    b.add(
        1,
        0,
        41,
        4,
        Some("src/modules/users/routes.ts"),
        None,
        false,
    );
    let map_json = {
        let mut out = Vec::new();
        let sm = b.into_sourcemap();
        sm.to_writer(&mut out).unwrap();
        String::from_utf8(out).unwrap()
    };

    let dir = temp_dir("sourcemap");
    let mut pack = fixture_pack();
    pack.bundle = bundle.to_string();
    pack.source_map = Some(map_json);
    pack.functions = vec![FunctionDecl {
        id: 0,
        key: "t".into(),
        kind: FunctionKind::RouteHandler,
    }];
    // single-route handler table must match the new bundle
    pack.handler_table = std::collections::BTreeMap::from([("t".to_string(), "t".to_string())]);
    let throw_route = pack
        .routes
        .iter()
        .position(|r| r.id == "throw.redacted")
        .unwrap();
    let mut route = pack.routes[throw_route].clone();
    route.handler = "t".into();
    route.plan = Some(RoutePlanDecl {
        route_id: 0,
        handler_id: 0,
        policy_id: None,
        policy_handler_id: None,
        params_schema_id: None,
        query_schema_id: None,
        headers_schema_id: None,
        body_schema_id: None,
        header_name_ids: vec![],
        query_name_ids: vec![],
        cookie_name_ids: vec![],
        default_status: 200,
        allowed_statuses: vec![200],
        field_needs: FieldNeeds::default(),
        response_strategy: Strategy::Js,
        validation_fallback_reason: None,
        response_fallback_reason: Some("explicit".into()),
        deadline_ms: 5000,
    });
    pack.routes = vec![route];
    pack.policies.clear();
    // recompute integrity (numeric finalization: no handlerTable, complete
    // schema manifest, serialized router over the CURRENT route set)
    finalize_numeric(&mut pack);
    {
        use sha2::{Digest, Sha256};
        pack.contract_hash = pack.public_contract_sha256()[..32].to_string();
        pack.integrity.bundle_sha256 = {
            let h = Sha256::digest(pack.bundle.as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
        pack.integrity.routes_sha256 = {
            let h = Sha256::digest(pack.routes_canonical_json().as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
    }
    let pack_path = dir.join("sourcemap.qpack");
    std::fs::write(&pack_path, serde_json::to_vec(&pack).unwrap()).unwrap();

    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port, Duration::from_secs(10));
    let _ = http(port, "GET /throw HTTP/1.1\r\nhost: t\r\n", None);
    std::thread::sleep(Duration::from_millis(150));
    child.kill().unwrap();
    let out = child.wait_with_output().unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("origin-boom"),
        "engine detail must reach internal stderr log: {err}"
    );
    assert!(
        err.contains("src/modules/users/routes.ts") && err.contains("42"),
        "original source location must be mapped into diagnostics: {err}"
    );
}

#[test]
fn routing_precedes_body_materialization() {
    let dir = temp_dir("route-first");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);

    // Announce a 2 MiB body but transmit only a fragment. A server that
    // polls the body before routing could not answer until the body
    // arrives; the route-first pipeline answers from the head alone while
    // the body is still unsent (M24-002-B no-poll proof).
    let cases: [(&str, u16); 2] = [
        (
            "POST /definitely-not-a-route HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
            404,
        ),
        (
            "POST /js-text HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
            405,
        ),
    ];
    for (req, want) in cases {
        let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let head = format!("{req}content-length: 2097152\r\n\r\n");
        stream.write_all(head.as_bytes()).unwrap();
        stream.write_all(&[b'x'; 1024]).unwrap(); // fragment of the announced body
        let mut buf = [0u8; 2048];
        let n = stream
            .read(&mut buf)
            .expect("answer must arrive before the body completes");
        let resp = parse_http(&buf[..n]);
        assert_eq!(
            resp.status,
            want,
            "route-first answer, got {}: {}",
            resp.status,
            resp.text()
        );
        drop(stream);
    }

    server.stop();
}

#[test]
fn routeplan_body_flag_controls_body_collection_independent_of_method() {
    let mut pack = fixture_pack();
    let route = pack
        .routes
        .iter_mut()
        .find(|r| r.id == "users.create")
        .expect("fixture route");
    route.method = "DELETE".into();
    finalize_numeric(&mut pack);
    let plan = pack
        .routes
        .iter()
        .find(|r| r.id == "users.create")
        .and_then(|r| r.plan.as_ref())
        .unwrap();
    assert!(plan.field_needs.body);
    let no_body = pack.routes.iter().find(|r| r.id == "health.live").unwrap();
    assert!(!no_body.plan.as_ref().unwrap().field_needs.body);
}

#[test]
fn content_length_over_limit_rejects_before_body_poll() {
    let dir = temp_dir("content-length-limit");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);
    let request = "POST /users HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: 70000\r\nConnection: close\r\n\r\n".to_string();
    let response = http(port, &request, None);
    assert_eq!(response.status, 413);
    server.stop();
}

#[test]
fn body_and_header_limits_reject_oversize() {
    let dir = temp_dir("limits");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);

    // body over the route limit (65536) → 413 problem
    let big = vec![b'{'];
    let mut huge = br#"{"name":""#.to_vec();
    huge.extend(vec![b'a'; 70_000]);
    let _ = big;
    let r = http(
        port,
        "POST /users HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(&huge),
    );
    assert_eq!(
        r.status,
        413,
        "oversize body must be 413, got {}: {}",
        r.status,
        r.text()
    );

    // header block over 32 KiB → 431 (below hyper's own buffer cap)
    let mut req = String::from("GET /js-text HTTP/1.1\r\nhost: t\r\n");
    req.push_str(&format!("x-big: {}\r\n", "h".repeat(33_000)));
    req.push_str("connection: close\r\n\r\n");
    let r = http(port, &req, None);
    assert_eq!(
        r.status, 431,
        "oversize headers must be 431, got {}",
        r.status
    );

    server.stop();
}

#[test]
fn queue_limit_returns_503_when_saturated() {
    let dir = temp_dir("queue");
    let pack_path = write_pack(&dir);
    // config: queue of 1
    let cfg = dir.join("limits.json");
    std::fs::write(
        &cfg,
        serde_json::to_vec(&serde_json::json!({"maxQueue": 1})).unwrap(),
    )
    .unwrap();

    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--config")
        .arg(&cfg)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    wait_tcp(port, Duration::from_secs(10));

    // request 1 occupies the single admission slot for ~1.5s
    let mut slow = TcpStream::connect(("127.0.0.1", port)).unwrap();
    slow.write_all(b"GET /cancel?ms=1500 HTTP/1.1\r\nhost: t\r\nconnection: close\r\n\r\n")
        .unwrap();
    std::thread::sleep(Duration::from_millis(100)); // let it enter the pipeline
                                                    // request 2 while saturated → 503
    let r = http(port, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 503, "saturated queue must 503, got {}", r.status);
    drop(slow);
    child.kill().unwrap();
    let _ = child.wait();
}

fn wait_tcp(port: u16, deadline: Duration) {
    let end = Instant::now() + deadline;
    while TcpStream::connect(("127.0.0.1", port)).is_err() {
        if Instant::now() > end {
            panic!("server not ready");
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn response_schema_violation_is_a_controlled_500() {
    // handler returns a shape that does NOT match its declared response schema
    let bundle = r#"
async function bad_shape(ctx) { return { wrong: true }; }
globalThis.__velquFunctionManifest = [["bad.shape", 0, bad_shape]];
globalThis.__velquFunctions = [bad_shape];
"#;
    let dir = temp_dir("respval");
    let mut pack = fixture_pack();
    pack.bundle = bundle.to_string();
    pack.functions = vec![FunctionDecl {
        id: 0,
        key: "bad.shape".into(),
        kind: FunctionKind::RouteHandler,
    }];
    pack.handler_table =
        std::collections::BTreeMap::from([("bad.shape".to_string(), "bad.shape".to_string())]);
    let route = pack
        .routes
        .iter()
        .position(|r| r.id == "users.get")
        .unwrap();
    let mut r = pack.routes[route].clone();
    r.id = "bad.shape".into();
    r.handler = "bad.shape".into();
    r.policy = None;
    r.params = None;
    r.query = None;
    r.body = None;
    r.headers = None;
    r.security.clear();
    r.plan = Some(RoutePlanDecl {
        route_id: 0,
        handler_id: 0,
        policy_id: None,
        policy_handler_id: None,
        params_schema_id: None,
        query_schema_id: None,
        headers_schema_id: None,
        body_schema_id: None,
        header_name_ids: vec![],
        query_name_ids: vec![],
        cookie_name_ids: vec![],
        default_status: 200,
        allowed_statuses: vec![200],
        field_needs: FieldNeeds::default(),
        response_strategy: Strategy::Native,
        validation_fallback_reason: None,
        response_fallback_reason: None,
        deadline_ms: 5000,
    });
    r.responses = {
        let mut m = std::collections::BTreeMap::new();
        m.insert(
            "200".to_string(),
            q_pack::ResponseDecl {
                schema: Some("sch:bad.200".to_string()),
                strategy: q_pack::Strategy::Native,
                problem: None,
            },
        );
        m
    };
    pack.routes = vec![r];
    pack.policies.clear();
    pack.schemas.insert(
        "sch:bad.200".to_string(),
        q_schema_runtime::SchemaIr::Object {
            properties: std::collections::BTreeMap::from([(
                "expected".to_string(),
                Box::new(q_schema_runtime::SchemaIr::String {
                    min_length: Some(1),
                    max_length: None,
                    pattern: None,
                    format: None,
                }),
            )]),
            required: vec!["expected".into()],
        },
    );
    finalize_numeric(&mut pack);
    {
        use sha2::{Digest, Sha256};
        pack.contract_hash = pack.public_contract_sha256()[..32].to_string();
        pack.integrity.bundle_sha256 = {
            let h = Sha256::digest(pack.bundle.as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
        pack.integrity.routes_sha256 = {
            let h = Sha256::digest(pack.routes_canonical_json().as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
    }
    let pack_path = dir.join("respval.qpack");
    std::fs::write(&pack_path, serde_json::to_vec(&pack).unwrap()).unwrap();

    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port, Duration::from_secs(10));
    let r = http(port, "GET /users/usr_1 HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(
        r.status,
        500,
        "schema-violating response must be a 500, got {}: {}",
        r.status,
        r.text()
    );
    assert!(r.text().contains("\"status\":500"));
    assert!(
        !r.text().contains("wrong"),
        "violation detail must not leak"
    );
    std::thread::sleep(Duration::from_millis(150));
    child.kill().unwrap();
    let out = child.wait_with_output().unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("contract.violation.response"),
        "internal log must carry the response contract violation: {err}"
    );
}

/// M25-005-A: a representable declared response schema on a native-strategy
/// route encodes through the generated one-traversal encoder. The handler
/// returns keys OUT of declared order; the wire bytes come back in the
/// schema's declared (byte-sorted) order, JSON-equal to the handler value —
/// and the mismatch sibling case (schema violation) stays a controlled 500
/// through the same encoder path.
#[test]
fn native_response_encoder_emits_declared_order() {
    let bundle = r#"
async function ordered_shape(ctx) {
    // insertion order deliberately differs from the declared schema order;
    // uni exercises the M25-005-C union encoder (first member misses, the
    // string member matches)
    return { zeta: "z", alpha: 7, mid: [1, 2], uni: "text" };
}
async function bad_shape2(ctx) { return { nope: true }; }
globalThis.__velquFunctionManifest = [["ordered.shape", 0, ordered_shape], ["bad.shape2", 0, bad_shape2]];
globalThis.__velquFunctions = [ordered_shape, bad_shape2];
"#;
    let dir = temp_dir("respenc");
    let mut pack = fixture_pack();
    pack.bundle = bundle.to_string();
    pack.functions = vec![
        FunctionDecl {
            id: 0,
            key: "ordered.shape".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "bad.shape2".into(),
            kind: FunctionKind::RouteHandler,
        },
    ];
    pack.handler_table = std::collections::BTreeMap::from([
        ("ordered.shape".to_string(), "ordered.shape".to_string()),
        ("bad.shape2".to_string(), "bad.shape2".to_string()),
    ]);
    let route = pack
        .routes
        .iter()
        .position(|r| r.id == "users.get")
        .unwrap();
    let base = pack.routes[route].clone();
    let plan = |handler_id: u32| RoutePlanDecl {
        route_id: handler_id,
        handler_id,
        policy_id: None,
        policy_handler_id: None,
        params_schema_id: None,
        query_schema_id: None,
        headers_schema_id: None,
        body_schema_id: None,
        header_name_ids: vec![],
        query_name_ids: vec![],
        cookie_name_ids: vec![],
        default_status: 200,
        allowed_statuses: vec![200],
        field_needs: FieldNeeds::default(),
        response_strategy: Strategy::Native,
        validation_fallback_reason: None,
        response_fallback_reason: None,
        deadline_ms: 5000,
    };
    let responses = |schema: &str| {
        std::collections::BTreeMap::from([(
            "200".to_string(),
            q_pack::ResponseDecl {
                schema: Some(schema.to_string()),
                strategy: q_pack::Strategy::Native,
                problem: None,
            },
        )])
    };
    let mut good = base.clone();
    good.id = "ordered.shape".into();
    good.handler = "ordered.shape".into();
    good.policy = None;
    good.params = None;
    good.query = None;
    good.body = None;
    good.headers = None;
    good.security.clear();
    good.path = "/ordered".into();
    good.path_segments = vec![PathSegment {
        kind: SegKind::Static,
        value: "ordered".into(),
    }];
    good.responses = responses("sch:ordered.200");
    good.plan = Some(plan(0));
    let mut bad = base.clone();
    bad.id = "bad.shape2".into();
    bad.handler = "bad.shape2".into();
    bad.policy = None;
    bad.params = None;
    bad.query = None;
    bad.body = None;
    bad.headers = None;
    bad.security.clear();
    bad.responses = responses("sch:ordered.200");
    bad.plan = Some(plan(1));
    // paths must be unique per route; give the bad twin its own segment
    bad.path = "/ordered-bad".into();
    bad.path_segments = vec![PathSegment {
        kind: SegKind::Static,
        value: "ordered-bad".into(),
    }];
    pack.routes = vec![good, bad];
    pack.policies.clear();
    pack.schemas.insert(
        "sch:ordered.200".to_string(),
        q_schema_runtime::SchemaIr::Object {
            properties: std::collections::BTreeMap::from([
                (
                    "zeta".to_string(),
                    Box::new(q_schema_runtime::SchemaIr::String {
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        format: None,
                    }),
                ),
                (
                    "mid".to_string(),
                    Box::new(q_schema_runtime::SchemaIr::Array {
                        items: Box::new(q_schema_runtime::SchemaIr::Integer {
                            minimum: None,
                            maximum: None,
                        }),
                        min_items: None,
                        max_items: None,
                    }),
                ),
                (
                    "alpha".to_string(),
                    Box::new(q_schema_runtime::SchemaIr::Integer {
                        minimum: None,
                        maximum: None,
                    }),
                ),
                (
                    "uni".to_string(),
                    Box::new(q_schema_runtime::SchemaIr::Union {
                        members: vec![
                            Box::new(q_schema_runtime::SchemaIr::Integer {
                                minimum: None,
                                maximum: None,
                            }),
                            Box::new(q_schema_runtime::SchemaIr::String {
                                min_length: None,
                                max_length: None,
                                pattern: None,
                                format: None,
                            }),
                        ],
                    }),
                ),
            ]),
            required: vec!["zeta".into(), "mid".into(), "alpha".into(), "uni".into()],
        },
    );
    finalize_numeric(&mut pack);
    {
        use sha2::{Digest, Sha256};
        pack.contract_hash = pack.public_contract_sha256()[..32].to_string();
        pack.integrity.bundle_sha256 = {
            let h = Sha256::digest(pack.bundle.as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
        pack.integrity.routes_sha256 = {
            let h = Sha256::digest(pack.routes_canonical_json().as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
    }
    let pack_path = dir.join("respenc.qpack");
    std::fs::write(&pack_path, serde_json::to_vec(&pack).unwrap()).unwrap();

    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port, Duration::from_secs(10));

    // valid response: wire bytes arrive in declared (byte-sorted) property
    // order — the one-traversal encoder engaged, output JSON-equal to the
    // handler's value
    let r = http(port, "GET /ordered HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200, "body: {}", r.text());
    assert_eq!(r.header("content-type"), Some("application/json"));
    assert_eq!(
        r.text(),
        "{\"alpha\":7,\"mid\":[1,2],\"uni\":\"text\",\"zeta\":\"z\"}",
        "encoder must emit declared property order"
    );

    // mismatch through the same encoder path stays a controlled 500
    let r = http(port, "GET /ordered-bad HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 500, "body: {}", r.text());
    assert!(!r.text().contains("nope"), "violation detail must not leak");

    std::thread::sleep(Duration::from_millis(150));
    child.kill().unwrap();
    let out = child.wait_with_output().unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("contract.violation.response"),
        "internal log must carry the encoder contract violation: {err}"
    );
}

/// M25-006-A: a DECLARED problem response (explicit Problem IR schema for
/// the status) encodes through the generated problem program — declared
/// title/type override the registry, the detail string crosses, and RFC
/// 9457 extension members (custom fields attached by the handler) survive
/// end-to-end. An UNDECLARED framework problem on the same pack keeps the
/// generic registry envelope.
#[test]
fn declared_problem_response_encodes_with_custom_fields() {
    let bundle = r#"
async function cancel_order(ctx) {
    return {
        __problem: true,
        problem: "validation",
        status: 409,
        detail: "order canceled by owner",
        orderId: "ord_42",
        retryable: true,
    };
}
async function plain_boom(ctx) {
    return { __problem: true, problem: "validation", status: 422, detail: "generic path" };
}
globalThis.__velquFunctionManifest = [["orders.cancel", 0, cancel_order], ["plain.boom", 0, plain_boom]];
globalThis.__velquFunctions = [cancel_order, plain_boom];
"#;
    let dir = temp_dir("probenc");
    let mut pack = fixture_pack();
    pack.bundle = bundle.to_string();
    pack.functions = vec![
        FunctionDecl {
            id: 0,
            key: "orders.cancel".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "plain.boom".into(),
            kind: FunctionKind::RouteHandler,
        },
    ];
    pack.handler_table = std::collections::BTreeMap::from([
        ("orders.cancel".to_string(), "orders.cancel".to_string()),
        ("plain.boom".to_string(), "plain.boom".to_string()),
    ]);
    let route_pos = pack
        .routes
        .iter()
        .position(|r| r.id == "users.get")
        .unwrap();
    let base = pack.routes[route_pos].clone();
    let route =
        |id: &str,
         path: &str,
         idx: u32,
         handler_id: u32,
         allowed: Vec<u16>,
         responses: std::collections::BTreeMap<String, q_pack::ResponseDecl>| {
            let mut r = base.clone();
            r.id = id.into();
            r.handler = id.into();
            r.policy = None;
            r.params = None;
            r.query = None;
            r.body = None;
            r.headers = None;
            r.security.clear();
            r.path = path.into();
            r.path_segments = vec![PathSegment {
                kind: SegKind::Static,
                value: path.trim_start_matches('/').into(),
            }];
            r.responses = responses;
            r.plan = Some(RoutePlanDecl {
                route_id: idx,
                handler_id,
                policy_id: None,
                policy_handler_id: None,
                params_schema_id: None,
                query_schema_id: None,
                headers_schema_id: None,
                body_schema_id: None,
                header_name_ids: vec![],
                query_name_ids: vec![],
                cookie_name_ids: vec![],
                default_status: allowed[0],
                allowed_statuses: allowed,
                field_needs: FieldNeeds::default(),
                response_strategy: Strategy::Native,
                validation_fallback_reason: None,
                response_fallback_reason: None,
                deadline_ms: 5000,
            });
            r
        };
    // declared problem IR for 409: custom type URI + title + detail shape
    let declared: std::collections::BTreeMap<String, q_pack::ResponseDecl> =
        std::collections::BTreeMap::from([(
            "409".to_string(),
            q_pack::ResponseDecl {
                schema: Some("sch:orders.cancel.409".into()),
                strategy: q_pack::Strategy::Native,
                problem: None,
            },
        )]);
    let routes = vec![
        route("orders.cancel", "/orders-cancel", 0, 0, vec![409], declared),
        route(
            "plain.boom",
            "/plain-boom",
            1,
            1,
            vec![422],
            std::collections::BTreeMap::from([(
                "422".to_string(),
                q_pack::ResponseDecl {
                    schema: None,
                    strategy: q_pack::Strategy::Native,
                    problem: Some("validation".into()),
                },
            )]),
        ),
    ];
    pack.routes = routes;
    pack.policies.clear();
    pack.schemas.insert(
        "sch:orders.cancel.409".to_string(),
        q_schema_runtime::SchemaIr::Problem {
            type_uri: Some("https://example.com/problems/order-canceled".into()),
            title: "Order canceled".into(),
            status: 409,
            detail: Some(Box::new(q_schema_runtime::SchemaIr::String {
                min_length: Some(1),
                max_length: Some(64),
                pattern: None,
                format: None,
            })),
        },
    );
    finalize_numeric(&mut pack);
    {
        use sha2::{Digest, Sha256};
        pack.contract_hash = pack.public_contract_sha256()[..32].to_string();
        pack.integrity.bundle_sha256 = {
            let h = Sha256::digest(pack.bundle.as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
        pack.integrity.routes_sha256 = {
            let h = Sha256::digest(pack.routes_canonical_json().as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
    }
    let pack_path = dir.join("probenc.qpack");
    std::fs::write(&pack_path, serde_json::to_vec(&pack).unwrap()).unwrap();

    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port, Duration::from_secs(10));

    // declared problem: generated program — declared title/type, detail,
    // and BOTH custom extension members survive (sorted)
    let r = http(port, "GET /orders-cancel HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 409, "body: {}", r.text());
    // M25-006-D: problem responses carry application/problem+json
    assert_eq!(r.header("content-type"), Some("application/problem+json"));
    let v = r.json();
    // instance keeps its frozen semantics: the request occurrence id
    assert!(
        v["instance"].as_str().unwrap_or("").starts_with("req-"),
        "instance must be the request occurrence id: {}",
        r.text()
    );
    assert_eq!(v["type"], "https://example.com/problems/order-canceled");
    assert_eq!(v["title"], "Order canceled");
    assert_eq!(v["status"], 409);
    assert_eq!(v["detail"], "order canceled by owner");
    assert_eq!(
        v["orderId"],
        "ord_42",
        "custom field must survive: {}",
        r.text()
    );
    assert_eq!(
        v["retryable"],
        true,
        "custom field must survive: {}",
        r.text()
    );
    assert!(v.get("instance").is_some());
    let text = r.text();
    let order: Vec<usize> = [
        "\"type\"",
        "\"title\"",
        "\"status\"",
        "\"instance\"",
        "\"detail\"",
        "\"orderId\"",
        "\"retryable\"",
    ]
    .iter()
    .map(|k| text.find(k).unwrap())
    .collect();
    assert!(
        order.windows(2).all(|w| w[0] < w[1]),
        "canonical order: {text}"
    );

    // undeclared problem: generic registry envelope, no custom fields
    let r = http(port, "GET /plain-boom HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 422, "body: {}", r.text());
    assert_eq!(r.header("content-type"), Some("application/problem+json"));
    let v = r.json();
    assert_eq!(v["type"], "https://velqu.dev/problems/validation");
    assert_eq!(v["title"], "Validation failed");
    assert_eq!(v["detail"], "generic path");

    std::thread::sleep(Duration::from_millis(150));
    child.kill().unwrap();
    let _ = child.wait();
}

/// M25-006-B: a problem settling as the framework's `internal` problem is
/// an unexpected failure — its detail and extension members NEVER cross to
/// the client (they may carry exception text, stacks, or secrets); they are
/// preserved in the internal log only. A declared registry problem on the
/// same pack keeps its detail by design.
#[test]
fn internal_problem_detail_and_extensions_are_redacted() {
    let bundle = r#"
async function leaky(ctx) {
    return {
        __problem: true,
        problem: "internal",
        status: 500,
        detail: "secret-token-abc123 at async boom (app.js:1:1)",
        apiKey: "sk-live-999",
    };
}
async function declared_ok(ctx) {
    return { __problem: true, problem: "validation", status: 422, detail: "field x required" };
}
globalThis.__velquFunctionManifest = [["leaky.handle", 0, leaky], ["declared.ok", 0, declared_ok]];
globalThis.__velquFunctions = [leaky, declared_ok];
"#;
    let dir = temp_dir("probredact");
    let mut pack = fixture_pack();
    pack.bundle = bundle.to_string();
    pack.functions = vec![
        FunctionDecl {
            id: 0,
            key: "leaky.handle".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "declared.ok".into(),
            kind: FunctionKind::RouteHandler,
        },
    ];
    pack.handler_table = std::collections::BTreeMap::from([
        ("leaky.handle".to_string(), "leaky.handle".to_string()),
        ("declared.ok".to_string(), "declared.ok".to_string()),
    ]);
    let route_pos = pack
        .routes
        .iter()
        .position(|r| r.id == "users.get")
        .unwrap();
    let base = pack.routes[route_pos].clone();
    let route = |id: &str, path: &str, idx: u32, handler_id: u32, status: u16| {
        let mut r = base.clone();
        r.id = id.into();
        r.handler = id.into();
        r.policy = None;
        r.params = None;
        r.query = None;
        r.body = None;
        r.headers = None;
        r.security.clear();
        r.path = path.into();
        r.path_segments = vec![PathSegment {
            kind: SegKind::Static,
            value: path.trim_start_matches('/').into(),
        }];
        r.responses = std::collections::BTreeMap::from([(
            status.to_string(),
            q_pack::ResponseDecl {
                schema: None,
                strategy: q_pack::Strategy::Native,
                problem: None,
            },
        )]);
        r.plan = Some(RoutePlanDecl {
            route_id: idx,
            handler_id,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            header_name_ids: vec![],
            query_name_ids: vec![],
            cookie_name_ids: vec![],
            default_status: status,
            allowed_statuses: vec![status],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Native,
            validation_fallback_reason: None,
            response_fallback_reason: None,
            deadline_ms: 5000,
        });
        r
    };
    pack.routes = vec![
        route("leaky.handle", "/leaky", 0, 0, 500),
        route("declared.ok", "/declared-ok", 1, 1, 422),
    ];
    pack.policies.clear();
    finalize_numeric(&mut pack);
    {
        use sha2::{Digest, Sha256};
        pack.contract_hash = pack.public_contract_sha256()[..32].to_string();
        pack.integrity.bundle_sha256 = {
            let h = Sha256::digest(pack.bundle.as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
        pack.integrity.routes_sha256 = {
            let h = Sha256::digest(pack.routes_canonical_json().as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
    }
    let pack_path = dir.join("probredact.qpack");
    std::fs::write(&pack_path, serde_json::to_vec(&pack).unwrap()).unwrap();

    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port, Duration::from_secs(10));

    // internal problem: detail and the apiKey extension member are
    // stripped from the wire entirely
    let r = http(port, "GET /leaky HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 500, "body: {}", r.text());
    let v = r.json();
    assert_eq!(v["type"], "https://velqu.dev/problems/internal");
    assert!(
        v.get("detail").is_none(),
        "internal detail must not cross: {}",
        r.text()
    );
    assert!(
        v.get("apiKey").is_none(),
        "extension member must not cross: {}",
        r.text()
    );
    assert!(!r.text().contains("secret-token-abc123"));
    assert!(!r.text().contains("sk-live-999"));
    assert!(!r.text().contains("app.js"));

    // declared registry problem on the same pack keeps its detail
    let r = http(port, "GET /declared-ok HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 422, "body: {}", r.text());
    assert_eq!(r.json()["detail"], "field x required");

    std::thread::sleep(Duration::from_millis(150));
    child.kill().unwrap();
    let out = child.wait_with_output().unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("problem.redacted"),
        "internal log must record the redacted payload: {err}"
    );
    assert!(err.contains("secret-token-abc123"), "log preserves detail");
}

/// M25-007-B: the raw Response and full Request escape hatches. A
/// `raw-response` capability route returns status/headers/body AS-IS
/// (bypassing response validation by design); a route WITHOUT the
/// capability returning a raw envelope fails closed as a contract
/// violation. A `full-request` route's handler sees EVERY request header
/// and all query pairs through the lazy request store.
#[test]
fn raw_response_and_full_request_escape_hatches() {
    let bundle = r#"
async function raw_handle(ctx) {
    return {
        __velquRaw: true,
        status: 201,
        headers: { "x-custom": "raw", "content-type": "text/plain" },
        body: "raw body as-is",
    };
}
async function raw_rogue(ctx) {
    return { __velquRaw: true, status: 200, body: "should not cross" };
}
async function full_handle(ctx) {
    const req = ctx.request;
    return {
        secret: req.headers["x-undeclared-secret"],
        q: req.query["z"],
        all: Object.keys(req.headers).length,
    };
}
globalThis.__velquFunctionManifest = [["raw.handle", 0, raw_handle], ["raw.rogue", 0, raw_rogue], ["full.handle", 0, full_handle]];
globalThis.__velquFunctions = [raw_handle, raw_rogue, full_handle];
"#;
    let dir = temp_dir("escape");
    let mut pack = fixture_pack();
    pack.bundle = bundle.to_string();
    pack.functions = vec![
        FunctionDecl {
            id: 0,
            key: "raw.handle".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "raw.rogue".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 2,
            key: "full.handle".into(),
            kind: FunctionKind::RouteHandler,
        },
    ];
    pack.handler_table = std::collections::BTreeMap::from([
        ("raw.handle".to_string(), "raw.handle".to_string()),
        ("raw.rogue".to_string(), "raw.rogue".to_string()),
        ("full.handle".to_string(), "full.handle".to_string()),
    ]);
    let route_pos = pack
        .routes
        .iter()
        .position(|r| r.id == "users.get")
        .unwrap();
    let base = pack.routes[route_pos].clone();
    let route = |id: &str, path: &str, idx: u32, handler_id: u32, caps: Vec<&str>| {
        let mut r = base.clone();
        r.id = id.into();
        r.handler = id.into();
        r.policy = None;
        r.params = None;
        r.query = None;
        r.body = None;
        r.headers = None;
        r.security.clear();
        r.path = path.into();
        r.path_segments = vec![PathSegment {
            kind: SegKind::Static,
            value: path.trim_start_matches('/').into(),
        }];
        r.responses = std::collections::BTreeMap::from([
            (
                "200".to_string(),
                q_pack::ResponseDecl {
                    schema: None,
                    strategy: q_pack::Strategy::Native,
                    problem: None,
                },
            ),
            (
                "201".to_string(),
                q_pack::ResponseDecl {
                    schema: None,
                    strategy: q_pack::Strategy::Native,
                    problem: None,
                },
            ),
        ]);
        r.capabilities = caps.into_iter().map(String::from).collect();
        r.plan = Some(RoutePlanDecl {
            route_id: idx,
            handler_id,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            header_name_ids: vec![],
            query_name_ids: vec![],
            cookie_name_ids: vec![],
            default_status: 200,
            allowed_statuses: vec![200, 201],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Native,
            validation_fallback_reason: None,
            response_fallback_reason: None,
            deadline_ms: 5000,
        });
        r
    };
    pack.routes = vec![
        route("raw.handle", "/raw", 0, 0, vec!["raw-response"]),
        route("raw.rogue", "/raw-rogue", 1, 1, vec![]),
        route("full.handle", "/full", 2, 2, vec!["full-request"]),
    ];
    pack.policies.clear();
    finalize_numeric(&mut pack);
    {
        use sha2::{Digest, Sha256};
        pack.contract_hash = pack.public_contract_sha256()[..32].to_string();
        pack.integrity.bundle_sha256 = {
            let h = Sha256::digest(pack.bundle.as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
        pack.integrity.routes_sha256 = {
            let h = Sha256::digest(pack.routes_canonical_json().as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
    }
    let pack_path = dir.join("escape.qpack");
    std::fs::write(&pack_path, serde_json::to_vec(&pack).unwrap()).unwrap();

    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port, Duration::from_secs(10));

    // raw response: declared status, custom headers, body AS-IS
    let r = http(port, "GET /raw HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 201, "body: {}", r.text());
    assert_eq!(r.text(), "raw body as-is");
    assert_eq!(r.header("x-custom"), Some("raw"));
    assert_eq!(r.header("content-type"), Some("text/plain"));

    // rogue raw return without the capability: controlled 500, nothing crosses
    let r = http(port, "GET /raw-rogue HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 500, "body: {}", r.text());
    assert!(!r.text().contains("should not cross"));

    // full request: an UNDECLARED header and an UNDECLARED query pair both
    // reach the handler through the lazy request store
    let r = http(
        port,
        "GET /full?z=9 HTTP/1.1\r\nhost: t\r\nx-undeclared-secret: s3cret\r\n",
        None,
    );
    assert_eq!(r.status, 200, "body: {}", r.text());
    assert_eq!(r.json()["secret"], "s3cret");
    assert_eq!(r.json()["q"], "9");
    assert!(
        r.json()["all"].as_u64().unwrap() >= 2,
        "full header set materialized"
    );

    std::thread::sleep(Duration::from_millis(150));
    child.kill().unwrap();
    let out = child.wait_with_output().unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("raw-response capability"),
        "rogue raw return must log the contract violation: {err}"
    );
}

/// M25-007-C: every fallback path stays bounded and deadline-aware. A
/// busy handler on a js-validation route, a raw-response route, and a
/// full-request route each settles 504 at the route deadline (the engine
/// interrupt kills the execution — no quarantine, the engine keeps
/// serving), and the js-fallback body admission still rejects oversize
/// payloads with 413 before the engine is ever entered.
#[test]
fn fallback_paths_are_bounded_and_deadline_aware() {
    let bundle = r#"
function spin_forever(ctx) {
  while (true) {}
}
async function ok_probe(ctx) {
  return { alive: true };
}
globalThis.__velquFunctionManifest = [["spin.forever", 0, spin_forever], ["ok.probe", 0, ok_probe]];
globalThis.__velquFunctions = [spin_forever, ok_probe];
"#;
    let dir = temp_dir("bounded");
    let mut pack = fixture_pack();
    pack.bundle = bundle.to_string();
    pack.functions = vec![
        FunctionDecl {
            id: 0,
            key: "spin.forever".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "ok.probe".into(),
            kind: FunctionKind::RouteHandler,
        },
    ];
    pack.handler_table = std::collections::BTreeMap::from([
        ("spin.forever".to_string(), "spin.forever".to_string()),
        ("ok.probe".to_string(), "ok.probe".to_string()),
    ]);
    let route_pos = pack
        .routes
        .iter()
        .position(|r| r.id == "users.get")
        .unwrap();
    let base = pack.routes[route_pos].clone();
    let route = |id: &str,
                 method: &str,
                 path: &str,
                 idx: u32,
                 caps: Vec<&str>,
                 needs_body: bool,
                 js_validation: bool| {
        let mut r = base.clone();
        r.id = id.into();
        r.method = method.into();
        r.handler = "spin.forever".into();
        r.policy = None;
        r.params = None;
        r.query = None;
        r.headers = None;
        r.security.clear();
        r.path = path.into();
        r.path_segments = vec![PathSegment {
            kind: SegKind::Static,
            value: path.trim_start_matches('/').into(),
        }];
        r.capabilities = caps.into_iter().map(String::from).collect();
        if needs_body {
            r.body = Some(SourceBinding {
                schema: Some("sch:fallback.body".into()),
                coerce: None,
                content_type: Some("application/json".into()),
                limit_bytes: 65_536,
            });
        } else {
            r.body = None;
        }
        if js_validation {
            r.validation_strategy = Strategy::Js;
        }
        r.responses = std::collections::BTreeMap::from([(
            "200".to_string(),
            q_pack::ResponseDecl {
                schema: None,
                strategy: q_pack::Strategy::Native,
                problem: None,
            },
        )]);
        r.plan = Some(RoutePlanDecl {
            route_id: idx,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: if needs_body { Some(2) } else { None },
            header_name_ids: vec![],
            query_name_ids: vec![],
            cookie_name_ids: vec![],
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds {
                params: false,
                query: false,
                headers: false,
                body: needs_body,
            },
            response_strategy: Strategy::Native,
            validation_fallback_reason: if js_validation {
                Some("explicit".into())
            } else {
                None
            },
            response_fallback_reason: None,
            deadline_ms: 200,
        });
        r.deadline_ms = 200;
        r
    };
    // body schema id: finalize_numeric assigns dense ids over the sorted
    // schema map — patch after finalize below instead of guessing here
    let mut routes = vec![
        route("slow.fb", "POST", "/slow-fb", 0, vec![], true, true),
        route(
            "slow.raw",
            "GET",
            "/slow-raw",
            1,
            vec!["raw-response"],
            false,
            false,
        ),
        route(
            "slow.full",
            "GET",
            "/slow-full",
            2,
            vec!["full-request"],
            false,
            false,
        ),
    ];
    let probe = {
        let mut r = routes[0].clone();
        r.id = "ok.probe".into();
        r.handler = "ok.probe".into();
        r.method = "GET".into();
        r.path = "/ok-probe".into();
        r.path_segments = vec![PathSegment {
            kind: SegKind::Static,
            value: "ok-probe".into(),
        }];
        r.capabilities = vec![];
        r.body = None;
        r.validation_strategy = Strategy::Native;
        if let Some(ref mut p) = r.plan {
            p.route_id = 3;
            p.handler_id = 1;
            p.body_schema_id = None;
            p.field_needs.body = false;
            p.validation_fallback_reason = None;
        }
        r
    };
    routes.push(probe);
    pack.routes = routes;
    pack.policies.clear();
    finalize_numeric(&mut pack);
    // bind the js-fallback body schema id through the manifest
    let fb_sid = pack
        .schema_manifest
        .iter()
        .find(|s| s.key == "sch:fallback.body")
        .map(|s| s.id)
        .unwrap();
    for r in pack.routes.iter_mut() {
        if r.id == "slow.fb" {
            if let Some(ref mut p) = r.plan {
                p.body_schema_id = Some(fb_sid);
            }
        }
    }
    {
        use sha2::{Digest, Sha256};
        pack.contract_hash = pack.public_contract_sha256()[..32].to_string();
        pack.integrity.bundle_sha256 = {
            let h = Sha256::digest(pack.bundle.as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
        pack.integrity.routes_sha256 = {
            let h = Sha256::digest(pack.routes_canonical_json().as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
    }
    let pack_path = dir.join("bounded.qpack");
    std::fs::write(&pack_path, serde_json::to_vec(&pack).unwrap()).unwrap();

    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port, Duration::from_secs(10));

    // js-validation fallback route: busy handler dies at the 200ms deadline
    let t0 = Instant::now();
    let r = http(
        port,
        "POST /slow-fb HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(br#"{"any":"thing"}"#),
    );
    assert_eq!(r.status, 504, "body: {}", r.text());
    assert!(
        t0.elapsed() < Duration::from_secs(2),
        "js fallback must settle at the deadline, took {:?}",
        t0.elapsed()
    );

    // raw-response route: the deadline applies BEFORE any raw mapping
    let r = http(port, "GET /slow-raw HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 504, "body: {}", r.text());

    // full-request route: same deadline ownership
    let r = http(port, "GET /slow-full HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 504, "body: {}", r.text());

    // interrupt kills are NOT quarantine: the engine keeps serving
    let r = http(port, "GET /ok-probe HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200, "engine must stay healthy: {}", r.text());
    assert_eq!(r.json()["alive"], true);

    // size bound on the fallback admission: oversize body rejects 413
    // before the engine is ever entered (the busy handler never runs)
    let big = vec![b'x'; 70_000];
    let r = http(
        port,
        "POST /slow-fb HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(&big),
    );
    assert_eq!(r.status, 413, "body: {}", r.text());

    std::thread::sleep(Duration::from_millis(150));
    child.kill().unwrap();
    let _ = child.wait();
}

/// M26-002-D: garbage bytecode with MATCHING integrity never silently
/// serves from source — startup rejects loudly before ready. (The
/// explicit source path from M26-002-C is a flag, never an automatic
/// fallback.)
#[test]
fn hash_valid_garbage_bytecode_rejects_before_ready() {
    // fixture pack with real embedded bytecode
    let dir = temp_dir("nofallback");
    let pack_path = write_pack(&dir);
    let bytecode_bin = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().and_then(|p| p.parent().map(|p| p.to_path_buf())))
        .map(|p| p.join("velqu-bytecode"))
        .expect("locate target dir");
    let bc_path = dir.join("app-bc.qpack");
    let out = Command::new(&bytecode_bin)
        .args([
            "embed",
            "--pack",
            pack_path.to_str().unwrap(),
            "--out",
            bc_path.to_str().unwrap(),
        ])
        .output()
        .expect("embed bytecode");
    assert!(out.status.success(), "embed failed");

    // replace the bytecode DATA with garbage and RECOMPUTE the integrity
    // hash so integrity passes — only the load itself can catch this
    let raw: Value = serde_json::from_slice(&std::fs::read(&bc_path).unwrap()).unwrap();
    let mut garbage_pack = raw.clone();
    let garbage = b"garbage-bytecode-not-a-module".to_vec();
    garbage_pack["bundleBytecode"]["data"] = q_pack::base64_encode(&garbage).into();
    {
        use sha2::{Digest, Sha256};
        let h = Sha256::digest(&garbage);
        let hex: String = h.iter().map(|b| format!("{:02x}", b)).collect();
        garbage_pack["integrity"]["bytecodeSha256"] = json!(hex);
    }
    let garbage_path = dir.join("app-garbage.qpack");
    std::fs::write(&garbage_path, serde_json::to_vec(&garbage_pack).unwrap()).unwrap();

    // startup rejects loudly: integrity + fingerprint pass, the bytecode
    // LOAD fails — never a silent source fallback
    let port = free_port();
    let out = Command::new(env!("CARGO_BIN_EXE_velqu-runtime"))
        .arg("--pack")
        .arg(&garbage_path)
        .arg("--port")
        .arg(port.to_string())
        .output()
        .expect("spawn runtime");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "garbage bytecode must not serve");
    assert!(
        stderr.contains("bytecode load failed") || stderr.contains("startup.rejected"),
        "expected loud rejection, got: {stderr}"
    );
    // never silently sourced: nothing listened
    assert!(std::net::TcpStream::connect(("127.0.0.1", port)).is_err());
}

/// M26-002-C: the explicit source-rebuild path. A pack whose embedded
/// bytecode targets a different machine rejects on normal startup (the
/// message names --no-bytecode and the rebuild remedy); the SAME pack
/// serves from the verified source bundle under `--no-bytecode`.
#[test]
fn no_bytecode_flag_recovers_cross_target_packs_from_source() {
    // build the standard fixture pack and embed host-matching bytecode
    let dir = temp_dir("srcpath");
    let pack_path = write_pack(&dir);
    // velqu-bytecode is a separate workspace bin: resolve it from the
    // test executable's target dir (target/debug/deps -> target/debug)
    let bin = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().and_then(|p| p.parent().map(|p| p.to_path_buf())))
        .map(|p| p.join("velqu-bytecode"))
        .expect("locate target dir");
    let bc_path = dir.join("app-bc.qpack");
    let out = Command::new(bin)
        .args([
            "embed",
            "--pack",
            pack_path.to_str().unwrap(),
            "--out",
            bc_path.to_str().unwrap(),
        ])
        .output()
        .expect("embed bytecode");
    assert!(
        out.status.success(),
        "embed failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // flip the bytecode target to a foreign arch (bytecode data hash is
    // untouched — only the target fingerprint lies)
    let raw: Value = serde_json::from_slice(&std::fs::read(&bc_path).unwrap()).unwrap();
    let mut foreign = raw.clone();
    foreign["bundleBytecode"]["target"]["arch"] = json!("bogus-arch");
    let foreign_path = dir.join("app-foreign.qpack");
    std::fs::write(&foreign_path, serde_json::to_vec(&foreign).unwrap()).unwrap();

    let port = free_port();

    // normal startup: cross-target rejection names both recovery paths
    let out = Command::new(env!("CARGO_BIN_EXE_velqu-runtime"))
        .arg("--pack")
        .arg(&foreign_path)
        .arg("--port")
        .arg(port.to_string())
        .output()
        .expect("spawn runtime");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "cross-target pack must not start");
    assert!(stderr.contains("cross-target pack rejected"), "{stderr}");
    assert!(stderr.contains("--no-bytecode"), "{stderr}");
    assert!(stderr.contains("rebuild the pack"), "{stderr}");

    // the explicit source path: the SAME pack serves from source
    let port2 = free_port();
    let mut child = Command::new(env!("CARGO_BIN_EXE_velqu-runtime"))
        .arg("--pack")
        .arg(&foreign_path)
        .arg("--port")
        .arg(port2.to_string())
        .arg("--no-bytecode")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port2, Duration::from_secs(10));
    let r = http(port2, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200, "source path must serve: {}", r.text());
    assert_eq!(r.text(), "{\"status\":\"ok\"}");
    child.kill().unwrap();
    let _ = child.wait();
}

/// M27-003-D: the full context profile is always available and is the
/// explicit compatibility baseline. The fixture bundle touches
/// `Map`, so: (1) forcing `--context-profile full` serves normally;
/// (2) an unknown profile name fails closed BEFORE serving with the
/// known-set named; (3) forcing `minimal` on a Map-using bundle fails
/// loudly at bundle load instead of silently misbehaving.
#[test]
fn full_profile_retained_for_compatibility_testing() {
    let dir = temp_dir("ctxprofile");
    let pack_path = write_pack(&dir);
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");

    // 1. forced FULL serves: the compatibility baseline always works.
    let port1 = free_port();
    let mut child = Command::new(bin)
        .args([
            "--pack",
            pack_path.to_str().unwrap(),
            "--port",
            &port1.to_string(),
            "--context-profile",
            "full",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port1, Duration::from_secs(10));
    let r = http(port1, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200, "forced full profile must serve");
    // M27-003-V: profile identity enters the runtime startup identity
    // block — the ready line names the serving profile. Drain the
    // piped stdout only after kill (a serving process never EOFs).
    child.kill().unwrap();
    let out1 = child.wait_with_output().unwrap();
    let stdout1 = String::from_utf8_lossy(&out1.stdout);
    assert!(
        stdout1.contains("\"contextProfile\":\"full\""),
        "ready line must carry the serving profile: {stdout1}"
    );

    // 2. unknown profile name fails closed before ready, naming the set.
    let out = Command::new(bin)
        .args([
            "--pack",
            pack_path.to_str().unwrap(),
            "--context-profile",
            "bogus",
        ])
        .output()
        .expect("spawn runtime");
    assert_eq!(out.status.code(), Some(2), "unknown profile must exit 2");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("unknown context profile 'bogus'"),
        "{stderr}"
    );
    assert!(stderr.contains("full, web, minimal"), "{stderr}");

    // 3. forced MINIMAL on this bundle: startup SUCCEEDS (the Map use
    //    is lazy inside users()), and the degraded route fails LOUDLY
    //    per request as a redacted internal problem — never a silent
    //    wrong answer. Full-profile retention (section 1) is what keeps
    //    compatibility testing possible.
    let port3 = free_port();
    let mut child3 = Command::new(bin)
        .args([
            "--pack",
            pack_path.to_str().unwrap(),
            "--port",
            &port3.to_string(),
            "--context-profile",
            "minimal",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port3, Duration::from_secs(10));
    let r3 = http(port3, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(
        r3.status, 200,
        "minimal profile must still start and serve livez"
    );
    let r3b = http(
        port3,
        "GET /users/usr_1 HTTP/1.1\r\nhost: t\r\nauthorization: Bearer q-demo-token\r\n",
        None,
    );
    assert_ne!(
        r3b.status, 200,
        "Map-using route must not silently succeed on minimal"
    );
    assert_eq!(r3b.status, 500);
    assert_eq!(
        r3b.json()["type"],
        "https://velqu.dev/problems/internal",
        "body: {}",
        r3b.text()
    );
    child3.kill().unwrap();
    let _ = child3.wait();
}

/// M25-005-D: the QuickJS stringify fallback stays retained and correct
/// next to the direct encoder. Twin routes share one declared schema; the
/// native route encodes through the generated program (declared property
/// order), the js route stringifies in the engine (handler insertion
/// order, host validation skipped per the disclosed fallback). Both
/// responses are 200 and JSON-equal — the fallback remains selectable
/// (e.g. via the `measured` fallback marker) without correctness drift.
#[test]
fn quickjs_stringify_fallback_stays_json_equivalent_to_encoder() {
    let bundle = r#"
async function twin_shape(ctx) {
    // handler key order deliberately differs from the declared order
    return { zeta: "z", alpha: 7 };
}
globalThis.__velquFunctionManifest = [["twin.native", 0, twin_shape]];
globalThis.__velquFunctions = [twin_shape];
"#;
    let dir = temp_dir("respenc-d");
    let mut pack = fixture_pack();
    pack.bundle = bundle.to_string();
    pack.functions = vec![FunctionDecl {
        id: 0,
        key: "twin.native".into(),
        kind: FunctionKind::RouteHandler,
    }];
    pack.handler_table = std::collections::BTreeMap::from([
        ("twin.native".to_string(), "twin.native".to_string()),
        ("twin.js".to_string(), "twin.js".to_string()),
    ]);
    let route_pos = pack
        .routes
        .iter()
        .position(|r| r.id == "users.get")
        .unwrap();
    let base = pack.routes[route_pos].clone();
    let plan = |strategy: Strategy| RoutePlanDecl {
        route_id: 0,
        handler_id: 0,
        policy_id: None,
        policy_handler_id: None,
        params_schema_id: None,
        query_schema_id: None,
        headers_schema_id: None,
        body_schema_id: None,
        header_name_ids: vec![],
        query_name_ids: vec![],
        cookie_name_ids: vec![],
        default_status: 200,
        allowed_statuses: vec![200],
        field_needs: FieldNeeds::default(),
        response_strategy: strategy,
        validation_fallback_reason: None,
        response_fallback_reason: match strategy {
            Strategy::Js => Some("explicit".into()),
            _ => None,
        },
        deadline_ms: 5000,
    };
    let twin = |id: &str, path: &str, strategy: Strategy| {
        let mut r = base.clone();
        r.id = id.into();
        r.handler = "twin.native".into();
        r.policy = None;
        r.params = None;
        r.query = None;
        r.body = None;
        r.headers = None;
        r.security.clear();
        r.path = path.into();
        r.path_segments = vec![PathSegment {
            kind: SegKind::Static,
            value: path.trim_start_matches('/').into(),
        }];
        r.responses = std::collections::BTreeMap::from([(
            "200".to_string(),
            q_pack::ResponseDecl {
                schema: Some("sch:twin.200".to_string()),
                strategy: match strategy {
                    Strategy::Js => q_pack::Strategy::Js,
                    _ => q_pack::Strategy::Native,
                },
                problem: None,
            },
        )]);
        r.plan = Some(plan(strategy));
        r
    };
    // native route at index 0, js route at index 1 — plan.route_id must
    // match the route index, so patch the js plan's ids after building
    let native = twin("twin.native", "/twin-native", Strategy::Native);
    let mut js = twin("twin.js", "/twin-js", Strategy::Js);
    if let Some(ref mut p) = js.plan {
        p.route_id = 1;
    }
    pack.routes = vec![native, js];
    pack.policies.clear();
    pack.schemas.insert(
        "sch:twin.200".to_string(),
        q_schema_runtime::SchemaIr::Object {
            properties: std::collections::BTreeMap::from([
                (
                    "zeta".to_string(),
                    Box::new(q_schema_runtime::SchemaIr::String {
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        format: None,
                    }),
                ),
                (
                    "alpha".to_string(),
                    Box::new(q_schema_runtime::SchemaIr::Integer {
                        minimum: None,
                        maximum: None,
                    }),
                ),
            ]),
            required: vec!["zeta".into(), "alpha".into()],
        },
    );
    finalize_numeric(&mut pack);
    {
        use sha2::{Digest, Sha256};
        pack.contract_hash = pack.public_contract_sha256()[..32].to_string();
        pack.integrity.bundle_sha256 = {
            let h = Sha256::digest(pack.bundle.as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
        pack.integrity.routes_sha256 = {
            let h = Sha256::digest(pack.routes_canonical_json().as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
    }
    let pack_path = dir.join("respenc-d.qpack");
    std::fs::write(&pack_path, serde_json::to_vec(&pack).unwrap()).unwrap();

    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port, Duration::from_secs(10));

    // native twin: one-traversal encoder, declared property order
    let native_resp = http(port, "GET /twin-native HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(native_resp.status, 200, "body: {}", native_resp.text());
    assert_eq!(native_resp.text(), "{\"alpha\":7,\"zeta\":\"z\"}");

    // js twin: QuickJS stringify fallback retained — handler insertion
    // order crosses unvalidated (disclosed per-route in the build report)
    let js_resp = http(port, "GET /twin-js HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(js_resp.status, 200, "body: {}", js_resp.text());
    assert_eq!(js_resp.text(), "{\"zeta\":\"z\",\"alpha\":7}");

    // JSON-equivalence of the retained fallback and the generated encoder
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&native_resp.body).unwrap(),
        serde_json::from_slice::<serde_json::Value>(&js_resp.body).unwrap(),
        "js fallback and native encoder must stay JSON-equal"
    );

    std::thread::sleep(Duration::from_millis(150));
    child.kill().unwrap();
    let _ = child.wait();
}

#[test]
fn embedded_prelude_pack_serves_identically_and_source_recovery_works() {
    // M26-004-D: the compiled module bytecode contains prelude + handler
    // manifest (bundlePrelude "embedded"); startup evaluates zero prelude
    // source. Serving must be identical, and --no-bytecode must recover
    // via the host prelude + source bundle.
    let dir = temp_dir("embedded");
    let pack_path = write_pack(&dir);
    let mut pack = q_pack::QPack::load_and_verify(&pack_path).unwrap();

    let module_source = format!("{}\n{}", q_engine_quickjs::prelude::PRELUDE, pack.bundle);
    let rt = rquickjs::Runtime::new().unwrap();
    let ctx = rquickjs::Context::full(&rt).unwrap();
    let bytecode_bytes = ctx
        .with(|ctx| -> rquickjs::Result<Vec<u8>> {
            let module = rquickjs::Module::declare(ctx.clone(), "app.js", module_source.as_str())?;
            module.write(rquickjs::WriteOptions {
                endianness: rquickjs::WriteOptionsEndianness::Native,
                ..Default::default()
            })
        })
        .unwrap();
    let bc_sha = {
        use sha2::{Digest, Sha256};
        let h = Sha256::digest(&bytecode_bytes);
        h.iter().map(|b| format!("{b:02x}")).collect::<String>()
    };
    let endian = if cfg!(target_endian = "big") {
        "big"
    } else {
        "little"
    }
    .to_string();
    pack.bundle_form = Some("module".to_string());
    pack.bundle_prelude = Some("embedded".to_string());
    pack.bundle_bytecode = Some(q_pack::BundleBytecode {
        quickjs: q_pack::ENGINE_VERSION.to_string(),
        binding: q_pack::ENGINE_BINDING.to_string(),
        endianness: endian.clone(),
        target: Some(q_pack::BytecodeTarget {
            arch: std::env::consts::ARCH.to_string(),
            os: std::env::consts::OS.to_string(),
            pointer_width: (std::mem::size_of::<usize>() * 8) as u8,
            endianness: endian,
        }),
        data: q_pack::base64_encode(&bytecode_bytes),
    });
    pack.integrity.bytecode_sha256 = Some(bc_sha);
    let bc_path = dir.join("app-embedded.qpack");
    std::fs::write(&bc_path, serde_json::to_vec(&pack).unwrap()).unwrap();

    // bytecode mode: prelude came from the module — serving identical
    let port = free_port();
    let server = Server::start(&bc_path, port);
    let r = http(port, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"status\":\"ok\"}");
    let r = http(port, "GET /hello/Rafi HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"message\":\"Hello Rafi\"}");
    let r = http(port, "GET /js-json HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"ok\":true}");
    server.stop();

    // explicit source recovery: host prelude evaluates, source bundle runs
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let port2 = free_port();
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&bc_path)
        .arg("--port")
        .arg(port2.to_string())
        .arg("--no-bytecode")
        .spawn()
        .unwrap();
    let mut ready = false;
    for _ in 0..100 {
        if std::net::TcpStream::connect(("127.0.0.1", port2)).is_ok() {
            ready = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    assert!(ready, "source recovery must boot");
    let r = http(port2, "GET /hello/Rafi HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"message\":\"Hello Rafi\"}");
    let _ = child.kill();
    let _ = child.wait();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn bytecode_pack_serves_identically_and_mismatch_fails_before_ready() {
    let dir = temp_dir("bytecode");
    let pack_path = write_pack(&dir);

    // 1. Embed valid bytecode
    let mut pack = q_pack::QPack::load_and_verify(&pack_path).unwrap();
    let rt = rquickjs::Runtime::new().unwrap();
    let ctx = rquickjs::Context::full(&rt).unwrap();
    let bytecode_bytes = ctx
        .with(|ctx| -> rquickjs::Result<Vec<u8>> {
            let module = rquickjs::Module::declare(ctx.clone(), "app.js", pack.bundle.as_str())?;
            module.write(rquickjs::WriteOptions {
                endianness: rquickjs::WriteOptionsEndianness::Native,
                ..Default::default()
            })
        })
        .unwrap();

    let bc_sha = {
        use sha2::{Digest, Sha256};
        let h = Sha256::digest(&bytecode_bytes);
        h.iter().map(|b| format!("{b:02x}")).collect::<String>()
    };
    let bc_b64 = q_pack::minimal_pack_public(); // dummy call to ensure symbols
    let _ = bc_b64;

    // encode base64
    let b64 = q_pack::base64_encode(&bytecode_bytes);

    pack.bundle_form = Some("module".to_string());
    let endian = if cfg!(target_endian = "big") {
        "big"
    } else {
        "little"
    }
    .to_string();
    pack.bundle_bytecode = Some(q_pack::BundleBytecode {
        quickjs: q_pack::ENGINE_VERSION.to_string(),
        binding: q_pack::ENGINE_BINDING.to_string(),
        endianness: endian.clone(),
        target: Some(q_pack::BytecodeTarget {
            arch: std::env::consts::ARCH.to_string(),
            os: std::env::consts::OS.to_string(),
            pointer_width: (std::mem::size_of::<usize>() * 8) as u8,
            endianness: endian,
        }),
        data: b64.clone(),
    });
    pack.integrity.bytecode_sha256 = Some(bc_sha);

    let bc_pack_path = dir.join("app-bc.qpack");
    std::fs::write(&bc_pack_path, serde_json::to_vec(&pack).unwrap()).unwrap();

    // 2. Server boots with bytecode and serves C0 + C3
    let port = free_port();
    let server = Server::start(&bc_pack_path, port);
    let r = http(port, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"status\":\"ok\"}");
    let r = http(port, "GET /hello/Rafi HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"message\":\"Hello Rafi\"}");
    server.stop();

    // 3. Tampered bytecode is rejected before ready
    let mut tampered_pack = pack.clone();
    let mut bad_b64 = b64.clone();
    bad_b64.replace_range(10..11, "Z");
    tampered_pack.bundle_bytecode.as_mut().unwrap().data = bad_b64;
    let bad_path = dir.join("bad-bc.qpack");
    std::fs::write(&bad_path, serde_json::to_vec(&tampered_pack).unwrap()).unwrap();

    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let out = Command::new(bin)
        .arg("--pack")
        .arg(&bad_path)
        .arg("--port")
        .arg(free_port().to_string())
        .output()
        .unwrap();
    assert_ne!(
        out.status.code(),
        Some(0),
        "tampered bytecode must exit non-zero"
    );
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("bytecode sha256 mismatch") || err.contains("integrity"),
        "stderr: {err}"
    );
}
