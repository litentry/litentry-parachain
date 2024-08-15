// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// The next line was manually fixed with the right import path
import type { LitentryIdentity, LitentryMultiSignature, LitentryValidationData, Web3Network } from '../identityManagement';
import type { Bytes, Enum, Option, Result, Struct, Text, U8aFixed, Vec, bool, u32, u8 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { H256, Index } from '@polkadot/types/interfaces/runtime';
// The next line was manually added to fix cross-definitions dependency issues
import type { CorePrimitivesAssertion } from '@polkadot/types/lookup';

/** @name AesKey */
export interface AesKey extends U8aFixed {}

/** @name DirectRequestStatus */
export interface DirectRequestStatus extends Enum {
  readonly isOk: boolean;
  readonly isTrustedOperationStatus: boolean;
  readonly asTrustedOperationStatus: ITuple<[TrustedOperationStatus, H256]>;
  readonly isError: boolean;
  readonly type: 'Ok' | 'TrustedOperationStatus' | 'Error';
}

/** @name EncryptedAesRequest */
export interface EncryptedAesRequest extends Struct {
  readonly shard: ShardIdentifier;
  readonly key: Bytes;
  readonly payload: KeyAesOutput;
}

/** @name ErrorDetail */
export interface ErrorDetail extends Enum {
  readonly isImportError: boolean;
  readonly isUnauthorizedSigner: boolean;
  readonly isStfError: boolean;
  readonly asStfError: Bytes;
  readonly isSendStfRequestFailed: boolean;
  readonly isParseError: boolean;
  readonly isDataProviderError: boolean;
  readonly asDataProviderError: Bytes;
  readonly isInvalidIdentity: boolean;
  readonly isWrongWeb2Handle: boolean;
  readonly isUnexpectedMessage: boolean;
  readonly isVerifyWeb3SignatureFailed: boolean;
  readonly isNoEligibleIdentity: boolean;
  readonly type: 'ImportError' | 'UnauthorizedSigner' | 'StfError' | 'SendStfRequestFailed' | 'ParseError' | 'DataProviderError' | 'InvalidIdentity' | 'WrongWeb2Handle' | 'UnexpectedMessage' | 'VerifyWeb3SignatureFailed' | 'NoEligibleIdentity';
}

/** @name Getter */
export interface Getter extends Enum {
  readonly isPublic: boolean;
  readonly asPublic: PublicGetter;
  readonly isTrusted: boolean;
  readonly asTrusted: TrustedGetterSigned;
  readonly type: 'Public' | 'Trusted';
}

/** @name KeyAesOutput */
export interface KeyAesOutput extends Struct {
  readonly ciphertext: Bytes;
  readonly aad: Bytes;
  readonly nonce: U8aFixed;
}

/** @name LinkIdentityResult */
export interface LinkIdentityResult extends Struct {
  readonly mutated_id_graph: KeyAesOutput;
  readonly id_graph_hash: H256;
}

/** @name PublicGetter */
export interface PublicGetter extends Enum {
  readonly isSomeValue: boolean;
  readonly asSomeValue: u32;
  readonly isNonce: boolean;
  readonly asNonce: LitentryIdentity;
  readonly isIdGraphHash: boolean;
  readonly asIdGraphHash: LitentryIdentity;
  readonly type: 'SomeValue' | 'Nonce' | 'IdGraphHash';
}

/** @name Request */
export interface Request extends Struct {
  readonly shard: ShardIdentifier;
  readonly payload: Bytes;
}

/** @name RequestVcErrorDetail */
export interface RequestVcErrorDetail extends Enum {
  readonly isUnexpectedCall: boolean;
  readonly asUnexpectedCall: Text;
  readonly isDuplicateAssertionRequest: boolean;
  readonly isShieldingKeyRetrievalFailed: boolean;
  readonly asShieldingKeyRetrievalFailed: Text;
  readonly isRequestPayloadDecodingFailed: boolean;
  readonly isSidechainDataRetrievalFailed: boolean;
  readonly asSidechainDataRetrievalFailed: Text;
  readonly isIdentityAlreadyLinked: boolean;
  readonly isNoEligibleIdentity: boolean;
  readonly isInvalidSignerAccount: boolean;
  readonly isUnauthorizedSigner: boolean;
  readonly isAssertionBuildFailed: boolean;
  readonly asAssertionBuildFailed: VCMPError;
  readonly isMissingAesKey: boolean;
  readonly isMrEnclaveRetrievalFailed: boolean;
  readonly isEnclaveSignerRetrievalFailed: boolean;
  readonly isSignatureVerificationFailed: boolean;
  readonly isConnectionHashNotFound: boolean;
  readonly asConnectionHashNotFound: Text;
  readonly isMetadataRetrievalFailed: boolean;
  readonly asMetadataRetrievalFailed: Text;
  readonly isInvalidMetadata: boolean;
  readonly asInvalidMetadata: Text;
  readonly isTrustedCallSendingFailed: boolean;
  readonly asTrustedCallSendingFailed: Text;
  readonly isCallSendingFailed: boolean;
  readonly asCallSendingFailed: Text;
  readonly isExtrinsicConstructionFailed: boolean;
  readonly asExtrinsicConstructionFailed: Text;
  readonly isExtrinsicSendingFailed: boolean;
  readonly asExtrinsicSendingFailed: Text;
  readonly type: 'UnexpectedCall' | 'DuplicateAssertionRequest' | 'ShieldingKeyRetrievalFailed' | 'RequestPayloadDecodingFailed' | 'SidechainDataRetrievalFailed' | 'IdentityAlreadyLinked' | 'NoEligibleIdentity' | 'InvalidSignerAccount' | 'UnauthorizedSigner' | 'AssertionBuildFailed' | 'MissingAesKey' | 'MrEnclaveRetrievalFailed' | 'EnclaveSignerRetrievalFailed' | 'SignatureVerificationFailed' | 'ConnectionHashNotFound' | 'MetadataRetrievalFailed' | 'InvalidMetadata' | 'TrustedCallSendingFailed' | 'CallSendingFailed' | 'ExtrinsicConstructionFailed' | 'ExtrinsicSendingFailed';
}

/** @name RequestVCResult */
export interface RequestVCResult extends Struct {
  readonly vc_payload: KeyAesOutput;
  readonly pre_mutated_id_graph: KeyAesOutput;
  readonly pre_id_graph_hash: H256;
}

/** @name RequestVcResultOrError */
export interface RequestVcResultOrError extends Struct {
  readonly result: Result<Bytes, RequestVcErrorDetail>;
  readonly idx: u8;
  readonly len: u8;
}

/** @name SetIdentityNetworksResult */
export interface SetIdentityNetworksResult extends Struct {
  readonly mutated_id_graph: KeyAesOutput;
  readonly id_graph_hash: H256;
}

/** @name ShardIdentifier */
export interface ShardIdentifier extends H256 {}

/** @name StfError */
export interface StfError extends Enum {
  readonly isLinkIdentityFailed: boolean;
  readonly asLinkIdentityFailed: ErrorDetail;
  readonly isDeactivateIdentityFailed: boolean;
  readonly asDeactivateIdentityFailed: ErrorDetail;
  readonly isActivateIdentityFailed: boolean;
  readonly asActivateIdentityFailed: ErrorDetail;
  readonly isRequestVCFailed: boolean;
  readonly asRequestVCFailed: ITuple<[CorePrimitivesAssertion, ErrorDetail]>;
  readonly isSetScheduledMrEnclaveFailed: boolean;
  readonly isSetIdentityNetworksFailed: boolean;
  readonly asSetIdentityNetworksFailed: ErrorDetail;
  readonly isInvalidAccount: boolean;
  readonly isUnclassifiedError: boolean;
  readonly isRemoveIdentityFailed: boolean;
  readonly asRemoveIdentityFailed: ErrorDetail;
  readonly isEmptyIDGraph: boolean;
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
  readonly type: 'LinkIdentityFailed' | 'DeactivateIdentityFailed' | 'ActivateIdentityFailed' | 'RequestVCFailed' | 'SetScheduledMrEnclaveFailed' | 'SetIdentityNetworksFailed' | 'InvalidAccount' | 'UnclassifiedError' | 'RemoveIdentityFailed' | 'EmptyIDGraph' | 'MissingPrivileges' | 'RequireEnclaveSignerAccount' | 'Dispatch' | 'MissingFunds' | 'InvalidNonce' | 'StorageHashMismatch' | 'InvalidStorageDiff' | 'InvalidMetadata';
}

/** @name TrustedCall */
export interface TrustedCall extends Enum {
  readonly isLinkIdentity: boolean;
  readonly asLinkIdentity: ITuple<[LitentryIdentity, LitentryIdentity, LitentryIdentity, LitentryValidationData, Vec<Web3Network>, Option<AesKey>, H256]>;
  readonly isDeactivateIdentity: boolean;
  readonly asDeactivateIdentity: ITuple<[LitentryIdentity, LitentryIdentity, LitentryIdentity, Option<AesKey>, H256]>;
  readonly isActivateIdentity: boolean;
  readonly asActivateIdentity: ITuple<[LitentryIdentity, LitentryIdentity, LitentryIdentity, Option<AesKey>, H256]>;
  readonly isRequestVc: boolean;
  readonly asRequestVc: ITuple<[LitentryIdentity, LitentryIdentity, CorePrimitivesAssertion, Option<AesKey>, H256]>;
  readonly isSetIdentityNetworks: boolean;
  readonly asSetIdentityNetworks: ITuple<[LitentryIdentity, LitentryIdentity, LitentryIdentity, Vec<Web3Network>, Option<AesKey>, H256]>;
  readonly isRequestBatchVc: boolean;
  readonly asRequestBatchVc: ITuple<[LitentryIdentity, LitentryIdentity, Vec<CorePrimitivesAssertion>, Option<AesKey>, H256]>;
  readonly type: 'LinkIdentity' | 'DeactivateIdentity' | 'ActivateIdentity' | 'RequestVc' | 'SetIdentityNetworks' | 'RequestBatchVc';
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
  readonly isIdGraph: boolean;
  readonly asIdGraph: LitentryIdentity;
  readonly isIdGraphStats: boolean;
  readonly asIdGraphStats: LitentryIdentity;
  readonly type: 'FreeBalance' | 'ReservedBalance' | 'IdGraph' | 'IdGraphStats';
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
  readonly type: 'IndirectCall' | 'DirectCall' | 'Get';
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
  readonly type: 'Submitted' | 'Future' | 'Ready' | 'Broadcast' | 'InSidechainBlock' | 'Retracted' | 'FinalityTimeout' | 'Finalized' | 'Usurped' | 'Dropped' | 'Invalid' | 'TopExecuted';
}

/** @name VCMPError */
export interface VCMPError extends Enum {
  readonly isRequestVCFailed: boolean;
  readonly asRequestVCFailed: ITuple<[CorePrimitivesAssertion, ErrorDetail]>;
  readonly isUnclassifiedError: boolean;
  readonly asUnclassifiedError: ErrorDetail;
  readonly type: 'RequestVCFailed' | 'UnclassifiedError';
}

/** @name WorkerRpcReturnValue */
export interface WorkerRpcReturnValue extends Struct {
  readonly value: Bytes;
  readonly do_watch: bool;
  readonly status: DirectRequestStatus;
}

export type PHANTOM_TRUSTEDCALLS = 'trustedCalls';
