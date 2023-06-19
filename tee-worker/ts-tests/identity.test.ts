import {
    describeLitentry,
    encryptWithTeeShieldingKey,
    generateVerificationMessage,
    checkErrorDetail,
    checkIDGraph,
    buildIdentityHelper,
    buildIdentityTxs,
    handleIdentityEvents,
    buildValidations,
    assertInitialIDGraphCreated,
    checkUserShieldingKeys,
    assertIdentityLinked,
    assertIdentityRemoved,
} from './common/utils';
import { aesKey } from './common/call';
import { SubstrateNetworkMapping } from './common/helpers';
import { hexToU8a, u8aConcat, u8aToHex, u8aToU8a, stringToU8a } from '@polkadot/util';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { multiAccountTxSender, sendTxsWithUtility } from './common/transactions';
import type { LitentryPrimitivesIdentity } from '@polkadot/types/lookup';
import type { LitentryValidationData } from './parachain-interfaces/identity/types';
import type { TransactionSubmit } from './common/type-definitions';
import type { HexString } from '@polkadot/util/types';
import { Event } from '@polkadot/types/interfaces';
import { ethers } from 'ethers';

const substrateExtensionIdentity = {
    Substrate: {
        address: '0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48', //Bob
        network: 'Litentry',
    },
} as unknown as LitentryPrimitivesIdentity;

