export * from "@polkadot/api/types";
export * from "@polkadot/api";
export * from "@polkadot/types/lookup";
export * from "../build/interfaces";

import {
    PalletIdentityManagementTeeCall,
    LitentryPrimitivesIdentity,
    PalletIdentityManagementTeeIdentityContext,
    PalletIdentityManagementTeeIdentityContextIdentityStatus,
    CorePrimitivesNetworkWeb3Network,
    SpRuntimeMultiSignature,
    PalletIdentityManagementTeeError,
} from "@polkadot/types/lookup";

// @fixme why doesn't work with `export * from '@polkadot/types/lookup' with the following types? But works with `export * from '@polkadot/types/lookup' with FrameSystem* types?
export type {
    PalletIdentityManagementTeeCall,
    LitentryPrimitivesIdentity,
    PalletIdentityManagementTeeIdentityContext,
    PalletIdentityManagementTeeIdentityContextIdentityStatus,
    CorePrimitivesNetworkWeb3Network,
    SpRuntimeMultiSignature,
    PalletIdentityManagementTeeError,
};
