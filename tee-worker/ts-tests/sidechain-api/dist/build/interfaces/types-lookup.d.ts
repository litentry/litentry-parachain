import "@polkadot/types/lookup";
import type { Bytes, Compact, Enum, Null, Option, Result, Struct, Text, U8aFixed, Vec, bool, u128, u32, u64, u8 } from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, Call, H256, MultiAddress } from "@polkadot/types/interfaces/runtime";
import type { Event } from "@polkadot/types/interfaces/system";
declare module "@polkadot/types/lookup" {
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
        readonly type: "Other" | "Consensus" | "Seal" | "PreRuntime" | "RuntimeEnvironmentUpdated";
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
        readonly type: "ExtrinsicSuccess" | "ExtrinsicFailed" | "CodeUpdated" | "NewAccount" | "KilledAccount" | "Remarked";
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
        readonly type: "Normal" | "Operational" | "Mandatory";
    }
    /** @name FrameSupportDispatchPays (23) */
    interface FrameSupportDispatchPays extends Enum {
        readonly isYes: boolean;
        readonly isNo: boolean;
        readonly type: "Yes" | "No";
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
        readonly type: "Other" | "CannotLookup" | "BadOrigin" | "Module" | "ConsumerRemaining" | "NoProviders" | "TooManyConsumers" | "Token" | "Arithmetic" | "Transactional" | "Exhausted" | "Corruption" | "Unavailable";
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
        readonly type: "NoFunds" | "WouldDie" | "BelowMinimum" | "CannotCreate" | "UnknownAsset" | "Frozen" | "Unsupported";
    }
    /** @name SpArithmeticArithmeticError (27) */
    interface SpArithmeticArithmeticError extends Enum {
        readonly isUnderflow: boolean;
        readonly isOverflow: boolean;
        readonly isDivisionByZero: boolean;
        readonly type: "Underflow" | "Overflow" | "DivisionByZero";
    }
    /** @name SpRuntimeTransactionalError (28) */
    interface SpRuntimeTransactionalError extends Enum {
        readonly isLimitReached: boolean;
        readonly isNoLayer: boolean;
        readonly type: "LimitReached" | "NoLayer";
    }
    /** @name PalletBalancesEvent (29) */
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
        readonly type: "Endowed" | "DustLost" | "Transfer" | "BalanceSet" | "Reserved" | "Unreserved" | "ReserveRepatriated" | "Deposit" | "Withdraw" | "Slashed";
    }
    /** @name FrameSupportTokensMiscBalanceStatus (30) */
    interface FrameSupportTokensMiscBalanceStatus extends Enum {
        readonly isFree: boolean;
        readonly isReserved: boolean;
        readonly type: "Free" | "Reserved";
    }
    /** @name PalletTransactionPaymentEvent (31) */
    interface PalletTransactionPaymentEvent extends Enum {
        readonly isTransactionFeePaid: boolean;
        readonly asTransactionFeePaid: {
            readonly who: AccountId32;
            readonly actualFee: u128;
            readonly tip: u128;
        } & Struct;
        readonly type: "TransactionFeePaid";
    }
    /** @name PalletSudoEvent (32) */
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
        readonly type: "Sudid" | "KeyChanged" | "SudoAsDone";
    }
    /** @name PalletIdentityManagementTeeEvent (36) */
    interface PalletIdentityManagementTeeEvent extends Enum {
        readonly isUserShieldingKeySet: boolean;
        readonly asUserShieldingKeySet: {
            readonly who: LitentryPrimitivesIdentity;
            readonly key: U8aFixed;
        } & Struct;
        readonly isIdentityLinked: boolean;
        readonly asIdentityLinked: {
            readonly who: LitentryPrimitivesIdentity;
            readonly identity: LitentryPrimitivesIdentity;
        } & Struct;
        readonly isIdentityDeactivated: boolean;
        readonly asIdentityDeactivated: {
            readonly who: LitentryPrimitivesIdentity;
            readonly identity: LitentryPrimitivesIdentity;
        } & Struct;
        readonly isIdentityActivated: boolean;
        readonly asIdentityActivated: {
            readonly who: LitentryPrimitivesIdentity;
            readonly identity: LitentryPrimitivesIdentity;
        } & Struct;
        readonly type: "UserShieldingKeySet" | "IdentityLinked" | "IdentityDeactivated" | "IdentityActivated";
    }
    /** @name LitentryPrimitivesIdentity (37) */
    interface LitentryPrimitivesIdentity extends Enum {
        readonly isTwitter: boolean;
        readonly asTwitter: Bytes;
        readonly isDiscord: boolean;
        readonly asDiscord: Bytes;
        readonly isGithub: boolean;
        readonly asGithub: Bytes;
        readonly isSubstrate: boolean;
        readonly asSubstrate: LitentryPrimitivesIdentityAddress32;
        readonly isEvm: boolean;
        readonly asEvm: LitentryPrimitivesIdentityAddress20;
        readonly type: "Twitter" | "Discord" | "Github" | "Substrate" | "Evm";
    }
    /** @name LitentryPrimitivesIdentityAddress32 (39) */
    interface LitentryPrimitivesIdentityAddress32 extends U8aFixed {
    }
    /** @name LitentryPrimitivesIdentityAddress20 (40) */
    interface LitentryPrimitivesIdentityAddress20 extends U8aFixed {
    }
    /** @name FrameSystemPhase (42) */
    interface FrameSystemPhase extends Enum {
        readonly isApplyExtrinsic: boolean;
        readonly asApplyExtrinsic: u32;
        readonly isFinalization: boolean;
        readonly isInitialization: boolean;
        readonly type: "ApplyExtrinsic" | "Finalization" | "Initialization";
    }
    /** @name FrameSystemLastRuntimeUpgradeInfo (46) */
    interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
        readonly specVersion: Compact<u32>;
        readonly specName: Text;
    }
    /** @name FrameSystemCall (50) */
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
        readonly type: "Remark" | "SetHeapPages" | "SetCode" | "SetCodeWithoutChecks" | "SetStorage" | "KillStorage" | "KillPrefix" | "RemarkWithEvent";
    }
    /** @name FrameSystemLimitsBlockWeights (54) */
    interface FrameSystemLimitsBlockWeights extends Struct {
        readonly baseBlock: SpWeightsWeightV2Weight;
        readonly maxBlock: SpWeightsWeightV2Weight;
        readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    }
    /** @name FrameSupportDispatchPerDispatchClassWeightsPerClass (55) */
    interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
        readonly normal: FrameSystemLimitsWeightsPerClass;
        readonly operational: FrameSystemLimitsWeightsPerClass;
        readonly mandatory: FrameSystemLimitsWeightsPerClass;
    }
    /** @name FrameSystemLimitsWeightsPerClass (56) */
    interface FrameSystemLimitsWeightsPerClass extends Struct {
        readonly baseExtrinsic: SpWeightsWeightV2Weight;
        readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
        readonly maxTotal: Option<SpWeightsWeightV2Weight>;
        readonly reserved: Option<SpWeightsWeightV2Weight>;
    }
    /** @name FrameSystemLimitsBlockLength (58) */
    interface FrameSystemLimitsBlockLength extends Struct {
        readonly max: FrameSupportDispatchPerDispatchClassU32;
    }
    /** @name FrameSupportDispatchPerDispatchClassU32 (59) */
    interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
        readonly normal: u32;
        readonly operational: u32;
        readonly mandatory: u32;
    }
    /** @name SpWeightsRuntimeDbWeight (60) */
    interface SpWeightsRuntimeDbWeight extends Struct {
        readonly read: u64;
        readonly write: u64;
    }
    /** @name SpVersionRuntimeVersion (61) */
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
    /** @name FrameSystemError (67) */
    interface FrameSystemError extends Enum {
        readonly isInvalidSpecName: boolean;
        readonly isSpecVersionNeedsToIncrease: boolean;
        readonly isFailedToExtractRuntimeVersion: boolean;
        readonly isNonDefaultComposite: boolean;
        readonly isNonZeroRefCount: boolean;
        readonly isCallFiltered: boolean;
        readonly type: "InvalidSpecName" | "SpecVersionNeedsToIncrease" | "FailedToExtractRuntimeVersion" | "NonDefaultComposite" | "NonZeroRefCount" | "CallFiltered";
    }
    /** @name PalletTimestampCall (68) */
    interface PalletTimestampCall extends Enum {
        readonly isSet: boolean;
        readonly asSet: {
            readonly now: Compact<u64>;
        } & Struct;
        readonly type: "Set";
    }
    /** @name PalletBalancesBalanceLock (70) */
    interface PalletBalancesBalanceLock extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
        readonly reasons: PalletBalancesReasons;
    }
    /** @name PalletBalancesReasons (71) */
    interface PalletBalancesReasons extends Enum {
        readonly isFee: boolean;
        readonly isMisc: boolean;
        readonly isAll: boolean;
        readonly type: "Fee" | "Misc" | "All";
    }
    /** @name PalletBalancesReserveData (74) */
    interface PalletBalancesReserveData extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
    }
    /** @name PalletBalancesCall (76) */
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
        readonly type: "Transfer" | "SetBalance" | "ForceTransfer" | "TransferKeepAlive" | "TransferAll" | "ForceUnreserve";
    }
    /** @name PalletBalancesError (80) */
    interface PalletBalancesError extends Enum {
        readonly isVestingBalance: boolean;
        readonly isLiquidityRestrictions: boolean;
        readonly isInsufficientBalance: boolean;
        readonly isExistentialDeposit: boolean;
        readonly isKeepAlive: boolean;
        readonly isExistingVestingSchedule: boolean;
        readonly isDeadAccount: boolean;
        readonly isTooManyReserves: boolean;
        readonly type: "VestingBalance" | "LiquidityRestrictions" | "InsufficientBalance" | "ExistentialDeposit" | "KeepAlive" | "ExistingVestingSchedule" | "DeadAccount" | "TooManyReserves";
    }
    /** @name PalletTransactionPaymentReleases (82) */
    interface PalletTransactionPaymentReleases extends Enum {
        readonly isV1Ancient: boolean;
        readonly isV2: boolean;
        readonly type: "V1Ancient" | "V2";
    }
    /** @name PalletSudoCall (83) */
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
        readonly type: "Sudo" | "SudoUncheckedWeight" | "SetKey" | "SudoAs";
    }
    /** @name PalletParentchainCall (85) */
    interface PalletParentchainCall extends Enum {
        readonly isSetBlock: boolean;
        readonly asSetBlock: {
            readonly header: SpRuntimeHeader;
        } & Struct;
        readonly type: "SetBlock";
    }
    /** @name SpRuntimeHeader (86) */
    interface SpRuntimeHeader extends Struct {
        readonly parentHash: H256;
        readonly number: Compact<u32>;
        readonly stateRoot: H256;
        readonly extrinsicsRoot: H256;
        readonly digest: SpRuntimeDigest;
    }
    /** @name SpRuntimeBlakeTwo256 (87) */
    type SpRuntimeBlakeTwo256 = Null;
    /** @name PalletIdentityManagementTeeCall (88) */
    interface PalletIdentityManagementTeeCall extends Enum {
        readonly isSetUserShieldingKey: boolean;
        readonly asSetUserShieldingKey: {
            readonly who: LitentryPrimitivesIdentity;
            readonly key: U8aFixed;
        } & Struct;
        readonly isLinkIdentity: boolean;
        readonly asLinkIdentity: {
            readonly who: LitentryPrimitivesIdentity;
            readonly identity: LitentryPrimitivesIdentity;
            readonly web3networks: Vec<CorePrimitivesNetworkWeb3Network>;
        } & Struct;
        readonly isDeactivateIdentity: boolean;
        readonly asDeactivateIdentity: {
            readonly who: LitentryPrimitivesIdentity;
            readonly identity: LitentryPrimitivesIdentity;
        } & Struct;
        readonly isActivateIdentity: boolean;
        readonly asActivateIdentity: {
            readonly who: LitentryPrimitivesIdentity;
            readonly identity: LitentryPrimitivesIdentity;
        } & Struct;
        readonly isSetIdentityNetworks: boolean;
        readonly asSetIdentityNetworks: {
            readonly who: LitentryPrimitivesIdentity;
            readonly identity: LitentryPrimitivesIdentity;
            readonly web3networks: Vec<CorePrimitivesNetworkWeb3Network>;
        } & Struct;
        readonly type: "SetUserShieldingKey" | "LinkIdentity" | "DeactivateIdentity" | "ActivateIdentity" | "SetIdentityNetworks";
    }
    /** @name CorePrimitivesNetworkWeb3Network (90) */
    interface CorePrimitivesNetworkWeb3Network extends Enum {
        readonly isPolkadot: boolean;
        readonly isKusama: boolean;
        readonly isLitentry: boolean;
        readonly isLitmus: boolean;
        readonly isLitentryRococo: boolean;
        readonly isKhala: boolean;
        readonly isSubstrateTestnet: boolean;
        readonly isEthereum: boolean;
        readonly isPolygon: boolean;
        readonly isBsc: boolean;
        readonly type: "Polkadot" | "Kusama" | "Litentry" | "Litmus" | "LitentryRococo" | "Khala" | "SubstrateTestnet" | "Ethereum" | "Polygon" | "Bsc";
    }
    /** @name PalletSudoError (91) */
    interface PalletSudoError extends Enum {
        readonly isRequireSudo: boolean;
        readonly type: "RequireSudo";
    }
    /** @name PalletIdentityManagementTeeIdentityContext (93) */
    interface PalletIdentityManagementTeeIdentityContext extends Struct {
        readonly linkBlock: u32;
        readonly web3networks: Vec<CorePrimitivesNetworkWeb3Network>;
        readonly status: PalletIdentityManagementTeeIdentityContextIdentityStatus;
    }
    /** @name PalletIdentityManagementTeeIdentityContextIdentityStatus (94) */
    interface PalletIdentityManagementTeeIdentityContextIdentityStatus extends Enum {
        readonly isActive: boolean;
        readonly isInactive: boolean;
        readonly type: "Active" | "Inactive";
    }
    /** @name PalletIdentityManagementTeeError (95) */
    interface PalletIdentityManagementTeeError extends Enum {
        readonly isIdentityAlreadyLinked: boolean;
        readonly isIdentityNotExist: boolean;
        readonly isLinkPrimeIdentityDisallowed: boolean;
        readonly isDeactivatePrimeIdentityDisallowed: boolean;
        readonly isIdGraphLenLimitReached: boolean;
        readonly isWrongWeb3NetworkTypes: boolean;
        readonly isNotSupportedIdentity: boolean;
        readonly type: "IdentityAlreadyLinked" | "IdentityNotExist" | "LinkPrimeIdentityDisallowed" | "DeactivatePrimeIdentityDisallowed" | "IdGraphLenLimitReached" | "WrongWeb3NetworkTypes" | "NotSupportedIdentity";
    }
    /** @name SpRuntimeMultiSignature (97) */
    interface SpRuntimeMultiSignature extends Enum {
        readonly isEd25519: boolean;
        readonly asEd25519: SpCoreEd25519Signature;
        readonly isSr25519: boolean;
        readonly asSr25519: SpCoreSr25519Signature;
        readonly isEcdsa: boolean;
        readonly asEcdsa: SpCoreEcdsaSignature;
        readonly type: "Ed25519" | "Sr25519" | "Ecdsa";
    }
    /** @name SpCoreEd25519Signature (98) */
    interface SpCoreEd25519Signature extends U8aFixed {
    }
    /** @name SpCoreSr25519Signature (100) */
    interface SpCoreSr25519Signature extends U8aFixed {
    }
    /** @name SpCoreEcdsaSignature (101) */
    interface SpCoreEcdsaSignature extends U8aFixed {
    }
    /** @name FrameSystemExtensionsCheckNonZeroSender (104) */
    type FrameSystemExtensionsCheckNonZeroSender = Null;
    /** @name FrameSystemExtensionsCheckSpecVersion (105) */
    type FrameSystemExtensionsCheckSpecVersion = Null;
    /** @name FrameSystemExtensionsCheckTxVersion (106) */
    type FrameSystemExtensionsCheckTxVersion = Null;
    /** @name FrameSystemExtensionsCheckGenesis (107) */
    type FrameSystemExtensionsCheckGenesis = Null;
    /** @name FrameSystemExtensionsCheckNonce (110) */
    interface FrameSystemExtensionsCheckNonce extends Compact<u32> {
    }
    /** @name FrameSystemExtensionsCheckWeight (111) */
    type FrameSystemExtensionsCheckWeight = Null;
    /** @name PalletTransactionPaymentChargeTransactionPayment (112) */
    interface PalletTransactionPaymentChargeTransactionPayment extends Compact<u128> {
    }
    /** @name ItaSgxRuntimeRuntime (113) */
    type ItaSgxRuntimeRuntime = Null;
}
//# sourceMappingURL=types-lookup.d.ts.map