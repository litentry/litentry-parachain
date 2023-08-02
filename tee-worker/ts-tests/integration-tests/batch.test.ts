import {
    describeLitentry,
    buildIdentityTxs,
    handleIdentityEvents,
    buildIdentityHelper,
    buildValidations,
    checkErrorDetail,
    buildIdentityFromKeypair, PolkadotSigner,
} from './common/utils';
import { aesKey } from './common/call';
import { u8aToHex } from '@polkadot/util';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { multiAccountTxSender, sendTxsWithUtility } from './common/transactions';
import { generateWeb3Wallets, assertIdentityLinked, assertIdentityDeactivated } from './common/utils';
import { ethers } from 'ethers';
import type { LitentryPrimitivesIdentity } from 'sidechain-api';
import type { LitentryValidationData, Web3Network } from 'parachain-api';

describeLitentry('Test Batch Utility', 0, (context) => {
    let identities: LitentryPrimitivesIdentity[] = [];
    let validations: LitentryValidationData[] = [];
    let ethereumSigners: ethers.Wallet[] = [];
    const we3networks: Web3Network[][] = [];
    const signerIdentities: LitentryPrimitivesIdentity[] = [];

    step('generate web3 wallets', async function () {
        const web3Wallets = await generateWeb3Wallets(10);
        ethereumSigners = web3Wallets.map((web3Signer) => {
            return web3Signer.ethereumWallet;
        });
    });

    step('set user shielding key', async function () {
        const [aliceTxs] = await buildIdentityTxs(context, [context.substrateWallet.alice], [], 'setUserShieldingKey');
        const events = await multiAccountTxSender(
            context,
            [aliceTxs],
            [context.substrateWallet.alice],
            'identityManagement',
            ['UserShieldingKeySet']
        );
        const [alice] = await handleIdentityEvents(context, aesKey, events, 'UserShieldingKeySet');
        assert.equal(
            alice.who,
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'alice shielding key should be set'
        );
    });

    step('batch test: link identities', async function () {
        const defaultNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum']);
        const aliceSubject = await buildIdentityFromKeypair(new PolkadotSigner(context.substrateWallet.alice), context);

        for (let index = 0; index < ethereumSigners.length; index++) {
            const signer = ethereumSigners[index];
            const ethereumIdentity = await buildIdentityHelper(signer.address, 'Evm', context);
            identities.push(ethereumIdentity);
            we3networks.push(defaultNetworks);
            signerIdentities.push(aliceSubject);
        }

        const ethereumValidations = await buildValidations(
            context,
            signerIdentities,
            identities,
            1,
            'ethereum',
            undefined,
            ethereumSigners
        );
        validations = [...ethereumValidations];

        const txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            identities,
            'linkIdentity',
            validations,
            we3networks
        );
        const events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'identityManagement', [
            'IdentityLinked',
        ]);
        assertIdentityLinked(context, context.substrateWallet.alice, events, identities);
    });

    step('batch test: deactivate identities', async function () {
        const txs = await buildIdentityTxs(context, context.substrateWallet.alice, identities, 'deactivateIdentity');
        const deactivatedEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            txs,
            'identityManagement',
            ['IdentityDeactivated']
        );

        await assertIdentityDeactivated(context, context.substrateWallet.alice, deactivatedEvents);
    });

    step('batch test: deactivate error identities', async function () {
        identities = [];
        // prepare new identities that were not linked - so they do not exist
        for (let index = 0; index < ethereumSigners.length; index++) {
            const ethereumIdentity = await buildIdentityHelper('twitter_user_' + index, 'Twitter', context);
            identities.push(ethereumIdentity);
        }

        const txs = await buildIdentityTxs(context, context.substrateWallet.alice, identities, 'deactivateIdentity');
        const deactivatedEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            txs,
            'identityManagement',
            ['DeactivateIdentityFailed']
        );
        await checkErrorDetail(deactivatedEvents, 'IdentityNotExist');
    });

    step('check IDGraph after deactivateIdentity', async function () {
        // TODO: check the idgraph is empty
    });
});
