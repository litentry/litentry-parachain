import { encryptWithTeeShieldingKey } from './common/utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';
import { Event } from '@polkadot/types/interfaces';

import {
    Assertion,
    IntegrationTestContext,
    LitentryIdentity,
    LitentryValidationData,
    TransactionSubmit,
} from './common/type-definitions';
import { expect } from 'chai';
import { listenEvent, sendTxUntilInBlock, sendTxUntilInBlockList } from './common/transactions';

export async function setErrorUserShieldingKey(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean
): Promise<string | undefined> {
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, aesKey).toString('hex');
    const tx = context.api.tx.identityManagement.setUserShieldingKey(context.mrEnclave, `0x${ciphertext}`);

    await sendTxUntilInBlock(context.api, tx, signer);

    if (listening) {
        const events = await listenEvent(context.api, 'identityManagement', ['SetUserShieldingKeyHandlingFailed']);
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
    const nonce = await context.api.rpc.system.accountNextIndex(signer.address);

    for (let k = 0; k < errorCiphertexts.length; k++) {
        const errorCiphertext = errorCiphertexts[k];
        const tx = context.api.tx.identityManagement.createIdentity(
            context.mrEnclave,
            signer.address,
            errorCiphertext,
            null
        );

        let newNonce = nonce.toNumber() + k;
        txs.push({
            tx,
            nonce: newNonce,
        });
    }

    await sendTxUntilInBlockList(context.api, txs, signer);

    if (listening) {
        const events = (await listenEvent(context.api, 'identityManagement', ['CreateIdentityHandlingFailed'])) as any;
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
    const nonce = await context.api.rpc.system.accountNextIndex(signer.address);

    for (let k = 0; k < identities.length; k++) {
        let identity = identities[k];
        let data = datas[k];
        const identity_encode = context.api.createType('LitentryIdentity', identity).toHex();
        const validation_encode = context.api.createType('LitentryValidationData', data).toHex();
        const identity_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, identity_encode).toString(
            'hex'
        );
        const validation_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, validation_encode).toString(
            'hex'
        );

        const tx = context.api.tx.identityManagement.verifyIdentity(
            context.mrEnclave,
            `0x${identity_ciphertext}`,
            `0x${validation_ciphertext}`
        );

        let newNonce = nonce.toNumber() + k;
        txs.push({
            tx,
            nonce: newNonce,
        });
    }

    await sendTxUntilInBlockList(context.api, txs, signer);

    if (listening) {
        const events = (await listenEvent(context.api, 'identityManagement', ['StfError'])) as any;
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
    const nonce = await context.api.rpc.system.accountNextIndex(signer.address);

    for (let index = 0; index < identities.length; index++) {
        const identity = identities[index];
        const encode = context.api.createType('LitentryIdentity', identity).toHex();
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
        const tx = context.api.tx.identityManagement.removeIdentity(context.mrEnclave, `0x${ciphertext}`);
        let newNonce = nonce.toNumber() + index;

        txs.push({
            tx,
            nonce: newNonce,
        });
    }

    await sendTxUntilInBlockList(context.api, txs, signer);

    if (listening) {
        const events = await listenEvent(context.api, 'identityManagement', ['StfError']) as any;
        let results: string[] = [];
        expect(events.length).to.be.equal(identities.length);
        for (let i = 0; i < events.length; i++) {
            const data = events[i].data as any;
            results.push(data.reason.toHuman());
        }
        return [...results];

    }
    return undefined;
}
export async function requestErrorVCs(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    mrEnclave: HexString,
    assertion: Assertion,
    keys: string[]
): Promise<Event[] | undefined> {
    let txs: TransactionSubmit[] = [];
    const nonce = await context.api.rpc.system.accountNextIndex(signer.address);

    for (let index = 0; index < keys.length; index++) {
        const key = keys[index];
        const tx = context.api.tx.vcManagement.requestVc(mrEnclave, {
            [key]: assertion[key as keyof Assertion],
        });
        let newNonce = nonce.toNumber() + index;
        txs.push({ tx, nonce: newNonce });
    }

    await sendTxUntilInBlockList(context.api, txs, signer);

    if (listening) {
        const events = (await listenEvent(context.api, 'vcManagement', ['StfError'])) as Event[];
        expect(events.length).to.be.equal(keys.length);
        return events;
    }
    return undefined;
}
export async function disableErrorVCs(
    context: IntegrationTestContext,
    signer: KeyringPair,
    listening: boolean,
    indexList: HexString[]
): Promise<string[] | undefined> {
    let txs: TransactionSubmit[] = [];
    const nonce = await context.api.rpc.system.accountNextIndex(signer.address);

    for (let k = 0; k < indexList.length; k++) {
        const tx = context.api.tx.vcManagement.disableVc(indexList[k]);
        let newNonce = nonce.toNumber() + k;
        txs.push({ tx, nonce: newNonce });
    }

    const res = (await sendTxUntilInBlockList(context.api, txs, signer)) as string[];

    return res.length ? res : undefined;
}
export async function revokeErrorVCs(
    context: IntegrationTestContext,
    signer: KeyringPair,
    listening: boolean,
    indexList: HexString[]
): Promise<string[] | undefined> {
    let txs: TransactionSubmit[] = [];
    const nonce = await context.api.rpc.system.accountNextIndex(signer.address);

    for (let k = 0; k < indexList.length; k++) {
        const tx = context.api.tx.vcManagement.revokeVc(indexList[k]);
        let newNonce = nonce.toNumber() + k;
        txs.push({ tx, nonce: newNonce });
    }

    const res = (await sendTxUntilInBlockList(context.api, txs, signer)) as string[];

    return res.length ? res : undefined;
}