describeLitentry('Test Identity', 0, (context) => {
    const errorAesKey = '0xError';
    const errorCiphertext = '0xError';
    // random wrong msg
    const wrong_msg = '0x693d9131808e7a8574c7ea5eb7813bdf356223263e61fa8fe2ee8e434508bc75';
    var signature_substrate;
    let alice_identities: LitentryPrimitivesIdentity[] = [];
    let bob_identities: LitentryPrimitivesIdentity[] = [];
    let alice_validations: LitentryValidationData[] = [];
    let bob_validations: LitentryValidationData[] = [];
    const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

    step('check user sidechain storage before create', async function () {
        const resp_shieldingKey = await checkUserShieldingKeys(
            context,
            'IdentityManagement',
            'UserShieldingKeys',
            u8aToHex(context.substrateWallet.alice.addressRaw)
        );
        assert.equal(resp_shieldingKey, '0x', 'shielding key should be empty before set');
    });

    step('Invalid user shielding key', async function () {
        let identity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Ethereum', 'Evm', context);
        // use empty `alice_validations`, the `UserShieldingKeyNotFound` error should be emitted before verification
        let txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [identity],
            'linkIdentity',
            alice_validations
        );

        let resp_events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'identityManagement', [
            'LinkIdentityFailed',
        ]);
        await checkErrorDetail(resp_events, 'UserShieldingKeyNotFound');
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

        await assertInitialIDGraphCreated(
            context,
            [context.substrateWallet.alice, context.substrateWallet.bob],
            resp_events
        );
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

    step('check idgraph from sidechain storage before linking', async function () {
        // the main address should be already inside the IDGraph
        const main_identity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            SubstrateNetworkMapping[context.chainID],
            'Substrate',
            context
        );
        const identity_hex = main_identity.toHex();
        const resp_id_graph = await checkIDGraph(
            context,
            'IdentityManagement',
            'IDGraphs',
            u8aToHex(context.substrateWallet.alice.addressRaw),
            identity_hex
        );
        assert.isTrue(resp_id_graph.linkBlock.toNumber() > 0, 'linkBlock should be greater than 0 for main address');
        assert.isTrue(resp_id_graph.status.isActive, 'status should be active for main address');
        // TODO: check IDGraph.length == 1 in the sidechain storage
    });

    step('link identities', async function () {
        // Alice
        const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2', context);
        const ethereum_identity = await buildIdentityHelper(
            context.ethersWallet.alice.address,
            'Ethereum',
            'Evm',
            context
        );
        const alice_substrate_identity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'Litentry',
            'Substrate',
            context
        );

        // Bob
        const bob_substrate_identity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.bob.addressRaw),
            'Litentry',
            'Substrate',
            context
        );

        alice_identities = [twitter_identity, ethereum_identity, alice_substrate_identity];

        bob_identities = [bob_substrate_identity];

        // TODO: being lazy - the nonce here is hardcoded
        //       it's better to retrieve the starting nonce from the sidechain and increment
        //       it for each such request, similar to the construction of substrate tx
        //       However, beware that we should query the nonce of the enclave-signer-account
        //       not alice or bob, as it's the indirect calls are signed by the enclave signer
        const alice_twitter_validations = await buildValidations(
            context,
            [twitter_identity],
            3,
            'twitter',
            context.substrateWallet.alice
        );

        const alice_ethereum_validations = await buildValidations(
            context,
            [ethereum_identity],
            4,
            'ethereum',
            context.substrateWallet.alice,
            [context.ethersWallet.alice]
        );

        const alice_substrate_validations = await buildValidations(
            context,
            [alice_substrate_identity],
            5,
            'substrate',
            context.substrateWallet.alice
        );

        alice_validations = [
            ...alice_twitter_validations,
            ...alice_ethereum_validations,
            ...alice_substrate_validations,
        ];

        let alice_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            alice_identities,
            'linkIdentity',
            alice_validations
        );

        let alice_resp_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            alice_txs,
            'identityManagement',
            ['IdentityLinked']
        );

        const alice_data = await handleIdentityEvents(context, aesKey, alice_resp_events, 'IdentityLinked');

        assertIdentityLinked(context, context.substrateWallet.alice, alice_resp_events, alice_identities);

        // Bob check extension substrate identity
        // https://github.com/litentry/litentry-parachain/issues/1137
        const substrateExtensionValidationData = {
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
            context.substrateWallet.bob.addressRaw,
            substrateExtensionIdentity,
            // 9 because each previous linking of Alice's identity would trigger an additional nonce bump
            // due to the callback trustedCall
            9
        );
        console.log('post verification msg to substrate: ', msg);
        substrateExtensionValidationData.Web3Validation.Substrate.message = msg;
        // sign the wrapped version as in polkadot-extension
        signature_substrate = context.substrateWallet.bob.sign(
            u8aConcat(stringToU8a('<Bytes>'), u8aToU8a(msg), stringToU8a('</Bytes>'))
        );
        substrateExtensionValidationData!.Web3Validation.Substrate.signature.Sr25519 = u8aToHex(signature_substrate);
        const bob_substrate_validation = context.api.createType(
            'LitentryValidationData',
            substrateExtensionValidationData
        ) as unknown as LitentryValidationData;
        bob_validations = [bob_substrate_validation];

        let bob_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.bob,
            bob_identities,
            'linkIdentity',
            bob_validations
        );

        let bob_resp_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.bob,
            bob_txs,
            'identityManagement',
            ['IdentityLinked']
        );
        assertIdentityLinked(context, context.substrateWallet.bob, bob_resp_events, bob_identities);
    });

    step('check IDGraph after LinkIdentity', async function () {
        const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2', context);
        const identity_hex = context.api.createType('LitentryIdentity', twitter_identity).toHex();

        const resp_id_graph = await checkIDGraph(
            context,
            'IdentityManagement',
            'IDGraphs',
            u8aToHex(context.substrateWallet.alice.addressRaw),
            identity_hex
        );
        assert.isTrue(resp_id_graph.linkBlock.toNumber() > 0, 'linkBlock should be greater than 0');
        assert.isTrue(resp_id_graph.status.isActive, 'status should be active');
    });

    step('link invalid identities', async function () {
        const twitter_identity = alice_identities[0];
        const ethereum_validation = alice_validations[1];

        // link twitter identity with ethereum validation data
        // the `InvalidIdentity` error should be emitted prior to `AlreadyLinked` error
        let alice_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [twitter_identity],
            'linkIdentity',
            [ethereum_validation]
        );
        let alice_resp_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            alice_txs,
            'identityManagement',
            ['LinkIdentityFailed']
        );
        await checkErrorDetail(alice_resp_events, 'InvalidIdentity');
    });

    step('link identities with wrong signature', async function () {
        const ethereum_identity = alice_identities[1];

        // link eth identity with wrong validation data
        // the `VerifyEvmSignatureFailed` error should be emitted prior to `AlreadyLinked` error
        const signature_ethereum = (await context.ethersWallet.alice.signMessage(
            ethers.utils.arrayify(wrong_msg)
        )) as HexString;

        const ethereumValidationData = {
            Web3Validation: {
                Evm: {
                    message: wrong_msg as HexString,
                    signature: {
                        Ethereum: signature_ethereum as HexString,
                    },
                },
            },
        };
        const ethereum_validation_data: LitentryValidationData = context.api.createType(
            'LitentryValidationData',
            ethereumValidationData
        ) as unknown as LitentryValidationData;
        context;
        let alice_txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [ethereum_identity],
            'linkIdentity',
            [ethereum_validation_data]
        );
        let alice_resp_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            alice_txs,
            'identityManagement',
            ['LinkIdentityFailed']
        );

        await checkErrorDetail(alice_resp_events, 'VerifyEvmSignatureFailed');
    });

    // TODO: testcase for linking prime address and already linked address

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

        // Bob remove substrate identities
        let bob_txs = await buildIdentityTxs(context, context.substrateWallet.bob, bob_identities, 'removeIdentity');
        let bob_resp_remove_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.bob,
            bob_txs,
            'identityManagement',
            ['IdentityRemoved']
        );

        // Alice check identity
        assertIdentityRemoved(context, context.substrateWallet.alice, alice_resp_remove_events);

        // Bob check identity
        assertIdentityRemoved(context, context.substrateWallet.bob, alice_resp_remove_events);
    });

    step('check IDGraph after removeIdentity', async function () {
        const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2', context);
        const identity_hex = twitter_identity.toHex();

        // TODO: we should verify the IDGraph is empty
    });

    step('remove prime identity is disallowed', async function () {
        // remove prime identity
        const substratePrimeIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            SubstrateNetworkMapping[context.chainID],
            'Substrate',
            context
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

        await checkErrorDetail(prime_resp_events, 'RemovePrimeIdentityDisallowed');
    });

    step('remove error identities', async function () {
        // Remove a nonexistent identity
        // context.substrateWallet.alice has aleady removed all identities in step('remove identities')
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

        await checkErrorDetail(alice_resp_remove_events, 'IdentityNotExist');

        // remove a wrong identity (alice) for charlie
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

        await checkErrorDetail(charile_resp_remove_events, 'UserShieldingKeyNotFound');
    });

    step('set error user shielding key', async function () {
        const error_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, hexToU8a(errorAesKey)).toString(
            'hex'
        );
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

        await checkErrorDetail(resp_error_events, 'ImportError');
    });

    step('exceeding IDGraph limit not allowed', async function () {
        // TODO: this needs to be reworked
        //       we have to provide validation data when linking
    });
});
