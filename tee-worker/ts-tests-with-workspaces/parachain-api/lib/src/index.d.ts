import "@polkadot/api/augment";
import "@polkadot/types/augment";
import { ApiOptions } from "@polkadot/api/types";
import { ApiPromise } from "@polkadot/api";
type ProviderInterface = Exclude<ApiOptions["provider"], undefined>;
export type ParachainApiPromise = ApiPromise;
export declare function create(provider: ProviderInterface): Promise<ParachainApiPromise>;
export {};
