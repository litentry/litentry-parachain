import { blake2AsHex, cryptoWaitReady } from '@polkadot/util-crypto';
import * as fs from 'fs';
import { Keyring, ApiPromise, WsProvider } from '@polkadot/api';
import { loadConfig, describeLitentry } from './utils';
import '@polkadot/wasm-crypto/initOnlyAsm';
import * as path from 'path';
import { expect } from 'chai';
import { step } from 'mocha-steps';

async function getRuntimeVersion(api: ApiPromise) {
    const runtime_version = await api.rpc.state.getRuntimeVersion();
    return +runtime_version['specVersion'];
}

async function runtimeUpgrade(api: ApiPromise, wasm: string) {
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const old_runtime_version = await getRuntimeVersion(api);
    let currentBlock = (await api.rpc.chain.getHeader()).number.toNumber();
    console.log(`Start doing runtime upgrade, current block = ${currentBlock}`);

    // authorize and enact the upgrade
    let nonce = (await api.rpc.system.accountNextIndex(alice.address)).toNumber();
    await api.tx.sudo
        .sudo(api.tx.parachainSystem.authorizeUpgrade(blake2AsHex(wasm)))
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

describeLitentry('Runtime upgrade test', ``, (context) => {
    step('Running runtime ugprade test', async function () {
        const wasmPath = path.resolve('/tmp/runtime.wasm');
        const wasm = fs.readFileSync(wasmPath).toString('hex');
        const runtimeVersion = await runtimeUpgrade(context.api, `0x${wasm}`);
        console.log(`result: ${runtimeVersion}`);
        expect(runtimeVersion === (await getRuntimeVersion(context.api)));
    });
});
