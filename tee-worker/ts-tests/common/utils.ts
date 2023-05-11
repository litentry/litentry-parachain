import './config';
import WebSocketAsPromised from 'websocket-as-promised';
import WebSocket from 'ws';
import Options from 'websocket-as-promised/types/options';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import {
    AESOutput,
    EnclaveResult,
    IntegrationTestContext,
    LitentryIdentity,
    teeTypes,
    JsonSchema,
    IdentityContext,
    Web3Wallets,
    IdentityGenericEvent,
    LitentryValidationData,
    SubstrateNetwork,
    EvmNetwork,
    Web2Network,
} from './type-definitions';

import { blake2AsHex, cryptoWaitReady, xxhashAsU8a } from '@polkadot/util-crypto';
import { Metadata } from '@polkadot/types';
import { SiLookupTypeId } from '@polkadot/types/interfaces';
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
    substrateEndpoint: string,
    walletsNumber: number
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

    const { types } = teeTypes;
    const api = await ApiPromise.create({
        provider,
        types,
    });

    await cryptoWaitReady();

    const wsp = await initWorkerConnection(workerEndpoint);

    const metaData = await getMetadata(wsp, api);

    const web3Signers = await generateWeb3Wallets(walletsNumber);
    const { mrEnclave, teeShieldingKey } = await getEnclave(api);
    return <IntegrationTestContext>{
        tee: wsp,
        api,
        teeShieldingKey,
        mrEnclave,
        ethersWallet,
        substrateWallet,
        metaData,
        web3Signers,
    };
}

export function decryptWithAES(key: HexString, aesOutput: AESOutput, type: string): HexString {
    if (aesOutput.ciphertext && aesOutput.nonce) {
        const secretKey = crypto.createSecretKey(hexToU8a(key));
        const tagSize = 16;
        const ciphertext = aesOutput.ciphertext ? aesOutput.ciphertext : hexToU8a('0x');
        console.log('ciphertext: ', u8aToHex(ciphertext));

        const nonce = aesOutput.nonce ? aesOutput.nonce : hexToU8a('0x');
        const aad = aesOutput.aad ? aesOutput.aad : hexToU8a('0x');

        // notice!!! extract author_tag from ciphertext
        // maybe this code only works with rust aes encryption
        const authorTag = ciphertext.subarray(ciphertext.length - tagSize);

        const decipher = crypto.createDecipheriv('aes-256-gcm', secretKey, nonce);
        decipher.setAAD(aad);
        decipher.setAuthTag(authorTag);

        let part1 = decipher.update(ciphertext.subarray(0, ciphertext.length - tagSize), undefined, type);

        let part2 = decipher.final(type);

        return `0x${part1 + part2}`;
    } else {
        return u8aToHex(aesOutput as Uint8Array);
    }
}

