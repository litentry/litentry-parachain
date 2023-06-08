import type { HexString } from '@polkadot/util/types';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import { KeyObject } from 'crypto';
import { AESOutput } from '../type-definitions';

const crypto = require('crypto');

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
export function encryptWithAES(key: HexString, nonce: Uint8Array, cleartext: Buffer): HexString {
    const secretKey = crypto.createSecretKey(hexToU8a(key));
    const cipher = crypto.createCipheriv('aes-256-gcm', secretKey, nonce, {
        authTagLength: 16,
    });
    let encrypted = cipher.update(cleartext, 'utf8', 'hex');
    encrypted += cipher.final('hex');
    encrypted += cipher.getAuthTag().toString('hex');
    return `0x${encrypted}`;
}

export function decryptWithAES(key: HexString, aesOutput: AESOutput, type: string): HexString {
    if (aesOutput.ciphertext && aesOutput.nonce) {
        const secretKey = crypto.createSecretKey(hexToU8a(key));
        const tagSize = 16;
        const ciphertext = aesOutput.ciphertext ? aesOutput.ciphertext : hexToU8a('0x');

        const nonce = aesOutput.nonce ? aesOutput.nonce : hexToU8a('0x');
        const aad = aesOutput.aad ? aesOutput.aad : hexToU8a('0x');

        // notice!!! extract author_tag from ciphertext
        // maybe this code only works with rust aes encryption
        const authorTag = ciphertext.subarray(ciphertext.length - tagSize);

        const decipher = crypto.createDecipheriv('aes-256-gcm', secretKey, nonce, {
            authTagLength: 16,
        });
        decipher.setAAD(aad);
        decipher.setAuthTag(authorTag);

        let part1 = decipher.update(ciphertext.subarray(0, ciphertext.length - tagSize), undefined, type);
        let part2 = decipher.final(type);

        return `0x${part1 + part2}`;
    } else {
        return u8aToHex(aesOutput as Uint8Array);
    }
}
