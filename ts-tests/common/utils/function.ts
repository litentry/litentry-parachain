import { AddressOrPair, ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import { ApiPromise } from '@polkadot/api';
import { FrameSystemEventRecord } from '@polkadot/types/lookup';

export function sleep(secs: number) {
    return new Promise((resolve) => {
        setTimeout(resolve, secs * 1000);
    });
}

export function signAndSend(tx: SubmittableExtrinsic<ApiTypes>, account: AddressOrPair) {
    return new Promise<{ block: string }>(async (resolve, reject) => {
        await tx.signAndSend(account, (result) => {
            console.log(`Current status is ${result.status}`);
            if (result.status.isInBlock) {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
            } else if (result.status.isFinalized) {
                console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
                resolve({
                    block: result.status.asFinalized.toString(),
                });
            } else if (result.status.isInvalid) {
                reject(`Transaction is ${result.status}`);
            }
        });
    });
}

// After removing the sudo module, we use `EnsureRootOrHalfTechnicalCommittee` instead of `Sudo`,
// and there are only one council members in litmus-dev/rococo-dev/litentry-dev.
// So only `propose` is required, no vote.
//
// TODO: support to send the `vote extrinsic`, if the number of council members is greater than 2.
export async function sudoWrapperTC(api: ApiPromise, tx: SubmittableExtrinsic<ApiTypes>) {
    const chain = (await api.rpc.system.chain()).toString().toLowerCase();
    if (chain != 'rococo-dev') {
        const threshold = api.createType('Compact<u32>', 1);
        const call = api.createType('Call', tx);
        return api.tx.technicalCommittee.propose(threshold, call, api.createType('Compact<u32>', tx.length));
    } else {
        return api.tx.sudo.sudo(tx);
    }
}

// After removing the sudo module, we use `EnsureRootOrHalfCouncil` instead of `Sudo`,
// and there are only two council members in litmus-dev/rococo-dev/litentry-dev.
// So only `propose` is required, no vote.
//
// TODO: support to send the `vote extrinsic`, if the number of council members is greater than 2.
export async function sudoWrapperGC(api: ApiPromise, tx: SubmittableExtrinsic<ApiTypes>) {
    const chain = (await api.rpc.system.chain()).toString().toLowerCase();
    if (chain != 'rococo-dev') {
        const threshold = api.createType('Compact<u32>', 1);
        const call = api.createType('Call', tx);
        return api.tx.council.propose(threshold, call, api.createType('Compact<u32>', tx.length));
    } else {
        return api.tx.sudo.sudo(tx);
    }
}

export const subscribeToEvents = async (
    section: string,
    method: string,
    api: ApiPromise
): Promise<FrameSystemEventRecord[]> => {
    return new Promise<FrameSystemEventRecord[]>((resolve, reject) => {
        let blocksToScan = 15;
        const unsubscribe = api.rpc.chain.subscribeNewHeads(async (blockHeader) => {
            const shiftedApi = await api.at(blockHeader.hash);

            const allBlockEvents = await shiftedApi.query.system.events();
            const allExtrinsicEvents = allBlockEvents.filter(({ phase }) => phase.isApplyExtrinsic);

            const matchingEvent = allExtrinsicEvents.filter(({ event, phase }) => {
                return event.section === section && event.method === method;
            });

            if (matchingEvent.length == 0) {
                blocksToScan -= 1;
                if (blocksToScan < 1) {
                    reject(new Error(`timed out listening for event ${section}.${method}`));
                    (await unsubscribe)();
                }
                return;
            }

            resolve(matchingEvent);
            (await unsubscribe)();
        });
    });
};

type EventQuery = (data: any) => boolean;
export type Event = { name: any; data: any; block: number; event_index: number };
export async function observeEvent(
    eventName: string,
    api: ApiPromise,
    eventQuery?: EventQuery,
    stopObserveEvent?: () => boolean,
    finalized = false,
    maxWaitTime = 120 // Maximum wait time in seconds (2 minutes)
): Promise<Event> {
    let result: Event | undefined;
    let eventFound = false;
    let waitTime = 0;

    const query = eventQuery ?? (() => true);
    const stopObserve = stopObserveEvent ?? (() => false);

    const [expectedSection, expectedMethod] = eventName.split(':');

    const subscribeMethod = finalized ? api.rpc.chain.subscribeFinalizedHeads : api.rpc.chain.subscribeNewHeads;

    const unsubscribe: any = await subscribeMethod(async (header) => {
        const events: any[] = await api.query.system.events.at(header.hash);
        events.forEach((record, index) => {
            const { event } = record;
            if (!eventFound && event.section.includes(expectedSection) && event.method.includes(expectedMethod)) {
                const expectedEvent = {
                    name: { section: event.section, method: event.method },
                    data: event.toHuman().data,
                    block: header.number.toNumber(),
                    event_index: index,
                };
                if (query(expectedEvent)) {
                    result = expectedEvent;
                    eventFound = true;
                    unsubscribe();
                }
            }
        });
    });

    while (!eventFound && !stopObserve() && waitTime < maxWaitTime) {
        await sleep(1000);
        waitTime++;
    }

    if (!eventFound && waitTime >= maxWaitTime) {
        throw new Error('Event not found within the specified time limit');
    }
    return result as Event;
}
