import './config';
import WebSocketAsPromised from 'websocket-as-promised';
import WebSocket from 'ws';
import Options from 'websocket-as-promised/types/options';
import { ApiPromise, WsProvider } from '@polkadot/api';
import {
    AESOutput,
    EnclaveResult,
    IntegrationTestContext,
    LitentryIdentity,
    teeTypes,
    JsonSchema,
    IdentityContext,
} from './type-definitions';
import { blake2AsHex, cryptoWaitReady, xxhashAsU8a } from '@polkadot/util-crypto';
import { Metadata } from '@polkadot/types';
import { SiLookupTypeId, EventRecord } from '@polkadot/types/interfaces';
import { KeyringPair } from '@polkadot/keyring/types';
import { Codec } from '@polkadot/types/types';
import { HexString } from '@polkadot/util/types';
import { hexToU8a, u8aToHex, u8aConcat } from '@polkadot/util';
import { KeyObject } from 'crypto';
import { Event, StorageEntryMetadataV14, StorageHasherV14 } from '@polkadot/types/interfaces';
import { after, before, describe } from 'mocha';
import { ethers } from 'ethers';
import { assert, expect } from 'chai';
import Ajv from 'ajv';
import * as ed from '@noble/ed25519';
import { blake2128Concat, getSubstrateSigner, identity, twox64Concat } from './helpers';
import { getMetadata, sendRequest } from './call';
const base58 = require('micro-base58');
const crypto = require('crypto');
import { getEthereumSigner } from '../common/helpers';

// in order to handle self-signed certificates we need to turn off the validation
// TODO add self signed certificate ??
process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

// maximum milliseconds that we wait in listening events before we timeout
const listenTimeoutInMilliSeconds = 3 * 60 * 1000;

export function sleep(secs: number) {
    return new Promise((resolve) => {
        setTimeout(resolve, secs * 1000);
    });
}

export async function getListenTimeoutInBlocks(api: ApiPromise) {
    const slotDuration = await api.call.auraApi.slotDuration();
    return listenTimeoutInMilliSeconds / parseInt(slotDuration.toString());
}

export async function initWorkerConnection(endpoint: string): Promise<WebSocketAsPromised> {
    const wsp = new WebSocketAsPromised(endpoint, <Options>(<unknown>{
        createWebSocket: (url: any) => new WebSocket(url),
        extractMessageData: (event: any) => event,
        packMessage: (data: any) => JSON.stringify(data),
        unpackMessage: (data: string | ArrayBuffer | Blob) => JSON.parse(data.toString()),
        attachRequestId: (data: any, requestId: string | number) => Object.assign({ id: requestId }, data),
        extractRequestId: (data: any) => data && data.id, // read requestId from message `id` field
    }));
    await wsp.open();
    return wsp;
}

export async function initIntegrationTestContext(
    workerEndpoint: string,
    substrateEndpoint: string
): Promise<IntegrationTestContext> {
    const provider = new WsProvider(substrateEndpoint);
    const ethersWallet = {
        alice: new ethers.Wallet(getEthereumSigner().alice),
        bob: new ethers.Wallet(getEthereumSigner().bob),
        charlie: new ethers.Wallet(getEthereumSigner().charlie),
        dave: new ethers.Wallet(getEthereumSigner().dave),
        eve: new ethers.Wallet(getEthereumSigner().eve),
    };
    const substrateWallet = {
        alice: getSubstrateSigner().alice,
        bob: getSubstrateSigner().bob,
        charlie: getSubstrateSigner().charlie,
        eve: getSubstrateSigner().eve,
    };
    const api = await ApiPromise.create({
        provider,
        types: teeTypes,
    });

    await cryptoWaitReady();

    const wsp = await initWorkerConnection(workerEndpoint);

    const metaData = await getMetadata(wsp, api);

    const { mrEnclave, teeShieldingKey } = await getEnclave(api);
    return <IntegrationTestContext>{
        tee: wsp,
        api,
        teeShieldingKey,
        mrEnclave,
        ethersWallet,
        substrateWallet,
        metaData,
    };
}

export function decryptWithAES(key: HexString, aesOutput: AESOutput, type: string): HexString {
    if (aesOutput.ciphertext && aesOutput.nonce) {
        const secretKey = crypto.createSecretKey(hexToU8a(key));
        const tagSize = 16;
        const ciphertext = aesOutput.ciphertext ? aesOutput.ciphertext : hexToU8a('0x');
        console.log('ciphertext: ', u8aToHex(ciphertext));

        const initialization_vector = aesOutput.nonce ? aesOutput.nonce : hexToU8a('0x');
        const aad = aesOutput.aad ? aesOutput.aad : hexToU8a('0x');

        // notice!!! extract author_tag from ciphertext
        // maybe this code only works with rust aes encryption

        const authorTag = ciphertext.subarray(ciphertext.length - tagSize);

        const decipher = crypto.createDecipheriv('aes-256-gcm', secretKey, initialization_vector);
        decipher.setAAD(aad);
        decipher.setAuthTag(authorTag);

        let part1 = decipher.update(ciphertext.subarray(0, ciphertext.length - tagSize), undefined, type);

        let part2 = decipher.final(type);

        return `0x${part1 + part2}`;
    } else {
        return u8aToHex(aesOutput as Uint8Array);
    }
}

