import {
    IdentityGenericEvent,
    IntegrationTestContext,
    LitentryIdentity,
    LitentryValidationData,
} from './type-definitions';
import {
    encryptWithTeeShieldingKey,
    listenEncryptedEvents,
    sendTxUntilInBlock,
    listenCreatedIdentityEvents,
} from './utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';
import { ApiPromise } from '@polkadot/api';
import { Assertion } from './type-definitions';

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
        const eventData = await listenEncryptedEvents(context, aesKey, {
            module: 'identityManagement',
            method: 'userShieldingKeySet',
            event: 'UserShieldingKeySet',
        });
        const [who] = eventData as HexString[];
        return who;
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
        const event = await listenCreatedIdentityEvents(context, aesKey);
        const [who, _identity, idGraph, challengeCode] = event.eventData;
        return decodeIdentityEvent(context.substrate, who, _identity, idGraph, challengeCode);
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
        const eventData = await listenEncryptedEvents(context, aesKey, {
            module: 'identityManagement',
            method: 'identityRemoved',
            event: 'IdentityRemoved',
        });
        const [who, identity, idGraph] = eventData as HexString[];
        return decodeIdentityEvent(context.substrate, who, identity, idGraph);
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
        const eventData = await listenEncryptedEvents(context, aesKey, {
            module: 'identityManagement',
            method: 'identityVerified',
            event: 'IdentityVerified',
        });
        const [who, identity, idGraph] = eventData as HexString[];

        return decodeIdentityEvent(context.substrate, who, identity, idGraph);
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
        const eventData = await listenEncryptedEvents(context, aesKey, {
            module: 'vcManagement',
            method: 'vcIssued',
            event: 'VCIssued',
        });

        const [who, index, vc] = eventData as HexString[];
        return [who, index, vc];
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
        const eventData = await listenEncryptedEvents(context, aesKey, {
            module: 'vcManagement',
            method: 'disableVc',
            event: 'VCDisabled',
        });

        const [index] = eventData as HexString[];
        return index;
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
        const eventData = await listenEncryptedEvents(context, aesKey, {
            module: 'vcManagement',
            method: 'revokeVc',
            event: 'VCRevoked',
        });

        const [index] = eventData as HexString[];
        return index;
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
