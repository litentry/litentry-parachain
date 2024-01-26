import { HexString } from '@polkadot/util/types';
import * as dataProviders from '../config/data-providers';
import { Networks, AccountNetworks } from '../config/networks';
import { CredentialDefinition } from '../config/types';

// https://polkadot.subscan.io/tools/format_transform
const mockAddress: HexString = '0xbB613509f2590ca489863551E7A27E89B863A8BD';
export const tokenHolderEth: CredentialDefinition = {
    id: 'token-holder-eth',
    name: 'ETH Holder',
    description: `The number of ETH tokens you hold > 0`,
    assertion: {
        id: 'Achainable',
        payload: {
            Amount: {
                name: 'Balance over {amount}',
                chain: 'Ethereum',
                amount: '0',
            },
        },
    },
    dataProvider: dataProviders.achainable,
    network: Networks['ethereum'],

    // mock data for link-identity via cli
    // https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/cli/src/trusted_base_cli/commands/litentry/link_identity.rs
    mockDid: `litentry:${AccountNetworks['evm']}:${mockAddress}`,
    mockWeb3Network: `${Networks['bsc']},${Networks['ethereum']}`,

    expectedCredentialValue: true,
};
