import 'mocha';
import '@polkadot/api-augment';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { loadConfig } from './config';

export class ParachainConfig {
    api!: ApiPromise;
    parachain!: string;
    alice!: KeyringPair;
    bob!: KeyringPair;
    eve!: KeyringPair;
    ferdie!: KeyringPair;
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
