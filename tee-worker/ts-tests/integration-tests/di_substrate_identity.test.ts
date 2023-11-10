import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { hexToU8a, u8aToHex, u8aToString } from '@polkadot/util';
import {
    assertIdentityLinkedResult,
    assertSetUserShieldingKeyResult,
    assertWorkerError,
    buildIdentityFromKeypair,
    buildIdentityHelper,
    buildValidations,
    initIntegrationTestContext,
    PolkadotSigner,
} from './common/utils';
import {
    assertFailedEvent,
    assertIdentity,
    assertInitialIdGraphCreated,
    assertIsInSidechainBlock,
    assertLinkedEvent,
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
    createSignedTrustedCallSetIdentityNetworks,
} from './common/di-utils'; // @fixme move to a better place
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
        teeShieldingKey = await getTeeShieldingKey(context);
        aliceSubject = await buildIdentityFromKeypair(new PolkadotSigner(context.substrateWallet.alice), context);
    });

    step('linking identity with without user shielding key(charlie)', async function () {
        const charlieSubject = await buildIdentityFromKeypair(
            new PolkadotSigner(context.substrateWallet.charlie),
            context
        );

        const bobSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.bob.addressRaw),
            'Substrate',
            context
        );
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;

        const nonce = await getSidechainNonce(context, teeShieldingKey, charlieSubject);
        const [bobValidationData] = await buildValidations(
            context,
            [charlieSubject],
            [bobSubstrateIdentity],
            nonce.toNumber(),
            'substrate',
            context.substrateWallet.bob
        );
        const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);

        const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
            context.api,
            context.mrEnclave,
            nonce,
            new PolkadotSigner(context.substrateWallet.charlie),
            charlieSubject,
            context.sidechainRegistry.createType('LitentryPrimitivesIdentity', bobSubstrateIdentity).toHex(),
            context.api.createType('LitentryValidationData', bobValidationData).toHex(),
            context.api.createType('Vec<Web3Network>', ['Litentry', 'Polkadot']).toHex(),
            keyNonce,
            context.api.createType('Option<UserShieldingKeyType>', aesKey).toHex(),
            requestIdentifier
        );

        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);

        /*
        In the case of an error, the RPC status will be false, right?
        However, will we still have events occurring in Parachain? Based on the example provided.
        */
        assert.isTrue(res.do_watch.isFalse);
        assert.isTrue(res.status.asTrustedOperationStatus[0].isInvalid);
        assertWorkerError(
            context,
            (v) => {
                assert.isTrue(v.isLinkIdentityFailed, `expected LinkIdentityFailed, received ${v.type} instead`);
                assert.isTrue(
                    v.asLinkIdentityFailed.isUserShieldingKeyNotFound,
                    `expected UserShieldingKeyNotFound, received ${v.asLinkIdentityFailed.type} instead`
                );
            },
            res
        );

        const events = await eventsPromise;
        await assertFailedEvent(context, events, 'LinkIdentityFailed', 'UserShieldingKeyNotFound');
    });

    step('check user sidechain storage before user shielding key creating(alice)', async function () {
        const shieldingKeyGetter = await createSignedTrustedGetterUserShieldingKey(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubject
        );

        const shieldingKeyGetResult = await sendRequestFromGetter(context, teeShieldingKey, shieldingKeyGetter);

        const k = context.api.createType('Option<Bytes>', hexToU8a(shieldingKeyGetResult.value.toHex()));
        assert.isTrue(k.isNone, 'shielding key should be empty before set');
    });

    ['alice', 'bob'].forEach((name) => {
        step(`setting user shielding key (${name})`, async function () {
            const wallet = context.substrateWallet[name];
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
    });

    step('check user shielding key from sidechain storage after user shielding key setting(alice)', async function () {
        const shieldingKeyGetter = await createSignedTrustedGetterUserShieldingKey(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubject
        );

        const shieldingKeyGetResult = await sendRequestFromGetter(context, teeShieldingKey, shieldingKeyGetter);

        const k = context.api.createType('Option<Bytes>', hexToU8a(shieldingKeyGetResult.value.toHex()));
        assert.equal(k.value.toString(), aesKey, 'respShieldingKey should be equal aesKey after set');
    });

    step('check idgraph from sidechain storage before linking', async function () {
        const idgraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubject
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idgraphGetter);

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        assert.lengthOf(idGraph, 1);
        const [idGraphNodeIdentity, idGraphNodeContext] = idGraph[0];
        assert.deepEqual(idGraphNodeIdentity.toHuman(), aliceSubject.toHuman(), 'idGraph should include main address');
        assert.equal(idGraphNodeContext.status.toString(), 'Active', 'status should be active for main address');
    });

    step('linking identities (alice)', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubject)).toNumber();
        const getNextNonce = () => currentNonce++;

        const twitterNonce = getNextNonce();
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const [twitterValidation] = await buildValidations(
            context,
            [aliceSubject],
            [twitterIdentity],
            twitterNonce,
            'twitter'
        );
        const twitterNetworks = context.api.createType('Vec<Web3Network>', []) as unknown as Vec<Web3Network>; // @fixme #1878
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
            [aliceSubject],
            [evmIdentity],
            evmNonce,
            'ethereum',
            undefined,
            [context.ethersWallet.alice]
        );
        const evmNetworks = context.api.createType('Vec<Web3Network>', [
            'Ethereum',
            'Bsc',
        ]) as unknown as Vec<Web3Network>; // @fixme #1878
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
            [aliceSubject],
            [eveSubstrateIdentity],
            eveSubstrateNonce,
            'substrate',
            context.substrateWallet.eve
        );
        const eveSubstrateNetworks = context.api.createType('Vec<Web3Network>', [
            'Polkadot',
            'Litentry',
        ]) as unknown as Vec<Web3Network>; // @fixme #1878
        linkIdentityRequestParams.push({
            nonce: eveSubstrateNonce,
            identity: eveSubstrateIdentity,
            validation: eveSubstrateValidation,
            networks: eveSubstrateNetworks,
        });

        const linkedIdentityEvents: any[] = [];
        let expectedIdGraphs: [LitentryPrimitivesIdentity, boolean][][] = [
            [
                [twitterIdentity, true],
                [aliceSubject, true],
            ],
            [
                [evmIdentity, true],
                [twitterIdentity, true],
                [aliceSubject, true],
            ],
            [
                [eveSubstrateIdentity, true],
                [evmIdentity, true],
                [twitterIdentity, true],
                [aliceSubject, true],
            ],
        ];
        for (const { nonce, identity, validation, networks } of linkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new PolkadotSigner(context.substrateWallet.alice),
                aliceSubject,
                identity.toHex(),
                validation.toHex(),
                networks.toHex(),
                keyNonce,
                context.api.createType('Option<UserShieldingKeyType>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);

            assertIdentityLinkedResult(context, identity, res, expectedIdGraphs[0]);
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('linkIdentityCall', res);

            const events = (await eventsPromise).map(({ event }) => event);
            let isIdentityLinked = false;
            events.forEach((event) => {
                if (context.api.events.identityManagement.LinkIdentityFailed.is(event)) {
                    assert.fail(JSON.stringify(event.toHuman(), null, 4));
                }
                if (context.api.events.identityManagement.IdentityLinked.is(event)) {
                    isIdentityLinked = true;
                    linkedIdentityEvents.push(event);
                }
            });
            assert.isTrue(isIdentityLinked);
        }
        assert.equal(linkedIdentityEvents.length, 3);

        await assertLinkedEvent(context, new PolkadotSigner(context.substrateWallet.alice), linkedIdentityEvents, [
            twitterIdentity,
            evmIdentity,
            eveSubstrateIdentity,
        ]);
    });

    step('check user sidechain storage after linking', async function () {
        const idgraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubject
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idgraphGetter);

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // according to the order of linkIdentityRequestParams
        const expectedWeb3Networks = [[], ['Ethereum', 'Bsc'], ['Polkadot', 'Litentry']];
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
    });

    step('linking invalid identity', async function () {
        const aliceSubject = await buildIdentityFromKeypair(new PolkadotSigner(context.substrateWallet.bob), context);

        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubject)).toNumber();

        const getNextNonce = () => currentNonce++;

        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const twitterNonce = getNextNonce();
        const evmNonce = getNextNonce();
        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
        const [evmValidation] = await buildValidations(
            context,
            [aliceSubject],
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
            aliceSubject,
            twitterIdentity.toHex(),
            evmValidation.toHex(),
            evmNetworks.toHex(),
            keyNonce,
            context.api.createType('Option<UserShieldingKeyType>', aesKey).toHex(),
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
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubject)).toNumber();
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
            aliceSubject,
            evmIdentity.toHex(),
            encodedVerifyIdentityValidation.toHex(),
            evmNetworks.toHex(),
            keyNonce,
            context.api.createType('Option<UserShieldingKeyType>', aesKey).toHex(),
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
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubject)).toNumber();
        const getNextNonce = () => currentNonce++;

        const twitterNonce = getNextNonce();
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const [twitterValidation] = await buildValidations(
            context,
            [aliceSubject],
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
            aliceSubject,
            twitterIdentity.toHex(),
            twitterValidation.toHex(),
            twitterNetworks.toHex(),
            keyNonce,
            context.api.createType('Option<UserShieldingKeyType>', aesKey).toHex(),
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

    step('deactivating identity', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubject)).toNumber();
        const getNextNonce = () => currentNonce++;

        const deactivateIdentityRequestParams: {
            nonce: number;
            identity: LitentryPrimitivesIdentity;
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
        const deactivatedIdentityEvents: any[] = [];

        for (const { nonce, identity } of deactivateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const deactivateIdentityCall = await createSignedTrustedCallDeactivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new PolkadotSigner(context.substrateWallet.alice),
                aliceSubject,
                identity.toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, deactivateIdentityCall);

            await assertIsInSidechainBlock('deactivateIdentityCall', res);

            const events = (await eventsPromise).map(({ event }) => event);
            let isIdentityDeactivated = false;
            events.forEach((event) => {
                if (context.api.events.identityManagement.DeactivateIdentityFailed.is(event)) {
                    assert.fail(JSON.stringify(event.toHuman(), null, 4));
                }
                if (context.api.events.identityManagement.IdentityDeactivated.is(event)) {
                    isIdentityDeactivated = true;
                    deactivatedIdentityEvents.push(event);
                }
            });
            assert.isTrue(isIdentityDeactivated);
        }
        assert.equal(deactivatedIdentityEvents.length, 3);

        await assertIdentity(context, deactivatedIdentityEvents, [twitterIdentity, evmIdentity, eveSubstrateIdentity]);
    });

    step('check idgraph from sidechain storage after deactivating', async function () {
        const idgraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubject
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idgraphGetter);
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
    });
    step('activating identity', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubject)).toNumber();
        const getNextNonce = () => currentNonce++;

        const activateIdentityRequestParams: {
            nonce: number;
            identity: LitentryPrimitivesIdentity;
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
        const activatedIdentityEvents: any[] = [];

        for (const { nonce, identity } of activateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const deactivateIdentityCall = await createSignedTrustedCallActivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new PolkadotSigner(context.substrateWallet.alice),
                aliceSubject,
                identity.toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, deactivateIdentityCall);

            await assertIsInSidechainBlock('activateIdentityCall', res);

            const events = (await eventsPromise).map(({ event }) => event);
            let isIdentityActivated = false;
            events.forEach((event) => {
                if (context.api.events.identityManagement.ActivateIdentityFailed.is(event)) {
                    assert.fail(JSON.stringify(event.toHuman(), null, 4));
                }
                if (context.api.events.identityManagement.IdentityActivated.is(event)) {
                    isIdentityActivated = true;
                    activatedIdentityEvents.push(event);
                }
            });
            assert.isTrue(isIdentityActivated);
        }
        assert.equal(activatedIdentityEvents.length, 3);
        await assertIdentity(context, activatedIdentityEvents, [twitterIdentity, evmIdentity, eveSubstrateIdentity]);
    });

    step('check idgraph from sidechain storage after activating', async function () {
        const idgraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubject
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idgraphGetter);
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
    });

    step('check idgraph from sidechain storage before setting identity network', async function () {
        const expectedWeb3Networks = ['Polkadot', 'Litentry'];
        const idgraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubject
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idgraphGetter);
        const idgraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // we have 3 identities and the first one is the eveSubstrateIdentity and network is ['Polkadot', 'Litentry']
        assert.equal(idgraph[0][1].web3networks.toHuman()?.toString(), expectedWeb3Networks.toString());
    });
    step('setting identity network(alice)', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubject)).toNumber();
        const getNextNonce = () => currentNonce++;
        const eveSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.eve.addressRaw),
            'Substrate',
            context
        );
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const nonce = getNextNonce();

        // we set the network to ['Litentry', 'Kusama']
        const setIdentityNetworksCall = await createSignedTrustedCallSetIdentityNetworks(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', nonce),
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubject,
            eveSubstrateIdentity.toHex(),
            context.api.createType('Vec<Web3Network>', ['Litentry', 'Kusama']).toHex(),
            requestIdentifier
        );
        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, setIdentityNetworksCall);
        console.log('setIdentityNetworks call returned', res.toHuman());
        assertIsInSidechainBlock('setIdentityNetworksCall', res);
    });

    step('check idgraph from sidechain storage after setting identity network', async function () {
        const expectedWeb3Networks = ['Kusama', 'Litentry'];
        const idgraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubject
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idgraphGetter);
        const idgraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // we have 3 identities and the first one should be changed to expectedWeb3Networks
        assert.equal(
            idgraph[0][1].web3networks.toHuman()?.toString(),
            expectedWeb3Networks.toString(),
            'idgraph should be changed after setting network'
        );
    });

    step('setting incompatible identity network(alice)', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubject)).toNumber();
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

            aliceSubject,
            eveSubstrateIdentity.toHex(),
            context.api.createType('Vec<Web3Network>', ['BSC', 'Ethereum']).toHex(),
            requestIdentifier
        );
        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, setIdentityNetworksCall);
        assertWorkerError(
            context,
            (v) => {
                assert.isTrue(v.isDispatch, `expected Dispatch, received ${v.type} instead`);
                assert.equal(
                    v.asDispatch.toString(),
                    ' error: Module(ModuleError { index: 6, error: [5, 0, 0, 0], message: Some("WrongWeb3NetworkTypes") })'
                );
            },
            res
        );
        console.log('setIdentityNetworks call returned', res.toHuman());
        assert.isTrue(res.status.isTrustedOperationStatus && res.status.asTrustedOperationStatus[0].isInvalid);
    });

    step('check idgraph from sidechain storage after setting incompatible identity network', async function () {
        const expectedWeb3Networks = ['Kusama', 'Litentry'];
        const idgraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubject
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idgraphGetter);
        const idgraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // we have 3 identities and the first one should be changed to expectedWeb3Networks
        assert.equal(
            idgraph[0][1].web3networks.toHuman()?.toString(),
            expectedWeb3Networks.toString(),
            'idgraph should not be changed after setting incompatible network'
        );
    });
    step('deactivating prime identity is disallowed', async function () {
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubject)).toNumber();
        const getNextNonce = () => currentNonce++;

        //  nonce should be plus 1 why?
        const nonce = getNextNonce() + 1;

        // prime identity
        const substratePrimeIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'Substrate',
            context
        );

        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
        const deactivateIdentityCall = await createSignedTrustedCallDeactivateIdentity(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', nonce),
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubject,
            substratePrimeIdentity.toHex(),
            requestIdentifier
        );

        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, deactivateIdentityCall);
        assert.isTrue(res.do_watch.isFalse);
        assert.isTrue(res.status.asTrustedOperationStatus[0].isInvalid);
        assertWorkerError(
            context,
            (v) => {
                assert.isTrue(
                    v.isDeactivateIdentityFailed,
                    `expected DeactivateIdentityFailed, received ${v.type} instead`
                );
                assert.isTrue(
                    v.asDeactivateIdentityFailed.isStfError,
                    `expected StfError, received ${v.asDeactivateIdentityFailed.type} instead`
                );
                assert.equal(u8aToString(v.asDeactivateIdentityFailed.asStfError), 'DeactivatePrimeIdentityDisallowed');
            },
            res
        );
        const events = await eventsPromise;
        await assertFailedEvent(context, events, 'DeactivateIdentityFailed', 'DeactivatePrimeIdentityDisallowed');
    });
});
