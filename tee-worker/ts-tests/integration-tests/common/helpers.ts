import { xxhashAsU8a, blake2AsU8a } from '@polkadot/util-crypto';
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { Keyring } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { HexString } from '@polkadot/util/types';
import './config';
import { IntegrationTestContext, JsonRpcRequest } from './common-types';
import { randomBytes } from 'crypto';
import { ECPairFactory, ECPairInterface } from 'ecpair';
import * as ecc from 'tiny-secp256k1';
import { ethers, Wallet } from 'ethers';
import { EthersSigner, PolkadotSigner, BitcoinSigner } from './utils/crypto';
import {Wallets} from './common-types';
export function blake2128Concat(data: HexString | Uint8Array): Uint8Array {
    return u8aConcat(blake2AsU8a(data, 128), u8aToU8a(data));
}

export function twox64Concat(data: HexString | Uint8Array): Uint8Array {
    return u8aConcat(xxhashAsU8a(data, 64), u8aToU8a(data));
}

export function identity(data: HexString | Uint8Array): Uint8Array {
    return u8aToU8a(data);
}

export function createJsonRpcRequest(method: string, params: any, id: number): JsonRpcRequest {
    return {
        jsonrpc: '2.0',
        method,
        params,
        id,
    };
}

export function nextRequestId(context: IntegrationTestContext): number {
    const nextId = context.requestId + 1;
    context.requestId = nextId;
    return nextId;
}

export function randomEvmWallet(): Wallet {
    return ethers.Wallet.createRandom();
}
export function randomSubstrateWallet(): KeyringPair {
    const keyring = new Keyring({ type: 'sr25519' });
    return keyring.addFromSeed(randomBytes(32));
}

export function randomBitcoinWallet(): ECPairInterface {
    const ecPair = ECPairFactory(ecc);
    const keyPair = ecPair.makeRandom();
    return keyPair;
}

export function genesisSubstrateWallet(name: string): KeyringPair {
    const keyring = new Keyring({ type: 'sr25519' });
    const keyPair = keyring.addFromUri(`//${name}`, { name });
    return keyPair;
}

export const createWeb3Wallets = (): Wallets => {
    const wallets: Wallets = {
        evm: {},
        substrate: {},
        bitcoin: {},
    };
    const walletNames = ['Alice', 'Bob', 'Charlie', 'Dave', 'Eve'];
    for (const name of walletNames) {
        wallets.evm[name] = new EthersSigner(randomEvmWallet());
        wallets.substrate[name] = new PolkadotSigner(genesisSubstrateWallet(name));
        wallets.bitcoin[name] = new BitcoinSigner(randomBitcoinWallet());
    }

    return wallets;
};
