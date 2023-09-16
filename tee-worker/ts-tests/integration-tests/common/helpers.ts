import { xxhashAsU8a, blake2AsU8a } from '@polkadot/util-crypto';
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { Keyring } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { HexString } from '@polkadot/util/types';
import './config';
import { IntegrationTestContext, JsonRpcRequest } from './type-definitions';

// format and setup
const keyring = new Keyring({ type: 'sr25519' });
export function getSubstrateSigner(): {
    alice: KeyringPair;
    bob: KeyringPair;
    charlie: KeyringPair;
    eve: KeyringPair;
} {
    const alice = keyring.addFromUri('//Alice', { name: 'Alice' });
    const bob = keyring.addFromUri('//Bob', { name: 'Bob' });
    const charlie = keyring.addFromUri('//Charlie', { name: 'Charlie' });
    const eve = keyring.addFromUri('//Eve', { name: 'Eve' });
    const signers = {
        alice,
        bob,
        charlie,
        eve,
    };
    return signers;
}
export function getEthereumSigner(): {
    alice: string;
    bob: string;
    charlie: string;
    dave: string;
    eve: string;
} {
    const secp256k1PrivateKeyLength = 32;
    const names = ['alice', 'bob', 'charlie', 'dave', 'eve'];
    const keys = new Array<string>();
    for (const name of names) {
        const result = Buffer.alloc(secp256k1PrivateKeyLength);
        result.fill(name, secp256k1PrivateKeyLength - Buffer.from(name, 'utf8').length);

        keys.push(result.toString('hex'));
    }
    return { alice: keys[0], bob: keys[1], charlie: keys[2], dave: keys[3], eve: keys[4] };
}

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
