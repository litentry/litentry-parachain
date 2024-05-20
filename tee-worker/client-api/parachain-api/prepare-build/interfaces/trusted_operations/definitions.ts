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
            },
        },
        //important
        TrustedCall: {
            _enum: {
                link_identity:
                    "(LitentryIdentity, LitentryIdentity, LitentryIdentity, LitentryValidationData, Option<RequestAesKey>, H256)",
                deactivate_identity: "(LitentryIdentity, LitentryIdentity, LitentryIdentity, Option<RequestAesKey>, H256)",
                activate_identity: "(LitentryIdentity, LitentryIdentity, LitentryIdentity, Option<RequestAesKey>, H256)",
                request_vc: "(LitentryIdentity, LitentryIdentity, Assertion, Option<RequestAesKey>, H256)",
                __Unused_set_identity_networks: "Null",
                __Unused_remove_identity: "Null",
                request_batch_vc: "(LitentryIdentity, LitentryIdentity, BoundedVec<Assertion, ConstU32<32>>, Option<RequestAesKey>, H256)",
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
