import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import {
    buildIdentityFromKeypair,
    buildIdentityHelper,
    buildValidations,
    checkIdGraph,
    initIntegrationTestContext,
} from './common/utils';
import { assertIdentityLinked, assertInitialIdGraphCreated } from './common/utils/assertion';
import {
    createSignedTrustedCallLinkIdentity,
    createSignedTrustedCallSetUserShieldingKey,
    createSignedTrustedGetterIdGraph,
    createSignedTrustedGetterUserShieldingKey,
    decodeIdGraph,
    getSidechainNonce,
    getTeeShieldingKey,
    sendRequestFromGetter,
    sendRequestFromTrustedCall,
} from './examples/direct-invocation/util'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/type-definitions';
import { aesKey, keyNonce } from './common/call';
import { FrameSystemEventRecord, LitentryValidationData, Web3Network } from 'parachain-api';
import { CorePrimitivesErrorErrorDetail } from 'parachain-api';
import { LitentryPrimitivesIdentity } from 'sidechain-api';
import { Vec } from '@polkadot/types';

const subscribeToEvents = async (
    requestIdentifier: string,
    context: IntegrationTestContext
): Promise<FrameSystemEventRecord[]> => {
    return new Promise<FrameSystemEventRecord[]>((resolve, reject) => {
        let blocksToScan = 30;

        const unsubscribe = context.api.rpc.chain.subscribeNewHeads(async (blockHeader) => {
            const shiftedApi = await context.api.at(blockHeader.hash);

            const allBlockEvents = await shiftedApi.query.system.events();
            const allExtrinsicEvents = allBlockEvents.filter(({ phase }) => phase.isApplyExtrinsic);

            const matchingEvent = allExtrinsicEvents.find((eventRecord) => {
                const eventData = eventRecord.event.data.toHuman();
                /**
                 * @FIXME I'd love a cleaner way to do this check :P
                 */

                return (
                    eventData != undefined &&
                    typeof eventData === 'object' &&
                    'reqExtHash' in eventData &&
                    eventData.reqExtHash === requestIdentifier
                );
            });

            if (matchingEvent == undefined) {
                blocksToScan -= 1;
                if (blocksToScan < 1) {
                    reject(new Error(`timed out listening for reqExtHash: ${requestIdentifier} in parachain events`));
                    (await unsubscribe)();
                }
                return;
            }

            const extrinsicIndex = matchingEvent.phase.asApplyExtrinsic;
            const requestEvents = allExtrinsicEvents.filter((eventRecord) =>
                eventRecord.phase.asApplyExtrinsic.eq(extrinsicIndex)
            );

            resolve(requestEvents);
            (await unsubscribe)();
        });
    });
};

