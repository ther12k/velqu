/**
 * @velqu/contract — published compact contract types (language-neutral shape).
 *
 * The compiler emits `contract.json` (this shape) and a generated
 * `contract.d.ts` that binds `Api` to it. Client repositories import ONLY this
 * — never server source (TRT-004, PR-006).
 */

/** Status -> response body type map. */
export type ResponseMap = Record<number, unknown>;

export type HttpMethodUpper = "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "HEAD";

/** A single route's compact contract. */
export interface RouteContract<
  Path extends string = string,
  Method extends string = string,
  Params = Record<string, never>,
  Query = Record<string, never>,
  Body = undefined,
  Resp extends ResponseMap = ResponseMap,
  Security extends string | undefined = undefined,
> {
  readonly path: Path;
  readonly method: Method;
  readonly params: Params;
  readonly query: Query;
  readonly body: Body;
  readonly responses: Resp;
  readonly security: Security;
}

export interface ContractMeta {
  readonly appId: string;
  readonly contractVersion: number;
  readonly contractHash: string;
  readonly generatedBy: string;
}

/**
 * Published API type: a map of route-id keys to route contracts.
 * Treaty navigates this type; the runtime shape is data.
 */
export interface PublishedContract<
  Routes extends Record<string, RouteContract> = Record<string, RouteContract>,
> extends ContractMeta {
  readonly routes: Routes;
}

/** Hand-authorable helper for tests/examples before the compiler exists. */
export function definePublishedContract<const R extends Record<string, RouteContract>>(
  contract: PublishedContract<R>,
): PublishedContract<R> {
  return contract;
}
