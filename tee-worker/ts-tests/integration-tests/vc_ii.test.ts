import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import {
    buildIdentityFromKeypair,
    buildIdentityHelper,
    buildValidations,
    initIntegrationTestContext,
    PolkadotSigner,
} from './common/utils';
import {
    assertFailedEvent,
    assertIdentityLinked,
    assertInitialIdGraphCreated,
    assertIsInSidechainBlock,
} from './common/utils/assertion';
import {
    createSignedTrustedCallLinkIdentity,
    createSignedTrustedCallSetUserShieldingKey,
    createSignedTrustedGetterIdGraph,
    createSignedTrustedGetterUserShieldingKey,
    createSignedTrustedCallDeactivateIdentity,
    createSignedTrustedCallActivateIdentity,
    decodeIdGraph,
    getSidechainNonce,
    getTeeShieldingKey,
    sendRequestFromGetter,
    sendRequestFromTrustedCall,
    createSignedTrustedCallRequestVc,
} from './examples/direct-invocation/util'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/type-definitions';
import { aesKey, keyNonce } from './common/call';
import { LitentryValidationData, Web3Network } from 'parachain-api';
import { LitentryPrimitivesIdentity } from 'sidechain-api';
import { Vec } from '@polkadot/types';
import { ethers } from 'ethers';
import type { HexString } from '@polkadot/util/types';
import { subscribeToEventsWithExtHash } from './common/transactions';

