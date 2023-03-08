import { encryptWithTeeShieldingKey, listenEvent, sendTxUntilInBlock, sendTxUntilInBlockList } from './utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';
import {
    IntegrationTestContext,
    LitentryIdentity,
    LitentryValidationData,
    TransactionSubmit,
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

export async function createErrorIdentities(
    context: IntegrationTestContext,
    signer: KeyringPair,
    listening: boolean,
    errorCiphertexts: string[]
): Promise<string[] | undefined> {
    let txs: TransactionSubmit[] = [];
    for (let k = 0; k < errorCiphertexts.length; k++) {
        const errorCiphertext = errorCiphertexts[k];
        const tx = context.substrate.tx.identityManagement.createIdentity(
            context.mrEnclave,
            signer.address,
            errorCiphertext,
            null
        );

        const nonce = await context.substrate.rpc.system.accountNextIndex(signer.address);
        let newNonce = nonce.toNumber() + k;
        txs.push({
            tx,
            nonce: newNonce,
        });
    }

    await sendTxUntilInBlockList(context.substrate, txs, signer);

    if (listening) {
        const events = (await listenEvent(context.substrate, 'identityManagement', [
            'CreateIdentityHandlingFailed',
        ])) as any;
        expect(events.length).to.be.equal(errorCiphertexts.length);
        let results: string[] = [];
        for (let i = 0; i < events.length; i++) {
            results.push(events[i].method as string);
        }
        return [...results];
    }
    return undefined;
}

export async function verifyErrorIdentities(
    context: IntegrationTestContext,
    signer: KeyringPair,
    listening: boolean,
    identities: LitentryIdentity[],
    datas: LitentryValidationData[]
): Promise<string[] | undefined> {
    let txs: TransactionSubmit[] = [];
    for (let k = 0; k < identities.length; k++) {
        let identity = identities[k];
        let data = datas[k];
        const identity_encode = context.substrate.createType('LitentryIdentity', identity).toHex();
        const validation_encode = context.substrate.createType('LitentryValidationData', data).toHex();
        const identity_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, identity_encode).toString(
            'hex'
        );
        const validation_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, validation_encode).toString(
            'hex'
        );

        const tx = context.substrate.tx.identityManagement.verifyIdentity(
            context.mrEnclave,
            `0x${identity_ciphertext}`,
            `0x${validation_ciphertext}`
        );

        const nonce = await context.substrate.rpc.system.accountNextIndex(signer.address);
        let newNonce = nonce.toNumber() + k;
        txs.push({
            tx,
            nonce: newNonce,
        });
    }

    await sendTxUntilInBlockList(context.substrate, txs, signer);

    if (listening) {
        const events = (await listenEvent(context.substrate, 'identityManagement', ['StfError'])) as any;
        expect(events.length).to.be.equal(identities.length);
        let results: string[] = [];
        for (let i = 0; i < events.length; i++) {
            const data = events[i].data as any;
            results.push(data.reason.toHuman());
        }
        return [...results];
    }
    return undefined;
}

export async function removeErrorIdentities(
    context: IntegrationTestContext,
    signer: KeyringPair,
    listening: boolean,
    identities: any[]
): Promise<any[] | undefined> {
    let txs: TransactionSubmit[] = [];
    for (let index = 0; index < identities.length; index++) {
        const identity = identities[index];
        const encode = context.substrate.createType('LitentryIdentity', identity).toHex();
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
        const tx = context.substrate.tx.identityManagement.removeIdentity(context.mrEnclave, `0x${ciphertext}`);
        const nonce = await context.substrate.rpc.system.accountNextIndex(signer.address);
        let newNonce = nonce.toNumber() + index;

        txs.push({
            tx,
            nonce: newNonce,
        });
    }

    sendTxUntilInBlockList(context.substrate, txs, signer);

    if (listening) {
        const events = await listenEvent(context.substrate, 'identityManagement', ['StfError']);
        expect(events.length).to.be.equal(identities.length);
        return events;
    }
    return undefined;
}
