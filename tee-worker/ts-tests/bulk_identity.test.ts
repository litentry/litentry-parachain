import { step } from 'mocha-steps';
import {
    buildValidations,
    describeLitentry,
    buildIdentityTxs,
    buildIdentityHelper,
    assertIdentityCreated,
    assertIdentityRemoved,
    assertIdentityVerified,
} from './common/utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { ethers } from 'ethers';
import { LitentryIdentity, LitentryValidationData } from './common/type-definitions';
import { handleIdentityEvents } from './common/utils';
import { assert } from 'chai';
import { listenEvent, multiAccountTxSender } from './common/transactions';
import { u8aToHex } from '@polkadot/util';
import { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';

//Explain how to use this test, which has two important parameters:
//1.The "number" parameter in describeLitentry represents the number of accounts generated, including Substrate wallets and Ethereum wallets.If you want to use a large number of accounts for testing, you can modify this parameter.
//2.Each time the test code is executed, new wallet account will be used.

describeLitentry('multiple accounts test', 10, async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    var substraetSigners: KeyringPair[] = [];
    var ethereumSigners: ethers.Wallet[] = [];
    var web3Validations: LitentryValidationData[] = [];
    var identities: LitentryIdentity[] = [];
    step('setup signers', async () => {
        substraetSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.substrateWallet;
        });
        ethereumSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.ethereumWallet;
        });
    });
    step('send test token to each account', async () => {
        const txs: SubmittableExtrinsic<ApiTypes>[] = [];
        for (let i = 0; i < substraetSigners.length; i++) {
            //1 token
            const tx = context.api.tx.balances.transfer(substraetSigners[i].address, '1000000000000');
            txs.push(tx);
        }
        await context.api.tx.utility.batch(txs).signAndSend(context.substrateWallet.alice);
        await listenEvent(context.api, 'balances', ['Transfer'], txs.length, [
            u8aToHex(context.substrateWallet.alice.addressRaw),
        ]);
    });

    //test with multiple accounts
    step('test set usershieldingkey with multiple accounts', async () => {
        let txs = await buildIdentityTxs(context, substraetSigners, [], 'setUserShieldingKey');
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'UserShieldingKeySet',
        ]);

        const event_datas = await handleIdentityEvents(context, aesKey, resp_events, 'UserShieldingKeySet');
        assert.equal(
            resp_events.length,
            substraetSigners.length,
            'set usershieldingkey with multiple accounts check fail'
        );
        event_datas.forEach((data: any, index: number) => {
            assert.equal(
                data.who,
                u8aToHex(substraetSigners[index].addressRaw),
                `shielding key should be set,account ${index + 1} is not set`
            );
        });
    });

    //test identity with multiple accounts
    step('test createIdentity with multiple accounts', async () => {
        for (let index = 0; index < ethereumSigners.length; index++) {
            let identity = await buildIdentityHelper(ethereumSigners[index].address, 'Ethereum', 'Evm');
            identities.push(identity);
        }

        let txs = await buildIdentityTxs(context, substraetSigners, identities, 'createIdentity');

        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'IdentityCreated',
        ]);
        const resp_events_datas = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityCreated');

        assert.equal(resp_events.length, identities.length, 'create identities with multiple accounts check fail');

        for (let index = 0; index < resp_events_datas.length; index++) {
            console.log('createIdentity', index);
            assertIdentityCreated(substraetSigners[index], resp_events_datas[index]);
        }
        const validations = await buildValidations(
            context,
            resp_events_datas,
            identities,
            'ethereum',
            substraetSigners,
            ethereumSigners
        );

        web3Validations = [...validations];
    });

    step('test verifyIdentity with multiple accounts', async () => {
        let txs = await buildIdentityTxs(context, substraetSigners, identities, 'verifyIdentity', web3Validations);
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'IdentityVerified',
        ]);
        assert.equal(resp_events.length, txs.length, 'verify identities with multiple accounts check fail');
        const [resp_events_datas] = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityVerified');
        for (let index = 0; index < resp_events_datas.length; index++) {
            console.log('verifyIdentity', index);
            assertIdentityVerified(substraetSigners[index], resp_events_datas);
        }
    });

    step('test removeIdentity with multiple accounts', async () => {
        let txs = await buildIdentityTxs(context, substraetSigners, identities, 'removeIdentity');

        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'IdentityRemoved',
        ]);
        assert.equal(resp_events.length, txs.length, 'remove identities with multiple accounts check fail');
        const [resp_events_datas] = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityRemoved');
        for (let index = 0; index < resp_events_datas.length; index++) {
            console.log('verifyIdentity', index);
            assertIdentityRemoved(substraetSigners[index], resp_events_datas);
        }
    });
});
