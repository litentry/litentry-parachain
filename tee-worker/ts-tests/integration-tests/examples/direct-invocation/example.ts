import { cryptoWaitReady } from '@polkadot/util-crypto';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { default as teeTypes } from '../../../parachain-api/build/interfaces/identity/definitions';
import { HexString } from '@polkadot/util/types';
import {
    createSignedTrustedCallSetUserShieldingKey,
    sendRequestFromTrustedCall,
    getTeeShieldingKey,
    createSignedTrustedCallLinkIdentity,
    createSignedTrustedGetterUserShieldingKey,
    createSignedTrustedCallSetIdentityNetworks,
    createSignedTrustedGetterIdGraph,
    sendRequestFromGetter,
    getSidechainNonce,
    decodeIdGraph,
    getKeyPair
} from './util';
import {
    getEnclave,
    sleep,
    buildIdentityHelper,
    initIntegrationTestContext,
    buildValidations,
    buildIdentityFromKeypair
} from '../../common/utils';
import { aesKey, keyNonce } from '../../common/call';
import { Metadata, TypeRegistry } from '@polkadot/types';
import sidechainMetaData from '../../litentry-sidechain-metadata.json' assert { type: 'json' };
import { hexToU8a, u8aToHex } from '@polkadot/util';
import { assert } from 'chai';
import Options from 'websocket-as-promised/types/options';
import crypto from 'crypto';
import { KeypairType } from '@polkadot/util-crypto/types';
// in order to handle self-signed certificates we need to turn off the validation
// TODO add self signed certificate
process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

import WebSocketAsPromised from 'websocket-as-promised';
import webSocket from 'ws';
const substrateKeyring = new Keyring({ type: 'sr25519' });

const PARACHAIN_WS_ENDPINT = 'ws://localhost:9944';
const WORKER_TRUSTED_WS_ENDPOINT = 'wss://localhost:2000';

export type Mode = 'substrate' | 'evm';

