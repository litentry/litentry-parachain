import 'mocha';

import '@polkadot/api-augment';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { AddressOrPair, ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import { KeyringPair } from '@polkadot/keyring/types';
import { TypeRegistry } from '@polkadot/types/create';

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

async function initApiPromise(config: any) {
    console.log(`Initiating the API (ignore message "Unable to resolve type B..." and "Unknown types found...")`);
    // Provider is set for parachain node
    const wsProvider = new WsProvider(config.parachain_ws);

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

    // Insert ocw session key
    const resInsertKey = api.rpc.author.insertKey(
        'ocw!',
        'loop high amazing chat tennis auto denial attend type quit liquid tonight',
        '0x8c35b97c56099cf3b5c631d1f296abbb11289857e74a8f60936290080d56da6d'
    );

	// Set Eve's balance to 1000000000000000
	const txSetBalance = api.tx.sudo.sudo(
		api.tx.balances.setBalance(eve.address, 1000000000000000, 0)
		);
	await signAndSend(txSetBalance, alice);

    const { nonce: nonceAlice, data: balanceAlice } = await api.query.system.account(alice.address);
    const { nonce: nonceBob, data: balanceBob } = await api.query.system.account(bob.address);
    const { nonce: nonceEve, data: balanceEve } = await api.query.system.account(eve.address);
    console.log(`Alice Substrate Account: ${alice.address} (nonce: ${nonceAlice}) balance, free: ${balanceAlice.free.toHex()}`);
    console.log(`Bob Substrate Account: ${bob.address} (nonce: ${nonceBob}) balance, free: ${balanceBob.free.toHex()}`);
    console.log(`Eve Substrate Account: ${eve.address} (nonce: ${nonceEve}) balance, free: ${balanceEve.free.toHex()}`);

    return { api, alice, bob, eve};
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

export function describeLitentry(
    title: string,
    specFilename: string,
    cb: (context: { api: ApiPromise; alice: KeyringPair; bob: KeyringPair , eve: KeyringPair}) => void
) {
    describe(title, function () {
        // Set timeout to 6000 seconds (Because of 50-blocks delay of rococo, so called "training wheels")
        this.timeout(6000000);

        let context: { api: ApiPromise; alice: KeyringPair; bob: KeyringPair; eve: KeyringPair} = {
            api: {} as ApiPromise,
            alice: {} as KeyringPair,
            bob: {} as KeyringPair,
            eve: {} as KeyringPair,
        };
        // Making sure the Litentry node has started
        before('Starting Litentry Test Node', async function () {
            const config = loadConfig();
            const initApi = await initApiPromise(config);
            context.api = initApi.api;
            context.alice = initApi.alice;
            context.bob = initApi.bob;
            context.eve = initApi.eve;
        });

        after(async function () {});

        cb(context);
    });
}
