import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { hexToU8a } from '@polkadot/util';

import { buildIdentityFromKeypair, buildIdentityHelper, initIntegrationTestContext } from './common/utils';
import { assertInitialIdGraphCreated } from './common/utils/assertion'
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

const subscribeToEvents = async (requestIdentifier: string, context: IntegrationTestContext): Promise<FrameSystemEventRecord[]> => {
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
}


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


    ['alice', 'bob'].forEach((name) => {
        step('set user shielding key', async function () {
            const wallet = context.substrateWallet[name] // @FIXME: support EVM!!!
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

            assert.isTrue(res.status.isTrustedOperationStatus, `setUserShieldingKeyCall should be trusted operation status, but is ${res.status.type}`);
            const status = res.status.asTrustedOperationStatus;
            assert.isTrue(status.isSubmitted || status.isInSidechainBlock, `setUserShieldingKeyCall should be submitted or in sidechain block, but is ${status.type}`);

            const events = await eventsPromise;
            const userShieldingKeySetEvents = events.map(({ event }) => event).filter(({ section, method }) => section === 'identityManagement' && method === 'UserShieldingKeySet')

            await assertInitialIdGraphCreated(context, [wallet], userShieldingKeySetEvents);
        })
    });
});
