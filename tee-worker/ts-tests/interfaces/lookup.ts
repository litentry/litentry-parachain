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
    data: 'PalletBalancesAccountData'
  },
  /**
   * Lookup5: pallet_balances::AccountData<Balance>
   **/
  PalletBalancesAccountData: {
    free: 'u128',
    reserved: 'u128',
    miscFrozen: 'u128',
    feeFrozen: 'u128'
  },
  /**
   * Lookup7: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
   **/
  FrameSupportDispatchPerDispatchClassWeight: {
    normal: 'SpWeightsWeightV2Weight',
    operational: 'SpWeightsWeightV2Weight',
    mandatory: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup8: sp_weights::weight_v2::Weight
   **/
  SpWeightsWeightV2Weight: {
    refTime: 'Compact<u64>',
    proofSize: 'Compact<u64>'
  },
  /**
   * Lookup13: sp_runtime::generic::digest::Digest
   **/
  SpRuntimeDigest: {
    logs: 'Vec<SpRuntimeDigestDigestItem>'
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
      RuntimeEnvironmentUpdated: 'Null'
    }
  },
  /**
   * Lookup18: frame_system::EventRecord<integritee_node_runtime::RuntimeEvent, primitive_types::H256>
   **/
  FrameSystemEventRecord: {
    phase: 'FrameSystemPhase',
    event: 'Event',
    topics: 'Vec<H256>'
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
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup21: frame_support::dispatch::DispatchInfo
   **/
  FrameSupportDispatchDispatchInfo: {
    weight: 'SpWeightsWeightV2Weight',
    class: 'FrameSupportDispatchDispatchClass',
    paysFee: 'FrameSupportDispatchPays'
  },
  /**
   * Lookup22: frame_support::dispatch::DispatchClass
   **/
  FrameSupportDispatchDispatchClass: {
    _enum: ['Normal', 'Operational', 'Mandatory']
  },
  /**
   * Lookup23: frame_support::dispatch::Pays
   **/
  FrameSupportDispatchPays: {
    _enum: ['Yes', 'No']
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
      Unavailable: 'Null'
    }
  },
  /**
   * Lookup25: sp_runtime::ModuleError
   **/
  SpRuntimeModuleError: {
    index: 'u8',
    error: '[u8;4]'
  },
  /**
   * Lookup26: sp_runtime::TokenError
   **/
  SpRuntimeTokenError: {
    _enum: ['NoFunds', 'WouldDie', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported']
  },
  /**
   * Lookup27: sp_arithmetic::ArithmeticError
   **/
  SpArithmeticArithmeticError: {
    _enum: ['Underflow', 'Overflow', 'DivisionByZero']
  },
  /**
   * Lookup28: sp_runtime::TransactionalError
   **/
  SpRuntimeTransactionalError: {
    _enum: ['LimitReached', 'NoLayer']
  },
  /**
   * Lookup29: pallet_preimage::pallet::Event<T>
   **/
  PalletPreimageEvent: {
    _enum: {
      Noted: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      Requested: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      Cleared: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup30: pallet_sudo::pallet::Event<T>
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
        sudoResult: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup34: pallet_multisig::pallet::Event<T>
   **/
  PalletMultisigEvent: {
    _enum: {
      NewMultisig: {
        approving: 'AccountId32',
        multisig: 'AccountId32',
        callHash: '[u8;32]',
      },
      MultisigApproval: {
        approving: 'AccountId32',
        timepoint: 'PalletMultisigTimepoint',
        multisig: 'AccountId32',
        callHash: '[u8;32]',
      },
      MultisigExecuted: {
        approving: 'AccountId32',
        timepoint: 'PalletMultisigTimepoint',
        multisig: 'AccountId32',
        callHash: '[u8;32]',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      MultisigCancelled: {
        cancelling: 'AccountId32',
        timepoint: 'PalletMultisigTimepoint',
        multisig: 'AccountId32',
        callHash: '[u8;32]'
      }
    }
  },
  /**
   * Lookup35: pallet_multisig::Timepoint<BlockNumber>
   **/
  PalletMultisigTimepoint: {
    height: 'u32',
    index: 'u32'
  },
  /**
   * Lookup36: pallet_proxy::pallet::Event<T>
   **/
  PalletProxyEvent: {
    _enum: {
      ProxyExecuted: {
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      PureCreated: {
        pure: 'AccountId32',
        who: 'AccountId32',
        proxyType: 'IntegriteeNodeRuntimeProxyType',
        disambiguationIndex: 'u16',
      },
      Announced: {
        real: 'AccountId32',
        proxy: 'AccountId32',
        callHash: 'H256',
      },
      ProxyAdded: {
        delegator: 'AccountId32',
        delegatee: 'AccountId32',
        proxyType: 'IntegriteeNodeRuntimeProxyType',
        delay: 'u32',
      },
      ProxyRemoved: {
        delegator: 'AccountId32',
        delegatee: 'AccountId32',
        proxyType: 'IntegriteeNodeRuntimeProxyType',
        delay: 'u32'
      }
    }
  },
  /**
   * Lookup37: integritee_node_runtime::ProxyType
   **/
  IntegriteeNodeRuntimeProxyType: {
    _enum: ['Any', 'NonTransfer', 'Governance', 'CancelProxy']
  },
  /**
   * Lookup39: pallet_scheduler::pallet::Event<T>
   **/
  PalletSchedulerEvent: {
    _enum: {
      Scheduled: {
        when: 'u32',
        index: 'u32',
      },
      Canceled: {
        when: 'u32',
        index: 'u32',
      },
      Dispatched: {
        task: '(u32,u32)',
        id: 'Option<[u8;32]>',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      CallUnavailable: {
        task: '(u32,u32)',
        id: 'Option<[u8;32]>',
      },
      PeriodicFailed: {
        task: '(u32,u32)',
        id: 'Option<[u8;32]>',
      },
      PermanentlyOverweight: {
        task: '(u32,u32)',
        id: 'Option<[u8;32]>'
      }
    }
  },
  /**
   * Lookup42: pallet_utility::pallet::Event
   **/
  PalletUtilityEvent: {
    _enum: {
      BatchInterrupted: {
        index: 'u32',
        error: 'SpRuntimeDispatchError',
      },
      BatchCompleted: 'Null',
      BatchCompletedWithErrors: 'Null',
      ItemCompleted: 'Null',
      ItemFailed: {
        error: 'SpRuntimeDispatchError',
      },
      DispatchedAs: {
        result: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup43: pallet_balances::pallet::Event<T, I>
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
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup44: frame_support::traits::tokens::misc::BalanceStatus
   **/
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ['Free', 'Reserved']
  },
  /**
   * Lookup45: pallet_transaction_payment::pallet::Event<T>
   **/
  PalletTransactionPaymentEvent: {
    _enum: {
      TransactionFeePaid: {
        who: 'AccountId32',
        actualFee: 'u128',
        tip: 'u128'
      }
    }
  },
  /**
   * Lookup46: pallet_vesting::pallet::Event<T>
   **/
  PalletVestingEvent: {
    _enum: {
      VestingUpdated: {
        account: 'AccountId32',
        unvested: 'u128',
      },
      VestingCompleted: {
        account: 'AccountId32'
      }
    }
  },
  /**
   * Lookup47: pallet_grandpa::pallet::Event
   **/
  PalletGrandpaEvent: {
    _enum: {
      NewAuthorities: {
        authoritySet: 'Vec<(SpFinalityGrandpaAppPublic,u64)>',
      },
      Paused: 'Null',
      Resumed: 'Null'
    }
  },
  /**
   * Lookup50: sp_finality_grandpa::app::Public
   **/
  SpFinalityGrandpaAppPublic: 'SpCoreEd25519Public',
  /**
   * Lookup51: sp_core::ed25519::Public
   **/
  SpCoreEd25519Public: '[u8;32]',
  /**
   * Lookup52: pallet_collective::pallet::Event<T, I>
   **/
  PalletCollectiveEvent: {
    _enum: {
      Proposed: {
        account: 'AccountId32',
        proposalIndex: 'u32',
        proposalHash: 'H256',
        threshold: 'u32',
      },
      Voted: {
        account: 'AccountId32',
        proposalHash: 'H256',
        voted: 'bool',
        yes: 'u32',
        no: 'u32',
      },
      Approved: {
        proposalHash: 'H256',
      },
      Disapproved: {
        proposalHash: 'H256',
      },
      Executed: {
        proposalHash: 'H256',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      MemberExecuted: {
        proposalHash: 'H256',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      Closed: {
        proposalHash: 'H256',
        yes: 'u32',
        no: 'u32'
      }
    }
  },
  /**
   * Lookup54: pallet_treasury::pallet::Event<T, I>
   **/
  PalletTreasuryEvent: {
    _enum: {
      Proposed: {
        proposalIndex: 'u32',
      },
      Spending: {
        budgetRemaining: 'u128',
      },
      Awarded: {
        proposalIndex: 'u32',
        award: 'u128',
        account: 'AccountId32',
      },
      Rejected: {
        proposalIndex: 'u32',
        slashed: 'u128',
      },
      Burnt: {
        burntFunds: 'u128',
      },
      Rollover: {
        rolloverBalance: 'u128',
      },
      Deposit: {
        value: 'u128',
      },
      SpendApproved: {
        proposalIndex: 'u32',
        amount: 'u128',
        beneficiary: 'AccountId32',
      },
      UpdatedInactive: {
        reactivated: 'u128',
        deactivated: 'u128'
      }
    }
  },
  /**
   * Lookup55: pallet_teerex::pallet::Event<T>
   **/
  PalletTeerexEvent: {
    _enum: {
      AdminChanged: {
        oldAdmin: 'Option<AccountId32>',
      },
      AddedEnclave: '(AccountId32,Bytes)',
      RemovedEnclave: 'AccountId32',
      Forwarded: 'H256',
      ShieldFunds: 'Bytes',
      UnshieldedFunds: 'AccountId32',
      ProcessedParentchainBlock: '(AccountId32,H256,H256,u32)',
      SetHeartbeatTimeout: 'u64',
      UpdatedScheduledEnclave: '(u64,[u8;32])',
      RemovedScheduledEnclave: 'u64',
      PublishedHash: {
        _alias: {
          hash_: 'hash',
        },
        mrEnclave: '[u8;32]',
        hash_: 'H256',
        data: 'Bytes'
      }
    }
  },
  /**
   * Lookup56: pallet_claims::pallet::Event<T>
   **/
  PalletClaimsEvent: {
    _enum: {
      Claimed: '(AccountId32,ClaimsPrimitivesEthereumAddress,u128)'
    }
  },
  /**
   * Lookup57: claims_primitives::EthereumAddress
   **/
  ClaimsPrimitivesEthereumAddress: '[u8;20]',
  /**
   * Lookup59: pallet_teeracle::pallet::Event<T>
   **/
  PalletTeeracleEvent: {
    _enum: {
      ExchangeRateUpdated: '(Text,Text,Option<SubstrateFixedFixedU64>)',
      ExchangeRateDeleted: '(Text,Text)',
      OracleUpdated: '(Text,Text)',
      AddedToWhitelist: '(Text,[u8;32])',
      RemovedFromWhitelist: '(Text,[u8;32])'
    }
  },
  /**
   * Lookup62: substrate_fixed::FixedU64<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>>
   **/
  SubstrateFixedFixedU64: {
    bits: 'u64'
  },
  /**
   * Lookup67: typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>
   **/
  TypenumUIntUInt: {
    msb: 'TypenumUIntUTerm',
    lsb: 'TypenumBitB0'
  },
  /**
   * Lookup68: typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>
   **/
  TypenumUIntUTerm: {
    msb: 'TypenumUintUTerm',
    lsb: 'TypenumBitB1'
  },
  /**
   * Lookup69: typenum::uint::UTerm
   **/
  TypenumUintUTerm: 'Null',
  /**
   * Lookup70: typenum::bit::B1
   **/
  TypenumBitB1: 'Null',
  /**
   * Lookup71: typenum::bit::B0
   **/
  TypenumBitB0: 'Null',
  /**
   * Lookup72: pallet_sidechain::pallet::Event<T>
   **/
  PalletSidechainEvent: {
    _enum: {
      ProposedSidechainBlock: '(AccountId32,H256)',
      FinalizedSidechainBlock: '(AccountId32,H256)'
    }
  },
  /**
   * Lookup73: pallet_identity_management::pallet::Event<T>
   **/
  PalletIdentityManagementEvent: {
    _enum: {
      DelegateeAdded: {
        account: 'AccountId32',
      },
      DelegateeRemoved: {
        account: 'AccountId32',
      },
      CreateIdentityRequested: {
        shard: 'H256',
      },
      RemoveIdentityRequested: {
        shard: 'H256',
      },
      VerifyIdentityRequested: {
        shard: 'H256',
      },
      SetUserShieldingKeyRequested: {
        shard: 'H256',
      },
      UserShieldingKeySet: {
        account: 'AccountId32',
        idGraph: 'CorePrimitivesKeyAesOutput',
        reqExtHash: 'H256',
      },
      IdentityCreated: {
        account: 'AccountId32',
        identity: 'CorePrimitivesKeyAesOutput',
        code: 'CorePrimitivesKeyAesOutput',
        reqExtHash: 'H256',
      },
      IdentityRemoved: {
        account: 'AccountId32',
        identity: 'CorePrimitivesKeyAesOutput',
        reqExtHash: 'H256',
      },
      IdentityVerified: {
        account: 'AccountId32',
        identity: 'CorePrimitivesKeyAesOutput',
        idGraph: 'CorePrimitivesKeyAesOutput',
        reqExtHash: 'H256',
      },
      SetUserShieldingKeyFailed: {
        account: 'Option<AccountId32>',
        detail: 'CorePrimitivesErrorErrorDetail',
        reqExtHash: 'H256',
      },
      CreateIdentityFailed: {
        account: 'Option<AccountId32>',
        detail: 'CorePrimitivesErrorErrorDetail',
        reqExtHash: 'H256',
      },
      RemoveIdentityFailed: {
        account: 'Option<AccountId32>',
        detail: 'CorePrimitivesErrorErrorDetail',
        reqExtHash: 'H256',
      },
      VerifyIdentityFailed: {
        account: 'Option<AccountId32>',
        detail: 'CorePrimitivesErrorErrorDetail',
        reqExtHash: 'H256',
      },
      ImportScheduledEnclaveFailed: 'Null',
      UnclassifiedError: {
        account: 'Option<AccountId32>',
        detail: 'CorePrimitivesErrorErrorDetail',
        reqExtHash: 'H256'
      }
    }
  },
  /**
   * Lookup74: core_primitives::key::AesOutput
   **/
  CorePrimitivesKeyAesOutput: {
    ciphertext: 'Bytes',
    aad: 'Bytes',
    nonce: '[u8;12]'
  },
  /**
   * Lookup76: core_primitives::error::ErrorDetail
   **/
  CorePrimitivesErrorErrorDetail: {
    _enum: {
      ImportError: 'Null',
      StfError: 'Bytes',
      SendStfRequestFailed: 'Null',
      ChallengeCodeNotFound: 'Null',
      UserShieldingKeyNotFound: 'Null',
      ParseError: 'Null',
      DataProviderError: 'Bytes',
      InvalidIdentity: 'Null',
      WrongWeb2Handle: 'Null',
      UnexpectedMessage: 'Null',
      WrongSignatureType: 'Null',
      VerifySubstrateSignatureFailed: 'Null',
      VerifyEvmSignatureFailed: 'Null',
      RecoverEvmAddressFailed: 'Null'
    }
  },
  /**
   * Lookup78: pallet_vc_management::pallet::Event<T>
   **/
  PalletVcManagementEvent: {
    _enum: {
      VCRequested: {
        account: 'AccountId32',
        shard: 'H256',
        assertion: 'CorePrimitivesAssertion',
      },
      VCDisabled: {
        account: 'AccountId32',
        index: 'H256',
      },
      VCRevoked: {
        account: 'AccountId32',
        index: 'H256',
      },
      VCIssued: {
        account: 'AccountId32',
        assertion: 'CorePrimitivesAssertion',
        index: 'H256',
        vc: 'CorePrimitivesKeyAesOutput',
        reqExtHash: 'H256',
      },
      AdminChanged: {
        oldAdmin: 'Option<AccountId32>',
        newAdmin: 'Option<AccountId32>',
      },
      SchemaIssued: {
        account: 'AccountId32',
        shard: 'H256',
        index: 'u64',
      },
      SchemaDisabled: {
        account: 'AccountId32',
        shard: 'H256',
        index: 'u64',
      },
      SchemaActivated: {
        account: 'AccountId32',
        shard: 'H256',
        index: 'u64',
      },
      SchemaRevoked: {
        account: 'AccountId32',
        shard: 'H256',
        index: 'u64',
      },
      RequestVCFailed: {
        account: 'Option<AccountId32>',
        assertion: 'CorePrimitivesAssertion',
        detail: 'CorePrimitivesErrorErrorDetail',
        reqExtHash: 'H256',
      },
      UnclassifiedError: {
        account: 'Option<AccountId32>',
        detail: 'CorePrimitivesErrorErrorDetail',
        reqExtHash: 'H256',
      },
      VCRegistryItemAdded: {
        account: 'AccountId32',
        assertion: 'CorePrimitivesAssertion',
        index: 'H256',
      },
      VCRegistryItemRemoved: {
        index: 'H256',
      },
      VCRegistryCleared: 'Null'
    }
  },
  /**
   * Lookup79: core_primitives::assertion::Assertion
   **/
  CorePrimitivesAssertion: {
    _enum: {
      A1: 'Null',
      A2: 'Bytes',
      A3: '(Bytes,Bytes,Bytes)',
      A4: 'Bytes',
      A5: 'Bytes',
      A6: 'Null',
      A7: 'Bytes',
      A8: 'Vec<CorePrimitivesAssertionIndexingNetwork>',
      A9: 'Null',
      A10: 'Bytes',
      A11: 'Bytes',
      A13: 'u32'
    }
  },
  /**
   * Lookup82: core_primitives::assertion::IndexingNetwork
   **/
  CorePrimitivesAssertionIndexingNetwork: {
    _enum: ['Litentry', 'Litmus', 'Polkadot', 'Kusama', 'Khala', 'Ethereum']
  },
  /**
   * Lookup84: pallet_group::pallet::Event<T, I>
   **/
  PalletGroupEvent: {
    _enum: {
      GroupMemberAdded: 'AccountId32',
      GroupMemberRemoved: 'AccountId32'
    }
  },
  /**
   * Lookup86: frame_system::Phase
   **/
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: 'u32',
      Finalization: 'Null',
      Initialization: 'Null'
    }
  },
  /**
   * Lookup89: frame_system::LastRuntimeUpgradeInfo
   **/
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: 'Compact<u32>',
    specName: 'Text'
  },
  /**
   * Lookup91: frame_system::pallet::Call<T>
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
        remark: 'Bytes'
      }
    }
  },
  /**
   * Lookup95: frame_system::limits::BlockWeights
   **/
  FrameSystemLimitsBlockWeights: {
    baseBlock: 'SpWeightsWeightV2Weight',
    maxBlock: 'SpWeightsWeightV2Weight',
    perClass: 'FrameSupportDispatchPerDispatchClassWeightsPerClass'
  },
  /**
   * Lookup96: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
   **/
  FrameSupportDispatchPerDispatchClassWeightsPerClass: {
    normal: 'FrameSystemLimitsWeightsPerClass',
    operational: 'FrameSystemLimitsWeightsPerClass',
    mandatory: 'FrameSystemLimitsWeightsPerClass'
  },
  /**
   * Lookup97: frame_system::limits::WeightsPerClass
   **/
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: 'SpWeightsWeightV2Weight',
    maxExtrinsic: 'Option<SpWeightsWeightV2Weight>',
    maxTotal: 'Option<SpWeightsWeightV2Weight>',
    reserved: 'Option<SpWeightsWeightV2Weight>'
  },
  /**
   * Lookup99: frame_system::limits::BlockLength
   **/
  FrameSystemLimitsBlockLength: {
    max: 'FrameSupportDispatchPerDispatchClassU32'
  },
  /**
   * Lookup100: frame_support::dispatch::PerDispatchClass<T>
   **/
  FrameSupportDispatchPerDispatchClassU32: {
    normal: 'u32',
    operational: 'u32',
    mandatory: 'u32'
  },
  /**
   * Lookup101: sp_weights::RuntimeDbWeight
   **/
  SpWeightsRuntimeDbWeight: {
    read: 'u64',
    write: 'u64'
  },
  /**
   * Lookup102: sp_version::RuntimeVersion
   **/
  SpVersionRuntimeVersion: {
    specName: 'Text',
    implName: 'Text',
    authoringVersion: 'u32',
    specVersion: 'u32',
    implVersion: 'u32',
    apis: 'Vec<([u8;8],u32)>',
    transactionVersion: 'u32',
    stateVersion: 'u8'
  },
  /**
   * Lookup107: frame_system::pallet::Error<T>
   **/
  FrameSystemError: {
    _enum: ['InvalidSpecName', 'SpecVersionNeedsToIncrease', 'FailedToExtractRuntimeVersion', 'NonDefaultComposite', 'NonZeroRefCount', 'CallFiltered']
  },
  /**
   * Lookup108: pallet_preimage::RequestStatus<sp_core::crypto::AccountId32, Balance>
   **/
  PalletPreimageRequestStatus: {
    _enum: {
      Unrequested: {
        deposit: '(AccountId32,u128)',
        len: 'u32',
      },
      Requested: {
        deposit: 'Option<(AccountId32,u128)>',
        count: 'u32',
        len: 'Option<u32>'
      }
    }
  },
  /**
   * Lookup114: pallet_preimage::pallet::Call<T>
   **/
  PalletPreimageCall: {
    _enum: {
      note_preimage: {
        bytes: 'Bytes',
      },
      unnote_preimage: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      request_preimage: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      unrequest_preimage: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup115: pallet_preimage::pallet::Error<T>
   **/
  PalletPreimageError: {
    _enum: ['TooBig', 'AlreadyNoted', 'NotAuthorized', 'NotNoted', 'Requested', 'NotRequested']
  },
  /**
   * Lookup117: pallet_timestamp::pallet::Call<T>
   **/
  PalletTimestampCall: {
    _enum: {
      set: {
        now: 'Compact<u64>'
      }
    }
  },
  /**
   * Lookup118: pallet_sudo::pallet::Call<T>
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
        call: 'Call'
      }
    }
  },
  /**
   * Lookup120: pallet_multisig::pallet::Call<T>
   **/
  PalletMultisigCall: {
    _enum: {
      as_multi_threshold_1: {
        otherSignatories: 'Vec<AccountId32>',
        call: 'Call',
      },
      as_multi: {
        threshold: 'u16',
        otherSignatories: 'Vec<AccountId32>',
        maybeTimepoint: 'Option<PalletMultisigTimepoint>',
        call: 'Call',
        maxWeight: 'SpWeightsWeightV2Weight',
      },
      approve_as_multi: {
        threshold: 'u16',
        otherSignatories: 'Vec<AccountId32>',
        maybeTimepoint: 'Option<PalletMultisigTimepoint>',
        callHash: '[u8;32]',
        maxWeight: 'SpWeightsWeightV2Weight',
      },
      cancel_as_multi: {
        threshold: 'u16',
        otherSignatories: 'Vec<AccountId32>',
        timepoint: 'PalletMultisigTimepoint',
        callHash: '[u8;32]'
      }
    }
  },
  /**
   * Lookup123: pallet_proxy::pallet::Call<T>
   **/
  PalletProxyCall: {
    _enum: {
      proxy: {
        real: 'MultiAddress',
        forceProxyType: 'Option<IntegriteeNodeRuntimeProxyType>',
        call: 'Call',
      },
      add_proxy: {
        delegate: 'MultiAddress',
        proxyType: 'IntegriteeNodeRuntimeProxyType',
        delay: 'u32',
      },
      remove_proxy: {
        delegate: 'MultiAddress',
        proxyType: 'IntegriteeNodeRuntimeProxyType',
        delay: 'u32',
      },
      remove_proxies: 'Null',
      create_pure: {
        proxyType: 'IntegriteeNodeRuntimeProxyType',
        delay: 'u32',
        index: 'u16',
      },
      kill_pure: {
        spawner: 'MultiAddress',
        proxyType: 'IntegriteeNodeRuntimeProxyType',
        index: 'u16',
        height: 'Compact<u32>',
        extIndex: 'Compact<u32>',
      },
      announce: {
        real: 'MultiAddress',
        callHash: 'H256',
      },
      remove_announcement: {
        real: 'MultiAddress',
        callHash: 'H256',
      },
      reject_announcement: {
        delegate: 'MultiAddress',
        callHash: 'H256',
      },
      proxy_announced: {
        delegate: 'MultiAddress',
        real: 'MultiAddress',
        forceProxyType: 'Option<IntegriteeNodeRuntimeProxyType>',
        call: 'Call'
      }
    }
  },
  /**
   * Lookup127: pallet_scheduler::pallet::Call<T>
   **/
  PalletSchedulerCall: {
    _enum: {
      schedule: {
        when: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      cancel: {
        when: 'u32',
        index: 'u32',
      },
      schedule_named: {
        id: '[u8;32]',
        when: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      cancel_named: {
        id: '[u8;32]',
      },
      schedule_after: {
        after: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      schedule_named_after: {
        id: '[u8;32]',
        after: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'Call'
      }
    }
  },
  /**
   * Lookup129: pallet_utility::pallet::Call<T>
   **/
  PalletUtilityCall: {
    _enum: {
      batch: {
        calls: 'Vec<Call>',
      },
      as_derivative: {
        index: 'u16',
        call: 'Call',
      },
      batch_all: {
        calls: 'Vec<Call>',
      },
      dispatch_as: {
        asOrigin: 'IntegriteeNodeRuntimeOriginCaller',
        call: 'Call',
      },
      force_batch: {
        calls: 'Vec<Call>',
      },
      with_weight: {
        call: 'Call',
        weight: 'SpWeightsWeightV2Weight'
      }
    }
  },
  /**
   * Lookup131: integritee_node_runtime::OriginCaller
   **/
  IntegriteeNodeRuntimeOriginCaller: {
    _enum: {
      system: 'FrameSupportDispatchRawOrigin',
      __Unused1: 'Null',
      Void: 'SpCoreVoid',
      __Unused3: 'Null',
      __Unused4: 'Null',
      __Unused5: 'Null',
      __Unused6: 'Null',
      __Unused7: 'Null',
      __Unused8: 'Null',
      __Unused9: 'Null',
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
      __Unused20: 'Null',
      __Unused21: 'Null',
      Council: 'PalletCollectiveRawOrigin'
    }
  },
  /**
   * Lookup132: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
   **/
  FrameSupportDispatchRawOrigin: {
    _enum: {
      Root: 'Null',
      Signed: 'AccountId32',
      None: 'Null'
    }
  },
  /**
   * Lookup133: pallet_collective::RawOrigin<sp_core::crypto::AccountId32, I>
   **/
  PalletCollectiveRawOrigin: {
    _enum: {
      Members: '(u32,u32)',
      Member: 'AccountId32',
      _Phantom: 'Null'
    }
  },
  /**
   * Lookup134: sp_core::Void
   **/
  SpCoreVoid: 'Null',
  /**
   * Lookup135: pallet_balances::pallet::Call<T, I>
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
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup137: pallet_vesting::pallet::Call<T>
   **/
  PalletVestingCall: {
    _enum: {
      vest: 'Null',
      vest_other: {
        target: 'MultiAddress',
      },
      vested_transfer: {
        target: 'MultiAddress',
        schedule: 'PalletVestingVestingInfo',
      },
      force_vested_transfer: {
        source: 'MultiAddress',
        target: 'MultiAddress',
        schedule: 'PalletVestingVestingInfo',
      },
      merge_schedules: {
        schedule1Index: 'u32',
        schedule2Index: 'u32'
      }
    }
  },
  /**
   * Lookup138: pallet_vesting::vesting_info::VestingInfo<Balance, BlockNumber>
   **/
  PalletVestingVestingInfo: {
    locked: 'u128',
    perBlock: 'u128',
    startingBlock: 'u32'
  },
  /**
   * Lookup139: pallet_grandpa::pallet::Call<T>
   **/
  PalletGrandpaCall: {
    _enum: {
      report_equivocation: {
        equivocationProof: 'SpFinalityGrandpaEquivocationProof',
        keyOwnerProof: 'SpCoreVoid',
      },
      report_equivocation_unsigned: {
        equivocationProof: 'SpFinalityGrandpaEquivocationProof',
        keyOwnerProof: 'SpCoreVoid',
      },
      note_stalled: {
        delay: 'u32',
        bestFinalizedBlockNumber: 'u32'
      }
    }
  },
  /**
   * Lookup140: sp_finality_grandpa::EquivocationProof<primitive_types::H256, N>
   **/
  SpFinalityGrandpaEquivocationProof: {
    setId: 'u64',
    equivocation: 'SpFinalityGrandpaEquivocation'
  },
  /**
   * Lookup141: sp_finality_grandpa::Equivocation<primitive_types::H256, N>
   **/
  SpFinalityGrandpaEquivocation: {
    _enum: {
      Prevote: 'FinalityGrandpaEquivocationPrevote',
      Precommit: 'FinalityGrandpaEquivocationPrecommit'
    }
  },
  /**
   * Lookup142: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public, finality_grandpa::Prevote<primitive_types::H256, N>, sp_finality_grandpa::app::Signature>
   **/
  FinalityGrandpaEquivocationPrevote: {
    roundNumber: 'u64',
    identity: 'SpFinalityGrandpaAppPublic',
    first: '(FinalityGrandpaPrevote,SpFinalityGrandpaAppSignature)',
    second: '(FinalityGrandpaPrevote,SpFinalityGrandpaAppSignature)'
  },
  /**
   * Lookup143: finality_grandpa::Prevote<primitive_types::H256, N>
   **/
  FinalityGrandpaPrevote: {
    targetHash: 'H256',
    targetNumber: 'u32'
  },
  /**
   * Lookup144: sp_finality_grandpa::app::Signature
   **/
  SpFinalityGrandpaAppSignature: 'SpCoreEd25519Signature',
  /**
   * Lookup145: sp_core::ed25519::Signature
   **/
  SpCoreEd25519Signature: '[u8;64]',
  /**
   * Lookup148: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public, finality_grandpa::Precommit<primitive_types::H256, N>, sp_finality_grandpa::app::Signature>
   **/
  FinalityGrandpaEquivocationPrecommit: {
    roundNumber: 'u64',
    identity: 'SpFinalityGrandpaAppPublic',
    first: '(FinalityGrandpaPrecommit,SpFinalityGrandpaAppSignature)',
    second: '(FinalityGrandpaPrecommit,SpFinalityGrandpaAppSignature)'
  },
  /**
   * Lookup149: finality_grandpa::Precommit<primitive_types::H256, N>
   **/
  FinalityGrandpaPrecommit: {
    targetHash: 'H256',
    targetNumber: 'u32'
  },
  /**
   * Lookup151: pallet_collective::pallet::Call<T, I>
   **/
  PalletCollectiveCall: {
    _enum: {
      set_members: {
        newMembers: 'Vec<AccountId32>',
        prime: 'Option<AccountId32>',
        oldCount: 'u32',
      },
      execute: {
        proposal: 'Call',
        lengthBound: 'Compact<u32>',
      },
      propose: {
        threshold: 'Compact<u32>',
        proposal: 'Call',
        lengthBound: 'Compact<u32>',
      },
      vote: {
        proposal: 'H256',
        index: 'Compact<u32>',
        approve: 'bool',
      },
      close_old_weight: {
        proposalHash: 'H256',
        index: 'Compact<u32>',
        proposalWeightBound: 'Compact<u64>',
        lengthBound: 'Compact<u32>',
      },
      disapprove_proposal: {
        proposalHash: 'H256',
      },
      close: {
        proposalHash: 'H256',
        index: 'Compact<u32>',
        proposalWeightBound: 'SpWeightsWeightV2Weight',
        lengthBound: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup154: pallet_treasury::pallet::Call<T, I>
   **/
  PalletTreasuryCall: {
    _enum: {
      propose_spend: {
        value: 'Compact<u128>',
        beneficiary: 'MultiAddress',
      },
      reject_proposal: {
        proposalId: 'Compact<u32>',
      },
      approve_proposal: {
        proposalId: 'Compact<u32>',
      },
      spend: {
        amount: 'Compact<u128>',
        beneficiary: 'MultiAddress',
      },
      remove_approval: {
        proposalId: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup155: pallet_teerex::pallet::Call<T>
   **/
  PalletTeerexCall: {
    _enum: {
      register_enclave: {
        raReport: 'Bytes',
        workerUrl: 'Bytes',
        shieldingKey: 'Option<Bytes>',
        vcPubkey: 'Option<Bytes>',
      },
      unregister_enclave: 'Null',
      call_worker: {
        request: 'TeerexPrimitivesRequest',
      },
      confirm_processed_parentchain_block: {
        blockHash: 'H256',
        blockNumber: 'u32',
        trustedCallsMerkleRoot: 'H256',
      },
      shield_funds: {
        incognitoAccountEncrypted: 'Bytes',
        amount: 'u128',
        bondingAccount: 'AccountId32',
      },
      unshield_funds: {
        publicAccount: 'AccountId32',
        amount: 'u128',
        bondingAccount: 'AccountId32',
        callHash: 'H256',
      },
      set_heartbeat_timeout: {
        timeout: 'u64',
      },
      register_dcap_enclave: {
        dcapQuote: 'Bytes',
        workerUrl: 'Bytes',
      },
      update_scheduled_enclave: {
        sidechainBlockNumber: 'u64',
        mrEnclave: '[u8;32]',
      },
      register_quoting_enclave: {
        enclaveIdentity: 'Bytes',
        signature: 'Bytes',
        certificateChain: 'Bytes',
      },
      remove_scheduled_enclave: {
        sidechainBlockNumber: 'u64',
      },
      register_tcb_info: {
        tcbInfo: 'Bytes',
        signature: 'Bytes',
        certificateChain: 'Bytes',
      },
      publish_hash: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
        extraTopics: 'Vec<H256>',
        data: 'Bytes',
      },
      set_admin: {
        _alias: {
          new_: 'new',
        },
        new_: 'AccountId32'
      }
    }
  },
  /**
   * Lookup157: teerex_primitives::Request
   **/
  TeerexPrimitivesRequest: {
    shard: 'H256',
    cyphertext: 'Bytes'
  },
  /**
   * Lookup158: pallet_claims::pallet::Call<T>
   **/
  PalletClaimsCall: {
    _enum: {
      claim: {
        dest: 'AccountId32',
        ethereumSignature: 'ClaimsPrimitivesEcdsaSignature',
      },
      mint_claim: {
        who: 'ClaimsPrimitivesEthereumAddress',
        value: 'u128',
        vestingSchedule: 'Option<(u128,u128,u32)>',
        statement: 'Option<ClaimsPrimitivesStatementKind>',
      },
      claim_attest: {
        dest: 'AccountId32',
        ethereumSignature: 'ClaimsPrimitivesEcdsaSignature',
        statement: 'Bytes',
      },
      attest: {
        statement: 'Bytes',
      },
      move_claim: {
        _alias: {
          new_: 'new',
        },
        old: 'ClaimsPrimitivesEthereumAddress',
        new_: 'ClaimsPrimitivesEthereumAddress',
        maybePreclaim: 'Option<AccountId32>'
      }
    }
  },
  /**
   * Lookup159: claims_primitives::EcdsaSignature
   **/
  ClaimsPrimitivesEcdsaSignature: '[u8;65]',
  /**
   * Lookup164: claims_primitives::StatementKind
   **/
  ClaimsPrimitivesStatementKind: {
    _enum: ['Regular', 'Saft']
  },
  /**
   * Lookup165: pallet_teeracle::pallet::Call<T>
   **/
  PalletTeeracleCall: {
    _enum: {
      add_to_whitelist: {
        dataSource: 'Text',
        mrenclave: '[u8;32]',
      },
      remove_from_whitelist: {
        dataSource: 'Text',
        mrenclave: '[u8;32]',
      },
      update_oracle: {
        oracleName: 'Text',
        dataSource: 'Text',
        newBlob: 'Bytes',
      },
      update_exchange_rate: {
        dataSource: 'Text',
        tradingPair: 'Text',
        newValue: 'Option<SubstrateFixedFixedU64>'
      }
    }
  },
  /**
   * Lookup167: pallet_sidechain::pallet::Call<T>
   **/
  PalletSidechainCall: {
    _enum: {
      confirm_imported_sidechain_block: {
        shardId: 'H256',
        blockNumber: 'u64',
        nextFinalizationCandidateBlockNumber: 'u64',
        blockHeaderHash: 'H256'
      }
    }
  },
  /**
   * Lookup168: pallet_identity_management::pallet::Call<T>
   **/
  PalletIdentityManagementCall: {
    _enum: {
      add_delegatee: {
        account: 'AccountId32',
      },
      remove_delegatee: {
        account: 'AccountId32',
      },
      set_user_shielding_key: {
        shard: 'H256',
        encryptedKey: 'Bytes',
      },
      create_identity: {
        shard: 'H256',
        user: 'AccountId32',
        encryptedIdentity: 'Bytes',
        encryptedMetadata: 'Option<Bytes>',
      },
      remove_identity: {
        shard: 'H256',
        encryptedIdentity: 'Bytes',
      },
      verify_identity: {
        shard: 'H256',
        encryptedIdentity: 'Bytes',
        encryptedValidationData: 'Bytes',
      },
      __Unused6: 'Null',
      __Unused7: 'Null',
      __Unused8: 'Null',
      __Unused9: 'Null',
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
      __Unused20: 'Null',
      __Unused21: 'Null',
      __Unused22: 'Null',
      __Unused23: 'Null',
      __Unused24: 'Null',
      __Unused25: 'Null',
      __Unused26: 'Null',
      __Unused27: 'Null',
      __Unused28: 'Null',
      __Unused29: 'Null',
      user_shielding_key_set: {
        account: 'AccountId32',
        idGraph: 'CorePrimitivesKeyAesOutput',
        reqExtHash: 'H256',
      },
      identity_created: {
        account: 'AccountId32',
        identity: 'CorePrimitivesKeyAesOutput',
        code: 'CorePrimitivesKeyAesOutput',
        reqExtHash: 'H256',
      },
      identity_removed: {
        account: 'AccountId32',
        identity: 'CorePrimitivesKeyAesOutput',
        reqExtHash: 'H256',
      },
      identity_verified: {
        account: 'AccountId32',
        identity: 'CorePrimitivesKeyAesOutput',
        idGraph: 'CorePrimitivesKeyAesOutput',
        reqExtHash: 'H256',
      },
      some_error: {
        account: 'Option<AccountId32>',
        error: 'CorePrimitivesErrorImpError',
        reqExtHash: 'H256'
      }
    }
  },
  /**
   * Lookup169: core_primitives::error::IMPError
   **/
  CorePrimitivesErrorImpError: {
    _enum: {
      SetUserShieldingKeyFailed: 'CorePrimitivesErrorErrorDetail',
      CreateIdentityFailed: 'CorePrimitivesErrorErrorDetail',
      RemoveIdentityFailed: 'CorePrimitivesErrorErrorDetail',
      VerifyIdentityFailed: 'CorePrimitivesErrorErrorDetail',
      ImportScheduledEnclaveFailed: 'Null',
      UnclassifiedError: 'CorePrimitivesErrorErrorDetail'
    }
  },
  /**
   * Lookup170: pallet_vc_management::pallet::Call<T>
   **/
  PalletVcManagementCall: {
    _enum: {
      request_vc: {
        shard: 'H256',
        assertion: 'CorePrimitivesAssertion',
      },
      disable_vc: {
        index: 'H256',
      },
      revoke_vc: {
        index: 'H256',
      },
      set_admin: {
        _alias: {
          new_: 'new',
        },
        new_: 'AccountId32',
      },
      add_schema: {
        shard: 'H256',
        id: 'Bytes',
        content: 'Bytes',
      },
      disable_schema: {
        shard: 'H256',
        index: 'u64',
      },
      activate_schema: {
        shard: 'H256',
        index: 'u64',
      },
      revoke_schema: {
        shard: 'H256',
        index: 'u64',
      },
      add_vc_registry_item: {
        _alias: {
          hash_: 'hash',
        },
        index: 'H256',
        subject: 'AccountId32',
        assertion: 'CorePrimitivesAssertion',
        hash_: 'H256',
      },
      remove_vc_registry_item: {
        index: 'H256',
      },
      clear_vc_registry: 'Null',
      __Unused11: 'Null',
      __Unused12: 'Null',
      __Unused13: 'Null',
      __Unused14: 'Null',
      __Unused15: 'Null',
      __Unused16: 'Null',
      __Unused17: 'Null',
      __Unused18: 'Null',
      __Unused19: 'Null',
      __Unused20: 'Null',
      __Unused21: 'Null',
      __Unused22: 'Null',
      __Unused23: 'Null',
      __Unused24: 'Null',
      __Unused25: 'Null',
      __Unused26: 'Null',
      __Unused27: 'Null',
      __Unused28: 'Null',
      __Unused29: 'Null',
      vc_issued: {
        _alias: {
          hash_: 'hash',
        },
        account: 'AccountId32',
        assertion: 'CorePrimitivesAssertion',
        index: 'H256',
        hash_: 'H256',
        vc: 'CorePrimitivesKeyAesOutput',
        reqExtHash: 'H256',
      },
      some_error: {
        account: 'Option<AccountId32>',
        error: 'CorePrimitivesErrorVcmpError',
        reqExtHash: 'H256'
      }
    }
  },
  /**
   * Lookup171: core_primitives::error::VCMPError
   **/
  CorePrimitivesErrorVcmpError: {
    _enum: {
      RequestVCFailed: '(CorePrimitivesAssertion,CorePrimitivesErrorErrorDetail)',
      UnclassifiedError: 'CorePrimitivesErrorErrorDetail'
    }
  },
  /**
   * Lookup172: pallet_group::pallet::Call<T, I>
   **/
  PalletGroupCall: {
    _enum: {
      add_group_member: {
        v: 'AccountId32',
      },
      batch_add_group_members: {
        vs: 'Vec<AccountId32>',
      },
      remove_group_member: {
        v: 'AccountId32',
      },
      batch_remove_group_members: {
        vs: 'Vec<AccountId32>',
      },
      switch_group_control_on: 'Null',
      switch_group_control_off: 'Null'
    }
  },
  /**
   * Lookup174: pallet_sudo::pallet::Error<T>
   **/
  PalletSudoError: {
    _enum: ['RequireSudo']
  },
  /**
   * Lookup176: pallet_multisig::Multisig<BlockNumber, Balance, sp_core::crypto::AccountId32, MaxApprovals>
   **/
  PalletMultisigMultisig: {
    when: 'PalletMultisigTimepoint',
    deposit: 'u128',
    depositor: 'AccountId32',
    approvals: 'Vec<AccountId32>'
  },
  /**
   * Lookup178: pallet_multisig::pallet::Error<T>
   **/
  PalletMultisigError: {
    _enum: ['MinimumThreshold', 'AlreadyApproved', 'NoApprovalsNeeded', 'TooFewSignatories', 'TooManySignatories', 'SignatoriesOutOfOrder', 'SenderInSignatories', 'NotFound', 'NotOwner', 'NoTimepoint', 'WrongTimepoint', 'UnexpectedTimepoint', 'MaxWeightTooLow', 'AlreadyStored']
  },
  /**
   * Lookup181: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, integritee_node_runtime::ProxyType, BlockNumber>
   **/
  PalletProxyProxyDefinition: {
    delegate: 'AccountId32',
    proxyType: 'IntegriteeNodeRuntimeProxyType',
    delay: 'u32'
  },
  /**
   * Lookup185: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber>
   **/
  PalletProxyAnnouncement: {
    real: 'AccountId32',
    callHash: 'H256',
    height: 'u32'
  },
  /**
   * Lookup187: pallet_proxy::pallet::Error<T>
   **/
  PalletProxyError: {
    _enum: ['TooMany', 'NotFound', 'NotProxy', 'Unproxyable', 'Duplicate', 'NoPermission', 'Unannounced', 'NoSelfProxy']
  },
  /**
   * Lookup190: pallet_scheduler::Scheduled<Name, frame_support::traits::preimages::Bounded<integritee_node_runtime::RuntimeCall>, BlockNumber, integritee_node_runtime::OriginCaller, sp_core::crypto::AccountId32>
   **/
  PalletSchedulerScheduled: {
    maybeId: 'Option<[u8;32]>',
    priority: 'u8',
    call: 'FrameSupportPreimagesBounded',
    maybePeriodic: 'Option<(u32,u32)>',
    origin: 'IntegriteeNodeRuntimeOriginCaller'
  },
  /**
   * Lookup191: frame_support::traits::preimages::Bounded<integritee_node_runtime::RuntimeCall>
   **/
  FrameSupportPreimagesBounded: {
    _enum: {
      Legacy: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      Inline: 'Bytes',
      Lookup: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
        len: 'u32'
      }
    }
  },
  /**
   * Lookup194: pallet_scheduler::pallet::Error<T>
   **/
  PalletSchedulerError: {
    _enum: ['FailedToSchedule', 'NotFound', 'TargetBlockNumberInPast', 'RescheduleNoChange', 'Named']
  },
  /**
   * Lookup195: pallet_utility::pallet::Error<T>
   **/
  PalletUtilityError: {
    _enum: ['TooManyCalls']
  },
  /**
   * Lookup197: pallet_balances::BalanceLock<Balance>
   **/
  PalletBalancesBalanceLock: {
    id: '[u8;8]',
    amount: 'u128',
    reasons: 'PalletBalancesReasons'
  },
  /**
   * Lookup198: pallet_balances::Reasons
   **/
  PalletBalancesReasons: {
    _enum: ['Fee', 'Misc', 'All']
  },
  /**
   * Lookup201: pallet_balances::ReserveData<ReserveIdentifier, Balance>
   **/
  PalletBalancesReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup203: pallet_balances::pallet::Error<T, I>
   **/
  PalletBalancesError: {
    _enum: ['VestingBalance', 'LiquidityRestrictions', 'InsufficientBalance', 'ExistentialDeposit', 'KeepAlive', 'ExistingVestingSchedule', 'DeadAccount', 'TooManyReserves']
  },
  /**
   * Lookup205: pallet_transaction_payment::Releases
   **/
  PalletTransactionPaymentReleases: {
    _enum: ['V1Ancient', 'V2']
  },
  /**
   * Lookup208: pallet_vesting::Releases
   **/
  PalletVestingReleases: {
    _enum: ['V0', 'V1']
  },
  /**
   * Lookup209: pallet_vesting::pallet::Error<T>
   **/
  PalletVestingError: {
    _enum: ['NotVesting', 'AtMaxVestingSchedules', 'AmountLow', 'ScheduleIndexOutOfBounds', 'InvalidScheduleParams']
  },
  /**
   * Lookup210: pallet_grandpa::StoredState<N>
   **/
  PalletGrandpaStoredState: {
    _enum: {
      Live: 'Null',
      PendingPause: {
        scheduledAt: 'u32',
        delay: 'u32',
      },
      Paused: 'Null',
      PendingResume: {
        scheduledAt: 'u32',
        delay: 'u32'
      }
    }
  },
  /**
   * Lookup211: pallet_grandpa::StoredPendingChange<N, Limit>
   **/
  PalletGrandpaStoredPendingChange: {
    scheduledAt: 'u32',
    delay: 'u32',
    nextAuthorities: 'Vec<(SpFinalityGrandpaAppPublic,u64)>',
    forced: 'Option<u32>'
  },
  /**
   * Lookup213: pallet_grandpa::pallet::Error<T>
   **/
  PalletGrandpaError: {
    _enum: ['PauseFailed', 'ResumeFailed', 'ChangePending', 'TooSoon', 'InvalidKeyOwnershipProof', 'InvalidEquivocationProof', 'DuplicateOffenceReport']
  },
  /**
   * Lookup215: pallet_collective::Votes<sp_core::crypto::AccountId32, BlockNumber>
   **/
  PalletCollectiveVotes: {
    index: 'u32',
    threshold: 'u32',
    ayes: 'Vec<AccountId32>',
    nays: 'Vec<AccountId32>',
    end: 'u32'
  },
  /**
   * Lookup216: pallet_collective::pallet::Error<T, I>
   **/
  PalletCollectiveError: {
    _enum: ['NotMember', 'DuplicateProposal', 'ProposalMissing', 'WrongIndex', 'DuplicateVote', 'AlreadyInitialized', 'TooEarly', 'TooManyProposals', 'WrongProposalWeight', 'WrongProposalLength']
  },
  /**
   * Lookup217: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
   **/
  PalletTreasuryProposal: {
    proposer: 'AccountId32',
    value: 'u128',
    beneficiary: 'AccountId32',
    bond: 'u128'
  },
  /**
   * Lookup222: frame_support::PalletId
   **/
  FrameSupportPalletId: '[u8;8]',
  /**
   * Lookup223: pallet_treasury::pallet::Error<T, I>
   **/
  PalletTreasuryError: {
    _enum: ['InsufficientProposersBalance', 'InvalidIndex', 'TooManyApprovals', 'InsufficientPermission', 'ProposalNotApproved']
  },
  /**
   * Lookup224: teerex_primitives::Enclave<sp_core::crypto::AccountId32, Url>
   **/
  TeerexPrimitivesEnclave: {
    pubkey: 'AccountId32',
    mrEnclave: '[u8;32]',
    timestamp: 'u64',
    url: 'Bytes',
    shieldingKey: 'Option<Bytes>',
    vcPubkey: 'Option<Bytes>',
    sgxMode: 'TeerexPrimitivesSgxBuildMode',
    sgxMetadata: 'TeerexPrimitivesSgxEnclaveMetadata'
  },
  /**
   * Lookup225: teerex_primitives::SgxBuildMode
   **/
  TeerexPrimitivesSgxBuildMode: {
    _enum: ['Debug', 'Production']
  },
  /**
   * Lookup226: teerex_primitives::SgxEnclaveMetadata
   **/
  TeerexPrimitivesSgxEnclaveMetadata: {
    quote: 'Bytes',
    quoteSig: 'Bytes',
    quoteCert: 'Bytes'
  },
  /**
   * Lookup227: teerex_primitives::QuotingEnclave
   **/
  TeerexPrimitivesQuotingEnclave: {
    issueDate: 'u64',
    nextUpdate: 'u64',
    miscselect: '[u8;4]',
    miscselectMask: '[u8;4]',
    attributes: '[u8;16]',
    attributesMask: '[u8;16]',
    mrsigner: '[u8;32]',
    isvprodid: 'u16',
    tcb: 'Vec<TeerexPrimitivesQeTcb>'
  },
  /**
   * Lookup230: teerex_primitives::QeTcb
   **/
  TeerexPrimitivesQeTcb: {
    isvsvn: 'u16'
  },
  /**
   * Lookup232: teerex_primitives::TcbInfoOnChain
   **/
  TeerexPrimitivesTcbInfoOnChain: {
    issueDate: 'u64',
    nextUpdate: 'u64',
    tcbLevels: 'Vec<TeerexPrimitivesTcbVersionStatus>'
  },
  /**
   * Lookup234: teerex_primitives::TcbVersionStatus
   **/
  TeerexPrimitivesTcbVersionStatus: {
    cpusvn: '[u8;16]',
    pcesvn: 'u16'
  },
  /**
   * Lookup235: pallet_teerex::pallet::Error<T>
   **/
  PalletTeerexError: {
    _enum: ['RequireAdmin', 'EnclaveSignerDecodeError', 'SenderIsNotAttestedEnclave', 'RemoteAttestationVerificationFailed', 'RemoteAttestationTooOld', 'SgxModeNotAllowed', 'EnclaveIsNotRegistered', 'WrongMrenclaveForBondingAccount', 'WrongMrenclaveForShard', 'EnclaveUrlTooLong', 'RaReportTooLong', 'EmptyEnclaveRegistry', 'ScheduledEnclaveNotExist', 'EnclaveNotInSchedule', 'CollateralInvalid', 'TooManyTopics', 'DataTooLong']
  },
  /**
   * Lookup236: pallet_claims::pallet::Error<T>
   **/
  PalletClaimsError: {
    _enum: ['InvalidEthereumSignature', 'SignerHasNoClaim', 'SenderHasNoClaim', 'PotUnderflow', 'InvalidStatement', 'VestedBalanceExists']
  },
  /**
   * Lookup240: pallet_teeracle::pallet::Error<T>
   **/
  PalletTeeracleError: {
    _enum: ['InvalidCurrency', 'ReleaseWhitelistOverflow', 'ReleaseNotWhitelisted', 'ReleaseAlreadyWhitelisted', 'TradingPairStringTooLong', 'OracleDataNameStringTooLong', 'DataSourceStringTooLong', 'OracleBlobTooBig']
  },
  /**
   * Lookup241: sidechain_primitives::SidechainBlockConfirmation
   **/
  SidechainPrimitivesSidechainBlockConfirmation: {
    blockNumber: 'u64',
    blockHeaderHash: 'H256'
  },
  /**
   * Lookup242: pallet_sidechain::pallet::Error<T>
   **/
  PalletSidechainError: {
    _enum: ['ReceivedUnexpectedSidechainBlock', 'InvalidNextFinalizationCandidateBlockNumber']
  },
  /**
   * Lookup243: pallet_identity_management::pallet::Error<T>
   **/
  PalletIdentityManagementError: {
    _enum: ['DelegateeNotExist', 'UnauthorisedUser']
  },
  /**
   * Lookup244: pallet_vc_management::vc_context::VCContext<T>
   **/
  PalletVcManagementVcContext: {
    _alias: {
      hash_: 'hash'
    },
    subject: 'AccountId32',
    assertion: 'CorePrimitivesAssertion',
    hash_: 'H256',
    status: 'PalletVcManagementVcContextStatus'
  },
  /**
   * Lookup245: pallet_vc_management::vc_context::Status
   **/
  PalletVcManagementVcContextStatus: {
    _enum: ['Active', 'Disabled']
  },
  /**
   * Lookup246: pallet_vc_management::schema::VCSchema<T>
   **/
  PalletVcManagementSchemaVcSchema: {
    id: 'Bytes',
    author: 'AccountId32',
    content: 'Bytes',
    status: 'PalletVcManagementVcContextStatus'
  },
  /**
   * Lookup249: pallet_vc_management::pallet::Error<T>
   **/
  PalletVcManagementError: {
    _enum: ['VCAlreadyExists', 'VCNotExist', 'VCSubjectMismatch', 'VCAlreadyDisabled', 'RequireAdmin', 'SchemaNotExists', 'SchemaAlreadyDisabled', 'SchemaAlreadyActivated', 'SchemaIndexOverFlow', 'LengthMismatch']
  },
  /**
   * Lookup250: pallet_group::pallet::Error<T, I>
   **/
  PalletGroupError: {
    _enum: ['GroupMemberAlreadyExists', 'GroupMemberInvalid']
  },
  /**
   * Lookup253: sp_runtime::MultiSignature
   **/
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: 'SpCoreEd25519Signature',
      Sr25519: 'SpCoreSr25519Signature',
      Ecdsa: 'SpCoreEcdsaSignature'
    }
  },
  /**
   * Lookup254: sp_core::sr25519::Signature
   **/
  SpCoreSr25519Signature: '[u8;64]',
  /**
   * Lookup255: sp_core::ecdsa::Signature
   **/
  SpCoreEcdsaSignature: '[u8;65]',
  /**
   * Lookup257: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
   **/
  FrameSystemExtensionsCheckNonZeroSender: 'Null',
  /**
   * Lookup258: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
   **/
  FrameSystemExtensionsCheckSpecVersion: 'Null',
  /**
   * Lookup259: frame_system::extensions::check_tx_version::CheckTxVersion<T>
   **/
  FrameSystemExtensionsCheckTxVersion: 'Null',
  /**
   * Lookup260: frame_system::extensions::check_genesis::CheckGenesis<T>
   **/
  FrameSystemExtensionsCheckGenesis: 'Null',
  /**
   * Lookup263: frame_system::extensions::check_nonce::CheckNonce<T>
   **/
  FrameSystemExtensionsCheckNonce: 'Compact<u32>',
  /**
   * Lookup264: frame_system::extensions::check_weight::CheckWeight<T>
   **/
  FrameSystemExtensionsCheckWeight: 'Null',
  /**
   * Lookup265: pallet_transaction_payment::ChargeTransactionPayment<T>
   **/
  PalletTransactionPaymentChargeTransactionPayment: 'Compact<u128>',
  /**
   * Lookup266: integritee_node_runtime::Runtime
   **/
  IntegriteeNodeRuntimeRuntime: 'Null'
};
