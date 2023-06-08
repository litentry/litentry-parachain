import {
    describeLitentry,
    buildIdentityTxs,
    handleIdentityEvents,
    buildIdentityHelper,
    buildValidations,
    checkErrorDetail,
    checkIDGraph,
} from './common/utils';
import { aesKey } from './common/call';
import { u8aToHex } from '@polkadot/util';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { multiAccountTxSender, sendTxsWithUtility } from './common/transactions';
import { generateWeb3Wallets, assertIdentityLinked, assertIdentityRemoved } from './common/utils';
import { ethers } from 'ethers';
import type { LitentryPrimitivesIdentity } from '@polkadot/types/lookup';
import type { LitentryValidationData } from './parachain-interfaces/identity/types';

describeLitentry('Test Batch Utility', 0, (context) => {
    let identities: LitentryPrimitivesIdentity[] = [];
    let validations: LitentryValidationData[] = [];
    var ethereumSigners: ethers.Wallet[] = [];

    step('generate web3 wallets', async function () {
        const web3Wallets = await generateWeb3Wallets(10);
        ethereumSigners = web3Wallets.map((web3Signer) => {
            return web3Signer.ethereumWallet;
        });
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
        assert.equal(
            alice.who,
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'alice shielding key should be set'
        );
    });

    step('batch test: link identities', async function () {
        for (let index = 0; index < ethereumSigners.length; index++) {
            const signer = ethereumSigners[index];
            const ethereum_identity = await buildIdentityHelper(signer.address, 'Ethereum', 'Evm', context);
            identities.push(ethereum_identity);
        }

        const ethereum_validations = await buildValidations(
            context,
            identities,
            1,
            'ethereum',
            context.substrateWallet.alice,
            ethereumSigners
        );
        validations = [...ethereum_validations];

        const txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            identities,
            'linkIdentity',
            validations
        );
        let resp_events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'identityManagement', [
            'IdentityLinked',
        ]);
        let event_data = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityLinked');
        assertIdentityLinked(context.substrateWallet.alice, event_data);
    });

    step('batch test: remove identities', async function () {
        let txs = await buildIdentityTxs(context, context.substrateWallet.alice, identities, 'removeIdentity');
        let resp_remove_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            txs,
            'identityManagement',
            ['IdentityRemoved']
        );

        await assertIdentityRemoved(context, context.substrateWallet.alice, resp_remove_events);
    });

    step('batch test: remove error identities', async function () {
        let txs = await buildIdentityTxs(context, context.substrateWallet.alice, identities, 'removeIdentity');
        let resp_remove_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            txs,
            'identityManagement',
            ['RemoveIdentityFailed']
        );
        await checkErrorDetail(resp_remove_events, 'IdentityNotExist');
    });

    // query here in the hope that the status remains unchanged after removes error identity
    step('check IDGraph after removeIdentity', async function () {
        for (let index = 0; index < identities.length; index++) {
            const identity_hex = context.sidechainRegistry
                .createType('LitentryPrimitivesIdentity', identities[index])
                .toHex();

            const resp_id_graph = await checkIDGraph(
                context,
                'IdentityManagement',
                'IDGraphs',
                u8aToHex(context.substrateWallet.alice.addressRaw),
                identity_hex
            );
            assert.equal(
                resp_id_graph.verificationRequestBlock.toHuman(),
                null,
                'verificationRequestBlock should  be null after removeIdentity'
            );
            assert.equal(
                resp_id_graph.creationRequestBlock.toHuman(),
                null,
                'creationRequestBlock should be null after removeIdentity'
            );
            assert.equal(resp_id_graph.isVerified.toHuman(), false, 'isVerified should be false after removeIdentity');
        }
    });
});