export async function runExample(mode: Mode) {
    const parachainWs = new WsProvider(PARACHAIN_WS_ENDPINT);
    const sidechainRegistry = new TypeRegistry();
    const metaData = new Metadata(sidechainRegistry, sidechainMetaData.result as HexString);
    sidechainRegistry.setMetadata(metaData);
    const { types } = teeTypes;
    const parachainApi = await ApiPromise.create({
        provider: parachainWs,
        types,
    });
    const context = await initIntegrationTestContext(WORKER_TRUSTED_WS_ENDPOINT, PARACHAIN_WS_ENDPINT, 0);

    await cryptoWaitReady();
    const wsp = new WebSocketAsPromised(WORKER_TRUSTED_WS_ENDPOINT, <Options>(<unknown>{
        createWebSocket: (url: any) => new webSocket(url),
        extractMessageData: (event: any) => event,
        packMessage: (data: any) => JSON.stringify(data),
        unpackMessage: (data: string) => JSON.parse(data),
        attachRequestId: (data: any, requestId: string | number) => Object.assign({ id: requestId }, data),
        extractRequestId: (data: any) => data && data.id,
    }));
    await wsp.open();

    const key = await getTeeShieldingKey(wsp, parachainApi);

    const alice: Signer =
        mode == 'substrate'
            ? new PolkadotSigner(context.substrateWallet['alice'])
            : new EthersSigner(context.ethersWallet['alice']);
    const bob: Signer =
        mode == 'substrate'
            ? new PolkadotSigner(context.substrateWallet['bob'])
            : new EthersSigner(context.ethersWallet['bob']);

    const bobSubstrateKey: KeyringPair = substrateKeyring.addFromUri('//Bob', { name: 'Bob' });

    const mrenclave = (await getEnclave(parachainApi)).mrEnclave;

    const aliceSubject = await buildIdentityFromKeypair(alice, context);
    const bobSubject = await buildIdentityFromKeypair(bob, context);

    let nonce = await getSidechainNonce(wsp, parachainApi, mrenclave, key, aliceSubject);

    // similar to `reqExtHash` in indirect calls, we need some "identifiers" to pair the response
    // with the request. Ideally it's the hash of the trusted operation, but we need it before constructing
    // a trusted call, hence a random number is used here - better ideas are welcome
    let hash = `0x${crypto.randomBytes(32).toString('hex')}`;
    console.log('Send direct setUserShieldingKey call for alice ... hash:', hash);
    let setUserShieldingKeyCall = await createSignedTrustedCallSetUserShieldingKey(
        parachainApi,
        mrenclave,
        nonce,
        alice,
        aliceSubject,
        aesKey,
        hash
    );
    let res = await sendRequestFromTrustedCall(wsp, parachainApi, mrenclave, key, setUserShieldingKeyCall);
    console.log('setUserShieldingKey call returned', res.toHuman());

    await sleep(10);

    hash = `0x${crypto.randomBytes(32).toString('hex')}`;
    nonce = await getSidechainNonce(wsp, parachainApi, mrenclave, key, aliceSubject);

    console.log('Send direct linkIdentity call... hash:', hash);
    const bobSubstrateIdentity = await buildIdentityHelper(u8aToHex(bobSubstrateKey.addressRaw), 'Substrate', context);
    const [bobValidationData] = await buildValidations(
        context,
        [aliceSubject],
        [bobSubstrateIdentity],
        nonce.toNumber(),
        'substrate',
        bobSubstrateKey
    );
    const linkIdentityCall = createSignedTrustedCallLinkIdentity(
        parachainApi,
        mrenclave,
        nonce,
        alice,
        aliceSubject,
        sidechainRegistry.createType('LitentryPrimitivesIdentity', bobSubstrateIdentity).toHex(),
        parachainApi.createType('LitentryValidationData', bobValidationData.toU8a()).toHex(),
        parachainApi.createType('Vec<Web3Network>', ['Polkadot', 'Litentry']).toHex(),
        keyNonce,
        hash
    );
    res = await sendRequestFromTrustedCall(wsp, parachainApi, mrenclave, key, linkIdentityCall);
    console.log('linkIdentity call returned', res.toHuman());

    // we should have listened to the parachain event, for demo purpose we only wait for enough
    // time and check the IDGraph
    await sleep(30);

    console.log('Send IDGraph getter for alice ...');
    const idgraphGetter = await createSignedTrustedGetterIdGraph(parachainApi, alice, aliceSubject);
    res = await sendRequestFromGetter(wsp, parachainApi, mrenclave, key, idgraphGetter);
    console.log('IDGraph getter returned', res.toHuman());
    let idgraph = decodeIdGraph(sidechainRegistry, res.value);
    assert.equal(idgraph.length, 2);
    // the first identity is the bob substrate identity
    assert.isTrue(idgraph[0][0].isSubstrate);
    assert.equal(idgraph[0][0].asSubstrate.toHex(), u8aToHex(bobSubstrateKey.publicKey));
    assert.equal(idgraph[0][1].web3networks.toHuman()?.toString(), ['Polkadot', 'Litentry'].toString());
    assert.isTrue(idgraph[0][1].status.isActive);
    // the second identity is the substrate identity (prime identity)
    if (alice.type === "ethereum") {
        assert.isTrue(idgraph[1][0].isEvm);
        assert.equal(idgraph[1][0].asEvm.toHex(), u8aToHex(alice.addressRaw));
        assert.isTrue(idgraph[1][1].status.isActive);
        assert.equal(
            idgraph[1][1].web3networks.toHuman()?.toString(),
            ['Ethereum', 'Polygon', 'BSC'].toString()
        );
    } else {
        assert.isTrue(idgraph[1][0].isSubstrate);
        assert.equal(idgraph[1][0].asSubstrate.toHex(), u8aToHex(alice.addressRaw));
        assert.isTrue(idgraph[1][1].status.isActive);
        assert.equal(
            idgraph[1][1].web3networks.toHuman()?.toString(),
            ['Polkadot', 'Kusama', 'Litentry', 'Litmus', 'LitentryRococo', 'Khala', 'SubstrateTestnet'].toString()
        );
    }


    console.log('Send UserShieldingKey getter for alice ...');
    let userShieldingKeyGetter = await createSignedTrustedGetterUserShieldingKey(parachainApi, alice, aliceSubject);
    res = await sendRequestFromGetter(wsp, parachainApi, mrenclave, key, userShieldingKeyGetter);
    console.log('UserShieldingKey getter returned', res.toHuman());
    // the returned res.value of the trustedGetter is of Option<> type
    // res.value should be `0x018022fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12`
    // TODO: why `createType` must accept an Uint8Array here? The following still prints the unwrapped value
    //       let k = parachainApi.createType('Option<Bytes>', res.value.toHex());
    //       console.log("k.isSome", k.isSome); // true
    //       console.log("k.unwrap", k.unwrap().toHex()); // still 0x018022fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12
    let k = parachainApi.createType('Option<Bytes>', hexToU8a(res.value.toHex()));
    assert.isTrue(k.isSome);
    assert.equal(k.unwrap().toHex(), aesKey);

    // set web3networks to alice
    console.log('Set new web3networks for alice ...');
    nonce = await getSidechainNonce(wsp, parachainApi, mrenclave, key, aliceSubject);
    let setIdentityNetworksCall = createSignedTrustedCallSetIdentityNetworks(
        parachainApi,
        mrenclave,
        nonce,
        alice,
        aliceSubject,
        bobSubstrateIdentity.toHex(),
        parachainApi.createType('Vec<Web3Network>', ['Litentry', 'Khala']).toHex()
    );
    res = await sendRequestFromTrustedCall(wsp, parachainApi, mrenclave, key, setIdentityNetworksCall);
    console.log('setIdentityNetworks call returned', res.toHuman());
    res = await sendRequestFromGetter(wsp, parachainApi, mrenclave, key, idgraphGetter);
    idgraph = decodeIdGraph(sidechainRegistry, res.value);
    assert.equal(idgraph.length, 2);
    assert.equal(idgraph[0][1].web3networks.toHuman()?.toString(), ['Litentry', 'Khala'].toString());

    // set incompatible web3networks to alice
    console.log('Set incompatible web3networks for alice ...');
    nonce = await getSidechainNonce(wsp, parachainApi, mrenclave, key, aliceSubject);
    setIdentityNetworksCall = createSignedTrustedCallSetIdentityNetworks(
        parachainApi,
        mrenclave,
        nonce,
        alice,
        aliceSubject,
        bobSubstrateIdentity.toHex(),
        parachainApi.createType('Vec<Web3Network>', ['BSC', 'Ethereum']).toHex()
    );
    res = await sendRequestFromTrustedCall(wsp, parachainApi, mrenclave, key, setIdentityNetworksCall);
    console.log('setIdentityNetworks call returned', res.toHuman());
    assert.isTrue!(res.status.isTrustedOperationStatus && res.status.asTrustedOperationStatus.isInvalid); // invalid status
    // idgraph should be unchanged
    res = await sendRequestFromGetter(wsp, parachainApi, mrenclave, key, idgraphGetter);
    idgraph = decodeIdGraph(sidechainRegistry, res.value);
    assert.equal(idgraph.length, 2);
    assert.equal(idgraph[0][1].web3networks.toHuman()?.toString(), ['Litentry', 'Khala'].toString());

    // bob's shielding key should be none
    console.log('Send UserShieldingKey getter for bob ...');
    userShieldingKeyGetter = await createSignedTrustedGetterUserShieldingKey(parachainApi, bob, bobSubject);
    res = await sendRequestFromGetter(wsp, parachainApi, mrenclave, key, userShieldingKeyGetter);
    console.log('UserShieldingKey getter returned', res.toHuman());
    k = parachainApi.createType('Option<Bytes>', hexToU8a(res.value.toHex()));
    assert.isTrue(k.isNone);

    await sleep(10);

    nonce = await getSidechainNonce(wsp, parachainApi, mrenclave, key, bobSubject);

    // set bob's shielding key, with wrapped bytes
    const keyBob = '0x8378193a4ce64180814bd60591d1054a04dbc4da02afde453799cd6888ee0c6c';
    hash = `0x${crypto.randomBytes(32).toString('hex')}`;
    console.log('Send direct setUserShieldingKey call for bob, with wrapped bytes... hash:', hash);
    setUserShieldingKeyCall = await createSignedTrustedCallSetUserShieldingKey(
        parachainApi,
        mrenclave,
        nonce,
        bob,
        bobSubject,
        keyBob,
        hash,
        true // with wrapped bytes
    );
    res = await sendRequestFromTrustedCall(wsp, parachainApi, mrenclave, key, setUserShieldingKeyCall);
    console.log('setUserShieldingKey call returned', res.toHuman());

    // verify that bob's key is set
    console.log('Send UserShieldingKey getter for bob ...');
    userShieldingKeyGetter = await createSignedTrustedGetterUserShieldingKey(parachainApi, bob, bobSubject);
    res = await sendRequestFromGetter(wsp, parachainApi, mrenclave, key, userShieldingKeyGetter);
    console.log('UserShieldingKey getter returned', res.toHuman());
    k = parachainApi.createType('Option<Bytes>', hexToU8a(res.value.toHex()));
    assert.isTrue(k.isSome);
    assert.equal(k.unwrap().toHex(), keyBob);
}