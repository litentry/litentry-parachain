import { cryptoWaitReady } from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { default as teeTypes } from '../../parachain-interfaces/identity/definitions';
import { HexString } from '@polkadot/util/types';
import {
    createSignedTrustedCallSetUserShieldingKey,
    sendRequestFromTrustedCall,
    getTEEShieldingKey,
    createSignedTrustedCallCreateIdentity,
    createSignedTrustedGetterUserShieldingKey,
    sendRequestFromTrustedGetter,
    createPublicGetterAccountNonce,
    sendRequestFromPublicGetter,
    decodeNonce,
} from './util';
import { getEnclave, sleep, buildIdentityHelper, initIntegrationTestContext } from '../../common/utils';
import { Metadata, TypeRegistry } from '@polkadot/types';
import sidechainMetaData from '../../litentry-sidechain-metadata.json';
import { hexToU8a, compactStripLength, u8aToString } from '@polkadot/util';
import { assert } from 'chai';

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
    const sidechainRegistry = new TypeRegistry();
    const metaData = new Metadata(sidechainRegistry, sidechainMetaData.result as HexString);
    const registry = new TypeRegistry();

    const { types } = teeTypes;

    sidechainRegistry.setMetadata(metaData);
    const parachain_api = await ApiPromise.create({
        provider: parachain_ws,
        types,
    });

    const context = await initIntegrationTestContext(
        WORKER_TRUSTED_WS_ENDPOINT, // @fixme evil assertion; centralize env access
        PARACHAIN_WS_ENDPINT, // @fixme evil assertion; centralize env access
        0
    );

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
    const bob: KeyringPair = keyring.addFromUri('//Bob', { name: 'Bob' });
    const mrenclave = (await getEnclave(parachain_api)).mrEnclave;

    const NonceGetter = createPublicGetterAccountNonce(parachain_api, alice);
    const noncex = await sendRequestFromPublicGetter(wsp, parachain_api, mrenclave, key, NonceGetter);
    const NonceValue = decodeNonce(noncex.value.toHex());
    let nonce = parachain_api.createType('Index', NonceValue);

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
    let res = await sendRequestFromTrustedCall(wsp, parachain_api, mrenclave, key, setUserShieldingKeyCall);
    console.log('setUserShieldingKey call returned', res.toHuman());

    sleep(10);

    hash = `0x${require('crypto').randomBytes(32).toString('hex')}`;

    const noncex1 = await sendRequestFromPublicGetter(wsp, parachain_api, mrenclave, key, NonceGetter);
    const NonceValue1 = decodeNonce(noncex1.value.toHex());

    nonce = parachain_api.createType('Index', NonceValue1);
    console.log('Send direct createIdentity call... hash:', hash);
    const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2', context);
    let createIdentityCall = createSignedTrustedCallCreateIdentity(
        parachain_api,
        mrenclave,
        nonce,
        alice,
        sidechainRegistry.createType('LitentryPrimitivesIdentity', twitter_identity).toHex(),
        '0x',
        parachain_api.createType('u32', 1).toHex(),
        hash
    );
    res = await sendRequestFromTrustedCall(wsp, parachain_api, mrenclave, key, createIdentityCall);
    console.log('createIdentity call returned', res.toHuman());

    sleep(10);

    console.log('Send UserShieldingKey getter for alice ...');
    let UserShieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(parachain_api, alice);
    res = await sendRequestFromTrustedGetter(wsp, parachain_api, mrenclave, key, UserShieldingKeyGetter);
    console.log('UserShieldingKey getter returned', res.toHuman());
    // the returned res.value of the trustedGetter is of Option<> type
    // res.value should be `0x018022fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12`
    // TODO: why `createType` must accept an Uint8Array here? The following still prints the unwrapped value
    //       let k = parachain_api.createType('Option<Bytes>', res.value.toHex());
    //       console.log("k.isSome", k.isSome); // true
    //       console.log("k.unwrap", k.unwrap().toHex()); // still 0x018022fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12
    let k = parachain_api.createType('Option<Bytes>', hexToU8a(res.value.toHex()));
    assert.isTrue(k.isSome);
    assert.equal(k.unwrap().toHex(), key_alice);

    // bob's shielding key should be none
    console.log('Send UserShieldingKey getter for bob ...');
    UserShieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(parachain_api, bob);
    res = await sendRequestFromTrustedGetter(wsp, parachain_api, mrenclave, key, UserShieldingKeyGetter);
    console.log('UserShieldingKey getter returned', res.toHuman());
    k = parachain_api.createType('Option<Bytes>', hexToU8a(res.value.toHex()));
    assert.isTrue(k.isNone);
}

(async () => {
    await runDirectCall().catch((e) => {
        console.error(e);
    });
    process.exit(0);
})();
