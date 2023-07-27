import { cryptoWaitReady } from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';
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
    createSignedTrustedCallRequestVc,
    sendRequestFromGetter,
    getSidechainNonce,
    decodeIdGraph,
    getKeyPair,
    getTopHash,
    parseAesOutput,
} from './util';
import {
    getEnclave,
    sleep,
    buildIdentityHelper,
    initIntegrationTestContext,
    buildValidations,
    buildIdentityFromKeypair,
    parseIdGraph,
} from '../../common/utils';
import { aesKey, keyNonce } from '../../common/call';
import { Metadata, TypeRegistry } from '@polkadot/types';
import sidechainMetaData from '../../../sidechain-api/prepare-build/litentry-sidechain-metadata.json' assert { type: 'json' };
import { hexToU8a, u8aToHex } from '@polkadot/util';
import type { LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext } from 'sidechain-api';
import { assert } from 'chai';
import Options from 'websocket-as-promised/types/options';
import crypto from 'crypto';
import { KeypairType } from '@polkadot/util-crypto/types';
import WebSocketAsPromised from 'websocket-as-promised';
import webSocket from 'ws';
import { decryptWithAes } from '../../common/utils';
import { SetUserShieldingKeyResponse, LinkIdentityResponse, RequestVCResponse } from 'parachain-api/build/interfaces';

// in order to handle self-signed certificates we need to turn off the validation
// TODO add self signed certificate
process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

const substrateKeyring = new Keyring({ type: 'sr25519' });
const PARACHAIN_WS_ENDPINT = 'ws://localhost:9944';
const WORKER_TRUSTED_WS_ENDPOINT = 'wss://localhost:2000';

