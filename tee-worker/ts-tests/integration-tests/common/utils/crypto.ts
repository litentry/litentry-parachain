import type { HexString } from '@polkadot/util/types';
import { bufferToU8a, hexToU8a, isString, stringToU8a, u8aToHex } from '@polkadot/util';
import { KeyObject } from 'crypto';
import { AesOutput, CorePrimitivesIdentity } from 'parachain-api';
import crypto from 'crypto';
import { KeyringPair } from '@polkadot/keyring/types';
import { ethers } from 'ethers';
import { blake2AsU8a } from '@polkadot/util-crypto';
import bitcore from 'bitcore-lib';
import { IntegrationTestContext } from 'common/common-types';
import { buildIdentityHelper } from './identity-helper';

export type KeypairType = 'ed25519' | 'sr25519' | 'ecdsa' | 'ethereum' | 'bitcoin';

export function encryptWithTeeShieldingKey(teeShieldingKey: KeyObject, plaintext: Uint8Array): Buffer {
    return crypto.publicEncrypt(
        {
            key: teeShieldingKey,
            padding: crypto.constants.RSA_PKCS1_OAEP_PADDING,
            oaepHash: 'sha256',
        },
        plaintext
    );
}

// A lazy version without aad. Append the tag to be consistent with rust implementation
export function encryptWithAes(key: HexString, nonce: Uint8Array, cleartext: Buffer): HexString {
    const secretKey = crypto.createSecretKey(hexToU8a(key));
    const cipher = crypto.createCipheriv('aes-256-gcm', secretKey, nonce, {
        authTagLength: 16,
    });
    let encrypted = cipher.update(cleartext.toString('hex'), 'hex', 'hex');
    encrypted += cipher.final('hex');
    encrypted += cipher.getAuthTag().toString('hex');
    return `0x${encrypted}`;
}

export function decryptWithAes(key: HexString, aesOutput: AesOutput, type: 'hex' | 'utf-8'): HexString {
    const secretKey = crypto.createSecretKey(hexToU8a(key));
    const tagSize = 16;
    const ciphertext = aesOutput.ciphertext ? aesOutput.ciphertext : hexToU8a('0x');

    const nonce = aesOutput.nonce ? aesOutput.nonce : hexToU8a('0x');
    const aad = aesOutput.aad ? aesOutput.aad : hexToU8a('0x');

    // notice!!! extract author_tag from ciphertext
    // maybe this code only works with rust aes encryption
    const authorTag = ciphertext.subarray(ciphertext.length - tagSize);

    const decipher = crypto.createDecipheriv('aes-256-gcm', secretKey, nonce, {
        authTagLength: tagSize,
    });
    decipher.setAAD(aad);
    decipher.setAuthTag(authorTag);

    const part1 = decipher.update(ciphertext.subarray(0, ciphertext.length - tagSize), undefined, type);
    const part2 = decipher.final(type);

    return `0x${part1 + part2}`;
}

export interface Signer {
    getAddressRaw(): Uint8Array;
    sign(message: HexString | string | Uint8Array): Promise<Uint8Array>;
    type(): KeypairType;
    getAddressInSubstrateFormat(): Uint8Array;
    getIdentity(api: IntegrationTestContext): Promise<CorePrimitivesIdentity>;
}

export class PolkadotSigner implements Signer {
    keypair: KeyringPair;

    constructor(keypair: KeyringPair) {
        this.keypair = keypair;
    }

    getAddressRaw(): Uint8Array {
        return this.keypair.addressRaw;
    }

    sign(message: HexString | string | Uint8Array): Promise<Uint8Array> {
        return new Promise((resolve) => resolve(this.keypair.sign(message)));
    }

    type(): KeypairType {
        return this.keypair.type;
    }

    getAddressInSubstrateFormat(): Uint8Array {
        return this.getAddressRaw();
    }

    getIdentity(context: IntegrationTestContext): Promise<CorePrimitivesIdentity> {
        return buildIdentityHelper(u8aToHex(this.getAddressRaw()), 'Substrate', context);
    }
}

export class EthersSigner implements Signer {
    wallet: ethers.Wallet;

    constructor(wallet: ethers.Wallet) {
        this.wallet = wallet;
    }

    getAddressRaw(): Uint8Array {
        return hexToU8a(this.wallet.address);
    }

    sign(message: HexString | string | Uint8Array): Promise<Uint8Array> {
        return this.wallet.signMessage(message).then((sig) => {
            return hexToU8a(sig);
        });
    }

    type(): KeypairType {
        return 'ethereum';
    }

    getAddressInSubstrateFormat(): Uint8Array {
        const prefix = stringToU8a('evm:');
        const address = this.getAddressRaw();
        const merged = new Uint8Array(prefix.length + address.length);
        merged.set(prefix);
        merged.set(address, 4);
        return blake2AsU8a(merged, 256);
    }

    getIdentity(context: IntegrationTestContext): Promise<CorePrimitivesIdentity> {
        return buildIdentityHelper(u8aToHex(this.getAddressRaw()), 'Evm', context);
    }
}

export class BitcoinSigner implements Signer {
    keypair: bitcore.PrivateKey;

    constructor(keypair: bitcore.PrivateKey) {
        this.keypair = keypair;
    }

    getAddressRaw(): Uint8Array {
        return bufferToU8a(this.keypair.toPublicKey().toBuffer());
    }

    sign(message: HexString | string | Uint8Array): Promise<Uint8Array> {
        return new Promise((resolve, reject) => {
            if (isString(message)) {
                const sig = new bitcore.Message(message).sign(this.keypair);
                resolve(bufferToU8a(Buffer.from(sig, 'base64')));
            } else {
                reject('wrong message type');
            }
        });
    }

    type(): KeypairType {
        return 'bitcoin';
    }

    getAddressInSubstrateFormat(): Uint8Array {
        return blake2AsU8a(this.getAddressRaw(), 256);
    }

    getIdentity(context: IntegrationTestContext): Promise<CorePrimitivesIdentity> {
        return buildIdentityHelper(u8aToHex(this.getAddressRaw()), 'Bitcoin', context);
    }
}
