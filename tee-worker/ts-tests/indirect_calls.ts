import {
    IdentityGenericEvent,
    IntegrationTestContext,
    LitentryIdentity,
    LitentryValidationData,
    Assertion,
} from './type-definitions';
import { decryptWithAES, encryptWithTeeShieldingKey, listenEvent, sendTxUntilInBlock } from './utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';
import { ApiPromise } from '@polkadot/api';
import { expect } from 'chai';

export async function setUserShieldingKey(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean
): Promise<HexString | undefined> {
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, aesKey).toString('hex');

    const tx = context.substrate.tx.identityManagement.setUserShieldingKey(context.mrEnclave, `0x${ciphertext}`);

    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const events = await listenEvent(context.substrate, 'identityManagement', ['UserShieldingKeySet']);
        expect(events.length).to.be.equal(1);
        return (events[0].data as any).account.toHex();
    }
    return undefined;
}

export async function createIdentity(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identity: LitentryIdentity
): Promise<IdentityGenericEvent | undefined> {
    const encode = context.substrate.createType('LitentryIdentity', identity).toHex();
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');

    const tx = context.substrate.tx.identityManagement.createIdentity(
        context.mrEnclave,
        signer.address,
        `0x${ciphertext}`,
        null
    );

    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const events = await listenEvent(context.substrate, 'identityManagement', ['IdentityCreated']);
        expect(events.length).to.be.equal(1);
        const data = events[0].data as any;
        return decodeIdentityEvent(
            context.substrate,
            data.account.toHex(),
            decryptWithAES(aesKey, data.identity, 'hex'),
            decryptWithAES(aesKey, data.idGraph, 'hex'),
            decryptWithAES(aesKey, data.code, 'hex')
        );
    }
    return undefined;
}

export async function removeIdentity(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identity: LitentryIdentity
): Promise<IdentityGenericEvent | undefined> {
    const encode = context.substrate.createType('LitentryIdentity', identity).toHex();
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');

    const tx = context.substrate.tx.identityManagement.removeIdentity(context.mrEnclave, `0x${ciphertext}`);

    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const events = await listenEvent(context.substrate, 'identityManagement', ['IdentityRemoved']);
        expect(events.length).to.be.equal(1);
        const data = events[0].data as any;
        return decodeIdentityEvent(
            context.substrate,
            data.account.toHex(),
            decryptWithAES(aesKey, data.identity, 'hex'),
            decryptWithAES(aesKey, data.idGraph, 'hex')
        );
    }
    return undefined;
}

export async function verifyIdentity(
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
        const events = await listenEvent(context.substrate, 'identityManagement', ['IdentityVerified']);
        expect(events.length).to.be.equal(1);
        const data = events[0].data as any;
        return decodeIdentityEvent(
            context.substrate,
            data.account.toHex(),
            decryptWithAES(aesKey, data.identity, 'hex'),
            decryptWithAES(aesKey, data.idGraph, 'hex')
        );
    }
    return undefined;
}

//vcManagement
export async function requestVC(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    mrEnclave: HexString,
    assertion: Assertion
): Promise<HexString[] | undefined> {
    const tx = context.substrate.tx.vcManagement.requestVc(mrEnclave, assertion);

    await sendTxUntilInBlock(context.substrate, tx, signer);
    if (listening) {
        const events = await listenEvent(context.substrate, 'vcManagement', ['VCIssued']);
        expect(events.length).to.be.equal(1);
        const [account, index, vc] = events[0].data as any;
        return [account.toHex(), index.toHex(), decryptWithAES(aesKey, vc, 'utf-8')];
    }
    return undefined;
}

export async function disableVC(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    index: HexString
): Promise<HexString | undefined> {
    const tx = context.substrate.tx.vcManagement.disableVc(index);

    await sendTxUntilInBlock(context.substrate, tx, signer);
    if (listening) {
        const events = await listenEvent(context.substrate, 'vcManagement', ['VCDisabled']);
        expect(events.length).to.be.equal(1);
        const data = events[0].data as any;
        return data.index.toHex();
    }
    return undefined;
}

export async function revokeVC(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    index: HexString
): Promise<HexString | undefined> {
    const tx = context.substrate.tx.vcManagement.revokeVc(index);

    await sendTxUntilInBlock(context.substrate, tx, signer);
    if (listening) {
        const events = await listenEvent(context.substrate, 'vcManagement', ['VCRevoked']);
        expect(events.length).to.be.equal(1);
        const data = events[0].data as any;
        return data.index.toHex();
    }
    return undefined;
}

function decodeIdentityEvent(
    api: ApiPromise,
    who: HexString,
    identityString: HexString,
    idGraphString: HexString,
    challengeCode?: HexString
): IdentityGenericEvent {
    let identity = api.createType('LitentryIdentity', identityString).toJSON();
    let idGraph = api.createType('Vec<(LitentryIdentity, IdentityContext)>', idGraphString).toJSON();
    return <IdentityGenericEvent>{
        who,
        identity,
        idGraph,
        challengeCode,
    };
}
