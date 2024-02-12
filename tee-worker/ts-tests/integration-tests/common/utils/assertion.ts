import { ApiPromise } from '@polkadot/api';
import { Event } from '@polkadot/types/interfaces';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import Ajv from 'ajv';
import { assert, expect } from 'chai';
import * as ed from '@noble/ed25519';
import { parseIdGraph } from './identity-helper';
import type { PalletIdentityManagementTeeError } from 'sidechain-api';
import { PalletTeebagEnclave, CorePrimitivesIdentity } from 'parachain-api';
import type { IntegrationTestContext } from '../common-types';
import { getIdGraphHash } from '../di-utils';
import type { HexString } from '@polkadot/util/types';
import { jsonSchema } from './vc-helper';
import { aesKey } from '../call';
import colors from 'colors';
import {
    CorePrimitivesErrorErrorDetail,
    FrameSystemEventRecord,
    WorkerRpcReturnValue,
    RequestVCResult,
    StfError,
} from 'parachain-api';
import { Bytes } from '@polkadot/types-codec';
import { Signer, decryptWithAes } from './crypto';
import { blake2AsHex } from '@polkadot/util-crypto';
import { PalletIdentityManagementTeeIdentityContext } from 'sidechain-api';
import { KeyObject } from 'crypto';
import * as base58 from 'micro-base58';

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

export function assertIdGraph(
    actual: [CorePrimitivesIdentity, PalletIdentityManagementTeeIdentityContext][],
    expected: [CorePrimitivesIdentity, boolean][]
) {
    assert.equal(actual.length, expected.length);
    expected.forEach((expected, i) => {
        assert.deepEqual(
            actual[i][0].toJSON(),
            expected[0].toJSON(),
            'event idGraph identity should be equal expectedIdentity'
        );

        const idGraphContext = actual[0][1];
        assert.isTrue(idGraphContext.linkBlock.toNumber() > 0, 'link_block should be greater than 0');
        assert.equal(idGraphContext.status.isActive, expected[1], 'isActive should be ' + expected[1]);
    });
}

export async function assertIdentityDeactivated(context: IntegrationTestContext, signer: Signer, events: any[]) {
    for (let index = 0; index < events.length; index++) {
        const eventData = events[index].data;
        const who = eventData.primeIdentity;
        const signerIdentity = await signer.getIdentity(context);
        assert.deepEqual(
            who.toHuman(),
            signerIdentity.toHuman(),
            'Check IdentityDeactivated error: signer should be equal to who'
        );
    }

    console.log(colors.green('assertIdentityDeactivated complete'));
}

