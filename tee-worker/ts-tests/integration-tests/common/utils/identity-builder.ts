import type { IntegrationTestContext } from '../common-types';
import { EthersSigner, PolkadotSigner, Signer } from './crypto';
import { randomEvmWallet, randomSubstrateWallet, genesisSubstrateWallet } from '../helpers';
import { CorePrimitivesIdentity, LitentryValidationData, Web3Network } from 'parachain-api';
import { buildValidations } from './identity-helper';
import { getSidechainNonce } from '../di-utils';
import type { Vec, Bytes } from '@polkadot/types';

export const buildLinkingIdentity = async (context: IntegrationTestContext) => {
    const mainBitcoinIdentity = await context.web3Wallets.bitcoin.Alice.getIdentity(context);
    const mainEvmIdentity = await context.web3Wallets.evm.Alice.getIdentity(context);
    const mainSubstrateIdentity = await context.web3Wallets.substrate.Alice.getIdentity(context);
    const mainSolanaIdentity = await context.web3Wallets.solana.Alice.getIdentity(context);

    const randomBitcoinIdentity = await context.web3Wallets.bitcoin.Bob.getIdentity(context);
};

export const buildRandomIdentity = async (context: IntegrationTestContext) => {
    const randomEvmSigner = new EthersSigner(randomEvmWallet());
    const randomSubstrateSigner = new PolkadotSigner(randomSubstrateWallet());
    // const randomBitcoinIdentity = await context.web3Wallets.bitcoin.randomWallet.getIdentity(context);
    const randomEvmIdentity = await randomEvmSigner.getIdentity(context);
    const randomSubstrateIdentity = await randomSubstrateSigner.getIdentity(context);
    // const aliceSolanaIdentity = await context.web3Wallets.solana.Alice.getIdentity(context);

    return [
        {
            // bitcoin: randomBitcoinIdentity,
            evm: randomEvmIdentity,
            substrate: randomSubstrateIdentity,
            // solana: aliceSolanaIdentity,
        },
    ];
};

export const buildSubstrateValidationHelper = async (
    aliceIdentity: CorePrimitivesIdentity,
    linkIdentity: CorePrimitivesIdentity,
    linkSigner: Signer,
    context: IntegrationTestContext
) => {
    const linkIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
        validation: LitentryValidationData;
        networks: Bytes | Vec<Web3Network>;
    }[] = [];
    const currentNonce = (await getSidechainNonce(context, aliceIdentity)).toNumber();
    console.log('currentNonce', currentNonce, currentNonce + 1, currentNonce + 2);

    const substrateValidation = await buildValidations(
        context,
        aliceIdentity,
        linkIdentity,
        currentNonce + 1,
        'substrate',
        linkSigner,
        { prettifiedMessage: true }
    );
    const substrateNetworks = context.api.createType('Vec<Web3Network>', ['Polkadot', 'Litentry']);

    linkIdentityRequestParams.push({
        nonce: currentNonce + 1,
        identity: linkIdentity,
        validation: substrateValidation,
        networks: substrateNetworks,
    });

    const evmValidation = await buildValidations(
        context,
        aliceIdentity,
        linkIdentity,
        currentNonce + 2,
        'ethereum',
        linkSigner,
        { prettifiedMessage: true }
    );
    const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);

    linkIdentityRequestParams.push({
        nonce: currentNonce + 2,
        identity: linkIdentity,
        validation: evmValidation,
        networks: evmNetworks,
    });

    return linkIdentityRequestParams;
};

export const buildEvmRandomValidation = async (
    context: IntegrationTestContext,
    aliceIdentity: CorePrimitivesIdentity,
    nonce: number
) => {
    const randomSigner = new EthersSigner(randomEvmWallet());
    const randomIdentity = await randomSigner.getIdentity(context);
    const validation = await buildValidations(
        context,
        aliceIdentity,
        randomIdentity,
        nonce++,
        'ethereum',
        randomSigner,
        { prettifiedMessage: true }
    );

    return {
        nonce: nonce,
        identity: randomIdentity,
        validation: validation,
    };
};

export const buildSubstrateRandomValidation = async (
    context: IntegrationTestContext,
    aliceIdentity: CorePrimitivesIdentity,
    nonce: number
) => {
    const randomSigner = new PolkadotSigner(randomSubstrateWallet());
    const randomIdentity = await randomSigner.getIdentity(context);
    const validation = await buildValidations(
        context,
        aliceIdentity,
        randomIdentity,
        nonce++,
        'substrate',
        randomSigner,
        { prettifiedMessage: true }
    );

    return {
        nonce: nonce,
        identity: randomIdentity,
        validation: validation,
    };
};
