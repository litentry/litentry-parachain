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
import { AccountId, Index } from '@polkadot/types/interfaces';
import { Vec } from '@polkadot/types';
import { KeyringPair } from '@polkadot/keyring/types';

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

const getCouncilThreshold = async (api: ApiPromise): Promise<number> => {
    const members = (await api.query.councilMembership.members()) as Vec<AccountId>;
    return Math.ceil(members.length / 2);
};
async function runtimeupgradeViaGovernance(api: ApiPromise, wasm: string) {
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const old_runtime_version = await getRuntimeVersion(api);
    console.log(`Old runtime version = ${old_runtime_version}`);
    let currentBlock = (await api.rpc.chain.getHeader()).number.toNumber();

    const encoded = api.tx.parachainSystem.authorizeUpgrade(blake2AsHex(wasm)).method.toHex();
    const encodedHash = blake2AsHex(encoded);
    const external = api.tx.democracy.externalProposeMajority({ Legacy: encodedHash });

    let preimageStatus = (await api.query.preimage.statusFor(encodedHash)).toHuman();

    if (!preimageStatus) {
        let eventsPromise: Promise<FrameSystemEventRecord[]>;
        eventsPromise = subscribeToEvents('preimage', 'notePreimage', api);
        const notePreimageTx = api.tx.preimage.notePreimage(encoded);
        await notePreimageTx.signAndSend(alice, { nonce: -1 });
        const notePreimageEvent = (await eventsPromise).map(({ event }) => event);
        expect(notePreimageEvent.length === 1);
        console.log('Preimage noted');
    }

    const proposalTx = api.tx.council.propose(await getCouncilThreshold(api), external, external.length);
    await proposalTx.signAndSend(alice, { nonce: -1 });

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
        console.log('Running runtime upgrade test---------');
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
