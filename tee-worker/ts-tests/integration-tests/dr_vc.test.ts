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
    createSignedTrustedCallRequestBatchVc,
} from './common/di-utils'; // @fixme move to a better place
import { buildIdentityHelper, buildValidations } from './common/utils';
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import { CorePrimitivesIdentity, WorkerRpcReturnValue } from 'parachain-api';
import { subscribeToEventsWithExtHash } from './common/transactions';
import { mockAssertions } from './common/utils/vc-helper';
import { LitentryValidationData, Web3Network } from 'parachain-api';
import { Vec, Bytes } from '@polkadot/types';

describe('Test Vc (direct request)', function () {
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
        networks: Bytes | Vec<Web3Network>;
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

    step('linking identities (alice)', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const twitterNonce = getNextNonce();
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const [twitterValidation] = await buildValidations(
            context,
            [aliceSubstrateIdentity],
            [twitterIdentity],
            twitterNonce,
            'twitter'
        );
        const twitterNetworks = context.api.createType('Vec<Web3Network>', []);
        linkIdentityRequestParams.push({
            nonce: twitterNonce,
            identity: twitterIdentity,
            validation: twitterValidation,
            networks: twitterNetworks,
        });

        const evmNonce = getNextNonce();
        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
        const [evmValidation] = await buildValidations(
            context,
            [aliceSubstrateIdentity],
            [evmIdentity],
            evmNonce,
            'ethereum',
            undefined,
            [context.ethersWallet.alice]
        );
        const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);
        linkIdentityRequestParams.push({
            nonce: evmNonce,
            identity: evmIdentity,
            validation: evmValidation,
            networks: evmNetworks,
        });

        const bitcoinNonce = getNextNonce();
        const bitcoinIdentity = await buildIdentityHelper(
            u8aToHex(bufferToU8a(context.bitcoinWallet.alice.toPublicKey().toBuffer())),
            'Bitcoin',
            context
        );
        console.log('bitcoin id: ', bitcoinIdentity.toHuman());
        const [bitcoinValidation] = await buildValidations(
            context,
            [aliceSubstrateIdentity],
            [bitcoinIdentity],
            bitcoinNonce,
            'bitcoin',
            undefined,
            undefined,
            context.bitcoinWallet.alice
        );
        const bitcoinNetworks = context.api.createType('Vec<Web3Network>', ['BitcoinP2tr']);
        linkIdentityRequestParams.push({
            nonce: bitcoinNonce,
            identity: bitcoinIdentity,
            validation: bitcoinValidation,
            networks: bitcoinNetworks,
        });

        for (const { nonce, identity, validation, networks } of linkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new PolkadotSigner(context.substrateWallet.alice),
                aliceSubstrateIdentity,
                identity.toHex(),
                validation.toHex(),
                networks.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);
            await assertIsInSidechainBlock('linkIdentityCall', res);
        }
    });

    mockAssertions.forEach(({ description, assertion }) => {
        step(`request vc payload: ${JSON.stringify(assertion)} (alice)`, async function () {
            let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubstrateIdentity)).toNumber();
            const getNextNonce = () => currentNonce++;
            const nonce = getNextNonce();
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            console.log(
                `request vc direct ${Object.keys(assertion)[0]} for Alice ... Assertion description: ${description}`
            );
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);

            let requestVcCall;
            if (Array.isArray(assertion)) {
                requestVcCall = await createSignedTrustedCallRequestBatchVc(
                    context.api,
                    context.mrEnclave,
                    context.api.createType('Index', nonce),
                    new PolkadotSigner(context.substrateWallet.alice),
                    aliceSubstrateIdentity,
                    context.api.createType('Vec<Assertion>', assertion).toHex(),
                    context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                    requestIdentifier
                );
            } else {
                requestVcCall = await createSignedTrustedCallRequestVc(
                    context.api,
                    context.mrEnclave,
                    context.api.createType('Index', nonce),
                    new PolkadotSigner(context.substrateWallet.alice),
                    aliceSubstrateIdentity,
                    context.api.createType('Assertion', assertion).toHex(),
                    context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                    requestIdentifier
                );
            }

            const isVcDirect = true;
            // Instead of waiting for final response we will listen all responses from the call
            const onMessageReceived = async (res: WorkerRpcReturnValue) => {
                // if response is a A1 or A2, etc....
                const vcresponse = context.api.createType('RequestVcResultOrError', res.value);
                console.log(`vcresponse len: ${vcresponse.len}, idx: ${vcresponse.idx}`);
                if (!vcresponse.is_error) await assertVc(context, aliceSubstrateIdentity, vcresponse.payload);
            };

            // the +res+ below is the last message with "do_watch: false" property and we may not need it at all
            const res = await sendRequestFromTrustedCall(
                context,
                teeShieldingKey,
                requestVcCall,
                isVcDirect,
                onMessageReceived
            );
            const events = await eventsPromise;
            const vcIssuedEvents = events
                .map(({ event }) => event)
                .filter(({ section, method }) => section === 'vcManagement' && method === 'VCIssued');

            assert.equal(
                vcIssuedEvents.length,
                Array.isArray(assertion) ? assertion.length : 1,
                `vcIssuedEvents.length != 1, please check the ${Object.keys(assertion)[0]} call`
            );
        });
    });
});