export function encryptWithTeeShieldingKey(teeShieldingKey: KeyObject, plaintext: Uint8Array): Buffer {
    return crypto.publicEncrypt(
        {
            key: teeShieldingKey,
            padding: crypto.constants.RSA_PKCS1_OAEP_PADDING,
            oaepHash: 'sha256',
        },
        plaintext
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

export function describeLitentry(title: string, walletsNumber: number, cb: (context: IntegrationTestContext) => void) {
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
            web3Signers: [] as Web3Wallets[],
        };

        before('Starting Litentry(parachain&tee)', async function () {
            //env url
            const tmp = await initIntegrationTestContext(
                process.env.WORKER_END_POINT!,
                process.env.SUBSTRATE_END_POINT!,
                walletsNumber
            );
            context.mrEnclave = tmp.mrEnclave;
            context.api = tmp.api;
            context.tee = tmp.tee;
            context.teeShieldingKey = tmp.teeShieldingKey;
            context.ethersWallet = tmp.ethersWallet;
            context.substrateWallet = tmp.substrateWallet;
            context.metaData = tmp.metaData;
            context.web3Signers = tmp.web3Signers;
        });

        after(async function () {});

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
export async function buildStorageHelper(
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
    const storageKey = await buildStorageHelper(context.metaData, pallet, method, address);

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
    const storageKey = await buildStorageHelper(context.metaData, pallet, method, address, identity);

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
    const storageKey = await buildStorageHelper(context.metaData, pallet, method, address, identity);

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

export async function generateWeb3Wallets(count: number): Promise<Web3Wallets[]> {
    const seed = 'litentry seed';
    const addresses: Web3Wallets[] = [];
    const keyring = new Keyring({ type: 'sr25519' });

    for (let i = 0; i < count; i++) {
        const substratePair = keyring.addFromUri(`${seed}//${i}`);
        const ethereumWallet = ethers.Wallet.createRandom();
        addresses.push({
            substrateWallet: substratePair,
            ethereumWallet: ethereumWallet,
        });
    }
    return addresses;
}
export function createIdentityEvent(
    api: ApiPromise,
    who: HexString,
    identityString?: HexString,
    idGraphString?: HexString,
    challengeCode?: HexString
): IdentityGenericEvent {
    let identity = identityString ? api.createType('LitentryIdentity', identityString).toJSON() : undefined;
    let idGraph = idGraphString
        ? api.createType('Vec<(LitentryIdentity, IdentityContext)>', idGraphString).toJSON()
        : undefined;
    return <IdentityGenericEvent>{
        who,
        identity,
        idGraph,
        challengeCode,
    };
}

export async function handleIdentityEvents(
    context: IntegrationTestContext,
    aesKey: HexString,
    events: any[],
    type: 'UserShieldingKeySet' | 'IdentityCreated' | 'IdentityVerified' | 'IdentityRemoved' | 'Failed'
): Promise<any[]> {
    let results: IdentityGenericEvent[] = [];

    for (let index = 0; index < events.length; index++) {
        switch (type) {
            case 'UserShieldingKeySet':
                results.push(
                    createIdentityEvent(
                        context.api,
                        events[index].data.account.toHex(),
                        undefined,
                        decryptWithAES(aesKey, events[index].data.idGraph, 'hex')
                    )
                );
                break;
            case 'IdentityCreated':
                results.push(
                    createIdentityEvent(
                        context.api,
                        events[index].data.account.toHex(),
                        decryptWithAES(aesKey, events[index].data.identity, 'hex'),
                        undefined,
                        decryptWithAES(aesKey, events[index].data.code, 'hex')
                    )
                );
                break;
            case 'IdentityVerified':
                results.push(
                    createIdentityEvent(
                        context.api,
                        events[index].data.account.toHex(),
                        decryptWithAES(aesKey, events[index].data.identity, 'hex'),
                        decryptWithAES(aesKey, events[index].data.idGraph, 'hex')
                    )
                );
                break;

            case 'IdentityRemoved':
                results.push(
                    createIdentityEvent(
                        context.api,
                        events[index].data.account.toHex(),
                        decryptWithAES(aesKey, events[index].data.identity, 'hex')
                    )
                );
                break;
            case 'Failed':
                results.push(events[index].data.detail.toHuman());
                break;
        }
    }
    console.log(`${type} event data:`, results);

    return [...results];
}

export async function handleVcEvents(
    aesKey: HexString,
    events: any[],
    method: 'VCIssued' | 'VCDisabled' | 'VCRevoked' | 'Failed'
): Promise<any> {
    let results: any = [];
    for (let k = 0; k < events.length; k++) {
        switch (method) {
            case 'VCIssued':
                results.push({
                    account: events[k].data.account.toHex(),
                    index: events[k].data.index.toHex(),
                    vc: decryptWithAES(aesKey, events[k].data.vc, 'utf-8'),
                });
                break;
            case 'VCDisabled':
                results.push(events[k].data.index.toHex());
                break;
            case 'VCRevoked':
                results.push(events[k].data.index.toHex());
                break;
            case 'Failed':
                results.push(events[k].data.detail.toHuman());
                break;
            default:
                break;
        }
    }
    return [...results];
}

export async function buildValidations(
    context: IntegrationTestContext,
    eventDatas: any[],
    identities: any[],
    network: 'ethereum' | 'substrate' | 'twitter',
    substrateSigners: KeyringPair[] | KeyringPair,
    ethereumSigners?: ethers.Wallet[]
): Promise<LitentryValidationData[]> {
    let signature_ethereum: HexString;
    let signature_substrate: Uint8Array;
    let verifyDatas: LitentryValidationData[] = [];

    for (let index = 0; index < eventDatas.length; index++) {
        const substrateSigner = Array.isArray(substrateSigners) ? substrateSigners[index] : substrateSigners;

        const ethereumSigner = network === 'ethereum' ? ethereumSigners![index] : undefined;

        const data = eventDatas[index];
        const msg = generateVerificationMessage(
            context,
            hexToU8a(data.challengeCode),
            substrateSigner.addressRaw,
            identities[index]
        );
        if (network === 'ethereum') {
            const ethereumValidationData: LitentryValidationData = {
                Web3Validation: {
                    Evm: {
                        message: '' as HexString,
                        signature: {
                            Ethereum: '' as HexString,
                        },
                    },
                },
            };
            console.log('post verification msg to ethereum: ', msg);
            ethereumValidationData!.Web3Validation!.Evm!.message = msg;
            const msgHash = ethers.utils.arrayify(msg);
            signature_ethereum = (await ethereumSigner!.signMessage(msgHash)) as HexString;
            console.log('signature_ethereum', ethereumSigners![index].address, signature_ethereum);

            ethereumValidationData!.Web3Validation!.Evm!.signature!.Ethereum = signature_ethereum;
            assert.isNotEmpty(data.challengeCode, 'ethereum challengeCode empty');
            console.log('ethereumValidationData', ethereumValidationData);

            verifyDatas.push(ethereumValidationData);
        } else if (network === 'substrate') {
            const substrateValidationData: LitentryValidationData = {
                Web3Validation: {
                    Substrate: {
                        message: '' as HexString,
                        signature: {
                            Sr25519: '' as HexString,
                        },
                    },
                },
            };
            console.log('post verification msg to substrate: ', msg);
            substrateValidationData!.Web3Validation!.Substrate!.message = msg;
            signature_substrate = substrateSigner.sign(msg) as Uint8Array;
            substrateValidationData!.Web3Validation!.Substrate!.signature!.Sr25519 = u8aToHex(signature_substrate);
            assert.isNotEmpty(data.challengeCode, 'substrate challengeCode empty');
            verifyDatas.push(substrateValidationData);
        } else if (network === 'twitter') {
            console.log('post verification msg to twitter', msg);
            const twitterValidationData: LitentryValidationData = {
                Web2Validation: {
                    Twitter: {
                        tweet_id: `0x${Buffer.from('100', 'utf8').toString('hex')}`,
                    },
                },
            };
            verifyDatas.push(twitterValidationData);
            assert.isNotEmpty(data.challengeCode, 'twitter challengeCode empty');
        }
    }
    return verifyDatas;
}

export async function buildIdentityHelper(
    address: HexString | string,
    network: SubstrateNetwork | EvmNetwork | Web2Network,
    type: 'Evm' | 'Substrate' | 'Web2'
): Promise<LitentryIdentity> {
    const identity: LitentryIdentity = {
        [type]: {
            address,
            network,
        },
    };
    return identity;
}

//If multiple transactions are built from multiple accounts, pass the signers as an array. If multiple transactions are built from a single account, signers cannot be an array.
export async function buildIdentityTxs(
    context: IntegrationTestContext,
    signers: KeyringPair[] | KeyringPair,
    identities: LitentryIdentity[],
    method: 'setUserShieldingKey' | 'createIdentity' | 'verifyIdentity' | 'removeIdentity',
    validations?: LitentryValidationData[]
): Promise<any[]> {
    const txs: any[] = [];
    const api = context.api;
    const mrEnclave = context.mrEnclave;
    const teeShieldingKey = context.teeShieldingKey;
    const len = Array.isArray(signers) ? signers.length : identities.length;
    for (let k = 0; k < len; k++) {
        const signer = Array.isArray(signers) ? signers[k] : signers;
        const identity = identities[k];
        let tx: SubmittableExtrinsic<ApiTypes>;
        let nonce: number;
        const encod_identity = api.createType('LitentryIdentity', identity).toU8a();
        const ciphertext_identity = encryptWithTeeShieldingKey(teeShieldingKey, encod_identity).toString('hex');
        nonce = (await api.rpc.system.accountNextIndex(signer.address)).toNumber();

        switch (method) {
            case 'setUserShieldingKey':
                const ciphertext = encryptWithTeeShieldingKey(
                    context.teeShieldingKey,
                    hexToU8a('0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12')
                ).toString('hex');
                tx = context.api.tx.identityManagement.setUserShieldingKey(context.mrEnclave, `0x${ciphertext}`);
                break;
            case 'createIdentity':
                tx = api.tx.identityManagement.createIdentity(
                    mrEnclave,
                    signer.address,
                    `0x${ciphertext_identity}`,
                    null
                );
                break;
            case 'verifyIdentity':
                const data = validations![k];
                const encode_verifyIdentity_validation = api.createType('LitentryValidationData', data).toU8a();
                const ciphertext_verifyIdentity_validation = encryptWithTeeShieldingKey(
                    teeShieldingKey,
                    encode_verifyIdentity_validation
                ).toString('hex');
                tx = api.tx.identityManagement.verifyIdentity(
                    mrEnclave,
                    `0x${ciphertext_identity}`,
                    `0x${ciphertext_verifyIdentity_validation}`
                );
                break;
            case 'removeIdentity':
                tx = api.tx.identityManagement.removeIdentity(mrEnclave, `0x${ciphertext_identity}`);
                break;
            default:
                throw new Error(`Invalid method: ${method}`);
        }
        txs.push({ tx, nonce });
    }

    return txs;
}

//campare two array of event_identities idgraph_identities whether equal
export function isArrayEqual(arr1: LitentryIdentity[], arr2: LitentryIdentity[]) {
    if (arr1.length !== arr2.length) {
        return false;
    }
    for (let i = 0; i < arr1.length; i++) {
        const obj1 = arr1[i];
        let found = false;

        for (let j = 0; j < arr2.length; j++) {
            const obj2 = arr2[j];

            if (isEqual(obj1, obj2)) {
                found = true;
                break;
            }
        }

        if (!found) {
            return false;
        }
    }
    return true;
}
function isEqual(obj1: LitentryIdentity, obj2: LitentryIdentity) {
    return JSON.stringify(obj1) === JSON.stringify(obj2);
}

export async function assertInitialIDGraphCreated(api: ApiPromise, signer: KeyringPair, event: IdentityGenericEvent) {
    assert.equal(event.who, u8aToHex(signer.addressRaw));
    assert.equal(event.idGraph.length, 1);
    // check identity in idgraph
    const expected_identity = api.createType(
        'LitentryIdentity',
        await buildIdentityHelper(u8aToHex(signer.addressRaw), 'LitentryRococo', 'Substrate')
    ) as LitentryIdentity;
    assert.isTrue(isEqual(event.idGraph[0][0], expected_identity));
    // check identityContext in idgraph
    assert.equal(event.idGraph[0][1].linking_request_block, 0);
    assert.equal(event.idGraph[0][1].verification_request_block, 0);
    assert.isTrue(event.idGraph[0][1].is_verified);
}

export function assertIdentityVerified(signer: KeyringPair, eventDatas: IdentityGenericEvent[]) {
    let event_identities: LitentryIdentity[] = [];
    let idgraph_identities: LitentryIdentity[] = [];
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
            assert.isTrue(data.idGraph[i][1].is_verified, 'identity should be verified');
        }
    }
    assert.equal(data?.who, u8aToHex(signer.addressRaw), 'check caller error');
}

export function assertIdentityCreated(signer: KeyringPair, identityEvent: IdentityGenericEvent | undefined) {
    assert.equal(identityEvent?.who, u8aToHex(signer.addressRaw), 'check caller error');
}

export function assertIdentityRemoved(signer: KeyringPair, identityEvent: IdentityGenericEvent | undefined) {
    assert.equal(identityEvent?.idGraph, null, 'check idGraph error,should be null after removed');
    assert.equal(identityEvent?.who, u8aToHex(signer.addressRaw), 'check caller error');
}