export async function runExample(keyPairType: KeypairType) {
    const keyring = new Keyring({ type: keyPairType });
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

    const alice: KeyringPair = getKeyPair('Alice', keyring);
    const bob: KeyringPair = getKeyPair('Bob', keyring);
    const bobSubstrateKey: KeyringPair = substrateKeyring.addFromUri('//Bob', { name: 'Bob' });

    const mrenclave = (await getEnclave(parachainApi)).mrEnclave;

    const aliceSubject = await buildIdentityFromKeypair(alice, context);
    const bobSubject = await buildIdentityFromKeypair(bob, context);

    // ==============================================================================
    // 1. Test set_user_shielding_key
    // ==============================================================================

    // similar to `reqExtHash` in indirect calls, we need some "identifiers" to pair the response
    // with the request. Ideally it's the hash of the trusted operation, but we need it before constructing
    // a trusted call, hence a random number is used here - better ideas are welcome
    let hash = `0x${crypto.randomBytes(32).toString('hex')}`;
    let nonce = await getSidechainNonce(wsp, parachainApi, mrenclave, key, aliceSubject);

    console.log('Send direct setUserShieldingKey call for alice ... hash:', hash);
    let setUserShieldingKeyCall = createSignedTrustedCallSetUserShieldingKey(
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
    assert.isTrue(res.do_watch.isFalse);
    assert.isTrue(res.status.asTrustedOperationStatus[0].isInSidechainBlock);
    assert.equal(u8aToHex(res.status.asTrustedOperationStatus[1]), getTopHash(parachainApi, setUserShieldingKeyCall));

    const setUserShieldingKeyRes = parachainApi.createType(
        'SetUserShieldingKeyResponse',
        res.value
    ) as unknown as SetUserShieldingKeyResponse;
    assert.equal(setUserShieldingKeyRes.account.toHex(), u8aToHex(alice.addressRaw));
    assert.equal(setUserShieldingKeyRes.req_ext_hash.toHex(), hash);
    let aesOutput = parseAesOutput(parachainApi, setUserShieldingKeyRes.id_graph.toHex());
    let idgraph = parseIdGraph(sidechainRegistry, aesOutput, aesKey);
    assert.equal(idgraph.length, 1);
    assertPrimeIdentity(idgraph[0], alice);

    await sleep(10);

    // ==============================================================================
    // 2. Test link_identity (happy path)
    // ==============================================================================

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
    let linkIdentityCall = createSignedTrustedCallLinkIdentity(
        parachainApi,
        mrenclave,
        nonce,
        alice,
        aliceSubject,
        sidechainRegistry.createType('LitentryPrimitivesIdentity', bobSubstrateIdentity).toHex(),
        parachainApi.createType('LitentryValidationData', bobValidationData).toHex(),
        parachainApi.createType('Vec<Web3Network>', ['Polkadot', 'Litentry']).toHex(),
        keyNonce,
        hash
    );
    res = await sendRequestFromTrustedCall(wsp, parachainApi, mrenclave, key, linkIdentityCall);
    console.log('linkIdentity call returned', res.toHuman());
    assert.isTrue(res.do_watch.isFalse);
    assert.isTrue(res.status.asTrustedOperationStatus[0].isInSidechainBlock);
    const linkIdentityRes = parachainApi.createType('LinkIdentityResponse', res.value) as unknown as LinkIdentityResponse;
    assert.equal(linkIdentityRes.account.toHex(), u8aToHex(alice.addressRaw));
    assert.equal(linkIdentityRes.req_ext_hash.toHex(), hash);
    aesOutput = parseAesOutput(parachainApi, linkIdentityRes.id_graph.toHex());
    idgraph = parseIdGraph(sidechainRegistry, aesOutput, aesKey);
    assert.equal(idgraph.length, 2);
    // the first identity is the bob substrate identity
    assertLinkedIdentity(idgraph[0], u8aToHex(bob.addressRaw));
    // the second identity is the substrate identity (prime identity)
    assertPrimeIdentity(idgraph[1], alice);

    // we should have listened to the parachain event, for demo purpose we only wait for enough
    // time and check the IDGraph
    await sleep(30);

    // ==============================================================================
    // 3. Test id_graph getter
    // ==============================================================================

    console.log('Send IDGraph getter for alice ...');
    const idgraphGetter = createSignedTrustedGetterIdGraph(parachainApi, alice, aliceSubject);
    res = await sendRequestFromGetter(wsp, parachainApi, mrenclave, key, idgraphGetter);
    console.log('IDGraph getter returned', res.toHuman());
    idgraph = decodeIdGraph(sidechainRegistry, res.value);
    assert.equal(idgraph.length, 2);
    // the first identity is the bob substrate identity
    assertLinkedIdentity(idgraph[0], u8aToHex(bob.addressRaw));
    // the second identity is the substrate identity (prime identity)
    assertPrimeIdentity(idgraph[1], alice);

    // ==============================================================================
    // 4. Test user_shielding_key getter
    // ==============================================================================

    console.log('Send UserShieldingKey getter for alice ...');
    let userShieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(parachainApi, alice, aliceSubject);
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

    // ==============================================================================
    // 5. Test link_identity (error case: IdentityAlreadyLinked)
    // ==============================================================================
    hash = `0x${crypto.randomBytes(32).toString('hex')}`;
    nonce = await getSidechainNonce(wsp, parachainApi, mrenclave, key, aliceSubject);
    console.log('Send direct linkIdentity call (error case)... hash:', hash);
    linkIdentityCall = createSignedTrustedCallLinkIdentity(
        parachainApi,
        mrenclave,
        nonce,
        alice,
        aliceSubject,
        sidechainRegistry.createType('LitentryPrimitivesIdentity', bobSubstrateIdentity).toHex(),
        parachainApi.createType('LitentryValidationData', bobValidationData).toHex(),
        parachainApi.createType('Vec<Web3Network>', ['Polkadot', 'Litentry']).toHex(),
        keyNonce,
        hash
    );
    res = await sendRequestFromTrustedCall(wsp, parachainApi, mrenclave, key, linkIdentityCall);
    console.log('linkIdentity call returned', res.toHuman());
    assert.isTrue(res.do_watch.isFalse);
    assert.isTrue(res.status.asTrustedOperationStatus[0].isInvalid);

    // ==============================================================================
    // 6. Test set_identity_networks (happy path)
    // ==============================================================================

    // set web3networks to alice
    hash = `0x${crypto.randomBytes(32).toString('hex')}`;
    nonce = await getSidechainNonce(wsp, parachainApi, mrenclave, key, aliceSubject);
    console.log('Set new web3networks for alice ...');
    let setIdentityNetworksCall = createSignedTrustedCallSetIdentityNetworks(
        parachainApi,
        mrenclave,
        nonce,
        alice,
        aliceSubject,
        bobSubstrateIdentity.toHex(),
        parachainApi.createType('Vec<Web3Network>', ['Litentry', 'Khala']).toHex(),
        hash
    );
    res = await sendRequestFromTrustedCall(wsp, parachainApi, mrenclave, key, setIdentityNetworksCall);
    console.log('setIdentityNetworks call returned', res.toHuman());
    res = await sendRequestFromGetter(wsp, parachainApi, mrenclave, key, idgraphGetter);
    idgraph = decodeIdGraph(sidechainRegistry, res.value);
    assert.equal(idgraph.length, 2);
    assert.equal(idgraph[0][1].web3networks.toHuman()?.toString(), ['Litentry', 'Khala'].toString());

    // ==============================================================================
    // 7. Test set_identity_networks (error case)
    // ==============================================================================

    // set incompatible web3networks to alice
    hash = `0x${crypto.randomBytes(32).toString('hex')}`;
    nonce = await getSidechainNonce(wsp, parachainApi, mrenclave, key, aliceSubject);
    console.log('Set incompatible web3networks for alice ...');
    setIdentityNetworksCall = createSignedTrustedCallSetIdentityNetworks(
        parachainApi,
        mrenclave,
        nonce,
        alice,
        aliceSubject,
        bobSubstrateIdentity.toHex(),
        parachainApi.createType('Vec<Web3Network>', ['BSC', 'Ethereum']).toHex(),
        hash
    );
    res = await sendRequestFromTrustedCall(wsp, parachainApi, mrenclave, key, setIdentityNetworksCall);
    console.log('setIdentityNetworks call returned', res.toHuman());
    assert.isTrue!(res.status.isTrustedOperationStatus && res.status.asTrustedOperationStatus[0].isInvalid); // invalid status
    // idgraph should be unchanged
    res = await sendRequestFromGetter(wsp, parachainApi, mrenclave, key, idgraphGetter);
    idgraph = decodeIdGraph(sidechainRegistry, res.value);
    assert.equal(idgraph.length, 2);
    assert.equal(idgraph[0][1].web3networks.toHuman()?.toString(), ['Litentry', 'Khala'].toString());

    // ==============================================================================
    // 8. Test set_user_shielding_key with wrapped bytes
    // ==============================================================================

    // bob's shielding key should be none
    console.log('Send UserShieldingKey getter for bob ...');
    userShieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(parachainApi, bob, bobSubject);
    res = await sendRequestFromGetter(wsp, parachainApi, mrenclave, key, userShieldingKeyGetter);
    console.log('UserShieldingKey getter returned', res.toHuman());
    k = parachainApi.createType('Option<Bytes>', hexToU8a(res.value.toHex()));
    assert.isTrue(k.isNone);

    await sleep(10);

    // set bob's shielding key, with wrapped bytes
    const keyBob = '0x8378193a4ce64180814bd60591d1054a04dbc4da02afde453799cd6888ee0c6c';
    hash = `0x${crypto.randomBytes(32).toString('hex')}`;
    nonce = await getSidechainNonce(wsp, parachainApi, mrenclave, key, bobSubject);
    console.log('Send direct setUserShieldingKey call for bob, with wrapped bytes... hash:', hash);
    setUserShieldingKeyCall = createSignedTrustedCallSetUserShieldingKey(
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
    userShieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(parachainApi, bob, bobSubject);
    res = await sendRequestFromGetter(wsp, parachainApi, mrenclave, key, userShieldingKeyGetter);
    console.log('UserShieldingKey getter returned', res.toHuman());
    k = parachainApi.createType('Option<Bytes>', hexToU8a(res.value.toHex()));
    assert.isTrue(k.isSome);
    assert.equal(k.unwrap().toHex(), keyBob);

    // ==============================================================================
    // 9. Test request_vc
    // ==============================================================================

    hash = `0x${crypto.randomBytes(32).toString('hex')}`;
    nonce = await getSidechainNonce(wsp, parachainApi, mrenclave, key, aliceSubject);
    console.log('request vc for alice ...');
    const requestVcCall = createSignedTrustedCallRequestVc(
        parachainApi,
        mrenclave,
        nonce,
        alice,
        aliceSubject,
        parachainApi.createType('Assertion', { A1: null }).toHex(),
        hash
    );
    res = await sendRequestFromTrustedCall(wsp, parachainApi, mrenclave, key, requestVcCall);
    console.log('requestVcCall call returned', res.toHuman());
    assert.isTrue(res.do_watch.isFalse);
    assert.isTrue(res.status.asTrustedOperationStatus[0].isInSidechainBlock);
    const requestVcRes = parachainApi.createType('RequestVCResponse', res.value) as unknown as RequestVCResponse;
    assert.equal(requestVcRes.account.toHex(), u8aToHex(alice.addressRaw));
    assert.equal(requestVcRes.req_ext_hash.toHex(), hash);
    aesOutput = parseAesOutput(parachainApi, requestVcRes.vc_payload.toHex());
    const decryptedVcPayload = u8aToString(hexToU8a(decryptWithAes(aesKey, aesOutput, 'hex')));
    console.log('decrypted vc payload:', decryptedVcPayload);
}

function assertPrimeIdentity(
    idgraph: [LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext],
    alice: KeyringPair
) {
    if (alice.type === 'ethereum') {
        assert.isTrue(idgraph[0].isEvm);
        assert.equal(idgraph[0].asEvm.toHex(), u8aToHex(alice.addressRaw));
        assert.isTrue(idgraph[1].status.isActive);
        assert.equal(idgraph[1].web3networks.toHuman()?.toString(), ['Ethereum', 'Polygon', 'BSC'].toString());
    } else {
        assert.isTrue(idgraph[0].isSubstrate);
        assert.equal(idgraph[0].asSubstrate.toHex(), u8aToHex(alice.addressRaw));
        assert.isTrue(idgraph[1].status.isActive);
        assert.equal(
            idgraph[1].web3networks.toHuman()?.toString(),
            ['Polkadot', 'Kusama', 'Litentry', 'Litmus', 'LitentryRococo', 'Khala', 'SubstrateTestnet'].toString()
        );
    }
}

function assertLinkedIdentity(
    idgraph: [LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext],
    address: HexString
) {
    assert.isTrue(idgraph[0].isSubstrate);
    assert.equal(idgraph[0].asSubstrate.toHex(), address);
    assert.equal(idgraph[1].web3networks.toHuman()?.toString(), ['Polkadot', 'Litentry'].toString());
    assert.isTrue(idgraph[1].status.isActive);
}
