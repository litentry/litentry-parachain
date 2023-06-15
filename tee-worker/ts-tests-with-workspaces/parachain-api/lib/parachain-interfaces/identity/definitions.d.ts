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
                challenge_code: string;
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
                create_identity: string;
                remove_identity: string;
                verify_identity: string;
                request_vc: string;
            };
        };
        UserShieldingKeyType: string;
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
                Substrate: string;
                Evm: string;
                Web2: string;
            };
        };
        SubstrateIdentity: {
            network: string;
            address: string;
        };
        EvmIdentity: {
            network: string;
            address: string;
        };
        Web2Identity: {
            network: string;
            address: string;
        };
        Address32: string;
        Address20: string;
        IdentityString: string;
        Web2Network: {
            _enum: string[];
        };
        SubstrateNetwork: {
            _enum: string[];
        };
        EvmNetwork: {
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
        IdentityMultiSignature: {
            _enum: {
                Ed25519: string;
                Sr25519: string;
                Ecdsa: string;
                Ethereum: string;
            };
        };
        EthereumSignature: string;
        IdentityGenericEvent: {
            who: string;
            identity: string;
            id_graph: string;
        };
        IdentityContext: {
            metadata: string;
            linking_request_block: string;
            verification_request_block: string;
            is_verified: string;
        };
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
    };
};
export default _default;
