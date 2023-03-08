import {
    IdentityGenericEvent,
    IntegrationTestContext,
    LitentryIdentity,
    LitentryValidationData,
    Assertion,
} from './type-definitions';
import {
    decryptWithAES,
    encryptWithTeeShieldingKey,
    listenEvent,
    sendTxUntilInBlock,
    sendTxUntilInBlockList,
} from './utils';
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
export async function createIdentities(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identities: LitentryIdentity[]
): Promise<IdentityGenericEvent[] | undefined> {
    let txs: any[] = [];
    for (let index = 0; index < identities.length; index++) {
        const identity = identities[index];
        const encode = context.substrate.createType('LitentryIdentity', identity).toHex();
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
        const tx = context.substrate.tx.identityManagement.createIdentity(
            context.mrEnclave,
            signer.address,
            `0x${ciphertext}`,
            null
        );
        const nonce = await context.substrate.rpc.system.accountNextIndex(signer.address);
        let newNonce = nonce.toNumber() + index;
        txs.push({ tx, nonce: newNonce });
    }

    await sendTxUntilInBlockList(context.substrate, txs, signer);

    if (listening) {
        const events = (await listenEvent(context.substrate, 'identityManagement', ['IdentityCreated'])) as any;
        expect(events.length).to.be.equal(identities.length);

        let results: IdentityGenericEvent[] = [];

        for (let index = 0; index < events.length; index++) {
            results.push(
                decodeIdentityEvent(
                    context.substrate,
                    events[index].data.account.toHex(),
                    decryptWithAES(aesKey, events[index].data.identity, 'hex'),
                    decryptWithAES(aesKey, events[index].data.idGraph, 'hex'),
                    decryptWithAES(aesKey, events[index].data.code, 'hex')
                )
            );
        }
        return [...results];
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
export async function removeIdentities(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identity: LitentryIdentity[]
): Promise<IdentityGenericEvent[] | undefined> {
    let txs: any[] = [];
    for (let index = 0; index < identity.length; index++) {
        const encode = context.substrate.createType('LitentryIdentity', identity[index]).toHex();
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');

        const tx = context.substrate.tx.identityManagement.removeIdentity(context.mrEnclave, `0x${ciphertext}`);
        const nonce = await context.substrate.rpc.system.accountNextIndex(signer.address);
        let newNonce = nonce.toNumber() + index;
        txs.push({ tx, nonce: newNonce });
    }

    await sendTxUntilInBlockList(context.substrate, txs, signer);

    if (listening) {
        const events = (await listenEvent(context.substrate, 'identityManagement', ['IdentityRemoved'])) as any;
        expect(events.length).to.be.equal(identity.length);

        let results: IdentityGenericEvent[] = [];

        for (let index = 0; index < events.length; index++) {
            results.push(
                decodeIdentityEvent(
                    context.substrate,
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

export async function verifyIdentities(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identities: LitentryIdentity[],
    datas: LitentryValidationData[]
): Promise<IdentityGenericEvent[] | undefined> {
    let txs: any[] = [];
    for (let index = 0; index < identities.length; index++) {
        let identity = identities[index];

        let data = datas[index];

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
        let newNonce = nonce.toNumber() + index;
        txs.push({ tx, nonce: newNonce });
    }

    await sendTxUntilInBlockList(context.substrate, txs, signer);

    if (listening) {
        const events = (await listenEvent(context.substrate, 'identityManagement', ['IdentityVerified'])) as any;

        expect(events.length).to.be.equal(identities.length);

        let results: IdentityGenericEvent[] = [];

        for (let index = 0; index < events.length; index++) {
            results.push(
                decodeIdentityEvent(
                    context.substrate,
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
export async function requestVCs(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    mrEnclave: HexString,
    assertion: Assertion
): Promise<
    | {
          account: HexString;
          index: HexString;
          vc: HexString;
      }[]
    | undefined
> {
    let txs: any[] = [];
    let len = 0;

    for (const key in assertion) {
        len++;
        const tx = context.substrate.tx.vcManagement.requestVc(mrEnclave, {
            [key]: assertion[key as keyof Assertion],
        });
        const nonce = await context.substrate.rpc.system.accountNextIndex(signer.address);

        let newNonce = nonce.toNumber() + (len - 1);
        txs.push({ tx, nonce: newNonce });
    }

    await sendTxUntilInBlockList(context.substrate, txs, signer);
    if (listening) {
        const events = (await listenEvent(context.substrate, 'vcManagement', ['VCIssued'])) as any;
        expect(events.length).to.be.equal(len);

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
        // const [account, index, vc] = events[0].data as any;
        // return [account.toHex(), index.toHex(), decryptWithAES(aesKey, vc, 'utf-8')];
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
export async function disableVCs(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    indexList: HexString[]
): Promise<HexString[] | undefined> {
    let txs: any[] = [];

    for (let k = 0; k < indexList.length; k++) {
        const tx = context.substrate.tx.vcManagement.disableVc(indexList[k]);
        const nonce = await context.substrate.rpc.system.accountNextIndex(signer.address);
        let newNonce = nonce.toNumber() + k;
        txs.push({ tx, nonce: newNonce });
    }

    await sendTxUntilInBlockList(context.substrate, txs, signer);
    if (listening) {
        const events = (await listenEvent(context.substrate, 'vcManagement', ['VCDisabled'])) as any;
        expect(events.length).to.be.equal(indexList.length);
        let results: HexString[] = [];
        for (let m = 0; m < events.length; m++) {
            results.push(events[m].data.index.toHex());
        }

        return [...results];
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
export async function revokeVCs(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    indexList: HexString[]
): Promise<HexString[] | undefined> {
    let txs: any[] = [];
    for (let k = 0; k < indexList.length; k++) {
        const tx = context.substrate.tx.vcManagement.revokeVc(indexList[k]);
        const nonce = await context.substrate.rpc.system.accountNextIndex(signer.address);
        let newNonce = nonce.toNumber() + k;
        txs.push({ tx, nonce: newNonce });
    }

    await sendTxUntilInBlockList(context.substrate, txs, signer);
    if (listening) {
        const events = (await listenEvent(context.substrate, 'vcManagement', ['VCRevoked'])) as any;
        expect(events.length).to.be.equal(indexList.length);
        let results: HexString[] = [];
        for (let m = 0; m < events.length; m++) {
            results.push(events[m].data.index.toHex());
        }
        return [...results];
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
