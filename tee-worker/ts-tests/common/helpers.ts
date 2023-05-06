import { xxhashAsU8a, blake2AsU8a } from '@polkadot/util-crypto';
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { HexString } from '@polkadot/util/types';
import { Keyring } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';

//format and setup
const keyring = new Keyring({ type: 'sr25519' });
export function getSubstrateSigner(): {
    alice: KeyringPair;
    bob: KeyringPair;
    charlie: KeyringPair;
    eve: KeyringPair;
} {
    let Alice = keyring.addFromUri('//Alice', { name: 'Alice' });
    let Bob = keyring.addFromUri('//Bob', { name: 'Bob' });
    let Charlie = keyring.addFromUri('//Charlie', { name: 'Charlie' });
    let Eve = keyring.addFromUri('//Eve', { name: 'Eve' });
    // const signers = [Alice, Bob, Charlie, Eve];
    const signers = {
        alice: Alice,
        bob: Bob,
        charlie: Charlie,
        eve: Eve,
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
    let keys = new Array<string>();
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
