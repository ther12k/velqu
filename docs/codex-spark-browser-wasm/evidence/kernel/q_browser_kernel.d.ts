/* tslint:disable */
/* eslint-disable */

/**
 * JS-facing kernel handle. Construction failure carries the
 * artifact problem as the error message (stable JSON).
 */
export class WasmKernel {
    free(): void;
    [Symbol.dispose](): void;
    authorize_capability(name: string): string;
    complete_invocation(completion_json: string): string;
    /**
     * Explicit disposal for JS visibility; equivalent to Drop.
     */
    dispose(): void;
    constructor(pack_bytes: Uint8Array);
    plan_request(request_json: string): string;
}

export function kernel_abi_version(): number;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_wasmkernel_free: (a: number, b: number) => void;
    readonly kernel_abi_version: () => number;
    readonly wasmkernel_authorize_capability: (a: number, b: number, c: number, d: number) => void;
    readonly wasmkernel_complete_invocation: (a: number, b: number, c: number, d: number) => void;
    readonly wasmkernel_dispose: (a: number) => void;
    readonly wasmkernel_new: (a: number, b: number, c: number) => void;
    readonly wasmkernel_plan_request: (a: number, b: number, c: number, d: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
