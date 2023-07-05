import "@polkadot/api/augment";
import "@polkadot/types/augment";
import { ApiPromise } from "@polkadot/api";
import rawMetadata from "sidechain-api/litentry-sidechain-metadata.json";
export { Metadata, TypeRegistry } from "@polkadot/types";
export { rawMetadata };
export async function create(provider) {
    return await ApiPromise.create({ provider });
}
