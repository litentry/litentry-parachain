declare const _default: {
    /**
     * Lookup3: frame_system::AccountInfo<Index, pallet_balances::AccountData<Balance>>
     **/
    FrameSystemAccountInfo: {
        nonce: string;
        consumers: string;
        providers: string;
        sufficients: string;
        data: string;
    };
    /**
     * Lookup5: pallet_balances::AccountData<Balance>
     **/
    PalletBalancesAccountData: {
        free: string;
        reserved: string;
        miscFrozen: string;
        feeFrozen: string;
    };
    /**
     * Lookup7: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
     **/
    FrameSupportDispatchPerDispatchClassWeight: {
        normal: string;
        operational: string;
        mandatory: string;
    };
    /**
     * Lookup8: sp_weights::weight_v2::Weight
     **/
    SpWeightsWeightV2Weight: {
        refTime: string;
        proofSize: string;
    };
    /**
     * Lookup13: sp_runtime::generic::digest::Digest
     **/
    SpRuntimeDigest: {
        logs: string;
    };
    /**
     * Lookup15: sp_runtime::generic::digest::DigestItem
     **/
    SpRuntimeDigestDigestItem: {
        _enum: {
            Other: string;
            __Unused1: string;
            __Unused2: string;
            __Unused3: string;
            Consensus: string;
            Seal: string;
            PreRuntime: string;
            __Unused7: string;
            RuntimeEnvironmentUpdated: string;
        };
    };
    /**
     * Lookup18: frame_system::EventRecord<ita_sgx_runtime::RuntimeEvent, primitive_types::H256>
     **/
    FrameSystemEventRecord: {
        phase: string;
        event: string;
        topics: string;
    };
    /**
     * Lookup20: frame_system::pallet::Event<T>
     **/
    FrameSystemEvent: {
        _enum: {
            ExtrinsicSuccess: {
                dispatchInfo: string;
            };
            ExtrinsicFailed: {
                dispatchError: string;
                dispatchInfo: string;
            };
            CodeUpdated: string;
            NewAccount: {
                account: string;
            };
            KilledAccount: {
                account: string;
            };
            Remarked: {
                _alias: {
                    hash_: string;
                };
                sender: string;
                hash_: string;
            };
        };
    };
    /**
     * Lookup21: frame_support::dispatch::DispatchInfo
     **/
    FrameSupportDispatchDispatchInfo: {
        weight: string;
        class: string;
        paysFee: string;
    };
    /**
     * Lookup22: frame_support::dispatch::DispatchClass
     **/
    FrameSupportDispatchDispatchClass: {
        _enum: string[];
    };
    /**
     * Lookup23: frame_support::dispatch::Pays
     **/
    FrameSupportDispatchPays: {
        _enum: string[];
    };
    /**
     * Lookup24: sp_runtime::DispatchError
     **/
    SpRuntimeDispatchError: {
        _enum: {
            Other: string;
            CannotLookup: string;
            BadOrigin: string;
            Module: string;
            ConsumerRemaining: string;
            NoProviders: string;
            TooManyConsumers: string;
            Token: string;
            Arithmetic: string;
            Transactional: string;
            Exhausted: string;
            Corruption: string;
            Unavailable: string;
        };
    };
    /**
     * Lookup25: sp_runtime::ModuleError
     **/
    SpRuntimeModuleError: {
        index: string;
        error: string;
    };
    /**
     * Lookup26: sp_runtime::TokenError
     **/
    SpRuntimeTokenError: {
        _enum: string[];
    };
    /**
     * Lookup27: sp_arithmetic::ArithmeticError
     **/
    SpArithmeticArithmeticError: {
        _enum: string[];
    };
    /**
     * Lookup28: sp_runtime::TransactionalError
     **/
    SpRuntimeTransactionalError: {
        _enum: string[];
    };
    /**
     * Lookup29: pallet_balances::pallet::Event<T, I>
     **/
    PalletBalancesEvent: {
        _enum: {
            Endowed: {
                account: string;
                freeBalance: string;
            };
            DustLost: {
                account: string;
                amount: string;
            };
            Transfer: {
                from: string;
                to: string;
                amount: string;
            };
            BalanceSet: {
                who: string;
                free: string;
                reserved: string;
            };
            Reserved: {
                who: string;
                amount: string;
            };
            Unreserved: {
                who: string;
                amount: string;
            };
            ReserveRepatriated: {
                from: string;
                to: string;
                amount: string;
                destinationStatus: string;
            };
            Deposit: {
                who: string;
                amount: string;
            };
            Withdraw: {
                who: string;
                amount: string;
            };
            Slashed: {
                who: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup30: frame_support::traits::tokens::misc::BalanceStatus
     **/
    FrameSupportTokensMiscBalanceStatus: {
        _enum: string[];
    };
    /**
     * Lookup31: pallet_transaction_payment::pallet::Event<T>
     **/
    PalletTransactionPaymentEvent: {
        _enum: {
            TransactionFeePaid: {
                who: string;
                actualFee: string;
                tip: string;
            };
        };
    };
    /**
     * Lookup32: pallet_sudo::pallet::Event<T>
     **/
    PalletSudoEvent: {
        _enum: {
            Sudid: {
                sudoResult: string;
            };
            KeyChanged: {
                oldSudoer: string;
            };
            SudoAsDone: {
                sudoResult: string;
            };
        };
    };
    /**
     * Lookup36: pallet_identity_management_tee::pallet::Event<T>
     **/
    PalletIdentityManagementTeeEvent: {
        _enum: {
            UserShieldingKeySet: {
                who: string;
                key: string;
            };
            IdentityLinked: {
                who: string;
                identity: string;
            };
            IdentityDeactivated: {
                who: string;
                identity: string;
            };
            IdentityActivated: {
                who: string;
                identity: string;
            };
        };
    };
    /**
     * Lookup37: litentry_primitives::identity::Identity
     **/
    LitentryPrimitivesIdentity: {
        _enum: {
            Twitter: string;
            Discord: string;
            Github: string;
            Substrate: string;
            Evm: string;
        };
    };
    /**
     * Lookup39: litentry_primitives::identity::Address32
     **/
    LitentryPrimitivesIdentityAddress32: string;
    /**
     * Lookup40: litentry_primitives::identity::Address20
     **/
    LitentryPrimitivesIdentityAddress20: string;
    /**
     * Lookup42: frame_system::Phase
     **/
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: string;
            Finalization: string;
            Initialization: string;
        };
    };
    /**
     * Lookup46: frame_system::LastRuntimeUpgradeInfo
     **/
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: string;
        specName: string;
    };
    /**
     * Lookup50: frame_system::pallet::Call<T>
     **/
    FrameSystemCall: {
        _enum: {
            remark: {
                remark: string;
            };
            set_heap_pages: {
                pages: string;
            };
            set_code: {
                code: string;
            };
            set_code_without_checks: {
                code: string;
            };
            set_storage: {
                items: string;
            };
            kill_storage: {
                _alias: {
                    keys_: string;
                };
                keys_: string;
            };
            kill_prefix: {
                prefix: string;
                subkeys: string;
            };
            remark_with_event: {
                remark: string;
            };
        };
    };
    /**
     * Lookup54: frame_system::limits::BlockWeights
     **/
    FrameSystemLimitsBlockWeights: {
        baseBlock: string;
        maxBlock: string;
        perClass: string;
    };
    /**
     * Lookup55: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
     **/
    FrameSupportDispatchPerDispatchClassWeightsPerClass: {
        normal: string;
        operational: string;
        mandatory: string;
    };
    /**
     * Lookup56: frame_system::limits::WeightsPerClass
     **/
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: string;
        maxExtrinsic: string;
        maxTotal: string;
        reserved: string;
    };
    /**
     * Lookup58: frame_system::limits::BlockLength
     **/
    FrameSystemLimitsBlockLength: {
        max: string;
    };
    /**
     * Lookup59: frame_support::dispatch::PerDispatchClass<T>
     **/
    FrameSupportDispatchPerDispatchClassU32: {
        normal: string;
        operational: string;
        mandatory: string;
    };
    /**
     * Lookup60: sp_weights::RuntimeDbWeight
     **/
    SpWeightsRuntimeDbWeight: {
        read: string;
        write: string;
    };
    /**
     * Lookup61: sp_version::RuntimeVersion
     **/
    SpVersionRuntimeVersion: {
        specName: string;
        implName: string;
        authoringVersion: string;
        specVersion: string;
        implVersion: string;
        apis: string;
        transactionVersion: string;
        stateVersion: string;
    };
    /**
     * Lookup67: frame_system::pallet::Error<T>
     **/
    FrameSystemError: {
        _enum: string[];
    };
    /**
     * Lookup68: pallet_timestamp::pallet::Call<T>
     **/
    PalletTimestampCall: {
        _enum: {
            set: {
                now: string;
            };
        };
    };
    /**
     * Lookup70: pallet_balances::BalanceLock<Balance>
     **/
    PalletBalancesBalanceLock: {
        id: string;
        amount: string;
        reasons: string;
    };
    /**
     * Lookup71: pallet_balances::Reasons
     **/
    PalletBalancesReasons: {
        _enum: string[];
    };
    /**
     * Lookup74: pallet_balances::ReserveData<ReserveIdentifier, Balance>
     **/
    PalletBalancesReserveData: {
        id: string;
        amount: string;
    };
    /**
     * Lookup76: pallet_balances::pallet::Call<T, I>
     **/
    PalletBalancesCall: {
        _enum: {
            transfer: {
                dest: string;
                value: string;
            };
            set_balance: {
                who: string;
                newFree: string;
                newReserved: string;
            };
            force_transfer: {
                source: string;
                dest: string;
                value: string;
            };
            transfer_keep_alive: {
                dest: string;
                value: string;
            };
            transfer_all: {
                dest: string;
                keepAlive: string;
            };
            force_unreserve: {
                who: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup80: pallet_balances::pallet::Error<T, I>
     **/
    PalletBalancesError: {
        _enum: string[];
    };
    /**
     * Lookup82: pallet_transaction_payment::Releases
     **/
    PalletTransactionPaymentReleases: {
        _enum: string[];
    };
    /**
     * Lookup83: pallet_sudo::pallet::Call<T>
     **/
    PalletSudoCall: {
        _enum: {
            sudo: {
                call: string;
            };
            sudo_unchecked_weight: {
                call: string;
                weight: string;
            };
            set_key: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            sudo_as: {
                who: string;
                call: string;
            };
        };
    };
    /**
     * Lookup85: pallet_parentchain::pallet::Call<T>
     **/
    PalletParentchainCall: {
        _enum: {
            set_block: {
                header: string;
            };
        };
    };
    /**
     * Lookup86: sp_runtime::generic::header::Header<Number, sp_runtime::traits::BlakeTwo256>
     **/
    SpRuntimeHeader: {
        parentHash: string;
        number: string;
        stateRoot: string;
        extrinsicsRoot: string;
        digest: string;
    };
    /**
     * Lookup87: sp_runtime::traits::BlakeTwo256
     **/
    SpRuntimeBlakeTwo256: string;
    /**
     * Lookup88: pallet_identity_management_tee::pallet::Call<T>
     **/
    PalletIdentityManagementTeeCall: {
        _enum: {
            set_user_shielding_key: {
                who: string;
                key: string;
            };
            link_identity: {
                who: string;
                identity: string;
                web3networks: string;
            };
            deactivate_identity: {
                who: string;
                identity: string;
            };
            activate_identity: {
                who: string;
                identity: string;
            };
            set_identity_networks: {
                who: string;
                identity: string;
                web3networks: string;
            };
        };
    };
    /**
     * Lookup90: core_primitives::network::Web3Network
     **/
    CorePrimitivesNetworkWeb3Network: {
        _enum: string[];
    };
    /**
     * Lookup91: pallet_sudo::pallet::Error<T>
     **/
    PalletSudoError: {
        _enum: string[];
    };
    /**
     * Lookup93: pallet_identity_management_tee::identity_context::IdentityContext<T>
     **/
    PalletIdentityManagementTeeIdentityContext: {
        linkBlock: string;
        web3networks: string;
        status: string;
    };
    /**
     * Lookup94: pallet_identity_management_tee::identity_context::IdentityStatus
     **/
    PalletIdentityManagementTeeIdentityContextIdentityStatus: {
        _enum: string[];
    };
    /**
     * Lookup95: pallet_identity_management_tee::pallet::Error<T>
     **/
    PalletIdentityManagementTeeError: {
        _enum: string[];
    };
    /**
     * Lookup97: sp_runtime::MultiSignature
     **/
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: string;
            Sr25519: string;
            Ecdsa: string;
        };
    };
    /**
     * Lookup98: sp_core::ed25519::Signature
     **/
    SpCoreEd25519Signature: string;
    /**
     * Lookup100: sp_core::sr25519::Signature
     **/
    SpCoreSr25519Signature: string;
    /**
     * Lookup101: sp_core::ecdsa::Signature
     **/
    SpCoreEcdsaSignature: string;
    /**
     * Lookup104: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
     **/
    FrameSystemExtensionsCheckNonZeroSender: string;
    /**
     * Lookup105: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
     **/
    FrameSystemExtensionsCheckSpecVersion: string;
    /**
     * Lookup106: frame_system::extensions::check_tx_version::CheckTxVersion<T>
     **/
    FrameSystemExtensionsCheckTxVersion: string;
    /**
     * Lookup107: frame_system::extensions::check_genesis::CheckGenesis<T>
     **/
    FrameSystemExtensionsCheckGenesis: string;
    /**
     * Lookup110: frame_system::extensions::check_nonce::CheckNonce<T>
     **/
    FrameSystemExtensionsCheckNonce: string;
    /**
     * Lookup111: frame_system::extensions::check_weight::CheckWeight<T>
     **/
    FrameSystemExtensionsCheckWeight: string;
    /**
     * Lookup112: pallet_transaction_payment::ChargeTransactionPayment<T>
     **/
    PalletTransactionPaymentChargeTransactionPayment: string;
    /**
     * Lookup113: ita_sgx_runtime::Runtime
     **/
    ItaSgxRuntimeRuntime: string;
};
export default _default;
//# sourceMappingURL=lookup.d.ts.map