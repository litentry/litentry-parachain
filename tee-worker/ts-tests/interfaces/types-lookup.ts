// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/lookup';

import type {
    Bytes,
    Compact,
    Enum,
    Null,
    Option,
    Result,
    Struct,
    Text,
    U8aFixed,
    Vec,
    bool,
    u128,
    u16,
    u32,
    u64,
    u8,
} from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, Call, H256, MultiAddress } from '@polkadot/types/interfaces/runtime';
import type { Event } from '@polkadot/types/interfaces/system';

declare module '@polkadot/types/lookup' {
    /** @name FrameSystemAccountInfo (3) */
    interface FrameSystemAccountInfo extends Struct {
        readonly nonce: u32;
        readonly consumers: u32;
        readonly providers: u32;
        readonly sufficients: u32;
        readonly data: PalletBalancesAccountData;
    }

    /** @name PalletBalancesAccountData (5) */
    interface PalletBalancesAccountData extends Struct {
        readonly free: u128;
        readonly reserved: u128;
        readonly miscFrozen: u128;
        readonly feeFrozen: u128;
    }

    /** @name FrameSupportDispatchPerDispatchClassWeight (7) */
    interface FrameSupportDispatchPerDispatchClassWeight extends Struct {
        readonly normal: SpWeightsWeightV2Weight;
        readonly operational: SpWeightsWeightV2Weight;
        readonly mandatory: SpWeightsWeightV2Weight;
    }

    /** @name SpWeightsWeightV2Weight (8) */
    interface SpWeightsWeightV2Weight extends Struct {
        readonly refTime: Compact<u64>;
        readonly proofSize: Compact<u64>;
    }

    /** @name SpRuntimeDigest (13) */
    interface SpRuntimeDigest extends Struct {
        readonly logs: Vec<SpRuntimeDigestDigestItem>;
    }

    /** @name SpRuntimeDigestDigestItem (15) */
    interface SpRuntimeDigestDigestItem extends Enum {
        readonly isOther: boolean;
        readonly asOther: Bytes;
        readonly isConsensus: boolean;
        readonly asConsensus: ITuple<[U8aFixed, Bytes]>;
        readonly isSeal: boolean;
        readonly asSeal: ITuple<[U8aFixed, Bytes]>;
        readonly isPreRuntime: boolean;
        readonly asPreRuntime: ITuple<[U8aFixed, Bytes]>;
        readonly isRuntimeEnvironmentUpdated: boolean;
        readonly type: 'Other' | 'Consensus' | 'Seal' | 'PreRuntime' | 'RuntimeEnvironmentUpdated';
    }

    /** @name FrameSystemEventRecord (18) */
    interface FrameSystemEventRecord extends Struct {
        readonly phase: FrameSystemPhase;
        readonly event: Event;
        readonly topics: Vec<H256>;
    }

    /** @name FrameSystemEvent (20) */
    interface FrameSystemEvent extends Enum {
        readonly isExtrinsicSuccess: boolean;
        readonly asExtrinsicSuccess: {
            readonly dispatchInfo: FrameSupportDispatchDispatchInfo;
        } & Struct;
        readonly isExtrinsicFailed: boolean;
        readonly asExtrinsicFailed: {
            readonly dispatchError: SpRuntimeDispatchError;
            readonly dispatchInfo: FrameSupportDispatchDispatchInfo;
        } & Struct;
        readonly isCodeUpdated: boolean;
        readonly isNewAccount: boolean;
        readonly asNewAccount: {
            readonly account: AccountId32;
        } & Struct;
        readonly isKilledAccount: boolean;
        readonly asKilledAccount: {
            readonly account: AccountId32;
        } & Struct;
        readonly isRemarked: boolean;
        readonly asRemarked: {
            readonly sender: AccountId32;
            readonly hash_: H256;
        } & Struct;
        readonly type:
            | 'ExtrinsicSuccess'
            | 'ExtrinsicFailed'
            | 'CodeUpdated'
            | 'NewAccount'
            | 'KilledAccount'
            | 'Remarked';
    }

    /** @name FrameSupportDispatchDispatchInfo (21) */
    interface FrameSupportDispatchDispatchInfo extends Struct {
        readonly weight: SpWeightsWeightV2Weight;
        readonly class: FrameSupportDispatchDispatchClass;
        readonly paysFee: FrameSupportDispatchPays;
    }

    /** @name FrameSupportDispatchDispatchClass (22) */
    interface FrameSupportDispatchDispatchClass extends Enum {
        readonly isNormal: boolean;
        readonly isOperational: boolean;
        readonly isMandatory: boolean;
        readonly type: 'Normal' | 'Operational' | 'Mandatory';
    }

    /** @name FrameSupportDispatchPays (23) */
    interface FrameSupportDispatchPays extends Enum {
        readonly isYes: boolean;
        readonly isNo: boolean;
        readonly type: 'Yes' | 'No';
    }

    /** @name SpRuntimeDispatchError (24) */
    interface SpRuntimeDispatchError extends Enum {
        readonly isOther: boolean;
        readonly isCannotLookup: boolean;
        readonly isBadOrigin: boolean;
        readonly isModule: boolean;
        readonly asModule: SpRuntimeModuleError;
        readonly isConsumerRemaining: boolean;
        readonly isNoProviders: boolean;
        readonly isTooManyConsumers: boolean;
        readonly isToken: boolean;
        readonly asToken: SpRuntimeTokenError;
        readonly isArithmetic: boolean;
        readonly asArithmetic: SpArithmeticArithmeticError;
        readonly isTransactional: boolean;
        readonly asTransactional: SpRuntimeTransactionalError;
        readonly isExhausted: boolean;
        readonly isCorruption: boolean;
        readonly isUnavailable: boolean;
        readonly type:
            | 'Other'
            | 'CannotLookup'
            | 'BadOrigin'
            | 'Module'
            | 'ConsumerRemaining'
            | 'NoProviders'
            | 'TooManyConsumers'
            | 'Token'
            | 'Arithmetic'
            | 'Transactional'
            | 'Exhausted'
            | 'Corruption'
            | 'Unavailable';
    }

    /** @name SpRuntimeModuleError (25) */
    interface SpRuntimeModuleError extends Struct {
        readonly index: u8;
        readonly error: U8aFixed;
    }

    /** @name SpRuntimeTokenError (26) */
    interface SpRuntimeTokenError extends Enum {
        readonly isNoFunds: boolean;
        readonly isWouldDie: boolean;
        readonly isBelowMinimum: boolean;
        readonly isCannotCreate: boolean;
        readonly isUnknownAsset: boolean;
        readonly isFrozen: boolean;
        readonly isUnsupported: boolean;
        readonly type:
            | 'NoFunds'
            | 'WouldDie'
            | 'BelowMinimum'
            | 'CannotCreate'
            | 'UnknownAsset'
            | 'Frozen'
            | 'Unsupported';
    }

    /** @name SpArithmeticArithmeticError (27) */
    interface SpArithmeticArithmeticError extends Enum {
        readonly isUnderflow: boolean;
        readonly isOverflow: boolean;
        readonly isDivisionByZero: boolean;
        readonly type: 'Underflow' | 'Overflow' | 'DivisionByZero';
    }

    /** @name SpRuntimeTransactionalError (28) */
    interface SpRuntimeTransactionalError extends Enum {
        readonly isLimitReached: boolean;
        readonly isNoLayer: boolean;
        readonly type: 'LimitReached' | 'NoLayer';
    }

    /** @name PalletPreimageEvent (29) */
    interface PalletPreimageEvent extends Enum {
        readonly isNoted: boolean;
        readonly asNoted: {
            readonly hash_: H256;
        } & Struct;
        readonly isRequested: boolean;
        readonly asRequested: {
            readonly hash_: H256;
        } & Struct;
        readonly isCleared: boolean;
        readonly asCleared: {
            readonly hash_: H256;
        } & Struct;
        readonly type: 'Noted' | 'Requested' | 'Cleared';
    }