export async function assertIdentityActivated(context: IntegrationTestContext, signer: Signer, events: any[]) {
    for (let index = 0; index < events.length; index++) {
        const eventData = events[index].data;
        const who = eventData.primeIdentity;
        const signerIdentity = await signer.getIdentity(context);
        assert.deepEqual(
            who.toHuman(),
            signerIdentity.toHuman(),
            'Check IdentityActivated error: signer should be equal to who'
        );
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
    const enclaveIdentifier = api.createType('Vec<AccountId>', await api.query.teebag.enclaveIdentifier('Identity'));
    const primaryEnclave = (
        await api.query.teebag.enclaveRegistry(enclaveIdentifier[0])
    ).toHuman() as unknown as PalletTeebagEnclave;

    // Check vc index
    expect(index).to.be.eq(data.id);
    const signature = Buffer.from(hexToU8a(`0x${proofJson.proofValue}`));
    const message = Buffer.from(data.issuer.mrenclave);
    const vcPubkeyBytes = api.createType('Option<Bytes>', primaryEnclave.vcPubkey).unwrap();
    const vcPubkey = Buffer.from(hexToU8a(vcPubkeyBytes.toHex()));

    const isValid = await ed.verify(signature, message, vcPubkey);
    console.log('🚀 ~ verifySignature ~ isValid:', isValid);

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

// for IdGraph mutation, assert the corresponding event is emitted for the given signer and the id_graph_hash matches
export async function assertIdGraphMutationEvent(
    context: IntegrationTestContext,
    signer: Signer,
    events: any[],
    idGraphHashResults: any[] | undefined,
    expectedLength: number
) {
    assert.equal(events.length, expectedLength);
    if (idGraphHashResults != undefined) {
        assert.equal(idGraphHashResults!.length, expectedLength);
    }

    const signerIdentity = await signer.getIdentity(context);
    events.forEach((e, i) => {
        assert.deepEqual(signerIdentity.toHuman(), e.data.primeIdentity.toHuman());
        if (idGraphHashResults != undefined) {
            assert.equal(idGraphHashResults![i], e.data.idGraphHash.toHex());
        }
    });
    console.log(colors.green('assertIdGraphMutationEvent passed'));
}

export function assertWorkerError(
    context: IntegrationTestContext,
    check: (returnValue: StfError) => void,
    returnValue: WorkerRpcReturnValue
) {
    const errValueDecoded = context.api.createType('StfError', returnValue.value) as unknown as StfError;
    check(errValueDecoded);
}

// a common assertion for all DI requests that might mutate the IdGraph
// returns the `id_graph_hash` in the `returnValue`
export async function assertIdGraphMutationResult(
    context: IntegrationTestContext,
    teeShieldingKey: KeyObject,
    identity: CorePrimitivesIdentity,
    returnValue: WorkerRpcReturnValue,
    resultType:
        | 'LinkIdentityResult'
        | 'DeactivateIdentityResult'
        | 'ActivateIdentityResult'
        | 'SetIdentityNetworksResult',
    expectedIdGraph: [CorePrimitivesIdentity, boolean][]
): Promise<HexString> {
    const decodedResult = context.api.createType(resultType, returnValue.value) as any;
    assert.isNotNull(decodedResult.mutated_id_graph);
    const idGraph = parseIdGraph(context.sidechainRegistry, decodedResult.mutated_id_graph, aesKey);
    assertIdGraph(idGraph, expectedIdGraph);
    const queriedIdGraphHash = (await getIdGraphHash(context, teeShieldingKey, identity)).toHex();
    assert.equal(u8aToHex(decodedResult.id_graph_hash), queriedIdGraphHash);

    console.log(colors.green('assertIdGraphMutationResult passed'));
    return u8aToHex(decodedResult.id_graph_hash);
}

/* 
    assert vc
    steps:
    1. check vc status should be Active
    2. compare vc payload hash(blake vc payload) with vc hash
    3. check subject
    4. compare vc index with vcPayload id
    5. check vc signature
    6. compare vc wtih jsonSchema

    TODO: This is incomplete; we still need to further check: https://github.com/litentry/litentry-parachain/issues/1873
*/

export async function assertVc(context: IntegrationTestContext, subject: CorePrimitivesIdentity, data: Bytes) {
    const results = context.api.createType('RequestVCResult', data) as unknown as RequestVCResult;
    // step 1
    // decryptWithAes function added 0x prefix
    const vcPayload = results.vc_payload;
    const decryptVcPayload = decryptWithAes(aesKey, vcPayload, 'utf-8').replace('0x', '');

    /* DID format
    did:litentry:substrate:0x12345...
    did:litentry:evm:0x123456...
    did:litentry:twitter:my_twitter_handle
    */

    // step 2
    // check credential subject's DID
    const credentialSubjectId = JSON.parse(decryptVcPayload).credentialSubject.id;
    const expectSubject = Object.entries(JSON.parse(subject.toString()));

    // step 3
    // convert to DID format
    const expectDid = 'did:litentry:' + expectSubject[0][0] + ':' + expectSubject[0][1];
    assert.equal(
        expectDid,
        credentialSubjectId,
        'Check credentialSubject error: expectDid should be equal to credentialSubject id'
    );

    // step 4
    // extrac proof and vc without proof json
    const vcPayloadJson = JSON.parse(decryptVcPayload);
    console.log('credential: ', vcPayloadJson);
    console.log('assertions: ', vcPayloadJson.credentialSubject.assertions);
    const { proof, ...vcWithoutProof } = vcPayloadJson;

    // step 5
    // prepare teebag enclave registry data for further checks
    const parachainBlockHash = await context.api.query.system.blockHash(vcPayloadJson.parachainBlockNumber);
    const apiAtVcIssuedBlock = await context.api.at(parachainBlockHash);
    const enclaveIdentifier = await apiAtVcIssuedBlock.query.teebag.enclaveIdentifier('Identity');
    const lastRegisteredEnclave = (
        await apiAtVcIssuedBlock.query.teebag.enclaveRegistry(enclaveIdentifier[enclaveIdentifier.length - 1])
    ).unwrap();

    // step 6
    // check vc signature
    const signature = Buffer.from(hexToU8a(`0x${proof.proofValue}`));
    const message = Buffer.from(JSON.stringify(vcWithoutProof));

    const vcPubkeyBytes = context.api.createType('Option<Bytes>', lastRegisteredEnclave.vcPubkey).unwrap();
    const vcPubkey = Buffer.from(hexToU8a(vcPubkeyBytes.toHex()));

    const signatureStatus = await ed.verify(signature, message, vcPubkey);

    assert.isTrue(signatureStatus, 'Check Vc signature error: signature should be valid');

    // step 7
    // check VC mrenclave with enclave's mrenclave from registry
    assert.equal(
        base58.encode(lastRegisteredEnclave.mrenclave),
        vcPayloadJson.issuer.mrenclave,
        'Check VC mrenclave: it should equals enclaves mrenclave from parachains enclave registry'
    );

    // step 8
    // check vc issuer id
    assert.equal(
        `did:litentry:substrate:${vcPubkeyBytes.toHex()}`,
        vcPayloadJson.issuer.id,
        'Check VC id: it should equals enclaves pubkey from parachains enclave registry'
    );

    // step 9
    // validate VC aganist schema
    const ajv = new Ajv();

    const validate = ajv.compile(jsonSchema);

    const isValid = validate(vcPayloadJson);

    assert.isTrue(isValid, 'Check Vc payload error: vcPayload should be valid');
    assert.equal(
        vcWithoutProof.type[0],
        'VerifiableCredential',
        'Check Vc payload type error: vcPayload type should be VerifiableCredential'
    );
    assert.equal(
        proof.type,
        'Ed25519Signature2020',
        'Check Vc proof type error: proof type should be Ed25519Signature2020'
    );
}

export async function assertIdGraphHash(
    context: IntegrationTestContext,
    teeShieldingKey: KeyObject,
    identity: CorePrimitivesIdentity,
    idGraph: [CorePrimitivesIdentity, PalletIdentityManagementTeeIdentityContext][]
) {
    const idGraphType = context.sidechainRegistry.createType(
        'Vec<(CorePrimitivesIdentity, PalletIdentityManagementTeeIdentityContext)>',
        idGraph
    );
    const computedIdGraphHash = blake2AsHex(idGraphType.toU8a());
    console.log('computed id graph hash: ', computedIdGraphHash);

    const queriedIdGraphHash = (await getIdGraphHash(context, teeShieldingKey, identity)).toHex();
    console.log('queried id graph hash: ', queriedIdGraphHash);
    assert.equal(computedIdGraphHash, queriedIdGraphHash);
}
