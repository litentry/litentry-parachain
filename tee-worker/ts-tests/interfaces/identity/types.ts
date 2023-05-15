// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Enum, Option, Struct, U8aFixed, Vec, bool, u128, u32 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { MultiSignature, Signature } from '@polkadot/types/interfaces/extrinsics';
import type { AccountId, Balance, BlockNumber, H256 } from '@polkadot/types/interfaces/runtime';

/** @name Address20 */
export interface Address20 extends U8aFixed {}

/** @name Address32 */
export interface Address32 extends U8aFixed {}

/** @name Assertion */
export interface Assertion extends Enum {
    readonly isA1: boolean;
    readonly isA2: boolean;
    readonly asA2: Bytes;
    readonly isA3: boolean;
    readonly asA3: ITuple<[Bytes, Bytes, Bytes]>;
    readonly isA4: boolean;
    readonly asA4: u128;
    readonly isA5: boolean;
    readonly asA5: ITuple<[Bytes, Bytes]>;
    readonly isA6: boolean;
    readonly isA7: boolean;
    readonly asA7: u128;
    readonly isA8: boolean;
    readonly asA8: Vec<Bytes>;
    readonly isA9: boolean;
    readonly isA10: boolean;
    readonly asA10: u128;
    readonly isA11: boolean;
    readonly asA11: u128;
    readonly isA13: boolean;
    readonly asA13: u32;
    readonly type: 'A1' | 'A2' | 'A3' | 'A4' | 'A5' | 'A6' | 'A7' | 'A8' | 'A9' | 'A10' | 'A11' | 'A13';
}

/** @name DirectRequestStatus */
export interface DirectRequestStatus extends Enum {
    readonly isOk: boolean;
    readonly isTrustedOperationStatus: boolean;
    readonly asTrustedOperationStatus: TrustedOperationStatus;
    readonly isError: boolean;
    readonly type: 'Ok' | 'TrustedOperationStatus' | 'Error';
}

/** @name DiscordValidationData */
export interface DiscordValidationData extends Struct {
    readonly channel_id: Bytes;
    readonly message_id: Bytes;
    readonly guild_id: Bytes;
}

/** @name EthereumSignature */
export interface EthereumSignature extends U8aFixed {}

/** @name EvmIdentity */
export interface EvmIdentity extends Struct {
    readonly network: EvmNetwork;
    readonly address: Address20;
}

/** @name EvmNetwork */
export interface EvmNetwork extends Enum {
    readonly isEthereum: boolean;
    readonly isBsc: boolean;
    readonly type: 'Ethereum' | 'Bsc';
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
    readonly type: 'Public' | 'Trusted';
}

/** @name IdentityContext */
export interface IdentityContext extends Struct {
    readonly metadata: Option<Bytes>;
    readonly linking_request_block: Option<BlockNumber>;
    readonly verification_request_block: Option<BlockNumber>;
    readonly is_verified: bool;
}

/** @name IdentityGenericEvent */
export interface IdentityGenericEvent extends Struct {
    readonly who: AccountId;
    readonly identity: LitentryIdentity;
    readonly id_graph: Vec<ITuple<[LitentryIdentity, IdentityContext]>>;
}

/** @name IdentityMultiSignature */
export interface IdentityMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: Signature;
    readonly isEthereum: boolean;
    readonly asEthereum: EthereumSignature;
    readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa' | 'Ethereum';
}

/** @name IdentityString */
export interface IdentityString extends Bytes {}

/** @name LitentryIdentity */
export interface LitentryIdentity extends Enum {
    readonly isSubstrate: boolean;
    readonly asSubstrate: SubstrateIdentity;
    readonly isEvm: boolean;
    readonly asEvm: EvmIdentity;
    readonly isWeb2: boolean;
    readonly asWeb2: Web2Identity;
    readonly type: 'Substrate' | 'Evm' | 'Web2';
}

/** @name LitentryValidationData */
export interface LitentryValidationData extends Enum {
    readonly isWeb2Validation: boolean;
    readonly asWeb2Validation: Web2ValidationData;
    readonly isWeb3Validation: boolean;
    readonly asWeb3Validation: Web3ValidationData;
    readonly type: 'Web2Validation' | 'Web3Validation';
}

/** @name PublicGetter */
export interface PublicGetter extends Enum {
    readonly isSomeValue: boolean;
    readonly type: 'SomeValue';
}

/** @name Request */
export interface Request extends Struct {
    readonly shard: ShardIdentifier;
    readonly cyphertext: Bytes;
}

/** @name ShardIdentifier */
export interface ShardIdentifier extends H256 {}

/** @name SubstrateIdentity */
export interface SubstrateIdentity extends Struct {
    readonly network: SubstrateNetwork;
    readonly address: Address32;
}

