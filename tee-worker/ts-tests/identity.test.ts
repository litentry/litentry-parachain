import {describeLitentry, generateVerificationMessage} from './utils';
import {hexToU8a, u8aConcat, u8aToHex, u8aToU8a, stringToU8a} from '@polkadot/util';
import {
    createIdentity,
    setUserShieldingKey,
    removeIdentity,
    verifyIdentity
} from './indirect_calls';
import {step} from 'mocha-steps';
import {assert} from 'chai';
import {
    EvmIdentity,
    IdentityGenericEvent,
    LitentryIdentity,
    LitentryValidationData, SubstrateIdentity,
    Web2Identity
} from './type-definitions';
import {ethers} from 'ethers';
import {HexString} from '@polkadot/util/types';
import {KeyringPair} from '@polkadot/keyring/types';


const twitterIdentity = <LitentryIdentity>{
    Web2: <Web2Identity>{
        address: 'mock_user',
        network: "Twitter"
    }
};

const ethereumIdentity = <LitentryIdentity>{
    Evm: <EvmIdentity>{
        address: '0xff93B45308FD417dF303D6515aB04D9e89a750Ca',
        network: 'Ethereum',
    },
};

const substrateIdentity = <LitentryIdentity>{
    Substrate: <SubstrateIdentity>{
        address: '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d', //alice
        network: 'Litentry'
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
    var signature_ethereum;
    var signature_substrate;

    step('set user shielding key', async function () {
        const who = await setUserShieldingKey(context, context.defaultSigner, aesKey, true);
        assert.equal(who, u8aToHex(context.defaultSigner.addressRaw), 'check caller error');
    });

    step('create identity', async function () {
        //create twitter identity
        const resp_twitter = await createIdentity(context, context.defaultSigner, aesKey, true, twitterIdentity);
        assertIdentityCreated(context.defaultSigner, resp_twitter);

        if (resp_twitter) {
            console.log('twitterIdentity challengeCode: ', resp_twitter.challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(resp_twitter.challengeCode),
                context.defaultSigner.addressRaw,
                twitterIdentity
            );
            console.log('post verification msg to twitter: ', msg);
            assert.isNotEmpty(resp_twitter.challengeCode, 'challengeCode empty');
        }
        //create ethereum identity
        const resp_ethereum = await createIdentity(context, context.defaultSigner, aesKey, true, ethereumIdentity);
        assertIdentityCreated(context.defaultSigner, resp_ethereum);

        if (resp_ethereum) {
            console.log('ethereumIdentity challengeCode: ', resp_ethereum.challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(resp_ethereum.challengeCode),
                context.defaultSigner.addressRaw,
                ethereumIdentity
            );
            console.log('post verification msg to ethereum: ', msg);
            ethereumValidationData!.Web3Validation!.Evm!.message = msg;
            const msgHash = ethers.utils.arrayify(msg);
            signature_ethereum = await context.ethersWallet.alice.signMessage(msgHash);
            ethereumValidationData!.Web3Validation!.Evm!.signature!.Ethereum = signature_ethereum;
            assert.isNotEmpty(resp_ethereum.challengeCode, 'challengeCode empty');
        }
        // create substrate identity
        const resp_substrate = await createIdentity(context, context.defaultSigner, aesKey, true, substrateIdentity);
        assertIdentityCreated(context.defaultSigner, resp_substrate);

        if (resp_substrate) {
            console.log('substrateIdentity challengeCode: ', resp_substrate.challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(resp_substrate.challengeCode),
                context.defaultSigner.addressRaw,
                substrateIdentity
            );

            console.log('post verification msg to substrate: ', msg);
            substrateValidationData!.Web3Validation!.Substrate!.message = msg;
            signature_substrate = context.defaultSigner.sign(msg);
            substrateValidationData!.Web3Validation!.Substrate!.signature!.Sr25519 = u8aToHex(signature_substrate);
            assert.isNotEmpty(resp_substrate.challengeCode, 'challengeCode empty');
        }
    });

    step('verify identity', async function () {
        //verify twitter identity
        const twitter_identity_verified = await verifyIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            twitterIdentity,
            twitterValidationData
        );
        assertIdentityVerified(context.defaultSigner, twitter_identity_verified);

        // verify ethereum identity
        const ethereum_identity_verified = await verifyIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            ethereumIdentity,
            ethereumValidationData
        );
        assertIdentityVerified(context.defaultSigner, ethereum_identity_verified);

        //verify substrate identity
        const substrate_identity_verified = await verifyIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            substrateIdentity,
            substrateValidationData
        );
        assertIdentityVerified(context.defaultSigner, substrate_identity_verified);
    });

    step('remove identity', async function () {
        //remove twitter identity
        const twitter_identity_removed = await removeIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            twitterIdentity
        );
        assertIdentityRemoved(context.defaultSigner, twitter_identity_removed);

        // remove ethereum identity
        const ethereum_identity_removed = await removeIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            ethereumIdentity
        );
        assertIdentityRemoved(context.defaultSigner, ethereum_identity_removed);

        // remove substrate identity
        const substrate_identity_removed = await removeIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            substrateIdentity
        );
        assertIdentityRemoved(context.defaultSigner, substrate_identity_removed);
    });

    // see https://github.com/litentry/litentry-parachain/issues/1137
    step('verify identity with wrapped signature', async function () {
        // create substrate identity again
        const resp_substrate = await createIdentity(context, context.defaultSigner, aesKey, true, substrateIdentity);
        if (resp_substrate) {
            console.log('substrateIdentity challengeCode: ', resp_substrate.challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(resp_substrate.challengeCode),
                context.defaultSigner.addressRaw,
                substrateIdentity
            );

            console.log('post verification msg to substrate: ', msg);
            substrateValidationData!.Web3Validation!.Substrate!.message = msg;
            // sign the wrapped version as in polkadot-extension
            signature_substrate = context.defaultSigner.sign(
                u8aConcat(stringToU8a('<Bytes>'), u8aToU8a(msg), stringToU8a('</Bytes>'))
            );
            substrateValidationData!.Web3Validation!.Substrate!.signature!.Sr25519 = u8aToHex(signature_substrate);
            assert.isNotEmpty(resp_substrate.challengeCode, 'challengeCode empty');
        }
        //verify substrate identity
        const substrate_identity_verified = await verifyIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            substrateIdentity,
            substrateValidationData
        );
        assertIdentityVerified(context.defaultSigner, substrate_identity_verified);
    });
});

