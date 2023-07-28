import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import {
    buildIdentityFromKeypair,
    buildIdentityHelper,
    buildValidations,
    initIntegrationTestContext,
    parseIdentity,
} from './common/utils';
import {
    assertFailedEvent,
    assertIsInSidechainBlock,
    assertLinkedEvent,
    assertInitialIdGraphCreated,
    assertIdentity,
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
} from './examples/direct-invocation/util'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/type-definitions';
import { aesKey, keyNonce } from './common/call';
import { LitentryValidationData, Web3Network } from 'parachain-api';
import { LitentryPrimitivesIdentity } from 'sidechain-api';
import { Vec } from '@polkadot/types';
import { subscribeToEventsWithExtHash } from './common/transactions';
import { Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import * as polkadotCryptoUtils from '@polkadot/util-crypto';
describe('Test Identity (evm direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceEvmSubject: LitentryPrimitivesIdentity = undefined as any;
    let aliceEvmPair: KeyringPair = undefined as any;

    // the address of substrate format of alice's evm account
    let aliceEvmToAddress: string = undefined as any;

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
            process.env.WORKER_END_POINT!, // @fixme evil assertion; centralize env access
            process.env.SUBSTRATE_END_POINT!, // @fixme evil assertion; centralize env access
            0
        );
        teeShieldingKey = await getTeeShieldingKey(context.tee, context.api);

        const evmKeyring = new Keyring({ type: 'ethereum' });
        const result = Buffer.alloc(32);

        //32 bytes
        const aliceEvmPhareser = `0x` + result.fill('alice', 32 - Buffer.from('alice', 'utf8').length).toString('hex');

        aliceEvmPair = evmKeyring.addFromUri(aliceEvmPhareser);
        aliceEvmSubject = await buildIdentityFromKeypair(aliceEvmPair, context);
        aliceEvmToAddress = polkadotCryptoUtils.evmToAddress(aliceEvmPair.address, context.chainIdentifier);
    });

    it('needs a lot more work to be complete');
    it('most of the bob cases are missing');

    step(`setting user shielding key (alice evm account)`, async function () {
        const nonce = await getSidechainNonce(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            aliceEvmSubject
        );

        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;

        const setUserShieldingKeyCall = createSignedTrustedCallSetUserShieldingKey(
            context.api,
            context.mrEnclave,
            nonce,
            aliceEvmPair,
            aliceEvmSubject,
            aesKey,
            requestIdentifier
        );

        const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
        const res = await sendRequestFromTrustedCall(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            setUserShieldingKeyCall
        );
        await assertIsInSidechainBlock('setUserShieldingKeyCall', res);

        const events = await eventsPromise;

        const userShieldingKeySetEvents = events
            .map(({ event }) => event)
            .filter(({ section, method }) => section === 'identityManagement' && method === 'UserShieldingKeySet');

        // check event length
        assert.equal(userShieldingKeySetEvents.length, 1, 'userShieldingKeySetEvents.length should be 1');
        await assertInitialIdGraphCreated(context, aliceEvmPair, userShieldingKeySetEvents);
    });

    step('check user shielding key from sidechain storage after user shielding key setting(alice)', async function () {
        const shieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(
            context.api,
            aliceEvmPair,
            aliceEvmSubject
        );

        const shieldingKeyGetResult = await sendRequestFromGetter(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            shieldingKeyGetter
        );
        console.log('shieldingKeyGetResult.value.toHex()', shieldingKeyGetResult.value.toHex());
        const k = context.api.createType('Option<Bytes>', hexToU8a(shieldingKeyGetResult.value.toHex()));
        assert.equal(k.value.toString(), aesKey, 'respShieldingKey should be equal aesKey after set');
    });

    step('check idgraph from sidechain storage before linking', async function () {
        const idgraphGetter = createSignedTrustedGetterIdGraph(context.api, aliceEvmPair, aliceEvmSubject);
        const res = await sendRequestFromGetter(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            idgraphGetter
        );

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        assert.lengthOf(idGraph, 1);
        const [idGraphNodeIdentity, idGraphNodeContext] = idGraph[0];
        assert.deepEqual(
            idGraphNodeIdentity.toHuman(),
            aliceEvmSubject.toHuman(),
            'idGraph should include main address'
        );
        assert.equal(idGraphNodeContext.status.toString(), 'Active', 'status should be active for main address');
    });

    step('linking identities (alice evm account)', async function () {
        let currentNonce = (
            await getSidechainNonce(context.tee, context.api, context.mrEnclave, teeShieldingKey, aliceEvmSubject)
        ).toNumber();
        const getNextNonce = () => currentNonce++;

        const bobEvmNonce = getNextNonce();
        const bobEvmIdentity = await buildIdentityHelper(context.ethersWallet.bob.address, 'Evm', context);
        const [bobEvmValidation] = await buildValidations(
            context,
            [aliceEvmSubject],
            [bobEvmIdentity],
            bobEvmNonce,
            'ethereum',
            undefined,
            [context.ethersWallet.bob]
        );
        const bobEvmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Polygon']);
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
            [aliceEvmSubject],
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
        const linkedIdentityEvents: any[] = [];
        for (const { nonce, identity, validation, networks } of linkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const linkIdentityCall = createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                aliceEvmPair,
                aliceEvmSubject,
                identity.toHex(),
                validation.toHex(),
                networks.toHex(),
                keyNonce,
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(
                context.tee,
                context.api,
                context.mrEnclave,
                teeShieldingKey,
                linkIdentityCall
            );
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

        // check event data
        assert.equal(linkedIdentityEvents.length, 2);
        await assertLinkedEvent(context, aliceEvmPair, linkedIdentityEvents, [bobEvmIdentity, eveSubstrateIdentity]);
    });

    step('check user sidechain storage after linking', async function () {
        const idgraphGetter = createSignedTrustedGetterIdGraph(context.api, aliceEvmPair, aliceEvmSubject);
        const res = await sendRequestFromGetter(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            idgraphGetter
        );

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // according to the order of linkIdentityRequestParams
        const expectedWeb3Networks = [
            ['Ethereum', 'Polygon'],
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
    });
    step('deactivating identity(alice evm account)', async function () {
        let currentNonce = (
            await getSidechainNonce(context.tee, context.api, context.mrEnclave, teeShieldingKey, aliceEvmSubject)
        ).toNumber();
        const getNextNonce = () => currentNonce++;

        const deactivateIdentityRequestParams: {
            nonce: number;
            identity: LitentryPrimitivesIdentity;
        }[] = [];

        const bobEvmNonce = getNextNonce();
        const bobEvmIdentity = await buildIdentityHelper(context.ethersWallet.bob.address, 'Evm', context);

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
        const deactivatedIdentityEvents: any[] = [];

        for (const { nonce, identity } of deactivateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const deactivateIdentityCall = createSignedTrustedCallDeactivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                aliceEvmPair,
                aliceEvmSubject,
                identity.toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(
                context.tee,
                context.api,
                context.mrEnclave,
                teeShieldingKey,
                deactivateIdentityCall
            );

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

        assert.equal(deactivatedIdentityEvents.length, 2);

        await assertIdentity(context, aliceEvmPair, deactivatedIdentityEvents, [bobEvmIdentity, eveSubstrateIdentity]);
    });

    step('check idgraph from sidechain storage after deactivating', async function () {
        const idgraphGetter = createSignedTrustedGetterIdGraph(context.api, aliceEvmPair, aliceEvmSubject);
        const res = await sendRequestFromGetter(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            idgraphGetter
        );
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
    step('activating identity(alice evm account)', async function () {
        let currentNonce = (
            await getSidechainNonce(context.tee, context.api, context.mrEnclave, teeShieldingKey, aliceEvmSubject)
        ).toNumber();
        const getNextNonce = () => currentNonce++;

        const activateIdentityRequestParams: {
            nonce: number;
            identity: LitentryPrimitivesIdentity;
        }[] = [];

        const bobEvmNonce = getNextNonce();
        const bobEvmIdentity = await buildIdentityHelper(context.ethersWallet.bob.address, 'Evm', context);

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
        const activatedIdentityEvents: any[] = [];

        for (const { nonce, identity } of activateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
            const deactivateIdentityCall = createSignedTrustedCallActivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                aliceEvmPair,
                aliceEvmSubject,
                identity.toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(
                context.tee,
                context.api,
                context.mrEnclave,
                teeShieldingKey,
                deactivateIdentityCall
            );

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

        assert.equal(activatedIdentityEvents.length, 2);

        await assertIdentity(context, aliceEvmPair, activatedIdentityEvents, [bobEvmIdentity, eveSubstrateIdentity]);
    });

    step('check idgraph from sidechain storage after activating', async function () {
        const idgraphGetter = createSignedTrustedGetterIdGraph(context.api, aliceEvmPair, aliceEvmSubject);
        const res = await sendRequestFromGetter(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            idgraphGetter
        );
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

    step('deactivating prime identity is disallowed', async function () {
        let currentNonce = (
            await getSidechainNonce(context.tee, context.api, context.mrEnclave, teeShieldingKey, aliceEvmSubject)
        ).toNumber();
        const getNextNonce = () => currentNonce++;
        const nonce = getNextNonce();

        // prime identity
        const substratePrimeIdentity = await buildIdentityHelper(u8aToHex(aliceEvmPair.addressRaw), 'Evm', context);

        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);
        const deactivateIdentityCall = createSignedTrustedCallDeactivateIdentity(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', nonce),
            aliceEvmPair,
            aliceEvmSubject,
            substratePrimeIdentity.toHex(),
            requestIdentifier
        );

        const res = await sendRequestFromTrustedCall(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            deactivateIdentityCall
        );
        assert.isTrue(res.do_watch.isFalse);
        assert.isTrue(res.status.asTrustedOperationStatus[0].isInvalid);

        const events = await eventsPromise;
        await assertFailedEvent(context, events, 'DeactivateIdentityFailed', 'DeactivatePrimeIdentityDisallowed');
    });
});
