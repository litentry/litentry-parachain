import "@polkadot/api-base/types/consts";
import type { ApiTypes, AugmentedConst } from "@polkadot/api-base/types";
import type { u128, u16, u32, u64, u8 } from "@polkadot/types-codec";
import type {
    FrameSystemLimitsBlockLength,
    FrameSystemLimitsBlockWeights,
    SpVersionRuntimeVersion,
    SpWeightsRuntimeDbWeight,
} from "@polkadot/types/lookup";
export type __AugmentedConst<ApiType extends ApiTypes> = AugmentedConst<ApiType>;
declare module "@polkadot/api-base/types/consts" {
    interface AugmentedConsts<ApiType extends ApiTypes> {
        balances: {
            /**
             * The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!
             *
             * If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for
             * this pallet. However, you do so at your own risk: this will open up a major DoS vector.
             * In case you have multiple sources of provider references, you may also get unexpected
             * behaviour if you set this to zero.
             *
             * Bottom line: Do yourself a favour and make it at least one!
             **/
            existentialDeposit: u128 & AugmentedConst<ApiType>;
            /**
             * The maximum number of individual freeze locks that can exist on an account at any time.
             **/
            maxFreezes: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of holds that can exist on an account at any time.
             **/
            maxHolds: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of locks that should exist on an account.
             * Not strictly enforced, but used for weight estimation.
             **/
            maxLocks: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of named reserves that can exist on an account.
             **/
            maxReserves: u32 & AugmentedConst<ApiType>;
        };
        identityManagement: {
            /**
             * maximum number of identities an account can have, if you change this value to lower some accounts may exceed this limit
             **/
            maxIDGraphLength: u32 & AugmentedConst<ApiType>;
        };
        system: {
            /**
             * Maximum number of block number to block hash mappings to keep (oldest pruned first).
             **/
            blockHashCount: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum length of a block (in bytes).
             **/
            blockLength: FrameSystemLimitsBlockLength & AugmentedConst<ApiType>;
            /**
             * Block & extrinsics weights: base values and limits.
             **/
            blockWeights: FrameSystemLimitsBlockWeights & AugmentedConst<ApiType>;
            /**
             * The weight of runtime database operations the runtime can invoke.
             **/
            dbWeight: SpWeightsRuntimeDbWeight & AugmentedConst<ApiType>;
            /**
             * The designated SS58 prefix of this chain.
             *
             * This replaces the "ss58Format" property declared in the chain spec. Reason is
             * that the runtime should know about the prefix in order to make use of it as
             * an identifier of the chain.
             **/
            ss58Prefix: u16 & AugmentedConst<ApiType>;
            /**
             * Get the chain's current version.
             **/
            version: SpVersionRuntimeVersion & AugmentedConst<ApiType>;
        };
        timestamp: {
            /**
             * The minimum period between blocks. Beware that this is different to the *expected*
             * period that the block production apparatus provides. Your chosen consensus system will
             * generally work with this to determine a sensible block time. e.g. For Aura, it will be
             * double this period on default settings.
             **/
            minimumPeriod: u64 & AugmentedConst<ApiType>;
        };
        transactionPayment: {
            /**
             * A fee mulitplier for `Operational` extrinsics to compute "virtual tip" to boost their
             * `priority`
             *
             * This value is multipled by the `final_fee` to obtain a "virtual tip" that is later
             * added to a tip component in regular `priority` calculations.
             * It means that a `Normal` transaction can front-run a similarly-sized `Operational`
             * extrinsic (with no tip), by including a tip value greater than the virtual tip.
             *
             * ```rust,ignore
             * // For `Normal`
             * let priority = priority_calc(tip);
             *
             * // For `Operational`
             * let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;
             * let priority = priority_calc(tip + virtual_tip);
             * ```
             *
             * Note that since we use `final_fee` the multiplier applies also to the regular `tip`
             * sent with the transaction. So, not only does the transaction get a priority bump based
             * on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`
             * transactions.
             **/
            operationalFeeMultiplier: u8 & AugmentedConst<ApiType>;
        };
    }
}
//# sourceMappingURL=augment-api-consts.d.ts.map
