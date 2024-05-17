import { blake2AsHex, blake2AsU8a } from '@polkadot/util-crypto';
import * as fs from 'fs';
import { Keyring, ApiPromise } from '@polkadot/api';
import { describeLitentry } from '../common/utils/integration-setup';
import '@polkadot/wasm-crypto/initOnlyAsm';
import * as path from 'path';
import { expect } from 'chai';
import { step } from 'mocha-steps';
import { signAndSend, subscribeToEvents, observeEvent } from '../common/utils';
import { FrameSystemEventRecord } from '@polkadot/types/lookup';
async function getRuntimeVersion(api: ApiPromise) {
    const runtime_version = await api.rpc.state.getRuntimeVersion();
    return +runtime_version['specVersion'];
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

/// Pushes a polkadot runtime update via governance.
/// preimage -> council proposal -> vote -> democracy pass -> fast track => democracy proposal=>democracy vote => enactAuthorizedUpgrade.
async function runtimeupgradeViaGovernance(api: ApiPromise, wasm: string) {
    let eventsPromise: Promise<FrameSystemEventRecord[]>;
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    const old_runtime_version = await getRuntimeVersion(api);
    const encoded = api.tx.parachainSystem.authorizeUpgrade(blake2AsHex(wasm)).method.toHex();
    const proposalEncodedlHash = blake2AsHex(encoded);
    console.log(`Preimage hash: ${proposalEncodedlHash}`);

    // Submit the preimage (if it doesn't already exist)
    let preimageStatus = (await api.query.preimage.statusFor(proposalEncodedlHash)).toHuman();
    if (!preimageStatus) {
        eventsPromise = subscribeToEvents('preimage', 'Noted', api);
        const notePreimageTx = api.tx.preimage.notePreimage(encoded);
        await signAndSend(notePreimageTx, alice);
        const notePreimageEvent = (await eventsPromise).map(({ event }) => event);
        expect(notePreimageEvent.length === 1);
        console.log('Preimage noted ✅');
    }
    eventsPromise = subscribeToEvents('council', 'Proposed', api);
    const externalMotion = api.tx.democracy.externalProposeMajority({ Legacy: proposalEncodedlHash });
    console.log(`External motion length = ${externalMotion.encodedLength}`);

    // submit a council proposal
    const proposalTx = api.tx.council.propose(2, externalMotion, externalMotion.encodedLength);
    await signAndSend(proposalTx, alice);
    const proposalTxEvent = (await eventsPromise).map(({ event }) => event);
    expect(proposalTxEvent.length === 1);

    console.log('Council Proposed ✅');
    const proposalIndex = Number(proposalTxEvent[0].data[1].toHuman());

    // vote on the council proposal
    const voteTx = api.tx.council.vote(proposalEncodedlHash, proposalIndex, true);
    eventsPromise = subscribeToEvents('council', 'Voted', api);
    await signAndSend(voteTx, alice);
    await signAndSend(voteTx, bob);
    const voteTxEvent = (await eventsPromise).map(({ event }) => event);
    expect(voteTxEvent.length === 2);

    console.log('Alice Bob council Voted ✅');

    // close the council proposal
    const councilCloseTx = api.tx.council.close(
        proposalEncodedlHash,
        proposalIndex,
        {
            refTime: 1_000_000_000,
            proofSize: 1_000_000,
        },
        externalMotion.encodedLength
    );
    eventsPromise = subscribeToEvents('council', 'Closed', api);
    await signAndSend(councilCloseTx, alice);
    await signAndSend(councilCloseTx, bob);
    const councilCloseTxEvent = (await eventsPromise).map(({ event }) => event);
    expect(councilCloseTxEvent.length === 2);

    console.log('Alice Bob council Closed ✅');

    // fast track the democracy proposal
    const observeDemocracyStarted = observeEvent('democracy:Started', api);

    /**
     * @param proposal hash.
     * @param voting period - block number.
     * @param delay block number.
     */
    const democracyFastTrack = api.tx.democracy.fastTrack(proposalEncodedlHash, 15, 1);
    const techCommitteeProposalTx = api.tx.technicalCommittee.propose(
        1,
        democracyFastTrack,
        democracyFastTrack.encodedLength
    );
    eventsPromise = subscribeToEvents('technicalCommittee', 'Executed', api);
    await observeDemocracyStarted;
    await signAndSend(techCommitteeProposalTx, alice);

    const techCommitteeProposalEvent = (await eventsPromise).map(({ event }) => event);
    expect(techCommitteeProposalEvent.length === 1);

    console.log('Tech committee proposal executed ✅');
    const observeDemocracyPassed = observeEvent('democracy:Passed', api);
    const observeDemocracyNotPassed = observeEvent('democracy:NotPassed', api);
    const observeSchedulerDispatched = observeEvent('scheduler:Dispatched', api);
    const observeparachainSystemValidationFunctionStored = observeEvent(
        'parachainSystem:ValidationFunctionStored',
        api
    );
    const observeparachainSystemValidationFunctionApplied = observeEvent(
        'parachainSystem:ValidationFunctionApplied',
        api
    );
    // vote on the democracy proposal
    const referendumCount = (await api.query.democracy.referendumCount()).toNumber();
    const democracyVoteTx = api.tx.democracy.vote(referendumCount - 1, {
        Standard: { vote: true, balance: 1_00_000_000_000_000 },
    });

    eventsPromise = subscribeToEvents('democracy', 'Voted', api);
    await signAndSend(democracyVoteTx, alice);
    await signAndSend(democracyVoteTx, bob);
    const democracyVoteTxEvent = (await eventsPromise).map(({ event }) => event);
    expect(democracyVoteTxEvent.length === 2);
    console.log('Alice Bob democracy Voted ✅');

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

    console.log('Waiting for a succulent scheduled runtime update...');
    await observeSchedulerDispatched;
    const parachainSystemScheduleUpgradeTx = api.tx.parachainSystem.enactAuthorizedUpgrade(wasm);
    await signAndSend(parachainSystemScheduleUpgradeTx, alice);

    await Promise.all([
        observeparachainSystemValidationFunctionStored,
        observeparachainSystemValidationFunctionApplied,
    ]);
    const newRuntimeVersion = await waitForRuntimeUpgrade(api, old_runtime_version);
    console.log(`New runtime version = ${newRuntimeVersion}`);
    return newRuntimeVersion;
}
describeLitentry('Runtime upgrade test', ``, (context) => {
    step('Running runtime ugprade test', async function () {
        let runtimeVersion: number;
        const wasmPath = path.resolve('/tmp/runtime.wasm');
        const wasm = fs.readFileSync(wasmPath).toString('hex');

        runtimeVersion = await runtimeupgradeViaGovernance(context.api, `0x${wasm}`);

        expect(runtimeVersion === (await getRuntimeVersion(context.api)));

        console.log('Runtime upgraded ✅');
    });
});
