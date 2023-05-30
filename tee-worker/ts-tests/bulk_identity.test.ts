import { step } from 'mocha-steps';
import {
    buildValidations,
    describeLitentry,
    buildIdentityTxs,
    buildIdentityHelper,
    assertIdentityCreated,
    assertIdentityRemoved,
    assertIdentityVerified,
    assertInitialIDGraphCreated,
} from './common/utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { ethers } from 'ethers';
import { LitentryIdentity, LitentryValidationData } from './common/type-definitions';
import { handleIdentityEvents } from './common/utils';
import { multiAccountTxSender } from './common/transactions';
import { SubmittableResult } from '@polkadot/api';

//Explain how to use this test, which has two important parameters:
//1.The "number" parameter in describeLitentry represents the number of accounts generated, including Substrate wallets and Ethereum wallets.If you want to use a large number of accounts for testing, you can modify this parameter.
//2.Each time the test code is executed, new wallet account will be used.

describeLitentry('multiple accounts test', 2, async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    var substrateSigners: KeyringPair[] = [];
    var ethereumSigners: ethers.Wallet[] = [];
    var web3Validations: LitentryValidationData[] = [];
    var identities: LitentryIdentity[] = [];
    step('setup signers', async () => {
        substrateSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.substrateWallet;
        });
        ethereumSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.ethereumWallet;
        });
    });
    step('send test token to each account', async () => {
        const txs: any = [];
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
        let txs = await buildIdentityTxs(context, substrateSigners, [], 'setUserShieldingKey');
        const resp_events = await multiAccountTxSender(context, txs, substrateSigners, 'identityManagement', [
            'UserShieldingKeySet',
        ]);

        const event_datas = await handleIdentityEvents(context, aesKey, resp_events, 'UserShieldingKeySet');
        event_datas.forEach(async (data: any, index: number) => {
            await assertInitialIDGraphCreated(context.api, substrateSigners[index], data);
        });
    });

    //test identity with multiple accounts
    step('test createIdentity with multiple accounts', async () => {
        for (let index = 0; index < ethereumSigners.length; index++) {
            let identity = await buildIdentityHelper(ethereumSigners[index].address, 'Ethereum', 'Evm');
            identities.push(identity);
        }

        let txs = await buildIdentityTxs(context, substrateSigners, identities, 'createIdentity');

        const resp_events = await multiAccountTxSender(context, txs, substrateSigners, 'identityManagement', [
            'IdentityCreated',
        ]);
        const resp_events_datas = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityCreated');

        for (let index = 0; index < resp_events_datas.length; index++) {
            console.log('createIdentity', index);
            assertIdentityCreated(substrateSigners[index], resp_events_datas[index]);
        }
        const validations = await buildValidations(
            context,
            resp_events_datas,
            identities,
            'ethereum',
            substrateSigners,
            ethereumSigners
        );

        web3Validations = [...validations];
    });

    step('test verifyIdentity with multiple accounts', async () => {
        let txs = await buildIdentityTxs(context, substrateSigners, identities, 'verifyIdentity', web3Validations);
        const resp_events = await multiAccountTxSender(context, txs, substrateSigners, 'identityManagement', [
            'IdentityVerified',
        ]);
        const [resp_events_datas] = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityVerified');
        for (let index = 0; index < resp_events_datas.length; index++) {
            console.log('verifyIdentity', index);
            assertIdentityVerified(substrateSigners[index], resp_events_datas);
        }
    });

    step('test removeIdentity with multiple accounts', async () => {
        let txs = await buildIdentityTxs(context, substrateSigners, identities, 'removeIdentity');

        const resp_events = await multiAccountTxSender(context, txs, substrateSigners, 'identityManagement', [
            'IdentityRemoved',
        ]);
        const [resp_events_datas] = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityRemoved');
        for (let index = 0; index < resp_events_datas.length; index++) {
            console.log('verifyIdentity', index);
            assertIdentityRemoved(substrateSigners[index], resp_events_datas);
        }
    });
});
