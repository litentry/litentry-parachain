import "@polkadot/api/augment";
import "@polkadot/types-augment";
import { ApiOptions, ApiTypes, AugmentedEvent } from "@polkadot/api/types";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";

import { identity } from "../build/interfaces/definitions";
import { LitentryIdentity } from "../build/interfaces";
import type { AnyTuple } from "@polkadot/types/types";

export type { CorePrimitivesErrorErrorDetail } from "@polkadot/types/lookup";

export type { FrameSystemEventRecord } from "@polkadot/types/lookup";
export type { ApiTypes, SubmittableExtrinsic } from "@polkadot/api/types";
export type {
    Assertion,
    LitentryIdentity,
    LitentryValidationData,
    Web3Network,
    WorkerRpcReturnValue,
    TrustedCallSigned,
    Getter,
    TrustedOperationResponse,
    StfError,
    LinkIdentityResult,
} from "../build/interfaces";
export type { Codec } from "@polkadot/types/types";
export type { Bytes } from "@polkadot/types-codec";
export { ApiPromise, Keyring, WsProvider }; // @fixme don't export WsProvider :P

export const definitions = identity; // @fixme don't export?

type ProviderInterface = Exclude<ApiOptions["provider"], undefined>;
export async function create(provider: ProviderInterface): Promise<ApiPromise> {
    const api = await ApiPromise.create({ provider, types: identity.types });
    const foo: LitentryIdentity = api.createType("LitentryIdentity"); // @fixme: temporary probe for typing sanity
    api.events.identityManagement.LinkIdentityFailed.is;
    return api;
}

type GuardType<GuardFunction> = GuardFunction extends (x: any) => x is infer Type ? Type : never;
type IEventLike = Parameters<AugmentedEvent<never>["is"]>[0];

export function filterEvents<ApiType extends ApiTypes, T extends AnyTuple, N>(
    eventType: AugmentedEvent<ApiType, T, N>,
    events: IEventLike[]
): GuardType<AugmentedEvent<ApiType, T, N>["is"]>[] {
    return events.filter(eventType.is.bind(eventType));
}
