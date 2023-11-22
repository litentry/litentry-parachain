export default {
    types: {
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
        TrustedGetterSigned: {
            getter: "TrustedGetter",
            signature: "LitentryMultiSignature",
        },

        //important
        TrustedGetter: {
            _enum: {
                free_balance: "(LitentryIdentity)",
                reserved_balance: "(LitentryIdentity)",
                __Unused_evm_nonce: "Null",
                __Unused_evm_account_codes: "Null",
                __Unused_evm_account_storages: "Null",
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
                __Unused_evm_withdraw: "Null",
                __Unused_evm_call: "Null",
                __Unused_evm_create: "Null",
                __Unused_evm_create2: "Null",
                link_identity:
                    "(LitentryIdentity, LitentryIdentity, LitentryIdentity, LitentryValidationData, Vec<Web3Network>, Option<RequestAesKey>, H256)",
                deactivate_identity: "(LitentryIdentity, LitentryIdentity, LitentryIdentity, H256)",
                activate_identity: "(LitentryIdentity, LitentryIdentity, LitentryIdentity, H256)",
                request_vc: "(LitentryIdentity, LitentryIdentity, Assertion, Option<RequestAesKey>, H256)",
                set_identity_networks: "(LitentryIdentity, LitentryIdentity, LitentryIdentity, Vec<Web3Network>, H256)",
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
                TopExecuted: "Bytes",
            },
        },
    },
};
