import {
    IdentityGenericEvent,
    IntegrationTestContext,
    LitentryIdentity,
    LitentryValidationData,
    Assertion,
    TransactionSubmit,
} from './common/type-definitions';
import { createIdentityEvent, decryptWithAES, encryptWithTeeShieldingKey } from './common/utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';
import { u8aToHex } from '@polkadot/util';
import { ApiPromise } from '@polkadot/api';
import { expect, assert } from 'chai';
import { listenEvent, sendTxUntilInBlock, sendTxUntilInBlockList } from './common/transactions';

export async function setUserShieldingKey(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean
): Promise<HexString | undefined> {
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, aesKey).toString('hex');

    const tx = context.api.tx.identityManagement.setUserShieldingKey(context.mrEnclave, `0x${ciphertext}`);

    await sendTxUntilInBlock(context.api, tx, signer);

    if (listening) {
        const events = await listenEvent(context.api, 'identityManagement', ['UserShieldingKeySet'], 1, [u8aToHex(signer.addressRaw)]);
        expect(events.length).to.be.equal(1);
        return (events[0].data as any).account.toHex();
    }
    return undefined;
}

export async function multiAccountTxSender(
    context: IntegrationTestContext,
    txs: TransactionSubmit[],
    signers: KeyringPair | KeyringPair[],
    pallet: string,
    events: string[]
): Promise<any> {
    let signers_hex: HexString[] = [];
    if (Array.isArray(signers)) {
        for (let index = 0; index < signers.length; index++) {
            signers_hex.push(
                u8aToHex(signers[index].addressRaw)
            )
        }
    } else {
        signers_hex.push(u8aToHex(signers.addressRaw));
    }
    console.log(888, txs);

    await sendTxUntilInBlockList(context.api, txs, signers);
    const resp_events = await listenEvent(context.api, pallet, events, txs.length, signers_hex);
    expect(resp_events.length).to.be.equal(txs.length);
    return resp_events.length ? resp_events : undefined;
}

export async function createIdentities(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identities: LitentryIdentity[]
): Promise<IdentityGenericEvent[] | undefined> {
    let txs: TransactionSubmit[] = [];
    for (let index = 0; index < identities.length; index++) {
        const identity = identities[index];
        const encode = context.api.createType('LitentryIdentity', identity).toHex();
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
        const tx = context.api.tx.identityManagement.createIdentity(
            context.mrEnclave,
            signer.address,
            `0x${ciphertext}`,
            null
        );
        const nonce = await context.api.rpc.system.accountNextIndex(signer.address);
        let newNonce = nonce.toNumber() + index;
        txs.push({ tx, nonce: newNonce });
    }

    await sendTxUntilInBlockList(context.api, txs, signer);

    if (listening) {
        const events = (await listenEvent(context.api, 'identityManagement', ['IdentityCreated'], txs.length, [u8aToHex(signer.addressRaw)])) as any;
        expect(events.length).to.be.equal(identities.length);

        let results: IdentityGenericEvent[] = [];

        for (let index = 0; index < events.length; index++) {
            results.push(
                createIdentityEvent(
                    context.api,
                    events[index].data.account.toHex(),
                    decryptWithAES(aesKey, events[index].data.identity, 'hex'),
                    undefined,
                    decryptWithAES(aesKey, events[index].data.code, 'hex')
                )
            );
        }
        return [...results];
    }
    return undefined;
}
export async function removeIdentities(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identity: LitentryIdentity[]
): Promise<IdentityGenericEvent[] | undefined> {
    let txs: TransactionSubmit[] = [];
    for (let index = 0; index < identity.length; index++) {
        const encode = context.api.createType('LitentryIdentity', identity[index]).toHex();
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');

        const tx = context.api.tx.identityManagement.removeIdentity(context.mrEnclave, `0x${ciphertext}`);
        const nonce = await context.api.rpc.system.accountNextIndex(signer.address);
        let newNonce = nonce.toNumber() + index;
        txs.push({ tx, nonce: newNonce });
    }

    await sendTxUntilInBlockList(context.api, txs, signer);

    if (listening) {
        const events = (await listenEvent(context.api, 'identityManagement', ['IdentityRemoved'], txs.length, [u8aToHex(signer.addressRaw)])) as any;
        expect(events.length).to.be.equal(identity.length);

        let results: IdentityGenericEvent[] = [];

        for (let index = 0; index < events.length; index++) {
            results.push(
                createIdentityEvent(
                    context.api,
                    events[index].data.account.toHex(),
                    decryptWithAES(aesKey, events[index].data.identity, 'hex')
                )
            );
        }
        return [...results];
    }
    return undefined;
}

