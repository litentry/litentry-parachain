// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
    /**
     * Lookup3: frame_system::AccountInfo<Index, pallet_balances::AccountData<Balance>>
     **/
    FrameSystemAccountInfo: {
        nonce: 'u32',
        consumers: 'u32',
        providers: 'u32',
        sufficients: 'u32',
        data: 'PalletBalancesAccountData',
    },
    /**
     * Lookup5: pallet_balances::AccountData<Balance>
     **/
    PalletBalancesAccountData: {
        free: 'u128',
        reserved: 'u128',
        miscFrozen: 'u128',
        feeFrozen: 'u128',
    },
    /**
     * Lookup7: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
     **/
    FrameSupportDispatchPerDispatchClassWeight: {
        normal: 'SpWeightsWeightV2Weight',
        operational: 'SpWeightsWeightV2Weight',
        mandatory: 'SpWeightsWeightV2Weight',
    },
    /**
     * Lookup8: sp_weights::weight_v2::Weight
     **/
    SpWeightsWeightV2Weight: {
        refTime: 'Compact<u64>',
        proofSize: 'Compact<u64>',
    },
    /**
     * Lookup13: sp_runtime::generic::digest::Digest
     **/
    SpRuntimeDigest: {
        logs: 'Vec<SpRuntimeDigestDigestItem>',
    },
    /**
     * Lookup15: sp_runtime::generic::digest::DigestItem
     **/
    SpRuntimeDigestDigestItem: {
        _enum: {
            Other: 'Bytes',
            __Unused1: 'Null',
            __Unused2: 'Null',
            __Unused3: 'Null',
            Consensus: '([u8;4],Bytes)',
            Seal: '([u8;4],Bytes)',
            PreRuntime: '([u8;4],Bytes)',
            __Unused7: 'Null',
            RuntimeEnvironmentUpdated: 'Null',
        },
    },
    /**
     * Lookup18: frame_system::EventRecord<ita_sgx_runtime::RuntimeEvent, primitive_types::H256>
     **/
    FrameSystemEventRecord: {
        phase: 'FrameSystemPhase',
        event: 'Event',
        topics: 'Vec<H256>',
    },
    /**
     * Lookup20: frame_system::pallet::Event<T>
     **/
    FrameSystemEvent: {
        _enum: {
            ExtrinsicSuccess: {
                dispatchInfo: 'FrameSupportDispatchDispatchInfo',
            },
            ExtrinsicFailed: {
                dispatchError: 'SpRuntimeDispatchError',
                dispatchInfo: 'FrameSupportDispatchDispatchInfo',
            },
            CodeUpdated: 'Null',
            NewAccount: {
                account: 'AccountId32',
            },
            KilledAccount: {
                account: 'AccountId32',
            },
            Remarked: {
                _alias: {
                    hash_: 'hash',
                },
                sender: 'AccountId32',
                hash_: 'H256',
            },
        },
    },
    /**
     * Lookup21: frame_support::dispatch::DispatchInfo
     **/
    FrameSupportDispatchDispatchInfo: {
        weight: 'SpWeightsWeightV2Weight',
        class: 'FrameSupportDispatchDispatchClass',
        paysFee: 'FrameSupportDispatchPays',
    },
    /**
     * Lookup22: frame_support::dispatch::DispatchClass
     **/
    FrameSupportDispatchDispatchClass: {
        _enum: ['Normal', 'Operational', 'Mandatory'],
    },
    /**
     * Lookup23: frame_support::dispatch::Pays
     **/
    FrameSupportDispatchPays: {
        _enum: ['Yes', 'No'],
    },
    /**
     * Lookup24: sp_runtime::DispatchError
     **/
    SpRuntimeDispatchError: {
        _enum: {
            Other: 'Null',
            CannotLookup: 'Null',
            BadOrigin: 'Null',
            Module: 'SpRuntimeModuleError',
            ConsumerRemaining: 'Null',
            NoProviders: 'Null',
            TooManyConsumers: 'Null',
            Token: 'SpRuntimeTokenError',
            Arithmetic: 'SpArithmeticArithmeticError',
            Transactional: 'SpRuntimeTransactionalError',
            Exhausted: 'Null',
            Corruption: 'Null',
            Unavailable: 'Null',
        },
    },
    /**
     * Lookup25: sp_runtime::ModuleError
     **/
    SpRuntimeModuleError: {
        index: 'u8',
        error: '[u8;4]',
    },
    /**
     * Lookup26: sp_runtime::TokenError
     **/
    SpRuntimeTokenError: {
        _enum: ['NoFunds', 'WouldDie', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported'],
    },
    /**
     * Lookup27: sp_arithmetic::ArithmeticError
     **/
    SpArithmeticArithmeticError: {
        _enum: ['Underflow', 'Overflow', 'DivisionByZero'],
    },
    /**
     * Lookup28: sp_runtime::TransactionalError
     **/
    SpRuntimeTransactionalError: {
        _enum: ['LimitReached', 'NoLayer'],
    },
    /**
     * Lookup29: pallet_balances::pallet::Event<T, I>
     **/
    PalletBalancesEvent: {
        _enum: {
            Endowed: {
                account: 'AccountId32',
                freeBalance: 'u128',
            },
            DustLost: {
                account: 'AccountId32',
                amount: 'u128',
            },
            Transfer: {
                from: 'AccountId32',
                to: 'AccountId32',
                amount: 'u128',
            },
            BalanceSet: {
                who: 'AccountId32',
                free: 'u128',
                reserved: 'u128',
            },
            Reserved: {
                who: 'AccountId32',
                amount: 'u128',
            },
            Unreserved: {
                who: 'AccountId32',
                amount: 'u128',
            },
            ReserveRepatriated: {
                from: 'AccountId32',
                to: 'AccountId32',
                amount: 'u128',
                destinationStatus: 'FrameSupportTokensMiscBalanceStatus',
            },
            Deposit: {
                who: 'AccountId32',
                amount: 'u128',
            },
            Withdraw: {
                who: 'AccountId32',
                amount: 'u128',
            },
            Slashed: {
                who: 'AccountId32',
                amount: 'u128',
            },
        },
    },
    /**
     * Lookup30: frame_support::traits::tokens::misc::BalanceStatus
     **/
    FrameSupportTokensMiscBalanceStatus: {
        _enum: ['Free', 'Reserved'],
    },
    /**
     * Lookup31: pallet_transaction_payment::pallet::Event<T>
     **/
    PalletTransactionPaymentEvent: {
        _enum: {
            TransactionFeePaid: {
                who: 'AccountId32',
                actualFee: 'u128',
                tip: 'u128',
            },
        },
    },
    /**
     * Lookup32: pallet_sudo::pallet::Event<T>
     **/
    PalletSudoEvent: {
        _enum: {
            Sudid: {
                sudoResult: 'Result<Null, SpRuntimeDispatchError>',
            },
            KeyChanged: {
                oldSudoer: 'Option<AccountId32>',
            },
            SudoAsDone: {
                sudoResult: 'Result<Null, SpRuntimeDispatchError>',
            },
        },
    },
    /**
     * Lookup36: pallet_identity_management_tee::pallet::Event<T>
     **/
    PalletIdentityManagementTeeEvent: {
        _enum: {
            UserShieldingKeySet: {
                who: 'AccountId32',
                key: '[u8;32]',
            },
            ChallengeCodeSet: {
                who: 'AccountId32',
                identity: 'LitentryPrimitivesIdentity',
                code: '[u8;16]',
            },
            ChallengeCodeRemoved: {
                who: 'AccountId32',
                identity: 'LitentryPrimitivesIdentity',
            },
            IdentityCreated: {
                who: 'AccountId32',
                identity: 'LitentryPrimitivesIdentity',
            },
            IdentityRemoved: {
                who: 'AccountId32',
                identity: 'LitentryPrimitivesIdentity',
            },
        },
    },
    /**
     * Lookup37: litentry_primitives::identity::Identity
     **/
    LitentryPrimitivesIdentity: {
        _enum: {
            Substrate: {
                network: 'LitentryPrimitivesIdentitySubstrateNetwork',
                address: 'LitentryPrimitivesIdentityAddress32',
            },
            Evm: {
                network: 'LitentryPrimitivesIdentityEvmNetwork',
                address: 'LitentryPrimitivesIdentityAddress20',
            },
            Web2: {
                network: 'LitentryPrimitivesIdentityWeb2Network',
                address: 'Bytes',
            },
        },
    },
    /**
     * Lookup38: litentry_primitives::identity::SubstrateNetwork
     **/
    LitentryPrimitivesIdentitySubstrateNetwork: {
        _enum: ['Polkadot', 'Kusama', 'Litentry', 'Litmus', 'LitentryRococo', 'Khala', 'TestNet'],
    },
    /**
     * Lookup39: litentry_primitives::identity::Address32
     **/
    LitentryPrimitivesIdentityAddress32: '[u8;32]',
    /**
     * Lookup40: litentry_primitives::identity::EvmNetwork
     **/
    LitentryPrimitivesIdentityEvmNetwork: {
        _enum: ['Ethereum', 'BSC'],
    },
    /**
     * Lookup41: litentry_primitives::identity::Address20
     **/
    LitentryPrimitivesIdentityAddress20: '[u8;20]',
    /**
     * Lookup43: litentry_primitives::identity::Web2Network
     **/
    LitentryPrimitivesIdentityWeb2Network: {
        _enum: ['Twitter', 'Discord', 'Github'],
    },
    /**
     * Lookup46: frame_system::Phase
     **/
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: 'u32',
            Finalization: 'Null',
            Initialization: 'Null',
        },
    },
    /**
     * Lookup50: frame_system::LastRuntimeUpgradeInfo
     **/
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: 'Compact<u32>',
        specName: 'Text',
    },
    /**
     * Lookup54: frame_system::pallet::Call<T>
     **/
    FrameSystemCall: {
        _enum: {
            remark: {
                remark: 'Bytes',
            },
            set_heap_pages: {
                pages: 'u64',
            },
            set_code: {
                code: 'Bytes',
            },
            set_code_without_checks: {
                code: 'Bytes',
            },
            set_storage: {
                items: 'Vec<(Bytes,Bytes)>',
            },
            kill_storage: {
                _alias: {
                    keys_: 'keys',
                },
                keys_: 'Vec<Bytes>',
            },
            kill_prefix: {
                prefix: 'Bytes',
                subkeys: 'u32',
            },
            remark_with_event: {
                remark: 'Bytes',
            },
        },
    },
    /**
     * Lookup58: frame_system::limits::BlockWeights
     **/
    FrameSystemLimitsBlockWeights: {
        baseBlock: 'SpWeightsWeightV2Weight',
        maxBlock: 'SpWeightsWeightV2Weight',
        perClass: 'FrameSupportDispatchPerDispatchClassWeightsPerClass',
    },
    /**
     * Lookup59: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
     **/
    FrameSupportDispatchPerDispatchClassWeightsPerClass: {
        normal: 'FrameSystemLimitsWeightsPerClass',
        operational: 'FrameSystemLimitsWeightsPerClass',
        mandatory: 'FrameSystemLimitsWeightsPerClass',
    },
    /**
     * Lookup60: frame_system::limits::WeightsPerClass
     **/
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: 'SpWeightsWeightV2Weight',
        maxExtrinsic: 'Option<SpWeightsWeightV2Weight>',
        maxTotal: 'Option<SpWeightsWeightV2Weight>',
        reserved: 'Option<SpWeightsWeightV2Weight>',
    },
    /**
     * Lookup62: frame_system::limits::BlockLength
     **/
    FrameSystemLimitsBlockLength: {
        max: 'FrameSupportDispatchPerDispatchClassU32',
    },
    /**
     * Lookup63: frame_support::dispatch::PerDispatchClass<T>
     **/
    FrameSupportDispatchPerDispatchClassU32: {
        normal: 'u32',
        operational: 'u32',
        mandatory: 'u32',
    },
    /**
     * Lookup64: sp_weights::RuntimeDbWeight
     **/
    SpWeightsRuntimeDbWeight: {
        read: 'u64',
        write: 'u64',
    },
    /**
     * Lookup65: sp_version::RuntimeVersion
     **/
    SpVersionRuntimeVersion: {
        specName: 'Text',
        implName: 'Text',
        authoringVersion: 'u32',
        specVersion: 'u32',
        implVersion: 'u32',
        apis: 'Vec<([u8;8],u32)>',
        transactionVersion: 'u32',
        stateVersion: 'u8',
    },
    /**
     * Lookup71: frame_system::pallet::Error<T>
     **/
    FrameSystemError: {
        _enum: [
            'InvalidSpecName',
            'SpecVersionNeedsToIncrease',
            'FailedToExtractRuntimeVersion',
            'NonDefaultComposite',
            'NonZeroRefCount',
            'CallFiltered',
        ],
    },
    /**
     * Lookup72: pallet_timestamp::pallet::Call<T>
     **/
    PalletTimestampCall: {
        _enum: {
            set: {
                now: 'Compact<u64>',
            },
        },
    },
    /**
     * Lookup74: pallet_balances::BalanceLock<Balance>
     **/
    PalletBalancesBalanceLock: {
        id: '[u8;8]',
        amount: 'u128',
        reasons: 'PalletBalancesReasons',
    },
    /**
     * Lookup75: pallet_balances::Reasons
     **/
    PalletBalancesReasons: {
        _enum: ['Fee', 'Misc', 'All'],
    },
    /**
     * Lookup78: pallet_balances::ReserveData<ReserveIdentifier, Balance>
     **/
    PalletBalancesReserveData: {
        id: '[u8;8]',
        amount: 'u128',
    },
    /**
     * Lookup80: pallet_balances::pallet::Call<T, I>
     **/
    PalletBalancesCall: {
        _enum: {
            transfer: {
                dest: 'MultiAddress',
                value: 'Compact<u128>',
            },
            set_balance: {
                who: 'MultiAddress',
                newFree: 'Compact<u128>',
                newReserved: 'Compact<u128>',
            },
            force_transfer: {
                source: 'MultiAddress',
                dest: 'MultiAddress',
                value: 'Compact<u128>',
            },
            transfer_keep_alive: {
                dest: 'MultiAddress',
                value: 'Compact<u128>',
            },
            transfer_all: {
                dest: 'MultiAddress',
                keepAlive: 'bool',
            },
            force_unreserve: {
                who: 'MultiAddress',
                amount: 'u128',
            },
        },
    },
    /**
     * Lookup84: pallet_balances::pallet::Error<T, I>
     **/
    PalletBalancesError: {
        _enum: [
            'VestingBalance',
            'LiquidityRestrictions',
            'InsufficientBalance',
            'ExistentialDeposit',
            'KeepAlive',
            'ExistingVestingSchedule',
            'DeadAccount',
            'TooManyReserves',
        ],
    },
    /**
     * Lookup86: pallet_transaction_payment::Releases
     **/
    PalletTransactionPaymentReleases: {
        _enum: ['V1Ancient', 'V2'],
    },
    /**
     * Lookup87: pallet_sudo::pallet::Call<T>
     **/
    PalletSudoCall: {
        _enum: {
            sudo: {
                call: 'Call',
            },
            sudo_unchecked_weight: {
                call: 'Call',
                weight: 'SpWeightsWeightV2Weight',
            },
            set_key: {
                _alias: {
                    new_: 'new',
                },
                new_: 'MultiAddress',
            },
            sudo_as: {
                who: 'MultiAddress',
                call: 'Call',
            },
        },
    },
    /**
     * Lookup89: pallet_parentchain::pallet::Call<T>
     **/
    PalletParentchainCall: {
        _enum: {
            set_block: {
                header: 'SpRuntimeHeader',
            },
        },
    },
    /**
     * Lookup90: sp_runtime::generic::header::Header<Number, sp_runtime::traits::BlakeTwo256>
     **/
    SpRuntimeHeader: {
        parentHash: 'H256',
        number: 'Compact<u32>',
        stateRoot: 'H256',
        extrinsicsRoot: 'H256',
        digest: 'SpRuntimeDigest',
    },
    /**
     * Lookup91: sp_runtime::traits::BlakeTwo256
     **/
    SpRuntimeBlakeTwo256: 'Null',
    /**
     * Lookup92: pallet_identity_management_tee::pallet::Call<T>
     **/
    PalletIdentityManagementTeeCall: {
        _enum: {
            set_user_shielding_key: {
                who: 'AccountId32',
                key: '[u8;32]',
                parentSs58Prefix: 'u16',
            },
            set_challenge_code: {
                who: 'AccountId32',
                identity: 'LitentryPrimitivesIdentity',
                code: '[u8;16]',
            },
            remove_challenge_code: {
                who: 'AccountId32',
                identity: 'LitentryPrimitivesIdentity',
            },
            create_identity: {
                who: 'AccountId32',
                identity: 'LitentryPrimitivesIdentity',
                metadata: 'Option<Bytes>',
                creationRequestBlock: 'u32',
                parentSs58Prefix: 'u16',
            },
            remove_identity: {
                who: 'AccountId32',
                identity: 'LitentryPrimitivesIdentity',
            },
            verify_identity: {
                who: 'AccountId32',
                identity: 'LitentryPrimitivesIdentity',
                verificationRequestBlock: 'u32',
            },
        },
    },
    /**
     * Lookup95: pallet_sudo::pallet::Error<T>
     **/
    PalletSudoError: {
        _enum: ['RequireSudo'],
    },
    /**
     * Lookup97: pallet_identity_management_tee::identity_context::IdentityContext<T>
     **/
    PalletIdentityManagementTeeIdentityContext: {
        metadata: 'Option<Bytes>',
        creationRequestBlock: 'Option<u32>',
        verificationRequestBlock: 'Option<u32>',
        isVerified: 'bool',
    },
    /**
     * Lookup99: pallet_identity_management_tee::pallet::Error<T>
     **/
    PalletIdentityManagementTeeError: {
        _enum: [
            'ChallengeCodeNotExist',
            'IdentityAlreadyVerified',
            'IdentityNotExist',
            'IdentityNotCreated',
            'CreatePrimeIdentityNotAllowed',
            'VerificationRequestTooEarly',
            'VerificationRequestTooLate',
            'RemovePrimeIdentityDisallowed',
        ],
    },
    /**
     * Lookup101: sp_runtime::MultiSignature
     **/
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: 'SpCoreEd25519Signature',
            Sr25519: 'SpCoreSr25519Signature',
            Ecdsa: 'SpCoreEcdsaSignature',
        },
    },
    /**
     * Lookup102: sp_core::ed25519::Signature
     **/
    SpCoreEd25519Signature: '[u8;64]',
    /**
     * Lookup104: sp_core::sr25519::Signature
     **/
    SpCoreSr25519Signature: '[u8;64]',
    /**
     * Lookup105: sp_core::ecdsa::Signature
     **/
    SpCoreEcdsaSignature: '[u8;65]',
    /**
     * Lookup108: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
     **/
    FrameSystemExtensionsCheckNonZeroSender: 'Null',
    /**
     * Lookup109: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
     **/
    FrameSystemExtensionsCheckSpecVersion: 'Null',
    /**
     * Lookup110: frame_system::extensions::check_tx_version::CheckTxVersion<T>
     **/
    FrameSystemExtensionsCheckTxVersion: 'Null',
    /**
     * Lookup111: frame_system::extensions::check_genesis::CheckGenesis<T>
     **/
    FrameSystemExtensionsCheckGenesis: 'Null',
    /**
     * Lookup114: frame_system::extensions::check_nonce::CheckNonce<T>
     **/
    FrameSystemExtensionsCheckNonce: 'Compact<u32>',
    /**
     * Lookup115: frame_system::extensions::check_weight::CheckWeight<T>
     **/
    FrameSystemExtensionsCheckWeight: 'Null',
    /**
     * Lookup116: pallet_transaction_payment::ChargeTransactionPayment<T>
     **/
    PalletTransactionPaymentChargeTransactionPayment: 'Compact<u128>',
    /**
     * Lookup117: ita_sgx_runtime::Runtime
     **/
    ItaSgxRuntimeRuntime: 'Null',
};
