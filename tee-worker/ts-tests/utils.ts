import './config';
import WebSocketAsPromised from 'websocket-as-promised';
import WebSocket from 'ws';
import Options from 'websocket-as-promised/types/options';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { StorageKey, Vec } from '@polkadot/types';
import {
    AESOutput,
    EnclaveResult,
    IntegrationTestContext,
    LitentryIdentity,
    PubicKeyJson,
    teeTypes,
    WorkerRpcReturnString,
    WorkerRpcReturnValue,
} from './type-definitions';
import { blake2AsHex, cryptoWaitReady, signatureVerify } from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';
import { Codec } from '@polkadot/types/types';
import { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import { HexString } from '@polkadot/util/types';
import { hexToU8a, u8aToHex, stringToU8a, stringToHex, u8aToU8a } from '@polkadot/util';
import { KeyObject } from 'crypto';
import { Event, EventRecord } from '@polkadot/types/interfaces';
import { after, before, describe } from 'mocha';
import { generateChallengeCode, getSigner } from './web3/setup';
import { ethers } from 'ethers';
import { generateTestKeys } from './web3/functions';
import { expect } from 'chai';
const ed25519 = require('tweetnacl').sign;
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
    const { mrEnclave, teeShieldingKey } = await getEnclave(api);
    return <IntegrationTestContext>{
        tee: wsp,
        substrate: api,
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

// Subscribe to the chain until we get the first specified event with given `section` and `methods`.
// We can listen to multiple `methods` as long as they are emitted in the same block.
// The event consumer should do the decryption optionaly as it's event specific
//
// TODO: occassionally multiple events for an extrinsic are not included in the same block,
//       e.g. `create_identity` => `IdentityCreated`, `ChallengeCodeGenerated`
//       this is because the extrinsics are submitted asynchronously and in rare cases these two
//       extrinsics are included in the different parentchain blocks
// Solutions:
//  1. (pallet change) use one single extrinsic to emit both events, if they should always be triggered on pair
//  2. (ts-test change) only resolve this promise when both events are received, but not necessarily in the same block
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
    const encode = context.substrate.createType('LitentryIdentity', identity).toU8a();
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
            substrate: {} as ApiPromise,
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
            context.substrate = tmp.substrate;
            context.tee = tmp.tee;
            context.teeShieldingKey = tmp.teeShieldingKey;
            context.ethersWallet = tmp.ethersWallet;
        });

        after(async function () {});

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
    const mrEnclave = res.mrEnclave;
    return {
        mrEnclave,
        teeShieldingKey,
    };
}

export async function verifyMsg(data: string, publicKey: KeyObject, signature: string, api: ApiPromise) {
    const count = await api.query.teerex.enclaveCount();
    const res = (await api.query.teerex.enclaveRegistry(count)).toHuman() as EnclaveResult;

    const hash = crypto.createHash('blake2s256').update(stringToU8a(res.shieldingKey)).digest();
    const message = JSON.parse(data);
    console.log(message);
    delete message.proof;
    console.log(999, hash);
    const keyPair = ed25519.keyPair.fromSeed(hash);
    console.log('keyPair', keyPair);

    const isValid = ed25519.detached.verify(stringToU8a(message), hexToU8a(`0x${signature}`), keyPair.publicKey);
    console.log('isValid', isValid);

    await crypto.generateKeyPair(
        'ed25519',
        {
            publicKeyEncoding: {
                type: 'spki',
                format: 'pem',
            },
            privateKeyEncoding: {
                type: 'pkcs8',
                format: 'pem',
                cipher: 'aes-256-cbc',
                passphrase: hash,
            },
        },
        (err: any, publicKey: any, privateKey: any) => {
            if (err) throw err;
            console.log(crypto.verify(null, stringToU8a(message), publicKey, stringToU8a(signature)));
        }
    );
}

export async function verifySignature(publicKey: KeyObject, vc: string, api: ApiPromise): Promise<any> {
    const vcObj = JSON.parse(vc);
    await verifyMsg(vc, publicKey, vcObj.proof.proofValue, api);
    const jsonStatus = await checkJSON(vc);
}

//Check VC json fields
export async function checkJSON(data: string): Promise<boolean> {
    const vc = JSON.parse(data);
    const vcStatus = ['@context', 'type', 'credentialSubject', 'proof', 'issuer'].every(
        (key) =>
            vc.hasOwnProperty(key) && (vc[key] != '{}' || vc[key] !== '[]' || vc[key] !== null || vc[key] !== undefined)
    );
    expect(vcStatus).to.be.true;
    expect(
        vc.type[0] === 'VerifiableCredential' &&
            vc.proof.type === 'Ed25519Signature2020' &&
            vc.issuer.id === vc.proof.verificationMethod
    ).to.be.true;
    return true;
}
