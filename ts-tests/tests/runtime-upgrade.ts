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
    const config = loadConfig();
    const provider = new WsProvider(config.parachain_ws);
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

    const config = loadConfig();
    const provider = new WsProvider(config.parachain_ws);
    const api = await ApiPromise.create({
        provider: provider,
    });

    const proposal = api.tx.parachainSystem.enactAuthorizedUpgrade(`0x${code}`)

    console.log(`Upgrading from ${admin.address}, ${code.length / 2} bytes`)
    // Perform the actual chain upgrade via the sudo module
    await api.tx.sudo
        .sudo(proposal)
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
    console.log(`parachain runtime upgrade ready`)

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
    // event
    await sleep(24000)
    await upgrade(admin, code)

})().catch(console.error)
