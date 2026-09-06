// BWASM-R-001 bundler smoke: a CLEAN consumer imports the package and
// constructs the runtime boundary exactly as an app would.
import {
  createBrowserRuntime,
  type KernelModule,
  type KernelPlanResult,
} from "@velqu/browser-runtime";

const okPlan: KernelPlanResult = {
  kind: "invoke",
  abiVersion: 1,
  routeId: 0,
  handlerKey: "health.live",
  allowedStatuses: [200],
  defaultStatus: 200,
  deadlineMs: 5000,
};

const kernel: KernelModule = Object.assign(
  class {
    plan_request(): string {
      return JSON.stringify(okPlan);
    }
    complete_invocation(completionJson: string): string {
      const c = JSON.parse(completionJson) as { result: { status: number } };
      return JSON.stringify({
        kind: "response",
        status: c.result.status,
        headers: [["content-type", "application/json"]],
        body: { status: "ok" },
      });
    }
    authorize_capability(): string {
      return JSON.stringify({ authorized: true });
    }
    dispose(): void {}
  },
  { kernel_abi_version: () => 1 },
);

export async function main(): Promise<{ state: string; status: number; body: unknown }> {
  const runtime = createBrowserRuntime({ packBytes: new Uint8Array([1]), kernel });
  const response = await runtime.fetch(new Request("https://app.example/health/live"));
  const result = {
    state: runtime.state,
    status: response.status,
    body: await response.json(),
  };
  runtime.dispose();
  return result;
}
