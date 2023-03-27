import { step } from 'mocha-steps';
import { checkVc, describeLitentry, encryptWithTeeShieldingKey } from './common/utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { ethers } from 'ethers';

import { Assertion, TransactionSubmit } from './common/type-definitions';
import { batchCall } from './common/utils';
import { handleVcEvents } from './common/utils';
import { multiAccountTxSender } from './indirect_calls';
import { blake2AsHex } from '@polkadot/util-crypto';
import { assert } from 'chai';
import { HexString } from '@polkadot/util/types';
import { listenEvent } from './common/transactions';
const assertion = <Assertion>{
    A1: 'A1',
    A2: ['A2'],
    A3: ['A3', 'A3', 'A3'],
    A4: [10],
    A7: [10],
    A8: ['litentry'],
    A10: [10],
    A11: [10],
};

//Explain how to use this test, which has two important parameters:

//1.request_times: the number of requestVc for a single account.If you want to test bulk operations on a single account, you can modify this parameter.

//2.The "number" parameter in describeLitentry represents the number of accounts generated, including Substrate wallets and Ethereum wallets.If you want to use a large number of accounts for testing, you can modify this parameter.

//3.Each time the test code is executed, new wallet account will be used.

describeLitentry('multiple accounts test', 1, async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    var substraetSigners: KeyringPair[] = [];
    var ethereumSigners: ethers.Wallet[] = [];
    var vcIndexList: HexString[] = [];

    //requestVC times for one account.
    let request_times = 10;

    // If want to test other assertions with multiple accounts,just need to make changes here.
    let assertion_type = assertion.A1;
    step('init', async () => {
        substraetSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.substrateWallet;
        });
        ethereumSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.ethereumWallet;
        });
    });
    step('send test token to each account', async () => {
        const txs: any = [];
        for (let i = 0; i < substraetSigners.length; i++) {
            //0.1 token
            const tx = context.api.tx.balances.transfer(substraetSigners[i].address, '100000000000');

            txs.push(tx);
        }
        await context.api.tx.utility.batch(txs).signAndSend(context.substrateWallet.alice);
        await listenEvent(context.api, 'balances', ['Transfer'], txs.length);
    });
    //test with multiple accounts
    step('test set usershieldingkey with multiple accounts', async () => {
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, aesKey).toString('hex');
        let txs: TransactionSubmit[] = [];
        for (let i = 0; i < substraetSigners.length; i++) {
            const tx = context.api.tx.identityManagement.setUserShieldingKey(context.mrEnclave, `0x${ciphertext}`);
            const nonce = (await context.api.rpc.system.accountNextIndex(substraetSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'UserShieldingKeySet',
        ]);
        assert.equal(resp_events.length, substraetSigners.length, 'set usershieldingkey check fail');
    });

    step('test requestVc with multiple accounts', async () => {
        let txs: TransactionSubmit[] = [];
        for (let i = 0; i < substraetSigners.length; i++) {
            const tx = context.api.tx.vcManagement.requestVc(context.mrEnclave, assertion_type);
            const nonce = (await context.api.rpc.system.accountNextIndex(substraetSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'vcManagement', ['VCIssued']);
        const event_data = await handleVcEvents(aesKey, resp_events, 'VCIssued');

        for (let k = 0; k < event_data.length; k++) {
            const vcString = event_data[k].vc.replace('0x', '');
            const vcObj = JSON.parse(vcString);

            const vcProof = vcObj.proof;

            const registry = (await context.api.query.vcManagement.vcRegistry(event_data[k].index)) as any;
            assert.equal(registry.toHuman()!['status'], 'Active', 'check registry error');

            const vcHash = blake2AsHex(Buffer.from(vcString));
            assert.equal(vcHash, registry.toHuman()!['hash_'], 'check vc json hash error');

            //check vc
            const vcValid = await checkVc(vcObj, event_data[k].index, vcProof, context.api);
            assert.equal(vcValid, true, 'check vc error');
            vcIndexList.push(event_data[k].index);
        }
    });

    step('test disableVc with multiple accounts', async () => {
        let txs: TransactionSubmit[] = [];
        for (let i = 0; i < substraetSigners.length; i++) {
            const tx = context.api.tx.vcManagement.disableVc(vcIndexList[i]);
            const nonce = (await context.api.rpc.system.accountNextIndex(substraetSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'vcManagement', ['VCIssued']);
        assert.equal(resp_events.length, vcIndexList.length, 'disable vc check fail');
    });

    step('test revokeVc with multiple accounts', async () => {
        let txs: TransactionSubmit[] = [];
        for (let i = 0; i < substraetSigners.length; i++) {
            const tx = context.api.tx.vcManagement.revokeVc(vcIndexList[i]);
            const nonce = (await context.api.rpc.system.accountNextIndex(substraetSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'vcManagement', ['VCIssued']);
        assert.equal(resp_events.length, vcIndexList.length, 'revoke vc check fail');
    });

    // test multiple vc with one account
    step('test multiple requestVc with one account', async () => {
        let txs: any = [];

        let vc_params: any[] = [];
        for (let i = 0; i < request_times; i++) {
            vc_params.push(assertion_type);
        }
        for (let i = 0; i < vc_params.length; i++) {
            const tx = context.api.tx.vcManagement.requestVc(context.mrEnclave, vc_params[i]);
            txs.push(tx);
        }

        const resp_events = await batchCall(context, substraetSigners[0], txs, 'vcManagement', ['VCIssued']);
        assert.equal(resp_events.length, request_times, 'request multiple vc check fail');
        const event_data = await handleVcEvents(aesKey, resp_events, 'requestVc');
        for (let m = 0; m < event_data.length; m++) {
            vcIndexList.push(event_data[m].index);
        }
    });
    step('test multiple disableVc with one account', async () => {
        let txs: any = [];
        let vc_params: any[] = [];
        let test_times = 10;
        for (let i = 0; i < test_times; i++) {
            vc_params.push(assertion_type);
        }
        for (let i = 0; i < vc_params.length; i++) {
            const tx = context.api.tx.vcManagement.disableVc(vcIndexList[i]);
            txs.push(tx);
        }
        const resp_events = await batchCall(context, substraetSigners[0], txs, 'vcManagement', ['VCDisabled']);
        assert.equal(resp_events.length, test_times, 'disabled multiple vc check fail');
        const events_data = await handleVcEvents(aesKey, resp_events, 'disableVc');
        await Promise.all(
            events_data.map(async (event: any, k: any) => {
                assert.equal(event, vcIndexList[k], 'check index error');
                const registry = (await context.api.query.vcManagement.vcRegistry(vcIndexList[k])) as any;
                assert.equal(registry.toHuman()!['status'], 'Disabled');
            })
        );
    });
    step('test multiple revokeVc with one account', async () => {
        let txs: any = [];
        let vc_params: any[] = [];
        let test_times = 10;
        for (let i = 0; i < test_times; i++) {
            vc_params.push(assertion_type);
        }
        for (let i = 0; i < vc_params.length; i++) {
            const tx = context.api.tx.vcManagement.revokeVc(vcIndexList[i]);
            txs.push(tx);
        }
        const resp_events = await batchCall(context, substraetSigners[0], txs, 'vcManagement', ['VCRevoked']);
        assert.equal(resp_events.length, test_times, 'revoked multiple vc check fail');
        const events_data = await handleVcEvents(aesKey, resp_events, 'disableVc');
        await Promise.all(
            events_data.map(async (event: any, k: any) => {
                assert.equal(event, vcIndexList[k], 'check index error');
                const registry = (await context.api.query.vcManagement.vcRegistry(vcIndexList[k])) as any;
                assert.equal(registry.toHuman()!['status'], null);
            })
        );
    });
});
