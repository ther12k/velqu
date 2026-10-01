# notes-service external consumer E2E transcript (#1404)

Captured 2026-10-01T16:59:12Z on the dev host — pinned Bun 1.4.0,
registry.npmjs.org access, docker daemon, locally built release runtime.
Ephemeral postgres:16-alpine (digest in transcript), removed on exit.
Command: BUN=bun scripts/beta-external/notes-service.sh

== notes-service external consumer transcript ==
bun=1.4.0 runtime=/home/ther12k/Workspace/Learning/velqu-worktrees/notes-consumer-a/target/release/velqu-runtime docker-image=postgres:16-alpine app=/tmp/velqu-notes-wtxO9Z/notes-service

== [1] copy template to /tmp/velqu-notes-wtxO9Z/notes-service (clean consumer tree)

== [2] bun install (public registry — no workspace links)
bun install v1.4.0 (34cbb9a40)
Saved lockfile

+ @types/bun@1.4.2
+ @velqu/cli@0.1.0-beta.1
+ typescript@5.9.3 (v7.0.2 available)
+ @velqu/core@0.1.0-beta.1
+ @velqu/schema@0.1.0-beta.1
+ @velqu/treaty@0.1.0-beta.1

12 packages installed [14.00ms]

== [3] velqu check
$ velqu check --project .
velqu check: 5 routes in . — clean

== [4] velqu build -> dist/app.qpack (+ generated contract artifacts)
$ velqu build --project .
velqu build [serverless]: 5 routes → dist in 775ms
  app.qpack  22215B
  route-manifest.json  2746B
  schema-manifest.json  4187B
  capability-manifest.json  847B
  contract.json  5825B
  contract.d.ts  1755B
  contract.meta.json  934B
  openapi.json  10931B
  contract.lock.json  5797B
  build-report.json  11522B
  app.qpack.sources.json  51676B
  published-manifest.json  970B

== [5] bun test (build ran first — the client imports generated artifacts)
bun test v1.4.0 (34cbb9a40)

src/client.test.ts:
skipping: no server on 127.0.0.1:32873
(pass) notes API (runtime-local via Treaty, generated contract) > health.live answers ok [0.91ms]
skipping: no server on 127.0.0.1:32873
(pass) notes API (runtime-local via Treaty, generated contract) > rejects a bad query with a typed 422 (generated contract round-trip) [0.28ms]

src/modules/notes/service.test.ts:
(pass) resolvePage > applies documented defaults [0.05ms]
(pass) resolvePage > computes offset for arbitrary valid pages [0.01ms]
(pass) resolvePage > honors explicit page 1 with max page size

 5 pass
 0 fail
 4 expect() calls
Ran 5 tests across 2 files. [6.00ms]

== [6] boot ephemeral Postgres (postgres:16-alpine, loopback-published, removed on exit)
postgres ready: 127.0.0.1:32773/velqu_e2e image=postgres@sha256:cf78e76683b9ca8c5733cbbdce6c9262b45b6767934dd0a95e671f9a0fc20685

== [7] fail-closed proof: the pack's postgres grant rejects startup without VELQU_DATABASE_URL
startup rejected (typed): {"event":"ready","ok":false,"error":"pack requires runtime:postgres but VELQU_DATABASE_URL is not configured"}

== [8] spawn velqu-runtime with the built pack + VELQU_DATABASE_URL
{"level":"info","event":"ready","appId":"notes-service","routes":5,"handlers":6,"addr":"127.0.0.1:42307","mode":"shared","engine":"quickjs-ng/0.15.1","runtimeAbi":1,"contextProfile":"full","serviceProfile":"serverless","startupWorkers":1,"config":{"host":"127.0.0.1","hostSource":"default","port":42307,"portSource":"cli","maxBodyBytes":1048576,"maxBodyBytesSource":"default","maxQueue":256,"maxQueueSource":"default","log":"errors","logSource":"cli","logSample":0,"logSampleSource":"default","metrics":"off","metricsSource":"default","activeProfile":null,"proxyMode":"reverse-proxy","proxyModeSource":"default"},"contractHash":"60f1567fcaa1604fe0324e8a9e2e4219","startupMs":6.243043,"stages":[{"stage":"pack.load","ms":1.413952},{"stage":"router.build","ms":0.03326},{"stage":"config.resolve","ms":0.04784},{"stage":"engine.spawn","ms":0.064563},{"stage":"bundle.load","ms":4.473433},{"stage":"listen","ms":0.030005}],"bundleEvalMs":1.107025}
runtime ready on :42307 (pid 1745874)

