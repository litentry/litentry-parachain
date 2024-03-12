import type { IntegrationTestContext } from './common-types';

import { FrameSystemEventRecord } from 'parachain-api';

// for DI-test
export const subscribeToEventsWithExtHash = async (
    requestIdentifier: string,
    context: IntegrationTestContext
): Promise<FrameSystemEventRecord[]> => {
    return new Promise<FrameSystemEventRecord[]>((resolve, reject) => {
        let blocksToScan = 30;
        /* 
        WARNING:The unsubscribe function is called inside the Promise callback, which is executed each time a new blockHeader is received. 
               `unsubscribe` is intended to unsubscribe a blockHeader if certain conditions are met. 
                If you use await, you will actually wait for this function to finish executing. 
                However, since it doesn't return a Promise, using await doesn't make sense and can lead to problematic code behaviour.
                soooooo, don't use await here
        */
        const unsubscribe = context.api.rpc.chain.subscribeNewHeads(async (blockHeader) => {
            const shiftedApi = await context.api.at(blockHeader.hash);

            const allBlockEvents = await shiftedApi.query.system.events();
            const allExtrinsicEvents = allBlockEvents.filter(({ phase }) => phase.isApplyExtrinsic);

            const matchingEvent = allExtrinsicEvents.filter((eventRecord) => {
                const eventData = eventRecord.event.data.toHuman();
                return (
                    eventData != undefined &&
                    typeof eventData === 'object' &&
                    'reqExtHash' in eventData &&
                    eventData.reqExtHash === requestIdentifier
                );
            });

            if (matchingEvent.length == 0) {
                blocksToScan -= 1;
                if (blocksToScan < 1) {
                    reject(new Error(`timed out listening for reqExtHash: ${requestIdentifier} in parachain events`));
                    (await unsubscribe)();
                }
                return;
            }

            resolve(matchingEvent);
            (await unsubscribe)();
        });
    });
};
