import "@polkadot/api/augment";
import "@polkadot/types/augment";
import { ApiOptions } from "@polkadot/api/types";
import { ApiPromise, WsProvider } from "@polkadot/api";

import { identity } from "parachain-api/interfaces/definitions";
import { LitentryIdentity } from "parachain-api/interfaces";

export type { ApiTypes, SubmittableExtrinsic } from "@polkadot/api/types";
export type { LitentryValidationData, WorkerRpcReturnValue } from "parachain-api/interfaces";
export type { Codec } from "@polkadot/types/types";
export { ApiPromise, WsProvider }; // @fixme don't export WsProvider :P

export const definitions = identity; // @fixme don't export?

type ProviderInterface = Exclude<ApiOptions["provider"], undefined>;
export async function create(provider: ProviderInterface): Promise<ApiPromise> {
    const api = await ApiPromise.create({ provider, types: identity.types });
    const foo: LitentryIdentity = api.createType("LitentryIdentity");
    return api;
}
