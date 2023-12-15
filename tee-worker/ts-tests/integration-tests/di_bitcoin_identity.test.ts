import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { u8aToHex, bufferToU8a } from '@polkadot/util';
import {
    buildIdentityFromKeypair,
    buildIdentityHelper,
    buildValidations,
    initIntegrationTestContext,
    EthersSigner,
    BitcoinSigner,
    assertIdGraphMutationResult,
    assertIdGraphHash,
} from './common/utils';
import { assertIsInSidechainBlock, assertIdGraphMutation } from './common/utils/assertion';
import {
    createSignedTrustedCallLinkIdentity,
    createSignedTrustedGetterIdGraph,
    createSignedTrustedCallDeactivateIdentity,
    createSignedTrustedCallActivateIdentity,
    decodeIdGraph,
    getSidechainNonce,
    getTeeShieldingKey,
    sendRequestFromGetter,
    sendRequestFromTrustedCall,
} from './common/di-utils'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import { LitentryValidationData, Web3Network } from 'parachain-api';
import { LitentryPrimitivesIdentity } from 'sidechain-api';
import { Vec } from '@polkadot/types';
import { subscribeToEventsWithExtHash } from './common/transactions';

describe('Test Identity (bitcoin direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceBitcoinIdentity: LitentryPrimitivesIdentity = undefined as any;
    let aliceEvmIdentity: LitentryPrimitivesIdentity;

    // Alice links:
    // - alice's evm identity
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
            process.env.NODE_ENDPOINT! // @fixme evil assertion; centralize env access
        );
        teeShieldingKey = await getTeeShieldingKey(context);
        aliceBitcoinIdentity = await buildIdentityHelper(
            u8aToHex(bufferToU8a(context.bitcoinWallet.alice.publicKey)),
            'Bitcoin',
            context
        );
        aliceEvmIdentity = await buildIdentityFromKeypair(new EthersSigner(context.ethersWallet.alice), context);
    });

    step('check idGraph from sidechain storage before linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new BitcoinSigner(context.bitcoinWallet.alice),
            aliceBitcoinIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);
        assert.lengthOf(idGraph, 0);
    });

    step('linking identities (alice bitcoin account)', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceBitcoinIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const aliceNonce = getNextNonce();
        const [aliceEvmValidation] = await buildValidations(
            context,
            [aliceBitcoinIdentity],
            [aliceEvmIdentity],
            aliceNonce,
            'ethereum',
            undefined,
            [context.ethersWallet.alice]
        );
        const aliceEvmNetworks = context.api.createType('Vec<Web3Network>', [
            'Ethereum',
            'Bsc',
        ]) as unknown as Vec<Web3Network>; // @fixme #1878
        linkIdentityRequestParams.push({
            nonce: aliceNonce,
            identity: aliceEvmIdentity,
            validation: aliceEvmValidation,
            networks: aliceEvmNetworks,
        });

        const identityLinkedEvents: any[] = [];
        const idGraphHashResults: any[] = [];
        let expectedIdGraphs: [LitentryPrimitivesIdentity, boolean][][] = [
            [
                [aliceBitcoinIdentity, true],
                [aliceEvmIdentity, true],
            ],
        ];

        for (const { nonce, identity, validation, networks } of linkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new BitcoinSigner(context.bitcoinWallet.alice),
                aliceBitcoinIdentity,
                identity.toHex(),
                validation.toHex(),
                networks.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);
            idGraphHashResults.push(
                assertIdGraphMutationResult(context, res, 'LinkIdentityResult', expectedIdGraphs[0])
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('linkIdentityCall', res);

            const events = (await eventsPromise).map(({ event }) => event);
            events.forEach((event) => {
                if (context.api.events.identityManagement.LinkIdentityFailed.is(event)) {
                    assert.fail(JSON.stringify(event.toHuman(), null, 4));
                }
                if (context.api.events.identityManagement.IdentityLinked.is(event)) {
                    identityLinkedEvents.push(event);
                }
            });
        }

        await assertIdGraphMutation(
            new BitcoinSigner(context.bitcoinWallet.alice),
            identityLinkedEvents,
            idGraphHashResults,
            1
        );
    });

    step('check user sidechain storage after linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new BitcoinSigner(context.bitcoinWallet.alice),
            aliceBitcoinIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // according to the order of linkIdentityRequestParams
        const expectedWeb3Networks = [['Ethereum', 'Bsc']];
        let currentIndex = 0;

        for (const { identity } of linkIdentityRequestParams) {
            const identityDump = JSON.stringify(identity.toHuman(), null, 4);
            console.debug(`checking identity: ${identityDump}`);
            const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity));
            assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
            const [, idGraphNodeContext] = idGraphNode!;

            const web3networks = idGraphNode![1].web3networks.toHuman();
            assert.deepEqual(web3networks, expectedWeb3Networks[currentIndex]);

            assert.equal(
                idGraphNodeContext.status.toString(),
                'Active',
                `status should be active for identity: ${identityDump}`
            );
            console.debug('active ✅');

            currentIndex++;
        }

        await assertIdGraphHash(context, new BitcoinSigner(context.bitcoinWallet.alice), idGraph);
    });
    step('deactivating identity(alice bitcoin account)', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceBitcoinIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const deactivateIdentityRequestParams: {
            nonce: number;
            identity: LitentryPrimitivesIdentity;
        }[] = [];

        const aliceEvmNonce = getNextNonce();

        deactivateIdentityRequestParams.push({
            nonce: aliceEvmNonce,
            identity: aliceEvmIdentity,
        });

        const identityDeactivatedEvents: any[] = [];
        const idGraphHashResults: any[] = [];
        let expectedIdGraphs: [LitentryPrimitivesIdentity, boolean][][] = [[[aliceEvmIdentity, false]]];

        for (const { nonce, identity } of deactivateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const deactivateIdentityCall = await createSignedTrustedCallDeactivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new BitcoinSigner(context.bitcoinWallet.alice),
                aliceBitcoinIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, deactivateIdentityCall);
            idGraphHashResults.push(
                assertIdGraphMutationResult(context, res, 'DeactivateIdentityResult', expectedIdGraphs[0])
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('deactivateIdentityCall', res);

            const events = (await eventsPromise).map(({ event }) => event);
            events.forEach((event) => {
                if (context.api.events.identityManagement.DeactivateIdentityFailed.is(event)) {
                    assert.fail(JSON.stringify(event.toHuman(), null, 4));
                }
                if (context.api.events.identityManagement.IdentityDeactivated.is(event)) {
                    identityDeactivatedEvents.push(event);
                }
            });
        }

        await assertIdGraphMutation(
            new BitcoinSigner(context.bitcoinWallet.alice),
            identityDeactivatedEvents,
            idGraphHashResults,
            1
        );
    });

    step('check idGraph from sidechain storage after deactivating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new BitcoinSigner(context.bitcoinWallet.alice),
            aliceBitcoinIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        for (const { identity } of linkIdentityRequestParams) {
            const identityDump = JSON.stringify(identity.toHuman(), null, 4);
            console.debug(`checking identity: ${identityDump}`);
            const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity));
            assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
            const [, idGraphNodeContext] = idGraphNode!;

            assert.equal(
                idGraphNodeContext.status.toString(),
                'Inactive',
                `status should be Inactive for identity: ${identityDump}`
            );
            console.debug('inactive ✅');
        }

        await assertIdGraphHash(context, new BitcoinSigner(context.bitcoinWallet.alice), idGraph);
    });
    step('activating identity(alice bitcoin account)', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceBitcoinIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const activateIdentityRequestParams: {
            nonce: number;
            identity: LitentryPrimitivesIdentity;
        }[] = [];

        const aliceEvmNonce = getNextNonce();

        activateIdentityRequestParams.push({
            nonce: aliceEvmNonce,
            identity: aliceEvmIdentity,
        });

        const identityActivatedEvents: any[] = [];
        const idGraphHashResults: any[] = [];
        let expectedIdGraphs: [LitentryPrimitivesIdentity, boolean][][] = [[[aliceEvmIdentity, true]]];

        for (const { nonce, identity } of activateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const activateIdentityCall = await createSignedTrustedCallActivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new BitcoinSigner(context.bitcoinWallet.alice),
                aliceBitcoinIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, activateIdentityCall);
            idGraphHashResults.push(
                assertIdGraphMutationResult(context, res, 'ActivateIdentityResult', expectedIdGraphs[0])
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('activateIdentityCall', res);

            const events = (await eventsPromise).map(({ event }) => event);
            events.forEach((event) => {
                if (context.api.events.identityManagement.ActivateIdentityFailed.is(event)) {
                    assert.fail(JSON.stringify(event.toHuman(), null, 4));
                }
                if (context.api.events.identityManagement.IdentityActivated.is(event)) {
                    identityActivatedEvents.push(event);
                }
            });
        }

        await assertIdGraphMutation(
            new BitcoinSigner(context.bitcoinWallet.alice),
            identityActivatedEvents,
            idGraphHashResults,
            1
        );
    });

    step('check idGraph from sidechain storage after activating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new BitcoinSigner(context.bitcoinWallet.alice),
            aliceBitcoinIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        for (const { identity } of linkIdentityRequestParams) {
            const identityDump = JSON.stringify(identity.toHuman(), null, 4);
            console.debug(`checking identity: ${identityDump}`);
            const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity));
            assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
            const [, idGraphNodeContext] = idGraphNode!;

            assert.equal(
                idGraphNodeContext.status.toString(),
                'Active',
                `status should be active for identity: ${identityDump}`
            );
            console.debug('active ✅');
        }

        await assertIdGraphHash(context, new BitcoinSigner(context.bitcoinWallet.alice), idGraph);
    });
});
