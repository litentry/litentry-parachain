import "@polkadot/api/augment";
import "@polkadot/types-augment";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { identity } from "../build/interfaces/definitions";
export { ApiPromise, Keyring, WsProvider }; // @fixme don't export WsProvider :P
export const definitions = identity; // @fixme don't export?
export async function create(provider) {
	const api = await ApiPromise.create({ provider, types: identity.types });
	const foo = api.createType("LitentryIdentity"); // @fixme: temporary probe for typing sanity
	api.events.identityManagement.LinkIdentityFailed.is;
	return api;
}
export function filterEvents(eventType, events) {
	return events.filter(eventType.is.bind(eventType));
}
