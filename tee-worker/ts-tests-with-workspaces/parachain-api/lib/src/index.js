import "@polkadot/api/augment";
import "@polkadot/types/augment";
import { ApiPromise } from "@polkadot/api";
import { identity } from "parachain-api/interfaces/definitions";
export async function create(provider) {
    return await ApiPromise.create({ provider, types: identity.types });
}
