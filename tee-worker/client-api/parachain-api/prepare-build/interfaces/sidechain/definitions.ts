export default {
    types: {
        WorkerRpcReturnValue: {
            value: "Vec<u8>",
            do_watch: "bool",
            status: "DirectRequestStatus",
        },
        DirectRequestStatus: {
            _enum: {
                Ok: null,
                TrustedOperationStatus: "(TrustedOperationStatus, H256)",
                Error: null,
            },
        },
        AesOutput: {
            ciphertext: "Vec<u8>",
            aad: "Vec<u8>",
            nonce: "[u8; 12]",
        },
        RsaRequest: {
            shard: "ShardIdentifier",
            payload: "Vec<u8>",
        },
        AesRequest: {
            shard: "ShardIdentifier",
            key: "Vec<u8>",
            payload: "AesOutput",
        },
        RequestAesKey: "[u8; 32]",
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
                id_graph_hash: "(LitentryIdentity)",
            },
        },

        StfError: {
            _enum: {
                MissingPrivileges: "(LitentryIdentity)",
                RequireEnclaveSignerAccount: "Null",
                Dispatch: "(String)",
                MissingFunds: "Null",
                InvalidNonce: "(Index, Index)",
                StorageHashMismatch: "Null",
                InvalidStorageDiff: "Null",
                InvalidMetadata: "Null",
                LinkIdentityFailed: "(ErrorDetail)",
                DeactivateIdentityFailed: "(ErrorDetail)",
                ActivateIdentityFailed: "(ErrorDetail)",
                RequestVCFailed: "(Assertion, ErrorDetail)",
                SetScheduledMrEnclaveFailed: "Null",
                SetIdentityNetworksFailed: "(ErrorDetail)",
                InvalidAccount: "Null",
                UnclassifiedError: "Null",
                RemoveIdentityFailed: "(ErrorDetail)",
                EmptyIDGraph: "Null",
            },
        },
        ErrorDetail: {
            _enum: {
                ImportError: "Null",
                UnauthorizedSigner: "Null",
                StfError: "(Bytes)",
                SendStfRequestFailed: "Null",
                ParseError: "Null",
                DataProviderError: "(Bytes)",
                InvalidIdentity: "Null",
                WrongWeb2Handle: "Null",
                UnexpectedMessage: "Null",
                __Unused_WrongSignatureType: "Null",
                VerifyWeb3SignatureFailed: "Null",
                NoEligibleIdentity: "Null",
            },
        },
        ShardIdentifier: "H256",
    },
};
