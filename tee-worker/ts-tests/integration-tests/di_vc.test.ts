import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import {
    assertSetUserShieldingKeyResult,
    assertWorkerError,
    buildIdentityFromKeypair,
    initIntegrationTestContext,
    PolkadotSigner,
} from './common/utils';
import { assertInitialIdGraphCreated, assertIsInSidechainBlock, assertVc } from './common/utils/assertion';
import {
    createSignedTrustedCallSetUserShieldingKey,
    getSidechainNonce,
    getTeeShieldingKey,
    sendRequestFromTrustedCall,
    createSignedTrustedCallRequestVc,
} from './examples/direct-invocation/util'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/type-definitions';
import { aesKey } from './common/call';
import { LitentryPrimitivesIdentity } from 'sidechain-api';
import { subscribeToEventsWithExtHash } from './common/transactions';
describe('Test Vc (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubject: LitentryPrimitivesIdentity = undefined as any;

    // https://github.com/litentry/litentry-parachain/tree/dev/tee-worker/litentry/core/assertion-build/src
    const assertions = [
        {
            A1: null,
        },
        {
            A2: 'A2',
        },
        {
            A3: ['A3', 'A3', 'A3'],
        },
        {
            A4: '10',
        },
        { A6: null },
        { A7: '10.01' },
        { A8: ['Litentry'] },
        { A10: '10' },
        { A11: '10' },

        //TODO add Achainable https://github.com/litentry/litentry-parachain/issues/2080
    ];
    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_ENDPOINT!, // @fixme evil assertion; centralize env access
            process.env.NODE_ENDPOINT!, // @fixme evil assertion; centralize env access
            0
        );
        teeShieldingKey = await getTeeShieldingKey(context);
        aliceSubject = await buildIdentityFromKeypair(new PolkadotSigner(context.substrateWallet.alice), context);
    });

    step(`setting user shielding key (alice)`, async function () {
        const wallet = context.substrateWallet['alice'];
        const subject = await buildIdentityFromKeypair(new PolkadotSigner(wallet), context);
        const nonce = await getSidechainNonce(context, teeShieldingKey, subject);

        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;

        const setUserShieldingKeyCall = await createSignedTrustedCallSetUserShieldingKey(
            context.api,
            context.mrEnclave,
            nonce,
            new PolkadotSigner(wallet),
            subject,
            aesKey,
            requestIdentifier
        );

        const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, setUserShieldingKeyCall);

        assertSetUserShieldingKeyResult(context, res, subject);
        await assertIsInSidechainBlock('setUserShieldingKeyCall', res);

        const events = await eventsPromise;
        const userShieldingKeySetEvents = events
            .map(({ event }) => event)
            .filter(({ section, method }) => section === 'identityManagement' && method === 'UserShieldingKeySet');

        await assertInitialIdGraphCreated(context, new PolkadotSigner(wallet), userShieldingKeySetEvents);
    });

    assertions.forEach((assertion) => {
        step(`request vc ${Object.keys(assertion)[0]} (alice)`, async function () {
            let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubject)).toNumber();
            const getNextNonce = () => currentNonce++;
            const nonce = getNextNonce();
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            console.log(`request vc ${Object.keys(assertion)[0]} for alice ...`);
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);

            const requestVcCall = await createSignedTrustedCallRequestVc(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new PolkadotSigner(context.substrateWallet.alice),
                aliceSubject,
                context.api.createType('Assertion', assertion).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, requestVcCall);

            await assertIsInSidechainBlock(`${Object.keys(assertion)[0]} requestVcCall`, res);
            const events = await eventsPromise;
            const vcIssuedEvents = events
                .map(({ event }) => event)
                .filter(({ section, method }) => section === 'vcManagement' && method === 'VCIssued');

            assert.equal(
                vcIssuedEvents.length,
                1,
                `vcIssuedEvents.length != 1, please check the ${Object.keys(assertion)[0]} call`
            );
            await assertVc(context, aliceSubject, res.value);
        });
    });

    step('request vc without shielding key (bob)', async function () {
        const bobSubject = await buildIdentityFromKeypair(new PolkadotSigner(context.substrateWallet.bob), context);
        const nonce = await getSidechainNonce(context, teeShieldingKey, bobSubject);
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const requestVcCall = await createSignedTrustedCallRequestVc(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', nonce),
            new PolkadotSigner(context.substrateWallet.bob),
            bobSubject,
            context.api.createType('Assertion', { A1: null }).toHex(),
            requestIdentifier
        );
        const callValue = await sendRequestFromTrustedCall(context, teeShieldingKey, requestVcCall);
        assertWorkerError(
            context,
            (v) => {
                assert.isTrue(v.isRequestVCFailed, `expected RequestVCFailed, received ${v.type} instead`);
                assert.isTrue(v.asRequestVCFailed[0].isA1);
                assert.isTrue(v.asRequestVCFailed[1].isUserShieldingKeyNotFound);
            },
            callValue
        );
        assert.isTrue(callValue.do_watch.isFalse);
        assert.isTrue(
            callValue.status.asTrustedOperationStatus[0].isInvalid,
            'request vc without shieldingkey should be invalid'
        );
    });
});