/** @name SubstrateNetwork */
export interface SubstrateNetwork extends Enum {
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly isLitentry: boolean;
    readonly isLitmus: boolean;
    readonly isLitentryRococo: boolean;
    readonly isKhala: boolean;
    readonly isTestNet: boolean;
    readonly type: 'Polkadot' | 'Kusama' | 'Litentry' | 'Litmus' | 'LitentryRococo' | 'Khala' | 'TestNet';
}

/** @name TrustedCall */
export interface TrustedCall extends Enum {
    readonly isBalanceSetBalance: boolean;
    readonly asBalanceSetBalance: ITuple<[AccountId, AccountId, Balance, Balance]>;
    readonly isBalanceTransfer: boolean;
    readonly asBalanceTransfer: ITuple<[AccountId, AccountId, Balance]>;
    readonly isBalanceUnshield: boolean;
    readonly asBalanceUnshield: ITuple<[AccountId, AccountId, Balance, ShardIdentifier]>;
    readonly isBalanceShield: boolean;
    readonly asBalanceShield: ITuple<[AccountId, AccountId, Balance]>;
    readonly isSetUserShieldingKeyDirect: boolean;
    readonly asSetUserShieldingKeyDirect: ITuple<[AccountId, UserShieldingKeyType, H256]>;
    readonly isCreateIdentityDirect: boolean;
    readonly asCreateIdentityDirect: ITuple<[AccountId, LitentryIdentity, Option<Bytes>, u32, H256]>;
    readonly type:
        | 'BalanceSetBalance'
        | 'BalanceTransfer'
        | 'BalanceUnshield'
        | 'BalanceShield'
        | 'SetUserShieldingKeyDirect'
        | 'CreateIdentityDirect';
}

/** @name TrustedCallSigned */
export interface TrustedCallSigned extends Struct {
    readonly call: TrustedCall;
    readonly index: u32;
    readonly signature: MultiSignature;
}

/** @name TrustedGetter */
export interface TrustedGetter extends Enum {
    readonly isFreeBalance: boolean;
    readonly asFreeBalance: AccountId;
    readonly type: 'FreeBalance';
}

/** @name TrustedGetterSigned */
export interface TrustedGetterSigned extends Struct {
    readonly getter: TrustedGetter;
    readonly signature: MultiSignature;
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
    readonly type:
        | 'Submitted'
        | 'Future'
        | 'Ready'
        | 'Broadcast'
        | 'InSidechainBlock'
        | 'Retracted'
        | 'FinalityTimeout'
        | 'Finalized'
        | 'Usurped'
        | 'Dropped'
        | 'Invalid';
}

/** @name TwitterValidationData */
export interface TwitterValidationData extends Struct {
    readonly tweet_id: Bytes;
}

/** @name UserShieldingKeyType */
export interface UserShieldingKeyType extends U8aFixed {}

/** @name VCRequested */
export interface VCRequested extends Struct {
    readonly account: AccountId;
    readonly mrEnclave: ShardIdentifier;
    readonly assertion: Assertion;
}

/** @name Web2Identity */
export interface Web2Identity extends Struct {
    readonly network: Web2Network;
    readonly address: IdentityString;
}

/** @name Web2Network */
export interface Web2Network extends Enum {
    readonly isTwitter: boolean;
    readonly isDiscord: boolean;
    readonly isGithub: boolean;
    readonly type: 'Twitter' | 'Discord' | 'Github';
}

/** @name Web2ValidationData */
export interface Web2ValidationData extends Enum {
    readonly isTwitter: boolean;
    readonly asTwitter: TwitterValidationData;
    readonly isDiscord: boolean;
    readonly asDiscord: DiscordValidationData;
    readonly type: 'Twitter' | 'Discord';
}

/** @name Web3CommonValidationData */
export interface Web3CommonValidationData extends Struct {
    readonly message: Bytes;
    readonly signature: IdentityMultiSignature;
}

/** @name Web3ValidationData */
export interface Web3ValidationData extends Enum {
    readonly isSubstrate: boolean;
    readonly asSubstrate: Web3CommonValidationData;
    readonly isEvm: boolean;
    readonly asEvm: Web3CommonValidationData;
    readonly type: 'Substrate' | 'Evm';
}

/** @name WorkerRpcReturnString */
export interface WorkerRpcReturnString extends Struct {
    readonly vec: Bytes;
}

/** @name WorkerRpcReturnValue */
export interface WorkerRpcReturnValue extends Struct {
    readonly value: Bytes;
    readonly do_watch: bool;
    readonly status: DirectRequestStatus;
}

export type PHANTOM_IDENTITY = 'identity';