== [9] typed failure: create without token -> 401 problem+json
{"level":"warn","event":"request.complete","requestId":"req-1790873957043-2","routeId":"notes.create","method":"POST","path":"/notes","status":401,"bodyBytes":119,"stage":"engine.problem","durationMs":0.20041199999999998,"traceId":null}
POST http://127.0.0.1:42307/notes -> 401 {"type":"https://velqu.dev/problems/unauthorized","title":"Unauthorized","status":401,"instance":"req-1790873957043-2"}

== [10] typed failure: out-of-range pageSize -> 422 validation
{"level":"warn","event":"request.complete","requestId":"req-1790873957043-3","routeId":"notes.list","method":"GET","path":"/notes","status":422,"bodyBytes":236,"stage":"validation.query","durationMs":0.022849,"traceId":null}
GET http://127.0.0.1:42307/notes?pageSize=999 -> 422 {"type":"https://velqu.dev/problems/validation","title":"Validation failed","status":422,"instance":"req-1790873957043-3","detail":"query validation failed","errors":[{"path":"pageSize","code":"maximum","message":"must b

== [11] typed failure: malformed id -> 422 params validation
{"level":"warn","event":"request.complete","requestId":"req-1790873957043-4","routeId":"notes.get","method":"GET","path":"/notes/not-an-id","status":422,"bodyBytes":250,"stage":"validation.params","durationMs":0.201273,"traceId":null}
GET http://127.0.0.1:42307/notes/not-an-id -> 422 {"type":"https://velqu.dev/problems/validation","title":"Validation failed","status":422,"instance":"req-1790873957043-4","detail":"path parameter validation failed","errors":[{"path":"id","code":"pattern","message":"mus

== [12] happy path: create with SQL metacharacters in the title -> 201 (parameterized)
created id: nts_51696ef49bdc

== [13] happy path: list page 1 sees the row (pagination round trip)
GET http://127.0.0.1:42307/notes?page=1&pageSize=10 -> 200 {"rows":[{"id":"nts_51696ef49bdc","title":"O'Brien; DROP TABLE notes;--","createdAt":"2026-10-01 16:59:17.488118+00"}],"total":1,"page":1,"pageSize":10}

== [14] happy path: read-back shows the title VERBATIM (injection stored safely)
GET http://127.0.0.1:42307/notes/nts_51696ef49bdc -> 200 {"id":"nts_51696ef49bdc","title":"O'Brien; DROP TABLE notes;--","body":"injection probe","createdAt":"2026-10-01 16:59:17.488118+00"}

== [15] typed failure: unknown (pattern-valid) id -> 404 problem
{"level":"warn","event":"request.complete","requestId":"req-1790873957043-8","routeId":"notes.get","method":"GET","path":"/notes/nts_000000000000","status":404,"bodyBytes":139,"stage":"engine.problem","durationMs":0.951855,"traceId":null}
GET http://127.0.0.1:42307/notes/nts_000000000000 -> 404 {"type":"https://velqu.dev/problems/not-found","title":"Not Found","status":404,"instance":"req-1790873957043-8","detail":"note not found"}

== [16] typed failure: delete with wrong token -> 401
{"level":"warn","event":"request.complete","requestId":"req-1790873957043-9","routeId":"notes.delete","method":"DELETE","path":"/notes/nts_51696ef49bdc","status":401,"bodyBytes":119,"stage":"engine.problem","durationMs":0.15586699999999998,"traceId":null}
DELETE http://127.0.0.1:42307/notes/nts_51696ef49bdc -> 401 {"type":"https://velqu.dev/problems/unauthorized","title":"Unauthorized","status":401,"instance":"req-1790873957043-9"}

