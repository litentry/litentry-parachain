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
    WorkerRpcReturnValue,
    TransactionSubmit,
    JsonSchema,
    WorkerRpcReturnString,
} from './type-definitions';
import { blake2AsHex, cryptoWaitReady, xxhashAsU8a, blake2AsU8a } from '@polkadot/util-crypto';
import { ApiTypes, SubmittableExtrinsic, } from '@polkadot/api/types';
import { Metadata, StorageKey, TypeRegistry, } from '@polkadot/types';
import { SiLookupTypeId } from "@polkadot/types/interfaces";

import { KeyringPair } from '@polkadot/keyring/types';
import { Codec } from '@polkadot/types/types';
import { HexString } from '@polkadot/util/types';
const { HttpProvider } = require('@polkadot/rpc-provider')
import { hexToU8a, u8aToHex, stringToU8a, u8aConcat, u8aToU8a } from '@polkadot/util';
import { KeyObject } from 'crypto';
import { Event, EventRecord, StorageEntryMetadataV14, StorageHasherV14 } from '@polkadot/types/interfaces';
import { after, before, describe } from 'mocha';
import { generateChallengeCode, getSigner } from './web3/setup';
import { ethers } from 'ethers';
import { generateTestKeys } from './web3/functions';
import { assert, expect } from 'chai';
import { Base64 } from 'js-base64';
import Ajv from 'ajv';
import * as ed from '@noble/ed25519';
import { BaseProvider } from '@ethersproject/providers';
const base58 = require('micro-base58');
const crypto = require('crypto');
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

