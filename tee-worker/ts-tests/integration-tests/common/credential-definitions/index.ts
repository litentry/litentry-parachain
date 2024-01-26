import { vip3MembershipCardGold } from './vip3/vip3-membership-card-gold';
import { vip3MembershipCardSilver } from './vip3/vip3-membership-card-sliver';
import { tokenHolderDot } from './achainable/token-holder-dot';
import { tokenHolderEth } from './achainable/token-holder-eth';
export const credentialDefinitionMap: any = {
    'vip3-membership-card-gold': vip3MembershipCardGold,
    'vip3-membership-card-silver': vip3MembershipCardSilver,
    // 'token-holder-dot': tokenHolderDot,
    // 'token-holder-eth': tokenHolderEth,
};
