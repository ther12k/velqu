# relay-service external consumer E2E transcript (#1400)

Captured 2026-10-01T16:12:50Z on the dev host — pinned Bun 1.4.0,
registry.npmjs.org network, locally built release runtime (pins-only
fingerprint: runtime build hash covers abi/engine/rquickjs constants, so a
pack built by the published compiler loads on it; SEC-001 holds).
Command: BUN=bun scripts/beta-external/relay-service.sh

== relay-service external consumer transcript ==
bun=1.4.0 runtime=/home/ther12k/Workspace/Learning/velqu-worktrees/relay-consumer-a/target/release/velqu-runtime app=/tmp/velqu-relay-Uaax14/relay-service

== [1] copy template to /tmp/velqu-relay-Uaax14/relay-service (clean consumer tree)

== [2] bun install (public registry — no workspace links)
bun install v1.4.0 (34cbb9a40)
Saved lockfile

+ @types/bun@1.4.2
+ @velqu/cli@0.1.0-beta.1
+ typescript@5.9.3 (v7.0.2 available)
+ @velqu/core@0.1.0-beta.1
+ @velqu/schema@0.1.0-beta.1
+ @velqu/treaty@0.1.0-beta.1

12 packages installed [10.00ms]

== [3] velqu check (via the scaffold-style velqu bin script)
$ velqu check --project .
velqu check: 4 routes in . — clean

== [4] bun test (unit green; runtime-local client tests skip without a server)
bun test v1.4.0 (34cbb9a40)

src/client.test.ts:
skipping: no server on 127.0.0.1:35143
(pass) relay API (runtime-local via Treaty) > health.live answers ok [1.91ms]
skipping: no server on 127.0.0.1:35143
(pass) relay API (runtime-local via Treaty) > rejects an invalid webhook token with a typed 401 [0.84ms]

src/modules/hooks/service.test.ts:
(pass) delivery store > stores and reads back a delivery [0.22ms]
(pass) delivery store > records enrichment on an existing delivery [0.07ms]
(pass) delivery store > evicts oldest deliveries beyond MAX_DELIVERIES (bounded store) [0.84ms]

 5 pass
 0 fail
 5 expect() calls
Ran 5 tests across 2 files. [14.00ms]

== [5] velqu build -> dist/app.qpack
$ velqu build --project .
velqu build [serverless]: 4 routes → dist in 1186ms
  app.qpack  17951B
  route-manifest.json  2152B
  schema-manifest.json  2704B
  capability-manifest.json  620B
  contract.json  3891B
  contract.d.ts  1404B
  contract.meta.json  757B
  openapi.json  6809B
  contract.lock.json  3863B
  build-report.json  8781B
  app.qpack.sources.json  46650B
  published-manifest.json  969B

== [6] start loopback upstream stub on :18971

== [7] spawn velqu-runtime with the built pack
{"level":"info","event":"ready","appId":"relay-service","routes":4,"handlers":5,"addr":"127.0.0.1:35427","mode":"shared","engine":"quickjs-ng/0.15.1","runtimeAbi":1,"contextProfile":"full","serviceProfile":"serverless","startupWorkers":1,"config":{"host":"127.0.0.1","hostSource":"default","port":35427,"portSource":"cli","maxBodyBytes":1048576,"maxBodyBytesSource":"default","maxQueue":256,"maxQueueSource":"default","log":"errors","logSource":"cli","logSample":0,"logSampleSource":"default","metrics":"off","metricsSource":"default","activeProfile":null,"proxyMode":"reverse-proxy","proxyModeSource":"default"},"contractHash":"0cbc57fc44ead696931c678f0213f51d","startupMs":4.648677999999999,"stages":[{"stage":"pack.load","ms":1.190738},{"stage":"router.build","ms":0.015951999999999997},{"stage":"config.resolve","ms":0.045271},{"stage":"engine.spawn","ms":0.08287900000000001},{"stage":"bundle.load","ms":3.058641},{"stage":"listen","ms":0.060927}],"bundleEvalMs":0.487208}
runtime ready on :35427 (pid 1682035)

== [8] typed failure: intake without token -> 401 problem+json
{"level":"warn","event":"request.complete","requestId":"req-1790871173330-2","routeId":"hooks.receive","method":"POST","path":"/hooks","status":401,"bodyBytes":119,"stage":"engine.problem","durationMs":0.487129,"traceId":null}
POST http://127.0.0.1:35427/hooks -> 401 {"type":"https://velqu.dev/problems/unauthorized","title":"Unauthorized","status":401,"instance":"req-1790871173330-2"}

