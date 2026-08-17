import type { ProofApi } from "./api-types";
import { treaty } from "@q/treaty";
const api = treaty<ProofApi>({ baseUrl: "http://x", contract: {} });
api.res7.get({ id: "not-a-number" });