export async function sendRequest(
    wsClient: WebSocketAsPromised,
    request: any,
    api: ApiPromise
): Promise<WorkerRpcReturnValue> {

    const resp = await wsClient.sendRequest(request, { requestId: 1, timeout: 6000 });

    const resp_json = api.createType('WorkerRpcReturnValue', resp.result).toJSON() as WorkerRpcReturnValue;

    return resp_json;
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

export function getStorageEntry(metadata: Metadata, prefix: string, method: string): StorageEntryMetadataV14 | null {
    for (const pallet of metadata.asV14.pallets) {
        console.log("pallet", pallet);

        if (pallet.name.toString() == prefix) {
            const storage = pallet.storage.unwrap();

            for (const item of storage.items) {

                if (item.name.toString() == method) {
                    console.log(3333, item);

                    return item;
                }
            }
        }
    }
    return null;
}

export type GetStorage = (prefix: string, method: string, ...input: Array<unknown>) => Promise<string | null>;
export function buildStorageKey(metadata: Metadata, prefix: string, method: string, keyTypeId?: SiLookupTypeId, hashers?: Array<StorageHasherV14>, input?: Array<unknown>): Uint8Array {
    let storageKey = u8aConcat(
        xxhashAsU8a(prefix, 128), xxhashAsU8a(method, 128)
    );
    if (keyTypeId && hashers && input) {
        let keyTypeIds = hashers.length === 1
            ? [keyTypeId]
            : metadata.registry.lookup.getSiType(keyTypeId).def.asTuple;

        for (let i = 0; i < keyTypeIds.length; i++) {
            const theKeyTypeId = keyTypeIds[i];
            const theHasher = hashers[i].toString();
            const theKeyItem = input[i];

            // get the scale encoded input data by encoding the input
            const theKeyType = metadata.registry.createLookupType(theKeyTypeId);
            const theKeyItemEncoded = metadata.registry.createType(theKeyType, theKeyItem).toU8a();

            // apply hasher
            let theKeyItemAppliedHasher;
            if (theHasher == "Blake2_128Concat") {
                theKeyItemAppliedHasher = blake2128Concat(theKeyItemEncoded);
            } else if (theHasher == "Twox64Concat") {
                theKeyItemAppliedHasher = twox64Concat(theKeyItemEncoded);
            } else if (theHasher == "Identity") {
                theKeyItemAppliedHasher = identity(theKeyItemEncoded);
            } else {
                throw new Error(`The hasher ${theHasher} is not support. Contact Aki for help`);
            }
            storageKey = u8aConcat(storageKey, theKeyItemAppliedHasher);
        }
    }
    return storageKey;
}
export function getStorage(metadata: Metadata) {
    return async (prefix: string, method: string, ...input: Array<unknown>): Promise<string | null> => {
        // 0. FIND STORAGE ENTRY FROM METADATA
        const storageEntry = getStorageEntry(metadata, prefix, method);

        if (!
            storageEntry
        ) {
            throw new Error("Can not find the storage entry from metadata");
        }

        // 1. GET STORAGE KEY & THE RESULT TYPE
        let storageKey, valueType;

        if (storageEntry.type.isPlain) {

            storageKey = buildStorageKey(metadata, prefix, method);
            console.log("storageKey1", storageKey);

            valueType = metadata.registry.createLookupType(storageEntry.type.asPlain);
            console.log(
                "valueType1", valueType
            );
        } else if (storageEntry.type.isMap) {
            const { hashers, key, value } = storageEntry.type.asMap;

            if (input.length != hashers.length) {
                throw new Error("The `input` param is not correct");
            }
            storageKey = buildStorageKey(metadata, prefix, method, key, hashers, input);
            valueType = metadata.registry.createLookupType(value);
            console.log(
                "valueType2", valueType
            );

        } else {
            throw new Error("Only support plain and map type");
        }

        console.debug(`storage key: ${u8aToHex(storageKey)}`);

        // // 2. GET RAW STORAGE DATA BY STORAGE KEY
        // let raw = await getStorageRaw(provider, storageKey);
        // console.debug(`storage raw: ${raw}`);
        // if (raw.toString() == "0x" && storageEntry.modifier.isDefault) {
        let raw = storageEntry.fallback

        // }

        // 3. DECODE THE RAW STORAGE DATA BY THE RESULT TYPE
        // if (raw.toString() == "0x") {
        //     return null;
        // } else {

        return metadata.registry.createType(valueType, raw).toString();
        // }
    }
}

export async function getMetadata(wsClient: WebSocketAsPromised, api: ApiPromise): Promise<any> {
    let request = { jsonrpc: '2.0', method: 'state_getMetadata', params: [], id: 1 };
    let respJSON = await sendRequest(wsClient, request, api) as any;
    const metadataPrc = await api.rpc.state.getMetadata();
    console.log(888, metadataPrc);

    console.log("respJSON", respJSON);

    // const provider = new HttpProvider('http://localhost:9944')
    // const chain_metadata = await provider.send('state_getMetadata', [])
    // console.debug("chain_metadata", chain_metadata);

    const registry = new TypeRegistry()
    // const pubKeyHex = api.createType('WorkerRpcReturnString', respJSON.value).toJSON() as any;

    const metadata = new Metadata(registry, respJSON.value)
    registry.setMetadata(metadataPrc)

    // return
    let prefix: any = 'System'
    let method: any = 'Account'
    let alice = '2P2pRoXYwZAWVPXXtR6is5o7L34Me72iuNdiMZxeNV2BkgsH'



    const getPangolinStorage = getStorage(metadataPrc);
    let result = await getPangolinStorage(
        "Timestamp", // start with a upcase char
        "Now", // start with a upcase char
    );
    console.log(111, result);

    // const storageKey = new StorageKey()

    // let chunk = Buffer.from(pubKeyHex.vec.slice(2), 'hex');
    // let pubKeyJSON = JSON.parse(chunk.toString('utf-8')) as any
    // console.log(33344, pubKeyJSON);

}

export async function initIntegrationTestContext(
    workerEndpoint: string,
    substrateEndpoint: string
): Promise<IntegrationTestContext> {
    const provider = new WsProvider(substrateEndpoint);
    const ethersWallet = {
        alice: new ethers.Wallet(generateTestKeys().alice),
        bob: new ethers.Wallet(generateTestKeys().bob),
        charlie: new ethers.Wallet(generateTestKeys().charlie),
        dave: new ethers.Wallet(generateTestKeys().dave),
        eve: new ethers.Wallet(generateTestKeys().eve),
    };
    const api = await ApiPromise.create({
        provider,
        types: teeTypes,
    });

    await cryptoWaitReady();

    const wsp = await initWorkerConnection(workerEndpoint);


    await getMetadata(wsp, api);
    const { mrEnclave, teeShieldingKey } = await getEnclave(api);
    return <IntegrationTestContext>{
        tee: wsp,
        api,
        teeShieldingKey,
        mrEnclave,
        defaultSigner: getSigner(),
        ethersWallet,
    };
}

export async function sendTxUntilInBlock(api: ApiPromise, tx: SubmittableExtrinsic<ApiTypes>, signer: KeyringPair) {
    return new Promise<{ block: string }>(async (resolve, reject) => {
        // The purpose of paymentInfo is to check whether the version of polkadot/api is suitable for the current test and to determine whether the transaction is successful.
        await tx.paymentInfo(signer);
        const nonce = await api.rpc.system.accountNextIndex(signer.address);
        await tx.signAndSend(signer, { nonce }, (result) => {
            if (result.status.isInBlock) {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                resolve({
                    block: result.status.asInBlock.toString(),
                });
            } else if (result.status.isInvalid) {
                reject(`Transaction is ${result.status}`);
            }
        });
    });
}

export async function sendTxUntilInBlockList(api: ApiPromise, txs: TransactionSubmit[], signer: KeyringPair) {
    return Promise.all(
        txs.map(async ({ tx, nonce }) => {
            const result = await new Promise((resolve, reject) => {
                tx.signAndSend(signer, { nonce }, (result) => {
                    if (result.status.isInBlock) {
                        //catch error
                        if (result.dispatchError) {
                            if (result.dispatchError.isModule) {
                                const decoded = api.registry.findMetaError(result.dispatchError.asModule);
                                const { docs, name, section } = decoded;

                                console.log(`${section}.${name}: ${docs.join(' ')}`);
                                resolve(`${section}.${name}`);
                            } else {
                                console.log(result.dispatchError.toString());
                                resolve(result.dispatchError.toString());
                            }
                        } else {
                            console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                            resolve({
                                block: result.status.asInBlock.toString(),
                            });
                        }
                    } else if (result.status.isInvalid) {
                        reject(`Transaction is ${result.status}`);
                    }
                });
            });
            return result;
        })
    );
}

// Subscribe to the chain until we get the first specified event with given `section` and `methods`.
// We can listen to multiple `methods` as long as they are emitted in the same block.
// The event consumer should do the decryption optionaly as it's event specific
export async function listenEvent(api: ApiPromise, section: string, methods: string[]) {
    return new Promise<Event[]>(async (resolve, reject) => {
        let startBlock = 0;
        const unsubscribe = await api.rpc.chain.subscribeNewHeads(async (header) => {
            const currentBlockNumber = header.number.toNumber();
            if (startBlock == 0) startBlock = currentBlockNumber;
            const timeout = await getListenTimeoutInBlocks(api);
            if (currentBlockNumber > startBlock + timeout) {
                reject('timeout');
                return;
            }
            console.log(`\n--------- block #${header.number}, hash ${header.hash} ---------\n`);
            const apiAt = await api.at(header.hash);

            const records: EventRecord[] = (await apiAt.query.system.events()) as any;
            records.forEach((e, i) => {
                const s = e.event.section;
                const m = e.event.method;
                const d = e.event.data;
                console.log(`Event[${i}]: ${s}.${m} ${d}`);
            });
            const events = records.filter(
                ({ phase, event }) =>
                    phase.isApplyExtrinsic && section === event.section && methods.includes(event.method)
            );

            if (events.length) {
                resolve(events.map((e) => e.event));
                unsubscribe();
                return;
            }
        });
    });
}

export function decryptWithAES(key: HexString, aesOutput: AESOutput, type: string): HexString {
    if (aesOutput.ciphertext && aesOutput.nonce) {
        const secretKey = crypto.createSecretKey(hexToU8a(key));
        const tagSize = 16;
        const ciphertext = aesOutput.ciphertext ? aesOutput.ciphertext : hexToU8a('0x');
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
            defaultSigner: [] as KeyringPair[],
            mrEnclave: '0x11' as HexString,
            api: {} as ApiPromise,
            tee: {} as WebSocketAsPromised,
            teeShieldingKey: {} as KeyObject,
            ethersWallet: {},
        };

        before('Starting Litentry(parachain&tee)', async function () {
            //env url
            const tmp = await initIntegrationTestContext(
                process.env.WORKER_END_POINT!,
                process.env.SUBSTRATE_END_POINT!
            );

            context.defaultSigner = tmp.defaultSigner;
            context.mrEnclave = tmp.mrEnclave;
            context.api = tmp.api;
            context.tee = tmp.tee;
            context.teeShieldingKey = tmp.teeShieldingKey;
            context.ethersWallet = tmp.ethersWallet;
        });

        after(async function () { });

        cb(context);
    });
}

export function getMessage(address: string, wallet: string): string {
    const challengeCode = generateChallengeCode();
    const messgae = `Signing in ${process.env.ID_HUB_URL} with ${address} using ${wallet} and challenge code is: ${challengeCode}`;
    return messgae;
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



export function blake2128Concat(data: HexString | Uint8Array): Uint8Array {
    return u8aConcat(blake2AsU8a(data, 128), u8aToU8a(data));
}

export function twox64Concat(data: HexString | Uint8Array): Uint8Array {
    return u8aConcat(
        xxhashAsU8a(data, 64), u8aToU8a(data)
    );
}

export function identity(data: HexString | Uint8Array): Uint8Array {
    return u8aToU8a(data);
}