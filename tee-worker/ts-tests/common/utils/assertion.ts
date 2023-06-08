import { ApiPromise } from '@polkadot/api';
import { Event, EventRecord } from '@polkadot/types/interfaces';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import Ajv from 'ajv';
import { assert, expect } from 'chai';
import * as ed from '@noble/ed25519';
import { buildIdentityHelper, parseIdGraph, createIdentityEvent, parseIdentity } from './identity-helper';
import type { LitentryPrimitivesIdentity } from '@polkadot/types/lookup';
import type { EnclaveResult, IntegrationTestContext } from '../type-definitions';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { HexString } from '@polkadot/util/types';
import { JsonSchema } from '../type-definitions';
import { env_network } from '../../common/helpers';
import colors from 'colors';

export async function assertInitialIDGraphCreated(
    context: IntegrationTestContext,
    signer: KeyringPair[],
    events: any[],
    aesKey: HexString
) {
    for (let index = 0; index < events.length; index++) {
        const event_data = events[index].data;

        const who = event_data.account.toHex();
        const idGraph_data = parseIdGraph(context.sidechainRegistry, event_data.idGraph, aesKey);
        assert.equal(idGraph_data.length, 1);
        assert.equal(who, u8aToHex(signer[index].addressRaw));

        // Check identity in idgraph
        const expected_identity = await buildIdentityHelper(
            u8aToHex(signer[index].addressRaw),
            env_network,
            'Substrate',
            context
        );
        const expected_target = expected_identity[`as${expected_identity.type}`];
        const idGraph_target = idGraph_data[0][0][`as${idGraph_data[0][0].type}`];
        assert.equal(expected_target.toString(), idGraph_target.toString());

        // Check identityContext in idgraph
        const idGraph_context = idGraph_data[0][1].toHuman();
        const creation_request_block = idGraph_context.creationRequestBlock;
        const verification_request_block = idGraph_context.verificationRequestBlock;
        assert.equal(creation_request_block, 0, 'Check InitialIDGraph error: creation_request_block should be 0');
        assert.equal(
            verification_request_block,
            0,
            'Check InitialIDGraph error: verification_request_block should be 0'
        );
        assert.isTrue(idGraph_context.isVerified, 'Check InitialIDGraph error: isVerified should be true');
    }
    console.log(colors.green('assertInitialIDGraphCreated complete'));
}

export async function assertIdentityLinked(
    context: IntegrationTestContext,
    signers: KeyringPair | KeyringPair[],
    events: any[],
    aesKey: HexString,
    expected_identities: LitentryPrimitivesIdentity[]
) {
    // We should parse idGraph from the last event, because the last event updates the verification status of all identities.
    const event_idGraph = parseIdGraph(context.sidechainRegistry, events[events.length - 1].data.idGraph, aesKey);

    for (let index = 0; index < events.length; index++) {
        const signer = Array.isArray(signers) ? signers[index] : signers;
        const expected_identity = expected_identities[index];
        const expected_identity_target = expected_identity[`as${expected_identity.type}`];

        const event_data = events[index].data;
        const who = event_data.account.toHex();

        // Check prime identity in idGraph
        const expected_prime_identity = await buildIdentityHelper(
            u8aToHex(signer.addressRaw),
            env_network,
            'Substrate',
            context
        );
        assert.equal(
            expected_prime_identity.toString(),
            event_idGraph[events.length][0].toString(),
            'Check IdentityVerified error: event_idGraph prime identity should be equal to expected_prime_identity'
        );

        // Check event identity with expected identity
        const event_identity = parseIdentity(context.sidechainRegistry, event_data.identity, aesKey);
        const event_identity_target = event_identity[`as${event_identity.type}`];
        assert.equal(
            expected_identity_target.toString(),
            event_identity_target.toString(),
            'Check IdentityVerified error: event_identity_target should be equal to expected_identity_target'
        );

        // Check identityContext in idGraph
        assert.isTrue(
            event_idGraph[index][1].isVerified.toHuman(),
            'Check IdentityVerified error: event_idGraph identity should be verified'
        );
        assert(
            Number(event_idGraph[index][1].verificationRequestBlock.toHuman()) > 0,
            'Check IdentityVerified error: event_idGraph verificationRequestBlock should be greater than 0'
        );
        assert(
            Number(event_idGraph[index][1].creationRequestBlock.toHuman()) > 0,
            'Check IdentityVerified error: event_idGraph creationRequestBlock should be greater than 0'
        );

        assert.equal(who, u8aToHex(signer.addressRaw), 'Check IdentityCreated error: signer should be equal to who');
    }
    console.log(colors.green('assertIdentityVerified complete'));
}

export async function assertIdentityCreated(
    context: IntegrationTestContext,
    signers: KeyringPair | KeyringPair[],
    events: any[],
    aesKey: HexString,
    expected_identities: LitentryPrimitivesIdentity[]
) {
    for (let index = 0; index < events.length; index++) {
        const signer = Array.isArray(signers) ? signers[index] : signers;
        const expected_identity = expected_identities[index];
        const expected_identity_target = expected_identity[`as${expected_identity.type}`];
        const event_data = events[index].data;
        const who = event_data.account.toHex();
        const event_identity = parseIdentity(context.sidechainRegistry, event_data.identity, aesKey);
        const event_identity_target = event_identity[`as${event_identity.type}`];
        // Check identity caller
        assert.equal(who, u8aToHex(signer.addressRaw), 'Check IdentityCreated error: signer should be equal to who');

        // Check identity type
        assert.equal(
            expected_identity.type,
            event_identity.type,
            'Check IdentityCreated error: event_identity type should be equal to expected_identity type'
        );
        // Check identity in event
        assert.equal(
            expected_identity_target.toString(),
            event_identity_target.toString(),
            'Check IdentityCreated error: event_identity_target should be equal to expected_identity_target'
        );
    }
    console.log(colors.green('assertIdentityCreated complete'));
}

export async function assertIdentityRemoved(
    context: IntegrationTestContext,
    signers: KeyringPair | KeyringPair[],
    events: any[]
) {
    for (let index = 0; index < events.length; index++) {
        const signer = Array.isArray(signers) ? signers[index] : signers;

        const event_data = events[index].data;
        const who = event_data.account.toHex();

        // Check idGraph
        assert.equal(
            event_data.idGraph,
            null,
            'check IdentityRemoved error: event idGraph should be null after removed identity'
        );

        assert.equal(who, u8aToHex(signer.addressRaw), 'Check IdentityRemoved error: signer should be equal to who');
    }

    console.log(colors.green('assertIdentityRemoved complete'));
}

export async function checkErrorDetail(events: Event[], expectedDetail: string) {
    // TODO: sometimes `item.data.detail.toHuman()` or `item` is treated as object (why?)
    //       I have to JSON.stringify it to assign it to a string
    events.map((item: any) => {
        console.log('error detail: ', item.data.detail.toHuman());
        const detail = JSON.stringify(item.data.detail.toHuman());

        assert.isTrue(
            detail.includes(expectedDetail),
            `check error detail failed, expected detail is ${expectedDetail}, but got ${detail}`
        );
    });
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
