import { step } from 'mocha-steps';
import {
    buildValidations,
    buildIdentities,
    describeLitentry,
    encryptWithTeeShieldingKey,
    buildIdentityTxs,
} from './common/utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { ethers } from 'ethers';
import { LitentryIdentity, LitentryValidationData, TransactionSubmit } from './common/type-definitions';
import { batchCall } from './common/utils';
import { handleIdentityEvents } from './common/utils';
import { multiAccountTxSender } from './indirect_calls';
import { assert } from 'chai';
import { IdentityNetwork } from './common/helpers';
import { listenEvent } from './common/transactions';

//Explain how to use this test, which has two important parameters:

//1.buildIdentityTimes: the number of create identities for aaccount.If you want to test bulk operations on a single account, you can modify this parameter.

//2.The "number" parameter in describeLitentry represents the number of accounts generated, including Substrate wallets and Ethereum wallets.If you want to use a large number of accounts for testing, you can modify this parameter.

//3.Each time the test code is executed, new wallet account will be used.

describeLitentry('multiple accounts test', 10, async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    var substraetSigners: KeyringPair[] = [];
    var ethereumSigners: ethers.Wallet[] = [];
    var web3Validations: LitentryValidationData[] = [];
    var identityDatas: LitentryIdentity[] = [];
    let identityNetwork = IdentityNetwork.ethereum;
    let buildIdentityTimes = 10;
    let ethereumWalletsForSign: any = [];

    step('setup signers', async () => {
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
        assert.equal(
            resp_events.length,
            substraetSigners.length,
            'set usershieldingkey with multiple accounts check fail'
        );
    });

    //test identity with multiple accounts
    step('test createIdentity with multiple accounts', async () => {
        //substrate ethereum twitter
        let identities = await buildIdentities(identityNetwork, 'batch', context);

        identityDatas = [...identities];
        let txs = await buildIdentityTxs(context, identities, 'createIdentity');
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'IdentityCreated',
        ]);
        assert.equal(resp_events.length, identities.length, 'create identities with multiple accounts check fail');
        const event_data = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityCreated');
        const validations = await buildValidations(context, event_data, identities, identityNetwork, 'multiple');
        console.log('validations', validations);

        web3Validations = [...validations];
    });

    step('test verifyIdentity with multiple accounts', async () => {
        let txs = await buildIdentityTxs(context, identityDatas, 'verifyIdentity', web3Validations);
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'IdentityVerified',
        ]);
        assert.equal(resp_events.length, txs.length, 'verify identities with multiple accounts check fail');
    });

    step('test removeIdentity with multiple accounts', async () => {
        let txs = await buildIdentityTxs(context, identityDatas, 'removeIdentity', web3Validations);
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'IdentityRemoved',
        ]);
        assert.equal(resp_events.length, txs.length, 'remove identities with multiple accounts check fail');
    });

    //one account test with multiple identities
    step('test multiple createIdentities with one account', async () => {
        let txs: any = [];
        for (let index = 0; index < buildIdentityTimes; index++) {
            ethereumWalletsForSign.push(ethers.Wallet.createRandom());
        }
        let identities = await buildIdentities(
            identityNetwork,
            'utility',
            context,
            buildIdentityTimes,
            ethereumWalletsForSign
        );
        console.log('identities', identities);
        identityDatas = [...identities];

        for (let k = 0; k < identities.length; k++) {
            const identity = identities[k];
            const encode = context.api.createType('LitentryIdentity', identity).toHex();
            const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
            const tx = context.api.tx.identityManagement.createIdentity(
                context.mrEnclave,
                substraetSigners[0].address,
                `0x${ciphertext}`,
                null
            );
            txs.push(tx);
        }
        const resp_events = await batchCall(context, substraetSigners[0], txs, 'identityManagement', [
            'IdentityCreated',
        ]);
        const event_data = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityCreated');
        const validations = await buildValidations(
            context,
            event_data,
            identities,
            identityNetwork,
            'single',
            ethereumWalletsForSign
        );
        web3Validations = [...validations];
    });

    step('test multiple verifyIdentities(ethereumIdentity) with one account', async () => {
        let txs: any = [];
        for (let index = 0; index < identityDatas.length; index++) {
            let identity = identityDatas[index];
            let data = web3Validations[index];
            const identity_encode = context.api.createType('LitentryIdentity', identity).toHex();
            const validation_encode = context.api.createType('LitentryValidationData', data).toHex();
            const identity_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, identity_encode).toString(
                'hex'
            );
            const validation_ciphertext = encryptWithTeeShieldingKey(
                context.teeShieldingKey,
                validation_encode
            ).toString('hex');
            const tx = context.api.tx.identityManagement.verifyIdentity(
                context.mrEnclave,
                `0x${identity_ciphertext}`,
                `0x${validation_ciphertext}`
            );
            txs.push(tx);
        }

        const resp_events = await batchCall(context, substraetSigners[0], txs, 'identityManagement', [
            'IdentityVerified',
        ]);
        assert.equal(resp_events.length, txs.length, 'verify identities with one account check fail');
    });
});
