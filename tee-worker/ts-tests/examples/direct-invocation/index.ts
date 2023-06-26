import { cryptoWaitReady } from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { default as teeTypes } from '../../parachain-interfaces/identity/definitions';
import { HexString } from '@polkadot/util/types';
import {
    createSignedTrustedCallSetUserShieldingKey,
    sendRequestFromTrustedCall,
    getTEEShieldingKey,
    createSignedTrustedCallLinkIdentity,
    createSignedTrustedGetterUserShieldingKey,
    createSignedTrustedGetterIDGraph,
    sendRequestFromGetter,
    getSidechainNonce,
    getKeyPair,
} from './util';
import {
    getEnclave,
    sleep,
    buildIdentityHelper,
    initIntegrationTestContext,
    buildAddressHelper,
    buildIdGraphIdentityHelper,
} from '../../common/utils';
import { aesKey, keyNonce } from '../../common/call';
import { Metadata, TypeRegistry } from '@polkadot/types';
import sidechainMetaData from '../../litentry-sidechain-metadata.json';
import { hexToU8a, u8aToString, u8aToHex } from '@polkadot/util';
import { assert } from 'chai';
import { KeypairType } from '@polkadot/util-crypto/types';
import type { LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext } from '@polkadot/types/lookup';

// in order to handle self-signed certificates we need to turn off the validation
// TODO add self signed certificate
process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

const WebSocketAsPromised = require('websocket-as-promised');
const WebSocket = require('ws');
const keyring = new Keyring({ type: 'sr25519' });

const PARACHAIN_WS_ENDPINT = 'ws://localhost:9944';
const WORKER_TRUSTED_WS_ENDPOINT = 'wss://localhost:2000';