export async function verifyIdentities(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identities: LitentryIdentity[],
    datas: LitentryValidationData[]
): Promise<IdentityGenericEvent[] | undefined> {
    let txs: TransactionSubmit[] = [];
    for (let index = 0; index < identities.length; index++) {
        let identity = identities[index];
        let data = datas[index];
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
        const nonce = await context.api.rpc.system.accountNextIndex(signer.address);
        let newNonce = nonce.toNumber() + index;
        txs.push({ tx, nonce: newNonce });
    }

    await sendTxUntilInBlockList(context.api, txs, signer);

    if (listening) {
        const events = (await listenEvent(context.api, 'identityManagement', ['IdentityVerified'], txs.length, [u8aToHex(signer.addressRaw)])) as any;
        expect(events.length).to.be.equal(identities.length);
        let results: IdentityGenericEvent[] = [];
        for (let index = 0; index < events.length; index++) {
            results.push(
                createIdentityEvent(
                    context.api,
                    events[index].data.account.toHex(),
                    decryptWithAES(aesKey, events[index].data.identity, 'hex'),
                    decryptWithAES(aesKey, events[index].data.idGraph, 'hex')
                )
            );
        }
        return [...results];
    }
    return undefined;
}

//vcManagement
export async function requestVCs(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    mrEnclave: HexString,
    assertion: Assertion,
    keys: string[]
): Promise<
    | {
        account: HexString;
        index: HexString;
        vc: HexString;
    }[]
    | undefined
> {
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
        const events = (await listenEvent(context.api, 'vcManagement', ['VCIssued'], txs.length, [u8aToHex(signer.addressRaw)])) as any;
        expect(events.length).to.be.equal(keys.length);

        let results: {
            account: HexString;
            index: HexString;
            vc: HexString;
        }[] = [];
        for (let k = 0; k < events.length; k++) {
            results.push({
                account: events[k].data.account.toHex(),
                index: events[k].data.index.toHex(),
                vc: decryptWithAES(aesKey, events[k].data.vc, 'utf-8'),
            });
        }
        return [...results];
    }
    return undefined;
}

export async function disableVCs(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    indexList: HexString[]
): Promise<HexString[] | undefined> {
    let txs: TransactionSubmit[] = [];
    for (let k = 0; k < indexList.length; k++) {
        const tx = context.api.tx.vcManagement.disableVc(indexList[k]);
        const nonce = await context.api.rpc.system.accountNextIndex(signer.address);
        let newNonce = nonce.toNumber() + k;
        txs.push({ tx, nonce: newNonce });
    }

    await sendTxUntilInBlockList(context.api, txs, signer);
    if (listening) {
        const events = (await listenEvent(context.api, 'vcManagement', ['VCDisabled'], txs.length, [u8aToHex(signer.addressRaw)])) as any;
        expect(events.length).to.be.equal(indexList.length);
        let results: HexString[] = [];
        for (let m = 0; m < events.length; m++) {
            results.push(events[m].data.index.toHex());
        }

        return [...results];
    }
    return undefined;
}

export async function revokeVCs(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    indexList: HexString[]
): Promise<HexString[] | undefined> {
    let txs: TransactionSubmit[] = [];
    for (let k = 0; k < indexList.length; k++) {
        const tx = context.api.tx.vcManagement.revokeVc(indexList[k]);
        const nonce = await context.api.rpc.system.accountNextIndex(signer.address);
        let newNonce = nonce.toNumber() + k;
        txs.push({ tx, nonce: newNonce });
    }

    await sendTxUntilInBlockList(context.api, txs, signer);
    if (listening) {
        const events = (await listenEvent(context.api, 'vcManagement', ['VCRevoked'], txs.length, [u8aToHex(signer.addressRaw)])) as any;
        expect(events.length).to.be.equal(indexList.length);
        let results: HexString[] = [];
        for (let m = 0; m < events.length; m++) {
            results.push(events[m].data.index.toHex());
        }
        return [...results];
    }
    return undefined;
}

export function assertIdentityCreated(signer: KeyringPair, identityEvent: IdentityGenericEvent | undefined) {
    assert.equal(identityEvent?.who, u8aToHex(signer.addressRaw), 'check caller error');
}

export function assertIdentityVerified(signer: KeyringPair, identityEvent: IdentityGenericEvent | undefined) {
    let idGraphExist = false;

    if (identityEvent) {
        for (let i = 0; i < identityEvent.idGraph.length; i++) {
            if (JSON.stringify(identityEvent.idGraph[i][0]) == JSON.stringify(identityEvent.identity)) {
                idGraphExist = true;
                assert.isTrue(identityEvent.idGraph[i][1].is_verified, 'identity should be verified');
            }
        }
    }
    assert.isTrue(idGraphExist, 'id_graph should exist');
    assert.equal(identityEvent?.who, u8aToHex(signer.addressRaw), 'check caller error');
}

export function assertIdentityRemoved(signer: KeyringPair, identityEvent: IdentityGenericEvent | undefined) {
    assert.equal(identityEvent?.who, u8aToHex(signer.addressRaw), 'check caller error');
}
