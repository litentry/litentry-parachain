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

    const tx = context.substrate.tx.identityManagement.setUserShieldingKey(context.shard, `0x${ciphertext}`);

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
        context.shard,
        signer.address,
        `0x${ciphertext}`,
        null
    );

    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const events = await listenEvent(context.substrate, 'identityManagement', [
            'IdentityCreated',
            'ChallengeCodeGenerated',
        ]);
        expect(events.length).to.be.equal(2);
        expect(events[0].method).to.be.equal('IdentityCreated');
        expect(events[1].method).to.be.equal('ChallengeCodeGenerated');
        const data0 = events[0].data as any;
        const data1 = events[1].data as any;
        return decodeIdentityEvent(
            context.substrate,
            data0.account.toHex(),
            decryptWithAES(aesKey, data0.identity),
            decryptWithAES(aesKey, data0.idGraph),
            decryptWithAES(aesKey, data1.code)
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

    const tx = context.substrate.tx.identityManagement.removeIdentity(context.shard, `0x${ciphertext}`);

    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const events = await listenEvent(context.substrate, 'identityManagement', ['IdentityRemoved']);
        expect(events.length).to.be.equal(1);
        const data = events[0].data as any;
        return decodeIdentityEvent(
            context.substrate,
            data.account.toHex(),
            decryptWithAES(aesKey, data.identity),
            decryptWithAES(aesKey, data.idGraph)
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
        context.shard,
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
            decryptWithAES(aesKey, data.identity),
            decryptWithAES(aesKey, data.idGraph)
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
    shard: HexString,
    assertion: Assertion
): Promise<HexString[] | undefined> {
    const tx = context.substrate.tx.vcManagement.requestVc(shard, assertion);

    await sendTxUntilInBlock(context.substrate, tx, signer);
    if (listening) {
        const events = await listenEvent(context.substrate, 'vcManagement', ['VCIssued']);
        expect(events.length).to.be.equal(1);
        const data = events[0].data as any;
        return [data.account.toHex(), data.index.toHex(), data.vc];
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