async function runDirectCall(keyPairType: KeypairType) {
    const keyring = new Keyring({ type: keyPairType });

    const parachain_ws = new WsProvider(PARACHAIN_WS_ENDPINT);
    const sidechainRegistry = new TypeRegistry();
    const metaData = new Metadata(sidechainRegistry, sidechainMetaData.result as HexString);
    sidechainRegistry.setMetadata(metaData);
    const { types } = teeTypes;
    const parachain_api = await ApiPromise.create({
        provider: parachain_ws,
        types,
    });
    const context = await initIntegrationTestContext(WORKER_TRUSTED_WS_ENDPOINT, PARACHAIN_WS_ENDPINT, 0);

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

    const alice: KeyringPair = getKeyPair('Alice', keyring);
    const bob: KeyringPair = getKeyPair('Bob', keyring);
    const mrenclave = (await getEnclave(parachain_api)).mrEnclave;

    let aliceAddress = await buildAddressHelper(alice);
    let bobAddress = await buildAddressHelper(bob);

    let aliceIdGraphIdentifier = await buildIdGraphIdentityHelper(alice);
    let bobIdGraphIdentifier = await buildIdGraphIdentityHelper(bob);

    let nonce = await getSidechainNonce(wsp, parachain_api, mrenclave, key, aliceAddress);

    // similar to `reqExtHash` in indirect calls, we need some "identifiers" to pair the response
    // with the request. Ideally it's the hash of the trusted operation, but we need it before constructing
    // a trusted call, hence a random number is used here - better ideas are welcome
    let hash = `0x${require('crypto').randomBytes(32).toString('hex')}`;
    console.log('Send direct setUserShieldingKey call for alice ... hash:', hash);
    let setUserShieldingKeyCall = createSignedTrustedCallSetUserShieldingKey(
        parachain_api,
        mrenclave,
        nonce,
        alice,
        aliceAddress,
        aliceIdGraphIdentifier,
        aesKey,
        hash
    );
    let res = await sendRequestFromTrustedCall(wsp, parachain_api, mrenclave, key, setUserShieldingKeyCall);
    console.log('setUserShieldingKey call returned', res.toHuman());

    await sleep(10);

    hash = `0x${require('crypto').randomBytes(32).toString('hex')}`;
    nonce = await getSidechainNonce(wsp, parachain_api, mrenclave, key, aliceAddress);

    console.log('Send direct linkIdentity call... hash:', hash);
    const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2', context);
    let linkIdentityCall = createSignedTrustedCallLinkIdentity(
        parachain_api,
        mrenclave,
        nonce,
        alice,
        aliceAddress,
        aliceIdGraphIdentifier,
        sidechainRegistry.createType('LitentryPrimitivesIdentity', twitter_identity).toHex(),
        parachain_api
            .createType('LitentryValidationData', {
                Web2Validation: {
                    Twitter: {
                        tweet_id: `0x${Buffer.from(nonce.toString(), 'utf8').toString('hex')}`,
                    },
                },
            })
            .toHex(),
        keyNonce,
        hash
    );
    res = await sendRequestFromTrustedCall(wsp, parachain_api, mrenclave, key, linkIdentityCall);
    console.log('linkIdentity call returned', res.toHuman());

    // we should have listened to the parachain event, for demo purpose we only wait for enough
    // time and check the IDGraph
    await sleep(30);

    console.log('Send IDGraph getter for alice ...');
    let idgraphGetter = createSignedTrustedGetterIDGraph(parachain_api, alice);
    res = await sendRequestFromGetter(wsp, parachain_api, mrenclave, key, idgraphGetter);
    console.log('IDGraph getter returned', res.toHuman());
    // somehow createType('Option<Vec<(....)>>') doesn't work, why?
    let idgraphBytes = sidechainRegistry.createType('Option<Bytes>', hexToU8a(res.value.toHex()));
    assert.isTrue(idgraphBytes.isSome);
    let idgraphArray = sidechainRegistry.createType(
        'Vec<(LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext)>',
        idgraphBytes.unwrap()
    ) as unknown as [LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext][];
    assert.equal(idgraphArray.length, 2);
    // the first identity is the twitter identity
    assert.isTrue(idgraphArray[0][0].isWeb2);
    assert.isTrue(idgraphArray[0][0].asWeb2.network.isTwitter);
    assert.equal(u8aToString(idgraphArray[0][0].asWeb2.address.toU8a()), '$mock_user');
    assert.isTrue(idgraphArray[0][1].status.isActive);
    // the second identity is the substrate identity (prime identity)
    assert.isTrue(idgraphArray[1][0].isSubstrate);
    assert.isTrue(idgraphArray[1][0].asSubstrate.network.isLitentryRococo);
    assert.equal(idgraphArray[1][0].asSubstrate.address.toHex(), u8aToHex(alice.publicKey));
    assert.isTrue(idgraphArray[1][1].status.isActive);

    console.log('Send UserShieldingKey getter for alice ...');
    let UserShieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(parachain_api, alice, aliceAddress);
    res = await sendRequestFromGetter(wsp, parachain_api, mrenclave, key, UserShieldingKeyGetter);
    console.log('UserShieldingKey getter returned', res.toHuman());
    // the returned res.value of the trustedGetter is of Option<> type
    // res.value should be `0x018022fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12`
    // TODO: why `createType` must accept an Uint8Array here? The following still prints the unwrapped value
    //       let k = parachain_api.createType('Option<Bytes>', res.value.toHex());
    //       console.log("k.isSome", k.isSome); // true
    //       console.log("k.unwrap", k.unwrap().toHex()); // still 0x018022fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12
    let k = parachain_api.createType('Option<Bytes>', hexToU8a(res.value.toHex()));
    assert.isTrue(k.isSome);
    assert.equal(k.unwrap().toHex(), aesKey);

    // bob's shielding key should be none
    console.log('Send UserShieldingKey getter for bob ...');
    UserShieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(parachain_api, bob, bobAddress);
    res = await sendRequestFromGetter(wsp, parachain_api, mrenclave, key, UserShieldingKeyGetter);
    console.log('UserShieldingKey getter returned', res.toHuman());
    k = parachain_api.createType('Option<Bytes>', hexToU8a(res.value.toHex()));
    assert.isTrue(k.isNone);

    await sleep(10);

    nonce = await getSidechainNonce(wsp, parachain_api, mrenclave, key, bobAddress);

    // set bob's shielding key, with wrapped bytes
    let key_bob = '0x8378193a4ce64180814bd60591d1054a04dbc4da02afde453799cd6888ee0c6c';
    hash = `0x${require('crypto').randomBytes(32).toString('hex')}`;
    console.log('Send direct setUserShieldingKey call for bob, with wrapped bytes... hash:', hash);
    setUserShieldingKeyCall = createSignedTrustedCallSetUserShieldingKey(
        parachain_api,
        mrenclave,
        nonce,
        bob,
        bobAddress,
        bobIdGraphIdentifier,
        key_bob,
        hash,
        true // with wrapped bytes
    );
    res = await sendRequestFromTrustedCall(wsp, parachain_api, mrenclave, key, setUserShieldingKeyCall);
    console.log('setUserShieldingKey call returned', res.toHuman());

    // verify that bob's key is set
    console.log('Send UserShieldingKey getter for bob ...');
    UserShieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(parachain_api, bob, bobAddress);
    res = await sendRequestFromGetter(wsp, parachain_api, mrenclave, key, UserShieldingKeyGetter);
    console.log('UserShieldingKey getter returned', res.toHuman());
    k = parachain_api.createType('Option<Bytes>', hexToU8a(res.value.toHex()));
    assert.isTrue(k.isSome);
    assert.equal(k.unwrap().toHex(), key_bob);
}

(async () => {
    await runDirectCall('sr25519').catch((e) => {
        console.error(e);
    });
    process.exit(0);
})();
