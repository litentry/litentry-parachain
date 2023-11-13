// augment on-chain lookup types
import "../build/interfaces/types-lookup.js";

// augment types for createType(...)
import "../build/interfaces/augment-types.js";
import "../build/interfaces/registry.js";

// augment API interfaces
import "../build/interfaces/augment-api.js";

export * from "@polkadot/types/lookup";
export * from "../build/interfaces";
export * from "@polkadot/api";
export * from "@polkadot/api/types";
import { default as identity } from "../build/interfaces/identity/definitions";
import { default as vc } from "../build/interfaces/vc/definitions";
import { default as trustedCalls } from "../build/interfaces/trustedCalls/definitions";
import { default as sidechain } from "../build/interfaces/sidechain/definitions";
export { identity, vc, trustedCalls, sidechain };
