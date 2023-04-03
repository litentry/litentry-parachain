import {
    describeLitentry,
    buildIdentityTxs,
    handleIdentityEvents,
    buildIdentityHelper,
    buildValidations,
} from './common/utils';
import { u8aToHex } from '@polkadot/util';
import { assertIdentityCreated, assertIdentityVerified, assertIdentityRemoved } from './indirect_calls';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { LitentryIdentity, LitentryValidationData, Web3Wallets } from './common/type-definitions';
import { multiAccountTxSender, sendTxsWithUtility } from './common/transactions';
import { generateWeb3Wallets } from './common/utils'
import { ethers } from 'ethers';
describeLitentry('Test Batch Utility', 0, (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    let identities: LitentryIdentity[] = [];
    let validations: LitentryValidationData[] = [];
    var ethereumSigners: ethers.Wallet[] = [];

    step('generate web3 wallets', async function () {
        const web3Wallets = await generateWeb3Wallets(1);
        ethereumSigners = web3Wallets.map((web3Signer) => {
            return web3Signer.ethereumWallet;
        })
    });
    step('set user shielding key', async function () {
        let [alice_txs] = await buildIdentityTxs(context, [context.substrateWallet.alice], [], 'setUserShieldingKey');
        const resp_events = await multiAccountTxSender(
            context,
            [alice_txs],
            [context.substrateWallet.alice],
            'identityManagement',
            ['UserShieldingKeySet']
        );

        const [alice] = await handleIdentityEvents(context, aesKey, resp_events, 'UserShieldingKeySet');
        assert.equal(alice, u8aToHex(context.substrateWallet.alice.addressRaw), 'alice shielding key should be set');

    });

    step('batch test: create identities', async function () {
        for (let index = 0; index < ethereumSigners.length; index++) {
            const signer = ethereumSigners[index];
            const ethereum_identity = await buildIdentityHelper(signer.address, 'Ethereum', 'Evm')
            identities.push(ethereum_identity)
        }

        const txs = await buildIdentityTxs(context, [context.substrateWallet.alice], identities, 'createIdentity');

        const resp_events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'identityManagement', [
            'IdentityCreated',
        ]);

        const event_datas = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityCreated');

        for (let i = 0; i < event_datas.length; i++) {
            assertIdentityCreated(context.substrateWallet.alice, event_datas[i])

            const [ethereum_validations] = await buildValidations(
                context,
                [event_datas[i]],
                [identities[i]],
                'ethereum',
                'single',
                [context.substrateWallet.alice],
                ethereumSigners
            );
            validations.push(ethereum_validations)
        }

    });

    step('batch test: verify identity', async function () {
        let txs = await buildIdentityTxs(
            context,
            [context.substrateWallet.alice],
            identities,
            'verifyIdentity',
            validations
        );

        let resp_events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'identityManagement', [
            'IdentityVerified',
        ]);

        let event_datas = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityVerified');

        for (let i = 0; i < event_datas.length; i++) {
            assertIdentityVerified(context.substrateWallet.alice, event_datas[i]);
        }
    });

    step('batch test: remove identities', async function () {
        // let txs = await buildIdentityTxs(context, [context.substrateWallet.alice], identities, 'removeIdentity');
        // let resp_remove_events = await sendTxsWithUtility(
        //     context,
        //     context.substrateWallet.alice,
        //     txs,
        //     'identityManagement',
        //     ['IdentityRemoved']
        // );
        // const resp_event_datas = await handleIdentityEvents(context, aesKey, resp_remove_events, 'IdentityRemoved');
        // for (let i = 0; i < resp_event_datas.length; i++) {
        //     assertIdentityRemoved(context.substrateWallet.alice, resp_event_datas[i]);
        // }
    });
});
