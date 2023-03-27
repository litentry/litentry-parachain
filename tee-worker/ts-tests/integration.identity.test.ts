import { step } from 'mocha-steps';
import {
    buildValidations,
    buildIdentities,
    checkVc,
    describeLitentry,
    encryptWithTeeShieldingKey,
    generateVerificationMessage,
    buildIdentityTxs,
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
import { handleVcEvents, handleIdentityEvents } from './common/utils';
import { multiAccountTxSender } from './indirect_calls';
import { blake2AsHex } from '@polkadot/util-crypto';
import { assert } from 'chai';
import { HexString } from '@polkadot/util/types';
import { identity, IdentityNetwork } from './common/helpers';
import { listenEvent } from './common/transactions';


describeLitentry('multiple accounts test', 2, async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    var substraetSigners: KeyringPair[] = [];
    var ethereumSigners: ethers.Wallet[] = [];
    var web3Validations: LitentryValidationData[] = [];
    var identityDatas: LitentryIdentity[] = [];
    let identityNetwork = IdentityNetwork.ethereum
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
            //0.1 token
            const tx = context.api.tx.balances.transfer(substraetSigners[i].address, '100000000000');

            txs.push(tx);
        }
        await context.api.tx.utility.batch(txs).signAndSend(context.substrateWallet.alice)
        await listenEvent(context.api, 'balances', ['Transfer'], txs.length);
    });

    //test with multiple accounts
    step('test set usershieldingkey with multiple accounts', async () => {
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
        assert.equal(resp_events.length, substraetSigners.length, 'set usershieldingkey with multiple accounts check fail');
    });

    //test identity with multiple accounts
    step('test createIdentity with multiple accounts', async () => {
        //substrate ethereum twitter
        let identities = await buildIdentities(identityNetwork, context);

        identityDatas = [...identities];
        let txs = await buildIdentityTxs(context, identities, 'createIdentity');
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'IdentityCreated',
        ]);
        assert.equal(resp_events.length, identities.length, 'create identities with multiple accounts check fail');
        const event_data = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityCreated');
        const validations = await buildValidations(
            context,
            event_data,
            identities,
            identityNetwork
        );
        console.log("validations", validations);

        web3Validations = [...validations];
    });

    step('test verifyIdentity with multiple accounts', async () => {
        let txs = await buildIdentityTxs(context, identityDatas, 'verifyIdentity', web3Validations);
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'IdentityVerified',
        ]);
        assert.equal(resp_events.length, txs.length, 'verify identities with multiple accounts check fail');
    });

    step('test removeIdentity with multiple accounts', async () => {
        let txs = await buildIdentityTxs(context, identityDatas, 'removeIdentity', web3Validations);
        const resp_events = await multiAccountTxSender(context, txs, substraetSigners, 'identityManagement', [
            'IdentityRemoved',
        ]);
        assert.equal(resp_events.length, txs.length, 'remove identities with multiple accounts check fail');
    })

    //one account test with multiple identities
    // step('test multiple createIdentities(ethereumIdentity) with one account', async () => {
    //     let txs: any = [];
    //     let identities: any[] = [];
    //     let create_times = 10;
    //     ethereumIdentity.Evm!.address = ethereumSigners[0].address as HexString;
    //     for (let i = 0; i < create_times; i++) {
    //         identities.push(ethereumIdentity);
    //     }
    //     identityDatas = [...identities];

    //     for (let k = 0; k < identities.length; k++) {
    //         const identity = identities[k];
    //         const encode = context.api.createType('LitentryIdentity', identity).toHex();
    //         const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
    //         const tx = context.api.tx.identityManagement.createIdentity(
    //             context.mrEnclave,
    //             substraetSigners[0].address,
    //             `0x${ciphertext}`,
    //             null
    //         );
    //         txs.push(tx);
    //     }
    //     const resp_events = await batchCall(context, substraetSigners[0], txs, 'identityManagement', [
    //         'IdentityCreated',
    //     ]);
    //     const event_data = await handleIdentityEvents(context, aesKey, resp_events, 'IdentityCreated');
    //     const validations = await buildIdentities(
    //         context,
    //         event_data,
    //         identities,
    //         [substraetSigners[0]],
    //         [ethereumSigners[0]],
    //         'ethereumIdentity'
    //     );
    //     web3Validations = [...validations];
    // });

    // step('test multiple verifyIdentities(ethereumIdentity) with one account', async () => {
    //     let txs: any = [];

    //     for (let index = 0; index < identityDatas.length; index++) {
    //         let identity = identityDatas[index];

    //         let data = web3Validations[index];
    //         const identity_encode = context.api.createType('LitentryIdentity', identity).toHex();
    //         const validation_encode = context.api.createType('LitentryValidationData', data).toHex();
    //         const identity_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, identity_encode).toString(
    //             'hex'
    //         );
    //         const validation_ciphertext = encryptWithTeeShieldingKey(
    //             context.teeShieldingKey,
    //             validation_encode
    //         ).toString('hex');
    //         const tx = context.api.tx.identityManagement.verifyIdentity(
    //             context.mrEnclave,
    //             `0x${identity_ciphertext}`,
    //             `0x${validation_ciphertext}`
    //         );
    //         txs.push(tx);
    //     }

    //     const resp_events = await batchCall(context, substraetSigners[0], txs, 'identityManagement', [
    //         'IdentityVerified',
    //     ]);
    // });
});
