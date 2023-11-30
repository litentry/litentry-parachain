import {
    describeLitentry,
    buildIdentityTxs,
    buildIdentityHelper,
    buildValidations,
    checkErrorDetail,
    buildIdentityFromKeypair,
    PolkadotSigner,
} from './common/utils';
import { step } from 'mocha-steps';
import { sendTxsWithUtility } from './common/transactions';
import { generateWeb3Wallets, assertIdentityLinked, assertIdentityDeactivated } from './common/utils';
import { ethers } from 'ethers';
import type { LitentryPrimitivesIdentity } from 'sidechain-api';
import type { LitentryValidationData, Web3Network } from 'parachain-api';
import { Vec } from '@polkadot/types';

describeLitentry('Test Batch Utility', 0, (context) => {
    let identities: LitentryPrimitivesIdentity[] = [];
    let validations: LitentryValidationData[] = [];
    let evmSigners: ethers.Wallet[] = [];
    const we3networks: Web3Network[][] = [];
    const signerIdentities: LitentryPrimitivesIdentity[] = [];

    step('generate web3 wallets', async function () {
        const web3Wallets = await generateWeb3Wallets(3);
        evmSigners = web3Wallets.map((web3Signer) => {
            return web3Signer.evmWallet;
        });
    });

    step('batch test: link identities', async function () {
        const defaultNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum']);
        const aliceSubject = await buildIdentityFromKeypair(new PolkadotSigner(context.substrateWallet.alice), context);

        for (let index = 0; index < evmSigners.length; index++) {
            const signer = evmSigners[index];
            const evmIdentity = await buildIdentityHelper(signer.address, 'Evm', context);
            identities.push(evmIdentity);
            we3networks.push(defaultNetworks as unknown as Vec<Web3Network>); // @fixme #1878
            signerIdentities.push(aliceSubject);
        }

        const evmValidations = await buildValidations(
            context,
            signerIdentities,
            identities,
            1,
            'ethereum',
            undefined,
            evmSigners
        );
        validations = [...evmValidations];

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

        await assertIdentityDeactivated(context.substrateWallet.alice, deactivatedEvents);
    });

    step('batch test: deactivate error identities', async function () {
        identities = [];
        // prepare new identities that were not linked - so they do not exist
        for (let index = 0; index < evmSigners.length; index++) {
            const evmIdentity = await buildIdentityHelper('twitter_user_' + index, 'Twitter', context);
            identities.push(evmIdentity);
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
