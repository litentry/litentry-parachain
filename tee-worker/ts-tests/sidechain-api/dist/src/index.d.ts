import { ApiOptions } from "@polkadot/api/types";
import { ApiPromise } from "@polkadot/api";
import rawMetadata from "sidechain-api/litentry-sidechain-metadata.json";
export { Metadata, TypeRegistry } from "@polkadot/types";
export type { Index } from "@polkadot/types/interfaces";
export type { ApiTypes, SubmittableExtrinsic } from "@polkadot/api/types";
export type {
    LitentryPrimitivesIdentity,
    PalletIdentityManagementTeeIdentityContext,
} from "@polkadot/types/lookup";
export { rawMetadata };
type ProviderInterface = Exclude<ApiOptions["provider"], undefined>;
export declare function create(provider: ProviderInterface): Promise<ApiPromise>;
//# sourceMappingURL=index.d.ts.map
