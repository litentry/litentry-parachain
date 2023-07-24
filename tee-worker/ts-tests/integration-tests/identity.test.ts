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
    buildIdentityFromKeypair,
} from './common/utils';
import { aesKey } from './common/call';
import { hexToU8a, u8aConcat, u8aToHex, u8aToU8a, stringToU8a } from '@polkadot/util';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { multiAccountTxSender, sendTxsWithUtility } from './common/transactions';
import type { LitentryPrimitivesIdentity } from 'sidechain-api';
import type { LitentryValidationData, Web3Network } from 'parachain-api';
import type { TransactionSubmit } from './common/type-definitions';
import type { HexString } from '@polkadot/util/types';
import { ethers } from 'ethers';
import { sendRequest } from './common/call';

import * as base58 from 'micro-base58';

async function getNonce(base58mrEnclave: string, context: any) {
    const request = { jsonrpc: '2.0', method: 'author_getNextNonce', params: [base58mrEnclave, context.mrEnclave.slice(2)], id: 1 };
    const res = await sendRequest(context.tee, request, context.api);
    const u8aValue = res.value.toU8a();
    const len = u8aValue.length;
    let nonce = 0;
    if(len > 0) {
        for( let i = len - 1; i > 0; i--){
            nonce *= 16;
            nonce += u8aValue[i];
        }
    }
    console.log("res value",u8aValue);
    console.log("nonce is:", nonce);
    return nonce;
}

