import "@polkadot/api/augment";
import "@polkadot/types/augment";
import { ApiOptions } from "@polkadot/api/types";

import { identity } from "./interfaces/definitions";

import { ApiPromise } from "@polkadot/api";
import { LitentryIdentity } from "./interfaces";

type ProviderInterface = Exclude<ApiOptions["provider"], undefined>;

export async function create(provider: ProviderInterface): Promise<ApiPromise> {
    return await ApiPromise.create({ provider, types: identity.types });
}
