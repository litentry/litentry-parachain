import "@polkadot/api/augment";
import "@polkadot/types/augment";
import { ApiOptions } from "@polkadot/api/types";
import { ApiPromise } from "@polkadot/api";

import { identity } from "parachain-api/interfaces/definitions";

type ProviderInterface = Exclude<ApiOptions["provider"], undefined>;

export type ParachainApiPromise = ApiPromise;

export async function create(provider: ProviderInterface): Promise<ParachainApiPromise> {
    return await ApiPromise.create({ provider, types: identity.types });
}
