import { blake2AsHex, blake2AsU8a } from '@polkadot/util-crypto';
import * as fs from 'fs';
import { Keyring, ApiPromise } from '@polkadot/api';
import { describeLitentry } from '../common/utils/integration-setup';
import '@polkadot/wasm-crypto/initOnlyAsm';
import * as path from 'path';
import { expect } from 'chai';
import { step } from 'mocha-steps';
import { setTimeout as sleep } from 'timers/promises';
import { signAndSend, subscribeToEvents } from '../common/utils';
import { FrameSystemEventRecord } from '@polkadot/types/lookup';
async function getRuntimeVersion(api: ApiPromise) {
    const runtime_version = await api.rpc.state.getRuntimeVersion();
    return +runtime_version['specVersion'];
}

type EventQuery = (data: any) => boolean;
export type Event = { name: any; data: any; block: number; event_index: number };
export async function observeEvent(
    eventName: string,
    api: ApiPromise,
    eventQuery?: EventQuery,
    stopObserveEvent?: () => boolean,
    finalized = false,
    maxWaitTime = 360 // Maximum wait time in seconds (6 minutes)
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
async function waitForRuntimeUpgrade(api: ApiPromise, oldRuntimeVersion: number): Promise<number> {
    return new Promise(async (resolve, reject) => {
        let runtimeUpgraded = false;
        let timeoutBlock = (await api.rpc.chain.getHeader()).number.toNumber() + 10;

        const unsub = await api.rpc.chain.subscribeNewHeads(async (header) => {
            console.log(`Polling .. block = ${header.number.toNumber()}`);
            const runtimeVersion = await getRuntimeVersion(api);
            if (!runtimeUpgraded) {
                if (runtimeVersion > oldRuntimeVersion) {
                    runtimeUpgraded = true;
                    console.log(
                        `Runtime upgrade OK, new runtime version = ${runtimeVersion}, waiting for 2 more blocks ...`
                    );
                    timeoutBlock = header.number.toNumber() + 2;
                }
            }
            if (header.number.toNumber() == timeoutBlock) {
                unsub();
                if (!runtimeUpgraded) {
                    reject('Runtime upgrade failed with timeout');
                } else {
                    console.log('All good');
                    resolve(runtimeVersion);
                }
            }
        });
    });
}

async function runtimeUpgradeWithSudo(api: ApiPromise, wasm: string) {
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const old_runtime_version = await getRuntimeVersion(api);
    let currentBlock = (await api.rpc.chain.getHeader()).number.toNumber();
    console.log(`Start doing runtime upgrade, current block = ${currentBlock}`);

    // authorize and enact the upgrade
    await api.tx.sudo
        .sudo(api.tx.parachainSystem.authorizeUpgrade(blake2AsHex(wasm), true))
        .signAndSend(alice, { nonce: -1 });
    console.log('Submitted authorizeUpgrade');
    await api.tx.parachainSystem.enactAuthorizedUpgrade(wasm).signAndSend(alice, { nonce: -1 });
    console.log('Submitted enactAuthorizedUpgrade');

    // wait for 10 blocks
    console.log('Wait for new runtime ...');
    currentBlock = (await api.rpc.chain.getHeader()).number.toNumber();
    // wait for runtime upgrade
    const newRuntimeVersion = await waitForRuntimeUpgrade(api, old_runtime_version);
    console.log(`New runtime version = ${newRuntimeVersion}`);
    return newRuntimeVersion;
}

/// Pushes a polkadot runtime update using the democracy pallet.
/// preimage -> proposal -> vote -> democracy pass -> scheduler dispatch runtime update.
async function runtimeupgradeViaGovernance(api: ApiPromise, wasm: string) {
    const launchPeriod = api.consts.democracy.launchPeriod.toNumber();
    console.log(`Launch period = ${launchPeriod}`);

    let eventsPromise: Promise<FrameSystemEventRecord[]>;
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const old_runtime_version = await getRuntimeVersion(api);
    console.log(`Old runtime version = ${old_runtime_version}`);
    let currentBlock = (await api.rpc.chain.getHeader()).number.toNumber();
    const encoded = api.tx.system.setCode(blake2AsHex(wasm)).method.toHex();
    const preimageHash = blake2AsHex(encoded);
    console.log(`Preimage hash: ${preimageHash}`);

    // Submit the preimage (if it doesn't already exist)
    let preimageStatus = (await api.query.preimage.statusFor(preimageHash)).toHuman();
    if (!preimageStatus) {
        eventsPromise = subscribeToEvents('preimage', 'Noted', api);
        const notePreimageTx = api.tx.preimage.notePreimage(encoded);
        await signAndSend(notePreimageTx, alice);
        const notePreimageEvent = (await eventsPromise).map(({ event }) => event);
        expect(notePreimageEvent.length === 1);
        console.log('Preimage noted');
    }

    // Submit the proposal
    const observeDemocracyStarted = observeEvent('democracy:Started', api);
    eventsPromise = subscribeToEvents('democracy', 'Proposed', api);
    const proposalTx = api.tx.democracy.propose({ Legacy: preimageHash }, 100000000000000);
    await signAndSend(proposalTx, alice);
    const democracyStartedEvent = (await eventsPromise).map(({ event }) => event);
    expect(democracyStartedEvent.length === 1);
    console.log('Democracy proposal started', democracyStartedEvent[0].data[0].toHuman());
    const proposalIndex = api.createType('ProposalIndex', democracyStartedEvent[0].data[0].toHuman());

    // Wait for the democracy started event
    console.log('Waiting for voting to start...');
    await observeDemocracyStarted;
    const observeDemocracyPassed = observeEvent('democracy:Passed', api);
    const observeDemocracyNotPassed = observeEvent('democracy:NotPassed', api);
    const observeSchedulerDispatched = observeEvent('scheduler:Dispatched', api);
    const observeCodeUpdated = observeEvent('system:CodeUpdated', api);

    // Vote for the proposal
    eventsPromise = subscribeToEvents('democracy', 'Voted', api);
    const vote = { Standard: { vote: true, balance: 100000000000000 } };

    // change proposalIndex to refrenum index
    const voteTx = api.tx.democracy.vote(proposalIndex, vote);
    await signAndSend(voteTx, alice);
    const democracyVotedEvent = (await eventsPromise).map(({ event }) => event);
    expect(democracyVotedEvent.length === 1);
    console.log('Democracy proposal voted', democracyVotedEvent[0].data[0].toHuman());

    // Wait for it to pass
    await Promise.race([observeDemocracyPassed, observeDemocracyNotPassed])
        .then((event) => {
            if (event.name.method !== 'Passed') {
                throw new Error(`Democracy failed for runtime update. ${proposalIndex}`);
            }
        })
        .catch((error) => {
            console.error(error);
            process.exit(-1);
        });

    console.log('Democracy manifest! waiting for a succulent scheduled runtime update...');

    // Wait for the runtime update to complete
    await observeSchedulerDispatched;
    const CodeUpdated = await observeCodeUpdated;
    console.log(`Code updated at block ${CodeUpdated.block}`);

    // wait for 10 blocks
    console.log('Wait for new runtime ...');
    currentBlock = (await api.rpc.chain.getHeader()).number.toNumber();
    const newRuntimeVersion = await waitForRuntimeUpgrade(api, old_runtime_version);
    console.log(`New runtime version = ${newRuntimeVersion}`);
    return newRuntimeVersion;
}
describeLitentry('Runtime upgrade test', ``, (context) => {
    const network = process.argv[7].slice(2);
    console.log('Running runtime upgrade test on network:', network);
    step('Running runtime ugprade test', async function () {
        let runtimeVersion: number;
        const wasmPath = path.resolve('/tmp/runtime.wasm');
        console.log(`wasmPath: ${wasmPath}`);
        const wasm = fs.readFileSync(wasmPath).toString('hex');
        if (network === 'rococo' || network === 'litentry') {
            runtimeVersion = await runtimeUpgradeWithSudo(context.api, `0x${wasm}`);
        } else if (network === 'litmus') {
            runtimeVersion = await runtimeupgradeViaGovernance(context.api, `0x${wasm}`);
        } else {
            throw new Error('Network error');
        }
        console.log(`result: ${runtimeVersion}`);
        expect(runtimeVersion === (await getRuntimeVersion(context.api)));
    });
});
