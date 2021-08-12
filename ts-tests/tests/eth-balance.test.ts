import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { expect } from 'chai';
import { step } from 'mocha-steps';

import { describeLitentry, loadConfig, sleep } from './utils';
import { ethLink, checkLinkingState } from './account-link';
import { assetClaim, getAssets } from './account-data-retrieve';


const DEFAULT_CONFIG = loadConfig();

const testEthAddress = DEFAULT_CONFIG.eth_address;
const privateKey = DEFAULT_CONFIG.private_key;
const ocwAccount = DEFAULT_CONFIG.ocw_account;

describeLitentry('Test Ethereum Link and Balance Fetch', ``, (context) => {
    step('Create Ethereum Link', async function () {
        await ethLink(context.api, context.alice, privateKey);
    });

    step("Retrieving Alice's linked Ethereum accounts", async function () {
        const ethAddr = await checkLinkingState(context.api, context.alice);

        expect(ethAddr.toString()).to.equal(testEthAddress);
    });

    step('Claim assets for Alice', async function () {
        await assetClaim(context.api, context.alice);
    });

    step('Retrieving assets information of Alice', async function () {
        // Retrieve ocw account balance
        const { nonce: old_n, data: old_balance } = await context.api.query.system.account(ocwAccount);

        // Wait for 150s ~ 6 blocks
        await sleep(150);
        const balances = await getAssets(context.api, context.alice);

        // TODO fetch real time balance and compare it here
        expect(balances.toString()).to.equal(`[null,"0x00000000000000004563918244f40000"]`);

        // Retrieve OCW account balances before and after assets claim
        const { nonce: new_n, data: balance } = await context.api.query.system.account(ocwAccount);
        console.log(`new is ${balance.free.toString()}  old is ${old_balance.free}`);
        // TODO Define a expect to test if this difference is within a range
        console.log(
            `difference is ${(Number(balance.free.toString()) - Number(old_balance.free.toString())).toString()}`
        );
    });
});
