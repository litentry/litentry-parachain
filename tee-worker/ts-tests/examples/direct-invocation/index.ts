import { cryptoWaitReady } from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { TypeRegistry, Bytes } from '@polkadot/types';
import { Metadata } from '@polkadot/types/metadata';
import { BN, u8aToHex, hexToU8a, u8aToBuffer, u8aToString, compactAddLength, bufferToU8a } from '@polkadot/util';
import { teeTypes } from '../../common/type-definitions';
import { Codec } from '@polkadot/types/types';
import { sendRequestBalanceTransfer, toBalance } from './util';

// in order to handle self-signed certificates we need to turn off the validation
// TODO add self signed certificate
process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

const base58 = require('micro-base58');

const WebSocketAsPromised = require('websocket-as-promised');
const WebSocket = require('ws');
const keyring = new Keyring({ type: 'sr25519' });

const PARACHAIN_WS_ENDPINT = 'ws://localhost:9944';
const WORKER_TRUSTED_WS_ENDPOINT = 'wss://localhost:2000';

type WorkerRpcReturnValue = {
    value: Uint8Array;
    do_watch: boolean;
    status: string;
};

type WorkerRpcReturnString = {
    vec: string;
};

type RsaPublicKey = {
    n: Uint8Array;
    e: Uint8Array;
};

async function runDirectCall() {
    const parachain_ws = new WsProvider(PARACHAIN_WS_ENDPINT);
    const registry = new TypeRegistry();
    const parachain_api = await ApiPromise.create({
        provider: parachain_ws,
        types: teeTypes,
    });
    await cryptoWaitReady();
    const wsp = new WebSocketAsPromised(WORKER_TRUSTED_WS_ENDPOINT, {
        createWebSocket: (url: any) => new WebSocket(url),
        extractMessageData: (event: any) => event,
        packMessage: (data: any) => JSON.stringify(data),
        unpackMessage: (data: string) => JSON.parse(data),
        attachRequestId: (data: any, requestId: string | number) => Object.assign({ id: requestId }, data),
        extractRequestId: (data: any) => data && data.id,
    });
    await wsp.open();

    const alice: KeyringPair = keyring.addFromUri('//Alice', { name: 'Alice' });
    const bob = keyring.addFromUri('//Bob', { name: 'Bob' });
    const mrenclave = '0x778a084fa0722b14b177e672bcee2a38c5b3690c11dd37d51adeb88ffaf75f72';
    await sendRequestBalanceTransfer(wsp, parachain_api, alice, bob.address, mrenclave, toBalance(1));
}

(async () => {
    await runDirectCall().catch((e) => {
        console.error(e);
    });
    process.exit(0);
})();
