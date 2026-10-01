/**
 * Treaty client built from the GENERATED contract artifacts — the
 * compact-contract consumption path (M4A-005/TRT-006): the client
 * imports only `dist/contract.json` (runtime table) and
 * `dist/contract.d.ts` (types), never server source. Run `velqu build`
 * before importing this module.
 */
import { treaty, type TreatyClient } from "@velqu/treaty";
import type { Api } from "../dist/contract";
import contractJson from "../dist/contract.json";

type RouteTable = Record<string, { path: string; method: string }>;

export function createClient(baseUrl = "http://127.0.0.1:3000"): TreatyClient<Api> {
  return treaty<Api>({ baseUrl, contract: contractJson.routes as RouteTable });
}

export type { Api };
