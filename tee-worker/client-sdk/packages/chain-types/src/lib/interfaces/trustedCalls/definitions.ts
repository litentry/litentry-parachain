/**
 * Reference:
 * @see All Parachain type definitions https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/ts-tests/parachain-interfaces/identity/definitions.ts
 * @see Polkadot.js Docs https://polkadot.js.org/docs/api/start/typescript.user
 */
export default {
  types: {
    WorkerRpcReturnValue: {
      value: 'Bytes',
      do_watch: 'bool',
      status: 'DirectRequestStatus',
    },
    DirectRequestStatus: {
      _enum: {
        Ok: null,
        TrustedOperationStatus: '(TrustedOperationStatus, H256)',
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
        TopExecuted: 'Bytes',
      },
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
        nonce: '(LitentryIdentity)',
        id_graph_hash: '(LitentryIdentity)',
      },
    },
    TrustedGetterSigned: {
      getter: 'TrustedGetter',
      signature: 'LitentryMultiSignature',
    },
    TrustedGetter: {
      _enum: {
        free_balance: '(LitentryIdentity)',
        reserved_balance: '(LitentryIdentity)',
        __Unused_evm_nonce: 'Null',
        __Unused_evm_account_codes: 'Null',
        __Unused_evm_account_storages: 'Null',
        id_graph: '(LitentryIdentity)',
        id_graph_stats: '(LitentryIdentity)',
      },
    },
    // All enum properties must be kept regardless of whether they are used or not.
    // Otherwise, a "Trusted operation has invalid format" error will be thrown by the worker.
    // Reference: https://github.com/litentry/litentry-parachain/blob/ade21845ad72baa831e06127cdcacbd25a68d52f/tee-worker/app-libs/stf/src/trusted_call.rs#L64
    TrustedCall: {
      _enum: {
        link_identity:
          '(LitentryIdentity, LitentryIdentity, LitentryIdentity, LitentryValidationData, Vec<Web3Network>, Option<AesKey>, H256)',
        deactivate_identity:
          '(LitentryIdentity, LitentryIdentity, LitentryIdentity, Option<AesKey>, H256)',
        activate_identity:
          '(LitentryIdentity, LitentryIdentity, LitentryIdentity, Option<AesKey>, H256)',
        request_vc:
          '(LitentryIdentity, LitentryIdentity, CorePrimitivesAssertion, Option<AesKey>, H256)',
        set_identity_networks:
          '(LitentryIdentity, LitentryIdentity, LitentryIdentity, Vec<Web3Network>, Option<AesKey>, H256)',
        __Unused_remove_identity: 'Null',
        request_batch_vc:
          '(LitentryIdentity, LitentryIdentity, Vec<CorePrimitivesAssertion>, Option<AesKey>, H256)',
      },
    },
    AesKey: '[u8; 32]',

    // Direct Requests
    ShardIdentifier: 'H256',
    Request: {
      shard: 'ShardIdentifier',
      payload: 'Bytes',
    },
    EncryptedAesRequest: {
      shard: 'ShardIdentifier',
      key: 'Bytes',
      payload: 'KeyAesOutput',
    },
    KeyAesOutput: {
      ciphertext: 'Bytes',
      aad: 'Bytes',
      nonce: '[u8;12]',
    },

    // Direct responses
    LinkIdentityResult: {
      mutated_id_graph: 'KeyAesOutput',
      id_graph_hash: 'H256',
    },
    SetIdentityNetworksResult: {
      mutated_id_graph: 'KeyAesOutput',
      id_graph_hash: 'H256',
    },
    RequestVCResult: {
      vc_payload: 'KeyAesOutput',
      // see comments in `lc-vc-task-receiver` why it's prefixed with `pre...`
      // they should be referenced/used only when the client's local IDGraph is empty
      pre_mutated_id_graph: 'KeyAesOutput',
      pre_id_graph_hash: 'H256',
    },

    RequestVcResultOrError: {
      result: 'Result<Vec<u8>, RequestVcErrorDetail>',
      idx: 'u8',
      len: 'u8',
    },

    RequestVcErrorDetail: {
      _enum: {
        UnexpectedCall: 'Text',
        DuplicateAssertionRequest: null,
        ShieldingKeyRetrievalFailed: 'Text', // Stringified itp_sgx_crypto::Error
        RequestPayloadDecodingFailed: null,
        SidechainDataRetrievalFailed: 'Text', // Stringified itp_stf_state_handler::Error
        IdentityAlreadyLinked: null,
        NoEligibleIdentity: null,
        InvalidSignerAccount: null,
        UnauthorizedSigner: null,
        AssertionBuildFailed: '(VCMPError)',
        MissingAesKey: null,
        MrEnclaveRetrievalFailed: null,
        EnclaveSignerRetrievalFailed: null,
        SignatureVerificationFailed: null,
        ConnectionHashNotFound: 'Text',
        MetadataRetrievalFailed: 'Text', // Stringified itp_node_api_metadata_provider::Error
        InvalidMetadata: 'Text', // Stringified itp_node_api_metadata::Error
        TrustedCallSendingFailed: 'Text', // Stringified mpsc::SendError<: (H256, TrustedCall)>
        CallSendingFailed: 'Text',
        ExtrinsicConstructionFailed: 'Text', // Stringified itp_extrinsics_factory::Error
        ExtrinsicSendingFailed: 'Text', // Stringified sgx_status_t
      },
    },

    VCMPError: {
      _enum: {
        RequestVCFailed: '(CorePrimitivesAssertion, ErrorDetail)',
        UnclassifiedError: '(ErrorDetail)',
      },
    },

    // Errors
    StfError: {
      _enum: {
        LinkIdentityFailed: '(ErrorDetail)',
        DeactivateIdentityFailed: '(ErrorDetail)',
        ActivateIdentityFailed: '(ErrorDetail)',
        RequestVCFailed: '(CorePrimitivesAssertion, ErrorDetail)',
        SetScheduledMrEnclaveFailed: 'Null',
        SetIdentityNetworksFailed: '(ErrorDetail)',
        InvalidAccount: 'Null',
        UnclassifiedError: 'Null',
        RemoveIdentityFailed: '(ErrorDetail)',
        EmptyIDGraph: 'Null',
        __Unused10: 'Null',
        __Unused11: 'Null',
        __Unused12: 'Null',
        __Unused13: 'Null',
        __Unused14: 'Null',
        __Unused15: 'Null',
        __Unused16: 'Null',
        __Unused17: 'Null',
        __Unused18: 'Null',
        __Unused19: 'Null',
        MissingPrivileges: '(LitentryIdentity)',
        RequireEnclaveSignerAccount: 'Null',
        Dispatch: '(String)',
        MissingFunds: 'Null',
        InvalidNonce: '(Index, Index)',
        StorageHashMismatch: 'Null',
        InvalidStorageDiff: 'Null',
        InvalidMetadata: 'Null',
      },
    },
    ErrorDetail: {
      _enum: {
        ImportError: 'Null',
        UnauthorizedSigner: 'Null',
        StfError: '(Bytes)',
        SendStfRequestFailed: 'Null',
        ParseError: 'Null',
        DataProviderError: '(Bytes)',
        InvalidIdentity: 'Null',
        WrongWeb2Handle: 'Null',
        UnexpectedMessage: 'Null',
        __Unused_WrongSignatureType: 'Null',
        VerifyWeb3SignatureFailed: 'Null',
        NoEligibleIdentity: 'Null',
      },
    },
  },
};