describe('Test Identity (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubject: LitentryPrimitivesIdentity = undefined as any;

    // Alice links:
    // - a `mock_user` twitter
    // - alice's evm identity
    // - eve's substrate identity (as alice can't link her own substrate again)
    const linkIdentityRequestParams: {
        nonce: number;
        identity: LitentryPrimitivesIdentity;
        validation: LitentryValidationData;
        networks: Vec<Web3Network>;
    }[] = [];
    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_ENDPOINT!, // @fixme evil assertion; centralize env access
            process.env.NODE_ENDPOINT!, // @fixme evil assertion; centralize env access
            0
        );
        teeShieldingKey = await getTeeShieldingKey(context.tee, context.api);
        aliceSubject = await buildIdentityFromKeypair(new PolkadotSigner(context.substrateWallet.alice), context);
    });

    it('needs a lot more work to be complete');
    it('most of the bob cases are missing');
    // step(`setting user shielding key (alice)`, async function () {
    //     const wallet = context.substrateWallet['alice'];
    //     const subject = await buildIdentityFromKeypair(new PolkadotSigner(wallet), context);
    //     const nonce = await getSidechainNonce(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         subject
    //     );

    //     const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;

    //     const setUserShieldingKeyCall = await createSignedTrustedCallSetUserShieldingKey(
    //         context.api,
    //         context.mrEnclave,
    //         nonce,
    //         new PolkadotSigner(wallet),
    //         subject,
    //         aesKey,
    //         requestIdentifier
    //     );

    //     const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
    //     const res = await sendRequestFromTrustedCall(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         setUserShieldingKeyCall
    //     );
    //     await assertIsInSidechainBlock('setUserShieldingKeyCall', res);

    //     const events = await eventsPromise;
    //     const userShieldingKeySetEvents = events
    //         .map(({ event }) => event)
    //         .filter(({ section, method }) => section === 'identityManagement' && method === 'UserShieldingKeySet');

    //     // await assertInitialIdGraphCreated(context, new PolkadotSigner(wallet), userShieldingKeySetEvents);
    // });

    // step(`request vc A1 (alice)`, async function () {
    //     const nonce = await getSidechainNonce(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         aliceSubject
    //     );
    //     const hash = `0x${randomBytes(32).toString('hex')}`;
    //     console.log('request vc for alice ...');
    //     const requestVcCall = await createSignedTrustedCallRequestVc(
    //         context.api,
    //         context.mrEnclave,
    //         nonce,
    //         new PolkadotSigner(context.substrateWallet.alice),
    //         aliceSubject,
    //         context.api.createType('Assertion', { A1: null }).toHex(),
    //         hash
    //     );
    //     const res = await sendRequestFromTrustedCall(context.tee, context.api, context.mrEnclave, teeShieldingKey, requestVcCall);
    //     const requestVcRes = context.api.createType('RequestVCResponse', res.value);

    //     console.log(requestVcRes);

    // });

    // step(`request vc A2  (alice)`, async function () {
    //     const nonce = await getSidechainNonce(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         aliceSubject
    //     );
    //     const hash = `0x${randomBytes(32).toString('hex')}`;
    //     console.log('request vc for alice ...');
    //     const requestVcCall = await createSignedTrustedCallRequestVc(
    //         context.api,
    //         context.mrEnclave,
    //         nonce,
    //         new PolkadotSigner(context.substrateWallet.alice),
    //         aliceSubject,
    //         context.api.createType('Assertion', { A2: ['A2'] }).toHex(),
    //         hash
    //     );
    //     const res = await sendRequestFromTrustedCall(context.tee, context.api, context.mrEnclave, teeShieldingKey, requestVcCall);

    //     const requestVcRes = context.api.createType('RequestVCResponse', res.value);

    //     console.log(requestVcRes);

    // });

    // step(`request vc A3  (alice)`, async function () {
    //     const nonce = await getSidechainNonce(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         aliceSubject
    //     );
    //     const hash = `0x${randomBytes(32).toString('hex')}`;
    //     console.log('request vc for alice ...');
    //     const requestVcCall = await createSignedTrustedCallRequestVc(
    //         context.api,
    //         context.mrEnclave,
    //         nonce,
    //         new PolkadotSigner(context.substrateWallet.alice),
    //         aliceSubject,
    //         context.api.createType('Assertion', { A2: ['A3', 'A3', 'A3'] }).toHex(),
    //         hash
    //     );
    //     const res = await sendRequestFromTrustedCall(context.tee, context.api, context.mrEnclave, teeShieldingKey, requestVcCall);

    //     const requestVcRes = context.api.createType('RequestVCResponse', res.value);

    //     console.log(requestVcRes);

    // });

    // step(`request vc A4  (alice)`, async function () {
    //     const nonce = await getSidechainNonce(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         aliceSubject
    //     );
    //     const hash = `0x${randomBytes(32).toString('hex')}`;
    //     console.log('request vc for alice ...');
    //     const requestVcCall = await createSignedTrustedCallRequestVc(
    //         context.api,
    //         context.mrEnclave,
    //         nonce,
    //         new PolkadotSigner(context.substrateWallet.alice),
    //         aliceSubject,
    //         context.api.createType('Assertion', { A4: '10' }).toHex(),
    //         hash
    //     );
    //     const res = await sendRequestFromTrustedCall(context.tee, context.api, context.mrEnclave, teeShieldingKey, requestVcCall);

    //     const requestVcRes = context.api.createType('RequestVCResponse', res.value);

    //     console.log(requestVcRes);

    // });

    // step(`request vc A6  (alice)`, async function () {
    //     const nonce = await getSidechainNonce(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         aliceSubject
    //     );
    //     const hash = `0x${randomBytes(32).toString('hex')}`;
    //     console.log('request vc for alice ...');
    //     const requestVcCall = await createSignedTrustedCallRequestVc(
    //         context.api,
    //         context.mrEnclave,
    //         nonce,
    //         new PolkadotSigner(context.substrateWallet.alice),
    //         aliceSubject,
    //         context.api.createType('Assertion', { A6: null }).toHex(),
    //         hash
    //     );
    //     const res = await sendRequestFromTrustedCall(context.tee, context.api, context.mrEnclave, teeShieldingKey, requestVcCall);

    //     const requestVcRes = context.api.createType('RequestVCResponse', res.value);

    //     console.log(requestVcRes);

    // });
    step(`request vc A7  (alice)`, async function () {
        const nonce = await getSidechainNonce(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            aliceSubject
        );
        const hash = `0x${randomBytes(32).toString('hex')}`;
        console.log('request vc for alice ...');
        const requestVcCall = await createSignedTrustedCallRequestVc(
            context.api,
            context.mrEnclave,
            nonce,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubject,
            context.api.createType('Assertion', { A7: '5' }).toHex(),
            hash
        );
        const res = await sendRequestFromTrustedCall(context.tee, context.api, context.mrEnclave, teeShieldingKey, requestVcCall);

        const requestVcRes = context.api.createType('RequestVCResponse', res.value);

        console.log(requestVcRes);

    });

    // step(`request vc A8 (alice)`, async function () {
    //     const nonce = await getSidechainNonce(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         aliceSubject
    //     );
    //     const hash = `0x${randomBytes(32).toString('hex')}`;
    //     console.log('request vc for alice ...');
    //     const requestVcCall = await createSignedTrustedCallRequestVc(
    //         context.api,
    //         context.mrEnclave,
    //         nonce,
    //         new PolkadotSigner(context.substrateWallet.alice),
    //         aliceSubject,
    //         context.api.createType('Assertion', { A8: ["Litentry"] }).toHex(),
    //         hash
    //     );
    //     const res = await sendRequestFromTrustedCall(context.tee, context.api, context.mrEnclave, teeShieldingKey, requestVcCall);

    //     const requestVcRes = context.api.createType('RequestVCResponse', res.value);

    //     console.log(requestVcRes);

    // });

    // step(`request vc A10  (alice)`, async function () {
    //     const nonce = await getSidechainNonce(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         aliceSubject
    //     );
    //     const hash = `0x${randomBytes(32).toString('hex')}`;
    //     console.log('request vc for alice ...');
    //     const requestVcCall = await createSignedTrustedCallRequestVc(
    //         context.api,
    //         context.mrEnclave,
    //         nonce,
    //         new PolkadotSigner(context.substrateWallet.alice),
    //         aliceSubject,
    //         context.api.createType('Assertion', { A10: '10' }).toHex(),
    //         hash
    //     );
    //     const res = await sendRequestFromTrustedCall(context.tee, context.api, context.mrEnclave, teeShieldingKey, requestVcCall);

    //     const requestVcRes = context.api.createType('RequestVCResponse', res.value);

    //     console.log(requestVcRes);

    // });
    // step(`request vc A11  (alice)`, async function () {
    //     const nonce = await getSidechainNonce(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         aliceSubject
    //     );
    //     const hash = `0x${randomBytes(32).toString('hex')}`;
    //     console.log('request vc for alice ...');
    //     const requestVcCall = await createSignedTrustedCallRequestVc(
    //         context.api,
    //         context.mrEnclave,
    //         nonce,
    //         new PolkadotSigner(context.substrateWallet.alice),
    //         aliceSubject,
    //         context.api.createType('Assertion', { A11: '10' }).toHex(),
    //         hash
    //     );
    //     const res = await sendRequestFromTrustedCall(context.tee, context.api, context.mrEnclave, teeShieldingKey, requestVcCall);

    //     const requestVcRes = context.api.createType('RequestVCResponse', res.value);

    //     console.log(requestVcRes);

    // });
});
