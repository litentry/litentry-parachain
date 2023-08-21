import "@polkadot/api/augment";
import "@polkadot/types-augment";
import { ApiOptions, ApiTypes, AugmentedEvent } from "@polkadot/api/types";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import type { AnyTuple } from "@polkadot/types/types";
export type { CorePrimitivesErrorErrorDetail } from "@polkadot/types/lookup";
export type { FrameSystemEventRecord } from "@polkadot/types/lookup";
export type { ApiTypes, SubmittableExtrinsic } from "@polkadot/api/types";
export type { Assertion, LitentryIdentity, LitentryValidationData, Web3Network, WorkerRpcReturnValue, TrustedCallSigned, Getter, PublicGetter, } from "../build/interfaces";
export type { Codec } from "@polkadot/types/types";
export type { Bytes } from "@polkadot/types-codec";
export { ApiPromise, Keyring, WsProvider };
export declare const definitions: {
    types: {
        WorkerRpcReturnValue: {
            value: string;
            do_watch: string;
            status: string;
        };
        TrustedOperation: {
            _enum: {
                indirect_call: string;
                direct_call: string;
                get: string;
            };
        };
        TrustedCallSigned: {
            call: string;
            index: string;
            signature: string;
        };
        Getter: {
            _enum: {
                public: string;
                trusted: string;
            };
        };
        PublicGetter: {
            _enum: {
                some_value: string;
                nonce: string;
            };
        };
        TrustedGetterSigned: {
            getter: string;
            signature: string;
        };
        TrustedGetter: {
            _enum: {
                free_balance: string;
                reserved_balance: string;
                user_shielding_key: string;
                id_graph: string;
                id_graph_stats: string;
            };
        };
        TrustedCall: {
            _enum: {
                balance_set_balance: string;
                balance_transfer: string;
                balance_unshield: string;
                balance_shield: string;
                set_user_shielding_key: string;
                link_identity: string;
                deactivate_identity: string;
                activate_identity: string;
                request_vc: string;
                set_identity_networks: string;
                set_user_shielding_key_with_networks: string;
            };
        };
        UserShieldingKeyType: string;
        UserShieldingKeyNonceType: string;
        DirectRequestStatus: {
            _enum: {
                Ok: null;
                TrustedOperationStatus: string;
                Error: null;
            };
        };
        TrustedOperationStatus: {
            _enum: {
                Submitted: null;
                Future: null;
                Ready: null;
                Broadcast: null;
                InSidechainBlock: string;
                Retracted: null;
                FinalityTimeout: null;
                Finalized: null;
                Usurped: null;
                Dropped: null;
                Invalid: null;
            };
        };
        LitentryIdentity: {
            _enum: {
                Twitter: string;
                Discord: string;
                Github: string;
                Substrate: string;
                Evm: string;
            };
        };
        Address32: string;
        Address20: string;
        IdentityString: string;
        Web3Network: {
            _enum: string[];
        };
        LitentryValidationData: {
            _enum: {
                Web2Validation: string;
                Web3Validation: string;
            };
        };
        Web2ValidationData: {
            _enum: {
                Twitter: string;
                Discord: string;
            };
        };
        TwitterValidationData: {
            tweet_id: string;
        };
        DiscordValidationData: {
            channel_id: string;
            message_id: string;
            guild_id: string;
        };
        Web3ValidationData: {
            _enum: {
                Substrate: string;
                Evm: string;
            };
        };
        Web3CommonValidationData: {
            message: string;
            signature: string;
        };
        LitentryMultiSignature: {
            _enum: {
                Ed25519: string;
                Sr25519: string;
                Ecdsa: string;
                Ethereum: string;
                EthereumPrettified: string;
            };
        };
        EthereumSignature: string;
        IdentityGenericEvent: {
            who: string;
            identity: string;
            id_graph: string;
        };
        IdentityStatus: {
            _enum: string[];
        };
        IdentityContext: {
            link_block: string;
            web3networks: string;
            status: string;
        };
        BoundedWeb3Network: string;
        ShardIdentifier: string;
        Request: {
            shard: string;
            cyphertext: string;
        };
        VCRequested: {
            account: string;
            mrEnclave: string;
            assertion: string;
        };
        Assertion: {
            _enum: {
                A1: string;
                A2: string;
                A3: string;
                A4: string;
                A5: string;
                A6: string;
                A7: string;
                A8: string;
                A9: string;
                A10: string;
                A11: string;
                A13: string;
            };
        };
        GenericEventWithAccount: {
            account: string;
        };
        SetUserShieldingKeyResponse: {
            account: string;
            id_graph: string;
            req_ext_hash: string;
        };
        LinkIdentityResponse: {
            account: string;
            identity: string;
            id_graph: string;
            req_ext_hash: string;
        };
        DeactivateIdentityResponse: {
            account: string;
            identity: string;
            req_ext_hash: string;
        };
        ActivateIdentityResponse: {
            account: string;
            identity: string;
            req_ext_hash: string;
        };
        SetIdentityNetworksResponse: {
            req_ext_hash: string;
        };
        RequestVCResponse: {
            account: string;
            assertion: string;
            vc_index: string;
            vc_hash: string;
            vc_payload: string;
            req_ext_hash: string;
        };
        AesOutput: {
            ciphertext: string;
            aad: string;
            nonce: string;
        };
    };
};
type ProviderInterface = Exclude<ApiOptions["provider"], undefined>;
export declare function create(provider: ProviderInterface): Promise<ApiPromise>;
type GuardType<GuardFunction> = GuardFunction extends (x: any) => x is infer Type ? Type : never;
type IEventLike = Parameters<AugmentedEvent<never>["is"]>[0];
export declare function filterEvents<ApiType extends ApiTypes, T extends AnyTuple, N>(eventType: AugmentedEvent<ApiType, T, N>, events: IEventLike[]): GuardType<AugmentedEvent<ApiType, T, N>["is"]>[];
//# sourceMappingURL=index.d.ts.map