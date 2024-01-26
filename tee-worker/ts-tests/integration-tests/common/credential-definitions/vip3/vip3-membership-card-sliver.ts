import { HexString } from '@polkadot/util/types';
import * as dataProviders from '../config/data-providers';
import { AccountNetworks, Networks } from '../config/networks';
import { CredentialDefinition } from '../config/types';

const mockAddress: HexString = '0x10CdF7F7A32E2F24c853AE6567b75D862Ee3B46f';
export const vip3MembershipCardSilver: CredentialDefinition = {
    id: 'vip3-membership-card-silver',
    name: 'VIP3 Membership Card Sliver',
    description: 'VIP3 Membership Card Silver',
    assertion: {
        id: 'Vip3MembershipCard',
        payload: 'Silver',
    },
    dataProvider: dataProviders.vip3,
    network: Networks['ethereum'],

    // mock data for link-identity via cli
    // https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/cli/src/trusted_base_cli/commands/litentry/link_identity.rs
    mockDid: `litentry:${AccountNetworks['evm']}:${mockAddress}`,
    mockWeb3Network: `${Networks['bsc']},${Networks['ethereum']}`,

    expectedCredentialValue: true,
};
