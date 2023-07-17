import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { hexToU8a } from '@polkadot/util';

import { buildIdentityFromKeypair, buildIdentityHelper, initIntegrationTestContext } from './common/utils';
import {
    createSignedTrustedCallSetUserShieldingKey,
    createSignedTrustedGetterUserShieldingKey,
    getSidechainNonce,
    getTeeShieldingKey,
    sendRequestFromGetter,
    sendRequestFromTrustedCall,
} from './examples/direct-invocation/util'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/type-definitions';
import { aesKey } from './common/call';
import { FrameSystemEventRecord } from 'parachain-api';

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

    step('check user sidechain storage before create', async function () {
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
        const shieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(
            context.api,
            context.substrateWallet.alice,
            aliceSubject
        );

        const shieldingKeyGetResult = await sendRequestFromGetter(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            shieldingKeyGetter
        );

        const k = context.api.createType('Option<Bytes>', hexToU8a(shieldingKeyGetResult.value.toHex()));
        assert.isTrue(k.isNone, 'shielding key should be empty before set');
    });

    step('Invalid user shielding key', async function () {
        const identity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);

        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
        const nonce = await getSidechainNonce(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            aliceSubject
        );

        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;

        const setUserShieldingKeyCall = createSignedTrustedCallSetUserShieldingKey(
            context.api,
            context.mrEnclave,
            nonce,
            context.substrateWallet.alice, // @FIXME: support EVM!!!
            aliceSubject,
            aesKey,
            requestIdentifier
        );

        const eventsPromise = new Promise<FrameSystemEventRecord[]>((resolve, reject) => {
            let blocksToScan = 30;

            const unsubscribe = context.api.rpc.chain.subscribeFinalizedHeads(async (blockHeader) => {
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
                        reject(
                            new Error(`timed out listening for reqExtHash: ${requestIdentifier} in parachain events`)
                        );
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

        const res = await sendRequestFromTrustedCall(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            setUserShieldingKeyCall
        );

        const events = await eventsPromise;
        events.forEach((event) => console.log(event.toHuman()));
    });
});
