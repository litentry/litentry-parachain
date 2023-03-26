import {
    describeLitentry,
    generateVerificationMessage,
    encryptWithTeeShieldingKey,
    decryptWithAES,
    createIdentityEvent,
} from './common/utils';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import {
    setUserShieldingKey,
    assertIdentityCreated,
    assertIdentityVerified,
    assertIdentityRemoved,
} from './indirect_calls';
import { step } from 'mocha-steps';
import { assert, expect } from 'chai';
import { LitentryIdentity, LitentryValidationData, Web2Identity } from './common/type-definitions';
import { listenEvent } from './common/transactions';

const twitterIdentity = <LitentryIdentity>{
    Web2: <Web2Identity>{
        address: 'mock_user',
        network: 'Twitter',
    },
};
const twitterIdentity2 = <LitentryIdentity>{
    Web2: <Web2Identity>{
        address: 'mock_user2',
        network: 'Twitter',
    },
};
const twitterValidationData = <LitentryValidationData>{
    Web2Validation: {
        Twitter: {
            tweet_id: `0x${Buffer.from('100', 'utf8').toString('hex')}`,
        },
    },
};

describeLitentry('Test Batch Utility', (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';

    step('set user shielding key', async function () {
        const alice = await setUserShieldingKey(context, context.substrateWallet.alice, aesKey, true);
        assert.equal(alice, u8aToHex(context.substrateWallet.alice.addressRaw), 'check caller error');
    });

    step('batch test: create identities', async function () {
        // Create Identity: twitter 1
        const encode = context.api.createType('LitentryIdentity', twitterIdentity).toHex();
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
        const twi_tx = context.api.tx.identityManagement.createIdentity(
            context.mrEnclave,
            context.substrateWallet.alice.address,
            `0x${ciphertext}`,
            null
        );

        // Create Identity: twitter 2
        const encode2 = context.api.createType('LitentryIdentity', twitterIdentity2).toHex();
        const ciphertext_2 = encryptWithTeeShieldingKey(context.teeShieldingKey, encode2).toString('hex');
        const twi_tx_2 = context.api.tx.identityManagement.createIdentity(
            context.mrEnclave,
            context.substrateWallet.alice.address,
            `0x${ciphertext_2}`,
            null
        );

        // Construct the batch and send the transactions
        const txs = [twi_tx, twi_tx_2];
        await context.api.tx.utility.batchAll(txs).signAndSend(context.substrateWallet.alice, ({ status }: any) => {
            if (status.isInBlock) {
                console.log(`included in ${status.asInBlock}`);
            }
        });

        const events = await listenEvent(context.api, 'identityManagement', ['IdentityCreated'], txs.length);
        expect(events.length).to.be.equal(2);
        for (let i = 0; i < 2; i++) {
            const data = events[i].data as any;
            const response = createIdentityEvent(
                context.api,
                data.account.toHex(),
                decryptWithAES(aesKey, data.identity, 'hex'),
                undefined,
                decryptWithAES(aesKey, data.code, 'hex')
            );
            assertIdentityCreated(context.substrateWallet.alice, response);
            if (response) {
                console.log('twitterIdentity challengeCode: ', response.challengeCode);
                const msg = generateVerificationMessage(
                    context,
                    hexToU8a(response.challengeCode),
                    context.substrateWallet.alice.addressRaw,
                    twitterIdentity
                );
                console.log('post verification msg to twitter: ', msg);
                assert.isNotEmpty(response.challengeCode, 'challengeCode empty');
            }
        }
    });

    step('batch test: verify identity', async function () {
        // Verify Identity: twitter 1
        const identity_encode = context.api.createType('LitentryIdentity', twitterIdentity).toHex();
        const validation_encode = context.api.createType('LitentryValidationData', twitterValidationData).toHex();
        const identity_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, identity_encode).toString(
            'hex'
        );
        const validation_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, validation_encode).toString(
            'hex'
        );
        const verify_tx = context.api.tx.identityManagement.verifyIdentity(
            context.mrEnclave,
            `0x${identity_ciphertext}`,
            `0x${validation_ciphertext}`
        );

        // Construct the batch and send the transactions
        const txs = [verify_tx];
        await context.api.tx.utility.batchAll(txs).signAndSend(context.substrateWallet.alice, ({ status }: any) => {
            if (status.isInBlock) {
                console.log(`included in ${status.asInBlock}`);
            }
        });

        const events = await listenEvent(context.api, 'identityManagement', ['IdentityVerified'], txs.length);
        expect(events.length).to.be.equal(1);
        const data = events[0].data as any;
        const response = createIdentityEvent(
            context.api,
            data.account.toHex(),
            decryptWithAES(aesKey, data.identity, 'hex'),
            decryptWithAES(aesKey, data.idGraph, 'hex')
        );
        assertIdentityVerified(context.substrateWallet.alice, response);
    });

    step('batch test: remove identities', async function () {
        // Remove Identity: twitter 1
        const rm_id_encode = context.api.createType('LitentryIdentity', twitterIdentity).toHex();
        const rm_id_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, rm_id_encode).toString('hex');
        const rm_id_tx = context.api.tx.identityManagement.removeIdentity(context.mrEnclave, `0x${rm_id_ciphertext}`);
        // Remove Identity: twitter 2
        const rm_id_encode2 = context.api.createType('LitentryIdentity', twitterIdentity2).toHex();
        const rm_id_ciphertext2 = encryptWithTeeShieldingKey(context.teeShieldingKey, rm_id_encode2).toString('hex');
        const rm_id_tx2 = context.api.tx.identityManagement.removeIdentity(context.mrEnclave, `0x${rm_id_ciphertext2}`);

        // Construct the batch and send the transactions
        const txs = [rm_id_tx, rm_id_tx2];
        await context.api.tx.utility.batchAll(txs).signAndSend(context.substrateWallet.alice, ({ status }: any) => {
            if (status.isInBlock) {
                console.log(`included in ${status.asInBlock}`);
            }
        });

        const events = await listenEvent(context.api, 'identityManagement', ['IdentityRemoved'], txs.length);
        expect(events.length).to.be.equal(2);
        for (let i = 0; i < 2; i++) {
            const data = events[i].data as any;
            const response = createIdentityEvent(
                context.api,
                data.account.toHex(),
                decryptWithAES(aesKey, data.identity, 'hex')
            );
            assertIdentityRemoved(context.substrateWallet.alice, response);
        }
    });
});
