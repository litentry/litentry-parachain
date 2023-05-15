import { cryptoWaitReady } from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { TypeRegistry } from '@polkadot/types';
import { hexToU8a, u8aToHex, u8aConcat } from '@polkadot/util';
import { teeTypes } from '../../common/type-definitions';
import {
    createSignedTrustedCallSetUserShieldingKey,
    sendRequestFromTrustedCall,
    getTEEShieldingKey,
    createSignedTrustedCallCreateIdentity,
} from './util';
import { getEnclave, sleep, buildIdentityHelper } from '../../common/utils';

// in order to handle self-signed certificates we need to turn off the validation
// TODO add self signed certificate
process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

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
    const mrenclave = (await getEnclave(parachain_api)).mrEnclave;
    let nonce = parachain_api.createType('Index', '0x00');

    // a hardcoded AES key which is used overall in tests - maybe we need to put it in a common place
    let key_alice = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    // similar to `reqExtHash` in indirect calls, we need some "identifiers" to pair the response
    // with the request. Ideally it's the hash of the trusted operation, but we need it before constructing
    // a trusted call, hence a random number is used here - better ideas are welcome
    let hash = `0x${require('crypto').randomBytes(32).toString('hex')}`;
    console.log('Send direct setUserShieldingKey call... hash:', hash);
    let setUserShieldingKeyCall = createSignedTrustedCallSetUserShieldingKey(
        parachain_api,
        mrenclave,
        nonce,
        alice,
        key_alice,
        hash
    );
    await sendRequestFromTrustedCall(wsp, parachain_api, mrenclave, key, setUserShieldingKeyCall);
    console.log('setUserShieldingKey call returned');

    sleep(10);

    hash = `0x${require('crypto').randomBytes(32).toString('hex')}`;
    nonce = parachain_api.createType('Index', '0x01');
    console.log('Send direct createIdentity call... hash:', hash);
    const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2');
    let createIdentityCall = createSignedTrustedCallCreateIdentity(
        parachain_api,
        mrenclave,
        nonce,
        alice,
        parachain_api.createType('LitentryIdentity', twitter_identity).toHex(),
        '0x',
        parachain_api.createType('u32', 1).toHex(),
        hash
    );
    await sendRequestFromTrustedCall(wsp, parachain_api, mrenclave, key, createIdentityCall);
    console.log('createIdentity call returned');

    sleep(10);
}

(async () => {
    await runDirectCall().catch((e) => {
        console.error(e);
    });
    process.exit(0);
})();
