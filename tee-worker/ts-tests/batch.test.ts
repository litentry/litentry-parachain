import { describeLitentry, generateVerificationMessage, encryptWithTeeShieldingKey, sendTxUntilInBlock, listenEvent } from './utils';
import { hexToU8a, u8aConcat, u8aToHex, u8aToU8a, stringToU8a } from '@polkadot/util';
import { createIdentity, setUserShieldingKey, removeIdentity, verifyIdentity } from './indirect_calls';
import { step } from 'mocha-steps';
import { assert, expect } from 'chai';
import {
    LitentryIdentity,
    LitentryValidationData,
    Web2Identity,
} from './type-definitions';


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
        const alice = await setUserShieldingKey(context, context.defaultSigner[0], aesKey, true);
        assert.equal(alice, u8aToHex(context.defaultSigner[0].addressRaw), 'check caller error');
    });

    step('batch test: create identities', async function () {
        // Create Identity: twitter 1
        const encode = context.substrate.createType('LitentryIdentity', twitterIdentity).toHex();
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
        const twi_tx = context.substrate.tx.identityManagement.createIdentity(
            context.shard,
            context.defaultSigner[0].address,
            `0x${ciphertext}`,
            null
        );

        // Create Identity: twitter 2
        const encode2 = context.substrate.createType('LitentryIdentity', twitterIdentity2).toHex();
        const ciphertext_2 = encryptWithTeeShieldingKey(context.teeShieldingKey, encode2).toString('hex');
        const twi_tx_2 = context.substrate.tx.identityManagement.createIdentity(
            context.shard,
            context.defaultSigner[0].address,
            `0x${ciphertext_2}`,
            null
        );

        // Construct the batch and send the transactions
        const txs = [twi_tx, twi_tx_2];
        await context.substrate.tx.utility
            .batchAll(txs)
            .signAndSend(context.defaultSigner[0], ({ status }) => {
                if (status.isInBlock) {
                    console.log(`included in ${status.asInBlock}`);
                }
            });

        const events = await listenEvent(context.substrate, 'identityManagement', ['IdentityCreated', 'IdentityCreated']);
        expect(events.length).to.be.equal(2);
    });

    step('batch test: verify identity', async function () {
        // Verify Identity: twitter 1
        const identity_encode = context.substrate.createType('LitentryIdentity', twitterIdentity).toHex();
        const validation_encode = context.substrate.createType('LitentryValidationData', twitterValidationData).toHex();
        const identity_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, identity_encode).toString('hex');
        const validation_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, validation_encode).toString('hex');
        const verify_tx = context.substrate.tx.identityManagement.verifyIdentity(
            context.shard,
            `0x${identity_ciphertext}`,
            `0x${validation_ciphertext}`
        );

        // Construct the batch and send the transactions
        const txs = [verify_tx];
        await context.substrate.tx.utility
            .batchAll(txs)
            .signAndSend(context.defaultSigner[0], ({ status }) => {
                if (status.isInBlock) {
                    console.log(`included in ${status.asInBlock}`);
                }
            });

        const events = await listenEvent(context.substrate, 'identityManagement', ['IdentityVerified']);
        expect(events.length).to.be.equal(1);
    });

    step('batch test: remove identities', async function () {
        // Remove Identity: twitter 1
        const rm_id_encode = context.substrate.createType('LitentryIdentity', twitterIdentity).toHex();
        const rm_id_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, rm_id_encode).toString('hex');
        const rm_id_tx = context.substrate.tx.identityManagement.removeIdentity(context.shard, `0x${rm_id_ciphertext}`);
        // Remove Identity: twitter 2
        const rm_id_encode2 = context.substrate.createType('LitentryIdentity', twitterIdentity2).toHex();
        const rm_id_ciphertext2 = encryptWithTeeShieldingKey(context.teeShieldingKey, rm_id_encode2).toString('hex');
        const rm_id_tx2 = context.substrate.tx.identityManagement.removeIdentity(context.shard, `0x${rm_id_ciphertext2}`);


        // Construct the batch and send the transactions
        const txs = [rm_id_tx, rm_id_tx2];
        await context.substrate.tx.utility
            .batchAll(txs)
            .signAndSend(context.defaultSigner[0], ({ status }) => {
                if (status.isInBlock) {
                    console.log(`included in ${status.asInBlock}`);
                }
            });

        const events = await listenEvent(context.substrate, 'identityManagement', ['IdentityRemoved', 'IdentityRemoved']);
        expect(events.length).to.be.equal(2);
    });
});
