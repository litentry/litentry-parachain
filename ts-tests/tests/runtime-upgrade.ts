//simulate runtime upgrade

import {blake2AsHex, cryptoWaitReady} from '@polkadot/util-crypto'
import * as fs from 'fs'
import {KeyringPair} from '@polkadot/keyring/types'
import {ISubmittableResult} from '@polkadot/types/types'
import {Keyring, ApiPromise, WsProvider} from '@polkadot/api'
import {loadConfig, signAndSend} from './utils';
import '@polkadot/wasm-crypto/initOnlyAsm';
import * as path from 'path';

const keyring = new Keyring({type: 'sr25519'})

export function sleep(ms: number) {
    return new Promise((resolve) => {
        setTimeout(resolve, ms)
    })
}

async function authorizeUpgrade(admin: KeyringPair, codeHash: string) {
    const config = loadConfig()
    const provider = new WsProvider(config.parachain_ws)
    const api = await ApiPromise.create({
        provider: provider,
    });

    await api.tx.sudo
        .sudo(api.tx.parachainSystem.authorizeUpgrade(codeHash))
        .signAndSend(admin, ({events, status}: ISubmittableResult) => {
            console.log('Proposal status:', status.type)
            if (status.isInBlock) {
                console.error('You have just upgraded your chain')
                console.log('Included at block hash', status.asInBlock.toHex())
                console.log('Events:')
                // @ts-ignore
                console.log(JSON.stringify(events.toString(), null, 2))
            } else if (status.isFinalized) {
                console.log('Finalized block hash', status.asFinalized.toHex())
            }
        })
}

async function upgrade(admin: KeyringPair, code: string) {

    const config = loadConfig()
    const provider = new WsProvider(config.parachain_ws)
    const api = await ApiPromise.create({
        provider: provider,
    });

    const proposal = api.tx.parachainSystem.enactAuthorizedUpgrade(`0x${code}`)

    const old_version =await runtime_version(api)

    console.log(`Upgrading from ${admin.address}, ${code.length / 2} bytes`)
    // Perform the actual chain upgrade via the sudo module
    const tx = api.tx.sudo.sudo(proposal)
    await signAndSend(tx, admin)

    console.log("tx send successful ensure the new runtime version~~ please wait a later")

    await sleep(72000)

    const new_version = await runtime_version(api)

    if (new_version.specVersion >old_version.specVersion){
        console.log("version upgrade successful")
    }else{
        console.log(`old version:${old_version.toString()}  new version ${new_version.toString()},runtime upgrade failed `)
    }

    // send a tx verify the chain produce blocks
    await transfer(api,admin)

    process.exit(0)

}
async function runtime_version(api: ApiPromise) {

    const runtime_version = await api.call.core.version()

    return runtime_version
}

// event
async function listenEvent(
    api:ApiPromise,
    filterObj: { module: string; method: string; event: string }
) {
    return new Promise(async (resolve, reject) => {
        let startBlock = 0;
        let timeout = 10; // 10 block number timeout
        const unsubscribe = await api.rpc.chain.subscribeNewHeads(async (header) => {
            const currentBlockNumber = header.number.toNumber();
            if (startBlock == 0) startBlock = currentBlockNumber;
            if (currentBlockNumber > startBlock + timeout) {
                reject("timeout");
                return;
            }
            console.log(`Chain is at block: #${header.number}`);
            const signedBlock = await api.rpc.chain.getBlock(header.hash);

            const allEvents = (await api.query.system.events.at(
                header.hash
            ));
            signedBlock.block.extrinsics.forEach((ex, index) => {
                if (
                    !(
                        ex.method.section === filterObj.module &&
                        ex.method.method === filterObj.method
                    )
                ) {
                    return;
                }
                allEvents
                    .filter(({ phase, event }) => {
                        return (
                            phase.isApplyExtrinsic &&
                            phase.asApplyExtrinsic.eq(index) &&
                            event.section == filterObj.module &&
                            event.method == filterObj.event
                        );
                    })
                    .forEach(({ event }) => {
                        // const eventData = event.data as AESOutput;
                        const data = event.data;
                        const eventData:string[] = [];
                        for (let i = 0; i < data.length; i++) {
                            eventData.push();
                        }
                        resolve({ eventData });
                        unsubscribe();
                        return;
                    });
            });
        });
    });
}

//  transfer  function verify the runtime upgrade
async function transfer(
    api:ApiPromise,
    admin: KeyringPair,
) {
    const bob = keyring.addFromUri('//Bob', {name: 'Bob'})
    const tx = await api.tx.balances
        .transfer(bob.address, 1000000000000)

    await signAndSend(tx,admin)

}

(async () => {
    await cryptoWaitReady()
    const wasmPath = path.resolve(`${__dirname}`, '../../docker/runtime.compact.compressed.wasm');

    const wasm = fs.readFileSync(wasmPath)
    const codeHash = blake2AsHex(wasm, 256)
    console.log('wasm authorizeCode', codeHash)
    const code = wasm.toString('hex')

    const admin = keyring.addFromUri('//Alice', {name: 'Alice'})
    await authorizeUpgrade(admin, codeHash)

    await sleep(24000)
    await upgrade(admin, code)

})().catch(console.error)
