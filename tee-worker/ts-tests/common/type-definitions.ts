import { ApiPromise, Keyring } from '@polkadot/api';
import { KeyObject } from 'crypto';
import { HexString } from '@polkadot/util/types';
import WebSocketAsPromised from 'websocket-as-promised';
import type { KeyringPair } from '@polkadot/keyring/types';
import { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import { Metadata } from '@polkadot/types';
import { Wallet } from 'ethers';
import type {
    SubstrateNetwork as SubNet,
    Web2Network as Web2Net,
    EvmNetwork as EvmNet,
    DirectRequestStatus,
} from '../interfaces/identity/types';
import { default as teeTypes } from '../interfaces/identity/definitions';

export { teeTypes };

export type Web2Network = Web2Net['type'];
export type SubstrateNetwork = SubNet['type'];
export type EvmNetwork = EvmNet['type'];

export type WorkerRpcReturnString = {
    vec: string;
};

export type WorkerRpcReturnValue = {
    value: `0x${string}`;
    do_watch: boolean;
    status: DirectRequestStatus['type'];
};
export type EnclaveResult = {
    mrEnclave: `0x${string}`;
    shieldingKey: `0x${string}`;
    vcPubkey: `0x${string}`;
    sgxMetadata: {};
};

export type PubicKeyJson = {
    n: Uint8Array;
    e: Uint8Array;
};

interface EthersWalletItem {
    [key: string]: Wallet;
}
interface SubstrateWalletItem {
    [key: string]: KeyringPair;
}
export type IntegrationTestContext = {
    tee: WebSocketAsPromised;
    api: ApiPromise;
    teeShieldingKey: KeyObject;
    mrEnclave: HexString;
    ethersWallet: EthersWalletItem;
    substrateWallet: SubstrateWalletItem;
    metaData: Metadata;
    web3Signers: Web3Wallets[];
};

export class AESOutput {
    ciphertext?: Uint8Array;
    aad?: Uint8Array;
    nonce?: Uint8Array;
}

//identity types
export type LitentryIdentity = {
    Substrate?: SubstrateIdentity;
    Evm?: EvmIdentity;
    Web2?: Web2Identity;
};

export type SubstrateIdentity = {
    network: SubstrateNetwork;
    address: HexString;
};

export type EvmIdentity = {
    network: EvmNetwork;
    address: HexString;
};

export type Web2Identity = {
    network: Web2Network;
    address: string;
};

export type LitentryValidationData = {
    Web2Validation?: Web2ValidationData;
    Web3Validation?: Web3ValidationData;
};

export type Web2ValidationData = {
    Twitter?: TwitterValidationData;
    Discord?: DiscordValidationData;
};

export type Web3ValidationData = {
    Substrate?: Web3CommonValidationData;
    Evm?: Web3CommonValidationData;
};

export type Web3CommonValidationData = {
    message: HexString;
    signature: IdentityMultiSignature;
};

export type IdentityMultiSignature = {
    Ethereum?: HexString;
    Ed25519?: HexString;
    Sr25519?: HexString;
};

export type Ed25519Signature = {
    Ed25519: HexString;
    Sr25519: HexString;
    Ecdsa: HexString;
    Ethereum: EthereumSignature;
};

export type EthereumSignature = HexString;

export type TwitterValidationData = {
    tweet_id: HexString;
};

export type DiscordValidationData = {
    channel_id: HexString;
    message_id: HexString;
    guild_id: HexString;
};

export type Web3Wallets = {
    substrateWallet: KeyringPair;
    ethereumWallet: Wallet;
};

// export type DiscordValidationData = {}

export type Web3Network = {
    Substrate?: SubstrateNetwork;
    Evm?: EvmNetwork;
};

export type IdentityGenericEvent = {
    who: HexString;
    identity: LitentryIdentity;
    idGraph: [LitentryIdentity, IdentityContext][];
    challengeCode?: HexString;
};

export type IdentityContext = {
    metadata?: HexString;
    linking_request_block?: number;
    verification_request_block?: number;
    is_verified: boolean;
};

//vc types
export type VCRequested = {
    account: HexString;
    mrEnclave: HexString;
    assertion: Assertion;
};

export enum IndexingNetwork {
    Litentry = 'Litentry',
    Litmus = 'Litmus',
    Polkadot = 'Polkadot',
    Kusama = 'Kusama',
    Khala = 'Khala',
    Ethereum = 'Ethereum',
}

export type Assertion = {
    A1?: string;
    A2?: [string];
    A3?: [string, string, string];
    A4?: string;
    A5?: [string, string];
    A6?: string;
    A7?: string;
    A8?: [IndexingNetwork];
    A9?: string;
    A10?: string;
    A11?: string;
};

export type TransactionSubmit = {
    tx: SubmittableExtrinsic<ApiTypes>;
    nonce: number;
};

//call types
export type RequestBody = {
    id: number;
    jsonrpc: string;
    method: string;
};

export const JsonSchema = {
    type: 'object',
    properties: {
        id: {
            type: 'string',
        },
        type: {
            type: 'array',
        },
        issuer: {
            type: 'object',
            properties: {
                id: {
                    type: 'string',
                },
                name: {
                    type: 'string',
                },
                shard: {
                    type: 'string',
                },
            },
        },
        issuanceBlockNumber: {
            type: 'integer',
        },
        credentialSubject: {
            type: 'object',
            properties: {
                id: {
                    type: 'string',
                },
                description: {
                    type: 'string',
                },
                type: {
                    type: 'string',
                },
                tag: {
                    type: 'array',
                },
                assertions: {
                    type: 'array',
                    items: {
                        type: 'object',
                    },
                },
                values: {
                    type: 'array',
                    items: {
                        type: 'boolean',
                    },
                },
                endpoint: {
                    type: 'string',
                },
            },
            required: ['id', 'description', 'type', 'assertions', 'values', 'endpoint'],
        },
        proof: {
            type: 'object',
            properties: {
                createdBlockNumber: {
                    type: 'integer',
                },
                type: {
                    enum: ['Ed25519Signature2020'],
                },
                proofPurpose: {
                    enum: ['assertionMethod'],
                },
                proofValue: {
                    type: 'string',
                },
                verificationMethod: {
                    type: 'string',
                },
            },
        },
    },
    required: ['id', 'type', 'credentialSubject', 'issuer', 'issuanceBlockNumber', 'proof'],
};