function assertIdentityCreated(signer: KeyringPair, identityEvent: IdentityGenericEvent | undefined) {
    let idGraphExist = false;
    if (identityEvent) {
        for (let i = 0; i < identityEvent.idGraph.length; i++) {
            if (JSON.stringify(identityEvent.idGraph[i][0]) == JSON.stringify(identityEvent.identity)) {
                idGraphExist = true;
                assert.isFalse(identityEvent.idGraph[i][1].is_verified, 'identity should not be verified');
            }
        }
    }
    assert.isTrue(idGraphExist, 'id_graph should exist');
    assert.equal(identityEvent?.who, u8aToHex(signer.addressRaw), 'check caller error');
}

function assertIdentityVerified(signer: KeyringPair, identityEvent: IdentityGenericEvent | undefined) {
    let idGraphExist = false;

    if (identityEvent) {
        for (let i = 0; i < identityEvent.idGraph.length; i++) {
            if (JSON.stringify(identityEvent.idGraph[i][0]) == JSON.stringify(identityEvent.identity)) {
                idGraphExist = true;
                assert.isTrue(identityEvent.idGraph[i][1].is_verified, 'identity should be verified');
            }
        }
    }
    assert.isTrue(idGraphExist, 'id_graph should exist');
    assert.equal(identityEvent?.who, u8aToHex(signer.addressRaw), 'check caller error');
}

function assertIdentityRemoved(signer: KeyringPair, identityEvent: IdentityGenericEvent | undefined) {
    let idGraphExist = false;
    if (identityEvent) {
        for (let i = 0; i < identityEvent.idGraph.length; i++) {
            if (JSON.stringify(identityEvent.idGraph[i][0]) == JSON.stringify(identityEvent.identity)) {
                idGraphExist = true;
            }
        }
    }
    assert.isFalse(idGraphExist, 'id_graph should be empty');
    assert.equal(identityEvent?.who, u8aToHex(signer.addressRaw), 'check caller error');
}