export async function createTrustedCallSigned(
    api: ApiPromise,
    trustedCall: [string, string],
    account: KeyringPair,
    mrenclave: string,
    mrEnclave: string,
    nonce: Codec,
    params: Array<any>
) {
    const [variant, argType] = trustedCall;
    const call = api.createType('TrustedCall', {
        [variant]: api.createType(argType, params),
    });
    const payload = Uint8Array.from([
        ...call.toU8a(),
        ...nonce.toU8a(),
        ...base58.decode(mrenclave),
        ...hexToU8a(mrEnclave),
    ]);
    const signature = api.createType('MultiSignature', {
        Sr25519: u8aToHex(account.sign(payload)),
    });
    return api.createType('TrustedCallSigned', {
        call: call,
        index: nonce,
        signature: signature,
    });
}

export function encryptWithTeeShieldingKey(teeShieldingKey: KeyObject, plaintext: HexString): Buffer {
    return crypto.publicEncrypt(
        {
            key: teeShieldingKey,
            padding: crypto.constants.RSA_PKCS1_OAEP_PADDING,
            oaepHash: 'sha256',
        },
        hexToU8a(plaintext)
    );
}

//<challeng-code> + <litentry-AccountId32> + <Identity>
export function generateVerificationMessage(
    context: IntegrationTestContext,
    challengeCode: Uint8Array,
    signerAddress: Uint8Array,
    identity: LitentryIdentity
): HexString {
    const encode = context.api.createType('LitentryIdentity', identity).toU8a();
    const msg = Buffer.concat([challengeCode, signerAddress, encode]);
    return blake2AsHex(msg, 256);
}

export function describeLitentry(title: string, cb: (context: IntegrationTestContext) => void) {
    describe(title, function () {
        // Set timeout to 6000 seconds
        this.timeout(6000000);
        let context: IntegrationTestContext = {
            mrEnclave: '0x11' as HexString,
            api: {} as ApiPromise,
            tee: {} as WebSocketAsPromised,
            teeShieldingKey: {} as KeyObject,
            ethersWallet: {},
            substrateWallet: {},
            metaData: {} as Metadata,
        };

        before('Starting Litentry(parachain&tee)', async function () {
            //env url
            const tmp = await initIntegrationTestContext(
                process.env.WORKER_END_POINT!,
                process.env.SUBSTRATE_END_POINT!
            );
            context.mrEnclave = tmp.mrEnclave;
            context.api = tmp.api;
            context.tee = tmp.tee;
            context.teeShieldingKey = tmp.teeShieldingKey;
            context.ethersWallet = tmp.ethersWallet;
            context.substrateWallet = tmp.substrateWallet;
            context.metaData = tmp.metaData;
        });

        after(async function () { });

        cb(context);
    });
}