== [9] typed failure: invalid deliveryId -> 422 validation
{"level":"warn","event":"request.complete","requestId":"req-1790871173330-3","routeId":"hooks.receive","method":"POST","path":"/hooks","status":422,"bodyBytes":251,"stage":"validation.body","durationMs":0.037332000000000004,"traceId":null}
POST http://127.0.0.1:35427/hooks -> 422 {"type":"https://velqu.dev/problems/validation","title":"Validation failed","status":422,"instance":"req-1790871173330-3","detail":"body validation failed","errors":[{"path":"deliveryId","code":"patte

== [10] happy path: authenticated intake -> 201
POST http://127.0.0.1:35427/hooks -> 201 {"deliveryId":"dlv-10000001","receivedAtMs":1790871173354}

== [11] happy path: enrichment via loopback fetch -> 200 stub-enrichment
POST http://127.0.0.1:35427/hooks/dlv-10000001/enrich -> 200 {"deliveryId":"dlv-10000001","eventType":"push","enrichment":"stub-enrichment"}

== [12] read-back shows enrichment (was pending)
GET http://127.0.0.1:35427/hooks/dlv-10000001 -> 200 {"deliveryId":"dlv-10000001","source":"github","eventType":"push","enrichment":"stub-enrichment"}

== [13] typed failure: unknown delivery -> 404 problem
{"level":"warn","event":"request.complete","requestId":"req-1790871173330-7","routeId":"hooks.get","method":"GET","path":"/hooks/missing-00001","status":404,"bodyBytes":143,"stage":"engine.problem","durationMs":0.129825,"traceId":null}
GET http://127.0.0.1:35427/hooks/missing-00001 -> 404 {"type":"https://velqu.dev/problems/not-found","title":"Not Found","status":404,"instance":"req-1790871173330-7","detail":"delivery not found"}

== [14] typed failure: upstream loss -> 502 typed error
{"level":"warn","event":"request.complete","requestId":"req-1790871173330-9","routeId":"hooks.enrich","method":"POST","path":"/hooks/dlv-20000002/enrich","status":502,"bodyBytes":32,"stage":"engine","durationMs":0.580777,"traceId":null}
POST http://127.0.0.1:35427/hooks/dlv-20000002/enrich -> 502 {"error":"upstream unavailable"}

== [15] graceful stop
{"level":"info","event":"drain.begin","pending":0}
{"level":"info","event":"ops.worker.status","status":{"queue":{"pending":0,"slabLive":0,"invocationsPending":0},"worker":{"quarantined":false,"queuePoisoned":false,"poisonEvents":0},"memory":{"heapUsedBytes":250433},"tasks":{"nativeStarted":2,"nativeAlive":0,"nativeCompleted":2,"nativeAborted":0},"slots":{"live":0,"capacity":256},"drain":{"draining":true,"refused":0},"loadShed":{"all_workers_full":0,"class_ceiling":0,"draining":0,"global_admission_full":0,"long_running_slots":0,"tracking_full":0,"worker_queue_full":0},"pools":{"fetch":{"initialized":true,"shutdown":false,"active":0,"max_active":128,"rejections":0},"postgres":{"linked":false}}}}
{"level":"info","event":"shutdown.complete","stats":{"invocations":7,"policy_calls":3,"handler_calls":7,"immediate_results":0,"promise_results":7,"promise_watches":7,"job_queue_drains":7,"settlement_scans":9,"timer_ops_started":0,"timer_ops_completed":0,"postgres_ops_started":0,"postgres_ops_completed":0,"pending_ops":0,"native_tasks_started":2,"native_tasks_alive":0,"native_tasks_completed":2,"native_tasks_aborted":0,"scheduler_boundary_violations":0,"queue_poisoned":false,"poison_events":0,"late_completions_dropped":0,"cancelled_invocations":0,"timeouts":0,"engine_failures":0,"contract_violations":0,"numeric_dispatches":7,"legacy_map_dispatches":0,"heap_used":254151,"defers_admitted":0,"defers_rejected":0,"defer_drains":0,"defers_drained":0,"defer_drains_interrupted":0,"defers_dropped_at_shutdown":0},"stageMetrics":{"route":9,"queue":7,"decode":4,"bridge":7,"js":7,"encode":7,"write":9,"slab_live":0,"queue_pending":0,"body_bytes":350},"invocations":{"pending":0,"registered":7,"settled":7},"drain":{"refused":0,"completed":9,"aborted":0},"loadShed":{"all_workers_full":0,"class_ceiling":0,"draining":0,"global_admission_full":0,"long_running_slots":0,"tracking_full":0,"worker_queue_full":0},"fetchPool":{"initialized":true,"drained":true}}

RELAY-OK
DRIVER_EXIT=0
