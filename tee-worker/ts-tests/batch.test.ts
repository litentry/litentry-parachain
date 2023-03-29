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
import { LitentryIdentity, LitentryValidationData, TransactionSubmit } from './common/type-definitions';
import { multiAccountTxSender, sendTxsWidthUtility } from './common/transactions';

describeLitentry('Test Batch Utility', 0, (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    let identities: LitentryIdentity[] = [];
    let validations: LitentryValidationData[] = [];
    step('set user shielding key', async function () {
        let [alice_txs] = (await buildIdentityTxs(
            context,
            [context.substrateWallet.alice],
            [],
            'setUserShieldingKey',
            'batch'
        )) as TransactionSubmit[];
        const resp_events = await multiAccountTxSender(
            context,
            [alice_txs],
            [context.substrateWallet.alice, context.substrateWallet.bob],
            'identityManagement',
            ['UserShieldingKeySet']
        );
        const [alice] = await handleIdentityEvents(context, aesKey, resp_events, 'UserShieldingKeySet');
        assert.equal(alice, u8aToHex(context.substrateWallet.alice.addressRaw), 'alice shielding key should be set');
    });

    step('batch test: create identities', async function () {
        const twiiter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2');
        const ethereum1_identity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Ethereum', 'Evm');
        const ethereum2_identity = await buildIdentityHelper(context.ethersWallet.bob.address, 'Ethereum', 'Evm');

        const substrate_identity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'Litentry',
            'Substrate'
        );

        identities = [twiiter_identity, ethereum1_identity, ethereum2_identity, substrate_identity];
        let txs = await buildIdentityTxs(
            context,
            [context.substrateWallet.alice],
            identities,
            'createIdentity',
            'utility'
        );

        let resp_events = await sendTxsWidthUtility(context, context.substrateWallet.alice, txs, 'identityManagement', [
            'IdentityCreated',
        ]);
        const [twitter_event_data, ethereum1_event_data, ethereum2_event_data, substrate_event_data] =
            await handleIdentityEvents(context, aesKey, resp_events, 'IdentityCreated');

        //Alice check twitter1 identity
        assertIdentityCreated(context.substrateWallet.alice, twitter_event_data);

        const [twitter_validations] = await buildValidations(
            context,
            [twitter_event_data],
            [twiiter_identity],
            'twitter',
            'single',
            [context.substrateWallet.alice]
        );

        //Alice check twitter2 identity
        assertIdentityCreated(context.substrateWallet.alice, ethereum1_event_data);
        const [ethereum1_validations] = await buildValidations(
            context,
            [ethereum1_event_data],
            [ethereum1_identity],
            'ethereum',
            'single',
            [context.substrateWallet.alice],
            [context.ethersWallet.alice]
        );

        //Alice check ethereum identity
        assertIdentityCreated(context.substrateWallet.alice, ethereum2_event_data);
        const [ethereum2_validations] = await buildValidations(
            context,
            [ethereum2_event_data],
            [ethereum2_identity],
            'ethereum',
            'single',
            [context.substrateWallet.alice],
            [context.ethersWallet.bob]
        );

        //Alice check substrate identity
        assertIdentityCreated(context.substrateWallet.alice, substrate_event_data);
        const [substrate_validations] = await buildValidations(
            context,
            [substrate_event_data],
            [substrate_identity],
            'substrate',
            'single',
            [context.substrateWallet.alice]
        );

        validations = [twitter_validations, ethereum1_validations, ethereum2_validations, substrate_validations];
    });

    step('batch test: verify identity', async function () {
        let txs = await buildIdentityTxs(
            context,
            [context.substrateWallet.alice],
            identities,
            'verifyIdentity',
            'utility',
            validations
        );

        let resp_events = await sendTxsWidthUtility(context, context.substrateWallet.alice, txs, 'identityManagement', [
            'IdentityVerified',
        ]);

        const [
            twitter_identity_verified,
            ethereum1_identity_verified,
            ethereum2_identity_verified,
            substrate_identity_verified,
        ] = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityVerified');

        assertIdentityVerified(context.substrateWallet.alice, twitter_identity_verified);
        assertIdentityVerified(context.substrateWallet.alice, ethereum1_identity_verified);

        // assertIdentityVerified(context.substrateWallet.alice, twitter2_identity_verified);
        assertIdentityVerified(context.substrateWallet.alice, ethereum2_identity_verified);
        assertIdentityVerified(context.substrateWallet.alice, substrate_identity_verified);
    });

    step('batch test: remove identities', async function () {
        let txs = await buildIdentityTxs(
            context,
            [context.substrateWallet.alice],
            identities,
            'removeIdentity',
            'utility'
        );
        let resp_remove_events = await sendTxsWidthUtility(
            context,
            context.substrateWallet.alice,
            txs,
            'identityManagement',
            ['IdentityRemoved']
        );
        const [
            twitter1_identity_removed,
            ethereum1_identity_removed,
            ethereum_2identity_removed,
            substrate_identity_removed,
        ] = await handleIdentityEvents(context, aesKey, resp_remove_events, 'IdentityRemoved');
        assertIdentityRemoved(context.substrateWallet.alice, twitter1_identity_removed);
        assertIdentityRemoved(context.substrateWallet.alice, ethereum1_identity_removed);
        assertIdentityRemoved(context.substrateWallet.alice, ethereum_2identity_removed);
        assertIdentityRemoved(context.substrateWallet.alice, substrate_identity_removed);
    });
});