export async function getEnclave(api: ApiPromise): Promise<{
    mrEnclave: string;
    teeShieldingKey: KeyObject;
}> {
    const count = await api.query.teerex.enclaveCount();

    const res = (await api.query.teerex.enclaveRegistry(count)).toHuman() as EnclaveResult;

    const teeShieldingKey = crypto.createPublicKey({
        key: {
            alg: 'RSA-OAEP-256',
            kty: 'RSA',
            use: 'enc',
            n: Buffer.from(JSON.parse(res.shieldingKey).n.reverse()).toString('base64url'),
            e: Buffer.from(JSON.parse(res.shieldingKey).e.reverse()).toString('base64url'),
        },
        format: 'jwk',
    });
    //@TODO mrEnclave should verify from storage
    const mrEnclave = res.mrEnclave;
    return {
        mrEnclave,
        teeShieldingKey,
    };
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

export async function checkFailReason(
    response: string[] | Event[],
    expectedReason: string,
    isModule: boolean
): Promise<boolean> {
    let failReason = '';
    response.map((item: any) => {
        isModule ? (failReason = item.toHuman().data.reason) : (failReason = item);
        assert.notEqual(
            failReason.search(expectedReason),
            -1,
            `check fail reason failed, expected reason is ${expectedReason}, but got ${failReason}`
        );
    });
    return true;
}

//sidechain storage utils
export function buildStorageEntry(metadata: Metadata, prefix: string, method: string): StorageEntryMetadataV14 | null {
    for (const pallet of metadata.asV14.pallets) {
        if (pallet.name.toString() == prefix) {
            const storage = pallet.storage.unwrap();

            for (const item of storage.items) {
                if (item.name.toString() == method) {
                    return item;
                }
            }
        }
    }
    return null;
}

export function buildStorageKey(
    metadata: Metadata,
    prefix: string,
    method: string,
    keyTypeId?: SiLookupTypeId,
    hashers?: Array<StorageHasherV14>,
    input?: Array<unknown>
): Uint8Array {
    let storageKey = u8aConcat(xxhashAsU8a(prefix, 128), xxhashAsU8a(method, 128));
    if (keyTypeId && hashers && input) {
        let keyTypeIds = hashers.length === 1 ? [keyTypeId] : metadata.registry.lookup.getSiType(keyTypeId).def.asTuple;
        for (let i = 0; i < keyTypeIds.length; i++) {
            const theKeyTypeId = keyTypeIds[i];
            const theHasher = hashers[i].toString();
            const theKeyItem = input[i];
            // get the scale encoded input data by encoding the input
            const theKeyType = metadata.registry.createLookupType(theKeyTypeId);
            const theKeyItemEncoded = metadata.registry.createType(theKeyType, theKeyItem).toU8a();
            // apply hasher
            let theKeyItemAppliedHasher;
            if (theHasher == 'Blake2_128Concat') {
                theKeyItemAppliedHasher = blake2128Concat(theKeyItemEncoded);
            } else if (theHasher == 'Twox64Concat') {
                theKeyItemAppliedHasher = twox64Concat(theKeyItemEncoded);
            } else if (theHasher == 'Identity') {
                theKeyItemAppliedHasher = identity(theKeyItemEncoded);
            } else {
                throw new Error(`The hasher ${theHasher} is not support.`);
            }
            storageKey = u8aConcat(storageKey, theKeyItemAppliedHasher);
        }
    }
    return storageKey;
}
export async function buildStorageData(
    metadata: Metadata,
    prefix: string,
    method: string,
    ...input: Array<unknown>
): Promise<string | null> {
    const storageEntry = buildStorageEntry(metadata, prefix, method);
    if (!storageEntry) {
        throw new Error('Can not find the storage entry from metadata');
    }
    let storageKey, valueType;

    if (storageEntry.type.isPlain) {
        storageKey = buildStorageKey(metadata, prefix, method);
        valueType = metadata.registry.createLookupType(storageEntry.type.asPlain);
    } else if (storageEntry.type.isMap) {
        const { hashers, key, value } = storageEntry.type.asMap;
        if (input.length != hashers.length) {
            throw new Error('The `input` param is not correct');
        }
        storageKey = buildStorageKey(metadata, prefix, method, key, hashers, input);
        valueType = metadata.registry.createLookupType(value);
    } else {
        throw new Error('Only support plain and map type');
    }
    console.debug(`storage key: ${u8aToHex(storageKey)}`);
    return u8aToHex(storageKey);
}

export async function checkUserShieldingKeys(
    context: IntegrationTestContext,
    pallet: string,
    method: string,
    address: HexString
): Promise<string> {
    const storageKey = await buildStorageData(context.metaData, pallet, method, address);

    let base58mrEnclave = base58.encode(Buffer.from(context.mrEnclave.slice(2), 'hex'));

    let request = {
        jsonrpc: '2.0',
        method: 'state_getStorage',
        params: [base58mrEnclave, storageKey],
        id: 1,
    };
    let resp = await sendRequest(context.tee, request, context.api);

    return resp.value;
}
export async function checkUserChallengeCode(
    context: IntegrationTestContext,
    pallet: string,
    method: string,
    address: HexString,
    identity: HexString
): Promise<string> {
    const storageKey = await buildStorageData(context.metaData, pallet, method, address, identity);

    let base58mrEnclave = base58.encode(Buffer.from(context.mrEnclave.slice(2), 'hex'));

    let request = {
        jsonrpc: '2.0',
        method: 'state_getStorage',
        params: [base58mrEnclave, storageKey],
        id: 1,
    };
    let resp = await sendRequest(context.tee, request, context.api);
    return resp.value;
}

export async function checkIDGraph(
    context: IntegrationTestContext,
    pallet: string,
    method: string,
    address: HexString,
    identity: HexString
): Promise<IdentityContext> {
    const storageKey = await buildStorageData(context.metaData, pallet, method, address, identity);

    let base58mrEnclave = base58.encode(Buffer.from(context.mrEnclave.slice(2), 'hex'));

    let request = {
        jsonrpc: '2.0',
        method: 'state_getStorage',
        params: [base58mrEnclave, storageKey],
        id: 1,
    };
    let resp = await sendRequest(context.tee, request, context.api);
    const IDGraph = context.api.createType('IdentityContext', resp.value).toJSON() as IdentityContext;
    return IDGraph;
}

//batch call utils
export async function handleEvent(events: EventRecord[]): Promise<any> {
    console.log(events);

}
