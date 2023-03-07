import { encryptWithTeeShieldingKey, listenEvent, sendTxUntilInBlock } from './utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';
import {
    IdentityGenericEvent,
    IntegrationTestContext,
    LitentryIdentity,
    LitentryValidationData,
} from './type-definitions';
import { expect } from 'chai';

export async function setErrorUserShieldingKey(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean
): Promise<string | undefined> {
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, aesKey).toString('hex');
    const tx = context.substrate.tx.identityManagement.setUserShieldingKey(context.mrEnclave, `0x${ciphertext}`);

    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const events = await listenEvent(context.substrate, 'identityManagement', [
            'SetUserShieldingKeyHandlingFailed',
        ]);
        expect(events.length).to.be.equal(1);
        return events[0].method as string;
    }
    return undefined;
}

export async function createErrorIdentity(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    errorCiphertext: string
): Promise<string | undefined> {
    const tx = context.substrate.tx.identityManagement.createIdentity(
        context.mrEnclave,
        signer.address,
        errorCiphertext,
        null
    );

    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const events = await listenEvent(context.substrate, 'identityManagement', ['CreateIdentityHandlingFailed']);
        expect(events.length).to.be.equal(1);
        return events[0].method as string;
    }
    return undefined;
}

export async function verifyErrorIdentity(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identity: LitentryIdentity,
    data: LitentryValidationData
): Promise<IdentityGenericEvent | undefined> {
    const identity_encode = context.substrate.createType('LitentryIdentity', identity).toHex();
    const validation_encode = context.substrate.createType('LitentryValidationData', data).toHex();
    const identity_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, identity_encode).toString('hex');
    const validation_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, validation_encode).toString(
        'hex'
    );

    const tx = context.substrate.tx.identityManagement.verifyIdentity(
        context.mrEnclave,
        `0x${identity_ciphertext}`,
        `0x${validation_ciphertext}`
    );

    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const events = await listenEvent(context.substrate, 'identityManagement', ['IdentityAlreadyVerified']);
        expect(events.length).to.be.equal(1);
        const data = events[0].data as any;
        // return decodeIdentityEvent(
        //     context.substrate,
        //     data.account.toHex(),
        //     decryptWithAES(aesKey, data.identity, 'hex'),
        //     decryptWithAES(aesKey, data.idGraph, 'hex')
        // );
    }
    return undefined;
}
