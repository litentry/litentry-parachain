import 'mocha';

import '@polkadot/api-augment';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { AddressOrPair, ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import { KeyringPair } from '@polkadot/keyring/types';
import { BN } from '@polkadot/util';

export class ParachainConfig {
    api!: ApiPromise;
    parachain!: string;
    alice!: KeyringPair;
    bob!: KeyringPair;
    eve!: KeyringPair;
    ferdie!: KeyringPair;
}

export function loadConfig() {
    require('dotenv').config();
    switch (process.env.NODE_ENV) {
        case 'development':
        case 'test':
        case 'ci':
            return require('../config.ci.json');
        case 'staging':
            return require('../config.staging.json');
        default:
            throw new Error(`Invalid NODE_ENV: ${process.env.NODE_ENV}`);
    }
}

export function sleep(secs: number) {
    return new Promise((resolve) => {
        setTimeout(resolve, secs * 1000);
    });
}

export async function initApiPromise(config: any): Promise<ParachainConfig> {
    console.log(`Initiating the API (ignore message "Unable to resolve type B..." and "Unknown types found...")`);
    // Provider is set for parachain node
    const wsProvider = new WsProvider(config.parachain_ws);
    // Intentionally return an unknown default value
    const parachain = process.env.PARACHAIN_TYPE || 'unknown_parachain';

    // Initiate the polkadot API.
    const api = await ApiPromise.create({
        provider: wsProvider,
    });

    console.log(`Initialization done`);
    console.log(`Genesis at block: ${api.genesisHash.toHex()}`);

    // Get keyring of Alice and Bob
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    const eve = keyring.addFromUri('//Eve');
    const ferdie = keyring.addFromUri('//Ferdie');

    // Insert ocw session key
    const resInsertKey = api.rpc.author.insertKey(
        'ocw!',
        'loop high amazing chat tennis auto denial attend type quit liquid tonight',
        '0x8c35b97c56099cf3b5c631d1f296abbb11289857e74a8f60936290080d56da6d'
    );

    const { nonce: nonceAlice, data: balanceAlice } = await api.query.system.account(alice.address);
    const { nonce: nonceBob, data: balanceBob } = await api.query.system.account(bob.address);
    const { nonce: nonceEve, data: balanceEve } = await api.query.system.account(eve.address);
    const { nonce: nonceFerdie, data: balanceFerdie } = await api.query.system.account(ferdie.address);
    console.log(
        `Alice Substrate Account: ${alice.address} (nonce: ${nonceAlice}) balance, free: ${balanceAlice.free.toHex()}`
    );
    console.log(`Bob Substrate Account: ${bob.address} (nonce: ${nonceBob}) balance, free: ${balanceBob.free.toHex()}`);
    console.log(`Eve Substrate Account: ${eve.address} (nonce: ${nonceEve}) balance, free: ${balanceEve.free.toHex()}`);
    console.log(
        `Ferdie Substrate Account: ${
            ferdie.address
        } (nonce: ${nonceFerdie}) balance, free: ${balanceFerdie.free.toHex()}`
    );

    return { api, parachain, alice, bob, eve, ferdie };
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

/// After removing the sudo module, we use `EnsureRootOrHalfCouncil` instead of `Sudo`,
//  and there are only two council members in litmus-dev/rococo-dev/litentry-dev.
//  So only `propose` is required, no vote.
// TODO support to send the `vote extrinsic`, if the number of council members is greater than 2.
export async function sudoWrapper(api: ApiPromise, tx: SubmittableExtrinsic<ApiTypes>) {
    const chain = (await api.rpc.system.chain()).toString().toLowerCase();
    if (chain == 'litmus-dev') {
        const threshold = api.createType('Compact<u32>', 1);
        const call = api.createType('Call', tx);
        return api.tx.council.propose(threshold, call, api.createType('Compact<u32>', tx.length));
    } else {
        return api.tx.sudo.sudo(tx);
    }
}

export function describeLitentry(title: string, specFilename: string, cb: (context: ParachainConfig) => void) {
    describe(title, function () {
        // Set timeout to 6000 seconds (Because of 50-blocks delay of rococo, so called "training wheels")
        this.timeout(6000000);

        let context: ParachainConfig = {
            api: {} as ApiPromise,
            parachain: {} as string,
            alice: {} as KeyringPair,
            bob: {} as KeyringPair,
            eve: {} as KeyringPair,
            ferdie: {} as KeyringPair,
        };
        // Making sure the Litentry node has started
        before('Starting Litentry Test Node', async function () {
            const config = loadConfig();
            const initApi = await initApiPromise(config);
            context.parachain = initApi.parachain;
            context.api = initApi.api;
            context.alice = initApi.alice;
            context.bob = initApi.bob;
            context.eve = initApi.eve;
        });

        after(async function () {});

        cb(context);
    });
}
