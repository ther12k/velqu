//! BWASM-K-005 — kernel request-path tests. The same binary runs
//! natively and on-target (`cargo test -p q-browser-kernel --target
//! wasm32-wasip1` via the K-004 harness), pinning the ADR-0037 §3
//! proof: Request → WASM plan → handler → WASM completion → Response.

use q_browser_kernel::{problem_ids, BrowserKernel, KERNEL_ABI_VERSION};
use q_pack::CapabilityInventoryEntryWire;
use serde_json::{json, Value};

/// The fuzz-support minimal pack (health.live route, 200-only) as
/// verified bytes.
fn pack_bytes() -> Vec<u8> {
    serde_json::to_vec(&q_pack::minimal_pack_public()).unwrap()
}

/// Minimal pack whose route declares `runtime:text` and whose
/// inventory carries it (integrity + inventory hash recomputed — same
/// construction the q-pack suite uses).
fn pack_with_capability() -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut pack = q_pack::minimal_pack_public();
    pack.routes[0].capabilities = vec!["runtime:text".into()];
    let inv = q_capabilities::CapabilityInventory::from_pairs(&[("runtime:text".to_string(), 1)])
        .unwrap();
    pack.capability_inventory = Some(
        inv.entries()
            .iter()
            .map(|e| CapabilityInventoryEntryWire {
                id: e.id.to_string(),
                version: e.version.0,
            })
            .collect(),
    );
    pack.capability_inventory_sha256 = Some(inv.sha256_hex());
    pack.integrity.bundle_sha256 = hex(&Sha256::digest(pack.bundle.as_bytes()));
    pack.integrity.routes_sha256 = hex(&Sha256::digest(pack.routes_canonical_json().as_bytes()));
    serde_json::to_vec(&pack).unwrap()
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn kernel() -> BrowserKernel {
    BrowserKernel::init(&pack_bytes()).expect("minimal pack verifies")
}

fn plan(k: &BrowserKernel, method: &str, path: &str) -> Value {
    let msg = json!({
        "abiVersion": KERNEL_ABI_VERSION,
        "method": method,
        "path": path,
    })
    .to_string();
    serde_json::from_str(&k.plan_request(&msg)).unwrap()
}

fn complete(k: &BrowserKernel, route_id: u32, result: Value) -> Value {
    let msg = json!({
        "abiVersion": KERNEL_ABI_VERSION,
        "routeId": route_id,
        "result": result,
    })
    .to_string();
    serde_json::from_str(&k.complete_invocation(&msg)).unwrap()
}

#[test]
fn init_verifies_and_exposes_abi() {
    let _k = kernel();
    assert_eq!(BrowserKernel::abi_version(), KERNEL_ABI_VERSION);
    assert_eq!(KERNEL_ABI_VERSION, 1);
}

#[test]
fn init_rejects_tampered_pack_bytes() {
    let mut bytes = pack_bytes();
    // Corrupt deep inside the bundle region; verification must fail
    // closed with an artifact problem.
    let mid = bytes.len() / 2;
    bytes[mid] = b'X';
    let err = BrowserKernel::init(&bytes).unwrap_err();
    assert_eq!(err.problem_id, problem_ids::ARTIFACT);
    assert_eq!(err.status, 500);
}

// Native-only: the 16 MiB oversize allocation SIGSEGVs under Node's
// WASI preview1 (runner memory-growth limitation; wasmtime-class
// runtimes are unaffected). The kernel path under test is a single
// length comparison — fully covered natively; all other kernel tests
// execute on-target.
#[cfg(not(target_os = "wasi"))]
#[test]
fn init_rejects_oversized_pack() {
    let mut bytes = vec![b' '; q_browser_kernel::MAX_PACK_BYTES + 1];
    bytes[0] = b'{';
    let err = BrowserKernel::init(&bytes).unwrap_err();
    assert_eq!(err.problem_id, problem_ids::ARTIFACT);
    assert!(err.detail.clone().unwrap().contains("bytes"), "{err:?}");
}

#[test]
fn plan_health_live_returns_invoke_plan() {
    let k = kernel();
    let out = plan(&k, "GET", "/health/live");
    assert_eq!(out["kind"], "invoke");
    assert_eq!(out["handlerKey"], "health.live");
    assert_eq!(out["allowedStatuses"], json!([200]));
    assert_eq!(out["defaultStatus"], 200);
    assert_eq!(out["deadlineMs"], 5000);
    assert_eq!(out["abiVersion"], KERNEL_ABI_VERSION);
}

#[test]
fn plan_unknown_route_is_stable_not_found_problem() {
    let k = kernel();
    let out = plan(&k, "GET", "/nope");
    assert_eq!(out["kind"], "problem");
    let p = &out["problem"];
    assert_eq!(p["problemId"], "not-found");
    assert_eq!(p["type"], "https://velqu.dev/problems/not-found");
    assert_eq!(p["status"], 404);
}

#[test]
fn plan_wrong_method_is_405_with_allow() {
    let k = kernel();
    let out = plan(&k, "POST", "/health/live");
    assert_eq!(out["kind"], "problem");
    let p = &out["problem"];
    assert_eq!(p["problemId"], "method");
    assert_eq!(p["status"], 405);
    assert_eq!(p["allow"], json!(["GET", "HEAD"]));
}

