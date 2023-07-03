import {
    describeLitentry,
    encryptWithTeeShieldingKey,
    generateVerificationMessage,
    checkErrorDetail,
    checkIdGraph,
    buildIdentityHelper,
    buildIdentityTxs,
    buildValidations,
    checkUserShieldingKeys,
    assertIdentityLinked,
    assertIdentityRemoved,
    assertInitialIdGraphCreated,
    buildAddressHelper,
} from './common/utils';
import { aesKey } from './common/call';
import { substrateNetworkMapping } from './common/helpers';
import { hexToU8a, u8aConcat, u8aToHex, u8aToU8a, stringToU8a } from '@polkadot/util';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { multiAccountTxSender, sendTxsWithUtility } from './common/transactions';
import type { LitentryPrimitivesIdentity } from '@polkadot/types/lookup';
import type { LitentryValidationData } from './parachain-interfaces/identity/types';
import type { TransactionSubmit } from './common/type-definitions';
import type { HexString } from '@polkadot/util/types';
import { ethers } from 'ethers';

const substrateExtensionIdentity = {
    Substrate: {
        address: '0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48', //Bob
        network: 'Litentry',
    },
} as unknown as LitentryPrimitivesIdentity;

describeLitentry('Test Identity', 0, (context) => {
    const errorAesKey = '0xError';
    // random wrong msg
    const wrongMsg = '0x693d9131808e7a8574c7ea5eb7813bdf356223263e61fa8fe2ee8e434508bc75';
    let signatureSubstrate;
    let aliceIdentities: LitentryPrimitivesIdentity[] = [];
    let bobIdentities: LitentryPrimitivesIdentity[] = [];
    let aliceValidations: LitentryValidationData[] = [];
    let bobValidations: LitentryValidationData[] = [];

    step('check user sidechain storage before create', async function () {
        let aliceAddress = await buildAddressHelper(context.substrateWallet.alice);
        const respShieldingKey = await checkUserShieldingKeys(
            context,
            'IdentityManagement',
            'UserShieldingKeys',
            aliceAddress
        );
        assert.equal(respShieldingKey, '0x', 'shielding key should be empty before set');
    });

    step('Invalid user shielding key', async function () {
        const identity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Ethereum', 'Evm', context);
        // use empty `aliceValidations`, the `UserShieldingKeyNotFound` error should be emitted before verification
        const txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [identity],
            'linkIdentity',
            aliceValidations
        );

        const respEvents = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'identityManagement', [
            'LinkIdentityFailed',
        ]);
        await checkErrorDetail(respEvents, 'UserShieldingKeyNotFound');
    });

    step('set user shielding key', async function () {
        const [aliceTxs] = (await buildIdentityTxs(
            context,
            [context.substrateWallet.alice],
            [],
            'setUserShieldingKey'
        )) as TransactionSubmit[];
        const [bobTxs] = (await buildIdentityTxs(
            context,
            [context.substrateWallet.bob],
            [],
            'setUserShieldingKey'
        )) as TransactionSubmit[];
        const respEvents = await multiAccountTxSender(
            context,
            [aliceTxs, bobTxs],
            [context.substrateWallet.alice, context.substrateWallet.bob],
            'identityManagement',
            ['UserShieldingKeySet']
        );

        await assertInitialIdGraphCreated(
            context,
            [context.substrateWallet.alice, context.substrateWallet.bob],
            respEvents
        );
    });

    step('check user shielding key from sidechain storage after setUserShieldingKey', async function () {
        let aliceAddress = await buildAddressHelper(context.substrateWallet.alice);
        const respShieldingKey = await checkUserShieldingKeys(
            context,
            'IdentityManagement',
            'UserShieldingKeys',
            aliceAddress
        );
        assert.equal(respShieldingKey, aesKey, 'respShieldingKey should be equal aesKey after set');
    });

    step('check idgraph from sidechain storage before linking', async function () {
        let aliceAddress = await buildAddressHelper(context.substrateWallet.alice);

        // the main address should be already inside the IDGraph
        const mainIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            substrateNetworkMapping[context.chainIdentifier],
            'Substrate',
            context
        );
        const identityHex = mainIdentity.toHex();
        const respIdGraph = await checkIdGraph(context, 'IdentityManagement', 'IDGraphs', aliceAddress, identityHex);
        assert.isTrue(respIdGraph.linkBlock.toNumber() > 0, 'linkBlock should be greater than 0 for main address');
        assert.isTrue(respIdGraph.status.isActive, 'status should be active for main address');
        // TODO: check IDGraph.length == 1 in the sidechain storage
    });

    step('link identities', async function () {
        // Alice
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2', context);
        const ethereumIdentity = await buildIdentityHelper(
            context.ethersWallet.alice.address,
            'Ethereum',
            'Evm',
            context
        );
        const aliceSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'Litentry',
            'Substrate',
            context
        );

        // Bob
        const bobSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.bob.addressRaw),
            'Litentry',
            'Substrate',
            context
        );

        aliceIdentities = [twitterIdentity, ethereumIdentity, aliceSubstrateIdentity];

        bobIdentities = [bobSubstrateIdentity];

        // TODO: being lazy - the nonce here is hardcoded
        //       it's better to retrieve the starting nonce from the sidechain and increment
        //       it for each such request, similar to the construction of substrate tx
        //       However, beware that we should query the nonce of the enclave-signer-account
        //       not alice or bob, as it's the indirect calls are signed by the enclave signer
        const aliceTwitterValidations = await buildValidations(
            context,
            [twitterIdentity],
            3,
            'twitter',
            context.substrateWallet.alice,
            []
        );

        const aliceEthereumValidations = await buildValidations(
            context,
            [ethereumIdentity],
            4,
            'ethereum',
            context.substrateWallet.alice,
            [context.ethersWallet.alice]
        );

        const aliceSubstrateValidations = await buildValidations(
            context,
            [aliceSubstrateIdentity],
            5,
            'substrate',
            context.substrateWallet.alice,
            []
        );

        aliceValidations = [...aliceTwitterValidations, ...aliceEthereumValidations, ...aliceSubstrateValidations];

        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            aliceIdentities,
            'linkIdentity',
            aliceValidations
        );

        const aliceRespEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceTxs,
            'identityManagement',
            ['IdentityLinked']
        );

        assertIdentityLinked(context, context.substrateWallet.alice, aliceRespEvents, aliceIdentities);

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
        signatureSubstrate = context.substrateWallet.bob.sign(
            u8aConcat(stringToU8a('<Bytes>'), u8aToU8a(msg), stringToU8a('</Bytes>'))
        );
        substrateExtensionValidationData!.Web3Validation.Substrate.signature.Sr25519 = u8aToHex(signatureSubstrate);
        const bobSubstrateValidation = context.api.createType(
            'LitentryValidationData',
            substrateExtensionValidationData
        ) as unknown as LitentryValidationData;
        bobValidations = [bobSubstrateValidation];

        const bobTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.bob,
            bobIdentities,
            'linkIdentity',
            bobValidations
        );

        const bobRespEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.bob,
            bobTxs,
            'identityManagement',
            ['IdentityLinked']
        );
        assertIdentityLinked(context, context.substrateWallet.bob, bobRespEvents, bobIdentities);
    });

    step('check IDGraph after LinkIdentity', async function () {
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2', context);
        const identityHex = context.api.createType('LitentryIdentity', twitterIdentity).toHex();
        let aliceAddress = await buildAddressHelper(context.substrateWallet.alice);

        const respIdGraph = await checkIdGraph(context, 'IdentityManagement', 'IDGraphs', aliceAddress, identityHex);
        assert.isTrue(respIdGraph.linkBlock.toNumber() > 0, 'linkBlock should be greater than 0');
        assert.isTrue(respIdGraph.status.isActive, 'status should be active');
    });

    step('link invalid identities', async function () {
        const twitterIdentity = aliceIdentities[0];
        const ethereumValidation = aliceValidations[1];

        // link twitter identity with ethereum validation data
        // the `InvalidIdentity` error should be emitted prior to `AlreadyLinked` error
        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [twitterIdentity],
            'linkIdentity',
            [ethereumValidation]
        );
        const aliceRespEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceTxs,
            'identityManagement',
            ['LinkIdentityFailed']
        );
        await checkErrorDetail(aliceRespEvents, 'InvalidIdentity');
    });

    step('link identities with wrong signature', async function () {
        const ethereumIdentity = aliceIdentities[1];

        // link eth identity with wrong validation data
        // the `VerifyEvmSignatureFailed` error should be emitted prior to `AlreadyLinked` error
        const ethereumSignature = (await context.ethersWallet.alice.signMessage(
            ethers.utils.arrayify(wrongMsg)
        )) as HexString;

        const validation = {
            Web3Validation: {
                Evm: {
                    message: wrongMsg as HexString,
                    signature: {
                        Ethereum: ethereumSignature as HexString,
                    },
                },
            },
        };
        const ethereumValidationData: LitentryValidationData = context.api.createType(
            'LitentryValidationData',
            validation
        ) as unknown as LitentryValidationData;
        context;
        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [ethereumIdentity],
            'linkIdentity',
            [ethereumValidationData]
        );
        const aliceRespEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceTxs,
            'identityManagement',
            ['LinkIdentityFailed']
        );

        await checkErrorDetail(aliceRespEvents, 'VerifyEvmSignatureFailed');
    });

    // TODO: testcase for linking prime address and already linked address

    step('remove identities', async function () {
        // Alice remove all identities
        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            aliceIdentities,
            'removeIdentity'
        );
        const aliceRemovedEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceTxs,
            'identityManagement',
            ['IdentityRemoved']
        );

        // Bob remove substrate identities
        const bobTxs = await buildIdentityTxs(context, context.substrateWallet.bob, bobIdentities, 'removeIdentity');
        const bobRemovedEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.bob,
            bobTxs,
            'identityManagement',
            ['IdentityRemoved']
        );

        // Alice check identity
        assertIdentityRemoved(context, context.substrateWallet.alice, aliceRemovedEvents);

        // Bob check identity
        assertIdentityRemoved(context, context.substrateWallet.bob, bobRemovedEvents);
    });

    step('check IDGraph after removeIdentity', async function () {
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2', context);
        const identityHex = twitterIdentity.toHex();

        // TODO: we should verify the IDGraph is empty
    });

    step('remove prime identity is disallowed', async function () {
        // remove prime identity
        const substratePrimeIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            substrateNetworkMapping[context.chainIdentifier],
            'Substrate',
            context
        );

        const primeTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [substratePrimeIdentity],
            'removeIdentity'
        );
        const primeEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            primeTxs,
            'identityManagement',
            ['RemoveIdentityFailed']
        );

        await checkErrorDetail(primeEvents, 'RemovePrimeIdentityDisallowed');
    });

    step('remove error identities', async function () {
        // Remove a nonexistent identity
        // context.substrateWallet.alice has aleady removed all identities in step('remove identities')
        const aliceRemoveTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            aliceIdentities,
            'removeIdentity'
        );
        const aliceRemovedEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceRemoveTxs,
            'identityManagement',
            ['RemoveIdentityFailed']
        );

        await checkErrorDetail(aliceRemovedEvents, 'IdentityNotExist');

        // remove a wrong identity (alice) for charlie
        const charlieRemoveTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.charlie,
            aliceIdentities,
            'removeIdentity'
        );
        const charileRemovedEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.charlie,
            charlieRemoveTxs,
            'identityManagement',
            ['RemoveIdentityFailed']
        );

        await checkErrorDetail(charileRemovedEvents, 'UserShieldingKeyNotFound');
    });

    step('set error user shielding key', async function () {
        const errorCiphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, hexToU8a(errorAesKey)).toString(
            'hex'
        );
        const errorTx = context.api.tx.identityManagement.setUserShieldingKey(
            context.mrEnclave,
            `0x${errorCiphertext}`
        );

        const respErrorEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            [{ tx: errorTx }] as any,
            'identityManagement',
            ['SetUserShieldingKeyFailed']
        );

        await checkErrorDetail(respErrorEvents, 'ImportError');
    });

    step('exceeding IDGraph limit not allowed', async function () {
        // TODO: this needs to be reworked
        //       we have to provide validation data when linking
    });
});
