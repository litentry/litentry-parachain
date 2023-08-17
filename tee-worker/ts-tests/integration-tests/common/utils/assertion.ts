import { ApiPromise } from '@polkadot/api';
import { Event } from '@polkadot/types/interfaces';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import Ajv from 'ajv';
import { assert, expect } from 'chai';
import * as ed from '@noble/ed25519';
import { buildIdentityHelper, parseIdGraph, parseIdentity } from './identity-helper';
import type { LitentryPrimitivesIdentity, PalletIdentityManagementTeeError } from 'sidechain-api';
import type { EnclaveResult, IntegrationTestContext } from '../type-definitions';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { HexString } from '@polkadot/util/types';
import { jsonSchema } from '../type-definitions';
import { aesKey } from '../call';
import colors from 'colors';
import { CorePrimitivesErrorErrorDetail, FrameSystemEventRecord, WorkerRpcReturnValue, RequestVCResponse, PalletVcManagementVcContext, TeerexPrimitivesEnclave } from 'parachain-api';
import { Bytes } from '@polkadot/types-codec';
import { PolkadotSigner, Signer, decryptWithAes } from './crypto';
import { blake2AsHex } from '@polkadot/util-crypto';
import { decodeAddress } from '@polkadot/keyring';
export async function assertFailedEvent(
    context: IntegrationTestContext,
    events: FrameSystemEventRecord[],
    eventType: 'LinkIdentityFailed' | 'DeactivateIdentityFailed',
    expectedEvent: CorePrimitivesErrorErrorDetail['type'] | PalletIdentityManagementTeeError['type']
) {
    const failedType = context.api.events.identityManagement[eventType];
    const isFailed = failedType.is.bind(failedType);
    type EventLike = Parameters<typeof isFailed>[0];
    const ievents: EventLike[] = events.map(({ event }) => event);
    const failedEvent = ievents.filter(isFailed);
    /* 
      @fix Why this type don't work?????? https://github.com/litentry/litentry-parachain/issues/1917
    */
    const eventData = failedEvent[0].data[1] as CorePrimitivesErrorErrorDetail;
    assert.lengthOf(failedEvent, 1);
    if (eventData.isStfError) {
        assert.equal(
            eventData.asStfError.toHuman(),
            expectedEvent,
            `check event detail is ${expectedEvent}, but is ${eventData.asStfError.toHuman()}`
        );
    } else {
        assert.equal(
            eventData.type,
            expectedEvent,
            `check event detail is  ${expectedEvent}, but is ${eventData.type}`
        );
    }
}

export async function assertInitialIdGraphCreated(
    context: IntegrationTestContext,
    signer: KeyringPair[],
    events: any[]
) {
    assert.isAtLeast(events.length, 1, 'Check InitialIDGraph error: events length should be greater than 1');
    for (let index = 0; index < events.length; index++) {
        const eventData = events[index].data;

        const who = eventData.account.toHex();
        const idGraphData = parseIdGraph(context.sidechainRegistry, eventData.idGraph, aesKey);
        assert.equal(idGraphData.length, 1);
        assert.equal(who, u8aToHex(signer[index].addressRaw));

        // Check identity in idgraph
        const expectedIdentity = await buildIdentityHelper(u8aToHex(signer[index].addressRaw), 'Substrate', context);
        const expectedTarget = expectedIdentity[`as${expectedIdentity.type}`];
        const idGraphTarget = idGraphData[0][0][`as${idGraphData[0][0].type}`];
        assert.equal(expectedTarget.toString(), idGraphTarget.toString());

        // Check identityContext in idgraph
        const idGraphContext = idGraphData[0][1];
        assert.isTrue(
            idGraphContext.linkBlock.toNumber() > 0,
            'Check InitialIDGraph error: link_block should be greater than 0'
        );
        assert.isTrue(idGraphContext.status.isActive, 'Check InitialIDGraph error: isActive should be true');
    }
    console.log(colors.green('assertInitialIdGraphCreated complete'));
}

