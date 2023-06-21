import { step } from 'mocha-steps';
import { checkVc, describeLitentry, encryptWithTeeShieldingKey } from './common/utils';
import { hexToU8a } from '@polkadot/util';
import { handleVcEvents } from './common/utils';
import { blake2AsHex } from '@polkadot/util-crypto';
import { assert } from 'chai';
import type { HexString } from '@polkadot/util/types';
import type { Assertion, TransactionSubmit } from './common/type-definitions';
import type { KeyringPair } from '@polkadot/keyring/types';
import { multiAccountTxSender } from './common/transactions';
import { aesKey } from './common/call';
import { SubmittableResult } from '@polkadot/api';

const assertionA1: Assertion = {
    A1: 'A1',
};

//Explain how to use this test, which has two important parameters:
//1.The "number" parameter in describeLitentry represents the number of accounts generated, including Substrate wallets and Ethereum wallets.If you want to use a large number of accounts for testing, you can modify this parameter.
//2.Each time the test code is executed, new wallet account will be used.
describeLitentry('multiple accounts test', 2, async (context) => {
    let substrateSigners: KeyringPair[] = [];
    const vcIndexList: HexString[] = [];
    // If want to test other assertions with multiple accounts,just need to make changes here.
    const assertionType = assertionA1;
    step('init', async () => {
        substrateSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.substrateWallet;
        });
    });
    step('send test token to each account', async () => {
        const txs: any[] = [];

        for (let i = 0; i < substrateSigners.length; i++) {
            //1 token
            const tx = context.api.tx.balances.transfer(substrateSigners[i].address, '1000000000000');
            txs.push(tx);
        }
        await new Promise((resolve, reject) => {
            context.api.tx.utility
                .batch(txs)
                .signAndSend(context.substrateWallet.alice, (result: SubmittableResult) => {
                    console.log(`Current status is ${result.status}`);
                    if (result.status.isFinalized) {
                        resolve(result.status);
                    } else if (result.status.isInvalid) {
                        console.log(`Transaction is ${result.status}`);
                        reject(result.status);
                    }
                });
        });
    });
    //test with multiple accounts
    step('test set usershieldingkey with multiple accounts', async () => {
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, hexToU8a(aesKey)).toString('hex');
        const txs: TransactionSubmit[] = [];
        for (let i = 0; i < substrateSigners.length; i++) {
            const tx = context.api.tx.identityManagement.setUserShieldingKey(context.mrEnclave, `0x${ciphertext}`);
            const nonce = (await context.api.rpc.system.accountNextIndex(substrateSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const events = await multiAccountTxSender(context, txs, substrateSigners, 'identityManagement', [
            'UserShieldingKeySet',
        ], 15);
        assert.equal(events.length, substrateSigners.length, 'set usershieldingkey check fail');
    });

    step('test requestVc with multiple accounts', async () => {
        const txs: TransactionSubmit[] = [];
        for (let i = 0; i < substrateSigners.length; i++) {
            const tx = context.api.tx.vcManagement.requestVc(context.mrEnclave, assertionType);
            const nonce = (await context.api.rpc.system.accountNextIndex(substrateSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const events = await multiAccountTxSender(context, txs, substrateSigners, 'vcManagement', ['VCIssued'], 15);

        const eventsData = await handleVcEvents(aesKey, events, 'VCIssued');

        for (let k = 0; k < eventsData.length; k++) {
            const vcString = eventsData[k].vc.replace('0x', '');
            const vcObj = JSON.parse(vcString);

            const vcProof = vcObj.proof;

            const registry = (await context.api.query.vcManagement.vcRegistry(eventsData[k].index)) as any;
            assert.equal(registry.toHuman()['status'], 'Active', 'check registry error');

            const vcHash = blake2AsHex(Buffer.from(vcString));
            assert.equal(vcHash, registry.toHuman()['hash_'], 'check vc json hash error');

            //check vc
            const vcValid = await checkVc(vcObj, eventsData[k].index, vcProof, context.api);
            assert.equal(vcValid, true, 'check vc error');
            vcIndexList.push(eventsData[k].index);
        }
    });

    step('test disableVc with multiple accounts', async () => {
        const txs: TransactionSubmit[] = [];
        for (let i = 0; i < substrateSigners.length; i++) {
            const tx = context.api.tx.vcManagement.disableVc(vcIndexList[i]);
            const nonce = (await context.api.rpc.system.accountNextIndex(substrateSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const events = await multiAccountTxSender(context, txs, substrateSigners, 'vcManagement', ['VCDisabled'], 15);

        assert.equal(events.length, vcIndexList.length, 'disable vc check fail');
        const eventsData = await handleVcEvents(aesKey, events, 'VCDisabled');
        for (let k = 0; k < vcIndexList.length; k++) {
            console.log('disableVc index:', k);
            assert.equal(eventsData[k], vcIndexList[k], 'check index error');
            const registry = (await context.api.query.vcManagement.vcRegistry(vcIndexList[k])) as any;
            assert.equal(registry.toHuman()['status'], 'Disabled');
        }
    });

    step('test revokeVc with multiple accounts', async () => {
        const txs: TransactionSubmit[] = [];
        for (let i = 0; i < substrateSigners.length; i++) {
            const tx = context.api.tx.vcManagement.revokeVc(vcIndexList[i]);
            const nonce = (await context.api.rpc.system.accountNextIndex(substrateSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const events = await multiAccountTxSender(context, txs, substrateSigners, 'vcManagement', ['VCRevoked'], 15);
        assert.equal(events.length, vcIndexList.length, 'revoke vc check fail');
        const eventsData = await handleVcEvents(aesKey, events, 'VCRevoked');
        console.log('eventsData', eventsData);

        for (let k = 0; k < vcIndexList.length; k++) {
            console.log('revokeVc index:', k);
            assert.equal(eventsData[k], vcIndexList[k], 'check index error');
            const registry = (await context.api.query.vcManagement.vcRegistry(vcIndexList[k])) as any;
            assert.equal(registry.toHuman(), null);
        }
    });
});
