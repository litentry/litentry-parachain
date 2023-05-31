import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';
import { Event } from '@polkadot/types/interfaces';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import Ajv from 'ajv';
import { assert, expect } from 'chai';
import * as ed from '@noble/ed25519';
import { EnclaveResult, IdentityGenericEvent, JsonSchema, IntegrationTestContext } from '../type-definitions';
import { buildIdentityHelper } from './identity-helper';
import { LitentryPrimitivesIdentity } from '@polkadot/types/lookup';
import { isEqual, isArrayEqual } from './common';
export async function assertInitialIDGraphCreated(
    context: IntegrationTestContext,
    signer: KeyringPair,
    event: IdentityGenericEvent
) {
    assert.equal(event.who, u8aToHex(signer.addressRaw));
    assert.equal(event.idGraph.length, 1);
    // check identity in idgraph
    const expected_identity: LitentryPrimitivesIdentity = context.sidechainRegistry.createType(
        'LitentryPrimitivesIdentity',
        await buildIdentityHelper(
            u8aToHex(signer.addressRaw),
            process.env.NODE_ENV === 'local' ? 'TestNet' : 'LitentryRococo',
            'Substrate',
            context
        )
    ) as any;

    const expected_target = expected_identity[`as${expected_identity.type}`];
    const idGraph_target = event.idGraph[0][0][`as${event.idGraph[0][0].type}`];

    assert.equal(expected_target.toString(), idGraph_target.toString());

    // check identityContext in idgraph
    const idGraph_context = event.idGraph[0][1].toHuman();

    const creation_request_block = idGraph_context.creationRequestBlock;
    const verification_request_block = idGraph_context.verificationRequestBlock;

    assert.equal(creation_request_block, 0);
    assert.equal(verification_request_block, 0);

    assert.isTrue(idGraph_context.isVerified);
}

export function assertIdentityVerified(signer: KeyringPair, eventDatas: IdentityGenericEvent[]) {
    let event_identities: LitentryPrimitivesIdentity[] = [];
    let idgraph_identities: LitentryPrimitivesIdentity[] = [];
    for (let index = 0; index < eventDatas.length; index++) {
        event_identities.push(eventDatas[index].identity);
    }
    for (let i = 0; i < eventDatas[eventDatas.length - 1].idGraph.length; i++) {
        idgraph_identities.push(eventDatas[eventDatas.length - 1].idGraph[i][0]);
    }
    //idgraph_identities[idgraph_identities.length - 1] is prime identity,don't need to compare
    assert.isTrue(
        isArrayEqual(event_identities, idgraph_identities.slice(0, idgraph_identities.length - 1)),
        'event identities should be equal to idgraph identities'
    );

    const data = eventDatas[eventDatas.length - 1];
    for (let i = 0; i < eventDatas[eventDatas.length - 1].idGraph.length; i++) {
        if (isEqual(data.idGraph[i][0], data.identity)) {
            assert.isTrue(data.idGraph[i][1].isVerified, 'identity should be verified');
        }
    }
    assert.equal(data?.who, u8aToHex(signer.addressRaw), 'check caller error');
}

export function assertIdentityCreated(signer: KeyringPair, identityEvent: IdentityGenericEvent) {
    assert.equal(identityEvent?.who, u8aToHex(signer.addressRaw), 'check caller error');
}

export function assertIdentityRemoved(signer: KeyringPair, identityEvent: IdentityGenericEvent) {
    assert.equal(identityEvent?.idGraph, null, 'check idGraph error,should be null after removed');
    assert.equal(identityEvent?.who, u8aToHex(signer.addressRaw), 'check caller error');
}

export async function checkErrorDetail(
    response: string[] | Event[],
    expectedDetail: string,
    isModule: boolean
): Promise<boolean> {
    let detail: string = '';
    // TODO: sometimes `item.data.detail.toHuman()` or `item` is treated as object (why?)
    //       I have to JSON.stringify it to assign it to a string
    response.map((item: any) => {
        isModule ? (detail = JSON.stringify(item.data.detail.toHuman())) : (detail = JSON.stringify(item));
        assert.isTrue(
            detail.includes(expectedDetail),
            `check error detail failed, expected detail is ${expectedDetail}, but got ${detail}`
        );
    });
    return true;
}

export async function verifySignature(data: any, index: HexString, proofJson: any, api: ApiPromise) {
    const count = await api.query.teerex.enclaveCount();
    const res = (await api.query.teerex.enclaveRegistry(count)).toHuman() as EnclaveResult;
    //check vc index
    expect(index).to.be.eq(data.id);

    const signature = Buffer.from(hexToU8a(`0x${proofJson.proofValue}`));
    const message = Buffer.from(JSON.stringify(data));
    const vcPubkey = Buffer.from(hexToU8a(`${res.vcPubkey}`));

    const isValid = await ed.verify(signature, message, vcPubkey);

    expect(isValid).to.be.true;
    return true;
}

export async function checkVc(vcObj: any, index: HexString, proof: any, api: ApiPromise): Promise<boolean> {
    const vc = JSON.parse(JSON.stringify(vcObj));
    delete vc.proof;
    const signatureValid = await verifySignature(vc, index, proof, api);
    expect(signatureValid).to.be.true;

    const jsonValid = await checkJSON(vcObj, proof);
    expect(jsonValid).to.be.true;
    return true;
}

//Check VC json fields
export async function checkJSON(vc: any, proofJson: any): Promise<boolean> {
    //check JsonSchema
    const ajv = new Ajv();
    const validate = ajv.compile(JsonSchema);
    const isValid = validate(vc);
    expect(isValid).to.be.true;
    expect(
        vc.type[0] === 'VerifiableCredential' &&
        vc.issuer.id === proofJson.verificationMethod &&
        proofJson.type === 'Ed25519Signature2020'
    ).to.be.true;
    return true;
}
