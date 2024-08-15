// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Enum, Struct, Text, U8aFixed, Vec } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { EthereumSignature } from '@polkadot/types/interfaces/eth';
import type { Signature } from '@polkadot/types/interfaces/extrinsics';
import type { AccountId, AccountId20, AccountId33, BlockNumber } from '@polkadot/types/interfaces/runtime';

/** @name DiscordOAuth2 */
export interface DiscordOAuth2 extends Struct {
  readonly code: Bytes;
  readonly redirect_uri: Bytes;
}

/** @name DiscordValidationData */
export interface DiscordValidationData extends Enum {
  readonly isPublicMessage: boolean;
  readonly asPublicMessage: PublicMessage;
  readonly isOAuth2: boolean;
  readonly asOAuth2: DiscordOAuth2;
  readonly type: 'PublicMessage' | 'OAuth2';
}

/** @name IdentityContext */
export interface IdentityContext extends Struct {
  readonly linkBlock: BlockNumber;
  readonly web3networks: Vec<Web3Network>;
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
  readonly type: 'Active' | 'Inactive';
}

/** @name LitentryIdentity */
export interface LitentryIdentity extends Enum {
  readonly isTwitter: boolean;
  readonly asTwitter: Text;
  readonly isDiscord: boolean;
  readonly asDiscord: Text;
  readonly isGithub: boolean;
  readonly asGithub: Text;
  readonly isSubstrate: boolean;
  readonly asSubstrate: AccountId;
  readonly isEvm: boolean;
  readonly asEvm: AccountId20;
  readonly isBitcoin: boolean;
  readonly asBitcoin: AccountId33;
  readonly isSolana: boolean;
  readonly asSolana: AccountId;
  readonly type: 'Twitter' | 'Discord' | 'Github' | 'Substrate' | 'Evm' | 'Bitcoin' | 'Solana';
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
  readonly isBitcoin: boolean;
  readonly asBitcoin: U8aFixed;
  readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa' | 'Ethereum' | 'Bitcoin';
}

/** @name LitentryValidationData */
export interface LitentryValidationData extends Enum {
  readonly isWeb2Validation: boolean;
  readonly asWeb2Validation: Web2ValidationData;
  readonly isWeb3Validation: boolean;
  readonly asWeb3Validation: Web3ValidationData;
  readonly type: 'Web2Validation' | 'Web3Validation';
}

/** @name PublicMessage */
export interface PublicMessage extends Struct {
  readonly channel_id: Bytes;
  readonly message_id: Bytes;
  readonly guild_id: Bytes;
}

/** @name PublicTweet */
export interface PublicTweet extends Struct {
  readonly tweet_id: Bytes;
}

/** @name TwitterOAuth2 */
export interface TwitterOAuth2 extends Struct {
  readonly code: Bytes;
  readonly state: Bytes;
  readonly redirect_uri: Bytes;
}

/** @name TwitterValidationData */
export interface TwitterValidationData extends Enum {
  readonly isPublicTweet: boolean;
  readonly asPublicTweet: PublicTweet;
  readonly isOAuth2: boolean;
  readonly asOAuth2: TwitterOAuth2;
  readonly type: 'PublicTweet' | 'OAuth2';
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
  readonly isBitcoinP2tr: boolean;
  readonly isBitcoinP2pkh: boolean;
  readonly isBitcoinP2sh: boolean;
  readonly isBitcoinP2wpkh: boolean;
  readonly isBitcoinP2wsh: boolean;
  readonly isPolygon: boolean;
  readonly isArbitrum: boolean;
  readonly isSolana: boolean;
  readonly isCombo: boolean;
  readonly type: 'Polkadot' | 'Kusama' | 'Litentry' | 'Litmus' | 'LitentryRococo' | 'Khala' | 'SubstrateTestnet' | 'Ethereum' | 'Bsc' | 'BitcoinP2tr' | 'BitcoinP2pkh' | 'BitcoinP2sh' | 'BitcoinP2wpkh' | 'BitcoinP2wsh' | 'Polygon' | 'Arbitrum' | 'Solana' | 'Combo';
}

/** @name Web3ValidationData */
export interface Web3ValidationData extends Enum {
  readonly isSubstrate: boolean;
  readonly asSubstrate: Web3CommonValidationData;
  readonly isEvm: boolean;
  readonly asEvm: Web3CommonValidationData;
  readonly isBitcoin: boolean;
  readonly asBitcoin: Web3CommonValidationData;
  readonly isSolana: boolean;
  readonly asSolana: Web3CommonValidationData;
  readonly type: 'Substrate' | 'Evm' | 'Bitcoin' | 'Solana';
}

export type PHANTOM_IDENTITYMANAGEMENT = 'identityManagement';
