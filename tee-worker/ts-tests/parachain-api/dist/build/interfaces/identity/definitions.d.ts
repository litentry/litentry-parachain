declare const _default: {
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
                A6: string;
                A7: string;
                A8: string;
                A9: string;
                A10: string;
                A11: string;
                A12: string;
                A13: string;
                A14: string;
                AchainableJsonObject: string;
            };
        };
        AssertionSupportedNetwork: {
            _enum: string[];
        };
        GenericEventWithAccount: {
            account: string;
        };
        SetUserShieldingKeyResult: {
            id_graph: string;
        };
        LinkIdentityResult: {
            id_graph: string;
        };
        RequestVCResult: {
            vc_index: string;
            vc_hash: string;
            vc_payload: string;
        };
        ErrorDetail: {
            _enum: {
                ImportError: string;
                UnauthorizedSigner: string;
                StfError: string;
                SendStfRequestFailed: string;
                UserShieldingKeyNotFound: string;
                ParseError: string;
                DataProviderError: string;
                InvalidIdentity: string;
                WrongWeb2Handle: string;
                UnexpectedMessage: string;
                WrongSignatureType: string;
                VerifySubstrateSignatureFailed: string;
                VerifyEvmSignatureFailed: string;
                RecoverEvmAddressFailed: string;
                Web3NetworkOutOfBounds: string;
            };
        };
        StfError: {
            _enum: {
                MissingPrivileges: string;
                RequireEnclaveSignerAccount: string;
                Dispatch: string;
                MissingFunds: string;
                InvalidNonce: string;
                StorageHashMismatch: string;
                InvalidStorageDiff: string;
                InvalidMetadata: string;
                SetUserShieldingKeyFailed: string;
                LinkIdentityFailed: string;
                DeactivateIdentityFailed: string;
                ActivateIdentityFailed: string;
                RequestVCFailed: string;
                SetScheduledMrEnclaveFailed: string;
                SetIdentityNetworksFailed: string;
                InvalidAccount: string;
                UnclassifiedError: string;
            };
        };
        AesOutput: {
            ciphertext: string;
            aad: string;
            nonce: string;
        };
    };
};
export default _default;
//# sourceMappingURL=definitions.d.ts.map
