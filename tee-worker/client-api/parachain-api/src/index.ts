import "@polkadot/api/augment";
import "@polkadot/types-augment";
export * from "@polkadot/api/types";
export * from "@polkadot/api";
export * from "@polkadot/types/lookup";
export * from "../build/interfaces";

import {
    CorePrimitivesKeyAesOutput,
    CorePrimitivesErrorErrorDetail,
    CorePrimitivesAssertion,
    CorePrimitivesAssertionAchainableParams,
    CorePrimitivesNetworkWeb3Network,
    PalletVcManagementVcContext,
    PalletVcManagementVcContextStatus,
    PalletVcManagementError,
    PalletIdentityManagementEvent,
} from "@polkadot/types/lookup";

import { identity } from "../build/interfaces/definitions";

export const definitions = identity;

// @fixme why doesn't work with `export * from '@polkadot/types/lookup' with the following types? But works with `export * from '@polkadot/types/lookup' with FrameSystem* types?
export type {
    CorePrimitivesKeyAesOutput,
    CorePrimitivesErrorErrorDetail,
    CorePrimitivesAssertion,
    CorePrimitivesAssertionAchainableParams,
    CorePrimitivesNetworkWeb3Network,
    PalletVcManagementVcContext,
    PalletVcManagementVcContextStatus,
    PalletVcManagementError,
    PalletIdentityManagementEvent,
};
