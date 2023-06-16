import "@polkadot/api/augment";
import "@polkadot/types/augment";
import { ApiPromise } from "@polkadot/api";
export async function create(provider) {
    return await ApiPromise.create({ provider });
}
