import type { IntegrationTestContext } from '../common-types';
import { Signer } from './crypto'

export const buildLinkingIdentity = async (context: IntegrationTestContext) => {
    const mainBitcoinIdentity = await context.web3Wallets.bitcoin.Alice.getIdentity(context);
    const mainEvmIdentity = await context.web3Wallets.evm.Alice.getIdentity(context);
    const mainSubstrateIdentity = await context.web3Wallets.substrate.Alice.getIdentity(context);
    const mainSolanaIdentity = await context.web3Wallets.solana.Alice.getIdentity(context);


    

};