describe('Test Identity (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_END_POINT!, // @fixme evil assertion; centralize env access
            process.env.SUBSTRATE_END_POINT!, // @fixme evil assertion; centralize env access
            0
        );
        teeShieldingKey = await getTeeShieldingKey(context.tee, context.api);
    });

    it('needs a lot more work to be complete');

    // step('check user sidechain storage before create', async function () {
    //     const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
    //     const shieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(
    //         context.api,
    //         context.substrateWallet.alice,
    //         aliceSubject
    //     );

    //     const shieldingKeyGetResult = await sendRequestFromGetter(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         shieldingKeyGetter
    //     );

    //     const k = context.api.createType('Option<Bytes>', hexToU8a(shieldingKeyGetResult.value.toHex()));
    //     assert.isTrue(k.isNone, 'shielding key should be empty before set');
    // });

    // step('Invalid user shielding key', async function () {
    //     const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
    //     const bobSubstrateIdentity = await buildIdentityHelper(
    //         u8aToHex(context.substrateWallet.bob.addressRaw),
    //         'Substrate',
    //         context
    //     );
    //     const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;

    //     const nonce = await getSidechainNonce(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         aliceSubject
    //     );
    //     const [bobValidationData] = await buildValidations(
    //         context,
    //         [aliceSubject],
    //         [bobSubstrateIdentity],
    //         nonce.toNumber(),
    //         'substrate',
    //         context.substrateWallet.bob
    //     );
    //     const eventsPromise = subscribeToEvents(requestIdentifier, context);

    //     const linkIdentityCall = createSignedTrustedCallLinkIdentity(
    //         context.api,
    //         context.mrEnclave,
    //         nonce,
    //         context.substrateWallet.alice,
    //         aliceSubject,
    //         context.sidechainRegistry.createType('LitentryPrimitivesIdentity', bobSubstrateIdentity).toHex(),
    //         context.api.createType('LitentryValidationData', bobValidationData).toHex(),
    //         context.api.createType('Vec<Web3Network>', ['Polkadot', 'Litentry']).toHex(),
    //         keyNonce,
    //         requestIdentifier
    //     );

    //     const res = await sendRequestFromTrustedCall(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         linkIdentityCall
    //     );
    //     assert.isTrue(
    //         res.status.isTrustedOperationStatus,
    //         `linkIdentityCall should be trusted operation status, but is ${res.status.type}`
    //     );
    //     const status = res.status.asTrustedOperationStatus;
    //     assert.isTrue(
    //         status.isSubmitted || status.isInSidechainBlock,
    //         `linkIdentityCall should be submitted or in sidechain block, but is ${status.type}`
    //     );

    //     const events = await eventsPromise;

    //     const linkIdentityFailed = context.api.events.identityManagement.LinkIdentityFailed;

    //     const isLinkIdentityFailed = linkIdentityFailed.is.bind(linkIdentityFailed);
    //     type EventLike = Parameters<typeof isLinkIdentityFailed>[0];
    //     const ievents: EventLike[] = events.map(({ event }) => event);
    //     const linkIdentityFailedEvents = ievents.filter(isLinkIdentityFailed);

    //     assert.lengthOf(linkIdentityFailedEvents, 1);
    //     /**
    //      * @fixme tsc is STILL not seeing the correct type for these events, WTF!?!?!?!?
    //      */
    //     assert.equal(
    //         (linkIdentityFailedEvents[0].data[1] as CorePrimitivesErrorErrorDetail).type,
    //         'UserShieldingKeyNotFound',
    //         'check linkIdentityFailedEvent detail is UserShieldingKeyNotFound, but is not'
    //     );
    // });

    ['alice', 'bob'].forEach((name) => {
        step(`set user shielding key (${name})`, async function () {
            const wallet = context.substrateWallet[name]; // @FIXME: support EVM!!!
            const subject = await buildIdentityFromKeypair(wallet, context);
            const nonce = await getSidechainNonce(
                context.tee,
                context.api,
                context.mrEnclave,
                teeShieldingKey,
                subject
            );

            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;

            const setUserShieldingKeyCall = createSignedTrustedCallSetUserShieldingKey(
                context.api,
                context.mrEnclave,
                nonce,
                wallet,
                subject,
                aesKey,
                requestIdentifier
            );

            const eventsPromise = subscribeToEvents(requestIdentifier, context);
            const res = await sendRequestFromTrustedCall(
                context.tee,
                context.api,
                context.mrEnclave,
                teeShieldingKey,
                setUserShieldingKeyCall
            );

            assert.isTrue(
                res.status.isTrustedOperationStatus,
                `setUserShieldingKeyCall should be trusted operation status, but is ${res.status.type}`
            );
            const status = res.status.asTrustedOperationStatus;
            assert.isTrue(
                status.isSubmitted || status.isInSidechainBlock,
                `setUserShieldingKeyCall should be submitted or in sidechain block, but is ${status.type}`
            );

            const events = await eventsPromise;
            const userShieldingKeySetEvents = events
                .map(({ event }) => event)
                .filter(({ section, method }) => section === 'identityManagement' && method === 'UserShieldingKeySet');

            await assertInitialIdGraphCreated(context, [wallet], userShieldingKeySetEvents);
        });
    });

    // step('check user shielding key from sidechain storage after setUserShieldingKey', async function () {
    //     const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
    //     const shieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(
    //         context.api,
    //         context.substrateWallet.alice,
    //         aliceSubject
    //     );

    //     const shieldingKeyGetResult = await sendRequestFromGetter(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         shieldingKeyGetter
    //     );

    //     const k = context.api.createType('Option<Bytes>', hexToU8a(shieldingKeyGetResult.value.toHex()));
    //     assert.equal(k.value.toString(), aesKey, 'respShieldingKey should be equal aesKey after set');
    // });

    // step('check idgraph from sidechain storage before linking', async function () {
    //     const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);

    //     const idgraphGetter = createSignedTrustedGetterIdGraph(
    //         context.api,
    //         context.substrateWallet.alice,
    //         aliceSubject
    //     );
    //     const res = await sendRequestFromGetter(
    //         context.tee,
    //         context.api,
    //         context.mrEnclave,
    //         teeShieldingKey,
    //         idgraphGetter
    //     );

    //     const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);
    //     assert.lengthOf(idGraph, 1);
    //     const [idGraphNodeIdentity, idGraphNodeContext] = idGraph[0];
    //     assert.deepEqual(idGraphNodeIdentity.toHuman(), aliceSubject.toHuman(), 'idGraph should include main address');
    //     assert.equal(idGraphNodeContext.status.toString(), 'Active', 'status should be active for main address');
    // });

    step('link identities (alice)', async function () {
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
        const getNextNonce = async () =>
            await getSidechainNonce(context.tee, context.api, context.mrEnclave, teeShieldingKey, aliceSubject);

        // Alice links:
        // - a `mock_user` twitter
        // - alice's evm identity
        // - eve's substrate identity (as alice can't link her own substrate again)
        const linkIdentityRequestParams: {
            identity: LitentryPrimitivesIdentity;
            validation: LitentryValidationData;
            networks: Vec<Web3Network>;
        }[] = [];

        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const [twitterValidation] = await buildValidations(
            context,
            [aliceSubject],
            [twitterIdentity],
            (await getNextNonce()).toNumber(),
            'twitter'
        );
        const twitterNetworks = context.api.createType('Vec<Web3Network>', []);
        linkIdentityRequestParams.push({
            identity: twitterIdentity,
            validation: twitterValidation,
            networks: twitterNetworks,
        });

        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
        const [evmValidation] = await buildValidations(
            context,
            [aliceSubject],
            [evmIdentity],
            (await getNextNonce()).toNumber() + 1,
            'ethereum',
            undefined,
            [context.ethersWallet.alice]
        );
        const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);
        linkIdentityRequestParams.push({
            identity: evmIdentity,
            validation: evmValidation,
            networks: evmNetworks,
        });

        const eveSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.eve.addressRaw),
            'Substrate',
            context
        );
        const [eveSubstrateValidation] = await buildValidations(
            context,
            [aliceSubject],
            [eveSubstrateIdentity],
            (await getNextNonce()).toNumber() + 2,
            'substrate',
            context.substrateWallet.eve
        );
        const eveSubstrateNetworks = context.api.createType('Vec<Web3Network>', ['Litentry', 'Polkadot']);
        linkIdentityRequestParams.push({
            identity: eveSubstrateIdentity,
            validation: eveSubstrateValidation,
            networks: eveSubstrateNetworks,
        });

        for (const { identity, validation, networks } of linkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEvents(requestIdentifier, context);
            const linkIdentityCall = createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                await getNextNonce(),
                context.substrateWallet.alice,
                aliceSubject,
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
            assert.isTrue(
                res.status.isTrustedOperationStatus,
                `linkIdentityCall should be trusted operation status, but is ${res.status.type}`
            );
            const status = res.status.asTrustedOperationStatus;
            assert.isTrue(
                status.isSubmitted || status.isInSidechainBlock,
                `linkIdentityCall should be submitted or in sidechain block, but is ${status.type}`
            );

            const events = await eventsPromise;

            // check events
            // events.forEach(({ event }) => {
            //     if (event.section === 'identityManagement' && event.method === 'IdentityLinked') {
            //         assertIdentityLinked(context, context.substrateWallet.alice, [event], [identity]);
            //     }
            // });
        }
    });

    step('check idgraph from sidechain storage after linking', async function () {
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
        const idgraphGetter = createSignedTrustedGetterIdGraph(
            context.api,
            context.substrateWallet.alice,
            aliceSubject
        );
        const res = await sendRequestFromGetter(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            idgraphGetter
        );

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // the main address should be already inside the IDGraph
        const mainIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'Substrate',
            context
        );
        const identityHex = mainIdentity.toHex();
        const respIdGraph = await checkIdGraph(context, 'IdentityManagement', 'IDGraphs', aliceSubject, identityHex);
        assert.isTrue(respIdGraph.linkBlock.toNumber() > 0, 'linkBlock should be greater than 0 for main address');
        assert.isTrue(respIdGraph.status.isActive, 'status should be active for main address');

        // assert.lengthOf(idGraph, 1);
        // const [idGraphNodeIdentity, idGraphNodeContext] = idGraph[0];
        // assert.deepEqual(idGraphNodeIdentity.toHuman(), aliceSubject.toHuman(), 'idGraph should include main address');
        // assert.equal(idGraphNodeContext.status.toString(), 'Active', 'status should be active for main address');

    });
});
