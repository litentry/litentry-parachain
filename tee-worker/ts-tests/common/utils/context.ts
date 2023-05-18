import { WsProvider, ApiPromise, Keyring } from '@polkadot/api';
import { cryptoWaitReady } from '@polkadot/util-crypto';
import { HexString } from '@polkadot/util/types';
import { ethers } from 'ethers';
import WebSocketAsPromised from 'websocket-as-promised';
import WebSocket from 'ws';
import Options from 'websocket-as-promised/types/options';
import { KeyObject } from 'crypto';
import { getMetadata } from '../call';
import { getEthereumSigner, getSubstrateSigner } from '../helpers';
import { IntegrationTestContext, teeTypes, EnclaveResult, Web3Wallets } from '../type-definitions';
import { decryptWithAES } from './crypto';

const crypto = require('crypto');

// maximum milliseconds that we wait in listening events before we timeout
const listenTimeoutInMilliSeconds = 3 * 60 * 1000;

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
