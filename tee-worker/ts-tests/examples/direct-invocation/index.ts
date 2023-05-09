import { cryptoWaitReady } from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { TypeRegistry } from '@polkadot/types';
import { hexToU8a, u8aToHex, u8aConcat } from '@polkadot/util';
import { teeTypes } from '../../common/type-definitions';
import {
    createSignedTrustedCallBalanceTransfer,
    createSignedTrustedCallSetUserShieldingKey,
    sendRequestFromTrustedCall,
    toBalance,
    getTEEShieldingKey,
} from './util';
import { getEnclave, sleep } from '../../common/utils';
import { H256 } from '@polkadot/types/interfaces';

// in order to handle self-signed certificates we need to turn off the validation
// TODO add self signed certificate
process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

const base58 = require('micro-base58');

const WebSocketAsPromised = require('websocket-as-promised');
const WebSocket = require('ws');
const keyring = new Keyring({ type: 'sr25519' });

const PARACHAIN_WS_ENDPINT = 'ws://localhost:9944';
const WORKER_TRUSTED_WS_ENDPOINT = 'wss://localhost:2000';

async function runDirectCall() {
    const parachain_ws = new WsProvider(PARACHAIN_WS_ENDPINT);
    const registry = new TypeRegistry();
    const { types } = teeTypes;
    const parachain_api = await ApiPromise.create({
        provider: parachain_ws,
        types,
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

    let key = await getTEEShieldingKey(wsp, parachain_api);

    const alice: KeyringPair = keyring.addFromUri('//Alice', { name: 'Alice' });
    const bob = keyring.addFromUri('//Bob', { name: 'Bob' });
    const mrenclave = (await getEnclave(parachain_api)).mrEnclave;
    let nonce = parachain_api.createType('Index', '0x00');

    console.log('sending balanceTransferCall...');
    // try out balance transfer call
    // let balanceTransferCall = createSignedTrustedCallBalanceTransfer(
    //     parachain_api,
    //     mrenclave,
    //     nonce,
    //     alice,
    //     bob.address,
    //     toBalance(1)
    // );
    // await sendRequestFromTrustedCall(wsp, parachain_api, mrenclave, key, balanceTransferCall);

    sleep(10);

    console.log('sending setUserShieldingKeyCall...');
    // try out set_user_shielding_key directly
    nonce = parachain_api.createType('Index', '0x00');
    // a hardcoded AES key which is used overall in tests - maybe we need to put it in a common place
    let key_alice = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    // the hash was used to track the request extrinsic hash
    // now that we don't send the request via extrinsic, it can be some random "ID" that uniquely
    // identifies a request
    let hash = `0x${require('crypto').randomBytes(32).toString('hex')}`;
    console.log('sendRequestFromTrustedCall, hash: ', hash);
    // hash = parachain_api.createType('H256', hash).toHex();
    let setUserShieldingKeyCall = createSignedTrustedCallSetUserShieldingKey(
        parachain_api,
        mrenclave,
        nonce,
        alice,
        key_alice,
        hash
    );
    await sendRequestFromTrustedCall(wsp, parachain_api, mrenclave, key, setUserShieldingKeyCall);
}

(async () => {
    await runDirectCall().catch((e) => {
        console.error(e);
    });
    process.exit(0);
})();
