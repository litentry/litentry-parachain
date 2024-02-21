import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { u8aToHex, u8aToString, bufferToU8a } from '@polkadot/util';
import {
    assertIdGraphMutationResult,
    assertIdGraphHash,
    assertWorkerError,
    buildIdentityFromKeypair,
    buildIdentityHelper,
    buildValidations,
    initIntegrationTestContext,
    PolkadotSigner,
} from './common/utils';
import { assertFailedEvent, assertIsInSidechainBlock, assertIdGraphMutationEvent } from './common/utils/assertion';
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
    createSignedTrustedCallSetIdentityNetworks,
} from './common/di-utils'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import { LitentryValidationData, Web3Network, CorePrimitivesIdentity } from 'parachain-api';
import { Vec, Bytes } from '@polkadot/types';
import { ethers } from 'ethers';
import type { HexString } from '@polkadot/util/types';
import { subscribeToEventsWithExtHash } from './common/transactions';

describe('Test Identity (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubstrateIdentity: CorePrimitivesIdentity = undefined as any;

    // Alice links:
    // - a `mock_user` twitter
    // - alice's evm identity
    // - eve's substrate identity (as alice can't link her own substrate again)
    // - alice's bitcoin identity
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

    step('check idgraph from sidechain storage before linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubstrateIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        assert.lengthOf(idGraph, 0);
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
        const twitterNetworks = context.api.createType('Vec<Web3Network>', []); // @fixme #1878
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
        const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']); // @fixme #1878
        linkIdentityRequestParams.push({
            nonce: evmNonce,
            identity: evmIdentity,
            validation: evmValidation,
            networks: evmNetworks,
        });

        const eveSubstrateNonce = getNextNonce();
        const eveSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.eve.addressRaw),
            'Substrate',
            context
        );
        const [eveSubstrateValidation] = await buildValidations(
            context,
            [aliceSubstrateIdentity],
            [eveSubstrateIdentity],
            eveSubstrateNonce,
            'substrate',
            context.substrateWallet.eve
        );
        const eveSubstrateNetworks = context.api.createType('Vec<Web3Network>', ['Polkadot', 'Litentry']); // @fixme #1878
        linkIdentityRequestParams.push({
            nonce: eveSubstrateNonce,
            identity: eveSubstrateIdentity,
            validation: eveSubstrateValidation,
            networks: eveSubstrateNetworks,
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
        const bitcoinNetworks = context.api.createType('Vec<Web3Network>', ['BitcoinP2tr']); // @fixme #1878
        linkIdentityRequestParams.push({
            nonce: bitcoinNonce,
            identity: bitcoinIdentity,
            validation: bitcoinValidation,
            networks: bitcoinNetworks,
        });

        const identityLinkedEvents: any[] = [];
        const idGraphHashResults: any[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [
                [aliceSubstrateIdentity, true],
                [twitterIdentity, true],
            ],
            [[evmIdentity, true]],
            [[eveSubstrateIdentity, true]],
            [[bitcoinIdentity, true]],
        ];

        for (const { nonce, identity, validation, networks } of linkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
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
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceSubstrateIdentity,
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
            new PolkadotSigner(context.substrateWallet.alice),
            identityLinkedEvents,
            idGraphHashResults,
            4
        );
    });

    step('check user sidechain storage after linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubstrateIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // according to the order of linkIdentityRequestParams
        const expectedWeb3Networks = [[], ['Ethereum', 'Bsc'], ['Polkadot', 'Litentry'], ['BitcoinP2tr']];
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

        await assertIdGraphHash(context, teeShieldingKey, aliceSubstrateIdentity, idGraph);
    });

    step('linking invalid identity', async function () {
        const bobSubstrateIdentity = await buildIdentityFromKeypair(
            new PolkadotSigner(context.substrateWallet.bob),
            context
        );

        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, bobSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const twitterNonce = getNextNonce();
        const evmNonce = getNextNonce();
        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
        const [evmValidation] = await buildValidations(
            context,
            [bobSubstrateIdentity],
            [evmIdentity],
            evmNonce,
            'ethereum',
            undefined,
            [context.ethersWallet.bob]
        );

        const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
        const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', twitterNonce),
            new PolkadotSigner(context.substrateWallet.bob),
            bobSubstrateIdentity,
            twitterIdentity.toHex(),
            evmValidation.toHex(),
            evmNetworks.toHex(),
            context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
            requestIdentifier
        );

        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);

        assert.isTrue(res.do_watch.isFalse);
        assert.isTrue(res.status.asTrustedOperationStatus[0].isInvalid);
        assertWorkerError(
            context,
            (v) => {
                assert.isTrue(v.isLinkIdentityFailed, `expected LinkIdentityFailed, received ${v.type} instead`);
                assert.isTrue(
                    v.asLinkIdentityFailed.isInvalidIdentity,
                    `expected InvalidIdentity, received ${v.asLinkIdentityFailed.type} instead`
                );
            },
            res
        );
        const events = await eventsPromise;
        await assertFailedEvent(context, events, 'LinkIdentityFailed', 'InvalidIdentity');
    });

    step('linking identity with wrong signature', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;
        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
        const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);

        const evmNonce = getNextNonce();
        // random wrong msg
        const wrongMsg = '0x693d9131808e7a8574c7ea5eb7813bdf356223263e61fa8fe2ee8e434508bc75';
        const evmSignature = (await context.ethersWallet.alice.signMessage(
            ethers.utils.arrayify(wrongMsg)
        )) as HexString;

        const evmValidationData = {
            Web3Validation: {
                Evm: {
                    message: wrongMsg as HexString,
                    signature: {
                        Ethereum: evmSignature as HexString,
                    },
                },
            },
        };
        const encodedVerifyIdentityValidation = context.api.createType('LitentryValidationData', evmValidationData);
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);

        const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', evmNonce),
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubstrateIdentity,
            evmIdentity.toHex(),
            encodedVerifyIdentityValidation.toHex(),
            evmNetworks.toHex(),
            context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
            requestIdentifier
        );
        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);

        assert.isTrue(res.do_watch.isFalse);
        assert.isTrue(res.status.asTrustedOperationStatus[0].isInvalid);
        assertWorkerError(
            context,
            (v) => {
                assert.isTrue(v.isLinkIdentityFailed, `expected LinkIdentityFailed, received ${v.type} instead`);
                assert.isTrue(
                    v.asLinkIdentityFailed.isUnexpectedMessage,
                    `expected UnexpectedMessage, received ${v.asLinkIdentityFailed.type} instead`
                );
            },
            res
        );
        const events = await eventsPromise;

        await assertFailedEvent(context, events, 'LinkIdentityFailed', 'UnexpectedMessage');
    });

    step('linking already linked identity', async function () {
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

        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
        const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', twitterNonce),
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubstrateIdentity,
            twitterIdentity.toHex(),
            twitterValidation.toHex(),
            twitterNetworks.toHex(),
            context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
            requestIdentifier
        );
        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);

        assert.isTrue(res.do_watch.isFalse);
        assert.isTrue(res.status.asTrustedOperationStatus[0].isInvalid);
        assertWorkerError(
            context,
            (v) => {
                assert.isTrue(v.isLinkIdentityFailed, `expected LinkIdentityFailed, received ${v.type} instead`);
                assert.isTrue(
                    v.asLinkIdentityFailed.isStfError,
                    `expected StfError, received ${v.asLinkIdentityFailed.type} instead`
                );
                assert.equal(u8aToString(v.asLinkIdentityFailed.asStfError), 'IdentityAlreadyLinked');
            },
            res
        );
        const events = await eventsPromise;
        await assertFailedEvent(context, events, 'LinkIdentityFailed', 'IdentityAlreadyLinked');
    });

    step('deactivating linked identities', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const deactivateIdentityRequestParams: {
            nonce: number;
            identity: CorePrimitivesIdentity;
        }[] = [];

        const twitterNonce = getNextNonce();
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);

        deactivateIdentityRequestParams.push({
            nonce: twitterNonce,
            identity: twitterIdentity,
        });

        const evmNonce = getNextNonce();
        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);

        deactivateIdentityRequestParams.push({
            nonce: evmNonce,
            identity: evmIdentity,
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

        const bitcoinNonce = getNextNonce();
        const bitcoinIdentity = await buildIdentityHelper(
            u8aToHex(bufferToU8a(context.bitcoinWallet.alice.toPublicKey().toBuffer())),
            'Bitcoin',
            context
        );
        deactivateIdentityRequestParams.push({
            nonce: bitcoinNonce,
            identity: bitcoinIdentity,
        });

        const identityDeactivatedEvents: any[] = [];
        const idGraphHashResults: any[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [[twitterIdentity, false]],
            [[evmIdentity, false]],
            [[eveSubstrateIdentity, false]],
            [[bitcoinIdentity, false]],
        ];

        for (const { nonce, identity } of deactivateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const deactivateIdentityCall = await createSignedTrustedCallDeactivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new PolkadotSigner(context.substrateWallet.alice),
                aliceSubstrateIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, deactivateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceSubstrateIdentity,
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
            new PolkadotSigner(context.substrateWallet.alice),
            identityDeactivatedEvents,
            idGraphHashResults,
            4
        );
    });

    step('check idgraph from sidechain storage after deactivating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubstrateIdentity
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

        await assertIdGraphHash(context, teeShieldingKey, aliceSubstrateIdentity, idGraph);
    });
    step('activating linked identities', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const activateIdentityRequestParams: {
            nonce: number;
            identity: CorePrimitivesIdentity;
        }[] = [];

        const twitterNonce = getNextNonce();
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);

        activateIdentityRequestParams.push({
            nonce: twitterNonce,
            identity: twitterIdentity,
        });

        const evmNonce = getNextNonce();
        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);

        activateIdentityRequestParams.push({
            nonce: evmNonce,
            identity: evmIdentity,
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

        const bitcoinNonce = getNextNonce();
        const bitcoinIdentity = await buildIdentityHelper(
            u8aToHex(bufferToU8a(context.bitcoinWallet.alice.toPublicKey().toBuffer())),
            'Bitcoin',
            context
        );
        activateIdentityRequestParams.push({
            nonce: bitcoinNonce,
            identity: bitcoinIdentity,
        });

        const identityActivatedEvents: any[] = [];
        const idGraphHashResults: any[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [[twitterIdentity, true]],
            [[evmIdentity, true]],
            [[eveSubstrateIdentity, true]],
            [[bitcoinIdentity, true]],
        ];

        for (const { nonce, identity } of activateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const activateIdentityCall = await createSignedTrustedCallActivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new PolkadotSigner(context.substrateWallet.alice),
                aliceSubstrateIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, activateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceSubstrateIdentity,
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
            new PolkadotSigner(context.substrateWallet.alice),
            identityActivatedEvents,
            idGraphHashResults,
            4
        );
    });

    step('check idgraph from sidechain storage after activating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubstrateIdentity
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

        await assertIdGraphHash(context, teeShieldingKey, aliceSubstrateIdentity, idGraph);
    });

    step('check idgraph from sidechain storage before setting identity network', async function () {
        const expectedWeb3Networks = ['Polkadot', 'Litentry'];
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubstrateIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // the third (last) identity in the IDGraph is eveSubstrateIdentity
        assert.equal(idGraph[3][1].web3networks.toHuman()?.toString(), expectedWeb3Networks.toString());
    });

    step('setting identity network(alice)', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;
        const eveSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.eve.addressRaw),
            'Substrate',
            context
        );
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const nonce = getNextNonce();

        const identityNetworksSetEvents: any[] = [];
        const idGraphHashResults: any[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [[[eveSubstrateIdentity, true]]];

        const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
        // we set the network to ['Litentry', 'Kusama']
        const setIdentityNetworksCall = await createSignedTrustedCallSetIdentityNetworks(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', nonce),
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubstrateIdentity,
            eveSubstrateIdentity.toHex(),
            context.api.createType('Vec<Web3Network>', ['Litentry', 'Kusama']).toHex(),
            context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
            requestIdentifier
        );

        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, setIdentityNetworksCall);
        idGraphHashResults.push(
            await assertIdGraphMutationResult(
                context,
                teeShieldingKey,
                aliceSubstrateIdentity,
                res,
                'ActivateIdentityResult',
                expectedIdGraphs[0]
            )
        );
        expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
        await assertIsInSidechainBlock('setIdentityNetworksCall', res);

        const events = (await eventsPromise).map(({ event }) => event);
        events.forEach((event) => {
            if (context.api.events.identityManagement.IdentityNetworksSet.is(event)) {
                identityNetworksSetEvents.push(event);
            }
        });
        await assertIdGraphMutationEvent(
            context,
            new PolkadotSigner(context.substrateWallet.alice),
            identityNetworksSetEvents,
            idGraphHashResults,
            1
        );
    });

    step('check idgraph from sidechain storage after setting identity network', async function () {
        const expectedWeb3Networks = ['Kusama', 'Litentry'];
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubstrateIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        assert.equal(
            idGraph[3][1].web3networks.toHuman()?.toString(),
            expectedWeb3Networks.toString(),
            'idGraph should be changed after setting network'
        );

        await assertIdGraphHash(context, teeShieldingKey, aliceSubstrateIdentity, idGraph);
    });

    step('setting incompatible identity network(alice)', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;
        const eveSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.eve.addressRaw),
            'Substrate',
            context
        );
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const nonce = getNextNonce();

        // alice address is not compatible with ethereum network
        const setIdentityNetworksCall = await createSignedTrustedCallSetIdentityNetworks(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', nonce),
            new PolkadotSigner(context.substrateWallet.alice),

            aliceSubstrateIdentity,
            eveSubstrateIdentity.toHex(),
            context.api.createType('Vec<Web3Network>', ['BSC', 'Ethereum']).toHex(),
            context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
            requestIdentifier
        );
        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, setIdentityNetworksCall);
        assertWorkerError(
            context,
            (v) => {
                assert.isTrue(v.isDispatch, `expected Dispatch, received ${v.type} instead`);
                assert.equal(
                    v.asDispatch.toString(),
                    ' error: Module(ModuleError { index: 6, error: [4, 0, 0, 0], message: Some("WrongWeb3NetworkTypes") })'
                );
            },
            res
        );
        console.log('setIdentityNetworks call returned', res.toHuman());
        assert.isTrue(res.status.isTrustedOperationStatus && res.status.asTrustedOperationStatus[0].isInvalid);
    });

    step('check idgraph from sidechain storage after setting incompatible identity network', async function () {
        const expectedWeb3Networks = ['Kusama', 'Litentry'];
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubstrateIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        assert.equal(
            idGraph[3][1].web3networks.toHuman()?.toString(),
            expectedWeb3Networks.toString(),
            'idGraph should not be changed after setting incompatible network'
        );
    });

    step('deactivate prime identity', async function () {
        // deactivating prime identity should be possible and create the IDGraph if one doesn't exist already
        const bobSubstrateIdentity = await buildIdentityFromKeypair(
            new PolkadotSigner(context.substrateWallet.bob),
            context
        );

        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, bobSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const deactivateIdentityRequestParams: {
            nonce: number;
            identity: CorePrimitivesIdentity;
        }[] = [];

        deactivateIdentityRequestParams.push({
            nonce: getNextNonce(),
            identity: bobSubstrateIdentity,
        });

        const identityDeactivatedEvents: any[] = [];
        const idGraphHashResults: any[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [[[bobSubstrateIdentity, false]]];

        for (const { nonce, identity } of deactivateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const deactivateIdentityCall = await createSignedTrustedCallDeactivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new PolkadotSigner(context.substrateWallet.bob),
                bobSubstrateIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, deactivateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    bobSubstrateIdentity,
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
            new PolkadotSigner(context.substrateWallet.bob),
            identityDeactivatedEvents,
            idGraphHashResults,
            1
        );
    });

    step('setting identity networks for prime identity)', async function () {
        const charlieSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.charlie.addressRaw),
            'Substrate',
            context
        );

        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, charlieSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const nonce = getNextNonce();

        const identityNetworksSetEvents: any[] = [];
        const idGraphHashResults: any[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [[[charlieSubstrateIdentity, true]]];

        const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
        // we set the network to ['Litentry', 'Kusama']
        const setIdentityNetworksCall = await createSignedTrustedCallSetIdentityNetworks(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', nonce),
            new PolkadotSigner(context.substrateWallet.charlie),
            charlieSubstrateIdentity,
            charlieSubstrateIdentity.toHex(),
            context.api.createType('Vec<Web3Network>', ['Litentry', 'Kusama']).toHex(),
            context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
            requestIdentifier
        );

        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, setIdentityNetworksCall);
        idGraphHashResults.push(
            await assertIdGraphMutationResult(
                context,
                teeShieldingKey,
                charlieSubstrateIdentity,
                res,
                'ActivateIdentityResult',
                expectedIdGraphs[0]
            )
        );
        expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
        await assertIsInSidechainBlock('setIdentityNetworksCall', res);

        const events = (await eventsPromise).map(({ event }) => event);
        events.forEach((event) => {
            if (context.api.events.identityManagement.IdentityNetworksSet.is(event)) {
                identityNetworksSetEvents.push(event);
            }
        });
        await assertIdGraphMutationEvent(
            context,
            new PolkadotSigner(context.substrateWallet.charlie),
            identityNetworksSetEvents,
            idGraphHashResults,
            1
        );
    });
});
