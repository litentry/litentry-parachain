import { ApiPromise } from "@polkadot/api";
export { Metadata, TypeRegistry } from "@polkadot/types";
export { ApiPromise } from "@polkadot/api";
export async function create(provider) {
	return await ApiPromise.create({ provider });
}
