export default {
    types: {
        WorkerRpcReturnValue: {
            value: 'Vec<u8>',
            do_watch: 'bool',
            status: 'DirectRequestStatus',
        },
        TrustedOperation: {
            _enum: {
                indirect_call: '(TrustedCallSigned)',
                direct_call: '(TrustedCallSigned)',
                get: '(Getter)',
            },
        },
        TrustedCallSigned: {
            call: 'TrustedCall',
            index: 'u32',
            signature: 'LitentryMultiSignature',
        },
        Getter: {
            _enum: {
                public: '(PublicGetter)',
                trusted: '(TrustedGetterSigned)',
            },
        },
        PublicGetter: {
            _enum: {
                some_value: 'u32',
                nonce: '(LitentryMultiAddress)',
            },
        },
        TrustedGetterSigned: {
            getter: 'TrustedGetter',
            signature: 'LitentryMultiSignature',
        },

        //important
        TrustedGetter: {
            _enum: {
                free_balance: '(LitentryMultiAddress)',
                reserved_balance: '(LitentryMultiAddress)',
                user_shielding_key: '(LitentryMultiAddress)',
                id_graph: '(LitentryMultiAddress)',
                id_graph_stats: '(LitentryMultiAddress)',
            },
        },
        //important
        TrustedCall: {
            _enum: {
                balance_set_balance: '(LitentryMultiAddress, AccountId, Balance, Balance)',
                balance_transfer: '(LitentryMultiAddress, AccountId, Balance)',
                balance_unshield: '(LitentryMultiAddress, AccountId, Balance, ShardIdentifier)',
                balance_shield: '(LitentryMultiAddress, AccountId, Balance)',
                set_user_shielding_key: '(LitentryMultiAddress, LitentryMultiAddress, UserShieldingKeyType, H256)',
                link_identity:
                    '(LitentryMultiAddress, LitentryMultiAddress, LitentryIdentity, LitentryValidationData, UserShieldingKeyNonceType, H256)',
                remove_identity: '(LitentryMultiAddress, LitentryMultiAddress, LitentryIdentity, H256)',
                request_vc: '(LitentryMultiAddress, LitentryMultiAddress, Assertion, H256)',
            },
        },
        UserShieldingKeyType: '[u8; 32]',
        UserShieldingKeyNonceType: '[u8; 12]',
        DirectRequestStatus: {
            _enum: {
                Ok: null,
                TrustedOperationStatus: 'TrustedOperationStatus',
                Error: null,
            },
        },
        TrustedOperationStatus: {
            _enum: {
                Submitted: null,
                Future: null,
                Ready: null,
                Broadcast: null,
                InSidechainBlock: 'H256',
                Retracted: null,
                FinalityTimeout: null,
                Finalized: null,
                Usurped: null,
                Dropped: null,
                Invalid: null,
            },
        },

        LitentryMultiAddress: {
            _enum: {
                Substrate: 'Address32',
                Evm: 'Address20',
            },
        },

        // identity management
        LitentryIdentity: {
            _enum: {
                Substrate: 'SubstrateIdentity',
                Evm: 'EvmIdentity',
                Web2: 'Web2Identity',
            },
        },
        SubstrateIdentity: {
            network: 'SubstrateNetwork',
            address: 'Address32',
        },
        EvmIdentity: {
            network: 'EvmNetwork',
            address: 'Address20',
        },
        Web2Identity: {
            network: 'Web2Network',
            address: 'IdentityString',
        },
        Address32: '[u8;32]',
        Address20: '[u8;20]',
        IdentityString: 'Vec<u8>',
        Web2Network: {
            _enum: ['Twitter', 'Discord', 'Github'],
        },
        SubstrateNetwork: {
            _enum: ['Polkadot', 'Kusama', 'Litentry', 'Litmus', 'LitentryRococo', 'Khala', 'TestNet'],
        },
        EvmNetwork: {
            _enum: ['Ethereum', 'BSC'],
        },
        LitentryValidationData: {
            _enum: {
                Web2Validation: 'Web2ValidationData',
                Web3Validation: 'Web3ValidationData',
            },
        },
        Web2ValidationData: {
            _enum: {
                Twitter: 'TwitterValidationData',
                Discord: 'DiscordValidationData',
            },
        },
        TwitterValidationData: {
            tweet_id: 'Vec<u8>',
        },
        DiscordValidationData: {
            channel_id: 'Vec<u8>',
            message_id: 'Vec<u8>',
            guild_id: 'Vec<u8>',
        },
        Web3ValidationData: {
            _enum: {
                Substrate: 'Web3CommonValidationData',
                Evm: 'Web3CommonValidationData',
            },
        },
        Web3CommonValidationData: {
            message: 'Vec<u8>',
            signature: 'LitentryMultiSignature',
        },

        LitentryMultiSignature: {
            _enum: {
                Ed25519: 'ed25519::Signature',
                Sr25519: 'sr25519::Signature',
                Ecdsa: 'ecdsa::Signature',
                Ethereum: 'EthereumSignature',
            },
        },
        EthereumSignature: '([u8; 65])',

        IdentityGenericEvent: {
            who: 'AccountId',
            identity: 'LitentryIdentity',
            id_graph: 'Vec<(LitentryIdentity, IdentityContext)>',
        },

        IdentityStatus: {
            _enum: ['Active', 'Inactive'],
        },

        IdentityContext: {
            link_block: 'BlockNumber',
            status: 'IdentityStatus',
        },

        // teerex
        ShardIdentifier: 'H256',
        Request: {
            shard: 'ShardIdentifier',
            cyphertext: 'Vec<u8>',
        },

        // vc management
        VCRequested: {
            account: 'AccountId',
            mrEnclave: 'ShardIdentifier',
            assertion: 'Assertion',
        },
        Assertion: {
            _enum: {
                A1: 'Null',
                A2: 'Bytes',
                A3: '(Bytes,Bytes,Bytes)',
                A4: 'u128',
                A5: '(Bytes,Bytes)',
                A6: 'Null',
                A7: 'u128',
                A8: 'Vec<Bytes>',
                A9: 'Null',
                A10: 'u128',
                A11: 'u128',
                A13: 'u32',
            },
        },
        GenericEventWithAccount: {
            account: 'AccountId',
        },
    },
};
