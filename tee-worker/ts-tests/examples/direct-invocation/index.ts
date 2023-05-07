import { cryptoWaitReady } from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { TypeRegistry, Bytes } from '@polkadot/types';
import { teeTypes } from '../../common/type-definitions';
import { createSignedTrustedCallBalanceTransfer, sendRequestFromTrustedCall, toBalance } from './util';
import { getEnclave } from '../../common/utils';

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
    const mrenclave = (await getEnclave(parachain_api)).mrEnclave;
    let nonce = parachain_api.createType('Index', '0x01');
    let balanceTransferCall = createSignedTrustedCallBalanceTransfer(
        parachain_api,
        nonce,
        alice,
        bob.address,
        mrenclave,
        toBalance(1)
    );
    await sendRequestFromTrustedCall(wsp, parachain_api, mrenclave, balanceTransferCall);
}

(async () => {
    await runDirectCall().catch((e) => {
        console.error(e);
    });
    process.exit(0);
})();
