import { step } from 'mocha-steps';
import { checkVc, describeLitentry, encryptWithTeeShieldingKey } from './common/utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { ethers } from 'ethers';
import { u8aToHex } from '@polkadot/util';
import { Assertion, IndexingNetwork, TransactionSubmit } from './common/type-definitions';
import { handleVcEvents } from './common/utils';
import { blake2AsHex } from '@polkadot/util-crypto';
import { assert } from 'chai';
import { HexString } from '@polkadot/util/types';
import { listenEvent, multiAccountTxSender } from './common/transactions';
import { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
const assertion = <Assertion>{
    A1: 'A1',
    A2: ['A2'],
    A3: ['A3', 'A3', 'A3'],
    A4: '10.001',
    A7: '10.002',
    A8: [IndexingNetwork.Litentry],
    A10: '10.003',
    A11: '10.004',
};

//Explain how to use this test, which has two important parameters:
//1.The "number" parameter in describeLitentry represents the number of accounts generated, including Substrate wallets and Ethereum wallets.If you want to use a large number of accounts for testing, you can modify this parameter.
//2.Each time the test code is executed, new wallet account will be used.

describeLitentry('multiple accounts test', 10, async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    var substrateSigners: KeyringPair[] = [];
    var ethereumSigners: ethers.Wallet[] = [];
    var vcIndexList: HexString[] = [];
    // If want to test other assertions with multiple accounts,just need to make changes here.
    let assertion_type = assertion.A1;
    step('init', async () => {
        substrateSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.substrateWallet;
        });
        ethereumSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.ethereumWallet;
        });
    });
    step('send test token to each account', async () => {
        const txs: SubmittableExtrinsic<ApiTypes>[] = [];

        for (let i = 0; i < substrateSigners.length; i++) {
            //1 token
            const tx = context.api.tx.balances.transfer(substrateSigners[i].address, '1000000000000');

            txs.push(tx);
        }
        await context.api.tx.utility.batch(txs).signAndSend(context.substrateWallet.alice);
        await listenEvent(context.api, 'balances', ['Transfer'], txs.length, [
            u8aToHex(context.substrateWallet.alice.addressRaw),
        ]);
    });
    //test with multiple accounts
    step('test set usershieldingkey with multiple accounts', async () => {
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, aesKey).toString('hex');
        let txs: TransactionSubmit[] = [];
        for (let i = 0; i < substrateSigners.length; i++) {
            const tx = context.api.tx.identityManagement.setUserShieldingKey(context.mrEnclave, `0x${ciphertext}`);
            const nonce = (await context.api.rpc.system.accountNextIndex(substrateSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const resp_events = await multiAccountTxSender(context, txs, substrateSigners, 'identityManagement', [
            'UserShieldingKeySet',
        ]);
        assert.equal(resp_events.length, substrateSigners.length, 'set usershieldingkey check fail');
    });

    step('test requestVc with multiple accounts', async () => {
        let txs: TransactionSubmit[] = [];
        for (let i = 0; i < substrateSigners.length; i++) {
            const tx = context.api.tx.vcManagement.requestVc(context.mrEnclave, assertion_type);
            const nonce = (await context.api.rpc.system.accountNextIndex(substrateSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const resp_events = await multiAccountTxSender(context, txs, substrateSigners, 'vcManagement', ['VCIssued']);
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
        for (let i = 0; i < substrateSigners.length; i++) {
            const tx = context.api.tx.vcManagement.disableVc(vcIndexList[i]);
            const nonce = (await context.api.rpc.system.accountNextIndex(substrateSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const resp_events = await multiAccountTxSender(context, txs, substrateSigners, 'vcManagement', ['VCDisabled']);

        assert.equal(resp_events.length, vcIndexList.length, 'disable vc check fail');
        const event_datas = await handleVcEvents(aesKey, resp_events, 'VCDisabled');
        for (let k = 0; k < vcIndexList.length; k++) {
            console.log('disableVc index:', k);
            assert.equal(event_datas[k], vcIndexList[k], 'check index error');
            const registry = (await context.api.query.vcManagement.vcRegistry(vcIndexList[k])) as any;
            assert.equal(registry.toHuman()!['status'], 'Disabled');
        }
    });

    step('test revokeVc with multiple accounts', async () => {
        let txs: TransactionSubmit[] = [];
        for (let i = 0; i < substrateSigners.length; i++) {
            const tx = context.api.tx.vcManagement.revokeVc(vcIndexList[i]);
            const nonce = (await context.api.rpc.system.accountNextIndex(substrateSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const resp_events = await multiAccountTxSender(context, txs, substrateSigners, 'vcManagement', ['VCRevoked']);
        assert.equal(resp_events.length, vcIndexList.length, 'revoke vc check fail');
        const event_datas = await handleVcEvents(aesKey, resp_events, 'VCRevoked');
        console.log('event_datas', event_datas);

        for (let k = 0; k < vcIndexList.length; k++) {
            console.log('revokeVc index:', k);
            assert.equal(event_datas[k], vcIndexList[k], 'check index error');
            const registry = (await context.api.query.vcManagement.vcRegistry(vcIndexList[k])) as any;
            assert.equal(registry.toHuman(), null);
        }
    });
});