export async function assertIdentityLinked(
    context: IntegrationTestContext,
    signers: KeyringPair | KeyringPair[],
    events: any[],
    expectedIdentities: LitentryPrimitivesIdentity[]
) {
    // We should parse idGraph from the last event, because the last event updates the verification status of all identities.
    const eventIdGraph = parseIdGraph(context.sidechainRegistry, events[events.length - 1].data.idGraph, aesKey);

    for (let index = 0; index < events.length; index++) {
        const signer = Array.isArray(signers) ? signers[index] : signers;
        const expectedIdentity = expectedIdentities[index];
        const expectedIdentityTarget = expectedIdentity[`as${expectedIdentity.type}`];

        const eventData = events[index].data;
        const who = eventData.account.toHex();

        // Check prime identity in idGraph
        const expectedPrimeIdentity = await buildIdentityHelper(u8aToHex(signer.addressRaw), 'Substrate', context);
        assert.equal(
            expectedPrimeIdentity.toString(),
            eventIdGraph[events.length][0].toString(),
            'Check IdentityVerified error: eventIdGraph prime identity should be equal to expectedPrimeIdentity'
        );

        // Check event identity with expected identity
        const eventIdentity = parseIdentity(context.sidechainRegistry, eventData.identity, aesKey);

        const eventIdentityTarget = eventIdentity[`as${eventIdentity.type}`];

        assert.equal(
            expectedIdentityTarget.toString(),
            eventIdentityTarget.toString(),
            'Check IdentityVerified error: eventIdentityTarget should be equal to expectedIdentityTarget'
        );

        // Check identityContext in idGraph
        const idGraphContext = eventIdGraph[index][1];
        assert.isTrue(
            idGraphContext.linkBlock.toNumber() > 0,
            'Check InitialIDGraph error: link_block should be greater than 0'
        );
        assert.isTrue(idGraphContext.status.isActive, 'Check InitialIDGraph error: isActive should be true');

        assert.equal(who, u8aToHex(signer.addressRaw), 'Check IdentityCreated error: signer should be equal to who');
    }
    console.log(colors.green('assertIdentityVerified complete'));
}

export async function assertIdentityCreated(
    context: IntegrationTestContext,
    signers: KeyringPair | KeyringPair[],
    events: any[],
    aesKey: HexString,
    expectedIdentities: LitentryPrimitivesIdentity[]
) {
    for (let index = 0; index < events.length; index++) {
        const signer = Array.isArray(signers) ? signers[index] : signers;
        const expectedIdentity = expectedIdentities[index];
        const expectedIdentityTarget = expectedIdentity[`as${expectedIdentity.type}`];
        const eventData = events[index].data;
        const who = eventData.account.toHex();
        const eventIdentity = parseIdentity(context.sidechainRegistry, eventData.identity, aesKey);
        const eventIdentityTarget = eventIdentity[`as${eventIdentity.type}`];
        // Check identity caller
        assert.equal(who, u8aToHex(signer.addressRaw), 'Check IdentityCreated error: signer should be equal to who');

        // Check identity type
        assert.equal(
            expectedIdentity.type,
            eventIdentity.type,
            'Check IdentityCreated error: eventIdentity type should be equal to expectedIdentity type'
        );
        // Check identity in event
        assert.equal(
            expectedIdentityTarget.toString(),
            eventIdentityTarget.toString(),
            'Check IdentityCreated error: eventIdentityTarget should be equal to expectedIdentityTarget'
        );
    }
    console.log(colors.green('assertIdentityCreated complete'));
}

export async function assertIdentityDeactivated(
    context: IntegrationTestContext,
    signers: KeyringPair | KeyringPair[],
    events: any[]
) {
    for (let index = 0; index < events.length; index++) {
        const signer = Array.isArray(signers) ? signers[index] : signers;

        const eventData = events[index].data;
        const who = eventData.account.toHex();

        assert.equal(
            who,
            u8aToHex(signer.addressRaw),
            'Check IdentityDeactivated error: signer should be equal to who'
        );
    }

    console.log(colors.green('assertIdentityDeactivated complete'));
}

export async function assertIdentityActivated(
    context: IntegrationTestContext,
    signers: KeyringPair | KeyringPair[],
    events: any[]
) {
    for (let index = 0; index < events.length; index++) {
        const signer = Array.isArray(signers) ? signers[index] : signers;

        const eventData = events[index].data;
        const who = eventData.account.toHex();

        assert.equal(who, u8aToHex(signer.addressRaw), 'Check IdentityActivated error: signer should be equal to who');
    }

    console.log(colors.green('assertIdentityActivated complete'));
}

