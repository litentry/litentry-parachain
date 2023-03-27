import {
    describeLitentry,
    encryptWithTeeShieldingKey,
    generateVerificationMessage,
    checkErrorDetail,
    checkUserShieldingKeys,
    checkUserChallengeCode,
    checkIDGraph,
} from './common/utils';
import { hexToU8a, u8aConcat, u8aToHex, u8aToU8a, stringToU8a } from '@polkadot/util';
import {
    setUserShieldingKey,
    createIdentities,
    verifyIdentities,
    removeIdentities,
    assertIdentityCreated,
    assertIdentityVerified,
    assertIdentityRemoved,
} from './indirect_calls';
import { step } from 'mocha-steps';
import { assert, expect } from 'chai';
import {
    EvmIdentity,
    IdentityGenericEvent,
    LitentryIdentity,
    LitentryValidationData,
    SubstrateIdentity,
    Web2Identity,
} from './common/type-definitions';
import { ethers } from 'ethers';
import { HexString } from '@polkadot/util/types';
import {
    createErrorIdentities,
    setErrorUserShieldingKey,
    removeErrorIdentities,
    verifyErrorIdentities,
} from './indirect_error_calls';
import { listenEvent, sendTxUntilInBlock } from './common/transactions';

const twitterIdentity = <LitentryIdentity>{
    Web2: <Web2Identity>{
        address: 'mock_user',
        network: 'Twitter',
    },
};
const ethereumIdentity = <LitentryIdentity>{
    Evm: <EvmIdentity>{
        address: '0xff93B45308FD417dF303D6515aB04D9e89a750Ca',
        network: 'Ethereum',
    },
};
const ethereumErrorIdentity = <LitentryIdentity>{
    Evm: <EvmIdentity>{
        address: '0xff93B45308FD417dF303D6515aB04D9e89a750Cb',
        network: 'Ethereum',
    },
};
const substrateIdentity = <LitentryIdentity>{
    Substrate: <SubstrateIdentity>{
        address: '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d', //Alice
        network: 'Litentry',
    },
};
const substrateExtensionIdentity = <LitentryIdentity>{
    Substrate: <SubstrateIdentity>{
        address: '0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48', //Bob
        network: 'Litentry',
    },
};
const twitterValidationData = <LitentryValidationData>{
    Web2Validation: {
        Twitter: {
            tweet_id: `0x${Buffer.from('100', 'utf8').toString('hex')}`,
        },
    },
};
const ethereumValidationData = <LitentryValidationData>{
    Web3Validation: {
        Evm: {
            message: `0x${Buffer.from('mock_message', 'utf8').toString('hex')}`,
            signature: {
                Ethereum: '' as HexString,
            },
        },
    },
};
const substrateValidationData = <LitentryValidationData>{
    Web3Validation: {
        Substrate: {
            message: `0x${Buffer.from('mock_message', 'utf8').toString('hex')}`,
            signature: {
                Sr25519: '' as HexString,
            },
        },
    },
};
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
const discordIdentity = <LitentryIdentity>{
    handle: {
        PlainString: `0x${Buffer.from('859641379851337798', 'utf8').toString('hex')}`,
    },
    web_type: {
        Web2Identity: 'Discord',
    },
};
const discordValidationData = <LitentryValidationData>{
    Web2Validation: {
        Discord: {
            channel_id: `0x${Buffer.from('919848392035794945', 'utf8').toString('hex')}`,
            guild_id: `0x${Buffer.from('919848390156767232', 'utf8').toString('hex')}`,
            message_id: `0x${Buffer.from('859641379851337798', 'utf8').toString('hex')}`,
        },
    },
};

