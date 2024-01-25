import { HexString } from '@polkadot/util/types';
import * as dataProviders from '../config/data-providers';
import { Networks, AccountNetworks } from '../config/networks';
import { CredentialDefinition } from '../config/types';

const mockAddress: HexString = '0x651614cA9097C5ba189Ef85e7851Ef9cff592B2c';
export const vip3MembershipCardGold: CredentialDefinition = {
    id: 'vip3-membership-card-gold',
    name: 'VIP3 Membership Card Gold',
    description: 'VIP3 Membership Card Gold',
    assertion: {
        id: 'Vip3MembershipCard',
        payload: 'Gold',
    },
    dataProvider: dataProviders.vip3,
    network: Networks['ethereum'],

    // mock data for link-identity via cli
    mockDid: `litentry:${AccountNetworks['evm']}:${mockAddress}`,
    mockWeb3Network: `${Networks['bsc']},${Networks['ethereum']}`,
};
