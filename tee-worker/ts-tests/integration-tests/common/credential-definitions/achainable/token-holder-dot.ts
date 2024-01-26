import { HexString } from '@polkadot/util/types';
import * as dataProviders from '../config/data-providers';
import { Networks, AccountNetworks } from '../config/networks';
import { CredentialDefinition } from '../config/types';

// https://polkadot.subscan.io/tools/format_transform
const mockAddress: HexString = '0xf20107661944f8a00a418d864292885767a2942e92abb5d3cdaf243fe065c171';
export const tokenHolderDot: CredentialDefinition = {
    id: 'token-holder-dot',
    name: 'DOT Holder',
    description: `The number of DOT tokens you hold > 0`,
    assertion: {
        id: 'Achainable',
        payload: {
            Amount: {
                name: 'Balance over {amount}',
                chain: 'Polkadot',
                amount: '0',
            },
        },
    },
    dataProvider: dataProviders.achainable,
    network: Networks['polkadot'],

    // mock data for link-identity via cli
    // https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/cli/src/trusted_base_cli/commands/litentry/link_identity.rs
    mockDid: `litentry:${AccountNetworks['substrate']}:${mockAddress}`,
    mockWeb3Network: `${Networks['litentry']},${Networks['kusama']}`,

    expectedCredentialValue: true,
};
