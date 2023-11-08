import type { Bytes, Enum, Option, Struct, Text, U8aFixed, Vec, bool, u32 } from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type { Signature } from "@polkadot/types/interfaces/extrinsics";
import type { AccountId, AccountId32, Balance, BlockNumber, H256, Index } from "@polkadot/types/interfaces/runtime";
/** @name Address20 */
export interface Address20 extends U8aFixed {
}
/** @name Address32 */
export interface Address32 extends U8aFixed {
}
/** @name AesOutput */
export interface AesOutput extends Struct {
    readonly ciphertext: Bytes;
    readonly aad: Bytes;
    readonly nonce: U8aFixed;
}
/** @name AesRequest */
export interface AesRequest extends Struct {
    readonly shard: ShardIdentifier;
    readonly key: Bytes;
    readonly payload: AesOutput;
}
/** @name Assertion */
export interface Assertion extends Enum {
    readonly isA1: boolean;
    readonly isA2: boolean;
    readonly asA2: Bytes;
    readonly isA3: boolean;
    readonly asA3: ITuple<[Bytes, Bytes, Bytes]>;
    readonly isA4: boolean;
    readonly asA4: Bytes;
    readonly isA6: boolean;
    readonly isA7: boolean;
    readonly asA7: Bytes;
    readonly isA8: boolean;
    readonly asA8: Vec<AssertionSupportedNetwork>;
    readonly isA10: boolean;
    readonly asA10: Bytes;
    readonly isA11: boolean;
    readonly asA11: Bytes;
    readonly isA12: boolean;
    readonly asA12: Bytes;
    readonly isA13: boolean;
    readonly asA13: AccountId32;
    readonly isA14: boolean;
    readonly type: "A1" | "A2" | "A3" | "A4" | "A6" | "A7" | "A8" | "A10" | "A11" | "A12" | "A13" | "A14";
}
/** @name AssertionSupportedNetwork */
export interface AssertionSupportedNetwork extends Enum {
    readonly isLitentry: boolean;
    readonly isLitmus: boolean;
    readonly isLitentryRococo: boolean;
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly isKhala: boolean;
    readonly isEthereum: boolean;
    readonly isTestNet: boolean;
    readonly type: "Litentry" | "Litmus" | "LitentryRococo" | "Polkadot" | "Kusama" | "Khala" | "Ethereum" | "TestNet";
}
/** @name BoundedWeb3Network */
export interface BoundedWeb3Network extends Vec<Web3Network> {
}
/** @name DirectRequestStatus */
export interface DirectRequestStatus extends Enum {
    readonly isOk: boolean;
    readonly isTrustedOperationStatus: boolean;
    readonly asTrustedOperationStatus: ITuple<[TrustedOperationStatus, H256]>;
    readonly isError: boolean;
    readonly type: "Ok" | "TrustedOperationStatus" | "Error";
}
/** @name DiscordValidationData */
export interface DiscordValidationData extends Struct {
    readonly channel_id: Bytes;
    readonly message_id: Bytes;
    readonly guild_id: Bytes;
}
/** @name ErrorDetail */
export interface ErrorDetail extends Enum {
    readonly isImportError: boolean;
    readonly isUnauthorizedSigner: boolean;
    readonly isStfError: boolean;
    readonly asStfError: Bytes;
    readonly isSendStfRequestFailed: boolean;
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
    readonly isWeb3NetworkOutOfBounds: boolean;
    readonly type: "ImportError" | "UnauthorizedSigner" | "StfError" | "SendStfRequestFailed" | "UserShieldingKeyNotFound" | "ParseError" | "DataProviderError" | "InvalidIdentity" | "WrongWeb2Handle" | "UnexpectedMessage" | "WrongSignatureType" | "VerifySubstrateSignatureFailed" | "VerifyEvmSignatureFailed" | "RecoverEvmAddressFailed" | "Web3NetworkOutOfBounds";
}
/** @name EthereumSignature */
export interface EthereumSignature extends U8aFixed {
}
/** @name GenericEventWithAccount */
export interface GenericEventWithAccount extends Struct {
    readonly account: AccountId;
}
/** @name Getter */
export interface Getter extends Enum {
    readonly isPublic: boolean;
    readonly asPublic: PublicGetter;
    readonly isTrusted: boolean;
    readonly asTrusted: TrustedGetterSigned;
    readonly type: "Public" | "Trusted";
}
/** @name IdentityContext */
export interface IdentityContext extends Struct {
    readonly link_block: BlockNumber;
    readonly web3networks: BoundedWeb3Network;
    readonly status: IdentityStatus;
}
/** @name IdentityGenericEvent */
export interface IdentityGenericEvent extends Struct {
    readonly who: AccountId;
    readonly identity: LitentryIdentity;
    readonly id_graph: Vec<ITuple<[LitentryIdentity, IdentityContext]>>;
}
/** @name IdentityStatus */
export interface IdentityStatus extends Enum {
    readonly isActive: boolean;
    readonly isInactive: boolean;
    readonly type: "Active" | "Inactive";
}
/** @name IdentityString */
export interface IdentityString extends Bytes {
}
/** @name LinkIdentityResult */
export interface LinkIdentityResult extends Struct {
    readonly id_graph: AesOutput;
}
/** @name LitentryIdentity */
export interface LitentryIdentity extends Enum {
    readonly isTwitter: boolean;
    readonly asTwitter: IdentityString;
    readonly isDiscord: boolean;
    readonly asDiscord: IdentityString;
    readonly isGithub: boolean;
    readonly asGithub: IdentityString;
    readonly isSubstrate: boolean;
    readonly asSubstrate: Address32;
    readonly isEvm: boolean;
    readonly asEvm: Address20;
    readonly type: "Twitter" | "Discord" | "Github" | "Substrate" | "Evm";
}
/** @name LitentryMultiSignature */
export interface LitentryMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: Signature;
    readonly isEthereum: boolean;
    readonly asEthereum: EthereumSignature;
    readonly isEthereumPrettified: boolean;
    readonly asEthereumPrettified: EthereumSignature;
    readonly type: "Ed25519" | "Sr25519" | "Ecdsa" | "Ethereum" | "EthereumPrettified";
}
/** @name LitentryValidationData */
export interface LitentryValidationData extends Enum {
    readonly isWeb2Validation: boolean;
    readonly asWeb2Validation: Web2ValidationData;
    readonly isWeb3Validation: boolean;
    readonly asWeb3Validation: Web3ValidationData;
    readonly type: "Web2Validation" | "Web3Validation";
}
/** @name PublicGetter */
export interface PublicGetter extends Enum {
    readonly isSomeValue: boolean;
    readonly asSomeValue: u32;
    readonly isNonce: boolean;
    readonly asNonce: LitentryIdentity;
    readonly type: "SomeValue" | "Nonce";
}
/** @name RequestVCResult */
export interface RequestVCResult extends Struct {
    readonly vc_index: H256;
    readonly vc_hash: H256;
    readonly vc_payload: AesOutput;
}
/** @name RsaRequest */
export interface RsaRequest extends Struct {
    readonly shard: ShardIdentifier;
    readonly payload: Bytes;
}
/** @name SetUserShieldingKeyResult */
export interface SetUserShieldingKeyResult extends Struct {
    readonly id_graph: AesOutput;
}
/** @name ShardIdentifier */
export interface ShardIdentifier extends H256 {
}
/** @name StfError */
export interface StfError extends Enum {
    readonly isMissingPrivileges: boolean;
    readonly asMissingPrivileges: LitentryIdentity;
    readonly isRequireEnclaveSignerAccount: boolean;
    readonly isDispatch: boolean;
    readonly asDispatch: Text;
    readonly isMissingFunds: boolean;
    readonly isInvalidNonce: boolean;
    readonly asInvalidNonce: ITuple<[Index, Index]>;
    readonly isStorageHashMismatch: boolean;
    readonly isInvalidStorageDiff: boolean;
    readonly isInvalidMetadata: boolean;
    readonly isSetUserShieldingKeyFailed: boolean;
    readonly asSetUserShieldingKeyFailed: ErrorDetail;
    readonly isLinkIdentityFailed: boolean;
    readonly asLinkIdentityFailed: ErrorDetail;
    readonly isDeactivateIdentityFailed: boolean;
    readonly asDeactivateIdentityFailed: ErrorDetail;
    readonly isActivateIdentityFailed: boolean;
    readonly asActivateIdentityFailed: ErrorDetail;
    readonly isRequestVCFailed: boolean;
    readonly asRequestVCFailed: ITuple<[Assertion, ErrorDetail]>;
    readonly isSetScheduledMrEnclaveFailed: boolean;
    readonly isSetIdentityNetworksFailed: boolean;
    readonly asSetIdentityNetworksFailed: ErrorDetail;
    readonly isInvalidAccount: boolean;
    readonly isUnclassifiedError: boolean;
    readonly type: "MissingPrivileges" | "RequireEnclaveSignerAccount" | "Dispatch" | "MissingFunds" | "InvalidNonce" | "StorageHashMismatch" | "InvalidStorageDiff" | "InvalidMetadata" | "SetUserShieldingKeyFailed" | "LinkIdentityFailed" | "DeactivateIdentityFailed" | "ActivateIdentityFailed" | "RequestVCFailed" | "SetScheduledMrEnclaveFailed" | "SetIdentityNetworksFailed" | "InvalidAccount" | "UnclassifiedError";
}
/** @name TrustedCall */
export interface TrustedCall extends Enum {
    readonly isBalanceSetBalance: boolean;
    readonly asBalanceSetBalance: ITuple<[LitentryIdentity, LitentryIdentity, Balance, Balance]>;
    readonly isBalanceTransfer: boolean;
    readonly asBalanceTransfer: ITuple<[LitentryIdentity, LitentryIdentity, Balance]>;
    readonly isBalanceUnshield: boolean;
    readonly asBalanceUnshield: ITuple<[LitentryIdentity, LitentryIdentity, Balance, ShardIdentifier]>;
    readonly isBalanceShield: boolean;
    readonly asBalanceShield: ITuple<[LitentryIdentity, LitentryIdentity, Balance]>;
    readonly isSetUserShieldingKey: boolean;
    readonly asSetUserShieldingKey: ITuple<[LitentryIdentity, LitentryIdentity, UserShieldingKeyType, H256]>;
    readonly isLinkIdentity: boolean;
    readonly asLinkIdentity: ITuple<[
        LitentryIdentity,
        LitentryIdentity,
        LitentryIdentity,
        LitentryValidationData,
        Vec<Web3Network>,
        UserShieldingKeyNonceType,
        Option<UserShieldingKeyType>,
        H256
    ]>;
    readonly isDeactivateIdentity: boolean;
    readonly asDeactivateIdentity: ITuple<[LitentryIdentity, LitentryIdentity, LitentryIdentity, H256]>;
    readonly isActivateIdentity: boolean;
    readonly asActivateIdentity: ITuple<[LitentryIdentity, LitentryIdentity, LitentryIdentity, H256]>;
    readonly isRequestVc: boolean;
    readonly asRequestVc: ITuple<[LitentryIdentity, LitentryIdentity, Assertion, Option<UserShieldingKeyType>, H256]>;
    readonly isSetIdentityNetworks: boolean;
    readonly asSetIdentityNetworks: ITuple<[LitentryIdentity, LitentryIdentity, LitentryIdentity, Vec<Web3Network>, H256]>;
    readonly isSetUserShieldingKeyWithNetworks: boolean;
    readonly asSetUserShieldingKeyWithNetworks: ITuple<[LitentryIdentity, LitentryIdentity, UserShieldingKeyType, Vec<Web3Network>, H256]>;
    readonly type: "BalanceSetBalance" | "BalanceTransfer" | "BalanceUnshield" | "BalanceShield" | "SetUserShieldingKey" | "LinkIdentity" | "DeactivateIdentity" | "ActivateIdentity" | "RequestVc" | "SetIdentityNetworks" | "SetUserShieldingKeyWithNetworks";
}
/** @name TrustedCallSigned */
export interface TrustedCallSigned extends Struct {
    readonly call: TrustedCall;
    readonly index: u32;
    readonly signature: LitentryMultiSignature;
}
/** @name TrustedGetter */
export interface TrustedGetter extends Enum {
    readonly isFreeBalance: boolean;
    readonly asFreeBalance: LitentryIdentity;
    readonly isReservedBalance: boolean;
    readonly asReservedBalance: LitentryIdentity;
    readonly isUserShieldingKey: boolean;
    readonly asUserShieldingKey: LitentryIdentity;
    readonly isIdGraph: boolean;
    readonly asIdGraph: LitentryIdentity;
    readonly isIdGraphStats: boolean;
    readonly asIdGraphStats: LitentryIdentity;
    readonly type: "FreeBalance" | "ReservedBalance" | "UserShieldingKey" | "IdGraph" | "IdGraphStats";
}
/** @name TrustedGetterSigned */
export interface TrustedGetterSigned extends Struct {
    readonly getter: TrustedGetter;
    readonly signature: LitentryMultiSignature;
}
/** @name TrustedOperation */
export interface TrustedOperation extends Enum {
    readonly isIndirectCall: boolean;
    readonly asIndirectCall: TrustedCallSigned;
    readonly isDirectCall: boolean;
    readonly asDirectCall: TrustedCallSigned;
    readonly isGet: boolean;
    readonly asGet: Getter;
    readonly type: "IndirectCall" | "DirectCall" | "Get";
}
/** @name TrustedOperationStatus */
export interface TrustedOperationStatus extends Enum {
    readonly isSubmitted: boolean;
    readonly isFuture: boolean;
    readonly isReady: boolean;
    readonly isBroadcast: boolean;
    readonly isInSidechainBlock: boolean;
    readonly asInSidechainBlock: H256;
    readonly isRetracted: boolean;
    readonly isFinalityTimeout: boolean;
    readonly isFinalized: boolean;
    readonly isUsurped: boolean;
    readonly isDropped: boolean;
    readonly isInvalid: boolean;
    readonly isTopExecuted: boolean;
    readonly asTopExecuted: Bytes;
    readonly type: "Submitted" | "Future" | "Ready" | "Broadcast" | "InSidechainBlock" | "Retracted" | "FinalityTimeout" | "Finalized" | "Usurped" | "Dropped" | "Invalid" | "TopExecuted";
}
/** @name TwitterValidationData */
export interface TwitterValidationData extends Struct {
    readonly tweet_id: Bytes;
}
/** @name UserShieldingKeyNonceType */
export interface UserShieldingKeyNonceType extends U8aFixed {
}
/** @name UserShieldingKeyType */
export interface UserShieldingKeyType extends U8aFixed {
}
/** @name VCRequested */
export interface VCRequested extends Struct {
    readonly account: AccountId;
    readonly mrEnclave: ShardIdentifier;
    readonly assertion: Assertion;
}
/** @name Web2ValidationData */
export interface Web2ValidationData extends Enum {
    readonly isTwitter: boolean;
    readonly asTwitter: TwitterValidationData;
    readonly isDiscord: boolean;
    readonly asDiscord: DiscordValidationData;
    readonly type: "Twitter" | "Discord";
}
/** @name Web3CommonValidationData */
export interface Web3CommonValidationData extends Struct {
    readonly message: Bytes;
    readonly signature: LitentryMultiSignature;
}
/** @name Web3Network */
export interface Web3Network extends Enum {
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly isLitentry: boolean;
    readonly isLitmus: boolean;
    readonly isLitentryRococo: boolean;
    readonly isKhala: boolean;
    readonly isSubstrateTestnet: boolean;
    readonly isEthereum: boolean;
    readonly isBsc: boolean;
    readonly type: "Polkadot" | "Kusama" | "Litentry" | "Litmus" | "LitentryRococo" | "Khala" | "SubstrateTestnet" | "Ethereum" | "Bsc";
}
/** @name Web3ValidationData */
export interface Web3ValidationData extends Enum {
    readonly isSubstrate: boolean;
    readonly asSubstrate: Web3CommonValidationData;
    readonly isEvm: boolean;
    readonly asEvm: Web3CommonValidationData;
    readonly type: "Substrate" | "Evm";
}
/** @name WorkerRpcReturnValue */
export interface WorkerRpcReturnValue extends Struct {
    readonly value: Bytes;
    readonly do_watch: bool;
    readonly status: DirectRequestStatus;
}
export type PHANTOM_IDENTITY = "identity";
//# sourceMappingURL=types.d.ts.map