#[test]
fn plan_abi_mismatch_is_stable_problem() {
    let k = kernel();
    let msg = json!({"abiVersion": 0, "method": "GET", "path": "/health/live"}).to_string();
    let out: Value = serde_json::from_str(&k.plan_request(&msg)).unwrap();
    assert_eq!(out["problem"]["problemId"], "abi");
    assert_eq!(out["problem"]["status"], 400);
    assert!(out["problem"]["detail"]
        .as_str()
        .unwrap()
        .contains("kernel ABI 1"));
}

#[test]
fn plan_oversized_message_is_limit_problem() {
    let k = kernel();
    let mut msg = String::from(r#"{"abiVersion":1,"method":"GET","path":"/"#);
    while msg.len() <= q_browser_kernel::MAX_MESSAGE_BYTES {
        msg.push('a');
    }
    msg.push_str("\"}");
    let out: Value = serde_json::from_str(&k.plan_request(&msg)).unwrap();
    assert_eq!(out["problem"]["problemId"], "limit");
    assert_eq!(out["problem"]["status"], 413);
}

#[test]
fn plan_malformed_message_is_body_problem() {
    let k = kernel();
    let out: Value = serde_json::from_str(&k.plan_request("not json")).unwrap();
    assert_eq!(out["problem"]["problemId"], "body");
}

#[test]
fn complete_declared_status_normalizes_response() {
    let k = kernel();
    let out = complete(
        &k,
        0,
        json!({"kind":"response","status":200,"headers":[["content-type","application/json"]],"body":{"status":"ok"}}),
    );
    assert_eq!(out["kind"], "response");
    assert_eq!(out["status"], 200);
    assert_eq!(out["body"]["status"], "ok");
    assert_eq!(out["headers"][0][0], "content-type");
}

#[test]
fn complete_undeclared_status_is_contract_violation() {
    let k = kernel();
    let out = complete(&k, 0, json!({"kind":"response","status":418,"body":{}}));
    assert_eq!(out["problem"]["problemId"], "internal");
    assert_eq!(out["problem"]["status"], 500);
    assert!(out["problem"]["detail"]
        .as_str()
        .unwrap()
        .contains("undeclared status 418"));
}

#[test]
fn complete_handler_problem_normalizes_through_registry() {
    let k = kernel();
    let out = complete(
        &k,
        0,
        json!({"kind":"problem","problemId":"validation","detail":"bad input","errors":[{"path":"name","code":"string","message":"required"}]}),
    );
    assert_eq!(out["problem"]["problemId"], "validation");
    assert_eq!(
        out["problem"]["type"],
        "https://velqu.dev/problems/validation"
    );
    assert_eq!(out["problem"]["status"], 400);
    assert_eq!(out["problem"]["errors"][0]["path"], "name");
}

#[test]
fn complete_abi_mismatch_and_unknown_route_are_stable_problems() {
    let k = kernel();
    let msg = json!({"abiVersion": 99, "routeId": 0, "result": {"kind":"response","status":200}})
        .to_string();
    let out: Value = serde_json::from_str(&k.complete_invocation(&msg)).unwrap();
    assert_eq!(out["problem"]["problemId"], "abi");

    let msg2 = json!({"abiVersion": 1, "routeId": 77, "result": {"kind":"response","status":200}})
        .to_string();
    let out2: Value = serde_json::from_str(&k.complete_invocation(&msg2)).unwrap();
    assert_eq!(out2["problem"]["problemId"], "internal");
    assert!(out2["problem"]["detail"]
        .as_str()
        .unwrap()
        .contains("unknown route"));
}

#[test]
fn plan_authorizes_route_capabilities_against_inventory() {
    // Route declares runtime:text and the inventory carries it → plan ok.
    let k = BrowserKernel::init(&pack_with_capability()).unwrap();
    let out = plan(&k, "GET", "/health/live");
    assert_eq!(out["kind"], "invoke", "{out}");

    // Route declares a capability the inventory does NOT carry → the
    // ADR-0037 §5 deployment-required problem, fail closed at PLAN time.
    use sha2::{Digest, Sha256};
    let mut pack = q_pack::minimal_pack_public();
    pack.routes[0].capabilities = vec!["runtime:text".into()];
    // No capability inventory at all.
    pack.integrity.bundle_sha256 = hex(&Sha256::digest(pack.bundle.as_bytes()));
    pack.integrity.routes_sha256 = hex(&Sha256::digest(pack.routes_canonical_json().as_bytes()));
    let k2 = BrowserKernel::init(&serde_json::to_vec(&pack).unwrap()).unwrap();
    let out2 = plan(&k2, "GET", "/health/live");
    assert_eq!(out2["problem"]["problemId"], "capability");
    assert_eq!(out2["problem"]["status"], 501);
    assert!(out2["problem"]["detail"]
        .as_str()
        .unwrap()
        .contains("runtime:text"));
}

#[test]
fn authorize_capability_query_fail_closed_without_inventory() {
    let k = kernel(); // minimal pack: no inventory
    let denied: Value = serde_json::from_str(&k.authorize_capability("runtime:text")).unwrap();
    assert_eq!(denied["problem"]["problemId"], "capability");

    let ok = BrowserKernel::init(&pack_with_capability()).unwrap();
    let granted: Value = serde_json::from_str(&ok.authorize_capability("runtime:text")).unwrap();
    assert_eq!(granted["authorized"], true);
}
