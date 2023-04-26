import {
    describeLitentry,
    buildIdentityTxs,
    handleIdentityEvents,
    buildIdentityHelper,
    buildValidations,
    checkErrorDetail,
    checkIDGraph,
} from './common/utils';

import { u8aToHex } from '@polkadot/util';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { LitentryIdentity, LitentryValidationData } from './common/type-definitions';
import { multiAccountTxSender, sendTxsWithUtility } from './common/transactions';
import {
    generateWeb3Wallets,
    assertIdentityVerified,
    assertIdentityCreated,
    assertIdentityRemoved,
} from './common/utils';
import { ethers } from 'ethers';

describeLitentry('Test Batch Utility', 0, (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    let identities: LitentryIdentity[] = [];
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

    step('batch test: create identities', async function () {
        for (let index = 0; index < ethereumSigners.length; index++) {
            const signer = ethereumSigners[index];
            const ethereum_identity = await buildIdentityHelper(signer.address, 'Ethereum', 'Evm');
            identities.push(ethereum_identity);

            //check idgraph from sidechain storage before create
            const identity_hex = context.api.createType('LitentryIdentity', ethereum_identity).toHex();
            const resp_id_graph = await checkIDGraph(
                context,
                'IdentityManagement',
                'IDGraphs',
                u8aToHex(context.substrateWallet.alice.addressRaw),
                identity_hex
            );
            assert.equal(
                resp_id_graph.verification_request_block,
                null,
                'verification_request_block should  be null before create'
            );
            assert.equal(
                resp_id_graph.linking_request_block,
                null,
                'linking_request_block should  be null before create'
            );

            assert.equal(resp_id_graph.is_verified, false, 'IDGraph is_verified should be equal false before create');
        }
        const txs = await buildIdentityTxs(context, context.substrateWallet.alice, identities, 'createIdentity');

        const resp_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            txs,
            'identityManagement',
            ['IdentityCreated']
        );
        const event_datas = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityCreated');
        for (let i = 0; i < event_datas.length; i++) {
            assertIdentityCreated(context.substrateWallet.alice, event_datas[i]);
        }
        const ethereum_validations = await buildValidations(
            context,
            event_datas,
            identities,
            'ethereum',
            context.substrateWallet.alice,
            ethereumSigners
        );
        validations = [...ethereum_validations];
    });

    step('batch test: create error identities', async function () {
        const txs = await buildIdentityTxs(context, context.substrateWallet.bob, identities, 'createIdentity');

        const resp_events = await sendTxsWithUtility(context, context.substrateWallet.bob, txs, 'identityManagement', [
            'CreateIdentityFailed',
        ]);
        await checkErrorDetail(resp_events, 'UserShieldingKeyNotFound', true);
    });
    step('batch test: verify identity', async function () {
        let txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            identities,
            'verifyIdentity',
            validations
        );

        let resp_events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'identityManagement', [
            'IdentityVerified',
        ]);

        let event_datas = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityVerified');

        assertIdentityVerified(context.substrateWallet.alice, event_datas);
    });

    step('batch test: verify error identity', async function () {
        let txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            identities,
            'verifyIdentity',
            validations
        );
        let resp_events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'identityManagement', [
            'VerifyIdentityFailed',
        ]);
        const resp_event_datas = await handleIdentityEvents(context, aesKey, resp_events, 'Failed');
        await checkErrorDetail(resp_event_datas, 'ChallengeCodeNotFound', false);
    });
    //query here in the hope that the status remains unchanged after verify error identity
    step('batch test:check IDGraph after verifyIdentity', async function () {
        for (let index = 0; index < identities.length; index++) {
            const identity_hex = context.api.createType('LitentryIdentity', identities[index]).toHex();
            const resp_id_graph = await checkIDGraph(
                context,
                'IdentityManagement',
                'IDGraphs',
                u8aToHex(context.substrateWallet.alice.addressRaw),
                identity_hex
            );
            assert.notEqual(
                resp_id_graph.verification_request_block,
                null,
                'verification_request_block should not be null after verifyIdentity'
            );
            assert.equal(resp_id_graph.is_verified, true, 'is_verified should be true after verifyIdentity');
        }
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
        const resp_event_datas = await handleIdentityEvents(context, aesKey, resp_remove_events, 'IdentityRemoved');
        for (let i = 0; i < resp_event_datas.length; i++) {
            assertIdentityRemoved(context.substrateWallet.alice, resp_event_datas[i]);
        }
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
        const resp_event_datas = await handleIdentityEvents(context, aesKey, resp_remove_events, 'Failed');
        await checkErrorDetail(resp_event_datas, 'IdentityNotExist', false);
    });

    //query here in the hope that the status remains unchanged after removes error identity
    step('check IDGraph after removeIdentity', async function () {
        for (let index = 0; index < identities.length; index++) {
            const identity_hex = context.api.createType('LitentryIdentity', identities[index]).toHex();

            const resp_id_graph = await checkIDGraph(
                context,
                'IdentityManagement',
                'IDGraphs',
                u8aToHex(context.substrateWallet.alice.addressRaw),
                identity_hex
            );
            assert.equal(
                resp_id_graph.verification_request_block,
                null,
                'verification_request_block should  be null after removeIdentity'
            );
            assert.equal(
                resp_id_graph.linking_request_block,
                null,
                'linking_request_block should  be null after removeIdentity'
            );
            assert.equal(resp_id_graph.is_verified, false, 'is_verified should be false after removeIdentity');
        }
    });
});
