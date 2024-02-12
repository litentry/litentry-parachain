import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { u8aToHex, bufferToU8a } from '@polkadot/util';
import { buildIdentityFromKeypair, initIntegrationTestContext, PolkadotSigner } from './common/utils';
import { assertIsInSidechainBlock, assertVc } from './common/utils/assertion';
import {
    getSidechainNonce,
    createSignedTrustedCallLinkIdentity,
    getTeeShieldingKey,
    sendRequestFromTrustedCall,
    createSignedTrustedCallRequestVc,
    sendRequestFromTrustedCallDirect,
} from './common/di-utils'; // @fixme move to a better place
import { buildIdentityHelper, buildValidations } from './common/utils';
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import { CorePrimitivesIdentity } from 'parachain-api';
import { subscribeToEventsWithExtHash } from './common/transactions';
import { defaultAssertions, unconfiguredAssertions } from './common/utils/vc-helper';
import { LitentryValidationData, Web3Network } from 'parachain-api';
import { Vec } from '@polkadot/types';

describe('Test Vc (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubstrateIdentity: CorePrimitivesIdentity = undefined as any;

    // Alice links:
    // - a `mock_user` twitter
    // - alice's evm identity
    // - alice's bitcoin identity]
    //
    // We need this linking to not have empty eligible identities for any vc request
    const linkIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
        validation: LitentryValidationData;
        networks: Vec<Web3Network>;
    }[] = [];
    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_ENDPOINT!, // @fixme evil assertion; centralize env access
            process.env.NODE_ENDPOINT! // @fixme evil assertion; centralize env access
        );
        teeShieldingKey = await getTeeShieldingKey(context);
        aliceSubstrateIdentity = await buildIdentityFromKeypair(
            new PolkadotSigner(context.substrateWallet.alice),
            context
        );
    });

    // step('linking identities (alice)', async function () {
    //     let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubstrateIdentity)).toNumber();
    //     const getNextNonce = () => currentNonce++;

    //     const twitterNonce = getNextNonce();
    //     const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
    //     const [twitterValidation] = await buildValidations(
    //         context,
    //         [aliceSubstrateIdentity],
    //         [twitterIdentity],
    //         twitterNonce,
    //         'twitter'
    //     );
    //     const twitterNetworks = context.api.createType('Vec<Web3Network>', []) as unknown as Vec<Web3Network>; // @fixme #1878
    //     linkIdentityRequestParams.push({
    //         nonce: twitterNonce,
    //         identity: twitterIdentity,
    //         validation: twitterValidation,
    //         networks: twitterNetworks,
    //     });

    //     const evmNonce = getNextNonce();
    //     const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
    //     const [evmValidation] = await buildValidations(
    //         context,
    //         [aliceSubstrateIdentity],
    //         [evmIdentity],
    //         evmNonce,
    //         'ethereum',
    //         undefined,
    //         [context.ethersWallet.alice]
    //     );
    //     const evmNetworks = context.api.createType('Vec<Web3Network>', [
    //         'Ethereum',
    //         'Bsc',
    //     ]) as unknown as Vec<Web3Network>; // @fixme #1878
    //     linkIdentityRequestParams.push({
    //         nonce: evmNonce,
    //         identity: evmIdentity,
    //         validation: evmValidation,
    //         networks: evmNetworks,
    //     });

    //     const bitcoinNonce = getNextNonce();
    //     const bitcoinIdentity = await buildIdentityHelper(
    //         u8aToHex(bufferToU8a(context.bitcoinWallet.alice.toPublicKey().toBuffer())),
    //         'Bitcoin',
    //         context
    //     );
    //     console.log('bitcoin id: ', bitcoinIdentity.toHuman());
    //     const [bitcoinValidation] = await buildValidations(
    //         context,
    //         [aliceSubstrateIdentity],
    //         [bitcoinIdentity],
    //         bitcoinNonce,
    //         'bitcoin',
    //         undefined,
    //         undefined,
    //         context.bitcoinWallet.alice
    //     );
    //     const bitcoinNetworks = context.api.createType('Vec<Web3Network>', [
    //         'BitcoinP2tr',
    //     ]) as unknown as Vec<Web3Network>; // @fixme #1878
    //     linkIdentityRequestParams.push({
    //         nonce: bitcoinNonce,
    //         identity: bitcoinIdentity,
    //         validation: bitcoinValidation,
    //         networks: bitcoinNetworks,
    //     });

    //     for (const { nonce, identity, validation, networks } of linkIdentityRequestParams) {
    //         const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
    //         const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
    //             context.api,
    //             context.mrEnclave,
    //             context.api.createType('Index', nonce),
    //             new PolkadotSigner(context.substrateWallet.alice),
    //             aliceSubstrateIdentity,
    //             identity.toHex(),
    //             validation.toHex(),
    //             networks.toHex(),
    //             context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
    //             requestIdentifier
    //         );

    //         const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);
    //         await assertIsInSidechainBlock('linkIdentityCall', res);
    //     }
    // });

    defaultAssertions.forEach(({ description, assertion }) => {
        step(`request vc direct ${Object.keys(assertion)[0]} (alice)`, async function () {
            let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubstrateIdentity)).toNumber();
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
                aliceSubstrateIdentity,
                context.api.createType('Assertion', assertion).toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCallDirect(context, teeShieldingKey, requestVcCall);
            const events = await eventsPromise;

            const vcIssuedEvents = events
                .map(({ event }) => event)
                .filter(({ section, method }) => section === 'vcManagement' && method === 'VCIssued');
            assert.equal(
                vcIssuedEvents.length,
                1,
                `vcIssuedEvents.length != 1, please check the ${Object.keys(assertion)[0]} call`
            );

            await assertVc(context, aliceSubstrateIdentity, res.value);
        });
    });
    unconfiguredAssertions.forEach(({ description, assertion }) => {
        it(`request vc ${Object.keys(assertion)[0]} (alice)`, async function () {
            let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubstrateIdentity)).toNumber();
            const getNextNonce = () => currentNonce++;
            const nonce = getNextNonce();
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            console.log(`request vc ${Object.keys(assertion)[0]} for Alice ... Assertion description: ${description}`);
            subscribeToEventsWithExtHash(requestIdentifier, context);

            const requestVcCall = await createSignedTrustedCallRequestVc(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new PolkadotSigner(context.substrateWallet.alice),
                aliceSubstrateIdentity,
                context.api.createType('Assertion', assertion).toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            await sendRequestFromTrustedCall(context, teeShieldingKey, requestVcCall);
            // pending test
            this.skip();
        });
    });
});
