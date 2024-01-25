import * as dataProviders from '../config/data-providers'

export const vip3MembershipCardSliver = {
    id: 'vip3-membership-card-sliver',
    name: 'VIP3 Membership Card Sliver',
    description: 'VIP3 Membership Card Sliver',
    assertion: {
        VIP3MembershipCard: 'sliver',
    },
    dataProvider:dataProviders.vip3,
}