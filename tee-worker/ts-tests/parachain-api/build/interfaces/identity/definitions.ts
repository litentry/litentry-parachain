export default {
    types: {
        WorkerRpcReturnValue: {
            value: "Vec<u8>",
            do_watch: "bool",
            status: "DirectRequestStatus",
        },
        TrustedOperation: {
            _enum: {
                indirect_call: "(TrustedCallSigned)",
                direct_call: "(TrustedCallSigned)",
                get: "(Getter)",
            },
        },
        TrustedCallSigned: {
            call: "TrustedCall",
            index: "u32",
            signature: "LitentryMultiSignature",
        },
        Getter: {
            _enum: {
                public: "(PublicGetter)",
                trusted: "(TrustedGetterSigned)",
            },
        },
        PublicGetter: {
            _enum: {
                some_value: "u32",
                nonce: "(LitentryIdentity)",
            },
        },
        TrustedGetterSigned: {
            getter: "TrustedGetter",
            signature: "LitentryMultiSignature",
        },

        //important
        TrustedGetter: {
            _enum: {
                free_balance: "(LitentryIdentity)",
                reserved_balance: "(LitentryIdentity)",
                user_shielding_key: "(LitentryIdentity)",
                id_graph: "(LitentryIdentity)",
                id_graph_stats: "(LitentryIdentity)",
            },
        },
        //important
        TrustedCall: {
            _enum: {
                balance_set_balance: "(LitentryIdentity, LitentryIdentity, Balance, Balance)",
                balance_transfer: "(LitentryIdentity, LitentryIdentity, Balance)",
                balance_unshield: "(LitentryIdentity, LitentryIdentity, Balance, ShardIdentifier)",
                balance_shield: "(LitentryIdentity, LitentryIdentity, Balance)",
                set_user_shielding_key:
                    "(LitentryIdentity, LitentryIdentity, UserShieldingKeyType, H256)",
                link_identity:
                    "(LitentryIdentity, LitentryIdentity, LitentryIdentity, LitentryValidationData, Vec<Web3Network>, UserShieldingKeyNonceType, H256)",
                remove_identity: "(LitentryIdentity, LitentryIdentity, LitentryIdentity, H256)",
                request_vc: "(LitentryIdentity, LitentryIdentity, Assertion, H256)",
                set_identity_networks:
                    "(LitentryIdentity, LitentryIdentity, LitentryIdentity, Vec<Web3Network>)",
            },
        },
        UserShieldingKeyType: "[u8; 32]",
        UserShieldingKeyNonceType: "[u8; 12]",
        DirectRequestStatus: {
            _enum: {
                Ok: null,
                TrustedOperationStatus: "(TrustedOperationStatus, H256)",
                Error: null,
            },
        },
        TrustedOperationStatus: {
            _enum: {
                Submitted: null,
                Future: null,
                Ready: null,
                Broadcast: null,
                InSidechainBlock: "H256",
                Retracted: null,
                FinalityTimeout: null,
                Finalized: null,
                Usurped: null,
                Dropped: null,
                Invalid: null,
            },
        },

        // identity management
        LitentryIdentity: {
            _enum: {
                Twitter: "IdentityString",
                Discord: "IdentityString",
                Github: "IdentityString",
                Substrate: "Address32",
                Evm: "Address20",
            },
        },
        Address32: "[u8;32]",
        Address20: "[u8;20]",
        IdentityString: "Vec<u8>",
        Web3Network: {
            _enum: [
                "Polkadot",
                "Kusama",
                "Litentry",
                "Litmus",
                "LitentryRococo",
                "Khala",
                "SubstrateTestnet",
                "Ethereum",
                "Polygon",
                "BSC",
            ],
        },
        LitentryValidationData: {
            _enum: {
                Web2Validation: "Web2ValidationData",
                Web3Validation: "Web3ValidationData",
            },
        },
        Web2ValidationData: {
            _enum: {
                Twitter: "TwitterValidationData",
                Discord: "DiscordValidationData",
            },
        },
        TwitterValidationData: {
            tweet_id: "Vec<u8>",
        },
        DiscordValidationData: {
            channel_id: "Vec<u8>",
            message_id: "Vec<u8>",
            guild_id: "Vec<u8>",
        },
        Web3ValidationData: {
            _enum: {
                Substrate: "Web3CommonValidationData",
                Evm: "Web3CommonValidationData",
            },
        },
        Web3CommonValidationData: {
            message: "Vec<u8>",
            signature: "LitentryMultiSignature",
        },

        LitentryMultiSignature: {
            _enum: {
                Ed25519: "ed25519::Signature",
                Sr25519: "sr25519::Signature",
                Ecdsa: "ecdsa::Signature",
                Ethereum: "EthereumSignature",
            },
        },
        EthereumSignature: "([u8; 65])",

        IdentityGenericEvent: {
            who: "AccountId",
            identity: "LitentryIdentity",
            id_graph: "Vec<(LitentryIdentity, IdentityContext)>",
        },

        IdentityStatus: {
            _enum: ["Active", "Inactive"],
        },

        IdentityContext: {
            link_block: "BlockNumber",
            web3networks: "BoundedWeb3Network",
            status: "IdentityStatus",
        },
        BoundedWeb3Network: "BoundedVec<Web3Network, ConstU32<128>>",

        // teerex
        ShardIdentifier: "H256",
        Request: {
            shard: "ShardIdentifier",
            cyphertext: "Vec<u8>",
        },

        // vc management
        VCRequested: {
            account: "AccountId",
            mrEnclave: "ShardIdentifier",
            assertion: "Assertion",
        },
        Assertion: {
            _enum: {
                A1: "Null",
                A2: "Bytes",
                A3: "(Bytes,Bytes,Bytes)",
                A4: "u128",
                A5: "(Bytes,Bytes)",
                A6: "Null",
                A7: "u128",
                A8: "Vec<Bytes>",
                A9: "Null",
                A10: "u128",
                A11: "u128",
                A13: "u32",
            },
        },
        GenericEventWithAccount: {
            account: "AccountId",
        },
        SetUserShieldingKeyResponse: {
            account: "AccountId",
            id_graph: "AesOutput",
            req_ext_hash: "H256",
        },
        AesOutput: {
            ciphertext: "Vec<u8>",
            aad: "Vec<u8>",
            nonce: "[u8; 12]",
        },
    },
};
