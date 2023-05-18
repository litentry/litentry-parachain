export default {
    types: {
        WorkerRpcReturnString: {
            vec: 'Bytes',
        },
        WorkerRpcReturnValue: {
            value: 'Bytes',
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
            signature: 'MultiSignature',
        },
        Getter: {
            _enum: {
                public: '(PublicGetter)',
                trusted: '(TrustedGetterSigned)',
            },
        },
        PublicGetter: {
            _enum: ['some_value'],
        },
        TrustedGetterSigned: {
            getter: 'TrustedGetter',
            signature: 'MultiSignature',
        },

        //important
        TrustedGetter: {
            _enum: {
                free_balance: '(AccountId)',
            },
        },
        //important
        TrustedCall: {
            _enum: {
                balance_set_balance: '(AccountId, AccountId, Balance, Balance)',
                balance_transfer: '(AccountId, AccountId, Balance)',
                balance_unshield: '(AccountId, AccountId, Balance, ShardIdentifier)',
                balance_shield: '(AccountId, AccountId, Balance)',
                set_user_shielding_key_direct: '(AccountId, UserShieldingKeyType, H256)',
                create_identity_direct: '(AccountId, LitentryIdentity, Option<Vec<u8>>, u32, H256)',
            },
        },
        UserShieldingKeyType: '[u8; 32]',
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
            signature: 'IdentityMultiSignature',
        },

        IdentityMultiSignature: {
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
        IdentityContext: {
            metadata: 'Option<Vec<u8>>',
            linking_request_block: 'Option<BlockNumber>',
            verification_request_block: 'Option<BlockNumber>',
            is_verified: 'bool',
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
