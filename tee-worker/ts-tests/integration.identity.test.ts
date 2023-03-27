import { step } from 'mocha-steps';
import {
    buildIdentities,
    checkVc,
    describeLitentry,
    encryptWithTeeShieldingKey,
    generateVerificationMessage,
} from './common/utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { ethers } from 'ethers';
const { Keyring } = require('@polkadot/keyring');
import { hexToU8a, u8aConcat, u8aToHex, u8aToU8a, stringToU8a } from '@polkadot/util';

import {
    EvmIdentity,
    LitentryIdentity,
    LitentryValidationData,
    SubstrateIdentity,
    TransactionSubmit,
    Web2Identity,
} from './common/type-definitions';
import { batchCall } from './common/utils';
import { handleVcEvents, handleIdentitiesEvents } from './common/utils';
import { multiAccountTxSender } from './indirect_calls';
import { blake2AsHex } from '@polkadot/util-crypto';
import { assert } from 'chai';
import { HexString } from '@polkadot/util/types';
import { identity } from './common/helpers';

const twitterIdentity = <LitentryIdentity>{
    Web2: <Web2Identity>{
        address: 'mock_user',
        network: 'Twitter',
    },
};
var ethereumIdentity = <LitentryIdentity>{
    Evm: <EvmIdentity>{
        address: '' as HexString,
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

describeLitentry('multiple accounts test', 10, async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    var substraetSigners: KeyringPair[] = [];
    var ethereumSigners: ethers.Wallet[] = [];
    var signature_ethereum;
    var web3Validations: LitentryValidationData[] = [];
    var identitiesData: LitentryIdentity[] = [];
    step('init', async () => {
        substraetSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.substrateWallet;
        });
        ethereumSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.ethereumWallet;
        });
    });
    step('send test token to each account', async () => {
        const txs: any = [];
        for (let i = 0; i < substraetSigners.length; i++) {
            console.log('substraetSigners[i].address', substraetSigners[i].address);

            // 0.1 test token
            const tx = context.api.tx.balances.transfer(substraetSigners[i].address, '100000000000');
            txs.push(tx);
        }
        const events = await batchCall(context, context.substrateWallet.alice, txs, 'balances', ['Transfer']);
        assert.equal(events.length, substraetSigners.length, 'transfer token check fail');
    });

    step('set usershieldingkey test', async () => {
        const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, aesKey).toString('hex');
        let txs: TransactionSubmit[] = [];
        for (let i = 0; i < substraetSigners.length; i++) {
            const tx = context.api.tx.identityManagement.setUserShieldingKey(context.mrEnclave, `0x${ciphertext}`);
            const nonce = (await context.api.rpc.system.accountNextIndex(substraetSigners[i].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'UserShieldingKeySet',
        ]);
        assert.equal(resp_events.length, substraetSigners.length, 'set usershieldingkey check fail');
    });
    //test identity with multiple accounts
    step('test createIdentities(ethereumIdentity) with multiple accounts', async () => {
        let txs: TransactionSubmit[] = [];
        let identities: any[] = [];
        for (let i = 0; i < ethereumSigners.length; i++) {
            ethereumIdentity.Evm!.address = ethereumSigners[i].address as HexString;
            identities.push(ethereumIdentity);
        }
        identitiesData = [...identities];
        for (let k = 0; k < identities.length; k++) {
            const identity = identities[k];
            const encode = context.api.createType('LitentryIdentity', identity).toHex();
            const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
            const tx = context.api.tx.identityManagement.createIdentity(
                context.mrEnclave,
                substraetSigners[k].address,
                `0x${ciphertext}`,
                null
            );
            const nonce = (await context.api.rpc.system.accountNextIndex(substraetSigners[k].address)).toNumber();
            txs.push({ tx, nonce });
        }
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'IdentityCreated',
        ]);
        assert.equal(resp_events.length, identities.length, 'create identities check fail');

        const event_data = await handleIdentitiesEvents(context, aesKey, resp_events, 'IdentityCreated');
        const validations = await buildIdentities(
            context,
            event_data,
            identities,
            substraetSigners,
            ethereumSigners,
            'ethereumIdentity'
        );
        web3Validations = [...validations];
    });
    step('test verifyIdentities(ethereumIdentity) with multiple accounts', async () => {
        let txs: TransactionSubmit[] = [];
        for (let index = 0; index < identitiesData.length; index++) {
            let identity = identitiesData[index];

            let data = web3Validations[index];
            const identity_encode = context.api.createType('LitentryIdentity', identity).toHex();
            const validation_encode = context.api.createType('LitentryValidationData', data).toHex();
            const identity_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, identity_encode).toString(
                'hex'
            );
            const validation_ciphertext = encryptWithTeeShieldingKey(
                context.teeShieldingKey,
                validation_encode
            ).toString('hex');
            const tx = context.api.tx.identityManagement.verifyIdentity(
                context.mrEnclave,
                `0x${identity_ciphertext}`,
                `0x${validation_ciphertext}`
            );
            const nonce = (await context.api.rpc.system.accountNextIndex(substraetSigners[index].address)).toNumber();

            txs.push({ tx, nonce });
        }

        //The testing here is very strange, it only succeeds when there is one account, if multiple accounts are being verified at the same time, only the last signature verification will succeed.It's necessary to check this.
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'IdentityVerified',
        ]);
    });

    //one account test with multiple identities
    step('test multiple createIdentities(ethereumIdentity) with one account', async () => {
        let txs: any = [];
        let identities: any[] = [];
        let create_times = 10;
        ethereumIdentity.Evm!.address = ethereumSigners[0].address as HexString;
        for (let i = 0; i < create_times; i++) {
            identities.push(ethereumIdentity);
        }
        identitiesData = [...identities];

        for (let k = 0; k < identities.length; k++) {
            const identity = identities[k];
            const encode = context.api.createType('LitentryIdentity', identity).toHex();
            const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
            const tx = context.api.tx.identityManagement.createIdentity(
                context.mrEnclave,
                substraetSigners[0].address,
                `0x${ciphertext}`,
                null
            );
            txs.push(tx);
        }
        const resp_events = await batchCall(context, substraetSigners[0], txs, 'identityManagement', [
            'IdentityCreated',
        ]);
        const event_data = await handleIdentitiesEvents(context, aesKey, resp_events, 'IdentityCreated');
        const validations = await buildIdentities(
            context,
            event_data,
            identities,
            [substraetSigners[0]],
            [ethereumSigners[0]],
            'ethereumIdentity'
        );
        web3Validations = [...validations];
    });

    step('test multiple verifyIdentities(ethereumIdentity) with one account', async () => {
        let txs: any = [];

        for (let index = 0; index < identitiesData.length; index++) {
            let identity = identitiesData[index];

            let data = web3Validations[index];
            const identity_encode = context.api.createType('LitentryIdentity', identity).toHex();
            const validation_encode = context.api.createType('LitentryValidationData', data).toHex();
            const identity_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, identity_encode).toString(
                'hex'
            );
            const validation_ciphertext = encryptWithTeeShieldingKey(
                context.teeShieldingKey,
                validation_encode
            ).toString('hex');
            const tx = context.api.tx.identityManagement.verifyIdentity(
                context.mrEnclave,
                `0x${identity_ciphertext}`,
                `0x${validation_ciphertext}`
            );
            txs.push(tx);
        }

        const resp_events = await batchCall(context, substraetSigners[0], txs, 'identityManagement', [
            'IdentityVerified',
        ]);
    });
});
