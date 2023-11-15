export default {
    types: {
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
                WrongSignatureType: "Null",
                VerifySubstrateSignatureFailed: "Null",
                VerifyEvmSignatureFailed: "Null",
                RecoverEvmAddressFailed: "Null",
                Web3NetworkOutOfBounds: "Null",
            },
        },
    },
};