describeLitentry('Test Identity', (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';

    const errorAseKey = '0xError';
    const errorCiphertext = '0xError';
    var signature_ethereum;
    var signature_substrate;
    step('check user sidechain storage before create', async function () {
        const encode = context.api.createType('LitentryIdentity', twitterIdentity).toHex();

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
            encode
        );

        assert.equal(resp_challengecode, '0x', 'challengecode should be empty before create');
    });
    step('Invalid user shielding key', async function () {
        const encode = context.api.createType('LitentryIdentity', substrateIdentity).toHex();
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
        const tx = context.api.tx.identityManagement.createIdentity(
            context.mrEnclave,
            context.substrateWallet.alice.address,
            `0x${ciphertext}`,
            null
        );
        await sendTxUntilInBlock(context.api, tx, context.substrateWallet.alice);

        const events = await listenEvent(context.api, 'identityManagement', ['CreateIdentityFailed']);
        expect(events.length).to.be.equal(1);

        await checkErrorDetail(events, 'InvalidUserShieldingKey', true);
    });

    step('set user shielding key', async function () {
        const alice = await setUserShieldingKey(context, context.substrateWallet.alice, aesKey, true);
        assert.equal(alice, u8aToHex(context.substrateWallet.alice.addressRaw), 'check caller error');
        const bob = await setUserShieldingKey(context, context.substrateWallet.bob, aesKey, true);
        assert.equal(bob, u8aToHex(context.substrateWallet.bob.addressRaw), 'check caller error');
    });

    step('check user shielding key from sidechain storage after setUserShieldingKey', async function () {
        const resp_shieldingKey = await checkUserShieldingKeys(
            context,
            'IdentityManagement',
            'UserShieldingKeys',
            u8aToHex(context.substrateWallet.alice.addressRaw)
        );
        console.log('resp_shieldingKey', resp_shieldingKey);
        assert.equal(resp_shieldingKey, aesKey, 'resp_shieldingKey should be equal aesKey after set');
    });

    step('check idgraph from sidechain storage before create', async function () {
        const encode = context.api.createType('LitentryIdentity', twitterIdentity).toHex();

        const resp_id_graph = await checkIDGraph(
            context,
            'IdentityManagement',
            'IDGraphs',
            u8aToHex(context.substrateWallet.alice.addressRaw),
            encode
        );
        assert.equal(
            resp_id_graph.verification_request_block,
            null,
            'verification_request_block should  be null before create'
        );
        assert.equal(resp_id_graph.linking_request_block, null, 'linking_request_block should  be null before create');

        assert.equal(resp_id_graph.is_verified, false, 'IDGraph is_verified should be equal false before create');
    });
    step('create identities', async function () {
        //Alice create all identities
        const [resp_twitter, resp_ethereum, resp_substrate] = (await createIdentities(
            context,
            context.substrateWallet.alice,
            aesKey,
            true,
            [twitterIdentity, ethereumIdentity, substrateIdentity]
        )) as IdentityGenericEvent[];

        //Bob create extension substrate identities
        const [resp_extension_substrate] = (await createIdentities(context, context.substrateWallet.bob, aesKey, true, [
            substrateExtensionIdentity,
        ])) as IdentityGenericEvent[];

        //Alice check twitter identity
        assertIdentityCreated(context.substrateWallet.alice, resp_twitter);

        if (resp_twitter) {
            console.log('twitterIdentity challengeCode: ', resp_twitter.challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(resp_twitter.challengeCode),
                context.substrateWallet.alice.addressRaw,
                twitterIdentity
            );
            console.log('post verification msg to twitter: ', msg);
            assert.isNotEmpty(resp_twitter.challengeCode, 'challengeCode empty');
        }
        //Alice check ethereum identity
        assertIdentityCreated(context.substrateWallet.alice, resp_ethereum);
        if (resp_ethereum) {
            console.log('ethereumIdentity challengeCode: ', resp_ethereum.challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(resp_ethereum.challengeCode),
                context.substrateWallet.alice.addressRaw,
                ethereumIdentity
            );
            console.log('post verification msg to ethereum: ', msg);
            ethereumValidationData!.Web3Validation!.Evm!.message = msg;
            const msgHash = ethers.utils.arrayify(msg);
            signature_ethereum = await context.ethersWallet.alice.signMessage(msgHash);
            ethereumValidationData!.Web3Validation!.Evm!.signature!.Ethereum = signature_ethereum;
            assert.isNotEmpty(resp_ethereum.challengeCode, 'challengeCode empty');
        }
        //Alice check substrate identity
        assertIdentityCreated(context.substrateWallet.alice, resp_substrate);
        if (resp_substrate) {
            console.log('substrateIdentity challengeCode: ', resp_substrate.challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(resp_substrate.challengeCode),
                context.substrateWallet.alice.addressRaw,
                substrateIdentity
            );
            console.log('post verification msg to substrate: ', msg);
            substrateValidationData!.Web3Validation!.Substrate!.message = msg;
            signature_substrate = context.substrateWallet.alice.sign(msg);
            substrateValidationData!.Web3Validation!.Substrate!.signature!.Sr25519 = u8aToHex(signature_substrate);
            assert.isNotEmpty(resp_substrate.challengeCode, 'challengeCode empty');
        }

        //Bob check extension substrate identity
        //https://github.com/litentry/litentry-parachain/issues/1137
        assertIdentityCreated(context.substrateWallet.bob, resp_extension_substrate);
        if (resp_extension_substrate) {
            console.log('substrateExtensionIdentity challengeCode: ', resp_extension_substrate.challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(resp_extension_substrate.challengeCode),
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
            assert.isNotEmpty(resp_extension_substrate.challengeCode, 'challengeCode empty');
        }
    });

    step('check IDGraph before verifyIdentity and after createIdentity', async function () {
        const encode = context.api.createType('LitentryIdentity', twitterIdentity).toHex();

        const resp_id_graph = await checkIDGraph(
            context,
            'IdentityManagement',
            'IDGraphs',
            u8aToHex(context.substrateWallet.alice.addressRaw),
            encode
        );
        assert.notEqual(
            resp_id_graph.linking_request_block,
            null,
            'linking_request_block should not be null after createIdentity'
        );
        assert.equal(resp_id_graph.is_verified, false, 'is_verified should be false before verifyIdentity');
    });
    step('verify identities', async function () {
        //Alice verify all identities
        const [twitter_identity_verified, ethereum_identity_verified, substrate_identity_verified] =
            (await verifyIdentities(
                context,
                context.substrateWallet.alice,
                aesKey,
                true,
                [twitterIdentity, ethereumIdentity, substrateIdentity],
                [twitterValidationData, ethereumValidationData, substrateValidationData]
            )) as IdentityGenericEvent[];
        //Bob verify extension substrate identities
        const [substrate_extension_identity_verified] = (await verifyIdentities(
            context,
            context.substrateWallet.bob,
            aesKey,
            true,
            [substrateExtensionIdentity],
            [substrateExtensionValidationData]
        )) as IdentityGenericEvent[];

        //Alice
        assertIdentityVerified(context.substrateWallet.alice, twitter_identity_verified);
        assertIdentityVerified(context.substrateWallet.alice, ethereum_identity_verified);
        assertIdentityVerified(context.substrateWallet.alice, substrate_identity_verified);
        //Bob
        assertIdentityVerified(context.substrateWallet.bob, substrate_extension_identity_verified);
    });
    step('check IDGraph after createIdentity', async function () {
        const encode = context.api.createType('LitentryIdentity', twitterIdentity).toHex();

        const resp_id_graph = await checkIDGraph(
            context,
            'IdentityManagement',
            'IDGraphs',
            u8aToHex(context.substrateWallet.alice.addressRaw),
            encode
        );
        assert.notEqual(
            resp_id_graph.verification_request_block,
            null,
            'verification_request_block should not be null after verifyIdentity'
        );
        assert.equal(resp_id_graph.is_verified, true, 'is_verified should be true after verifyIdentity');
    });
    step('verify error identities', async function () {
        // verify same identities to one account
        const resp_same_verify = (await verifyErrorIdentities(
            context,
            context.substrateWallet.alice,
            true,
            [twitterIdentity, ethereumIdentity, substrateIdentity],
            [twitterValidationData, ethereumValidationData, substrateValidationData]
        )) as string[];
        await checkErrorDetail(resp_same_verify, 'ChallengeCodeNotFound', false);

        //verify an identity to an account but it isn't created before
        const resp_not_exist_verify = (await verifyErrorIdentities(
            context,
            context.substrateWallet.charlie,
            true,
            [twitterIdentity, ethereumIdentity, substrateIdentity],
            [twitterValidationData, ethereumValidationData, substrateValidationData]
        )) as string[];
        await checkErrorDetail(resp_not_exist_verify, 'ChallengeCodeNotFound', false);
    });

    step('remove identities', async function () {
        // Alice remove all identities
        const [twitter_identity_removed, ethereum_identity_removed, substrate_identity_removed] =
            (await removeIdentities(context, context.substrateWallet.alice, aesKey, true, [
                twitterIdentity,
                ethereumIdentity,
                substrateIdentity,
            ])) as IdentityGenericEvent[];

        // Bob remove substrate identities
        const [substrate_extension_identity_removed] = (await removeIdentities(
            context,
            context.substrateWallet.bob,
            aesKey,
            true,
            [substrateExtensionIdentity]
        )) as IdentityGenericEvent[];

        //Alice
        assertIdentityRemoved(context.substrateWallet.alice, twitter_identity_removed);
        assertIdentityRemoved(context.substrateWallet.alice, ethereum_identity_removed);
        assertIdentityRemoved(context.substrateWallet.alice, substrate_identity_removed);

        // Bob
        assertIdentityRemoved(context.substrateWallet.bob, substrate_extension_identity_removed);
    });
    step('check IDGraph after removeIdentity', async function () {
        const encode = context.api.createType('LitentryIdentity', twitterIdentity).toHex();

        const resp_id_graph = await checkIDGraph(
            context,
            'IdentityManagement',
            'IDGraphs',
            u8aToHex(context.substrateWallet.alice.addressRaw),
            encode
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
        const [resp_substrate] = (await createIdentities(context, context.substrateWallet.alice, aesKey, true, [
            substrateIdentity,
        ])) as IdentityGenericEvent[];
        assertIdentityCreated(context.substrateWallet.alice, resp_substrate);

        if (resp_substrate) {
            console.log('substrateIdentity challengeCode: ', resp_substrate.challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(resp_substrate.challengeCode),
                context.substrateWallet.alice.addressRaw,
                substrateIdentity
            );

            console.log('post verification msg to substrate: ', msg);
            substrateValidationData!.Web3Validation!.Substrate!.message = msg;
            signature_substrate = context.substrateWallet.alice.sign(msg);
            substrateValidationData!.Web3Validation!.Substrate!.signature!.Sr25519 = u8aToHex(signature_substrate);
            assert.isNotEmpty(resp_substrate.challengeCode, 'challengeCode empty');
        }

        // remove substrate identity
        const [substrate_identity_removed] = (await removeIdentities(
            context,
            context.substrateWallet.alice,
            aesKey,
            true,
            [substrateIdentity]
        )) as IdentityGenericEvent[];
        assertIdentityRemoved(context.substrateWallet.alice, substrate_identity_removed);

        // remove prime identity
        const substratePrimeIdentity = <LitentryIdentity>{
            Substrate: <SubstrateIdentity>{
                address: `0x${Buffer.from(context.substrateWallet.alice.publicKey).toString('hex')}`,
                // When testing with integritee-node, change network to: TestNet
                network: 'LitentryRococo',
            },
        };

        const encode = context.api.createType('LitentryIdentity', substratePrimeIdentity).toHex();
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
        const tx = context.api.tx.identityManagement.removeIdentity(context.mrEnclave, `0x${ciphertext}`);
        await sendTxUntilInBlock(context.api, tx, context.substrateWallet.alice);

        const events = await listenEvent(context.api, 'identityManagement', ['RemoveIdentityFailed']);
        expect(events.length).to.be.equal(1);

        await checkErrorDetail(events, 'RemovePrimeIdentityDisallowed', true);
    });

    step('remove error identities', async function () {
        const identities = [twitterIdentity, ethereumIdentity, substrateIdentity];

        //remove a nonexistent identity
        //context.substrateWallet.alice has aleady removed all identities in step('remove identities')
        const resp_not_exist_identities = (await removeErrorIdentities(
            context,
            context.substrateWallet.alice,
            true,
            identities
        )) as string[];

        await checkErrorDetail(resp_not_exist_identities, 'IdentityNotExist', false);

        //charile doesn't have a challenge code
        const charlie = await setUserShieldingKey(context, context.substrateWallet.charlie, aesKey, true);
        assert.equal(charlie, u8aToHex(context.substrateWallet.charlie.addressRaw), 'check caller error');
        const resp_not_created_identities = (await removeErrorIdentities(
            context,
            context.substrateWallet.charlie,
            true,
            identities
        )) as string[];

        await checkErrorDetail(resp_not_created_identities, 'IdentityNotExist', false);
    });

    step('set error user shielding key', async function () {
        const resp_error_shielding_key = await setErrorUserShieldingKey(
            context,
            context.substrateWallet.alice,
            errorAseKey,
            true
        );
        await checkErrorDetail([resp_error_shielding_key] as string[], 'ImportError', false);
    });

    step('create error identities', async function () {
        //The simulation generates the wrong Ciphertext
        const resp_error_identities = (await createErrorIdentities(context, context.substrateWallet.alice, true, [
            errorCiphertext,
        ])) as string[];
        await checkErrorDetail(resp_error_identities, 'ImportError', false);
    });
});
