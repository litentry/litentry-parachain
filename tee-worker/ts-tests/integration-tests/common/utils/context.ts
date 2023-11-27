import { WsProvider, ApiPromise, TeerexPrimitivesEnclave } from 'parachain-api';
import { Keyring } from '@polkadot/api';
import { cryptoWaitReady } from '@polkadot/util-crypto';
import { ethers } from 'ethers';
import WebSocketAsPromised from 'websocket-as-promised';
import WebSocket from 'ws';
import Options from 'websocket-as-promised/types/options';
import { KeyObject } from 'crypto';
import { getSidechainMetadata } from '../call';
import { getEvmSigner, getSubstrateSigner } from '../helpers';
import type { IntegrationTestContext, Web3Wallets } from '../common-definitions';

import { identity, vc, trusted_operations, sidechain } from 'parachain-api';
import crypto from 'crypto';
import type { HexString } from '@polkadot/util/types';

// maximum block number that we wait in listening events before we timeout
export const defaultListenTimeoutInBlockNumber = 15;

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
    await cryptoWaitReady();

    const ethersWallet = {
        alice: new ethers.Wallet(getEvmSigner().alice),
        bob: new ethers.Wallet(getEvmSigner().bob),
        charlie: new ethers.Wallet(getEvmSigner().charlie),
        dave: new ethers.Wallet(getEvmSigner().dave),
        eve: new ethers.Wallet(getEvmSigner().eve),
    };

    const substrateWallet = getSubstrateSigner();

    const types = { ...identity.types, ...vc.types, ...trusted_operations.types, ...sidechain.types };

    const api = await ApiPromise.create({
        provider,
        types,
    });

    const chainIdentifier = api.registry.chainSS58 as number;

    const wsp = await initWorkerConnection(workerEndpoint);
    const requestId = 1;

    const { sidechainMetaData, sidechainRegistry } = await getSidechainMetadata(wsp, api, requestId);
    const web3Signers = await generateWeb3Wallets(walletsNumber);
    const { mrEnclave, teeShieldingKey } = await getEnclave(api);
    return {
        tee: wsp,
        api,
        teeShieldingKey,
        mrEnclave,
        ethersWallet,
        substrateWallet,
        sidechainMetaData,
        sidechainRegistry,
        web3Signers,
        chainIdentifier,
        requestId,
    };
}

export async function getEnclave(api: ApiPromise): Promise<{
    mrEnclave: HexString;
    teeShieldingKey: KeyObject;
}> {
    const count = await api.query.teerex.enclaveCount();

    const enclaveRegistry = (
        await api.query.teerex.enclaveRegistry(count)
    ).toHuman() as unknown as TeerexPrimitivesEnclave;

    const teeShieldingKey = crypto.createPublicKey({
        key: {
            alg: 'RSA-OAEP-256',
            kty: 'RSA',
            use: 'enc',
            n: Buffer.from(JSON.parse(enclaveRegistry.shieldingKey as unknown as HexString).n.reverse()).toString(
                'base64url'
            ),
            e: Buffer.from(JSON.parse(enclaveRegistry.shieldingKey as unknown as HexString).e.reverse()).toString(
                'base64url'
            ),
        },
        format: 'jwk',
    });
    //@TODO mrEnclave should verify from storage
    const mrEnclave = enclaveRegistry.mrEnclave as unknown as HexString;
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
        const evmWallet = ethers.Wallet.createRandom();
        addresses.push({
            substrateWallet: substratePair,
            evmWallet: evmWallet,
        });
    }
    return addresses;
}
