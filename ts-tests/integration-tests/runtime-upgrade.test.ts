import { blake2AsHex, blake2AsU8a } from '@polkadot/util-crypto';
import * as fs from 'fs';
import { Keyring, ApiPromise, WsProvider } from '@polkadot/api';
import { describeLitentry } from '../common/utils/integration-setup';
import '@polkadot/wasm-crypto/initOnlyAsm';
import * as path from 'path';
import { expect } from 'chai';
import { step } from 'mocha-steps';
import { setTimeout as sleep } from 'timers/promises';
import { subscribeToEvents } from '../common/utils';
import { FrameSystemEventRecord } from '@polkadot/types/lookup';
const BN = require('bn.js');
const bn1e12 = new BN(10).pow(new BN(12)).mul(new BN(1));

async function getRuntimeVersion(api: ApiPromise) {
    const runtime_version = await api.rpc.state.getRuntimeVersion();
    return +runtime_version['specVersion'];
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
    let timeoutBlock = currentBlock + 10;
    let runtimeUpgraded = false;

    return new Promise(async (resolve, reject) => {
        const unsub = await api.rpc.chain.subscribeNewHeads(async (header) => {
            console.log(`Polling .. block = ${header.number.toNumber()}`);
            const runtime_version = await getRuntimeVersion(api);
            if (!runtimeUpgraded) {
                if (runtime_version > old_runtime_version) {
                    runtimeUpgraded = true;
                    console.log(
                        `Runtime upgrade OK, new runtime version = ${runtime_version}, waiting for 2 more blocks ...`
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
                    resolve(runtime_version);
                }
            }
        });
    });
}

type EventQuery = (data: any) => boolean;
type Event = { name: any; data: any; block: number; event_index: number };

export async function observeEvent(
    eventName: string,
    api: ApiPromise,
    eventQuery?: EventQuery,
    stopObserveEvent?: () => boolean,
    finalized = false
): Promise<Event> {
    let result: Event | undefined;
    let eventFound = false;
    const query = eventQuery ?? (() => true);
    const stopObserve = stopObserveEvent ?? (() => false);
    const [expectedSection, expectedMethod] = eventName.split(':');
    const subscribeMethod = finalized ? api.rpc.chain.subscribeFinalizedHeads : api.rpc.chain.subscribeNewHeads;
    const unsubscribe = await subscribeMethod(async (header) => {
        const events = await api.query.system.events.at(header.hash);
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
    while (!eventFound && !stopObserve()) {
        await sleep(1000);
    }
    return result as Event;
}
/// Pushes a polkadot runtime update using the democracy pallet.
/// preimage -> proposal -> vote -> democracy pass -> scheduler dispatch runtime update.
const proposalAmount = bn1e12;
async function runtimeUpgradeWithoutSudo(api: ApiPromise, wasm: string) {
    console.log('Starting runtime upgrade without sudo');
    const old_runtime_version = await getRuntimeVersion(api);
    console.log(`Old runtime version = ${old_runtime_version}`);
    let eventsPromise: Promise<FrameSystemEventRecord[]>;
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const setCodeCall = api.tx.system.setCode(wasm);
    const preimage = setCodeCall.method.toHex();
    console.log('preimage', preimage);
    const preimageHash = '0x' + Buffer.from(blake2AsU8a(preimage)).toString('hex');
    console.log(`Preimage hash: ${preimageHash}`);
    let preimageStatus = (await api.query.preimage.statusFor(preimageHash)) as any;
    if (JSON.stringify(preimageStatus) !== 'null') {
        preimageStatus = JSON.parse(preimageStatus);
        if (!preimageStatus?.unrequested && !preimageStatus?.requested) {
            throw new Error('Invalid preimage status');
        }
    }
    if (preimageStatus?.unrequested?.len > 0 || preimageStatus?.requested?.len > 0) {
        console.log('Preimage already exists, skipping submission');
    } else {
        await api.tx.preimage.notePreimage(preimage).signAndSend(alice, { nonce: -1 }),
            console.log(`Preimage submitted: ${preimageHash}`);
    }

    const observeDemocracyStarted = observeEvent('democracy:Started', api);

    console.log('observeDemocracyStarted', observeDemocracyStarted);
    eventsPromise = subscribeToEvents('democracy', 'Voted', api);
    const democracyStartedEvent = (await eventsPromise).map(({ event }) => event);
    console.log('events[0].toHuman()', democracyStartedEvent[0].toHuman());

    // Vote for the proposal
    const observeDemocracyPassed = observeEvent('democracy:Passed', api);
    const observeDemocracyNotPassed = observeEvent('democracy:NotPassed', api);
    const observeSchedulerDispatched = observeEvent('scheduler:Dispatched', api);
    const observeCodeUpdated = observeEvent('system:CodeUpdated', api);
    const vote = { Standard: { vote: true, balance: proposalAmount } };
    eventsPromise = subscribeToEvents('democracy', 'Voted', api);
    const proposalIndex = democracyStartedEvent[0].data[0];
    console.log('proposalIndex.toHuman()', proposalIndex.toHuman());
    await api.tx.democracy.vote(api.createType('Compact<u32>', proposalIndex), vote).signAndSend(alice, { nonce: -1 });
    console.log('Democracy manifest! waiting for a succulent scheduled runtime update...');

    // Wait for the runtime update to complete
    const schedulerDispatchedEvent = await observeSchedulerDispatched;
    if (schedulerDispatchedEvent.data.result.Err) {
        console.log('Runtime update failed');
        process.exit(-1);
    }

    console.log(`Scheduler dispatched Runtime update at block ${schedulerDispatchedEvent.block}`);

    const CodeUpdated = await observeCodeUpdated;
    console.log(`Code updated at block ${CodeUpdated.block}`);

    console.log('-- Polkadot runtime update complete --');

    let currentBlock = (await api.rpc.chain.getHeader()).number.toNumber();
    let timeoutBlock = currentBlock + 10;
    let runtimeUpgraded = false;
    return new Promise(async (resolve, reject) => {
        const unsub = await api.rpc.chain.subscribeNewHeads(async (header) => {
            console.log(`Polling .. block = ${header.number.toNumber()}`);
            const runtime_version = await getRuntimeVersion(api);
            if (!runtimeUpgraded) {
                if (runtime_version > old_runtime_version) {
                    runtimeUpgraded = true;
                    console.log(
                        `Runtime upgrade OK, new runtime version = ${runtime_version}, waiting for 2 more blocks ...`
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
                    resolve(runtime_version);
                }
            }
        });
    });
}

describeLitentry('Runtime upgrade test', ``, (context) => {
    step('Running runtime ugprade test', async function () {
        console.log('Running runtime upgrade test---------');
        const wasmPath = path.resolve('/tmp/runtime.wasm');
        console.log(`wasmPath: ${wasmPath}`);
        const wasm = fs.readFileSync(wasmPath).toString('hex');
        const runtimeVersion = await runtimeUpgradeWithoutSudo(context.api, `0x${wasm}`);
        console.log(`result: ${runtimeVersion}`);
        expect(runtimeVersion === (await getRuntimeVersion(context.api)));
    });
});