    /** @name PalletSudoEvent (30) */
    interface PalletSudoEvent extends Enum {
        readonly isSudid: boolean;
        readonly asSudid: {
            readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isKeyChanged: boolean;
        readonly asKeyChanged: {
            readonly oldSudoer: Option<AccountId32>;
        } & Struct;
        readonly isSudoAsDone: boolean;
        readonly asSudoAsDone: {
            readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly type: 'Sudid' | 'KeyChanged' | 'SudoAsDone';
    }

    /** @name PalletMultisigEvent (34) */
    interface PalletMultisigEvent extends Enum {
        readonly isNewMultisig: boolean;
        readonly asNewMultisig: {
            readonly approving: AccountId32;
            readonly multisig: AccountId32;
            readonly callHash: U8aFixed;
        } & Struct;
        readonly isMultisigApproval: boolean;
        readonly asMultisigApproval: {
            readonly approving: AccountId32;
            readonly timepoint: PalletMultisigTimepoint;
            readonly multisig: AccountId32;
            readonly callHash: U8aFixed;
        } & Struct;
        readonly isMultisigExecuted: boolean;
        readonly asMultisigExecuted: {
            readonly approving: AccountId32;
            readonly timepoint: PalletMultisigTimepoint;
            readonly multisig: AccountId32;
            readonly callHash: U8aFixed;
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isMultisigCancelled: boolean;
        readonly asMultisigCancelled: {
            readonly cancelling: AccountId32;
            readonly timepoint: PalletMultisigTimepoint;
            readonly multisig: AccountId32;
            readonly callHash: U8aFixed;
        } & Struct;
        readonly type: 'NewMultisig' | 'MultisigApproval' | 'MultisigExecuted' | 'MultisigCancelled';
    }

    /** @name PalletMultisigTimepoint (35) */
    interface PalletMultisigTimepoint extends Struct {
        readonly height: u32;
        readonly index: u32;
    }

    /** @name PalletProxyEvent (36) */
    interface PalletProxyEvent extends Enum {
        readonly isProxyExecuted: boolean;
        readonly asProxyExecuted: {
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isPureCreated: boolean;
        readonly asPureCreated: {
            readonly pure: AccountId32;
            readonly who: AccountId32;
            readonly proxyType: IntegriteeNodeRuntimeProxyType;
            readonly disambiguationIndex: u16;
        } & Struct;
        readonly isAnnounced: boolean;
        readonly asAnnounced: {
            readonly real: AccountId32;
            readonly proxy: AccountId32;
            readonly callHash: H256;
        } & Struct;
        readonly isProxyAdded: boolean;
        readonly asProxyAdded: {
            readonly delegator: AccountId32;
            readonly delegatee: AccountId32;
            readonly proxyType: IntegriteeNodeRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isProxyRemoved: boolean;
        readonly asProxyRemoved: {
            readonly delegator: AccountId32;
            readonly delegatee: AccountId32;
            readonly proxyType: IntegriteeNodeRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly type: 'ProxyExecuted' | 'PureCreated' | 'Announced' | 'ProxyAdded' | 'ProxyRemoved';
    }

    /** @name IntegriteeNodeRuntimeProxyType (37) */
    interface IntegriteeNodeRuntimeProxyType extends Enum {
        readonly isAny: boolean;
        readonly isNonTransfer: boolean;
        readonly isGovernance: boolean;
        readonly isCancelProxy: boolean;
        readonly type: 'Any' | 'NonTransfer' | 'Governance' | 'CancelProxy';
    }

    /** @name PalletSchedulerEvent (39) */
    interface PalletSchedulerEvent extends Enum {
        readonly isScheduled: boolean;
        readonly asScheduled: {
            readonly when: u32;
            readonly index: u32;
        } & Struct;
        readonly isCanceled: boolean;
        readonly asCanceled: {
            readonly when: u32;
            readonly index: u32;
        } & Struct;
        readonly isDispatched: boolean;
        readonly asDispatched: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isCallUnavailable: boolean;
        readonly asCallUnavailable: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
        } & Struct;
        readonly isPeriodicFailed: boolean;
        readonly asPeriodicFailed: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
        } & Struct;
        readonly isPermanentlyOverweight: boolean;
        readonly asPermanentlyOverweight: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
        } & Struct;
        readonly type:
            | 'Scheduled'
            | 'Canceled'
            | 'Dispatched'
            | 'CallUnavailable'
            | 'PeriodicFailed'
            | 'PermanentlyOverweight';
    }

    /** @name PalletUtilityEvent (42) */
    interface PalletUtilityEvent extends Enum {
        readonly isBatchInterrupted: boolean;
        readonly asBatchInterrupted: {
            readonly index: u32;
            readonly error: SpRuntimeDispatchError;
        } & Struct;
        readonly isBatchCompleted: boolean;
        readonly isBatchCompletedWithErrors: boolean;
        readonly isItemCompleted: boolean;
        readonly isItemFailed: boolean;
        readonly asItemFailed: {
            readonly error: SpRuntimeDispatchError;
        } & Struct;
        readonly isDispatchedAs: boolean;
        readonly asDispatchedAs: {
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly type:
            | 'BatchInterrupted'
            | 'BatchCompleted'
            | 'BatchCompletedWithErrors'
            | 'ItemCompleted'
            | 'ItemFailed'
            | 'DispatchedAs';
    }

    /** @name PalletBalancesEvent (43) */
    interface PalletBalancesEvent extends Enum {
        readonly isEndowed: boolean;
        readonly asEndowed: {
            readonly account: AccountId32;
            readonly freeBalance: u128;
        } & Struct;
        readonly isDustLost: boolean;
        readonly asDustLost: {
            readonly account: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isTransfer: boolean;
        readonly asTransfer: {
            readonly from: AccountId32;
            readonly to: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isBalanceSet: boolean;
        readonly asBalanceSet: {
            readonly who: AccountId32;
            readonly free: u128;
            readonly reserved: u128;
        } & Struct;
        readonly isReserved: boolean;
        readonly asReserved: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isUnreserved: boolean;
        readonly asUnreserved: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isReserveRepatriated: boolean;
        readonly asReserveRepatriated: {
            readonly from: AccountId32;
            readonly to: AccountId32;
            readonly amount: u128;
            readonly destinationStatus: FrameSupportTokensMiscBalanceStatus;
        } & Struct;
        readonly isDeposit: boolean;
        readonly asDeposit: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isWithdraw: boolean;
        readonly asWithdraw: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isSlashed: boolean;
        readonly asSlashed: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly type:
            | 'Endowed'
            | 'DustLost'
            | 'Transfer'
            | 'BalanceSet'
            | 'Reserved'
            | 'Unreserved'
            | 'ReserveRepatriated'
            | 'Deposit'
            | 'Withdraw'
            | 'Slashed';
    }

    /** @name FrameSupportTokensMiscBalanceStatus (44) */
    interface FrameSupportTokensMiscBalanceStatus extends Enum {
        readonly isFree: boolean;
        readonly isReserved: boolean;
        readonly type: 'Free' | 'Reserved';
    }

    /** @name PalletTransactionPaymentEvent (45) */
    interface PalletTransactionPaymentEvent extends Enum {
        readonly isTransactionFeePaid: boolean;
        readonly asTransactionFeePaid: {
            readonly who: AccountId32;
            readonly actualFee: u128;
            readonly tip: u128;
        } & Struct;
        readonly type: 'TransactionFeePaid';
    }

    /** @name PalletVestingEvent (46) */
    interface PalletVestingEvent extends Enum {
        readonly isVestingUpdated: boolean;
        readonly asVestingUpdated: {
            readonly account: AccountId32;
            readonly unvested: u128;
        } & Struct;
        readonly isVestingCompleted: boolean;
        readonly asVestingCompleted: {
            readonly account: AccountId32;
        } & Struct;
        readonly type: 'VestingUpdated' | 'VestingCompleted';
    }

    /** @name PalletGrandpaEvent (47) */
    interface PalletGrandpaEvent extends Enum {
        readonly isNewAuthorities: boolean;
        readonly asNewAuthorities: {
            readonly authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
        } & Struct;
        readonly isPaused: boolean;
        readonly isResumed: boolean;
        readonly type: 'NewAuthorities' | 'Paused' | 'Resumed';
    }

    /** @name SpFinalityGrandpaAppPublic (50) */
    interface SpFinalityGrandpaAppPublic extends SpCoreEd25519Public {}

    /** @name SpCoreEd25519Public (51) */
    interface SpCoreEd25519Public extends U8aFixed {}

    /** @name PalletCollectiveEvent (52) */
    interface PalletCollectiveEvent extends Enum {
        readonly isProposed: boolean;
        readonly asProposed: {
            readonly account: AccountId32;
            readonly proposalIndex: u32;
            readonly proposalHash: H256;
            readonly threshold: u32;
        } & Struct;
        readonly isVoted: boolean;
        readonly asVoted: {
            readonly account: AccountId32;
            readonly proposalHash: H256;
            readonly voted: bool;
            readonly yes: u32;
            readonly no: u32;
        } & Struct;
        readonly isApproved: boolean;
        readonly asApproved: {
            readonly proposalHash: H256;
        } & Struct;
        readonly isDisapproved: boolean;
        readonly asDisapproved: {
            readonly proposalHash: H256;
        } & Struct;
        readonly isExecuted: boolean;
        readonly asExecuted: {
            readonly proposalHash: H256;
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isMemberExecuted: boolean;
        readonly asMemberExecuted: {
            readonly proposalHash: H256;
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isClosed: boolean;
        readonly asClosed: {
            readonly proposalHash: H256;
            readonly yes: u32;
            readonly no: u32;
        } & Struct;
        readonly type: 'Proposed' | 'Voted' | 'Approved' | 'Disapproved' | 'Executed' | 'MemberExecuted' | 'Closed';
    }

    /** @name PalletTreasuryEvent (54) */
    interface PalletTreasuryEvent extends Enum {
        readonly isProposed: boolean;
        readonly asProposed: {
            readonly proposalIndex: u32;
        } & Struct;
        readonly isSpending: boolean;
        readonly asSpending: {
            readonly budgetRemaining: u128;
        } & Struct;
        readonly isAwarded: boolean;
        readonly asAwarded: {
            readonly proposalIndex: u32;
            readonly award: u128;
            readonly account: AccountId32;
        } & Struct;
        readonly isRejected: boolean;
        readonly asRejected: {
            readonly proposalIndex: u32;
            readonly slashed: u128;
        } & Struct;
        readonly isBurnt: boolean;
        readonly asBurnt: {
            readonly burntFunds: u128;
        } & Struct;
        readonly isRollover: boolean;
        readonly asRollover: {
            readonly rolloverBalance: u128;
        } & Struct;
        readonly isDeposit: boolean;
        readonly asDeposit: {
            readonly value: u128;
        } & Struct;
        readonly isSpendApproved: boolean;
        readonly asSpendApproved: {
            readonly proposalIndex: u32;
            readonly amount: u128;
            readonly beneficiary: AccountId32;
        } & Struct;
        readonly isUpdatedInactive: boolean;
        readonly asUpdatedInactive: {
            readonly reactivated: u128;
            readonly deactivated: u128;
        } & Struct;
        readonly type:
            | 'Proposed'
            | 'Spending'
            | 'Awarded'
            | 'Rejected'
            | 'Burnt'
            | 'Rollover'
            | 'Deposit'
            | 'SpendApproved'
            | 'UpdatedInactive';
    }

    /** @name PalletTeerexEvent (55) */
    interface PalletTeerexEvent extends Enum {
        readonly isAdminChanged: boolean;
        readonly asAdminChanged: {
            readonly oldAdmin: Option<AccountId32>;
        } & Struct;
        readonly isAddedEnclave: boolean;
        readonly asAddedEnclave: ITuple<[AccountId32, Bytes]>;
        readonly isRemovedEnclave: boolean;
        readonly asRemovedEnclave: AccountId32;
        readonly isForwarded: boolean;
        readonly asForwarded: H256;
        readonly isShieldFunds: boolean;
        readonly asShieldFunds: Bytes;
        readonly isUnshieldedFunds: boolean;
        readonly asUnshieldedFunds: AccountId32;
        readonly isProcessedParentchainBlock: boolean;
        readonly asProcessedParentchainBlock: ITuple<[AccountId32, H256, H256, u32]>;
        readonly isSetHeartbeatTimeout: boolean;
        readonly asSetHeartbeatTimeout: u64;
        readonly isUpdatedScheduledEnclave: boolean;
        readonly asUpdatedScheduledEnclave: ITuple<[u64, U8aFixed]>;
        readonly isRemovedScheduledEnclave: boolean;
        readonly asRemovedScheduledEnclave: u64;
        readonly isPublishedHash: boolean;
        readonly asPublishedHash: {
            readonly mrEnclave: U8aFixed;
            readonly hash_: H256;
            readonly data: Bytes;
        } & Struct;
        readonly type:
            | 'AdminChanged'
            | 'AddedEnclave'
            | 'RemovedEnclave'
            | 'Forwarded'
            | 'ShieldFunds'
            | 'UnshieldedFunds'
            | 'ProcessedParentchainBlock'
            | 'SetHeartbeatTimeout'
            | 'UpdatedScheduledEnclave'
            | 'RemovedScheduledEnclave'
            | 'PublishedHash';
    }

    /** @name PalletClaimsEvent (56) */
    interface PalletClaimsEvent extends Enum {
        readonly isClaimed: boolean;
        readonly asClaimed: ITuple<[AccountId32, ClaimsPrimitivesEthereumAddress, u128]>;
        readonly type: 'Claimed';
    }

    /** @name ClaimsPrimitivesEthereumAddress (57) */
    interface ClaimsPrimitivesEthereumAddress extends U8aFixed {}

    /** @name PalletTeeracleEvent (59) */
    interface PalletTeeracleEvent extends Enum {
        readonly isExchangeRateUpdated: boolean;
        readonly asExchangeRateUpdated: ITuple<[Text, Text, Option<SubstrateFixedFixedU64>]>;
        readonly isExchangeRateDeleted: boolean;
        readonly asExchangeRateDeleted: ITuple<[Text, Text]>;
        readonly isOracleUpdated: boolean;
        readonly asOracleUpdated: ITuple<[Text, Text]>;
        readonly isAddedToWhitelist: boolean;
        readonly asAddedToWhitelist: ITuple<[Text, U8aFixed]>;
        readonly isRemovedFromWhitelist: boolean;
        readonly asRemovedFromWhitelist: ITuple<[Text, U8aFixed]>;
        readonly type:
            | 'ExchangeRateUpdated'
            | 'ExchangeRateDeleted'
            | 'OracleUpdated'
            | 'AddedToWhitelist'
            | 'RemovedFromWhitelist';
    }

    /** @name SubstrateFixedFixedU64 (62) */
    interface SubstrateFixedFixedU64 extends Struct {
        readonly bits: u64;
    }

    /** @name TypenumUIntUInt (67) */
    interface TypenumUIntUInt extends Struct {
        readonly msb: TypenumUIntUTerm;
        readonly lsb: TypenumBitB0;
    }

    /** @name TypenumUIntUTerm (68) */
    interface TypenumUIntUTerm extends Struct {
        readonly msb: TypenumUintUTerm;
        readonly lsb: TypenumBitB1;
    }

    /** @name TypenumUintUTerm (69) */
    type TypenumUintUTerm = Null;

    /** @name TypenumBitB1 (70) */
    type TypenumBitB1 = Null;

    /** @name TypenumBitB0 (71) */
    type TypenumBitB0 = Null;

    /** @name PalletSidechainEvent (72) */
    interface PalletSidechainEvent extends Enum {
        readonly isProposedSidechainBlock: boolean;
        readonly asProposedSidechainBlock: ITuple<[AccountId32, H256]>;
        readonly isFinalizedSidechainBlock: boolean;
        readonly asFinalizedSidechainBlock: ITuple<[AccountId32, H256]>;
        readonly type: 'ProposedSidechainBlock' | 'FinalizedSidechainBlock';
    }

    /** @name PalletIdentityManagementEvent (73) */
    interface PalletIdentityManagementEvent extends Enum {
        readonly isDelegateeAdded: boolean;
        readonly asDelegateeAdded: {
            readonly account: AccountId32;
        } & Struct;
        readonly isDelegateeRemoved: boolean;
        readonly asDelegateeRemoved: {
            readonly account: AccountId32;
        } & Struct;
        readonly isCreateIdentityRequested: boolean;
        readonly asCreateIdentityRequested: {
            readonly shard: H256;
        } & Struct;
        readonly isRemoveIdentityRequested: boolean;
        readonly asRemoveIdentityRequested: {
            readonly shard: H256;
        } & Struct;
        readonly isVerifyIdentityRequested: boolean;
        readonly asVerifyIdentityRequested: {
            readonly shard: H256;
        } & Struct;
        readonly isSetUserShieldingKeyRequested: boolean;
        readonly asSetUserShieldingKeyRequested: {
            readonly shard: H256;
        } & Struct;
        readonly isUserShieldingKeySet: boolean;
        readonly asUserShieldingKeySet: {
            readonly account: AccountId32;
            readonly idGraph: CorePrimitivesKeyAesOutput;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isIdentityCreated: boolean;
        readonly asIdentityCreated: {
            readonly account: AccountId32;
            readonly identity: CorePrimitivesKeyAesOutput;
            readonly code: CorePrimitivesKeyAesOutput;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isIdentityRemoved: boolean;
        readonly asIdentityRemoved: {
            readonly account: AccountId32;
            readonly identity: CorePrimitivesKeyAesOutput;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isIdentityVerified: boolean;
        readonly asIdentityVerified: {
            readonly account: AccountId32;
            readonly identity: CorePrimitivesKeyAesOutput;
            readonly idGraph: CorePrimitivesKeyAesOutput;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isSetUserShieldingKeyFailed: boolean;
        readonly asSetUserShieldingKeyFailed: {
            readonly account: Option<AccountId32>;
            readonly detail: CorePrimitivesErrorErrorDetail;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isCreateIdentityFailed: boolean;
        readonly asCreateIdentityFailed: {
            readonly account: Option<AccountId32>;
            readonly detail: CorePrimitivesErrorErrorDetail;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isRemoveIdentityFailed: boolean;
        readonly asRemoveIdentityFailed: {
            readonly account: Option<AccountId32>;
            readonly detail: CorePrimitivesErrorErrorDetail;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isVerifyIdentityFailed: boolean;
        readonly asVerifyIdentityFailed: {
            readonly account: Option<AccountId32>;
            readonly detail: CorePrimitivesErrorErrorDetail;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isImportScheduledEnclaveFailed: boolean;
        readonly isUnclassifiedError: boolean;
        readonly asUnclassifiedError: {
            readonly account: Option<AccountId32>;
            readonly detail: CorePrimitivesErrorErrorDetail;
            readonly reqExtHash: H256;
        } & Struct;
        readonly type:
            | 'DelegateeAdded'
            | 'DelegateeRemoved'
            | 'CreateIdentityRequested'
            | 'RemoveIdentityRequested'
            | 'VerifyIdentityRequested'
            | 'SetUserShieldingKeyRequested'
            | 'UserShieldingKeySet'
            | 'IdentityCreated'
            | 'IdentityRemoved'
            | 'IdentityVerified'
            | 'SetUserShieldingKeyFailed'
            | 'CreateIdentityFailed'
            | 'RemoveIdentityFailed'
            | 'VerifyIdentityFailed'
            | 'ImportScheduledEnclaveFailed'
            | 'UnclassifiedError';
    }

    /** @name CorePrimitivesKeyAesOutput (74) */
    interface CorePrimitivesKeyAesOutput extends Struct {
        readonly ciphertext: Bytes;
        readonly aad: Bytes;
        readonly nonce: U8aFixed;
    }

    /** @name CorePrimitivesErrorErrorDetail (76) */
    interface CorePrimitivesErrorErrorDetail extends Enum {
        readonly isImportError: boolean;
        readonly isStfError: boolean;
        readonly asStfError: Bytes;
        readonly isSendStfRequestFailed: boolean;
        readonly isChallengeCodeNotFound: boolean;
        readonly isUserShieldingKeyNotFound: boolean;
        readonly isParseError: boolean;
        readonly isDataProviderError: boolean;
        readonly asDataProviderError: Bytes;
        readonly isInvalidIdentity: boolean;
        readonly isWrongWeb2Handle: boolean;
        readonly isUnexpectedMessage: boolean;
        readonly isWrongSignatureType: boolean;
        readonly isVerifySubstrateSignatureFailed: boolean;
        readonly isVerifyEvmSignatureFailed: boolean;
        readonly isRecoverEvmAddressFailed: boolean;
        readonly type:
            | 'ImportError'
            | 'StfError'
            | 'SendStfRequestFailed'
            | 'ChallengeCodeNotFound'
            | 'UserShieldingKeyNotFound'
            | 'ParseError'
            | 'DataProviderError'
            | 'InvalidIdentity'
            | 'WrongWeb2Handle'
            | 'UnexpectedMessage'
            | 'WrongSignatureType'
            | 'VerifySubstrateSignatureFailed'
            | 'VerifyEvmSignatureFailed'
            | 'RecoverEvmAddressFailed';
    }

    /** @name PalletVcManagementEvent (78) */
    interface PalletVcManagementEvent extends Enum {
        readonly isVcRequested: boolean;
        readonly asVcRequested: {
            readonly account: AccountId32;
            readonly shard: H256;
            readonly assertion: CorePrimitivesAssertion;
        } & Struct;
        readonly isVcDisabled: boolean;
        readonly asVcDisabled: {
            readonly account: AccountId32;
            readonly index: H256;
        } & Struct;
        readonly isVcRevoked: boolean;
        readonly asVcRevoked: {
            readonly account: AccountId32;
            readonly index: H256;
        } & Struct;
        readonly isVcIssued: boolean;
        readonly asVcIssued: {
            readonly account: AccountId32;
            readonly assertion: CorePrimitivesAssertion;
            readonly index: H256;
            readonly vc: CorePrimitivesKeyAesOutput;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isAdminChanged: boolean;
        readonly asAdminChanged: {
            readonly oldAdmin: Option<AccountId32>;
            readonly newAdmin: Option<AccountId32>;
        } & Struct;
        readonly isSchemaIssued: boolean;
        readonly asSchemaIssued: {
            readonly account: AccountId32;
            readonly shard: H256;
            readonly index: u64;
        } & Struct;
        readonly isSchemaDisabled: boolean;
        readonly asSchemaDisabled: {
            readonly account: AccountId32;
            readonly shard: H256;
            readonly index: u64;
        } & Struct;
        readonly isSchemaActivated: boolean;
        readonly asSchemaActivated: {
            readonly account: AccountId32;
            readonly shard: H256;
            readonly index: u64;
        } & Struct;
        readonly isSchemaRevoked: boolean;
        readonly asSchemaRevoked: {
            readonly account: AccountId32;
            readonly shard: H256;
            readonly index: u64;
        } & Struct;
        readonly isRequestVCFailed: boolean;
        readonly asRequestVCFailed: {
            readonly account: Option<AccountId32>;
            readonly assertion: CorePrimitivesAssertion;
            readonly detail: CorePrimitivesErrorErrorDetail;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isUnclassifiedError: boolean;
        readonly asUnclassifiedError: {
            readonly account: Option<AccountId32>;
            readonly detail: CorePrimitivesErrorErrorDetail;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isVcRegistryItemAdded: boolean;
        readonly asVcRegistryItemAdded: {
            readonly account: AccountId32;
            readonly assertion: CorePrimitivesAssertion;
            readonly index: H256;
        } & Struct;
        readonly isVcRegistryItemRemoved: boolean;
        readonly asVcRegistryItemRemoved: {
            readonly index: H256;
        } & Struct;
        readonly isVcRegistryCleared: boolean;
        readonly type:
            | 'VcRequested'
            | 'VcDisabled'
            | 'VcRevoked'
            | 'VcIssued'
            | 'AdminChanged'
            | 'SchemaIssued'
            | 'SchemaDisabled'
            | 'SchemaActivated'
            | 'SchemaRevoked'
            | 'RequestVCFailed'
            | 'UnclassifiedError'
            | 'VcRegistryItemAdded'
            | 'VcRegistryItemRemoved'
            | 'VcRegistryCleared';
    }

    /** @name CorePrimitivesAssertion (79) */
    interface CorePrimitivesAssertion extends Enum {
        readonly isA1: boolean;
        readonly isA2: boolean;
        readonly asA2: Bytes;
        readonly isA3: boolean;
        readonly asA3: ITuple<[Bytes, Bytes, Bytes]>;
        readonly isA4: boolean;
        readonly asA4: Bytes;
        readonly isA5: boolean;
        readonly asA5: Bytes;
        readonly isA6: boolean;
        readonly isA7: boolean;
        readonly asA7: Bytes;
        readonly isA8: boolean;
        readonly asA8: Vec<CorePrimitivesAssertionIndexingNetwork>;
        readonly isA9: boolean;
        readonly isA10: boolean;
        readonly asA10: Bytes;
        readonly isA11: boolean;
        readonly asA11: Bytes;
        readonly isA13: boolean;
        readonly asA13: u32;
        readonly type: 'A1' | 'A2' | 'A3' | 'A4' | 'A5' | 'A6' | 'A7' | 'A8' | 'A9' | 'A10' | 'A11' | 'A13';
    }

    /** @name CorePrimitivesAssertionIndexingNetwork (82) */
    interface CorePrimitivesAssertionIndexingNetwork extends Enum {
        readonly isLitentry: boolean;
        readonly isLitmus: boolean;
        readonly isPolkadot: boolean;
        readonly isKusama: boolean;
        readonly isKhala: boolean;
        readonly isEthereum: boolean;
        readonly type: 'Litentry' | 'Litmus' | 'Polkadot' | 'Kusama' | 'Khala' | 'Ethereum';
    }

    /** @name PalletGroupEvent (84) */
    interface PalletGroupEvent extends Enum {
        readonly isGroupMemberAdded: boolean;
        readonly asGroupMemberAdded: AccountId32;
        readonly isGroupMemberRemoved: boolean;
        readonly asGroupMemberRemoved: AccountId32;
        readonly type: 'GroupMemberAdded' | 'GroupMemberRemoved';
    }

    /** @name FrameSystemPhase (86) */
    interface FrameSystemPhase extends Enum {
        readonly isApplyExtrinsic: boolean;
        readonly asApplyExtrinsic: u32;
        readonly isFinalization: boolean;
        readonly isInitialization: boolean;
        readonly type: 'ApplyExtrinsic' | 'Finalization' | 'Initialization';
    }

    /** @name FrameSystemLastRuntimeUpgradeInfo (89) */
    interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
        readonly specVersion: Compact<u32>;
        readonly specName: Text;
    }

    /** @name FrameSystemCall (91) */
    interface FrameSystemCall extends Enum {
        readonly isRemark: boolean;
        readonly asRemark: {
            readonly remark: Bytes;
        } & Struct;
        readonly isSetHeapPages: boolean;
        readonly asSetHeapPages: {
            readonly pages: u64;
        } & Struct;
        readonly isSetCode: boolean;
        readonly asSetCode: {
            readonly code: Bytes;
        } & Struct;
        readonly isSetCodeWithoutChecks: boolean;
        readonly asSetCodeWithoutChecks: {
            readonly code: Bytes;
        } & Struct;
        readonly isSetStorage: boolean;
        readonly asSetStorage: {
            readonly items: Vec<ITuple<[Bytes, Bytes]>>;
        } & Struct;
        readonly isKillStorage: boolean;
        readonly asKillStorage: {
            readonly keys_: Vec<Bytes>;
        } & Struct;
        readonly isKillPrefix: boolean;
        readonly asKillPrefix: {
            readonly prefix: Bytes;
            readonly subkeys: u32;
        } & Struct;
        readonly isRemarkWithEvent: boolean;
        readonly asRemarkWithEvent: {
            readonly remark: Bytes;
        } & Struct;
        readonly type:
            | 'Remark'
            | 'SetHeapPages'
            | 'SetCode'
            | 'SetCodeWithoutChecks'
            | 'SetStorage'
            | 'KillStorage'
            | 'KillPrefix'
            | 'RemarkWithEvent';
    }

    /** @name FrameSystemLimitsBlockWeights (95) */
    interface FrameSystemLimitsBlockWeights extends Struct {
        readonly baseBlock: SpWeightsWeightV2Weight;
        readonly maxBlock: SpWeightsWeightV2Weight;
        readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    }

    /** @name FrameSupportDispatchPerDispatchClassWeightsPerClass (96) */
    interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
        readonly normal: FrameSystemLimitsWeightsPerClass;
        readonly operational: FrameSystemLimitsWeightsPerClass;
        readonly mandatory: FrameSystemLimitsWeightsPerClass;
    }

    /** @name FrameSystemLimitsWeightsPerClass (97) */
    interface FrameSystemLimitsWeightsPerClass extends Struct {
        readonly baseExtrinsic: SpWeightsWeightV2Weight;
        readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
        readonly maxTotal: Option<SpWeightsWeightV2Weight>;
        readonly reserved: Option<SpWeightsWeightV2Weight>;
    }

    /** @name FrameSystemLimitsBlockLength (99) */
    interface FrameSystemLimitsBlockLength extends Struct {
        readonly max: FrameSupportDispatchPerDispatchClassU32;
    }

    /** @name FrameSupportDispatchPerDispatchClassU32 (100) */
    interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
        readonly normal: u32;
        readonly operational: u32;
        readonly mandatory: u32;
    }

    /** @name SpWeightsRuntimeDbWeight (101) */
    interface SpWeightsRuntimeDbWeight extends Struct {
        readonly read: u64;
        readonly write: u64;
    }

    /** @name SpVersionRuntimeVersion (102) */
    interface SpVersionRuntimeVersion extends Struct {
        readonly specName: Text;
        readonly implName: Text;
        readonly authoringVersion: u32;
        readonly specVersion: u32;
        readonly implVersion: u32;
        readonly apis: Vec<ITuple<[U8aFixed, u32]>>;
        readonly transactionVersion: u32;
        readonly stateVersion: u8;
    }

    /** @name FrameSystemError (107) */
    interface FrameSystemError extends Enum {
        readonly isInvalidSpecName: boolean;
        readonly isSpecVersionNeedsToIncrease: boolean;
        readonly isFailedToExtractRuntimeVersion: boolean;
        readonly isNonDefaultComposite: boolean;
        readonly isNonZeroRefCount: boolean;
        readonly isCallFiltered: boolean;
        readonly type:
            | 'InvalidSpecName'
            | 'SpecVersionNeedsToIncrease'
            | 'FailedToExtractRuntimeVersion'
            | 'NonDefaultComposite'
            | 'NonZeroRefCount'
            | 'CallFiltered';
    }

    /** @name PalletPreimageRequestStatus (108) */
    interface PalletPreimageRequestStatus extends Enum {
        readonly isUnrequested: boolean;
        readonly asUnrequested: {
            readonly deposit: ITuple<[AccountId32, u128]>;
            readonly len: u32;
        } & Struct;
        readonly isRequested: boolean;
        readonly asRequested: {
            readonly deposit: Option<ITuple<[AccountId32, u128]>>;
            readonly count: u32;
            readonly len: Option<u32>;
        } & Struct;
        readonly type: 'Unrequested' | 'Requested';
    }

    /** @name PalletPreimageCall (114) */
    interface PalletPreimageCall extends Enum {
        readonly isNotePreimage: boolean;
        readonly asNotePreimage: {
            readonly bytes: Bytes;
        } & Struct;
        readonly isUnnotePreimage: boolean;
        readonly asUnnotePreimage: {
            readonly hash_: H256;
        } & Struct;
        readonly isRequestPreimage: boolean;
        readonly asRequestPreimage: {
            readonly hash_: H256;
        } & Struct;
        readonly isUnrequestPreimage: boolean;
        readonly asUnrequestPreimage: {
            readonly hash_: H256;
        } & Struct;
        readonly type: 'NotePreimage' | 'UnnotePreimage' | 'RequestPreimage' | 'UnrequestPreimage';
    }

    /** @name PalletPreimageError (115) */
    interface PalletPreimageError extends Enum {
        readonly isTooBig: boolean;
        readonly isAlreadyNoted: boolean;
        readonly isNotAuthorized: boolean;
        readonly isNotNoted: boolean;
        readonly isRequested: boolean;
        readonly isNotRequested: boolean;
        readonly type: 'TooBig' | 'AlreadyNoted' | 'NotAuthorized' | 'NotNoted' | 'Requested' | 'NotRequested';
    }

    /** @name PalletTimestampCall (117) */
    interface PalletTimestampCall extends Enum {
        readonly isSet: boolean;
        readonly asSet: {
            readonly now: Compact<u64>;
        } & Struct;
        readonly type: 'Set';
    }

    /** @name PalletSudoCall (118) */
    interface PalletSudoCall extends Enum {
        readonly isSudo: boolean;
        readonly asSudo: {
            readonly call: Call;
        } & Struct;
        readonly isSudoUncheckedWeight: boolean;
        readonly asSudoUncheckedWeight: {
            readonly call: Call;
            readonly weight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isSetKey: boolean;
        readonly asSetKey: {
            readonly new_: MultiAddress;
        } & Struct;
        readonly isSudoAs: boolean;
        readonly asSudoAs: {
            readonly who: MultiAddress;
            readonly call: Call;
        } & Struct;
        readonly type: 'Sudo' | 'SudoUncheckedWeight' | 'SetKey' | 'SudoAs';
    }

    /** @name PalletMultisigCall (120) */
    interface PalletMultisigCall extends Enum {
        readonly isAsMultiThreshold1: boolean;
        readonly asAsMultiThreshold1: {
            readonly otherSignatories: Vec<AccountId32>;
            readonly call: Call;
        } & Struct;
        readonly isAsMulti: boolean;
        readonly asAsMulti: {
            readonly threshold: u16;
            readonly otherSignatories: Vec<AccountId32>;
            readonly maybeTimepoint: Option<PalletMultisigTimepoint>;
            readonly call: Call;
            readonly maxWeight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isApproveAsMulti: boolean;
        readonly asApproveAsMulti: {
            readonly threshold: u16;
            readonly otherSignatories: Vec<AccountId32>;
            readonly maybeTimepoint: Option<PalletMultisigTimepoint>;
            readonly callHash: U8aFixed;
            readonly maxWeight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isCancelAsMulti: boolean;
        readonly asCancelAsMulti: {
            readonly threshold: u16;
            readonly otherSignatories: Vec<AccountId32>;
            readonly timepoint: PalletMultisigTimepoint;
            readonly callHash: U8aFixed;
        } & Struct;
        readonly type: 'AsMultiThreshold1' | 'AsMulti' | 'ApproveAsMulti' | 'CancelAsMulti';
    }

    /** @name PalletProxyCall (123) */
    interface PalletProxyCall extends Enum {
        readonly isProxy: boolean;
        readonly asProxy: {
            readonly real: MultiAddress;
            readonly forceProxyType: Option<IntegriteeNodeRuntimeProxyType>;
            readonly call: Call;
        } & Struct;
        readonly isAddProxy: boolean;
        readonly asAddProxy: {
            readonly delegate: MultiAddress;
            readonly proxyType: IntegriteeNodeRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isRemoveProxy: boolean;
        readonly asRemoveProxy: {
            readonly delegate: MultiAddress;
            readonly proxyType: IntegriteeNodeRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isRemoveProxies: boolean;
        readonly isCreatePure: boolean;
        readonly asCreatePure: {
            readonly proxyType: IntegriteeNodeRuntimeProxyType;
            readonly delay: u32;
            readonly index: u16;
        } & Struct;
        readonly isKillPure: boolean;
        readonly asKillPure: {
            readonly spawner: MultiAddress;
            readonly proxyType: IntegriteeNodeRuntimeProxyType;
            readonly index: u16;
            readonly height: Compact<u32>;
            readonly extIndex: Compact<u32>;
        } & Struct;
        readonly isAnnounce: boolean;
        readonly asAnnounce: {
            readonly real: MultiAddress;
            readonly callHash: H256;
        } & Struct;
        readonly isRemoveAnnouncement: boolean;
        readonly asRemoveAnnouncement: {
            readonly real: MultiAddress;
            readonly callHash: H256;
        } & Struct;
        readonly isRejectAnnouncement: boolean;
        readonly asRejectAnnouncement: {
            readonly delegate: MultiAddress;
            readonly callHash: H256;
        } & Struct;
        readonly isProxyAnnounced: boolean;
        readonly asProxyAnnounced: {
            readonly delegate: MultiAddress;
            readonly real: MultiAddress;
            readonly forceProxyType: Option<IntegriteeNodeRuntimeProxyType>;
            readonly call: Call;
        } & Struct;
        readonly type:
            | 'Proxy'
            | 'AddProxy'
            | 'RemoveProxy'
            | 'RemoveProxies'
            | 'CreatePure'
            | 'KillPure'
            | 'Announce'
            | 'RemoveAnnouncement'
            | 'RejectAnnouncement'
            | 'ProxyAnnounced';
    }

    /** @name PalletSchedulerCall (127) */
    interface PalletSchedulerCall extends Enum {
        readonly isSchedule: boolean;
        readonly asSchedule: {
            readonly when: u32;
            readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
            readonly priority: u8;
            readonly call: Call;
        } & Struct;
        readonly isCancel: boolean;
        readonly asCancel: {
            readonly when: u32;
            readonly index: u32;
        } & Struct;
        readonly isScheduleNamed: boolean;
        readonly asScheduleNamed: {
            readonly id: U8aFixed;
            readonly when: u32;
            readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
            readonly priority: u8;
            readonly call: Call;
        } & Struct;
        readonly isCancelNamed: boolean;
        readonly asCancelNamed: {
            readonly id: U8aFixed;
        } & Struct;
        readonly isScheduleAfter: boolean;
        readonly asScheduleAfter: {
            readonly after: u32;
            readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
            readonly priority: u8;
            readonly call: Call;
        } & Struct;
        readonly isScheduleNamedAfter: boolean;
        readonly asScheduleNamedAfter: {
            readonly id: U8aFixed;
            readonly after: u32;
            readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
            readonly priority: u8;
            readonly call: Call;
        } & Struct;
        readonly type: 'Schedule' | 'Cancel' | 'ScheduleNamed' | 'CancelNamed' | 'ScheduleAfter' | 'ScheduleNamedAfter';
    }

    /** @name PalletUtilityCall (129) */
    interface PalletUtilityCall extends Enum {
        readonly isBatch: boolean;
        readonly asBatch: {
            readonly calls: Vec<Call>;
        } & Struct;
        readonly isAsDerivative: boolean;
        readonly asAsDerivative: {
            readonly index: u16;
            readonly call: Call;
        } & Struct;
        readonly isBatchAll: boolean;
        readonly asBatchAll: {
            readonly calls: Vec<Call>;
        } & Struct;
        readonly isDispatchAs: boolean;
        readonly asDispatchAs: {
            readonly asOrigin: IntegriteeNodeRuntimeOriginCaller;
            readonly call: Call;
        } & Struct;
        readonly isForceBatch: boolean;
        readonly asForceBatch: {
            readonly calls: Vec<Call>;
        } & Struct;
        readonly isWithWeight: boolean;
        readonly asWithWeight: {
            readonly call: Call;
            readonly weight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly type: 'Batch' | 'AsDerivative' | 'BatchAll' | 'DispatchAs' | 'ForceBatch' | 'WithWeight';
    }

    /** @name IntegriteeNodeRuntimeOriginCaller (131) */
    interface IntegriteeNodeRuntimeOriginCaller extends Enum {
        readonly isSystem: boolean;
        readonly asSystem: FrameSupportDispatchRawOrigin;
        readonly isVoid: boolean;
        readonly isCouncil: boolean;
        readonly asCouncil: PalletCollectiveRawOrigin;
        readonly type: 'System' | 'Void' | 'Council';
    }

    /** @name FrameSupportDispatchRawOrigin (132) */
    interface FrameSupportDispatchRawOrigin extends Enum {
        readonly isRoot: boolean;
        readonly isSigned: boolean;
        readonly asSigned: AccountId32;
        readonly isNone: boolean;
        readonly type: 'Root' | 'Signed' | 'None';
    }

    /** @name PalletCollectiveRawOrigin (133) */
    interface PalletCollectiveRawOrigin extends Enum {
        readonly isMembers: boolean;
        readonly asMembers: ITuple<[u32, u32]>;
        readonly isMember: boolean;
        readonly asMember: AccountId32;
        readonly isPhantom: boolean;
        readonly type: 'Members' | 'Member' | 'Phantom';
    }

    /** @name SpCoreVoid (134) */
    type SpCoreVoid = Null;

    /** @name PalletBalancesCall (135) */
    interface PalletBalancesCall extends Enum {
        readonly isTransfer: boolean;
        readonly asTransfer: {
            readonly dest: MultiAddress;
            readonly value: Compact<u128>;
        } & Struct;
        readonly isSetBalance: boolean;
        readonly asSetBalance: {
            readonly who: MultiAddress;
            readonly newFree: Compact<u128>;
            readonly newReserved: Compact<u128>;
        } & Struct;
        readonly isForceTransfer: boolean;
        readonly asForceTransfer: {
            readonly source: MultiAddress;
            readonly dest: MultiAddress;
            readonly value: Compact<u128>;
        } & Struct;
        readonly isTransferKeepAlive: boolean;
        readonly asTransferKeepAlive: {
            readonly dest: MultiAddress;
            readonly value: Compact<u128>;
        } & Struct;
        readonly isTransferAll: boolean;
        readonly asTransferAll: {
            readonly dest: MultiAddress;
            readonly keepAlive: bool;
        } & Struct;
        readonly isForceUnreserve: boolean;
        readonly asForceUnreserve: {
            readonly who: MultiAddress;
            readonly amount: u128;
        } & Struct;
        readonly type:
            | 'Transfer'
            | 'SetBalance'
            | 'ForceTransfer'
            | 'TransferKeepAlive'
            | 'TransferAll'
            | 'ForceUnreserve';
    }

    /** @name PalletVestingCall (137) */
    interface PalletVestingCall extends Enum {
        readonly isVest: boolean;
        readonly isVestOther: boolean;
        readonly asVestOther: {
            readonly target: MultiAddress;
        } & Struct;
        readonly isVestedTransfer: boolean;
        readonly asVestedTransfer: {
            readonly target: MultiAddress;
            readonly schedule: PalletVestingVestingInfo;
        } & Struct;
        readonly isForceVestedTransfer: boolean;
        readonly asForceVestedTransfer: {
            readonly source: MultiAddress;
            readonly target: MultiAddress;
            readonly schedule: PalletVestingVestingInfo;
        } & Struct;
        readonly isMergeSchedules: boolean;
        readonly asMergeSchedules: {
            readonly schedule1Index: u32;
            readonly schedule2Index: u32;
        } & Struct;
        readonly type: 'Vest' | 'VestOther' | 'VestedTransfer' | 'ForceVestedTransfer' | 'MergeSchedules';
    }

    /** @name PalletVestingVestingInfo (138) */
    interface PalletVestingVestingInfo extends Struct {
        readonly locked: u128;
        readonly perBlock: u128;
        readonly startingBlock: u32;
    }

    /** @name PalletGrandpaCall (139) */
    interface PalletGrandpaCall extends Enum {
        readonly isReportEquivocation: boolean;
        readonly asReportEquivocation: {
            readonly equivocationProof: SpFinalityGrandpaEquivocationProof;
            readonly keyOwnerProof: SpCoreVoid;
        } & Struct;
        readonly isReportEquivocationUnsigned: boolean;
        readonly asReportEquivocationUnsigned: {
            readonly equivocationProof: SpFinalityGrandpaEquivocationProof;
            readonly keyOwnerProof: SpCoreVoid;
        } & Struct;
        readonly isNoteStalled: boolean;
        readonly asNoteStalled: {
            readonly delay: u32;
            readonly bestFinalizedBlockNumber: u32;
        } & Struct;
        readonly type: 'ReportEquivocation' | 'ReportEquivocationUnsigned' | 'NoteStalled';
    }

    /** @name SpFinalityGrandpaEquivocationProof (140) */
    interface SpFinalityGrandpaEquivocationProof extends Struct {
        readonly setId: u64;
        readonly equivocation: SpFinalityGrandpaEquivocation;
    }

    /** @name SpFinalityGrandpaEquivocation (141) */
    interface SpFinalityGrandpaEquivocation extends Enum {
        readonly isPrevote: boolean;
        readonly asPrevote: FinalityGrandpaEquivocationPrevote;
        readonly isPrecommit: boolean;
        readonly asPrecommit: FinalityGrandpaEquivocationPrecommit;
        readonly type: 'Prevote' | 'Precommit';
    }

    /** @name FinalityGrandpaEquivocationPrevote (142) */
    interface FinalityGrandpaEquivocationPrevote extends Struct {
        readonly roundNumber: u64;
        readonly identity: SpFinalityGrandpaAppPublic;
        readonly first: ITuple<[FinalityGrandpaPrevote, SpFinalityGrandpaAppSignature]>;
        readonly second: ITuple<[FinalityGrandpaPrevote, SpFinalityGrandpaAppSignature]>;
    }

    /** @name FinalityGrandpaPrevote (143) */
    interface FinalityGrandpaPrevote extends Struct {
        readonly targetHash: H256;
        readonly targetNumber: u32;
    }

    /** @name SpFinalityGrandpaAppSignature (144) */
    interface SpFinalityGrandpaAppSignature extends SpCoreEd25519Signature {}

    /** @name SpCoreEd25519Signature (145) */
    interface SpCoreEd25519Signature extends U8aFixed {}

    /** @name FinalityGrandpaEquivocationPrecommit (148) */
    interface FinalityGrandpaEquivocationPrecommit extends Struct {
        readonly roundNumber: u64;
        readonly identity: SpFinalityGrandpaAppPublic;
        readonly first: ITuple<[FinalityGrandpaPrecommit, SpFinalityGrandpaAppSignature]>;
        readonly second: ITuple<[FinalityGrandpaPrecommit, SpFinalityGrandpaAppSignature]>;
    }

    /** @name FinalityGrandpaPrecommit (149) */
    interface FinalityGrandpaPrecommit extends Struct {
        readonly targetHash: H256;
        readonly targetNumber: u32;
    }

    /** @name PalletCollectiveCall (151) */
    interface PalletCollectiveCall extends Enum {
        readonly isSetMembers: boolean;
        readonly asSetMembers: {
            readonly newMembers: Vec<AccountId32>;
            readonly prime: Option<AccountId32>;
            readonly oldCount: u32;
        } & Struct;
        readonly isExecute: boolean;
        readonly asExecute: {
            readonly proposal: Call;
            readonly lengthBound: Compact<u32>;
        } & Struct;
        readonly isPropose: boolean;
        readonly asPropose: {
            readonly threshold: Compact<u32>;
            readonly proposal: Call;
            readonly lengthBound: Compact<u32>;
        } & Struct;
        readonly isVote: boolean;
        readonly asVote: {
            readonly proposal: H256;
            readonly index: Compact<u32>;
            readonly approve: bool;
        } & Struct;
        readonly isCloseOldWeight: boolean;
        readonly asCloseOldWeight: {
            readonly proposalHash: H256;
            readonly index: Compact<u32>;
            readonly proposalWeightBound: Compact<u64>;
            readonly lengthBound: Compact<u32>;
        } & Struct;
        readonly isDisapproveProposal: boolean;
        readonly asDisapproveProposal: {
            readonly proposalHash: H256;
        } & Struct;
        readonly isClose: boolean;
        readonly asClose: {
            readonly proposalHash: H256;
            readonly index: Compact<u32>;
            readonly proposalWeightBound: SpWeightsWeightV2Weight;
            readonly lengthBound: Compact<u32>;
        } & Struct;
        readonly type:
            | 'SetMembers'
            | 'Execute'
            | 'Propose'
            | 'Vote'
            | 'CloseOldWeight'
            | 'DisapproveProposal'
            | 'Close';
    }

    /** @name PalletTreasuryCall (154) */
    interface PalletTreasuryCall extends Enum {
        readonly isProposeSpend: boolean;
        readonly asProposeSpend: {
            readonly value: Compact<u128>;
            readonly beneficiary: MultiAddress;
        } & Struct;
        readonly isRejectProposal: boolean;
        readonly asRejectProposal: {
            readonly proposalId: Compact<u32>;
        } & Struct;
        readonly isApproveProposal: boolean;
        readonly asApproveProposal: {
            readonly proposalId: Compact<u32>;
        } & Struct;
        readonly isSpend: boolean;
        readonly asSpend: {
            readonly amount: Compact<u128>;
            readonly beneficiary: MultiAddress;
        } & Struct;
        readonly isRemoveApproval: boolean;
        readonly asRemoveApproval: {
            readonly proposalId: Compact<u32>;
        } & Struct;
        readonly type: 'ProposeSpend' | 'RejectProposal' | 'ApproveProposal' | 'Spend' | 'RemoveApproval';
    }

    /** @name PalletTeerexCall (155) */
    interface PalletTeerexCall extends Enum {
        readonly isRegisterEnclave: boolean;
        readonly asRegisterEnclave: {
            readonly raReport: Bytes;
            readonly workerUrl: Bytes;
            readonly shieldingKey: Option<Bytes>;
            readonly vcPubkey: Option<Bytes>;
        } & Struct;
        readonly isUnregisterEnclave: boolean;
        readonly isCallWorker: boolean;
        readonly asCallWorker: {
            readonly request: TeerexPrimitivesRequest;
        } & Struct;
        readonly isConfirmProcessedParentchainBlock: boolean;
        readonly asConfirmProcessedParentchainBlock: {
            readonly blockHash: H256;
            readonly blockNumber: u32;
            readonly trustedCallsMerkleRoot: H256;
        } & Struct;
        readonly isShieldFunds: boolean;
        readonly asShieldFunds: {
            readonly incognitoAccountEncrypted: Bytes;
            readonly amount: u128;
            readonly bondingAccount: AccountId32;
        } & Struct;
        readonly isUnshieldFunds: boolean;
        readonly asUnshieldFunds: {
            readonly publicAccount: AccountId32;
            readonly amount: u128;
            readonly bondingAccount: AccountId32;
            readonly callHash: H256;
        } & Struct;
        readonly isSetHeartbeatTimeout: boolean;
        readonly asSetHeartbeatTimeout: {
            readonly timeout: u64;
        } & Struct;
        readonly isRegisterDcapEnclave: boolean;
        readonly asRegisterDcapEnclave: {
            readonly dcapQuote: Bytes;
            readonly workerUrl: Bytes;
        } & Struct;
        readonly isUpdateScheduledEnclave: boolean;
        readonly asUpdateScheduledEnclave: {
            readonly sidechainBlockNumber: u64;
            readonly mrEnclave: U8aFixed;
        } & Struct;
        readonly isRegisterQuotingEnclave: boolean;
        readonly asRegisterQuotingEnclave: {
            readonly enclaveIdentity: Bytes;
            readonly signature: Bytes;
            readonly certificateChain: Bytes;
        } & Struct;
        readonly isRemoveScheduledEnclave: boolean;
        readonly asRemoveScheduledEnclave: {
            readonly sidechainBlockNumber: u64;
        } & Struct;
        readonly isRegisterTcbInfo: boolean;
        readonly asRegisterTcbInfo: {
            readonly tcbInfo: Bytes;
            readonly signature: Bytes;
            readonly certificateChain: Bytes;
        } & Struct;
        readonly isPublishHash: boolean;
        readonly asPublishHash: {
            readonly hash_: H256;
            readonly extraTopics: Vec<H256>;
            readonly data: Bytes;
        } & Struct;
        readonly isSetAdmin: boolean;
        readonly asSetAdmin: {
            readonly new_: AccountId32;
        } & Struct;
        readonly type:
            | 'RegisterEnclave'
            | 'UnregisterEnclave'
            | 'CallWorker'
            | 'ConfirmProcessedParentchainBlock'
            | 'ShieldFunds'
            | 'UnshieldFunds'
            | 'SetHeartbeatTimeout'
            | 'RegisterDcapEnclave'
            | 'UpdateScheduledEnclave'
            | 'RegisterQuotingEnclave'
            | 'RemoveScheduledEnclave'
            | 'RegisterTcbInfo'
            | 'PublishHash'
            | 'SetAdmin';
    }

    /** @name TeerexPrimitivesRequest (157) */
    interface TeerexPrimitivesRequest extends Struct {
        readonly shard: H256;
        readonly cyphertext: Bytes;
    }

    /** @name PalletClaimsCall (158) */
    interface PalletClaimsCall extends Enum {
        readonly isClaim: boolean;
        readonly asClaim: {
            readonly dest: AccountId32;
            readonly ethereumSignature: ClaimsPrimitivesEcdsaSignature;
        } & Struct;
        readonly isMintClaim: boolean;
        readonly asMintClaim: {
            readonly who: ClaimsPrimitivesEthereumAddress;
            readonly value: u128;
            readonly vestingSchedule: Option<ITuple<[u128, u128, u32]>>;
            readonly statement: Option<ClaimsPrimitivesStatementKind>;
        } & Struct;
        readonly isClaimAttest: boolean;
        readonly asClaimAttest: {
            readonly dest: AccountId32;
            readonly ethereumSignature: ClaimsPrimitivesEcdsaSignature;
            readonly statement: Bytes;
        } & Struct;
        readonly isAttest: boolean;
        readonly asAttest: {
            readonly statement: Bytes;
        } & Struct;
        readonly isMoveClaim: boolean;
        readonly asMoveClaim: {
            readonly old: ClaimsPrimitivesEthereumAddress;
            readonly new_: ClaimsPrimitivesEthereumAddress;
            readonly maybePreclaim: Option<AccountId32>;
        } & Struct;
        readonly type: 'Claim' | 'MintClaim' | 'ClaimAttest' | 'Attest' | 'MoveClaim';
    }

    /** @name ClaimsPrimitivesEcdsaSignature (159) */
    interface ClaimsPrimitivesEcdsaSignature extends U8aFixed {}

    /** @name ClaimsPrimitivesStatementKind (164) */
    interface ClaimsPrimitivesStatementKind extends Enum {
        readonly isRegular: boolean;
        readonly isSaft: boolean;
        readonly type: 'Regular' | 'Saft';
    }

    /** @name PalletTeeracleCall (165) */
    interface PalletTeeracleCall extends Enum {
        readonly isAddToWhitelist: boolean;
        readonly asAddToWhitelist: {
            readonly dataSource: Text;
            readonly mrenclave: U8aFixed;
        } & Struct;
        readonly isRemoveFromWhitelist: boolean;
        readonly asRemoveFromWhitelist: {
            readonly dataSource: Text;
            readonly mrenclave: U8aFixed;
        } & Struct;
        readonly isUpdateOracle: boolean;
        readonly asUpdateOracle: {
            readonly oracleName: Text;
            readonly dataSource: Text;
            readonly newBlob: Bytes;
        } & Struct;
        readonly isUpdateExchangeRate: boolean;
        readonly asUpdateExchangeRate: {
            readonly dataSource: Text;
            readonly tradingPair: Text;
            readonly newValue: Option<SubstrateFixedFixedU64>;
        } & Struct;
        readonly type: 'AddToWhitelist' | 'RemoveFromWhitelist' | 'UpdateOracle' | 'UpdateExchangeRate';
    }

    /** @name PalletSidechainCall (167) */
    interface PalletSidechainCall extends Enum {
        readonly isConfirmImportedSidechainBlock: boolean;
        readonly asConfirmImportedSidechainBlock: {
            readonly shardId: H256;
            readonly blockNumber: u64;
            readonly nextFinalizationCandidateBlockNumber: u64;
            readonly blockHeaderHash: H256;
        } & Struct;
        readonly type: 'ConfirmImportedSidechainBlock';
    }

    /** @name PalletIdentityManagementCall (168) */
    interface PalletIdentityManagementCall extends Enum {
        readonly isAddDelegatee: boolean;
        readonly asAddDelegatee: {
            readonly account: AccountId32;
        } & Struct;
        readonly isRemoveDelegatee: boolean;
        readonly asRemoveDelegatee: {
            readonly account: AccountId32;
        } & Struct;
        readonly isSetUserShieldingKey: boolean;
        readonly asSetUserShieldingKey: {
            readonly shard: H256;
            readonly encryptedKey: Bytes;
        } & Struct;
        readonly isCreateIdentity: boolean;
        readonly asCreateIdentity: {
            readonly shard: H256;
            readonly user: AccountId32;
            readonly encryptedIdentity: Bytes;
            readonly encryptedMetadata: Option<Bytes>;
        } & Struct;
        readonly isRemoveIdentity: boolean;
        readonly asRemoveIdentity: {
            readonly shard: H256;
            readonly encryptedIdentity: Bytes;
        } & Struct;
        readonly isVerifyIdentity: boolean;
        readonly asVerifyIdentity: {
            readonly shard: H256;
            readonly encryptedIdentity: Bytes;
            readonly encryptedValidationData: Bytes;
        } & Struct;
        readonly isUserShieldingKeySet: boolean;
        readonly asUserShieldingKeySet: {
            readonly account: AccountId32;
            readonly idGraph: CorePrimitivesKeyAesOutput;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isIdentityCreated: boolean;
        readonly asIdentityCreated: {
            readonly account: AccountId32;
            readonly identity: CorePrimitivesKeyAesOutput;
            readonly code: CorePrimitivesKeyAesOutput;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isIdentityRemoved: boolean;
        readonly asIdentityRemoved: {
            readonly account: AccountId32;
            readonly identity: CorePrimitivesKeyAesOutput;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isIdentityVerified: boolean;
        readonly asIdentityVerified: {
            readonly account: AccountId32;
            readonly identity: CorePrimitivesKeyAesOutput;
            readonly idGraph: CorePrimitivesKeyAesOutput;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isSomeError: boolean;
        readonly asSomeError: {
            readonly account: Option<AccountId32>;
            readonly error: CorePrimitivesErrorImpError;
            readonly reqExtHash: H256;
        } & Struct;
        readonly type:
            | 'AddDelegatee'
            | 'RemoveDelegatee'
            | 'SetUserShieldingKey'
            | 'CreateIdentity'
            | 'RemoveIdentity'
            | 'VerifyIdentity'
            | 'UserShieldingKeySet'
            | 'IdentityCreated'
            | 'IdentityRemoved'
            | 'IdentityVerified'
            | 'SomeError';
    }

    /** @name CorePrimitivesErrorImpError (169) */
    interface CorePrimitivesErrorImpError extends Enum {
        readonly isSetUserShieldingKeyFailed: boolean;
        readonly asSetUserShieldingKeyFailed: CorePrimitivesErrorErrorDetail;
        readonly isCreateIdentityFailed: boolean;
        readonly asCreateIdentityFailed: CorePrimitivesErrorErrorDetail;
        readonly isRemoveIdentityFailed: boolean;
        readonly asRemoveIdentityFailed: CorePrimitivesErrorErrorDetail;
        readonly isVerifyIdentityFailed: boolean;
        readonly asVerifyIdentityFailed: CorePrimitivesErrorErrorDetail;
        readonly isImportScheduledEnclaveFailed: boolean;
        readonly isUnclassifiedError: boolean;
        readonly asUnclassifiedError: CorePrimitivesErrorErrorDetail;
        readonly type:
            | 'SetUserShieldingKeyFailed'
            | 'CreateIdentityFailed'
            | 'RemoveIdentityFailed'
            | 'VerifyIdentityFailed'
            | 'ImportScheduledEnclaveFailed'
            | 'UnclassifiedError';
    }

    /** @name PalletVcManagementCall (170) */
    interface PalletVcManagementCall extends Enum {
        readonly isRequestVc: boolean;
        readonly asRequestVc: {
            readonly shard: H256;
            readonly assertion: CorePrimitivesAssertion;
        } & Struct;
        readonly isDisableVc: boolean;
        readonly asDisableVc: {
            readonly index: H256;
        } & Struct;
        readonly isRevokeVc: boolean;
        readonly asRevokeVc: {
            readonly index: H256;
        } & Struct;
        readonly isSetAdmin: boolean;
        readonly asSetAdmin: {
            readonly new_: AccountId32;
        } & Struct;
        readonly isAddSchema: boolean;
        readonly asAddSchema: {
            readonly shard: H256;
            readonly id: Bytes;
            readonly content: Bytes;
        } & Struct;
        readonly isDisableSchema: boolean;
        readonly asDisableSchema: {
            readonly shard: H256;
            readonly index: u64;
        } & Struct;
        readonly isActivateSchema: boolean;
        readonly asActivateSchema: {
            readonly shard: H256;
            readonly index: u64;
        } & Struct;
        readonly isRevokeSchema: boolean;
        readonly asRevokeSchema: {
            readonly shard: H256;
            readonly index: u64;
        } & Struct;
        readonly isAddVcRegistryItem: boolean;
        readonly asAddVcRegistryItem: {
            readonly index: H256;
            readonly subject: AccountId32;
            readonly assertion: CorePrimitivesAssertion;
            readonly hash_: H256;
        } & Struct;
        readonly isRemoveVcRegistryItem: boolean;
        readonly asRemoveVcRegistryItem: {
            readonly index: H256;
        } & Struct;
        readonly isClearVcRegistry: boolean;
        readonly isVcIssued: boolean;
        readonly asVcIssued: {
            readonly account: AccountId32;
            readonly assertion: CorePrimitivesAssertion;
            readonly index: H256;
            readonly hash_: H256;
            readonly vc: CorePrimitivesKeyAesOutput;
            readonly reqExtHash: H256;
        } & Struct;
        readonly isSomeError: boolean;
        readonly asSomeError: {
            readonly account: Option<AccountId32>;
            readonly error: CorePrimitivesErrorVcmpError;
            readonly reqExtHash: H256;
        } & Struct;
        readonly type:
            | 'RequestVc'
            | 'DisableVc'
            | 'RevokeVc'
            | 'SetAdmin'
            | 'AddSchema'
            | 'DisableSchema'
            | 'ActivateSchema'
            | 'RevokeSchema'
            | 'AddVcRegistryItem'
            | 'RemoveVcRegistryItem'
            | 'ClearVcRegistry'
            | 'VcIssued'
            | 'SomeError';
    }

    /** @name CorePrimitivesErrorVcmpError (171) */
    interface CorePrimitivesErrorVcmpError extends Enum {
        readonly isRequestVCFailed: boolean;
        readonly asRequestVCFailed: ITuple<[CorePrimitivesAssertion, CorePrimitivesErrorErrorDetail]>;
        readonly isUnclassifiedError: boolean;
        readonly asUnclassifiedError: CorePrimitivesErrorErrorDetail;
        readonly type: 'RequestVCFailed' | 'UnclassifiedError';
    }

    /** @name PalletGroupCall (172) */
    interface PalletGroupCall extends Enum {
        readonly isAddGroupMember: boolean;
        readonly asAddGroupMember: {
            readonly v: AccountId32;
        } & Struct;
        readonly isBatchAddGroupMembers: boolean;
        readonly asBatchAddGroupMembers: {
            readonly vs: Vec<AccountId32>;
        } & Struct;
        readonly isRemoveGroupMember: boolean;
        readonly asRemoveGroupMember: {
            readonly v: AccountId32;
        } & Struct;
        readonly isBatchRemoveGroupMembers: boolean;
        readonly asBatchRemoveGroupMembers: {
            readonly vs: Vec<AccountId32>;
        } & Struct;
        readonly isSwitchGroupControlOn: boolean;
        readonly isSwitchGroupControlOff: boolean;
        readonly type:
            | 'AddGroupMember'
            | 'BatchAddGroupMembers'
            | 'RemoveGroupMember'
            | 'BatchRemoveGroupMembers'
            | 'SwitchGroupControlOn'
            | 'SwitchGroupControlOff';
    }

    /** @name PalletSudoError (174) */
    interface PalletSudoError extends Enum {
        readonly isRequireSudo: boolean;
        readonly type: 'RequireSudo';
    }

    /** @name PalletMultisigMultisig (176) */
    interface PalletMultisigMultisig extends Struct {
        readonly when: PalletMultisigTimepoint;
        readonly deposit: u128;
        readonly depositor: AccountId32;
        readonly approvals: Vec<AccountId32>;
    }

    /** @name PalletMultisigError (178) */
    interface PalletMultisigError extends Enum {
        readonly isMinimumThreshold: boolean;
        readonly isAlreadyApproved: boolean;
        readonly isNoApprovalsNeeded: boolean;
        readonly isTooFewSignatories: boolean;
        readonly isTooManySignatories: boolean;
        readonly isSignatoriesOutOfOrder: boolean;
        readonly isSenderInSignatories: boolean;
        readonly isNotFound: boolean;
        readonly isNotOwner: boolean;
        readonly isNoTimepoint: boolean;
        readonly isWrongTimepoint: boolean;
        readonly isUnexpectedTimepoint: boolean;
        readonly isMaxWeightTooLow: boolean;
        readonly isAlreadyStored: boolean;
        readonly type:
            | 'MinimumThreshold'
            | 'AlreadyApproved'
            | 'NoApprovalsNeeded'
            | 'TooFewSignatories'
            | 'TooManySignatories'
            | 'SignatoriesOutOfOrder'
            | 'SenderInSignatories'
            | 'NotFound'
            | 'NotOwner'
            | 'NoTimepoint'
            | 'WrongTimepoint'
            | 'UnexpectedTimepoint'
            | 'MaxWeightTooLow'
            | 'AlreadyStored';
    }

    /** @name PalletProxyProxyDefinition (181) */
    interface PalletProxyProxyDefinition extends Struct {
        readonly delegate: AccountId32;
        readonly proxyType: IntegriteeNodeRuntimeProxyType;
        readonly delay: u32;
    }

    /** @name PalletProxyAnnouncement (185) */
    interface PalletProxyAnnouncement extends Struct {
        readonly real: AccountId32;
        readonly callHash: H256;
        readonly height: u32;
    }

    /** @name PalletProxyError (187) */
    interface PalletProxyError extends Enum {
        readonly isTooMany: boolean;
        readonly isNotFound: boolean;
        readonly isNotProxy: boolean;
        readonly isUnproxyable: boolean;
        readonly isDuplicate: boolean;
        readonly isNoPermission: boolean;
        readonly isUnannounced: boolean;
        readonly isNoSelfProxy: boolean;
        readonly type:
            | 'TooMany'
            | 'NotFound'
            | 'NotProxy'
            | 'Unproxyable'
            | 'Duplicate'
            | 'NoPermission'
            | 'Unannounced'
            | 'NoSelfProxy';
    }

    /** @name PalletSchedulerScheduled (190) */
    interface PalletSchedulerScheduled extends Struct {
        readonly maybeId: Option<U8aFixed>;
        readonly priority: u8;
        readonly call: FrameSupportPreimagesBounded;
        readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
        readonly origin: IntegriteeNodeRuntimeOriginCaller;
    }

    /** @name FrameSupportPreimagesBounded (191) */
    interface FrameSupportPreimagesBounded extends Enum {
        readonly isLegacy: boolean;
        readonly asLegacy: {
            readonly hash_: H256;
        } & Struct;
        readonly isInline: boolean;
        readonly asInline: Bytes;
        readonly isLookup: boolean;
        readonly asLookup: {
            readonly hash_: H256;
            readonly len: u32;
        } & Struct;
        readonly type: 'Legacy' | 'Inline' | 'Lookup';
    }

    /** @name PalletSchedulerError (194) */
    interface PalletSchedulerError extends Enum {
        readonly isFailedToSchedule: boolean;
        readonly isNotFound: boolean;
        readonly isTargetBlockNumberInPast: boolean;
        readonly isRescheduleNoChange: boolean;
        readonly isNamed: boolean;
        readonly type: 'FailedToSchedule' | 'NotFound' | 'TargetBlockNumberInPast' | 'RescheduleNoChange' | 'Named';
    }

    /** @name PalletUtilityError (195) */
    interface PalletUtilityError extends Enum {
        readonly isTooManyCalls: boolean;
        readonly type: 'TooManyCalls';
    }

    /** @name PalletBalancesBalanceLock (197) */
    interface PalletBalancesBalanceLock extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
        readonly reasons: PalletBalancesReasons;
    }

    /** @name PalletBalancesReasons (198) */
    interface PalletBalancesReasons extends Enum {
        readonly isFee: boolean;
        readonly isMisc: boolean;
        readonly isAll: boolean;
        readonly type: 'Fee' | 'Misc' | 'All';
    }

    /** @name PalletBalancesReserveData (201) */
    interface PalletBalancesReserveData extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
    }

    /** @name PalletBalancesError (203) */
    interface PalletBalancesError extends Enum {
        readonly isVestingBalance: boolean;
        readonly isLiquidityRestrictions: boolean;
        readonly isInsufficientBalance: boolean;
        readonly isExistentialDeposit: boolean;
        readonly isKeepAlive: boolean;
        readonly isExistingVestingSchedule: boolean;
        readonly isDeadAccount: boolean;
        readonly isTooManyReserves: boolean;
        readonly type:
            | 'VestingBalance'
            | 'LiquidityRestrictions'
            | 'InsufficientBalance'
            | 'ExistentialDeposit'
            | 'KeepAlive'
            | 'ExistingVestingSchedule'
            | 'DeadAccount'
            | 'TooManyReserves';
    }

    /** @name PalletTransactionPaymentReleases (205) */
    interface PalletTransactionPaymentReleases extends Enum {
        readonly isV1Ancient: boolean;
        readonly isV2: boolean;
        readonly type: 'V1Ancient' | 'V2';
    }

    /** @name PalletVestingReleases (208) */
    interface PalletVestingReleases extends Enum {
        readonly isV0: boolean;
        readonly isV1: boolean;
        readonly type: 'V0' | 'V1';
    }

    /** @name PalletVestingError (209) */
    interface PalletVestingError extends Enum {
        readonly isNotVesting: boolean;
        readonly isAtMaxVestingSchedules: boolean;
        readonly isAmountLow: boolean;
        readonly isScheduleIndexOutOfBounds: boolean;
        readonly isInvalidScheduleParams: boolean;
        readonly type:
            | 'NotVesting'
            | 'AtMaxVestingSchedules'
            | 'AmountLow'
            | 'ScheduleIndexOutOfBounds'
            | 'InvalidScheduleParams';
    }

    /** @name PalletGrandpaStoredState (210) */
    interface PalletGrandpaStoredState extends Enum {
        readonly isLive: boolean;
        readonly isPendingPause: boolean;
        readonly asPendingPause: {
            readonly scheduledAt: u32;
            readonly delay: u32;
        } & Struct;
        readonly isPaused: boolean;
        readonly isPendingResume: boolean;
        readonly asPendingResume: {
            readonly scheduledAt: u32;
            readonly delay: u32;
        } & Struct;
        readonly type: 'Live' | 'PendingPause' | 'Paused' | 'PendingResume';
    }

    /** @name PalletGrandpaStoredPendingChange (211) */
    interface PalletGrandpaStoredPendingChange extends Struct {
        readonly scheduledAt: u32;
        readonly delay: u32;
        readonly nextAuthorities: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
        readonly forced: Option<u32>;
    }

    /** @name PalletGrandpaError (213) */
    interface PalletGrandpaError extends Enum {
        readonly isPauseFailed: boolean;
        readonly isResumeFailed: boolean;
        readonly isChangePending: boolean;
        readonly isTooSoon: boolean;
        readonly isInvalidKeyOwnershipProof: boolean;
        readonly isInvalidEquivocationProof: boolean;
        readonly isDuplicateOffenceReport: boolean;
        readonly type:
            | 'PauseFailed'
            | 'ResumeFailed'
            | 'ChangePending'
            | 'TooSoon'
            | 'InvalidKeyOwnershipProof'
            | 'InvalidEquivocationProof'
            | 'DuplicateOffenceReport';
    }

    /** @name PalletCollectiveVotes (215) */
    interface PalletCollectiveVotes extends Struct {
        readonly index: u32;
        readonly threshold: u32;
        readonly ayes: Vec<AccountId32>;
        readonly nays: Vec<AccountId32>;
        readonly end: u32;
    }

    /** @name PalletCollectiveError (216) */
    interface PalletCollectiveError extends Enum {
        readonly isNotMember: boolean;
        readonly isDuplicateProposal: boolean;
        readonly isProposalMissing: boolean;
        readonly isWrongIndex: boolean;
        readonly isDuplicateVote: boolean;
        readonly isAlreadyInitialized: boolean;
        readonly isTooEarly: boolean;
        readonly isTooManyProposals: boolean;
        readonly isWrongProposalWeight: boolean;
        readonly isWrongProposalLength: boolean;
        readonly type:
            | 'NotMember'
            | 'DuplicateProposal'
            | 'ProposalMissing'
            | 'WrongIndex'
            | 'DuplicateVote'
            | 'AlreadyInitialized'
            | 'TooEarly'
            | 'TooManyProposals'
            | 'WrongProposalWeight'
            | 'WrongProposalLength';
    }

    /** @name PalletTreasuryProposal (217) */
    interface PalletTreasuryProposal extends Struct {
        readonly proposer: AccountId32;
        readonly value: u128;
        readonly beneficiary: AccountId32;
        readonly bond: u128;
    }

    /** @name FrameSupportPalletId (222) */
    interface FrameSupportPalletId extends U8aFixed {}

    /** @name PalletTreasuryError (223) */
    interface PalletTreasuryError extends Enum {
        readonly isInsufficientProposersBalance: boolean;
        readonly isInvalidIndex: boolean;
        readonly isTooManyApprovals: boolean;
        readonly isInsufficientPermission: boolean;
        readonly isProposalNotApproved: boolean;
        readonly type:
            | 'InsufficientProposersBalance'
            | 'InvalidIndex'
            | 'TooManyApprovals'
            | 'InsufficientPermission'
            | 'ProposalNotApproved';
    }

    /** @name TeerexPrimitivesEnclave (224) */
    interface TeerexPrimitivesEnclave extends Struct {
        readonly pubkey: AccountId32;
        readonly mrEnclave: U8aFixed;
        readonly timestamp: u64;
        readonly url: Bytes;
        readonly shieldingKey: Option<Bytes>;
        readonly vcPubkey: Option<Bytes>;
        readonly sgxMode: TeerexPrimitivesSgxBuildMode;
        readonly sgxMetadata: TeerexPrimitivesSgxEnclaveMetadata;
    }

    /** @name TeerexPrimitivesSgxBuildMode (225) */
    interface TeerexPrimitivesSgxBuildMode extends Enum {
        readonly isDebug: boolean;
        readonly isProduction: boolean;
        readonly type: 'Debug' | 'Production';
    }

    /** @name TeerexPrimitivesSgxEnclaveMetadata (226) */
    interface TeerexPrimitivesSgxEnclaveMetadata extends Struct {
        readonly quote: Bytes;
        readonly quoteSig: Bytes;
        readonly quoteCert: Bytes;
    }

    /** @name TeerexPrimitivesQuotingEnclave (227) */
    interface TeerexPrimitivesQuotingEnclave extends Struct {
        readonly issueDate: u64;
        readonly nextUpdate: u64;
        readonly miscselect: U8aFixed;
        readonly miscselectMask: U8aFixed;
        readonly attributes: U8aFixed;
        readonly attributesMask: U8aFixed;
        readonly mrsigner: U8aFixed;
        readonly isvprodid: u16;
        readonly tcb: Vec<TeerexPrimitivesQeTcb>;
    }

    /** @name TeerexPrimitivesQeTcb (230) */
    interface TeerexPrimitivesQeTcb extends Struct {
        readonly isvsvn: u16;
    }

    /** @name TeerexPrimitivesTcbInfoOnChain (232) */
    interface TeerexPrimitivesTcbInfoOnChain extends Struct {
        readonly issueDate: u64;
        readonly nextUpdate: u64;
        readonly tcbLevels: Vec<TeerexPrimitivesTcbVersionStatus>;
    }

    /** @name TeerexPrimitivesTcbVersionStatus (234) */
    interface TeerexPrimitivesTcbVersionStatus extends Struct {
        readonly cpusvn: U8aFixed;
        readonly pcesvn: u16;
    }

    /** @name PalletTeerexError (235) */
    interface PalletTeerexError extends Enum {
        readonly isRequireAdmin: boolean;
        readonly isEnclaveSignerDecodeError: boolean;
        readonly isSenderIsNotAttestedEnclave: boolean;
        readonly isRemoteAttestationVerificationFailed: boolean;
        readonly isRemoteAttestationTooOld: boolean;
        readonly isSgxModeNotAllowed: boolean;
        readonly isEnclaveIsNotRegistered: boolean;
        readonly isWrongMrenclaveForBondingAccount: boolean;
        readonly isWrongMrenclaveForShard: boolean;
        readonly isEnclaveUrlTooLong: boolean;
        readonly isRaReportTooLong: boolean;
        readonly isEmptyEnclaveRegistry: boolean;
        readonly isScheduledEnclaveNotExist: boolean;
        readonly isEnclaveNotInSchedule: boolean;
        readonly isCollateralInvalid: boolean;
        readonly isTooManyTopics: boolean;
        readonly isDataTooLong: boolean;
        readonly type:
            | 'RequireAdmin'
            | 'EnclaveSignerDecodeError'
            | 'SenderIsNotAttestedEnclave'
            | 'RemoteAttestationVerificationFailed'
            | 'RemoteAttestationTooOld'
            | 'SgxModeNotAllowed'
            | 'EnclaveIsNotRegistered'
            | 'WrongMrenclaveForBondingAccount'
            | 'WrongMrenclaveForShard'
            | 'EnclaveUrlTooLong'
            | 'RaReportTooLong'
            | 'EmptyEnclaveRegistry'
            | 'ScheduledEnclaveNotExist'
            | 'EnclaveNotInSchedule'
            | 'CollateralInvalid'
            | 'TooManyTopics'
            | 'DataTooLong';
    }

    /** @name PalletClaimsError (236) */
    interface PalletClaimsError extends Enum {
        readonly isInvalidEthereumSignature: boolean;
        readonly isSignerHasNoClaim: boolean;
        readonly isSenderHasNoClaim: boolean;
        readonly isPotUnderflow: boolean;
        readonly isInvalidStatement: boolean;
        readonly isVestedBalanceExists: boolean;
        readonly type:
            | 'InvalidEthereumSignature'
            | 'SignerHasNoClaim'
            | 'SenderHasNoClaim'
            | 'PotUnderflow'
            | 'InvalidStatement'
            | 'VestedBalanceExists';
    }

    /** @name PalletTeeracleError (240) */
    interface PalletTeeracleError extends Enum {
        readonly isInvalidCurrency: boolean;
        readonly isReleaseWhitelistOverflow: boolean;
        readonly isReleaseNotWhitelisted: boolean;
        readonly isReleaseAlreadyWhitelisted: boolean;
        readonly isTradingPairStringTooLong: boolean;
        readonly isOracleDataNameStringTooLong: boolean;
        readonly isDataSourceStringTooLong: boolean;
        readonly isOracleBlobTooBig: boolean;
        readonly type:
            | 'InvalidCurrency'
            | 'ReleaseWhitelistOverflow'
            | 'ReleaseNotWhitelisted'
            | 'ReleaseAlreadyWhitelisted'
            | 'TradingPairStringTooLong'
            | 'OracleDataNameStringTooLong'
            | 'DataSourceStringTooLong'
            | 'OracleBlobTooBig';
    }

    /** @name SidechainPrimitivesSidechainBlockConfirmation (241) */
    interface SidechainPrimitivesSidechainBlockConfirmation extends Struct {
        readonly blockNumber: u64;
        readonly blockHeaderHash: H256;
    }

    /** @name PalletSidechainError (242) */
    interface PalletSidechainError extends Enum {
        readonly isReceivedUnexpectedSidechainBlock: boolean;
        readonly isInvalidNextFinalizationCandidateBlockNumber: boolean;
        readonly type: 'ReceivedUnexpectedSidechainBlock' | 'InvalidNextFinalizationCandidateBlockNumber';
    }

    /** @name PalletIdentityManagementError (243) */
    interface PalletIdentityManagementError extends Enum {
        readonly isDelegateeNotExist: boolean;
        readonly isUnauthorisedUser: boolean;
        readonly type: 'DelegateeNotExist' | 'UnauthorisedUser';
    }

    /** @name PalletVcManagementVcContext (244) */
    interface PalletVcManagementVcContext extends Struct {
        readonly subject: AccountId32;
        readonly assertion: CorePrimitivesAssertion;
        readonly hash_: H256;
        readonly status: PalletVcManagementVcContextStatus;
    }

    /** @name PalletVcManagementVcContextStatus (245) */
    interface PalletVcManagementVcContextStatus extends Enum {
        readonly isActive: boolean;
        readonly isDisabled: boolean;
        readonly type: 'Active' | 'Disabled';
    }

    /** @name PalletVcManagementSchemaVcSchema (246) */
    interface PalletVcManagementSchemaVcSchema extends Struct {
        readonly id: Bytes;
        readonly author: AccountId32;
        readonly content: Bytes;
        readonly status: PalletVcManagementVcContextStatus;
    }

    /** @name PalletVcManagementError (249) */
    interface PalletVcManagementError extends Enum {
        readonly isVcAlreadyExists: boolean;
        readonly isVcNotExist: boolean;
        readonly isVcSubjectMismatch: boolean;
        readonly isVcAlreadyDisabled: boolean;
        readonly isRequireAdmin: boolean;
        readonly isSchemaNotExists: boolean;
        readonly isSchemaAlreadyDisabled: boolean;
        readonly isSchemaAlreadyActivated: boolean;
        readonly isSchemaIndexOverFlow: boolean;
        readonly isLengthMismatch: boolean;
        readonly type:
            | 'VcAlreadyExists'
            | 'VcNotExist'
            | 'VcSubjectMismatch'
            | 'VcAlreadyDisabled'
            | 'RequireAdmin'
            | 'SchemaNotExists'
            | 'SchemaAlreadyDisabled'
            | 'SchemaAlreadyActivated'
            | 'SchemaIndexOverFlow'
            | 'LengthMismatch';
    }

    /** @name PalletGroupError (250) */
    interface PalletGroupError extends Enum {
        readonly isGroupMemberAlreadyExists: boolean;
        readonly isGroupMemberInvalid: boolean;
        readonly type: 'GroupMemberAlreadyExists' | 'GroupMemberInvalid';
    }

    /** @name SpRuntimeMultiSignature (253) */
    interface SpRuntimeMultiSignature extends Enum {
        readonly isEd25519: boolean;
        readonly asEd25519: SpCoreEd25519Signature;
        readonly isSr25519: boolean;
        readonly asSr25519: SpCoreSr25519Signature;
        readonly isEcdsa: boolean;
        readonly asEcdsa: SpCoreEcdsaSignature;
        readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
    }

    /** @name SpCoreSr25519Signature (254) */
    interface SpCoreSr25519Signature extends U8aFixed {}

    /** @name SpCoreEcdsaSignature (255) */
    interface SpCoreEcdsaSignature extends U8aFixed {}

    /** @name FrameSystemExtensionsCheckNonZeroSender (257) */
    type FrameSystemExtensionsCheckNonZeroSender = Null;

    /** @name FrameSystemExtensionsCheckSpecVersion (258) */
    type FrameSystemExtensionsCheckSpecVersion = Null;

    /** @name FrameSystemExtensionsCheckTxVersion (259) */
    type FrameSystemExtensionsCheckTxVersion = Null;

    /** @name FrameSystemExtensionsCheckGenesis (260) */
    type FrameSystemExtensionsCheckGenesis = Null;

    /** @name FrameSystemExtensionsCheckNonce (263) */
    interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

    /** @name FrameSystemExtensionsCheckWeight (264) */
    type FrameSystemExtensionsCheckWeight = Null;

    /** @name PalletTransactionPaymentChargeTransactionPayment (265) */
    interface PalletTransactionPaymentChargeTransactionPayment extends Compact<u128> {}

    /** @name IntegriteeNodeRuntimeRuntime (266) */
    type IntegriteeNodeRuntimeRuntime = Null;
} // declare module
