import {
    describeLitentry,
    encryptWithTeeShieldingKey,
    generateVerificationMessage,
    checkErrorDetail,
    checkUserShieldingKeys,
    checkUserChallengeCode,
    checkIDGraph,
    buildIdentityHelper,
    buildIdentityTxs,
    handleIdentityEvents,
    buildValidations,
    assertInitialIDGraphCreated,
} from './common/utils';
import { hexToU8a, u8aConcat, u8aToHex, u8aToU8a, stringToU8a } from '@polkadot/util';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import {
    LitentryIdentity,
    LitentryValidationData,
    SubstrateIdentity,
    TransactionSubmit,
} from './common/type-definitions';
import { HexString } from '@polkadot/util/types';
import { multiAccountTxSender, sendTxsWithUtility } from './common/transactions';
import { assertIdentityVerified, assertIdentityCreated, assertIdentityRemoved } from './common/utils';
import { ethers } from 'ethers';
const substrateExtensionIdentity = <LitentryIdentity>{
    Substrate: <SubstrateIdentity>{
        address: '0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48', //Bob
        network: 'Litentry',
    },
};

describeLitentry('Test Identity', 0, (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    const errorAesKey = '0xError';
    const errorCiphertext = '0xError';
    //random wrong msg
    const wrong_msg = '0x693d9131808e7a8574c7ea5eb7813bdf356223263e61fa8fe2ee8e434508bc75';
    var signature_substrate;
    let alice_identities: LitentryIdentity[] = [];
    let bob_identities: LitentryIdentity[] = [];
    let alice_validations: LitentryValidationData[] = [];
    let bob_validations: LitentryValidationData[] = [];

    step('check user sidechain storage before create', async function () {
        const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2');
        const identity_hex = context.api.createType('LitentryIdentity', twitter_identity).toHex();
        const resp_shieldingKey = await checkUserShieldingKeys(
            context,
            'IdentityManagement',
            'UserShieldingKeys',
            u8aToHex(context.substrateWallet.alice.addressRaw)
        );
        assert.equal(resp_shieldingKey, '0x', 'shielding key should be empty before set');

        const resp_challengecode = await checkUserChallengeCode(
            context,
            'IdentityManagement',
            'ChallengeCodes',
            u8aToHex(context.substrateWallet.alice.addressRaw),
            identity_hex
        );

        assert.equal(resp_challengecode, '0x', 'challengecode should be empty before create');
    });

    step('Invalid user shielding key', async function () {
        let identity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Ethereum', 'Evm');
        let txs = await buildIdentityTxs(context, context.substrateWallet.alice, [identity], 'createIdentity');

        let resp_events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'identityManagement', [
            'CreateIdentityFailed',
        ]);
        await checkErrorDetail(resp_events, 'UserShieldingKeyNotFound', true);
    });

    step('set user shielding key', async function () {
        let [alice_txs] = (await buildIdentityTxs(
            context,
            [context.substrateWallet.alice],
            [],
            'setUserShieldingKey'
        )) as TransactionSubmit[];
        let [bob_txs] = (await buildIdentityTxs(
            context,
            [context.substrateWallet.bob],
            [],
            'setUserShieldingKey'
        )) as TransactionSubmit[];
        const resp_events = await multiAccountTxSender(
            context,
            [alice_txs, bob_txs],
            [context.substrateWallet.alice, context.substrateWallet.bob],
            'identityManagement',
            ['UserShieldingKeySet']
        );
        const [alice, bob] = await handleIdentityEvents(context, aesKey, resp_events, 'UserShieldingKeySet');
        await assertInitialIDGraphCreated(context.api, context.substrateWallet.alice, alice);
        await assertInitialIDGraphCreated(context.api, context.substrateWallet.bob, bob);
    });

    step('check user shielding key from sidechain storage after setUserShieldingKey', async function () {
        const resp_shieldingKey = await checkUserShieldingKeys(
            context,
            'IdentityManagement',
            'UserShieldingKeys',
            u8aToHex(context.substrateWallet.alice.addressRaw)
        );
        assert.equal(resp_shieldingKey, aesKey, 'resp_shieldingKey should be equal aesKey after set');
    });

    step('check idgraph from sidechain storage before create', async function () {
        // the main address should be already inside the IDGraph
        const main_identity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'LitentryRococo',
            'Substrate'
        );
        const identity_hex = context.api.createType('LitentryIdentity', main_identity).toHex();
        const resp_id_graph = await checkIDGraph(
            context,
            'IdentityManagement',
            'IDGraphs',
            u8aToHex(context.substrateWallet.alice.addressRaw),
            identity_hex
        );
        assert.equal(
            resp_id_graph.verification_request_block,
            0,
            'verification_request_block should be 0 for main address'
        );
        assert.equal(resp_id_graph.linking_request_block, 0, 'linking_request_block should be 0 for main address');
        assert.equal(resp_id_graph.is_verified, true, 'IDGraph is_verified should be true for main address');
        // TODO: check IDGraph.length == 1 in the sidechain storage
    });
    step('create identities', async function () {
        //Alice
        const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2');
        const ethereum_identity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Ethereum', 'Evm');
        const alice_substrate_identity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'Litentry',
            'Substrate'
        );

        //Bob
        const bob_substrate_identity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.bob.addressRaw),
            'Litentry',
            'Substrate'
        );

        alice_identities = [twitter_identity, ethereum_identity, alice_substrate_identity];
        bob_identities = [bob_substrate_identity];

        let alice_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            alice_identities,
            'createIdentity'
        );

        let alice_resp_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            alice_txs,
            'identityManagement',
            ['IdentityCreated']
        );

        const [twitter_event_data, ethereum_event_data, substrate_event_data] = await handleIdentityEvents(
            context,
            aesKey,
            alice_resp_events,
            'IdentityCreated'
        );

        //Alice check twitter identity
        assertIdentityCreated(context.substrateWallet.alice, twitter_event_data);

        const alice_twitter_validations = await buildValidations(
            context,
            [twitter_event_data],
            [twitter_identity],
            'twitter',
            context.substrateWallet.alice
        );

        //Alice check ethereum identity
        assertIdentityCreated(context.substrateWallet.alice, ethereum_event_data);
        const alice_ethereum_validations = await buildValidations(
            context,
            [ethereum_event_data],
            [ethereum_identity],
            'ethereum',
            context.substrateWallet.alice,
            [context.ethersWallet.alice]
        );

        //Alice check substrate identity
        assertIdentityCreated(context.substrateWallet.alice, substrate_event_data);
        const alice_substrate_validations = await buildValidations(
            context,
            [substrate_event_data],
            [alice_substrate_identity],
            'substrate',
            context.substrateWallet.alice
        );

        alice_validations = [
            ...alice_twitter_validations,
            ...alice_ethereum_validations,
            ...alice_substrate_validations,
        ];

        //Bob check extension substrate identity
        //https://github.com/litentry/litentry-parachain/issues/1137
        let bob_txs = await buildIdentityTxs(context, context.substrateWallet.bob, bob_identities, 'createIdentity');

        let bob_resp_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.bob,
            bob_txs,
            'identityManagement',
            ['IdentityCreated']
        );

        const [resp_extension_data] = await handleIdentityEvents(context, aesKey, bob_resp_events, 'IdentityCreated');

        assertIdentityCreated(context.substrateWallet.bob, resp_extension_data);
        if (resp_extension_data) {
            console.log('substrateExtensionIdentity challengeCode: ', resp_extension_data.challengeCode);
            const substrateExtensionValidationData = <LitentryValidationData>{
                Web3Validation: {
                    Substrate: {
                        message: `0x${Buffer.from('mock_message', 'utf8').toString('hex')}`,
                        signature: {
                            Sr25519: '' as HexString,
                        },
                    },
                },
            };
            const msg = generateVerificationMessage(
                context,
                hexToU8a(resp_extension_data.challengeCode),
                context.substrateWallet.bob.addressRaw,
                substrateExtensionIdentity
            );
            console.log('post verification msg to substrate: ', msg);
            substrateExtensionValidationData!.Web3Validation!.Substrate!.message = msg;
            // sign the wrapped version as in polkadot-extension
            signature_substrate = context.substrateWallet.bob.sign(
                u8aConcat(stringToU8a('<Bytes>'), u8aToU8a(msg), stringToU8a('</Bytes>'))
            );
            substrateExtensionValidationData!.Web3Validation!.Substrate!.signature!.Sr25519 =
                u8aToHex(signature_substrate);
            assert.isNotEmpty(resp_extension_data.challengeCode, 'challengeCode empty');
            bob_validations = [substrateExtensionValidationData];
        }
    });

    step('check IDGraph before verifyIdentity and after createIdentity', async function () {
        const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2');
        const identity_hex = context.api.createType('LitentryIdentity', twitter_identity).toHex();

        const resp_id_graph = await checkIDGraph(
            context,
            'IdentityManagement',
            'IDGraphs',
            u8aToHex(context.substrateWallet.alice.addressRaw),
            identity_hex
        );
        assert.notEqual(
            resp_id_graph.linking_request_block,
            null,
            'linking_request_block should not be null after createIdentity'
        );
        assert.equal(resp_id_graph.is_verified, false, 'is_verified should be false before verifyIdentity');
    });

    step('verify invalid identities', async function () {
        const twitter_identity = alice_identities[0];
        const ethereum_validation = alice_validations[1];

        //verify twitter identity with ethereum validation
        let alice_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [twitter_identity],
            'verifyIdentity',
            [ethereum_validation]
        );
        let alice_resp_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            alice_txs,
            'identityManagement',
            ['VerifyIdentityFailed']
        );
        const verified_event_datas = await handleIdentityEvents(context, aesKey, alice_resp_events, 'Failed');
        await checkErrorDetail(verified_event_datas, 'InvalidIdentity', false);
    });
    step('verify wrong signature', async function () {
        const ethereum_identity = alice_identities[1];

        //use wrong signature
        const signature_ethereum = (await context.ethersWallet.alice!.signMessage(
            ethers.utils.arrayify(wrong_msg)
        )) as HexString;

        const ethereumValidationData: LitentryValidationData = {
            Web3Validation: {
                Evm: {
                    message: wrong_msg as HexString,
                    signature: {
                        Ethereum: signature_ethereum as HexString,
                    },
                },
            },
        };
        let alice_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [ethereum_identity],
            'verifyIdentity',
            [ethereumValidationData]
        );
        let alice_resp_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            alice_txs,
            'identityManagement',
            ['VerifyIdentityFailed']
        );
        const verified_event_datas = await handleIdentityEvents(context, aesKey, alice_resp_events, 'Failed');

        await checkErrorDetail(verified_event_datas, 'VerifyEvmSignatureFailed', false);
    });
    step('verify identities', async function () {
        //Alice verify all identities
        let alice_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            alice_identities,
            'verifyIdentity',
            alice_validations
        );
        let alice_resp_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            alice_txs,
            'identityManagement',
            ['IdentityVerified']
        );
        let bob_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.bob,
            bob_identities,
            'verifyIdentity',
            bob_validations
        );

        let bob_resp_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.bob,
            bob_txs,
            'identityManagement',
            ['IdentityVerified']
        );
        const verified_event_datas = await handleIdentityEvents(context, aesKey, alice_resp_events, 'IdentityVerified');
        const [substrate_extension_identity_verified] = await handleIdentityEvents(
            context,
            aesKey,
            bob_resp_events,
            'IdentityVerified'
        );
        //Alice
        assertIdentityVerified(context.substrateWallet.alice, verified_event_datas);

        //Bob
        assertIdentityVerified(context.substrateWallet.bob, [substrate_extension_identity_verified]);
    });

    step('check IDGraph after verifyIdentity', async function () {
        const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2');
        const identity_hex = context.api.createType('LitentryIdentity', twitter_identity).toHex();

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
    });

    step('verify error identities', async function () {
        // verify same identities(alice) to one account
        let alice_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            alice_identities,
            'verifyIdentity',
            alice_validations
        );
        let alice_resp_same_verify_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            alice_txs,
            'identityManagement',
            ['VerifyIdentityFailed']
        );
        const alice_resp_same_verify_event_datas = await handleIdentityEvents(
            context,
            aesKey,
            alice_resp_same_verify_events,
            'Failed'
        );
        await checkErrorDetail(alice_resp_same_verify_event_datas, 'ChallengeCodeNotFound', false);

        //verify an identity(charlie) to an account but it isn't created before
        let charlie_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.charlie,
            alice_identities,
            'verifyIdentity',
            alice_validations
        );
        let charlie_resp_same_verify_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.charlie,
            charlie_txs,
            'identityManagement',
            ['VerifyIdentityFailed']
        );
        const charlie_resp_same_verify_event_datas = await handleIdentityEvents(
            context,
            aesKey,
            charlie_resp_same_verify_events,
            'Failed'
        );
        await checkErrorDetail(charlie_resp_same_verify_event_datas, 'ChallengeCodeNotFound', false);
    });

    step('remove identities', async function () {
        // Alice remove all identities
        let alice_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            alice_identities,
            'removeIdentity'
        );
        let alice_resp_remove_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            alice_txs,
            'identityManagement',
            ['IdentityRemoved']
        );
        const [twitter_identity_removed, ethereum_identity_removed, substrate_identity_removed] =
            await handleIdentityEvents(context, aesKey, alice_resp_remove_events, 'IdentityRemoved');

        // Bob remove substrate identities
        let bob_txs = await buildIdentityTxs(context, context.substrateWallet.bob, bob_identities, 'removeIdentity');
        let bob_resp_remove_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.bob,
            bob_txs,
            'identityManagement',
            ['IdentityRemoved']
        );
        const [substrate_extension_identity_removed] = await handleIdentityEvents(
            context,
            aesKey,
            bob_resp_remove_events,
            'IdentityRemoved'
        );
        //Alice
        assertIdentityRemoved(context.substrateWallet.alice, twitter_identity_removed);
        assertIdentityRemoved(context.substrateWallet.alice, ethereum_identity_removed);
        assertIdentityRemoved(context.substrateWallet.alice, substrate_identity_removed);

        // Bob
        assertIdentityRemoved(context.substrateWallet.bob, substrate_extension_identity_removed);
    });

    step('check challengeCode from storage after removeIdentity', async function () {
        const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2');
        const identity_hex = context.api.createType('LitentryIdentity', twitter_identity).toHex();
        const resp_challengecode = await checkUserChallengeCode(
            context,
            'IdentityManagement',
            'ChallengeCodes',
            u8aToHex(context.substrateWallet.alice.addressRaw),
            identity_hex
        );
        assert.equal(resp_challengecode, '0x', 'challengecode should be empty after removeIdentity');
    });

    step('check IDGraph after removeIdentity', async function () {
        const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2');
        const identity_hex = context.api.createType('LitentryIdentity', twitter_identity).toHex();

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
    });
    step('remove prime identity NOT allowed', async function () {
        // create substrate identity
        const alice_substrate_identity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'Litentry',
            'Substrate'
        );
        let alice_create_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [alice_substrate_identity],
            'createIdentity'
        );
        let alice_resp_create__events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            alice_create_txs,
            'identityManagement',
            ['IdentityCreated']
        );
        const [substrate_create_event_data] = await handleIdentityEvents(
            context,
            aesKey,
            alice_resp_create__events,
            'IdentityCreated'
        );
        assertIdentityCreated(context.substrateWallet.alice, substrate_create_event_data);

        // remove substrate identity
        let alice_remove_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [alice_substrate_identity],
            'removeIdentity'
        );
        await sendTxsWithUtility(context, context.substrateWallet.alice, alice_remove_txs, 'identityManagement', [
            'IdentityRemoved',
        ]);

        // remove prime identity
        const substratePrimeIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'LitentryRococo',
            'Substrate'
        );

        let prime_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [substratePrimeIdentity],
            'removeIdentity'
        );
        let prime_resp_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            prime_txs,
            'identityManagement',
            ['RemoveIdentityFailed']
        );
        const prime_resp_event_datas = await handleIdentityEvents(context, aesKey, prime_resp_events, 'Failed');

        await checkErrorDetail(prime_resp_event_datas, 'RemovePrimeIdentityDisallowed', false);
    });

    step('remove error identities', async function () {
        //remove a nonexistent identity
        //context.substrateWallet.alice has aleady removed all identities in step('remove identities')
        let alice_remove_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            alice_identities,
            'removeIdentity'
        );
        let alice_resp_remove_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            alice_remove_txs,
            'identityManagement',
            ['RemoveIdentityFailed']
        );
        const alice_resp_remove_event_datas = await handleIdentityEvents(
            context,
            aesKey,
            alice_resp_remove_events,
            'Failed'
        );

        await checkErrorDetail(alice_resp_remove_event_datas, 'IdentityNotExist', false);

        //charlie doesn't have a challenge code,use alice identity
        let charlie_remove_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.charlie,
            alice_identities,
            'removeIdentity'
        );
        let charile_resp_remove_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.charlie,
            charlie_remove_txs,
            'identityManagement',
            ['RemoveIdentityFailed']
        );

        const charile_resp_remove_events_data = await handleIdentityEvents(
            context,
            aesKey,
            charile_resp_remove_events,
            'Failed'
        );

        await checkErrorDetail(charile_resp_remove_events_data, 'UserShieldingKeyNotFound', false);
    });

    step('set error user shielding key', async function () {
        const error_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, hexToU8a(errorAesKey)).toString('hex');
        const error_tx = context.api.tx.identityManagement.setUserShieldingKey(
            context.mrEnclave,
            `0x${error_ciphertext}`
        );

        let resp_error_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            [{ tx: error_tx }] as any,
            'identityManagement',
            ['SetUserShieldingKeyFailed']
        );

        let error_event_datas = await handleIdentityEvents(context, aesKey, resp_error_events, 'Failed');

        await checkErrorDetail(error_event_datas, 'ImportError', false);
    });

    step('create error identities', async function () {
        const error_tx = context.api.tx.identityManagement.createIdentity(
            context.mrEnclave,
            context.substrateWallet.alice.address,
            errorCiphertext,
            null
        );
        let resp_error_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            [{ tx: error_tx }] as any,
            'identityManagement',
            ['CreateIdentityFailed']
        );
        let error_event_datas = await handleIdentityEvents(context, aesKey, resp_error_events, 'Failed');
        await checkErrorDetail(error_event_datas, 'ImportError', false);
    });
});