describeLitentry('Test Identity', 0, (context) => {
    const errorAesKey = '0xError';
    // random wrong msg
    const wrongMsg = '0x693d9131808e7a8574c7ea5eb7813bdf356223263e61fa8fe2ee8e434508bc75';
    let signatureSubstrate;
    let eveIdentities: LitentryPrimitivesIdentity[] = [];
    let aliceIdentities: LitentryPrimitivesIdentity[] = [];
    let eveValidations: LitentryValidationData[] = [];
    let bobValidations: LitentryValidationData[] = [];
    let web3networks: Web3Network[][] = [];

    step('check user sidechain storage before create', async function () {
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
        const respShieldingKey = await checkUserShieldingKeys(
            context,
            'IdentityManagement',
            'UserShieldingKeys',
            aliceSubject
        );
        assert.equal(respShieldingKey, '0x', 'shielding key should be empty before set');
    });

    step('Invalid user shielding key', async function () {
        const identity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
        // use empty `eveValidations`, the `UserShieldingKeyNotFound` error should be emitted before verification
        const txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [identity],
            'linkIdentity',
            eveValidations,
            web3networks
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
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
        const respShieldingKey = await checkUserShieldingKeys(
            context,
            'IdentityManagement',
            'UserShieldingKeys',
            aliceSubject
        );
        assert.equal(respShieldingKey, aesKey, 'respShieldingKey should be equal aesKey after set');
    });

    step('check idgraph from sidechain storage before linking', async function () {
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);

        // the main address should be already inside the IDGraph
        const mainIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'Substrate',
            context
        );
        const identityHex = mainIdentity.toHex();
        const respIdGraph = await checkIdGraph(context, 'IdentityManagement', 'IDGraphs', aliceSubject, identityHex);
        assert.isTrue(respIdGraph.linkBlock.toNumber() > 0, 'linkBlock should be greater than 0 for main address');
        assert.isTrue(respIdGraph.status.isActive, 'status should be active for main address');
        // TODO: check IDGraph.length == 1 in the sidechain storage
    });

    step('link identities', async function () {
        // Alice links:
        // - a `mock_user` twitter
        // - alice's evm identity
        // - eve's substrate identity (as she can't link her own substrate again)
        const base58mrEnclave = base58.encode(Buffer.from(context.mrEnclave.slice(2), 'hex'));
        const nonce = await getNonce(base58mrEnclave, context);
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
        const eveSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.eve.addressRaw),
            'Substrate',
            context
        );
        

        // Bob links:
        // - alice's substrate identity
        const aliceSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.charlie.addressRaw),
            'Substrate',
            context
        );
        eveIdentities = [twitterIdentity, evmIdentity, eveSubstrateIdentity];
        aliceIdentities = [aliceSubstrateIdentity];
        // TODO: being lazy - the nonce here is hardcoded
        //       it's better to retrieve the starting nonce from the sidechain and increment
        //       it for each such request, similar to the construction of substrate tx
        //       However, beware that we should query the nonce of the enclave-signer-account
        //       not alice or bob, as it's the indirect calls are signed by the enclave signer
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
        const twitterValidations = await buildValidations(
            context,
            [aliceSubject],
            [twitterIdentity],
            nonce,
            'twitter'
        );

        const evmValidations = await buildValidations(
            context,
            [aliceSubject],
            [evmIdentity],
            nonce + 1,
            'ethereum',
            undefined,
            [context.ethersWallet.alice]
        );

        const eveSubstrateValidations = await buildValidations(
            context,
            [aliceSubject],
            [eveSubstrateIdentity],
            nonce + 2 ,
            'substrate',
            context.substrateWallet.eve
        );

        eveValidations = [...twitterValidations, ...evmValidations, ...eveSubstrateValidations];

        const twitterNetworks = context.api.createType('Vec<Web3Network>', []) as unknown as Web3Network[];
        const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']) as unknown as Web3Network[];
        const eveSubstrateNetworks = context.api.createType('Vec<Web3Network>', [
            'Litentry',
            'Polkadot',
        ]) as unknown as Web3Network[];

        web3networks = [twitterNetworks, evmNetworks, eveSubstrateNetworks];

        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            eveIdentities,
            'linkIdentity',
            eveValidations,
            web3networks
        );
        console.log("aliceTxs:", aliceTxs);

        const aliceRespEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceTxs,
            'identityManagement',
            ['IdentityLinked']
        );
        console.log("aliceRespEvents:", aliceRespEvents);

        assertIdentityLinked(context, context.substrateWallet.alice, aliceRespEvents, eveIdentities);

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
        const bobSubject = await buildIdentityFromKeypair(context.substrateWallet.bob, context);
        const msg = generateVerificationMessage(
            context,
            bobSubject,
            aliceSubstrateIdentity,
            // 9 because each previous linking of Alice's identity would trigger an additional nonce bump
            // due to the callback trustedCall
            nonce + 6
        );
        console.log('post verification msg to substrate: ', msg);
        substrateExtensionValidationData.Web3Validation.Substrate.message = msg;
        // sign the wrapped version as in polkadot-extension
        signatureSubstrate = context.substrateWallet.charlie.sign(
            u8aConcat(stringToU8a('<Bytes>'), u8aToU8a(msg), stringToU8a('</Bytes>'))
        );
        substrateExtensionValidationData!.Web3Validation.Substrate.signature.Sr25519 = u8aToHex(signatureSubstrate);
        const bobSubstrateValidation = context.api.createType(
            'LitentryValidationData',
            substrateExtensionValidationData
        ) as unknown as LitentryValidationData;
        bobValidations = [bobSubstrateValidation];

        const bobSubstrateNetworks = context.api.createType('Vec<Web3Network>', [
            'Litentry',
            'Polkadot',
        ]) as unknown as Web3Network[];

        const bobTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.bob,
            aliceIdentities,
            'linkIdentity',
            bobValidations,
            [bobSubstrateNetworks]
        );

        const bobRespEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.bob,
            bobTxs,
            'identityManagement',
            ['IdentityLinked']
        );
        assertIdentityLinked(context, context.substrateWallet.bob, bobRespEvents, aliceIdentities);
    });

    step('check IDGraph after LinkIdentity', async function () {
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const identityHex = context.api.createType('LitentryIdentity', twitterIdentity).toHex();
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);

        const respIdGraph = await checkIdGraph(context, 'IdentityManagement', 'IDGraphs', aliceSubject, identityHex);
        assert.isTrue(respIdGraph.linkBlock.toNumber() > 0, 'linkBlock should be greater than 0');
        assert.isTrue(respIdGraph.status.isActive, 'status should be active');
    });

    step('link invalid identities', async function () {
        const twitterIdentity = eveIdentities[0];
        const ethereumValidation = eveValidations[1];

        // link twitter identity with ethereum validation data
        // the `InvalidIdentity` error should be emitted prior to `AlreadyLinked` error
        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [twitterIdentity],
            'linkIdentity',
            [ethereumValidation],
            []
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
        const evmIdentity = eveIdentities[1];

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
            [evmIdentity],
            'linkIdentity',
            [ethereumValidationData],
            []
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

    step('link already linked identity', async function () {
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);


        const aliceIdentities = [twitterIdentity];

        // TODO: being lazy - the nonce here is hardcoded
        //       it's better to retrieve the starting nonce from the sidechain and increment
        //       it for each such request, similar to the construction of substrate tx
        //       However, beware that we should query the nonce of the enclave-signer-account
        //       not alice or bob, as it's the indirect calls are signed by the enclave signer
        const base58mrEnclave = base58.encode(Buffer.from(context.mrEnclave.slice(2), 'hex'));
        const nonce = await getNonce(base58mrEnclave, context);
        console.log("nonce 15", nonce);
        const aliceTwitterValidations = await buildValidations(
            context,
            [aliceSubject],
            [twitterIdentity],
            nonce + 1,
            'twitter',
            context.substrateWallet.alice,
            []
        );

        const aliceValidations = [...aliceTwitterValidations]

        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            aliceIdentities,
            'linkIdentity',
            aliceValidations,
            []
        );

        const aliceRespEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceTxs,
            'identityManagement',
            ['LinkIdentityFailed']
        );

        await checkErrorDetail(aliceRespEvents, 'IdentityAlreadyLinked');
    });

    // TODO: testcase for linking prime address

    step('remove identities', async function () {
        // Alice remove all identities
        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            eveIdentities,
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
        const bobTxs = await buildIdentityTxs(context, context.substrateWallet.bob, aliceIdentities, 'removeIdentity');
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
        // TODO: we should verify the IDGraph is empty
    });

    step('remove prime identity is disallowed', async function () {
        // remove prime identity
        const substratePrimeIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
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
            eveIdentities,
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
            eveIdentities,
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
