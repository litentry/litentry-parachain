import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { buildIdentityFromKeypair, initIntegrationTestContext, PolkadotSigner } from './common/utils';
import { assertIsInSidechainBlock, assertVc, assertWorkerError } from './common/utils/assertion';
import {
    getSidechainNonce,
    getTeeShieldingKey,
    sendRequestFromTrustedCall,
    createSignedTrustedCallRequestVc,
} from './common/di-utils'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import { LitentryPrimitivesIdentity } from 'sidechain-api';
import { subscribeToEventsWithExtHash } from './common/transactions';
import { defaultAssertions, unconfiguredAssertions } from './common/utils/vc-helper';

describe('Test Vc (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubject: LitentryPrimitivesIdentity = undefined as any;

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

    defaultAssertions.forEach(({ description, assertion }) => {
        step(`request vc ${Object.keys(assertion)[0]} (alice)`, async function () {
            let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubject)).toNumber();
            const getNextNonce = () => currentNonce++;
            const nonce = getNextNonce();
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            console.log(`request vc ${Object.keys(assertion)[0]} for Alice ... Assertion description: ${description}`);
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);

            const requestVcCall = await createSignedTrustedCallRequestVc(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new PolkadotSigner(context.substrateWallet.alice),
                aliceSubject,
                context.api.createType('Assertion', assertion).toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
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
    unconfiguredAssertions.forEach(({ description, assertion }) => {
        it(`request vc ${Object.keys(assertion)[0]} (alice)`, async function () {
            let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubject)).toNumber();
            const getNextNonce = () => currentNonce++;
            const nonce = getNextNonce();
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            console.log(`request vc ${Object.keys(assertion)[0]} for Alice ... Assertion description: ${description}`);
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);

            const requestVcCall = await createSignedTrustedCallRequestVc(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new PolkadotSigner(context.substrateWallet.alice),
                aliceSubject,
                context.api.createType('Assertion', assertion).toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, requestVcCall);
            // pending test
            this.skip();
        });
    });
});
