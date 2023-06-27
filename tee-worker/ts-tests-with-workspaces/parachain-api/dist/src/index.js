import "@polkadot/api/augment";
import "@polkadot/types/augment";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { identity } from "parachain-api/interfaces/definitions";
export { ApiPromise, WsProvider }; // @fixme don't export WsProvider :P
export const definitions = identity; // @fixme don't export?
export async function create(provider) {
    const api = await ApiPromise.create({ provider, types: identity.types });
    const foo = api.createType("LitentryIdentity");
    return api;
}
