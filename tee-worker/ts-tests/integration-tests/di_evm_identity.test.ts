import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { u8aToHex } from '@polkadot/util';
import {
    buildIdentityFromKeypair,
    buildIdentityHelper,
    buildValidations,
    initIntegrationTestContext,
    EthersSigner,
    assertIdGraphMutationResult,
    assertIdGraphHash,
} from './common/utils';
import { assertIsInSidechainBlock, assertIdGraphMutationEvent } from './common/utils/assertion';
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
import { LitentryValidationData, Web3Network, CorePrimitivesIdentity } from 'parachain-api';
import { Vec, Bytes } from '@polkadot/types';
import { subscribeToEventsWithExtHash } from './common/transactions';

describe('Test Identity (evm direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceEvmIdentity: CorePrimitivesIdentity = undefined as any;
    let bobEvmIdentity: CorePrimitivesIdentity;

    // Alice links:
    // - a `mock_user` twitter
    // - alice's evm identity
    // - eve's substrate identity (as alice can't link her own substrate again)
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

        aliceEvmIdentity = await buildIdentityFromKeypair(new EthersSigner(context.ethersWallet.alice), context);
        bobEvmIdentity = await buildIdentityFromKeypair(new EthersSigner(context.ethersWallet.bob), context);
    });

    step('check idGraph from sidechain storage before linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new EthersSigner(context.ethersWallet.alice),
            aliceEvmIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        assert.lengthOf(idGraph, 0);
    });

    step('linking identities (alice evm account)', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceEvmIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const bobEvmNonce = getNextNonce();
        const [bobEvmValidation] = await buildValidations(
            context,
            [aliceEvmIdentity],
            [bobEvmIdentity],
            bobEvmNonce,
            'ethereum',
            undefined,
            [context.ethersWallet.bob]
        );
        const bobEvmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);
        linkIdentityRequestParams.push({
            nonce: bobEvmNonce,
            identity: bobEvmIdentity,
            validation: bobEvmValidation,
            networks: bobEvmNetworks,
        });

        const eveSubstrateNonce = getNextNonce();
        const eveSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.eve.addressRaw),
            'Substrate',
            context
        );
        const [eveSubstrateValidation] = await buildValidations(
            context,
            [aliceEvmIdentity],
            [eveSubstrateIdentity],
            eveSubstrateNonce,
            'substrate',
            context.substrateWallet.eve
        );
        const eveSubstrateNetworks = context.api.createType('Vec<Web3Network>', ['Litentry', 'Khala']);
        linkIdentityRequestParams.push({
            nonce: eveSubstrateNonce,
            identity: eveSubstrateIdentity,
            validation: eveSubstrateValidation,
            networks: eveSubstrateNetworks,
        });

        const identityLinkedEvents: any[] = [];
        const idGraphHashResults: any[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [
                [aliceEvmIdentity, true],
                [bobEvmIdentity, true],
            ],
            [[eveSubstrateIdentity, true]],
        ];

        for (const { nonce, identity, validation, networks } of linkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new EthersSigner(context.ethersWallet.alice),
                aliceEvmIdentity,
                identity.toHex(),
                validation.toHex(),
                networks.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceEvmIdentity,
                    res,
                    'LinkIdentityResult',
                    expectedIdGraphs[0]
                )
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

        await assertIdGraphMutationEvent(
            context,
            new EthersSigner(context.ethersWallet.alice),
            identityLinkedEvents,
            idGraphHashResults,
            2
        );
    });

    step('check user sidechain storage after linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new EthersSigner(context.ethersWallet.alice),
            aliceEvmIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // according to the order of linkIdentityRequestParams
        const expectedWeb3Networks = [
            ['Ethereum', 'Bsc'],
            ['Litentry', 'Khala'],
        ];
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

        await assertIdGraphHash(context, teeShieldingKey, aliceEvmIdentity, idGraph);
    });
    step('deactivating identity(alice evm account)', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceEvmIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const deactivateIdentityRequestParams: {
            nonce: number;
            identity: CorePrimitivesIdentity;
        }[] = [];

        const bobEvmNonce = getNextNonce();

        deactivateIdentityRequestParams.push({
            nonce: bobEvmNonce,
            identity: bobEvmIdentity,
        });

        const eveSubstrateNonce = getNextNonce();
        const eveSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.eve.addressRaw),
            'Substrate',
            context
        );
        deactivateIdentityRequestParams.push({
            nonce: eveSubstrateNonce,
            identity: eveSubstrateIdentity,
        });

        const identityDeactivatedEvents: any[] = [];
        const idGraphHashResults: any[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [[bobEvmIdentity, false]],
            [[eveSubstrateIdentity, false]],
        ];

        for (const { nonce, identity } of deactivateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const deactivateIdentityCall = await createSignedTrustedCallDeactivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new EthersSigner(context.ethersWallet.alice),
                aliceEvmIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, deactivateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceEvmIdentity,
                    res,
                    'DeactivateIdentityResult',
                    expectedIdGraphs[0]
                )
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

        await assertIdGraphMutationEvent(
            context,
            new EthersSigner(context.ethersWallet.alice),
            identityDeactivatedEvents,
            idGraphHashResults,
            2
        );
    });

    step('check idGraph from sidechain storage after deactivating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new EthersSigner(context.ethersWallet.alice),
            aliceEvmIdentity
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

        await assertIdGraphHash(context, teeShieldingKey, aliceEvmIdentity, idGraph);
    });
    step('activating identity(alice evm account)', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceEvmIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const activateIdentityRequestParams: {
            nonce: number;
            identity: CorePrimitivesIdentity;
        }[] = [];

        const bobEvmNonce = getNextNonce();

        activateIdentityRequestParams.push({
            nonce: bobEvmNonce,
            identity: bobEvmIdentity,
        });

        const eveSubstrateNonce = getNextNonce();
        const eveSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.eve.addressRaw),
            'Substrate',
            context
        );
        activateIdentityRequestParams.push({
            nonce: eveSubstrateNonce,
            identity: eveSubstrateIdentity,
        });

        const identityActivatedEvents: any[] = [];
        const idGraphHashResults: any[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [[bobEvmIdentity, true]],
            [[eveSubstrateIdentity, true]],
        ];

        for (const { nonce, identity } of activateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const activateIdentityCall = await createSignedTrustedCallActivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new EthersSigner(context.ethersWallet.alice),
                aliceEvmIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, activateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceEvmIdentity,
                    res,
                    'ActivateIdentityResult',
                    expectedIdGraphs[0]
                )
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

        await assertIdGraphMutationEvent(
            context,
            new EthersSigner(context.ethersWallet.alice),
            identityActivatedEvents,
            idGraphHashResults,
            2
        );
    });

    step('check idGraph from sidechain storage after activating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new EthersSigner(context.ethersWallet.alice),
            aliceEvmIdentity
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

        await assertIdGraphHash(context, teeShieldingKey, aliceEvmIdentity, idGraph);
    });
});
