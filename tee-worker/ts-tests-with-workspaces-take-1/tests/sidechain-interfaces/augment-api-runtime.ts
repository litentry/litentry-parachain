// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/calls';

import type { ApiTypes, AugmentedCall, DecoratedCallBase } from '@polkadot/api-base/types';
import type { Null } from '@polkadot/types-codec';
import type { OpaqueMetadata } from '@polkadot/types/interfaces/metadata';
import type { Block, Header } from '@polkadot/types/interfaces/runtime';
import type { RuntimeVersion } from '@polkadot/types/interfaces/state';
import type { Observable } from '@polkadot/types/types';

export type __AugmentedCall<ApiType extends ApiTypes> = AugmentedCall<ApiType>;
export type __DecoratedCallBase<ApiType extends ApiTypes> = DecoratedCallBase<ApiType>;

declare module '@polkadot/api-base/types/calls' {
    interface AugmentedCalls<ApiType extends ApiTypes> {
        /** 0xdf6acb689907609b/4 */
        core: {
            /**
             * Execute the given block.
             **/
            executeBlock: AugmentedCall<
                ApiType,
                (block: Block | { header?: any; extrinsics?: any } | string | Uint8Array) => Observable<Null>
            >;
            /**
             * Initialize a block with the given header.
             **/
            initializeBlock: AugmentedCall<
                ApiType,
                (
                    header:
                        | Header
                        | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any }
                        | string
                        | Uint8Array
                ) => Observable<Null>
            >;
            /**
             * Returns the version of the runtime.
             **/
            version: AugmentedCall<ApiType, () => Observable<RuntimeVersion>>;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0x37e397fc7c91f5e4/1 */
        metadata: {
            /**
             * Returns the metadata of a runtime
             **/
            metadata: AugmentedCall<ApiType, () => Observable<OpaqueMetadata>>;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
    } // AugmentedCalls
} // declare module