== [17] happy path: delete -> 200, then re-get -> 404
DELETE http://127.0.0.1:42307/notes/nts_51696ef49bdc -> 200 {"deleted":true}
{"level":"warn","event":"request.complete","requestId":"req-1790873957043-11","routeId":"notes.get","method":"GET","path":"/notes/nts_51696ef49bdc","status":404,"bodyBytes":140,"stage":"engine.problem","durationMs":0.9315950000000001,"traceId":null}
GET http://127.0.0.1:42307/notes/nts_51696ef49bdc -> 404 {"type":"https://velqu.dev/problems/not-found","title":"Not Found","status":404,"instance":"req-1790873957043-11","detail":"note not found"}

== [18] typed failure: live database loss -> declared 503 (fail closed, no leak)
{"level":"warn","event":"request.complete","requestId":"req-1790873957043-12","routeId":"notes.create","method":"POST","path":"/notes","status":503,"bodyBytes":32,"stage":"engine","durationMs":0.43518,"traceId":null}
POST http://127.0.0.1:42307/notes -> 503 {"error":"database unavailable"}

== [19] graceful stop
{"level":"info","event":"drain.begin","pending":0}
{"level":"info","event":"ops.worker.status","status":{"queue":{"pending":0,"slabLive":0,"invocationsPending":0},"worker":{"quarantined":false,"queuePoisoned":false,"poisonEvents":0},"memory":{"heapUsedBytes":258086},"tasks":{"nativeStarted":0,"nativeAlive":0,"nativeCompleted":14,"nativeAborted":0},"slots":{"live":0,"capacity":256},"drain":{"draining":true,"refused":0},"loadShed":{"all_workers_full":0,"class_ceiling":0,"draining":0,"global_admission_full":0,"long_running_slots":0,"tracking_full":0,"worker_queue_full":0},"pools":{"fetch":{"initialized":false,"shutdown":false,"active":0,"max_active":128,"rejections":0},"postgres":{"linked":true,"pool":{"idle":0,"inUse":0,"createdTotal":1,"maxConnections":10,"acquiresOk":13,"reused":12,"created":1,"discardedStale":0,"discardedDead":1,"discardedError":0,"atCapacity":0,"connectTimeouts":0,"connectRejected":1,"shutdownRefusals":0}}}}}
{"level":"info","event":"shutdown.complete","stats":{"invocations":9,"policy_calls":5,"handler_calls":9,"immediate_results":0,"promise_results":9,"promise_watches":9,"job_queue_drains":19,"settlement_scans":23,"timer_ops_started":0,"timer_ops_completed":0,"postgres_ops_started":14,"postgres_ops_completed":14,"pending_ops":0,"native_tasks_started":0,"native_tasks_alive":0,"native_tasks_completed":14,"native_tasks_aborted":0,"scheduler_boundary_violations":0,"queue_poisoned":false,"poison_events":0,"late_completions_dropped":0,"cancelled_invocations":0,"timeouts":0,"engine_failures":0,"contract_violations":0,"numeric_dispatches":9,"legacy_map_dispatches":0,"heap_used":259006,"defers_admitted":0,"defers_rejected":0,"defer_drains":0,"defers_drained":0,"defer_drains_interrupted":0,"defers_dropped_at_shutdown":0},"stageMetrics":{"route":12,"queue":9,"decode":3,"bridge":9,"js":9,"encode":9,"write":12,"slab_live":0,"queue_pending":0,"body_bytes":106},"invocations":{"pending":0,"registered":9,"settled":9},"drain":{"refused":0,"completed":12,"aborted":0},"loadShed":{"all_workers_full":0,"class_ceiling":0,"draining":0,"global_admission_full":0,"long_running_slots":0,"tracking_full":0,"worker_queue_full":0},"fetchPool":{"initialized":false,"drained":true}}

NOTES-OK
DRIVER_EXIT=0
