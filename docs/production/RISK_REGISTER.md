# Production Program Risk Register

| Risk | Trigger | Mitigation / decision gate |
|---|---|---|
| QuickJS interpreter remains materially behind JIT for realistic dynamic workloads | M2.5/M5 evidence below approved threshold | Position Velqu for cold/high-density workloads or add generated Bun target; do not embed JSC/V8 impulsively. |
| Bridge work erases cold-start value | C1–C4 fixed overhead remains high after M2.4/M2.5 | Revisit request/response representation under ADR; retain measured negative results. |
| QPack/bytecode ABI causes unsafe or confusing upgrades | Engine/build mismatch or silent fallback | Strict fingerprint, fail closed, migration tooling, release compatibility matrix. |
| Capability ecosystem bloats core | Unused app pays startup/RSS cost | Compile-time linker, size budget per capability, no default-all feature. |
| Fetch introduces SSRF or unbounded streaming | DNS/redirect/private-address bypass, memory growth | M2.8 security ADR, revalidation, limits, backpressure, independent review. |
| Multi-worker hides single-worker defects | Scaling added before M2.3–M2.8 gates | Critical path forbids M3 integration before prior gates. |
| Developer mode differs from release runtime | Bun-local behavior passes while QuickJS fails | Actual-runtime `velqu dev`, dev/release manifest parity tests. |
| Evidence drift creates false claims | Reports disagree with source/raw data | Machine-readable evidence index and verify parity checks. |
| Production scope expands without closure | WebSocket/ORM/Node compatibility enters early | Scope-change rule and ADR; move optional features post-GA. |
| One maintainer becomes release/security bottleneck | Owner decisions/governance unresolved | Resolve governance and security authority before RC. |
| Platform packaging is fragile | Cross-build succeeds but runtime fails on architecture | Run artifacts on real x86_64/arm64 systems before support claim. |
| Real-world benchmark is database-limited and hides framework costs | All candidates look identical at 10ms DB latency | Controlled I/O, CPU/JIT crossover, queue/pool/CPU metrics. |
