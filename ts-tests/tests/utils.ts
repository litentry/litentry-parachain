import 'mocha';

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { Bytes } from '@polkadot/types';
import { TypeRegistry } from '@polkadot/types/create';
import { ChildProcess, spawn, exec } from 'child_process';
import fs from 'fs';

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
const DEFAULT_CONFIG = loadConfig();

// OCW account
const ocwAccount = DEFAULT_CONFIG.ocw_account;

// Provider is set for parachain node
const wsProvider = new WsProvider(DEFAULT_CONFIG.parachain_ws);

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

export async function launchRelayNodesAndParachainRegister(api: ApiPromise) {
    // Get keyring of Alice
    const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
    // Get keyring of Sudo
    const sudoKey = await api.query.sudo.key();

    const sudo = keyring.addFromAddress(sudoKey);

    // const sudoPair = keyring.getPair(sudoKey);

    const registry = new TypeRegistry();

    const genesisHeadBytes = fs.readFileSync(PARA_GENESIS_HEAD_PATH, 'utf8');

    const validationCodeBytes = fs.readFileSync(PARA_WASM_PATH, 'utf8');

    const tx = api.tx.sudo.sudo(
        api.tx.parasSudoWrapper.sudoScheduleParaInitialize(2022, {
            genesisHead: new Bytes(registry, genesisHeadBytes),
            validationCode: new Bytes(registry, validationCodeBytes),
            parachain: true,
        })
    );
    console.log(`Parachain registration tx Sent!`);

    // await new Promise(r => setTimeout(r, 6000));
    const parachainRegister = new Promise<{ block: string }>(async (resolve, reject) => {
        const unsub = await tx.signAndSend(alice, (result) => {
            console.log(`Parachain registration is ${result.status}`);
            if (result.status.isInBlock) {
                console.log(`Parachain registration included at blockHash ${result.status.asInBlock}`);
                console.log(`Waiting for finalization... (can take a minute)`);
            } else if (result.status.isFinalized) {
                console.log(`Transfer finalized at blockHash ${result.status.asFinalized}`);
                unsub();
                resolve({
                    block: result.status.asFinalized.toString(),
                });
            }
        });
    });

    return parachainRegister;
}

export async function initApiPromise(wsProvider: WsProvider) {
    console.log(`Initiating the API (ignore message "Unable to resolve type B..." and "Unknown types found...")`);

    // Initiate the polkadot API.
    const api = await ApiPromise.create({
        provider: wsProvider,
        types: {
            // mapping the actual specified address format
            Address: 'MultiAddress',
            // mapping the lookup
            LookupSource: 'MultiAddress',
            Account: { nonce: 'U256', balance: 'U256' },
            Transaction: {
                nonce: 'U256',
                action: 'String',
                gas_price: 'u64',
                gas_limit: 'u64',
                value: 'U256',
                input: 'Vec<u8>',
                signature: 'Signature',
            },
            Signature: '[u8; 65]', //{ v: "u64", r: "H256", s: "H256" },
            BlockWeights: 'u64',
            BlockLength: 'u64',
            ParachainInherentData: 'u64',
            DataSource: 'u64',
            EthAddress: '[u8; 20]',
            QueryKey: 'u64',
        },
    });

    console.log(`Initialization done`);
    console.log(`Genesis at block: ${api.genesisHash.toHex()}`);

    // Get keyring of Alice
    const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });

    // Insert ocw session key
    const resInsertKey = api.rpc.author.insertKey(
        'ocw!',
        'loop high amazing chat tennis auto denial attend type quit liquid tonight',
        '0x8c35b97c56099cf3b5c631d1f296abbb11289857e74a8f60936290080d56da6d'
    );

    const { nonce, data: balance } = await api.query.system.account(alice.address);
    console.log(`Alice Substrate Account: ${alice.address}`);
    console.log(`Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free.toHex()}`);

    return { api, alice };
}

async function sendTokenToOcw(api: ApiPromise, alice: KeyringPair) {
    // Transfer tokens from Alice to ocw account
    console.log(`Transfer tokens from Alice to ocw account`);
    return new Promise<{ block: string }>(async (resolve, reject) => {
        const unsub = await api.tx.balances.transfer(ocwAccount, 1000000000000000).signAndSend(alice, (result) => {
            console.log(`Current status is ${result.status}`);
            if (result.status.isInBlock) {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
            } else if (result.status.isFinalized) {
                console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
                unsub();
                resolve({
                    block: result.status.asFinalized.toString(),
                });
            }
        });
    });
}

export function describeLitentry(
    title: string,
    specFilename: string,
    cb: (context: { api: ApiPromise; alice: KeyringPair }) => void,
    provider?: string
) {
    describe(title, function () {
        // Set timeout to 6000 seconds (Because of 50-blocks delay of rococo, so called "training wheels")
        this.timeout(6000000);

        let tokenServer: ChildProcess;
        let binary: ChildProcess;
        let relayNodes: ChildProcess;
        let context: { api: ApiPromise; alice: KeyringPair } = {
            api: {} as ApiPromise,
            alice: {} as KeyringPair,
        };
        // Making sure the Litentry node has started
        before('Starting Litentry Test Node', async function () {
            const initApi = await initApiPromise(wsProvider);
            context.api = initApi.api;
            context.alice = initApi.alice;
            return sendTokenToOcw(initApi.api, initApi.alice);
        });

        after(async function () {
        });

        cb(context);
    });
}
