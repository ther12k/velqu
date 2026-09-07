# BWASM-Q-004 Inspector Panel Representation

The optional `InspectorPanelAdapter` provides formatted outputs for developer preview and debugging UI without requiring native developer tools.

## Text Report Format

```text
=== Velqu Diagnostic Trace: cr_m1a2b3_1_xyz890 (7 events) ===
2026-09-07T12:00:00.000Z INFO  [load]        DIAG_LOAD_MANIFEST_OK (cr_m1a2b3_1_xyz890): loaded manifest for app proof
2026-09-07T12:00:00.010Z INFO  [verify]      DIAG_VERIFY_INTEGRITY_OK (cr_m1a2b3_1_xyz890): all 5 artifact digests verified against manifest
2026-09-07T12:00:00.025Z INFO  [instantiate] DIAG_LIFECYCLE_READY (cr_m1a2b3_1_xyz890): kernel ready (ABI 1)
2026-09-07T12:00:00.050Z DEBUG [route]       DIAG_ROUTE_MATCHED (cr_m1a2b3_1_xyz890) route=1: GET /hello/world
2026-09-07T12:00:00.055Z DEBUG [validate]    DIAG_VALIDATE_PASSED (cr_m1a2b3_1_xyz890) route=1: params schema matched
2026-09-07T12:00:00.060Z DEBUG [invoke]      DIAG_INVOKE_START (cr_m1a2b3_1_xyz890) route=1: invoking handler hello.get
2026-09-07T12:00:00.075Z DEBUG [invoke]      DIAG_INVOKE_SUCCESS (cr_m1a2b3_1_xyz890) route=1: status 200
--- Summary: Failures: 0 | Stages: {"load":1,"verify":1,"instantiate":1,"route":1,"validate":1,"invoke":2} ---
```

## HTML UI Render Output (XSS-safe)

```html
<div class="velqu-diagnostics-panel">
  <div class="velqu-diag-header">
    <h3>Velqu Browser-WASM Diagnostics</h3>
    <span class="velqu-diag-count">7 events (0 errors)</span>
  </div>
  <table class="velqu-diag-table">
    <thead>
      <tr><th>Time</th><th>Level</th><th>Stage</th><th>Code</th><th>Detail</th></tr>
    </thead>
    <tbody>
      <tr><td>12:00:00.000</td><td class="velqu-lvl-info">INFO</td><td>load</td><td><code>DIAG_LOAD_MANIFEST_OK</code></td><td><span class="velqu-corr">cr_m1a2b3_1_xyz890</span> loaded manifest for app proof</td></tr>
      <tr><td>12:00:00.010</td><td class="velqu-lvl-info">INFO</td><td>verify</td><td><code>DIAG_VERIFY_INTEGRITY_OK</code></td><td><span class="velqu-corr">cr_m1a2b3_1_xyz890</span> all 5 artifact digests verified against manifest</td></tr>
      <tr><td>12:00:00.025</td><td class="velqu-lvl-info">INFO</td><td>instantiate</td><td><code>DIAG_LIFECYCLE_READY</code></td><td><span class="velqu-corr">cr_m1a2b3_1_xyz890</span> kernel ready (ABI 1)</td></tr>
      <tr><td>12:00:00.050</td><td class="velqu-lvl-debug">DEBUG</td><td>route</td><td><code>DIAG_ROUTE_MATCHED</code></td><td><span class="velqu-corr">cr_m1a2b3_1_xyz890</span> GET /hello/world</td></tr>
      <tr><td>12:00:00.055</td><td class="velqu-lvl-debug">DEBUG</td><td>validate</td><td><code>DIAG_VALIDATE_PASSED</code></td><td><span class="velqu-corr">cr_m1a2b3_1_xyz890</span> params schema matched</td></tr>
      <tr><td>12:00:00.060</td><td class="velqu-lvl-debug">DEBUG</td><td>invoke</td><td><code>DIAG_INVOKE_START</code></td><td><span class="velqu-corr">cr_m1a2b3_1_xyz890</span> invoking handler hello.get</td></tr>
      <tr><td>12:00:00.075</td><td class="velqu-lvl-debug">DEBUG</td><td>invoke</td><td><code>DIAG_INVOKE_SUCCESS</code></td><td><span class="velqu-corr">cr_m1a2b3_1_xyz890</span> status 200</td></tr>
    </tbody>
  </table>
</div>
```

All textual fields are HTML-escaped before insertion to prevent cross-site scripting vulnerabilities from user-provided error messages or URL inputs.