export async function assertIsInSidechainBlock(callType: string, res: WorkerRpcReturnValue) {
    assert.isTrue(
        res.status.isTrustedOperationStatus,
        `${callType} should be trusted operation status, but is ${res.status.type}`
    );
    const status = res.status.asTrustedOperationStatus;
    console.log(res.toHuman());

    assert.isTrue(
        status[0].isSubmitted || status[0].isInSidechainBlock,
        `${callType} should be submitted or in sidechain block, but is ${status[0].type}`
    );
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
    // Check vc index
    expect(index).to.be.eq(data.id);
    const signature = Buffer.from(hexToU8a(`0x${proofJson.proofValue}`));
    const message = Buffer.from(data.issuer.mrenclave);
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

    const jsonValid = await checkJson(vcObj, proof);
    expect(jsonValid).to.be.true;
    return true;
}

// Check VC json fields
export async function checkJson(vc: any, proofJson: any): Promise<boolean> {
    //check jsonSchema
    const ajv = new Ajv();
    const validate = ajv.compile(jsonSchema);
    const isValid = validate(vc);
    expect(isValid).to.be.true;
    expect(vc.type[0] === 'VerifiableCredential' && proofJson.type === 'Ed25519Signature2020').to.be.true;
    return true;
}




/* 
    assert vc
    steps:
    1. compare vc account with signer
    2. check vc status should be Active
    3. compare vc payload hash(blake vc payload) with vc hash
    4. compare vc index vc payload id
    5. check vc signature
    6. compare vc wtih jsonSchema
*/

export async function assertVc(context: IntegrationTestContext, signer: Signer, data: Bytes) {
    const vc = context.api.createType('RequestVCResponse', data) as unknown as RequestVCResponse;

    const vcHash = vc.vc_hash.toString()
    const signerAddress = u8aToHex(signer.getAddressRaw())

    // step 1
    const vcAccount = vc.account.toString()
    const decodedAccount = decodeAddress(vcAccount);
    assert.equal(u8aToHex(decodedAccount), signerAddress, 'Check VC error: signer should be equal to vc account');


    // step 2
    const vcIndex = vc.vc_index.toString()
    const vcRegistry = await context.api.query.vcManagement.vcRegistry(vcIndex) as unknown as PalletVcManagementVcContext
    const vcStatus = vcRegistry.toHuman()['status']
    assert.equal(vcStatus, 'Active', 'Check VcRegistry error:status should be equal to Active');

    // step 3
    // decryptWithAes function added 0x prefix
    const vcPayload = vc.vc_payload
    const decryptVcPayload = decryptWithAes(aesKey, vcPayload, 'utf-8').replace('0x', '');
    const vcPayloadHash = blake2AsHex(Buffer.from(decryptVcPayload));
    assert.equal(vcPayloadHash, vcHash, 'Check VcPayload error: vcPayloadHash should be equal to vcHash');

    // step 4
    const vcPayloadJson = JSON.parse(decryptVcPayload);
    const { proof, ...vcWithoutProof } = vcPayloadJson;
    assert.equal(vcIndex, vcPayloadJson.id, 'Check VcIndex error: VcIndex should be equal to vcPayload id');

    // step 5
    const enclaveCount = await context.api.query.teerex.enclaveCount();
    const enclaveRegistry = await context.api.query.teerex.enclaveRegistry(enclaveCount) as any;

    const signature = Buffer.from(hexToU8a(`0x${proof.proofValue}`));

    const message = Buffer.from(vcWithoutProof.issuer.mrenclave);

    const vcPubkey = Buffer.from(hexToU8a(`${enclaveRegistry.toHuman()['vcPubkey']}`));
    const signatureStatus = await ed.verify(signature, message, vcPubkey);

    assert.isTrue(signatureStatus, 'Check Vc signature error: signature should be valid');

    // step 6
    const ajv = new Ajv();

    const validate = ajv.compile(jsonSchema);

    const isValid = validate(vcPayloadJson);

    assert.isTrue(isValid, 'Check Vc payload error: vcPayload should be valid');
    assert.equal(vcWithoutProof.type[0], 'VerifiableCredential', 'Check Vc payload type error: vcPayload type should be VerifiableCredential');
    assert.equal(proof.type, 'Ed25519Signature2020', 'Check Vc proof type error: proof type should be Ed25519Signature2020');
}