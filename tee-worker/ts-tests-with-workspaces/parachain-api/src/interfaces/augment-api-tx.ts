// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/submittable";

import type {
    ApiTypes,
    AugmentedSubmittable,
    SubmittableExtrinsic,
    SubmittableExtrinsicFunction,
} from "@polkadot/api-base/types";
import type { Data } from "@polkadot/types";
import type {
    Bytes,
    Compact,
    Option,
    Struct,
    U8aFixed,
    Vec,
    bool,
    u128,
    u16,
    u32,
    u64,
    u8,
} from "@polkadot/types-codec";
import type { AnyNumber, IMethod, ITuple } from "@polkadot/types-codec/types";
import type {
    AccountId32,
    Call,
    H256,
    MultiAddress,
    Perbill,
    Percent,
} from "@polkadot/types/interfaces/runtime";
import type {
    CorePrimitivesAssertion,
    CorePrimitivesErrorImpError,
    CorePrimitivesErrorVcmpError,
    CorePrimitivesKeyAesOutput,
    CumulusPrimitivesParachainInherentParachainInherentData,
    FrameSupportPreimagesBounded,
    PalletAssetManagerAssetMetadata,
    PalletDemocracyConviction,
    PalletDemocracyMetadataOwner,
    PalletDemocracyVoteAccountVote,
    PalletExtrinsicFilterOperationalMode,
    PalletIdentityBitFlags,
    PalletIdentityIdentityInfo,
    PalletIdentityJudgement,
    PalletMultisigTimepoint,
    PalletVestingVestingInfo,
    RococoParachainRuntimeOriginCaller,
    RococoParachainRuntimeProxyType,
    RococoParachainRuntimeSessionKeys,
    RuntimeCommonXcmImplCurrencyId,
    SpWeightsWeightV2Weight,
    SubstrateFixedFixedU64,
    TeerexPrimitivesRequest,
    XcmV3MultiLocation,
    XcmV3WeightLimit,
    XcmVersionedMultiAsset,
    XcmVersionedMultiAssets,
    XcmVersionedMultiLocation,
    XcmVersionedXcm,
} from "@polkadot/types/lookup";

export type __AugmentedSubmittable = AugmentedSubmittable<() => unknown>;
export type __SubmittableExtrinsic<ApiType extends ApiTypes> = SubmittableExtrinsic<ApiType>;
export type __SubmittableExtrinsicFunction<ApiType extends ApiTypes> =
    SubmittableExtrinsicFunction<ApiType>;

declare module "@polkadot/api-base/types/submittable" {
    interface AugmentedSubmittables<ApiType extends ApiTypes> {
        assetManager: {
            /**
             * Add the xcm type mapping for a existing assetId, other assetType still exists if any.
             * TODO: Change add_asset_type with internal function wrapper
             **/
            addAssetType: AugmentedSubmittable<
                (
                    assetId: u128 | AnyNumber | Uint8Array,
                    newAssetType:
                        | RuntimeCommonXcmImplCurrencyId
                        | { SelfReserve: any }
                        | { ParachainReserve: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u128, RuntimeCommonXcmImplCurrencyId]
            >;
            /**
             * Register new asset with the asset manager
             * TODO::Reserve native token multilocation through GenesisBuild/RuntimeUpgrade
             * TODO::Add Multilocation filter for register
             **/
            registerForeignAssetType: AugmentedSubmittable<
                (
                    assetType:
                        | RuntimeCommonXcmImplCurrencyId
                        | { SelfReserve: any }
                        | { ParachainReserve: any }
                        | string
                        | Uint8Array,
                    metadata:
                        | PalletAssetManagerAssetMetadata
                        | {
                              name?: any;
                              symbol?: any;
                              decimals?: any;
                              minimalBalance?: any;
                              isFrozen?: any;
                          }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [RuntimeCommonXcmImplCurrencyId, PalletAssetManagerAssetMetadata]
            >;
            /**
             * We do not allow the destroy of asset id so far; So at least one AssetTpye should be
             * assigned to existing AssetId Both asset_type and potential new_default_asset_type must
             * be an existing relation with asset_id
             * TODO: Change remove_asset_type with internal function wrapper
             **/
            removeAssetType: AugmentedSubmittable<
                (
                    assetType:
                        | RuntimeCommonXcmImplCurrencyId
                        | { SelfReserve: any }
                        | { ParachainReserve: any }
                        | string
                        | Uint8Array,
                    newDefaultAssetType:
                        | Option<RuntimeCommonXcmImplCurrencyId>
                        | null
                        | Uint8Array
                        | RuntimeCommonXcmImplCurrencyId
                        | { SelfReserve: any }
                        | { ParachainReserve: any }
                        | string
                ) => SubmittableExtrinsic<ApiType>,
                [RuntimeCommonXcmImplCurrencyId, Option<RuntimeCommonXcmImplCurrencyId>]
            >;
            /**
             * Change the amount of units we are charging per execution second
             * for a given ForeignAssetType
             * 0 means not support
             **/
            setAssetUnitsPerSecond: AugmentedSubmittable<
                (
                    assetId: u128 | AnyNumber | Uint8Array,
                    unitsPerSecond: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u128, u128]
            >;
            updateForeignAssetMetadata: AugmentedSubmittable<
                (
                    assetId: u128 | AnyNumber | Uint8Array,
                    metadata:
                        | PalletAssetManagerAssetMetadata
                        | {
                              name?: any;
                              symbol?: any;
                              decimals?: any;
                              minimalBalance?: any;
                              isFrozen?: any;
                          }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u128, PalletAssetManagerAssetMetadata]
            >;
        };
        balances: {
            /**
             * Exactly as `transfer`, except the origin must be root and the source account may be
             * specified.
             * ## Complexity
             * - Same as transfer, but additional read and write because the source account is not
             * assumed to be in the overlay.
             **/
            forceTransfer: AugmentedSubmittable<
                (
                    source:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    value: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress, Compact<u128>]
            >;
            /**
             * Unreserve some balance from a user by force.
             *
             * Can only be called by ROOT.
             **/
            forceUnreserve: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, u128]
            >;
            /**
             * Set the balances of a given account.
             *
             * This will alter `FreeBalance` and `ReservedBalance` in storage. it will
             * also alter the total issuance of the system (`TotalIssuance`) appropriately.
             * If the new free or reserved balance is below the existential deposit,
             * it will reset the account nonce (`frame_system::AccountNonce`).
             *
             * The dispatch origin for this call is `root`.
             **/
            setBalance: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    newFree: Compact<u128> | AnyNumber | Uint8Array,
                    newReserved: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Compact<u128>, Compact<u128>]
            >;
            /**
             * Transfer some liquid free balance to another account.
             *
             * `transfer` will set the `FreeBalance` of the sender and receiver.
             * If the sender's account is below the existential deposit as a result
             * of the transfer, the account will be reaped.
             *
             * The dispatch origin for this call must be `Signed` by the transactor.
             *
             * ## Complexity
             * - Dependent on arguments but not critical, given proper implementations for input config
             * types. See related functions below.
             * - It contains a limited number of reads and writes internally and no complex
             * computation.
             *
             * Related functions:
             *
             * - `ensure_can_withdraw` is always called internally but has a bounded complexity.
             * - Transferring balances to accounts that did not exist before will cause
             * `T::OnNewAccount::on_new_account` to be called.
             * - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`.
             * - `transfer_keep_alive` works the same way as `transfer`, but has an additional check
             * that the transfer will not kill the origin account.
             **/
            transfer: AugmentedSubmittable<
                (
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    value: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Compact<u128>]
            >;
            /**
             * Transfer the entire transferable balance from the caller account.
             *
             * NOTE: This function only attempts to transfer _transferable_ balances. This means that
             * any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be
             * transferred by this function. To ensure that this function results in a killed account,
             * you might need to prepare the account by removing any reference counters, storage
             * deposits, etc...
             *
             * The dispatch origin of this call must be Signed.
             *
             * - `dest`: The recipient of the transfer.
             * - `keep_alive`: A boolean to determine if the `transfer_all` operation should send all
             * of the funds the account has, causing the sender account to be killed (false), or
             * transfer everything except at least the existential deposit, which will guarantee to
             * keep the sender account alive (true). ## Complexity
             * - O(1). Just like transfer, but reading the user's transferable balance first.
             **/
            transferAll: AugmentedSubmittable<
                (
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    keepAlive: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, bool]
            >;
            /**
             * Same as the [`transfer`] call, but with a check that the transfer will not kill the
             * origin account.
             *
             * 99% of the time you want [`transfer`] instead.
             *
             * [`transfer`]: struct.Pallet.html#method.transfer
             **/
            transferKeepAlive: AugmentedSubmittable<
                (
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    value: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Compact<u128>]
            >;
        };
        bounties: {
            /**
             * Accept the curator role for a bounty.
             * A deposit will be reserved from curator and refund upon successful payout.
             *
             * May only be called from the curator.
             *
             * ## Complexity
             * - O(1).
             **/
            acceptCurator: AugmentedSubmittable<
                (bountyId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Approve a bounty proposal. At a later time, the bounty will be funded and become active
             * and the original deposit will be returned.
             *
             * May only be called from `T::SpendOrigin`.
             *
             * ## Complexity
             * - O(1).
             **/
            approveBounty: AugmentedSubmittable<
                (bountyId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Award bounty to a beneficiary account. The beneficiary will be able to claim the funds
             * after a delay.
             *
             * The dispatch origin for this call must be the curator of this bounty.
             *
             * - `bounty_id`: Bounty ID to award.
             * - `beneficiary`: The beneficiary account whom will receive the payout.
             *
             * ## Complexity
             * - O(1).
             **/
            awardBounty: AugmentedSubmittable<
                (
                    bountyId: Compact<u32> | AnyNumber | Uint8Array,
                    beneficiary:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, MultiAddress]
            >;
            /**
             * Claim the payout from an awarded bounty after payout delay.
             *
             * The dispatch origin for this call must be the beneficiary of this bounty.
             *
             * - `bounty_id`: Bounty ID to claim.
             *
             * ## Complexity
             * - O(1).
             **/
            claimBounty: AugmentedSubmittable<
                (bountyId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Cancel a proposed or active bounty. All the funds will be sent to treasury and
             * the curator deposit will be unreserved if possible.
             *
             * Only `T::RejectOrigin` is able to cancel a bounty.
             *
             * - `bounty_id`: Bounty ID to cancel.
             *
             * ## Complexity
             * - O(1).
             **/
            closeBounty: AugmentedSubmittable<
                (bountyId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Extend the expiry time of an active bounty.
             *
             * The dispatch origin for this call must be the curator of this bounty.
             *
             * - `bounty_id`: Bounty ID to extend.
             * - `remark`: additional information.
             *
             * ## Complexity
             * - O(1).
             **/
            extendBountyExpiry: AugmentedSubmittable<
                (
                    bountyId: Compact<u32> | AnyNumber | Uint8Array,
                    remark: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, Bytes]
            >;
            /**
             * Propose a new bounty.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Payment: `TipReportDepositBase` will be reserved from the origin account, as well as
             * `DataDepositPerByte` for each byte in `reason`. It will be unreserved upon approval,
             * or slashed when rejected.
             *
             * - `curator`: The curator account whom will manage this bounty.
             * - `fee`: The curator fee.
             * - `value`: The total payment amount of this bounty, curator fee included.
             * - `description`: The description of this bounty.
             **/
            proposeBounty: AugmentedSubmittable<
                (
                    value: Compact<u128> | AnyNumber | Uint8Array,
                    description: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u128>, Bytes]
            >;
            /**
             * Assign a curator to a funded bounty.
             *
             * May only be called from `T::SpendOrigin`.
             *
             * ## Complexity
             * - O(1).
             **/
            proposeCurator: AugmentedSubmittable<
                (
                    bountyId: Compact<u32> | AnyNumber | Uint8Array,
                    curator:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    fee: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, MultiAddress, Compact<u128>]
            >;
            /**
             * Unassign curator from a bounty.
             *
             * This function can only be called by the `RejectOrigin` a signed origin.
             *
             * If this function is called by the `RejectOrigin`, we assume that the curator is
             * malicious or inactive. As a result, we will slash the curator when possible.
             *
             * If the origin is the curator, we take this as a sign they are unable to do their job and
             * they willingly give up. We could slash them, but for now we allow them to recover their
             * deposit and exit without issue. (We may want to change this if it is abused.)
             *
             * Finally, the origin can be anyone if and only if the curator is "inactive". This allows
             * anyone in the community to call out that a curator is not doing their due diligence, and
             * we should pick a new curator. In this case the curator should also be slashed.
             *
             * ## Complexity
             * - O(1).
             **/
            unassignCurator: AugmentedSubmittable<
                (bountyId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
        };
        bridgeTransfer: {
            setExternalBalances: AugmentedSubmittable<
                (externalBalances: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u128]
            >;
            setMaximumIssuance: AugmentedSubmittable<
                (maximumIssuance: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u128]
            >;
            /**
             * Executes a simple currency transfer using the bridge account as the source
             **/
            transfer: AugmentedSubmittable<
                (
                    to: AccountId32 | string | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array,
                    rid: U8aFixed | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, u128, U8aFixed]
            >;
            /**
             * Transfers some amount of the native token to some recipient on a (whitelisted)
             * destination chain.
             **/
            transferNative: AugmentedSubmittable<
                (
                    amount: u128 | AnyNumber | Uint8Array,
                    recipient: Bytes | string | Uint8Array,
                    destId: u8 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u128, Bytes, u8]
            >;
        };
        chainBridge: {
            /**
             * Commits a vote in favour of the provided proposal.
             *
             * If a proposal with the given nonce and source chain ID does not already exist, it will
             * be created with an initial vote in favour from the caller.
             *
             * # <weight>
             * - weight of proposed call, regardless of whether execution is performed
             * # </weight>
             **/
            acknowledgeProposal: AugmentedSubmittable<
                (
                    nonce: u64 | AnyNumber | Uint8Array,
                    srcId: u8 | AnyNumber | Uint8Array,
                    rId: U8aFixed | string | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, u8, U8aFixed, Call]
            >;
            /**
             * Adds a new relayer to the relayer set.
             *
             * # <weight>
             * - O(1) lookup and insert
             * # </weight>
             **/
            addRelayer: AugmentedSubmittable<
                (v: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Evaluate the state of a proposal given the current vote threshold.
             *
             * A proposal with enough votes will be either executed or cancelled, and the status
             * will be updated accordingly.
             *
             * # <weight>
             * - weight of proposed call, regardless of whether execution is performed
             * # </weight>
             **/
            evalVoteState: AugmentedSubmittable<
                (
                    nonce: u64 | AnyNumber | Uint8Array,
                    srcId: u8 | AnyNumber | Uint8Array,
                    prop: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, u8, Call]
            >;
            /**
             * Commits a vote against a provided proposal.
             *
             * # <weight>
             * - Fixed, since execution of proposal should not be included
             * # </weight>
             **/
            rejectProposal: AugmentedSubmittable<
                (
                    nonce: u64 | AnyNumber | Uint8Array,
                    srcId: u8 | AnyNumber | Uint8Array,
                    rId: U8aFixed | string | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, u8, U8aFixed, Call]
            >;
            /**
             * Removes an existing relayer from the set.
             *
             * # <weight>
             * - O(1) lookup and removal
             * # </weight>
             **/
            removeRelayer: AugmentedSubmittable<
                (v: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Removes a resource ID from the resource mapping.
             *
             * After this call, bridge transfers with the associated resource ID will
             * be rejected.
             *
             * # <weight>
             * - O(1) removal
             * # </weight>
             **/
            removeResource: AugmentedSubmittable<
                (id: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [U8aFixed]
            >;
            /**
             * Stores a method name on chain under an associated resource ID.
             *
             * # <weight>
             * - O(1) write
             * # </weight>
             **/
            setResource: AugmentedSubmittable<
                (
                    id: U8aFixed | string | Uint8Array,
                    method: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [U8aFixed, Bytes]
            >;
            /**
             * Sets the vote threshold for proposals.
             *
             * This threshold is used to determine how many votes are required
             * before a proposal is executed.
             *
             * # <weight>
             * - O(1) lookup and insert
             * # </weight>
             **/
            setThreshold: AugmentedSubmittable<
                (threshold: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Change extra bridge transfer fee that user should pay
             *
             * # <weight>
             * - O(1) lookup and insert
             * # </weight>
             **/
            updateFee: AugmentedSubmittable<
                (
                    destId: u8 | AnyNumber | Uint8Array,
                    fee: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u8, u128]
            >;
            /**
             * Enables a chain ID as a source or destination for a bridge transfer.
             *
             * # <weight>
             * - O(1) lookup and insert
             * # </weight>
             **/
            whitelistChain: AugmentedSubmittable<
                (id: u8 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u8]
            >;
        };
        council: {
            /**
             * Close a vote that is either approved, disapproved or whose voting period has ended.
             *
             * May be called by any signed account in order to finish voting and close the proposal.
             *
             * If called before the end of the voting period it will only close the vote if it is
             * has enough votes to be approved or disapproved.
             *
             * If called after the end of the voting period abstentions are counted as rejections
             * unless there is a prime member set and the prime member cast an approval.
             *
             * If the close operation completes successfully with disapproval, the transaction fee will
             * be waived. Otherwise execution of the approved operation will be charged to the caller.
             *
             * + `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed
             * proposal.
             * + `length_bound`: The upper bound for the length of the proposal in storage. Checked via
             * `storage::read` so it is `size_of::<u32>() == 4` larger than the pure length.
             *
             * ## Complexity
             * - `O(B + M + P1 + P2)` where:
             * - `B` is `proposal` size in bytes (length-fee-bounded)
             * - `M` is members-count (code- and governance-bounded)
             * - `P1` is the complexity of `proposal` preimage.
             * - `P2` is proposal-count (code-bounded)
             **/
            close: AugmentedSubmittable<
                (
                    proposalHash: H256 | string | Uint8Array,
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    proposalWeightBound:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array,
                    lengthBound: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Compact<u32>, SpWeightsWeightV2Weight, Compact<u32>]
            >;
            /**
             * Close a vote that is either approved, disapproved or whose voting period has ended.
             *
             * May be called by any signed account in order to finish voting and close the proposal.
             *
             * If called before the end of the voting period it will only close the vote if it is
             * has enough votes to be approved or disapproved.
             *
             * If called after the end of the voting period abstentions are counted as rejections
             * unless there is a prime member set and the prime member cast an approval.
             *
             * If the close operation completes successfully with disapproval, the transaction fee will
             * be waived. Otherwise execution of the approved operation will be charged to the caller.
             *
             * + `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed
             * proposal.
             * + `length_bound`: The upper bound for the length of the proposal in storage. Checked via
             * `storage::read` so it is `size_of::<u32>() == 4` larger than the pure length.
             *
             * ## Complexity
             * - `O(B + M + P1 + P2)` where:
             * - `B` is `proposal` size in bytes (length-fee-bounded)
             * - `M` is members-count (code- and governance-bounded)
             * - `P1` is the complexity of `proposal` preimage.
             * - `P2` is proposal-count (code-bounded)
             **/
            closeOldWeight: AugmentedSubmittable<
                (
                    proposalHash: H256 | string | Uint8Array,
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    proposalWeightBound: Compact<u64> | AnyNumber | Uint8Array,
                    lengthBound: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Compact<u32>, Compact<u64>, Compact<u32>]
            >;
            /**
             * Disapprove a proposal, close, and remove it from the system, regardless of its current
             * state.
             *
             * Must be called by the Root origin.
             *
             * Parameters:
             * * `proposal_hash`: The hash of the proposal that should be disapproved.
             *
             * ## Complexity
             * O(P) where P is the number of max proposals
             **/
            disapproveProposal: AugmentedSubmittable<
                (proposalHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Dispatch a proposal from a member using the `Member` origin.
             *
             * Origin must be a member of the collective.
             *
             * ## Complexity:
             * - `O(B + M + P)` where:
             * - `B` is `proposal` size in bytes (length-fee-bounded)
             * - `M` members-count (code-bounded)
             * - `P` complexity of dispatching `proposal`
             **/
            execute: AugmentedSubmittable<
                (
                    proposal: Call | IMethod | string | Uint8Array,
                    lengthBound: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Call, Compact<u32>]
            >;
            /**
             * Add a new proposal to either be voted on or executed directly.
             *
             * Requires the sender to be member.
             *
             * `threshold` determines whether `proposal` is executed directly (`threshold < 2`)
             * or put up for voting.
             *
             * ## Complexity
             * - `O(B + M + P1)` or `O(B + M + P2)` where:
             * - `B` is `proposal` size in bytes (length-fee-bounded)
             * - `M` is members-count (code- and governance-bounded)
             * - branching is influenced by `threshold` where:
             * - `P1` is proposal execution complexity (`threshold < 2`)
             * - `P2` is proposals-count (code-bounded) (`threshold >= 2`)
             **/
            propose: AugmentedSubmittable<
                (
                    threshold: Compact<u32> | AnyNumber | Uint8Array,
                    proposal: Call | IMethod | string | Uint8Array,
                    lengthBound: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, Call, Compact<u32>]
            >;
            /**
             * Set the collective's membership.
             *
             * - `new_members`: The new member list. Be nice to the chain and provide it sorted.
             * - `prime`: The prime member whose vote sets the default.
             * - `old_count`: The upper bound for the previous number of members in storage. Used for
             * weight estimation.
             *
             * The dispatch of this call must be `SetMembersOrigin`.
             *
             * NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but
             * the weight estimations rely on it to estimate dispatchable weight.
             *
             * # WARNING:
             *
             * The `pallet-collective` can also be managed by logic outside of the pallet through the
             * implementation of the trait [`ChangeMembers`].
             * Any call to `set_members` must be careful that the member set doesn't get out of sync
             * with other logic managing the member set.
             *
             * ## Complexity:
             * - `O(MP + N)` where:
             * - `M` old-members-count (code- and governance-bounded)
             * - `N` new-members-count (code- and governance-bounded)
             * - `P` proposals-count (code-bounded)
             **/
            setMembers: AugmentedSubmittable<
                (
                    newMembers: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    prime: Option<AccountId32> | null | Uint8Array | AccountId32 | string,
                    oldCount: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>, Option<AccountId32>, u32]
            >;
            /**
             * Add an aye or nay vote for the sender to the given proposal.
             *
             * Requires the sender to be a member.
             *
             * Transaction fees will be waived if the member is voting on any particular proposal
             * for the first time and the call is successful. Subsequent vote changes will charge a
             * fee.
             * ## Complexity
             * - `O(M)` where `M` is members-count (code- and governance-bounded)
             **/
            vote: AugmentedSubmittable<
                (
                    proposal: H256 | string | Uint8Array,
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    approve: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Compact<u32>, bool]
            >;
        };
        councilMembership: {
            /**
             * Add a member `who` to the set.
             *
             * May only be called from `T::AddOrigin`.
             **/
            addMember: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Swap out the sending member for some other key `new`.
             *
             * May only be called from `Signed` origin of a current member.
             *
             * Prime membership is passed from the origin account to `new`, if extant.
             **/
            changeKey: AugmentedSubmittable<
                (
                    updated:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Remove the prime member if it exists.
             *
             * May only be called from `T::PrimeOrigin`.
             **/
            clearPrime: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Remove a member `who` from the set.
             *
             * May only be called from `T::RemoveOrigin`.
             **/
            removeMember: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Change the membership to a new set, disregarding the existing membership. Be nice and
             * pass `members` pre-sorted.
             *
             * May only be called from `T::ResetOrigin`.
             **/
            resetMembers: AugmentedSubmittable<
                (
                    members: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /**
             * Set the prime member. Must be a current member.
             *
             * May only be called from `T::PrimeOrigin`.
             **/
            setPrime: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Swap out one member `remove` for another `add`.
             *
             * May only be called from `T::SwapOrigin`.
             *
             * Prime membership is *not* passed from `remove` to `add`, if extant.
             **/
            swapMember: AugmentedSubmittable<
                (
                    remove:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    add:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress]
            >;
        };
        cumulusXcm: {};
        democracy: {
            /**
             * Permanently place a proposal into the blacklist. This prevents it from ever being
             * proposed again.
             *
             * If called on a queued public or external proposal, then this will result in it being
             * removed. If the `ref_index` supplied is an active referendum with the proposal hash,
             * then it will be cancelled.
             *
             * The dispatch origin of this call must be `BlacklistOrigin`.
             *
             * - `proposal_hash`: The proposal hash to blacklist permanently.
             * - `ref_index`: An ongoing referendum whose hash is `proposal_hash`, which will be
             * cancelled.
             *
             * Weight: `O(p)` (though as this is an high-privilege dispatch, we assume it has a
             * reasonable value).
             **/
            blacklist: AugmentedSubmittable<
                (
                    proposalHash: H256 | string | Uint8Array,
                    maybeRefIndex: Option<u32> | null | Uint8Array | u32 | AnyNumber
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Option<u32>]
            >;
            /**
             * Remove a proposal.
             *
             * The dispatch origin of this call must be `CancelProposalOrigin`.
             *
             * - `prop_index`: The index of the proposal to cancel.
             *
             * Weight: `O(p)` where `p = PublicProps::<T>::decode_len()`
             **/
            cancelProposal: AugmentedSubmittable<
                (propIndex: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Remove a referendum.
             *
             * The dispatch origin of this call must be _Root_.
             *
             * - `ref_index`: The index of the referendum to cancel.
             *
             * # Weight: `O(1)`.
             **/
            cancelReferendum: AugmentedSubmittable<
                (refIndex: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Clears all public proposals.
             *
             * The dispatch origin of this call must be _Root_.
             *
             * Weight: `O(1)`.
             **/
            clearPublicProposals: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Delegate the voting power (with some given conviction) of the sending account.
             *
             * The balance delegated is locked for as long as it's delegated, and thereafter for the
             * time appropriate for the conviction's lock period.
             *
             * The dispatch origin of this call must be _Signed_, and the signing account must either:
             * - be delegating already; or
             * - have no voting activity (if there is, then it will need to be removed/consolidated
             * through `reap_vote` or `unvote`).
             *
             * - `to`: The account whose voting the `target` account's voting power will follow.
             * - `conviction`: The conviction that will be attached to the delegated votes. When the
             * account is undelegated, the funds will be locked for the corresponding period.
             * - `balance`: The amount of the account's balance to be used in delegating. This must not
             * be more than the account's current balance.
             *
             * Emits `Delegated`.
             *
             * Weight: `O(R)` where R is the number of referendums the voter delegating to has
             * voted on. Weight is charged as if maximum votes.
             **/
            delegate: AugmentedSubmittable<
                (
                    to:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    conviction:
                        | PalletDemocracyConviction
                        | "None"
                        | "Locked1x"
                        | "Locked2x"
                        | "Locked3x"
                        | "Locked4x"
                        | "Locked5x"
                        | "Locked6x"
                        | number
                        | Uint8Array,
                    balance: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, PalletDemocracyConviction, u128]
            >;
            /**
             * Schedule an emergency cancellation of a referendum. Cannot happen twice to the same
             * referendum.
             *
             * The dispatch origin of this call must be `CancellationOrigin`.
             *
             * -`ref_index`: The index of the referendum to cancel.
             *
             * Weight: `O(1)`.
             **/
            emergencyCancel: AugmentedSubmittable<
                (refIndex: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Schedule a referendum to be tabled once it is legal to schedule an external
             * referendum.
             *
             * The dispatch origin of this call must be `ExternalOrigin`.
             *
             * - `proposal_hash`: The preimage hash of the proposal.
             **/
            externalPropose: AugmentedSubmittable<
                (
                    proposal:
                        | FrameSupportPreimagesBounded
                        | { Legacy: any }
                        | { Inline: any }
                        | { Lookup: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [FrameSupportPreimagesBounded]
            >;
            /**
             * Schedule a negative-turnout-bias referendum to be tabled next once it is legal to
             * schedule an external referendum.
             *
             * The dispatch of this call must be `ExternalDefaultOrigin`.
             *
             * - `proposal_hash`: The preimage hash of the proposal.
             *
             * Unlike `external_propose`, blacklisting has no effect on this and it may replace a
             * pre-scheduled `external_propose` call.
             *
             * Weight: `O(1)`
             **/
            externalProposeDefault: AugmentedSubmittable<
                (
                    proposal:
                        | FrameSupportPreimagesBounded
                        | { Legacy: any }
                        | { Inline: any }
                        | { Lookup: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [FrameSupportPreimagesBounded]
            >;
            /**
             * Schedule a majority-carries referendum to be tabled next once it is legal to schedule
             * an external referendum.
             *
             * The dispatch of this call must be `ExternalMajorityOrigin`.
             *
             * - `proposal_hash`: The preimage hash of the proposal.
             *
             * Unlike `external_propose`, blacklisting has no effect on this and it may replace a
             * pre-scheduled `external_propose` call.
             *
             * Weight: `O(1)`
             **/
            externalProposeMajority: AugmentedSubmittable<
                (
                    proposal:
                        | FrameSupportPreimagesBounded
                        | { Legacy: any }
                        | { Inline: any }
                        | { Lookup: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [FrameSupportPreimagesBounded]
            >;
            /**
             * Schedule the currently externally-proposed majority-carries referendum to be tabled
             * immediately. If there is no externally-proposed referendum currently, or if there is one
             * but it is not a majority-carries referendum then it fails.
             *
             * The dispatch of this call must be `FastTrackOrigin`.
             *
             * - `proposal_hash`: The hash of the current external proposal.
             * - `voting_period`: The period that is allowed for voting on this proposal. Increased to
             * Must be always greater than zero.
             * For `FastTrackOrigin` must be equal or greater than `FastTrackVotingPeriod`.
             * - `delay`: The number of block after voting has ended in approval and this should be
             * enacted. This doesn't have a minimum amount.
             *
             * Emits `Started`.
             *
             * Weight: `O(1)`
             **/
            fastTrack: AugmentedSubmittable<
                (
                    proposalHash: H256 | string | Uint8Array,
                    votingPeriod: u32 | AnyNumber | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, u32, u32]
            >;
            /**
             * Propose a sensitive action to be taken.
             *
             * The dispatch origin of this call must be _Signed_ and the sender must
             * have funds to cover the deposit.
             *
             * - `proposal_hash`: The hash of the proposal preimage.
             * - `value`: The amount of deposit (must be at least `MinimumDeposit`).
             *
             * Emits `Proposed`.
             **/
            propose: AugmentedSubmittable<
                (
                    proposal:
                        | FrameSupportPreimagesBounded
                        | { Legacy: any }
                        | { Inline: any }
                        | { Lookup: any }
                        | string
                        | Uint8Array,
                    value: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [FrameSupportPreimagesBounded, Compact<u128>]
            >;
            /**
             * Remove a vote for a referendum.
             *
             * If the `target` is equal to the signer, then this function is exactly equivalent to
             * `remove_vote`. If not equal to the signer, then the vote must have expired,
             * either because the referendum was cancelled, because the voter lost the referendum or
             * because the conviction period is over.
             *
             * The dispatch origin of this call must be _Signed_.
             *
             * - `target`: The account of the vote to be removed; this account must have voted for
             * referendum `index`.
             * - `index`: The index of referendum of the vote to be removed.
             *
             * Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on.
             * Weight is calculated for the maximum number of vote.
             **/
            removeOtherVote: AugmentedSubmittable<
                (
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    index: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, u32]
            >;
            /**
             * Remove a vote for a referendum.
             *
             * If:
             * - the referendum was cancelled, or
             * - the referendum is ongoing, or
             * - the referendum has ended such that
             * - the vote of the account was in opposition to the result; or
             * - there was no conviction to the account's vote; or
             * - the account made a split vote
             * ...then the vote is removed cleanly and a following call to `unlock` may result in more
             * funds being available.
             *
             * If, however, the referendum has ended and:
             * - it finished corresponding to the vote of the account, and
             * - the account made a standard vote with conviction, and
             * - the lock period of the conviction is not over
             * ...then the lock will be aggregated into the overall account's lock, which may involve
             * *overlocking* (where the two locks are combined into a single lock that is the maximum
             * of both the amount locked and the time is it locked for).
             *
             * The dispatch origin of this call must be _Signed_, and the signer must have a vote
             * registered for referendum `index`.
             *
             * - `index`: The index of referendum of the vote to be removed.
             *
             * Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on.
             * Weight is calculated for the maximum number of vote.
             **/
            removeVote: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Signals agreement with a particular proposal.
             *
             * The dispatch origin of this call must be _Signed_ and the sender
             * must have funds to cover the deposit, equal to the original deposit.
             *
             * - `proposal`: The index of the proposal to second.
             **/
            second: AugmentedSubmittable<
                (proposal: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Set or clear a metadata of a proposal or a referendum.
             *
             * Parameters:
             * - `origin`: Must correspond to the `MetadataOwner`.
             * - `ExternalOrigin` for an external proposal with the `SuperMajorityApprove`
             * threshold.
             * - `ExternalDefaultOrigin` for an external proposal with the `SuperMajorityAgainst`
             * threshold.
             * - `ExternalMajorityOrigin` for an external proposal with the `SimpleMajority`
             * threshold.
             * - `Signed` by a creator for a public proposal.
             * - `Signed` to clear a metadata for a finished referendum.
             * - `Root` to set a metadata for an ongoing referendum.
             * - `owner`: an identifier of a metadata owner.
             * - `maybe_hash`: The hash of an on-chain stored preimage. `None` to clear a metadata.
             **/
            setMetadata: AugmentedSubmittable<
                (
                    owner:
                        | PalletDemocracyMetadataOwner
                        | { External: any }
                        | { Proposal: any }
                        | { Referendum: any }
                        | string
                        | Uint8Array,
                    maybeHash: Option<H256> | null | Uint8Array | H256 | string
                ) => SubmittableExtrinsic<ApiType>,
                [PalletDemocracyMetadataOwner, Option<H256>]
            >;
            /**
             * Undelegate the voting power of the sending account.
             *
             * Tokens may be unlocked following once an amount of time consistent with the lock period
             * of the conviction with which the delegation was issued.
             *
             * The dispatch origin of this call must be _Signed_ and the signing account must be
             * currently delegating.
             *
             * Emits `Undelegated`.
             *
             * Weight: `O(R)` where R is the number of referendums the voter delegating to has
             * voted on. Weight is charged as if maximum votes.
             **/
            undelegate: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Unlock tokens that have an expired lock.
             *
             * The dispatch origin of this call must be _Signed_.
             *
             * - `target`: The account to remove the lock on.
             *
             * Weight: `O(R)` with R number of vote of target.
             **/
            unlock: AugmentedSubmittable<
                (
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Veto and blacklist the external proposal hash.
             *
             * The dispatch origin of this call must be `VetoOrigin`.
             *
             * - `proposal_hash`: The preimage hash of the proposal to veto and blacklist.
             *
             * Emits `Vetoed`.
             *
             * Weight: `O(V + log(V))` where V is number of `existing vetoers`
             **/
            vetoExternal: AugmentedSubmittable<
                (proposalHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Vote in a referendum. If `vote.is_aye()`, the vote is to enact the proposal;
             * otherwise it is a vote to keep the status quo.
             *
             * The dispatch origin of this call must be _Signed_.
             *
             * - `ref_index`: The index of the referendum to vote for.
             * - `vote`: The vote configuration.
             **/
            vote: AugmentedSubmittable<
                (
                    refIndex: Compact<u32> | AnyNumber | Uint8Array,
                    vote:
                        | PalletDemocracyVoteAccountVote
                        | { Standard: any }
                        | { Split: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, PalletDemocracyVoteAccountVote]
            >;
        };
        dmpQueue: {
            /**
             * Service a single overweight message.
             **/
            serviceOverweight: AugmentedSubmittable<
                (
                    index: u64 | AnyNumber | Uint8Array,
                    weightLimit:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, SpWeightsWeightV2Weight]
            >;
        };
        drop3: {
            /**
             * Approve a RewardPool proposal, must be called from admin
             **/
            approveRewardPool: AugmentedSubmittable<
                (id: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Close a reward pool, can be called by admin or reward pool owner
             *
             * Note here `approved` state is not required, which gives the owner a
             * chance to close it before the admin evaluates the proposal
             **/
            closeRewardPool: AugmentedSubmittable<
                (id: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Create a RewardPool proposal, can be called by any signed account
             **/
            proposeRewardPool: AugmentedSubmittable<
                (
                    name: Bytes | string | Uint8Array,
                    total: u128 | AnyNumber | Uint8Array,
                    startAt: u32 | AnyNumber | Uint8Array,
                    endAt: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, u128, u32, u32]
            >;
            /**
             * Reject a RewardPool proposal, must be called from admin
             **/
            rejectRewardPool: AugmentedSubmittable<
                (id: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * transfer an amount of reserved balance to some other user
             * must be called by reward pool owner
             * TODO:
             * `repatriate_reserved()` requires that the destination account is active
             * otherwise `DeadAccount` error is returned. Is it OK in our case?
             **/
            sendReward: AugmentedSubmittable<
                (
                    id: u64 | AnyNumber | Uint8Array,
                    to: AccountId32 | string | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, AccountId32, u128]
            >;
            /**
             * Change the admin account
             * similar to sudo.set_key, the old account will be supplied in event
             **/
            setAdmin: AugmentedSubmittable<
                (updated: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Start a reward pool, can be called by admin or reward pool owner
             **/
            startRewardPool: AugmentedSubmittable<
                (id: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Stop a reward pool, can be called by admin or reward pool owner
             **/
            stopRewardPool: AugmentedSubmittable<
                (id: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
        };
        extrinsicFilter: {
            /**
             * block the given extrinsics
             * (pallet_name_bytes, function_name_bytes) can uniquely identify an extrinsic
             * if function_name_bytes is None, all extrinsics in `pallet_name_bytes` will be blocked
             **/
            blockExtrinsics: AugmentedSubmittable<
                (
                    palletNameBytes: Bytes | string | Uint8Array,
                    functionNameBytes: Option<Bytes> | null | Uint8Array | Bytes | string
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, Option<Bytes>]
            >;
            /**
             * Set the mode
             *
             * The storage of `BlockedExtrinsics` is unaffected.
             * The reason is we'd rather have this pallet behave conservatively:
             * having extra blocked extrinsics is better than having unexpected whitelisted extrinsics.
             * See the test `set_mode_should_not_clear_blocked_extrinsics()`
             *
             * Weights should be 2 DB writes: 1 for mode and 1 for event
             **/
            setMode: AugmentedSubmittable<
                (
                    mode:
                        | PalletExtrinsicFilterOperationalMode
                        | "Normal"
                        | "Safe"
                        | "Test"
                        | number
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PalletExtrinsicFilterOperationalMode]
            >;
            /**
             * unblock the given extrinsics
             * (pallet_name_bytes, function_name_bytes) can uniquely identify an extrinsic
             * if function_name_bytes is None, all extrinsics in `pallet_name_bytes` will be unblocked
             **/
            unblockExtrinsics: AugmentedSubmittable<
                (
                    palletNameBytes: Bytes | string | Uint8Array,
                    functionNameBytes: Option<Bytes> | null | Uint8Array | Bytes | string
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, Option<Bytes>]
            >;
        };
        identityManagement: {
            /**
             * add an account to the delegatees
             **/
            addDelegatee: AugmentedSubmittable<
                (account: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Create an identity
             * We do the origin check for this extrinsic, it has to be
             * - either the caller him/herself, i.e. ensure_signed(origin)? == who
             * - or from a delegatee in the list
             **/
            createIdentity: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    user: AccountId32 | string | Uint8Array,
                    encryptedIdentity: Bytes | string | Uint8Array,
                    encryptedMetadata: Option<Bytes> | null | Uint8Array | Bytes | string
                ) => SubmittableExtrinsic<ApiType>,
                [H256, AccountId32, Bytes, Option<Bytes>]
            >;
            identityCreated: AugmentedSubmittable<
                (
                    account: AccountId32 | string | Uint8Array,
                    identity:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array,
                    code:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array,
                    reqExtHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, CorePrimitivesKeyAesOutput, CorePrimitivesKeyAesOutput, H256]
            >;
            identityRemoved: AugmentedSubmittable<
                (
                    account: AccountId32 | string | Uint8Array,
                    identity:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array,
                    reqExtHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, CorePrimitivesKeyAesOutput, H256]
            >;
            identityVerified: AugmentedSubmittable<
                (
                    account: AccountId32 | string | Uint8Array,
                    identity:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array,
                    idGraph:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array,
                    reqExtHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, CorePrimitivesKeyAesOutput, CorePrimitivesKeyAesOutput, H256]
            >;
            /**
             * remove an account from the delegatees
             **/
            removeDelegatee: AugmentedSubmittable<
                (account: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Remove an identity
             **/
            removeIdentity: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    encryptedIdentity: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Bytes]
            >;
            /**
             * Set or update user's shielding key
             **/
            setUserShieldingKey: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    encryptedKey: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Bytes]
            >;
            someError: AugmentedSubmittable<
                (
                    account: Option<AccountId32> | null | Uint8Array | AccountId32 | string,
                    error:
                        | CorePrimitivesErrorImpError
                        | { SetUserShieldingKeyFailed: any }
                        | { CreateIdentityFailed: any }
                        | { RemoveIdentityFailed: any }
                        | { VerifyIdentityFailed: any }
                        | { ImportScheduledEnclaveFailed: any }
                        | { UnclassifiedError: any }
                        | string
                        | Uint8Array,
                    reqExtHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Option<AccountId32>, CorePrimitivesErrorImpError, H256]
            >;
            /**
             * ---------------------------------------------------
             * The following extrinsics are supposed to be called by TEE only
             * ---------------------------------------------------
             **/
            userShieldingKeySet: AugmentedSubmittable<
                (
                    account: AccountId32 | string | Uint8Array,
                    idGraph:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array,
                    reqExtHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, CorePrimitivesKeyAesOutput, H256]
            >;
            /**
             * Verify an identity
             **/
            verifyIdentity: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    encryptedIdentity: Bytes | string | Uint8Array,
                    encryptedValidationData: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Bytes, Bytes]
            >;
        };
        identityManagementMock: {
            /**
             * add an account to the delegatees
             **/
            addDelegatee: AugmentedSubmittable<
                (account: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Create an identity
             **/
            createIdentity: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    user: AccountId32 | string | Uint8Array,
                    encryptedIdentity: Bytes | string | Uint8Array,
                    encryptedMetadata: Option<Bytes> | null | Uint8Array | Bytes | string
                ) => SubmittableExtrinsic<ApiType>,
                [H256, AccountId32, Bytes, Option<Bytes>]
            >;
            identityCreated: AugmentedSubmittable<
                (
                    account: AccountId32 | string | Uint8Array,
                    identity:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array,
                    code:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array,
                    idGraph:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [
                    AccountId32,
                    CorePrimitivesKeyAesOutput,
                    CorePrimitivesKeyAesOutput,
                    CorePrimitivesKeyAesOutput
                ]
            >;
            identityRemoved: AugmentedSubmittable<
                (
                    account: AccountId32 | string | Uint8Array,
                    identity:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array,
                    idGraph:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, CorePrimitivesKeyAesOutput, CorePrimitivesKeyAesOutput]
            >;
            identityVerified: AugmentedSubmittable<
                (
                    account: AccountId32 | string | Uint8Array,
                    identity:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array,
                    idGraph:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, CorePrimitivesKeyAesOutput, CorePrimitivesKeyAesOutput]
            >;
            /**
             * remove an account from the delegatees
             **/
            removeDelegatee: AugmentedSubmittable<
                (account: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Remove an identity
             **/
            removeIdentity: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    encryptedIdentity: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Bytes]
            >;
            /**
             * Set or update user's shielding key
             **/
            setUserShieldingKey: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    encryptedKey: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Bytes]
            >;
            someError: AugmentedSubmittable<
                (
                    func: Bytes | string | Uint8Array,
                    error: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, Bytes]
            >;
            userShieldingKeySet: AugmentedSubmittable<
                (account: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Verify a created identity
             **/
            verifyIdentity: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    encryptedIdentity: Bytes | string | Uint8Array,
                    encryptedValidationData: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Bytes, Bytes]
            >;
        };
        impExtrinsicWhitelist: {
            /**
             * Adds a new group member
             **/
            addGroupMember: AugmentedSubmittable<
                (v: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Batch adding of new group members
             **/
            batchAddGroupMembers: AugmentedSubmittable<
                (
                    vs: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /**
             * Batch Removing existing group members
             **/
            batchRemoveGroupMembers: AugmentedSubmittable<
                (
                    vs: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /**
             * Removes an existing group members
             **/
            removeGroupMember: AugmentedSubmittable<
                (v: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Swith GroupControlOn off
             **/
            switchGroupControlOff: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Swith GroupControlOn on
             **/
            switchGroupControlOn: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
        };
        multisig: {
            /**
             * Register approval for a dispatch to be made from a deterministic composite account if
             * approved by a total of `threshold - 1` of `other_signatories`.
             *
             * Payment: `DepositBase` will be reserved if this is the first approval, plus
             * `threshold` times `DepositFactor`. It is returned once this dispatch happens or
             * is cancelled.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `threshold`: The total number of approvals for this dispatch before it is executed.
             * - `other_signatories`: The accounts (other than the sender) who can approve this
             * dispatch. May not be empty.
             * - `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is
             * not the first approval, then it must be `Some`, with the timepoint (block number and
             * transaction index) of the first approval transaction.
             * - `call_hash`: The hash of the call to be executed.
             *
             * NOTE: If this is the final approval, you will want to use `as_multi` instead.
             *
             * ## Complexity
             * - `O(S)`.
             * - Up to one balance-reserve or unreserve operation.
             * - One passthrough operation, one insert, both `O(S)` where `S` is the number of
             * signatories. `S` is capped by `MaxSignatories`, with weight being proportional.
             * - One encode & hash, both of complexity `O(S)`.
             * - Up to one binary search and insert (`O(logS + S)`).
             * - I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove.
             * - One event.
             * - Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit
             * taken for its lifetime of `DepositBase + threshold * DepositFactor`.
             **/
            approveAsMulti: AugmentedSubmittable<
                (
                    threshold: u16 | AnyNumber | Uint8Array,
                    otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    maybeTimepoint:
                        | Option<PalletMultisigTimepoint>
                        | null
                        | Uint8Array
                        | PalletMultisigTimepoint
                        | { height?: any; index?: any }
                        | string,
                    callHash: U8aFixed | string | Uint8Array,
                    maxWeight:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [
                    u16,
                    Vec<AccountId32>,
                    Option<PalletMultisigTimepoint>,
                    U8aFixed,
                    SpWeightsWeightV2Weight
                ]
            >;
            /**
             * Register approval for a dispatch to be made from a deterministic composite account if
             * approved by a total of `threshold - 1` of `other_signatories`.
             *
             * If there are enough, then dispatch the call.
             *
             * Payment: `DepositBase` will be reserved if this is the first approval, plus
             * `threshold` times `DepositFactor`. It is returned once this dispatch happens or
             * is cancelled.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `threshold`: The total number of approvals for this dispatch before it is executed.
             * - `other_signatories`: The accounts (other than the sender) who can approve this
             * dispatch. May not be empty.
             * - `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is
             * not the first approval, then it must be `Some`, with the timepoint (block number and
             * transaction index) of the first approval transaction.
             * - `call`: The call to be executed.
             *
             * NOTE: Unless this is the final approval, you will generally want to use
             * `approve_as_multi` instead, since it only requires a hash of the call.
             *
             * Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise
             * on success, result is `Ok` and the result from the interior call, if it was executed,
             * may be found in the deposited `MultisigExecuted` event.
             *
             * ## Complexity
             * - `O(S + Z + Call)`.
             * - Up to one balance-reserve or unreserve operation.
             * - One passthrough operation, one insert, both `O(S)` where `S` is the number of
             * signatories. `S` is capped by `MaxSignatories`, with weight being proportional.
             * - One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len.
             * - One encode & hash, both of complexity `O(S)`.
             * - Up to one binary search and insert (`O(logS + S)`).
             * - I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove.
             * - One event.
             * - The weight of the `call`.
             * - Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit
             * taken for its lifetime of `DepositBase + threshold * DepositFactor`.
             **/
            asMulti: AugmentedSubmittable<
                (
                    threshold: u16 | AnyNumber | Uint8Array,
                    otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    maybeTimepoint:
                        | Option<PalletMultisigTimepoint>
                        | null
                        | Uint8Array
                        | PalletMultisigTimepoint
                        | { height?: any; index?: any }
                        | string,
                    call: Call | IMethod | string | Uint8Array,
                    maxWeight:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [
                    u16,
                    Vec<AccountId32>,
                    Option<PalletMultisigTimepoint>,
                    Call,
                    SpWeightsWeightV2Weight
                ]
            >;
            /**
             * Immediately dispatch a multi-signature call using a single approval from the caller.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `other_signatories`: The accounts (other than the sender) who are part of the
             * multi-signature, but do not participate in the approval process.
             * - `call`: The call to be executed.
             *
             * Result is equivalent to the dispatched result.
             *
             * ## Complexity
             * O(Z + C) where Z is the length of the call and C its execution weight.
             **/
            asMultiThreshold1: AugmentedSubmittable<
                (
                    otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>, Call]
            >;
            /**
             * Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously
             * for this operation will be unreserved on success.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `threshold`: The total number of approvals for this dispatch before it is executed.
             * - `other_signatories`: The accounts (other than the sender) who can approve this
             * dispatch. May not be empty.
             * - `timepoint`: The timepoint (block number and transaction index) of the first approval
             * transaction for this dispatch.
             * - `call_hash`: The hash of the call to be executed.
             *
             * ## Complexity
             * - `O(S)`.
             * - Up to one balance-reserve or unreserve operation.
             * - One passthrough operation, one insert, both `O(S)` where `S` is the number of
             * signatories. `S` is capped by `MaxSignatories`, with weight being proportional.
             * - One encode & hash, both of complexity `O(S)`.
             * - One event.
             * - I/O: 1 read `O(S)`, one remove.
             * - Storage: removes one item.
             **/
            cancelAsMulti: AugmentedSubmittable<
                (
                    threshold: u16 | AnyNumber | Uint8Array,
                    otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    timepoint:
                        | PalletMultisigTimepoint
                        | { height?: any; index?: any }
                        | string
                        | Uint8Array,
                    callHash: U8aFixed | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Vec<AccountId32>, PalletMultisigTimepoint, U8aFixed]
            >;
        };
        parachainIdentity: {
            /**
             * Add a registrar to the system.
             *
             * The dispatch origin for this call must be `T::RegistrarOrigin`.
             *
             * - `account`: the account of the registrar.
             *
             * Emits `RegistrarAdded` if successful.
             *
             * ## Complexity
             * - `O(R)` where `R` registrar-count (governance-bounded and code-bounded).
             **/
            addRegistrar: AugmentedSubmittable<
                (
                    account:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Add the given account to the sender's subs.
             *
             * Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated
             * to the sender.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * sub identity of `sub`.
             **/
            addSub: AugmentedSubmittable<
                (
                    sub:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    data:
                        | Data
                        | { None: any }
                        | { Raw: any }
                        | { BlakeTwo256: any }
                        | { Sha256: any }
                        | { Keccak256: any }
                        | { ShaThree256: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Data]
            >;
            /**
             * Cancel a previous request.
             *
             * Payment: A previously reserved deposit is returned on success.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a
             * registered identity.
             *
             * - `reg_index`: The index of the registrar whose judgement is no longer requested.
             *
             * Emits `JudgementUnrequested` if successful.
             *
             * ## Complexity
             * - `O(R + X)`.
             * - where `R` registrar-count (governance-bounded).
             * - where `X` additional-field-count (deposit-bounded and code-bounded).
             **/
            cancelRequest: AugmentedSubmittable<
                (regIndex: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Clear an account's identity info and all sub-accounts and return all deposits.
             *
             * Payment: All reserved balances on the account are returned.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * identity.
             *
             * Emits `IdentityCleared` if successful.
             *
             * ## Complexity
             * - `O(R + S + X)`
             * - where `R` registrar-count (governance-bounded).
             * - where `S` subs-count (hard- and deposit-bounded).
             * - where `X` additional-field-count (deposit-bounded and code-bounded).
             **/
            clearIdentity: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Remove an account's identity and sub-account information and slash the deposits.
             *
             * Payment: Reserved balances from `set_subs` and `set_identity` are slashed and handled by
             * `Slash`. Verification request deposits are not returned; they should be cancelled
             * manually using `cancel_request`.
             *
             * The dispatch origin for this call must match `T::ForceOrigin`.
             *
             * - `target`: the account whose identity the judgement is upon. This must be an account
             * with a registered identity.
             *
             * Emits `IdentityKilled` if successful.
             *
             * ## Complexity
             * - `O(R + S + X)`
             * - where `R` registrar-count (governance-bounded).
             * - where `S` subs-count (hard- and deposit-bounded).
             * - where `X` additional-field-count (deposit-bounded and code-bounded).
             **/
            killIdentity: AugmentedSubmittable<
                (
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Provide a judgement for an account's identity.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account
             * of the registrar whose index is `reg_index`.
             *
             * - `reg_index`: the index of the registrar whose judgement is being made.
             * - `target`: the account whose identity the judgement is upon. This must be an account
             * with a registered identity.
             * - `judgement`: the judgement of the registrar of index `reg_index` about `target`.
             * - `identity`: The hash of the [`IdentityInfo`] for that the judgement is provided.
             *
             * Emits `JudgementGiven` if successful.
             *
             * ## Complexity
             * - `O(R + X)`.
             * - where `R` registrar-count (governance-bounded).
             * - where `X` additional-field-count (deposit-bounded and code-bounded).
             **/
            provideJudgement: AugmentedSubmittable<
                (
                    regIndex: Compact<u32> | AnyNumber | Uint8Array,
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    judgement:
                        | PalletIdentityJudgement
                        | { Unknown: any }
                        | { FeePaid: any }
                        | { Reasonable: any }
                        | { KnownGood: any }
                        | { OutOfDate: any }
                        | { LowQuality: any }
                        | { Erroneous: any }
                        | string
                        | Uint8Array,
                    identity: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, MultiAddress, PalletIdentityJudgement, H256]
            >;
            /**
             * Remove the sender as a sub-account.
             *
             * Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated
             * to the sender (*not* the original depositor).
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * super-identity.
             *
             * NOTE: This should not normally be used, but is provided in the case that the non-
             * controller of an account is maliciously registered as a sub-account.
             **/
            quitSub: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Remove the given account from the sender's subs.
             *
             * Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated
             * to the sender.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * sub identity of `sub`.
             **/
            removeSub: AugmentedSubmittable<
                (
                    sub:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Alter the associated name of the given sub-account.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * sub identity of `sub`.
             **/
            renameSub: AugmentedSubmittable<
                (
                    sub:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    data:
                        | Data
                        | { None: any }
                        | { Raw: any }
                        | { BlakeTwo256: any }
                        | { Sha256: any }
                        | { Keccak256: any }
                        | { ShaThree256: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Data]
            >;
            /**
             * Request a judgement from a registrar.
             *
             * Payment: At most `max_fee` will be reserved for payment to the registrar if judgement
             * given.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a
             * registered identity.
             *
             * - `reg_index`: The index of the registrar whose judgement is requested.
             * - `max_fee`: The maximum fee that may be paid. This should just be auto-populated as:
             *
             * ```nocompile
             * Self::registrars().get(reg_index).unwrap().fee
             * ```
             *
             * Emits `JudgementRequested` if successful.
             *
             * ## Complexity
             * - `O(R + X)`.
             * - where `R` registrar-count (governance-bounded).
             * - where `X` additional-field-count (deposit-bounded and code-bounded).
             **/
            requestJudgement: AugmentedSubmittable<
                (
                    regIndex: Compact<u32> | AnyNumber | Uint8Array,
                    maxFee: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, Compact<u128>]
            >;
            /**
             * Change the account associated with a registrar.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account
             * of the registrar whose index is `index`.
             *
             * - `index`: the index of the registrar whose fee is to be set.
             * - `new`: the new account ID.
             *
             * ## Complexity
             * - `O(R)`.
             * - where `R` registrar-count (governance-bounded).
             **/
            setAccountId: AugmentedSubmittable<
                (
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    updated:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, MultiAddress]
            >;
            /**
             * Set the fee required for a judgement to be requested from a registrar.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account
             * of the registrar whose index is `index`.
             *
             * - `index`: the index of the registrar whose fee is to be set.
             * - `fee`: the new fee.
             *
             * ## Complexity
             * - `O(R)`.
             * - where `R` registrar-count (governance-bounded).
             **/
            setFee: AugmentedSubmittable<
                (
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    fee: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, Compact<u128>]
            >;
            /**
             * Set the field information for a registrar.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account
             * of the registrar whose index is `index`.
             *
             * - `index`: the index of the registrar whose fee is to be set.
             * - `fields`: the fields that the registrar concerns themselves with.
             *
             * ## Complexity
             * - `O(R)`.
             * - where `R` registrar-count (governance-bounded).
             **/
            setFields: AugmentedSubmittable<
                (
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    fields: PalletIdentityBitFlags
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, PalletIdentityBitFlags]
            >;
            /**
             * Set an account's identity information and reserve the appropriate deposit.
             *
             * If the account already has identity information, the deposit is taken as part payment
             * for the new deposit.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `info`: The identity information.
             *
             * Emits `IdentitySet` if successful.
             *
             * ## Complexity
             * - `O(X + X' + R)`
             * - where `X` additional-field-count (deposit-bounded and code-bounded)
             * - where `R` judgements-count (registrar-count-bounded)
             **/
            setIdentity: AugmentedSubmittable<
                (
                    info:
                        | PalletIdentityIdentityInfo
                        | {
                              additional?: any;
                              display?: any;
                              legal?: any;
                              web?: any;
                              riot?: any;
                              email?: any;
                              pgpFingerprint?: any;
                              image?: any;
                              twitter?: any;
                          }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PalletIdentityIdentityInfo]
            >;
            /**
             * Set the sub-accounts of the sender.
             *
             * Payment: Any aggregate balance reserved by previous `set_subs` calls will be returned
             * and an amount `SubAccountDeposit` will be reserved for each item in `subs`.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * identity.
             *
             * - `subs`: The identity's (new) sub-accounts.
             *
             * ## Complexity
             * - `O(P + S)`
             * - where `P` old-subs-count (hard- and deposit-bounded).
             * - where `S` subs-count (hard- and deposit-bounded).
             **/
            setSubs: AugmentedSubmittable<
                (
                    subs:
                        | Vec<ITuple<[AccountId32, Data]>>
                        | [
                              AccountId32 | string | Uint8Array,
                              (
                                  | Data
                                  | { None: any }
                                  | { Raw: any }
                                  | { BlakeTwo256: any }
                                  | { Sha256: any }
                                  | { Keccak256: any }
                                  | { ShaThree256: any }
                                  | string
                                  | Uint8Array
                              )
                          ][]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<ITuple<[AccountId32, Data]>>]
            >;
        };
        parachainInfo: {};
        parachainStaking: {
            /**
             * add white list of candidates
             * This function should be safe to delete after restriction removed
             **/
            addCandidatesWhitelist: AugmentedSubmittable<
                (candidate: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Cancel pending request to adjust the collator candidate self bond
             **/
            cancelCandidateBondLess: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Cancel request to change an existing delegation.
             **/
            cancelDelegationRequest: AugmentedSubmittable<
                (candidate: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Cancel open request to leave candidates
             * - only callable by collator account
             * - result upon successful call is the candidate is active in the candidate pool
             **/
            cancelLeaveCandidates: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Cancel a pending request to exit the set of delegators. Success clears the pending exit
             * request (thereby resetting the delay upon another `leave_delegators` call).
             **/
            cancelLeaveDelegators: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Increase collator candidate self bond by `more`
             **/
            candidateBondMore: AugmentedSubmittable<
                (more: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u128]
            >;
            /**
             * If caller is not a delegator and not a collator, then join the set of delegators
             * If caller is a delegator, then makes delegation to change their delegation state
             **/
            delegate: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, u128]
            >;
            /**
             * If caller is not a delegator and not a collator, then join the set of delegators
             * If caller is a delegator, then makes delegation to change their delegation state
             * Sets the auto-compound config for the delegation
             **/
            delegateWithAutoCompound: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array,
                    autoCompound: Percent | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, u128, Percent]
            >;
            /**
             * Bond more for delegators wrt a specific collator candidate.
             **/
            delegatorBondMore: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    more: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, u128]
            >;
            /**
             * Execute pending request to adjust the collator candidate self bond
             **/
            executeCandidateBondLess: AugmentedSubmittable<
                (candidate: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Execute pending request to change an existing delegation
             **/
            executeDelegationRequest: AugmentedSubmittable<
                (
                    delegator: AccountId32 | string | Uint8Array,
                    candidate: AccountId32 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, AccountId32]
            >;
            /**
             * Execute leave candidates request
             **/
            executeLeaveCandidates: AugmentedSubmittable<
                (candidate: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Execute the right to exit the set of delegators and revoke all ongoing delegations.
             **/
            executeLeaveDelegators: AugmentedSubmittable<
                (delegator: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Temporarily leave the set of collator candidates without unbonding
             **/
            goOffline: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Rejoin the set of collator candidates if previously had called `go_offline`
             **/
            goOnline: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Join the set of collator candidates
             **/
            joinCandidates: AugmentedSubmittable<
                (bond: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u128]
            >;
            /**
             * remove white list of candidates
             * This function should be safe to delete after restriction removed
             **/
            removeCandidatesWhitelist: AugmentedSubmittable<
                (candidate: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Request by collator candidate to decrease self bond by `less`
             **/
            scheduleCandidateBondLess: AugmentedSubmittable<
                (less: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u128]
            >;
            /**
             * Request bond less for delegators wrt a specific collator candidate.
             **/
            scheduleDelegatorBondLess: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    less: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, u128]
            >;
            /**
             * Request to leave the set of candidates. If successful, the account is immediately
             * removed from the candidate pool to prevent selection as a collator.
             **/
            scheduleLeaveCandidates: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Request to leave the set of delegators. If successful, the caller is scheduled to be
             * allowed to exit via a [DelegationAction::Revoke] towards all existing delegations.
             * Success forbids future delegation requests until the request is invoked or cancelled.
             **/
            scheduleLeaveDelegators: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Request to revoke an existing delegation. If successful, the delegation is scheduled
             * to be allowed to be revoked via the `execute_delegation_request` extrinsic.
             **/
            scheduleRevokeDelegation: AugmentedSubmittable<
                (collator: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Sets the auto-compounding reward percentage for a delegation.
             **/
            setAutoCompound: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    value: Percent | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, Percent]
            >;
            /**
             * Set blocks per round
             * - if called with `new` less than length of current round, will transition immediately
             * in the next block
             * - also updates per-round inflation config
             **/
            setBlocksPerRound: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the commission for all collators
             **/
            setCollatorCommission: AugmentedSubmittable<
                (updated: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Perbill]
            >;
            /**
             * Set the annual inflation rate to derive per-round inflation
             **/
            setInflation: AugmentedSubmittable<
                (
                    schedule:
                        | ({
                              readonly min: Perbill;
                              readonly ideal: Perbill;
                              readonly max: Perbill;
                          } & Struct)
                        | { min?: any; ideal?: any; max?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [
                    {
                        readonly min: Perbill;
                        readonly ideal: Perbill;
                        readonly max: Perbill;
                    } & Struct
                ]
            >;
            /**
             * Set the account that will hold funds set aside for parachain bond
             **/
            setParachainBondAccount: AugmentedSubmittable<
                (updated: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Set the percent of inflation set aside for parachain bond
             **/
            setParachainBondReservePercent: AugmentedSubmittable<
                (updated: Percent | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Percent]
            >;
            /**
             * Set the expectations for total staked. These expectations determine the issuance for
             * the round according to logic in `fn compute_issuance`
             **/
            setStakingExpectations: AugmentedSubmittable<
                (
                    expectations:
                        | ({
                              readonly min: u128;
                              readonly ideal: u128;
                              readonly max: u128;
                          } & Struct)
                        | { min?: any; ideal?: any; max?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [
                    {
                        readonly min: u128;
                        readonly ideal: u128;
                        readonly max: u128;
                    } & Struct
                ]
            >;
            /**
             * Set the total number of collator candidates selected per round
             * - changes are not applied until the start of the next round
             **/
            setTotalSelected: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
        };
        parachainSystem: {
            authorizeUpgrade: AugmentedSubmittable<
                (codeHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            enactAuthorizedUpgrade: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the current validation data.
             *
             * This should be invoked exactly once per block. It will panic at the finalization
             * phase if the call was not invoked.
             *
             * The dispatch origin for this call must be `Inherent`
             *
             * As a side effect, this function upgrades the current validation function
             * if the appropriate time has come.
             **/
            setValidationData: AugmentedSubmittable<
                (
                    data:
                        | CumulusPrimitivesParachainInherentParachainInherentData
                        | {
                              validationData?: any;
                              relayChainState?: any;
                              downwardMessages?: any;
                              horizontalMessages?: any;
                          }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [CumulusPrimitivesParachainInherentParachainInherentData]
            >;
            sudoSendUpwardMessage: AugmentedSubmittable<
                (message: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
        };
        polkadotXcm: {
            /**
             * Execute an XCM message from a local, signed, origin.
             *
             * An event is deposited indicating whether `msg` could be executed completely or only
             * partially.
             *
             * No more than `max_weight` will be used in its attempted execution. If this is less than the
             * maximum amount of weight that the message could take to be executed, then no execution
             * attempt will be made.
             *
             * NOTE: A successful return to this does *not* imply that the `msg` was executed successfully
             * to completion; only that *some* of it was executed.
             **/
            execute: AugmentedSubmittable<
                (
                    message: XcmVersionedXcm | { V2: any } | { V3: any } | string | Uint8Array,
                    maxWeight:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedXcm, SpWeightsWeightV2Weight]
            >;
            /**
             * Set a safe XCM version (the version that XCM should be encoded with if the most recent
             * version a destination can accept is unknown).
             *
             * - `origin`: Must be Root.
             * - `maybe_xcm_version`: The default XCM encoding version, or `None` to disable.
             **/
            forceDefaultXcmVersion: AugmentedSubmittable<
                (
                    maybeXcmVersion: Option<u32> | null | Uint8Array | u32 | AnyNumber
                ) => SubmittableExtrinsic<ApiType>,
                [Option<u32>]
            >;
            /**
             * Ask a location to notify us regarding their XCM version and any changes to it.
             *
             * - `origin`: Must be Root.
             * - `location`: The location to which we should subscribe for XCM version notifications.
             **/
            forceSubscribeVersionNotify: AugmentedSubmittable<
                (
                    location:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation]
            >;
            /**
             * Require that a particular destination should no longer notify us regarding any XCM
             * version changes.
             *
             * - `origin`: Must be Root.
             * - `location`: The location to which we are currently subscribed for XCM version
             * notifications which we no longer desire.
             **/
            forceUnsubscribeVersionNotify: AugmentedSubmittable<
                (
                    location:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation]
            >;
            /**
             * Extoll that a particular destination can be communicated with through a particular
             * version of XCM.
             *
             * - `origin`: Must be Root.
             * - `location`: The destination that is being described.
             * - `xcm_version`: The latest version of XCM that `location` supports.
             **/
            forceXcmVersion: AugmentedSubmittable<
                (
                    location:
                        | XcmV3MultiLocation
                        | { parents?: any; interior?: any }
                        | string
                        | Uint8Array,
                    xcmVersion: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmV3MultiLocation, u32]
            >;
            /**
             * Transfer some assets from the local chain to the sovereign account of a destination
             * chain and forward a notification XCM.
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of
             * index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight
             * is needed than `weight_limit`, then the operation will fail and the assets send may be
             * at risk.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send
             * from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be
             * an `AccountId32` value.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the
             * `dest` side.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
             * fees.
             * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
             **/
            limitedReserveTransferAssets: AugmentedSubmittable<
                (
                    dest:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    beneficiary:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    assets:
                        | XcmVersionedMultiAssets
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array,
                    weightLimit:
                        | XcmV3WeightLimit
                        | { Unlimited: any }
                        | { Limited: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [
                    XcmVersionedMultiLocation,
                    XcmVersionedMultiLocation,
                    XcmVersionedMultiAssets,
                    u32,
                    XcmV3WeightLimit
                ]
            >;
            /**
             * Teleport some assets from the local chain to some destination chain.
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of
             * index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight
             * is needed than `weight_limit`, then the operation will fail and the assets send may be
             * at risk.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send
             * from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be
             * an `AccountId32` value.
             * - `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the
             * `dest` side. May not be empty.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
             * fees.
             * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
             **/
            limitedTeleportAssets: AugmentedSubmittable<
                (
                    dest:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    beneficiary:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    assets:
                        | XcmVersionedMultiAssets
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array,
                    weightLimit:
                        | XcmV3WeightLimit
                        | { Unlimited: any }
                        | { Limited: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [
                    XcmVersionedMultiLocation,
                    XcmVersionedMultiLocation,
                    XcmVersionedMultiAssets,
                    u32,
                    XcmV3WeightLimit
                ]
            >;
            /**
             * Transfer some assets from the local chain to the sovereign account of a destination
             * chain and forward a notification XCM.
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of
             * index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,
             * with all fees taken as needed from the asset.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send
             * from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be
             * an `AccountId32` value.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the
             * `dest` side.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
             * fees.
             **/
            reserveTransferAssets: AugmentedSubmittable<
                (
                    dest:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    beneficiary:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    assets:
                        | XcmVersionedMultiAssets
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32]
            >;
            send: AugmentedSubmittable<
                (
                    dest:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    message: XcmVersionedXcm | { V2: any } | { V3: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation, XcmVersionedXcm]
            >;
            /**
             * Teleport some assets from the local chain to some destination chain.
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of
             * index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,
             * with all fees taken as needed from the asset.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send
             * from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be
             * an `AccountId32` value.
             * - `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the
             * `dest` side. May not be empty.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
             * fees.
             **/
            teleportAssets: AugmentedSubmittable<
                (
                    dest:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    beneficiary:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    assets:
                        | XcmVersionedMultiAssets
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32]
            >;
        };
        preimage: {
            /**
             * Register a preimage on-chain.
             *
             * If the preimage was previously requested, no fees or deposits are taken for providing
             * the preimage. Otherwise, a deposit is taken proportional to the size of the preimage.
             **/
            notePreimage: AugmentedSubmittable<
                (bytes: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Request a preimage be uploaded to the chain without paying any fees or deposits.
             *
             * If the preimage requests has already been provided on-chain, we unreserve any deposit
             * a user may have paid, and take the control of the preimage out of their hands.
             **/
            requestPreimage: AugmentedSubmittable<
                (hash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Clear an unrequested preimage from the runtime storage.
             *
             * If `len` is provided, then it will be a much cheaper operation.
             *
             * - `hash`: The hash of the preimage to be removed from the store.
             * - `len`: The length of the preimage of `hash`.
             **/
            unnotePreimage: AugmentedSubmittable<
                (hash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Clear a previously made request for a preimage.
             *
             * NOTE: THIS MUST NOT BE CALLED ON `hash` MORE TIMES THAN `request_preimage`.
             **/
            unrequestPreimage: AugmentedSubmittable<
                (hash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
        };
        proxy: {
            /**
             * Register a proxy account for the sender that is able to make calls on its behalf.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `proxy`: The account that the `caller` would like to make a proxy.
             * - `proxy_type`: The permissions allowed for this proxy account.
             * - `delay`: The announcement period required of the initial proxy. Will generally be
             * zero.
             **/
            addProxy: AugmentedSubmittable<
                (
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    proxyType:
                        | RococoParachainRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "CancelProxy"
                        | "Collator"
                        | "Governance"
                        | number
                        | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, RococoParachainRuntimeProxyType, u32]
            >;
            /**
             * Publish the hash of a proxy-call that will be made in the future.
             *
             * This must be called some number of blocks before the corresponding `proxy` is attempted
             * if the delay associated with the proxy relationship is greater than zero.
             *
             * No more than `MaxPending` announcements may be made at any one time.
             *
             * This will take a deposit of `AnnouncementDepositFactor` as well as
             * `AnnouncementDepositBase` if there are no other pending announcements.
             *
             * The dispatch origin for this call must be _Signed_ and a proxy of `real`.
             *
             * Parameters:
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `call_hash`: The hash of the call to be made by the `real` account.
             **/
            announce: AugmentedSubmittable<
                (
                    real:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    callHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, H256]
            >;
            /**
             * Spawn a fresh new account that is guaranteed to be otherwise inaccessible, and
             * initialize it with a proxy of `proxy_type` for `origin` sender.
             *
             * Requires a `Signed` origin.
             *
             * - `proxy_type`: The type of the proxy that the sender will be registered as over the
             * new account. This will almost always be the most permissive `ProxyType` possible to
             * allow for maximum flexibility.
             * - `index`: A disambiguation index, in case this is called multiple times in the same
             * transaction (e.g. with `utility::batch`). Unless you're using `batch` you probably just
             * want to use `0`.
             * - `delay`: The announcement period required of the initial proxy. Will generally be
             * zero.
             *
             * Fails with `Duplicate` if this has already been called in this transaction, from the
             * same sender, with the same parameters.
             *
             * Fails if there are insufficient funds to pay for deposit.
             **/
            createPure: AugmentedSubmittable<
                (
                    proxyType:
                        | RococoParachainRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "CancelProxy"
                        | "Collator"
                        | "Governance"
                        | number
                        | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array,
                    index: u16 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [RococoParachainRuntimeProxyType, u32, u16]
            >;
            /**
             * Removes a previously spawned pure proxy.
             *
             * WARNING: **All access to this account will be lost.** Any funds held in it will be
             * inaccessible.
             *
             * Requires a `Signed` origin, and the sender account must have been created by a call to
             * `pure` with corresponding parameters.
             *
             * - `spawner`: The account that originally called `pure` to create this account.
             * - `index`: The disambiguation index originally passed to `pure`. Probably `0`.
             * - `proxy_type`: The proxy type originally passed to `pure`.
             * - `height`: The height of the chain when the call to `pure` was processed.
             * - `ext_index`: The extrinsic index in which the call to `pure` was processed.
             *
             * Fails with `NoPermission` in case the caller is not a previously created pure
             * account whose `pure` call has corresponding parameters.
             **/
            killPure: AugmentedSubmittable<
                (
                    spawner:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    proxyType:
                        | RococoParachainRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "CancelProxy"
                        | "Collator"
                        | "Governance"
                        | number
                        | Uint8Array,
                    index: u16 | AnyNumber | Uint8Array,
                    height: Compact<u32> | AnyNumber | Uint8Array,
                    extIndex: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, RococoParachainRuntimeProxyType, u16, Compact<u32>, Compact<u32>]
            >;
            /**
             * Dispatch the given `call` from an account that the sender is authorised for through
             * `add_proxy`.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `force_proxy_type`: Specify the exact proxy type to be used and checked for this call.
             * - `call`: The call to be made by the `real` account.
             **/
            proxy: AugmentedSubmittable<
                (
                    real:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    forceProxyType:
                        | Option<RococoParachainRuntimeProxyType>
                        | null
                        | Uint8Array
                        | RococoParachainRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "CancelProxy"
                        | "Collator"
                        | "Governance"
                        | number,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Option<RococoParachainRuntimeProxyType>, Call]
            >;
            /**
             * Dispatch the given `call` from an account that the sender is authorized for through
             * `add_proxy`.
             *
             * Removes any corresponding announcement(s).
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `force_proxy_type`: Specify the exact proxy type to be used and checked for this call.
             * - `call`: The call to be made by the `real` account.
             **/
            proxyAnnounced: AugmentedSubmittable<
                (
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    real:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    forceProxyType:
                        | Option<RococoParachainRuntimeProxyType>
                        | null
                        | Uint8Array
                        | RococoParachainRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "CancelProxy"
                        | "Collator"
                        | "Governance"
                        | number,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress, Option<RococoParachainRuntimeProxyType>, Call]
            >;
            /**
             * Remove the given announcement of a delegate.
             *
             * May be called by a target (proxied) account to remove a call that one of their delegates
             * (`delegate`) has announced they want to execute. The deposit is returned.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `delegate`: The account that previously announced the call.
             * - `call_hash`: The hash of the call to be made.
             **/
            rejectAnnouncement: AugmentedSubmittable<
                (
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    callHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, H256]
            >;
            /**
             * Remove a given announcement.
             *
             * May be called by a proxy account to remove a call they previously announced and return
             * the deposit.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `call_hash`: The hash of the call to be made by the `real` account.
             **/
            removeAnnouncement: AugmentedSubmittable<
                (
                    real:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    callHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, H256]
            >;
            /**
             * Unregister all proxy accounts for the sender.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * WARNING: This may be called on accounts created by `pure`, however if done, then
             * the unreserved fees will be inaccessible. **All access to this account will be lost.**
             **/
            removeProxies: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Unregister a proxy account for the sender.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `proxy`: The account that the `caller` would like to remove as a proxy.
             * - `proxy_type`: The permissions currently enabled for the removed proxy account.
             **/
            removeProxy: AugmentedSubmittable<
                (
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    proxyType:
                        | RococoParachainRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "CancelProxy"
                        | "Collator"
                        | "Governance"
                        | number
                        | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, RococoParachainRuntimeProxyType, u32]
            >;
        };
        scheduler: {
            /**
             * Cancel an anonymously scheduled task.
             **/
            cancel: AugmentedSubmittable<
                (
                    when: u32 | AnyNumber | Uint8Array,
                    index: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Cancel a named scheduled task.
             **/
            cancelNamed: AugmentedSubmittable<
                (id: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [U8aFixed]
            >;
            /**
             * Anonymously schedule a task.
             **/
            schedule: AugmentedSubmittable<
                (
                    when: u32 | AnyNumber | Uint8Array,
                    maybePeriodic:
                        | Option<ITuple<[u32, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u32, u32]>
                        | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    priority: u8 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<ITuple<[u32, u32]>>, u8, Call]
            >;
            /**
             * Anonymously schedule a task after a delay.
             **/
            scheduleAfter: AugmentedSubmittable<
                (
                    after: u32 | AnyNumber | Uint8Array,
                    maybePeriodic:
                        | Option<ITuple<[u32, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u32, u32]>
                        | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    priority: u8 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<ITuple<[u32, u32]>>, u8, Call]
            >;
            /**
             * Schedule a named task.
             **/
            scheduleNamed: AugmentedSubmittable<
                (
                    id: U8aFixed | string | Uint8Array,
                    when: u32 | AnyNumber | Uint8Array,
                    maybePeriodic:
                        | Option<ITuple<[u32, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u32, u32]>
                        | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    priority: u8 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [U8aFixed, u32, Option<ITuple<[u32, u32]>>, u8, Call]
            >;
            /**
             * Schedule a named task after a delay.
             **/
            scheduleNamedAfter: AugmentedSubmittable<
                (
                    id: U8aFixed | string | Uint8Array,
                    after: u32 | AnyNumber | Uint8Array,
                    maybePeriodic:
                        | Option<ITuple<[u32, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u32, u32]>
                        | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    priority: u8 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [U8aFixed, u32, Option<ITuple<[u32, u32]>>, u8, Call]
            >;
        };
        session: {
            /**
             * Removes any session key(s) of the function caller.
             *
             * This doesn't take effect until the next session.
             *
             * The dispatch origin of this function must be Signed and the account must be either be
             * convertible to a validator ID using the chain's typical addressing system (this usually
             * means being a controller account) or directly convertible into a validator ID (which
             * usually means being a stash account).
             *
             * ## Complexity
             * - `O(1)` in number of key types. Actual cost depends on the number of length of
             * `T::Keys::key_ids()` which is fixed.
             **/
            purgeKeys: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Sets the session key(s) of the function caller to `keys`.
             * Allows an account to set its session key prior to becoming a validator.
             * This doesn't take effect until the next session.
             *
             * The dispatch origin of this function must be signed.
             *
             * ## Complexity
             * - `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is
             * fixed.
             **/
            setKeys: AugmentedSubmittable<
                (
                    keys: RococoParachainRuntimeSessionKeys | { aura?: any } | string | Uint8Array,
                    proof: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [RococoParachainRuntimeSessionKeys, Bytes]
            >;
        };
        sidechain: {
            /**
             * The integritee worker calls this function for every imported sidechain_block.
             **/
            confirmImportedSidechainBlock: AugmentedSubmittable<
                (
                    shardId: H256 | string | Uint8Array,
                    blockNumber: u64 | AnyNumber | Uint8Array,
                    nextFinalizationCandidateBlockNumber: u64 | AnyNumber | Uint8Array,
                    blockHeaderHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, u64, u64, H256]
            >;
        };
        sudo: {
            /**
             * Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo
             * key.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * ## Complexity
             * - O(1).
             **/
            setKey: AugmentedSubmittable<
                (
                    updated:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Authenticates the sudo key and dispatches a function call with `Root` origin.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * ## Complexity
             * - O(1).
             **/
            sudo: AugmentedSubmittable<
                (call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Call]
            >;
            /**
             * Authenticates the sudo key and dispatches a function call with `Signed` origin from
             * a given account.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * ## Complexity
             * - O(1).
             **/
            sudoAs: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Call]
            >;
            /**
             * Authenticates the sudo key and dispatches a function call with `Root` origin.
             * This function does not check the weight of the call, and instead allows the
             * Sudo user to specify the weight of the call.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * ## Complexity
             * - O(1).
             **/
            sudoUncheckedWeight: AugmentedSubmittable<
                (
                    call: Call | IMethod | string | Uint8Array,
                    weight:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Call, SpWeightsWeightV2Weight]
            >;
        };
        system: {
            /**
             * Kill all storage items with a key that starts with the given prefix.
             *
             * **NOTE:** We rely on the Root origin to provide us the number of subkeys under
             * the prefix we are removing to accurately calculate the weight of this function.
             **/
            killPrefix: AugmentedSubmittable<
                (
                    prefix: Bytes | string | Uint8Array,
                    subkeys: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, u32]
            >;
            /**
             * Kill some items from storage.
             **/
            killStorage: AugmentedSubmittable<
                (
                    keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<Bytes>]
            >;
            /**
             * Make some on-chain remark.
             *
             * ## Complexity
             * - `O(1)`
             **/
            remark: AugmentedSubmittable<
                (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Make some on-chain remark and emit event.
             **/
            remarkWithEvent: AugmentedSubmittable<
                (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the new runtime code.
             *
             * ## Complexity
             * - `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`
             **/
            setCode: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the new runtime code without doing any checks of the given `code`.
             *
             * ## Complexity
             * - `O(C)` where `C` length of `code`
             **/
            setCodeWithoutChecks: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the number of pages in the WebAssembly environment's heap.
             **/
            setHeapPages: AugmentedSubmittable<
                (pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Set some items of storage.
             **/
            setStorage: AugmentedSubmittable<
                (
                    items:
                        | Vec<ITuple<[Bytes, Bytes]>>
                        | [Bytes | string | Uint8Array, Bytes | string | Uint8Array][]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<ITuple<[Bytes, Bytes]>>]
            >;
        };
        technicalCommittee: {
            /**
             * Close a vote that is either approved, disapproved or whose voting period has ended.
             *
             * May be called by any signed account in order to finish voting and close the proposal.
             *
             * If called before the end of the voting period it will only close the vote if it is
             * has enough votes to be approved or disapproved.
             *
             * If called after the end of the voting period abstentions are counted as rejections
             * unless there is a prime member set and the prime member cast an approval.
             *
             * If the close operation completes successfully with disapproval, the transaction fee will
             * be waived. Otherwise execution of the approved operation will be charged to the caller.
             *
             * + `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed
             * proposal.
             * + `length_bound`: The upper bound for the length of the proposal in storage. Checked via
             * `storage::read` so it is `size_of::<u32>() == 4` larger than the pure length.
             *
             * ## Complexity
             * - `O(B + M + P1 + P2)` where:
             * - `B` is `proposal` size in bytes (length-fee-bounded)
             * - `M` is members-count (code- and governance-bounded)
             * - `P1` is the complexity of `proposal` preimage.
             * - `P2` is proposal-count (code-bounded)
             **/
            close: AugmentedSubmittable<
                (
                    proposalHash: H256 | string | Uint8Array,
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    proposalWeightBound:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array,
                    lengthBound: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Compact<u32>, SpWeightsWeightV2Weight, Compact<u32>]
            >;
            /**
             * Close a vote that is either approved, disapproved or whose voting period has ended.
             *
             * May be called by any signed account in order to finish voting and close the proposal.
             *
             * If called before the end of the voting period it will only close the vote if it is
             * has enough votes to be approved or disapproved.
             *
             * If called after the end of the voting period abstentions are counted as rejections
             * unless there is a prime member set and the prime member cast an approval.
             *
             * If the close operation completes successfully with disapproval, the transaction fee will
             * be waived. Otherwise execution of the approved operation will be charged to the caller.
             *
             * + `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed
             * proposal.
             * + `length_bound`: The upper bound for the length of the proposal in storage. Checked via
             * `storage::read` so it is `size_of::<u32>() == 4` larger than the pure length.
             *
             * ## Complexity
             * - `O(B + M + P1 + P2)` where:
             * - `B` is `proposal` size in bytes (length-fee-bounded)
             * - `M` is members-count (code- and governance-bounded)
             * - `P1` is the complexity of `proposal` preimage.
             * - `P2` is proposal-count (code-bounded)
             **/
            closeOldWeight: AugmentedSubmittable<
                (
                    proposalHash: H256 | string | Uint8Array,
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    proposalWeightBound: Compact<u64> | AnyNumber | Uint8Array,
                    lengthBound: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Compact<u32>, Compact<u64>, Compact<u32>]
            >;
            /**
             * Disapprove a proposal, close, and remove it from the system, regardless of its current
             * state.
             *
             * Must be called by the Root origin.
             *
             * Parameters:
             * * `proposal_hash`: The hash of the proposal that should be disapproved.
             *
             * ## Complexity
             * O(P) where P is the number of max proposals
             **/
            disapproveProposal: AugmentedSubmittable<
                (proposalHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Dispatch a proposal from a member using the `Member` origin.
             *
             * Origin must be a member of the collective.
             *
             * ## Complexity:
             * - `O(B + M + P)` where:
             * - `B` is `proposal` size in bytes (length-fee-bounded)
             * - `M` members-count (code-bounded)
             * - `P` complexity of dispatching `proposal`
             **/
            execute: AugmentedSubmittable<
                (
                    proposal: Call | IMethod | string | Uint8Array,
                    lengthBound: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Call, Compact<u32>]
            >;
            /**
             * Add a new proposal to either be voted on or executed directly.
             *
             * Requires the sender to be member.
             *
             * `threshold` determines whether `proposal` is executed directly (`threshold < 2`)
             * or put up for voting.
             *
             * ## Complexity
             * - `O(B + M + P1)` or `O(B + M + P2)` where:
             * - `B` is `proposal` size in bytes (length-fee-bounded)
             * - `M` is members-count (code- and governance-bounded)
             * - branching is influenced by `threshold` where:
             * - `P1` is proposal execution complexity (`threshold < 2`)
             * - `P2` is proposals-count (code-bounded) (`threshold >= 2`)
             **/
            propose: AugmentedSubmittable<
                (
                    threshold: Compact<u32> | AnyNumber | Uint8Array,
                    proposal: Call | IMethod | string | Uint8Array,
                    lengthBound: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, Call, Compact<u32>]
            >;
            /**
             * Set the collective's membership.
             *
             * - `new_members`: The new member list. Be nice to the chain and provide it sorted.
             * - `prime`: The prime member whose vote sets the default.
             * - `old_count`: The upper bound for the previous number of members in storage. Used for
             * weight estimation.
             *
             * The dispatch of this call must be `SetMembersOrigin`.
             *
             * NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but
             * the weight estimations rely on it to estimate dispatchable weight.
             *
             * # WARNING:
             *
             * The `pallet-collective` can also be managed by logic outside of the pallet through the
             * implementation of the trait [`ChangeMembers`].
             * Any call to `set_members` must be careful that the member set doesn't get out of sync
             * with other logic managing the member set.
             *
             * ## Complexity:
             * - `O(MP + N)` where:
             * - `M` old-members-count (code- and governance-bounded)
             * - `N` new-members-count (code- and governance-bounded)
             * - `P` proposals-count (code-bounded)
             **/
            setMembers: AugmentedSubmittable<
                (
                    newMembers: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    prime: Option<AccountId32> | null | Uint8Array | AccountId32 | string,
                    oldCount: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>, Option<AccountId32>, u32]
            >;
            /**
             * Add an aye or nay vote for the sender to the given proposal.
             *
             * Requires the sender to be a member.
             *
             * Transaction fees will be waived if the member is voting on any particular proposal
             * for the first time and the call is successful. Subsequent vote changes will charge a
             * fee.
             * ## Complexity
             * - `O(M)` where `M` is members-count (code- and governance-bounded)
             **/
            vote: AugmentedSubmittable<
                (
                    proposal: H256 | string | Uint8Array,
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    approve: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Compact<u32>, bool]
            >;
        };
        technicalCommitteeMembership: {
            /**
             * Add a member `who` to the set.
             *
             * May only be called from `T::AddOrigin`.
             **/
            addMember: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Swap out the sending member for some other key `new`.
             *
             * May only be called from `Signed` origin of a current member.
             *
             * Prime membership is passed from the origin account to `new`, if extant.
             **/
            changeKey: AugmentedSubmittable<
                (
                    updated:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Remove the prime member if it exists.
             *
             * May only be called from `T::PrimeOrigin`.
             **/
            clearPrime: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Remove a member `who` from the set.
             *
             * May only be called from `T::RemoveOrigin`.
             **/
            removeMember: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Change the membership to a new set, disregarding the existing membership. Be nice and
             * pass `members` pre-sorted.
             *
             * May only be called from `T::ResetOrigin`.
             **/
            resetMembers: AugmentedSubmittable<
                (
                    members: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /**
             * Set the prime member. Must be a current member.
             *
             * May only be called from `T::PrimeOrigin`.
             **/
            setPrime: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Swap out one member `remove` for another `add`.
             *
             * May only be called from `T::SwapOrigin`.
             *
             * Prime membership is *not* passed from `remove` to `add`, if extant.
             **/
            swapMember: AugmentedSubmittable<
                (
                    remove:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    add:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress]
            >;
        };
        teeracle: {
            addToWhitelist: AugmentedSubmittable<
                (
                    dataSource: Bytes | string | Uint8Array,
                    mrenclave: U8aFixed | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, U8aFixed]
            >;
            removeFromWhitelist: AugmentedSubmittable<
                (
                    dataSource: Bytes | string | Uint8Array,
                    mrenclave: U8aFixed | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, U8aFixed]
            >;
            updateExchangeRate: AugmentedSubmittable<
                (
                    dataSource: Bytes | string | Uint8Array,
                    tradingPair: Bytes | string | Uint8Array,
                    newValue:
                        | Option<SubstrateFixedFixedU64>
                        | null
                        | Uint8Array
                        | SubstrateFixedFixedU64
                        | { bits?: any }
                        | string
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, Bytes, Option<SubstrateFixedFixedU64>]
            >;
            updateOracle: AugmentedSubmittable<
                (
                    oracleName: Bytes | string | Uint8Array,
                    dataSource: Bytes | string | Uint8Array,
                    newBlob: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, Bytes, Bytes]
            >;
        };
        teerex: {
            callWorker: AugmentedSubmittable<
                (
                    request:
                        | TeerexPrimitivesRequest
                        | { shard?: any; cyphertext?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [TeerexPrimitivesRequest]
            >;
            /**
             * The integritee worker calls this function for every processed parentchain_block to
             * confirm a state update.
             **/
            confirmProcessedParentchainBlock: AugmentedSubmittable<
                (
                    blockHash: H256 | string | Uint8Array,
                    blockNumber: Compact<u32> | AnyNumber | Uint8Array,
                    trustedCallsMerkleRoot: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Compact<u32>, H256]
            >;
            /**
             * Publish a hash as a result of an arbitrary enclave operation.
             *
             * The `mrenclave` of the origin will be used as an event topic a client can subscribe to.
             * `extra_topics`, if any, will be used as additional event topics.
             *
             * `data` can be anything worthwhile publishing related to the hash. If it is a
             * utf8-encoded string, the UIs will usually even render the text.
             **/
            publishHash: AugmentedSubmittable<
                (
                    hash: H256 | string | Uint8Array,
                    extraTopics: Vec<H256> | (H256 | string | Uint8Array)[],
                    data: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Vec<H256>, Bytes]
            >;
            registerDcapEnclave: AugmentedSubmittable<
                (
                    dcapQuote: Bytes | string | Uint8Array,
                    workerUrl: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, Bytes]
            >;
            registerEnclave: AugmentedSubmittable<
                (
                    raReport: Bytes | string | Uint8Array,
                    workerUrl: Bytes | string | Uint8Array,
                    shieldingKey: Option<Bytes> | null | Uint8Array | Bytes | string,
                    vcPubkey: Option<Bytes> | null | Uint8Array | Bytes | string
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, Bytes, Option<Bytes>, Option<Bytes>]
            >;
            registerQuotingEnclave: AugmentedSubmittable<
                (
                    enclaveIdentity: Bytes | string | Uint8Array,
                    signature: Bytes | string | Uint8Array,
                    certificateChain: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, Bytes, Bytes]
            >;
            registerTcbInfo: AugmentedSubmittable<
                (
                    tcbInfo: Bytes | string | Uint8Array,
                    signature: Bytes | string | Uint8Array,
                    certificateChain: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, Bytes, Bytes]
            >;
            removeScheduledEnclave: AugmentedSubmittable<
                (
                    sidechainBlockNumber: u64 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Change the admin account
             * similar to sudo.set_key, the old account will be supplied in event
             **/
            setAdmin: AugmentedSubmittable<
                (updated: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            setHeartbeatTimeout: AugmentedSubmittable<
                (timeout: Compact<u64> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u64>]
            >;
            /**
             * Sent by a client who requests to get shielded funds managed by an enclave. For this
             * on-chain balance is sent to the bonding_account of the enclave. The bonding_account does
             * not have a private key as the balance on this account is exclusively managed from
             * withing the pallet_teerex. Note: The bonding_account is bit-equivalent to the worker
             * shard.
             **/
            shieldFunds: AugmentedSubmittable<
                (
                    incognitoAccountEncrypted: Bytes | string | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array,
                    bondingAccount: AccountId32 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, u128, AccountId32]
            >;
            unregisterEnclave: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Sent by enclaves only as a result of an `unshield` request from a client to an enclave.
             **/
            unshieldFunds: AugmentedSubmittable<
                (
                    publicAccount: AccountId32 | string | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array,
                    bondingAccount: AccountId32 | string | Uint8Array,
                    callHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, u128, AccountId32, H256]
            >;
            updateScheduledEnclave: AugmentedSubmittable<
                (
                    sidechainBlockNumber: Compact<u64> | AnyNumber | Uint8Array,
                    mrEnclave: U8aFixed | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u64>, U8aFixed]
            >;
        };
        timestamp: {
            /**
             * Set the current time.
             *
             * This call should be invoked exactly once per block. It will panic at the finalization
             * phase, if this call hasn't been invoked by that time.
             *
             * The timestamp should be greater than the previous one by the amount specified by
             * `MinimumPeriod`.
             *
             * The dispatch origin for this call must be `Inherent`.
             *
             * ## Complexity
             * - `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)
             * - 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in
             * `on_finalize`)
             * - 1 event handler `on_timestamp_set`. Must be `O(1)`.
             **/
            set: AugmentedSubmittable<
                (now: Compact<u64> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u64>]
            >;
        };
        tips: {
            /**
             * Close and payout a tip.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * The tip identified by `hash` must have finished its countdown period.
             *
             * - `hash`: The identity of the open tip for which a tip value is declared. This is formed
             * as the hash of the tuple of the original tip `reason` and the beneficiary account ID.
             *
             * ## Complexity
             * - : `O(T)` where `T` is the number of tippers. decoding `Tipper` vec of length `T`. `T`
             * is charged as upper bound given by `ContainsLengthBound`. The actual cost depends on
             * the implementation of `T::Tippers`.
             **/
            closeTip: AugmentedSubmittable<
                (hash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Report something `reason` that deserves a tip and claim any eventual the finder's fee.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Payment: `TipReportDepositBase` will be reserved from the origin account, as well as
             * `DataDepositPerByte` for each byte in `reason`.
             *
             * - `reason`: The reason for, or the thing that deserves, the tip; generally this will be
             * a UTF-8-encoded URL.
             * - `who`: The account which should be credited for the tip.
             *
             * Emits `NewTip` if successful.
             *
             * ## Complexity
             * - `O(R)` where `R` length of `reason`.
             * - encoding and hashing of 'reason'
             **/
            reportAwesome: AugmentedSubmittable<
                (
                    reason: Bytes | string | Uint8Array,
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, MultiAddress]
            >;
            /**
             * Retract a prior tip-report from `report_awesome`, and cancel the process of tipping.
             *
             * If successful, the original deposit will be unreserved.
             *
             * The dispatch origin for this call must be _Signed_ and the tip identified by `hash`
             * must have been reported by the signing account through `report_awesome` (and not
             * through `tip_new`).
             *
             * - `hash`: The identity of the open tip for which a tip value is declared. This is formed
             * as the hash of the tuple of the original tip `reason` and the beneficiary account ID.
             *
             * Emits `TipRetracted` if successful.
             *
             * ## Complexity
             * - `O(1)`
             * - Depends on the length of `T::Hash` which is fixed.
             **/
            retractTip: AugmentedSubmittable<
                (hash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Remove and slash an already-open tip.
             *
             * May only be called from `T::RejectOrigin`.
             *
             * As a result, the finder is slashed and the deposits are lost.
             *
             * Emits `TipSlashed` if successful.
             *
             * ## Complexity
             * - O(1).
             **/
            slashTip: AugmentedSubmittable<
                (hash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Declare a tip value for an already-open tip.
             *
             * The dispatch origin for this call must be _Signed_ and the signing account must be a
             * member of the `Tippers` set.
             *
             * - `hash`: The identity of the open tip for which a tip value is declared. This is formed
             * as the hash of the tuple of the hash of the original tip `reason` and the beneficiary
             * account ID.
             * - `tip_value`: The amount of tip that the sender would like to give. The median tip
             * value of active tippers will be given to the `who`.
             *
             * Emits `TipClosing` if the threshold of tippers has been reached and the countdown period
             * has started.
             *
             * ## Complexity
             * - `O(T)` where `T` is the number of tippers. decoding `Tipper` vec of length `T`, insert
             * tip and check closing, `T` is charged as upper bound given by `ContainsLengthBound`.
             * The actual cost depends on the implementation of `T::Tippers`.
             *
             * Actually weight could be lower as it depends on how many tips are in `OpenTip` but it
             * is weighted as if almost full i.e of length `T-1`.
             **/
            tip: AugmentedSubmittable<
                (
                    hash: H256 | string | Uint8Array,
                    tipValue: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Compact<u128>]
            >;
            /**
             * Give a tip for something new; no finder's fee will be taken.
             *
             * The dispatch origin for this call must be _Signed_ and the signing account must be a
             * member of the `Tippers` set.
             *
             * - `reason`: The reason for, or the thing that deserves, the tip; generally this will be
             * a UTF-8-encoded URL.
             * - `who`: The account which should be credited for the tip.
             * - `tip_value`: The amount of tip that the sender would like to give. The median tip
             * value of active tippers will be given to the `who`.
             *
             * Emits `NewTip` if successful.
             *
             * ## Complexity
             * - `O(R + T)` where `R` length of `reason`, `T` is the number of tippers.
             * - `O(T)`: decoding `Tipper` vec of length `T`. `T` is charged as upper bound given by
             * `ContainsLengthBound`. The actual cost depends on the implementation of
             * `T::Tippers`.
             * - `O(R)`: hashing and encoding of reason of length `R`
             **/
            tipNew: AugmentedSubmittable<
                (
                    reason: Bytes | string | Uint8Array,
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    tipValue: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, MultiAddress, Compact<u128>]
            >;
        };
        tokens: {
            /**
             * Exactly as `transfer`, except the origin must be root and the source
             * account may be specified.
             *
             * The dispatch origin for this call must be _Root_.
             *
             * - `source`: The sender of the transfer.
             * - `dest`: The recipient of the transfer.
             * - `currency_id`: currency type.
             * - `amount`: free balance amount to tranfer.
             **/
            forceTransfer: AugmentedSubmittable<
                (
                    source:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    currencyId: u128 | AnyNumber | Uint8Array,
                    amount: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress, u128, Compact<u128>]
            >;
            /**
             * Set the balances of a given account.
             *
             * This will alter `FreeBalance` and `ReservedBalance` in storage. it
             * will also decrease the total issuance of the system
             * (`TotalIssuance`). If the new free or reserved balance is below the
             * existential deposit, it will reap the `AccountInfo`.
             *
             * The dispatch origin for this call is `root`.
             **/
            setBalance: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    currencyId: u128 | AnyNumber | Uint8Array,
                    newFree: Compact<u128> | AnyNumber | Uint8Array,
                    newReserved: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, u128, Compact<u128>, Compact<u128>]
            >;
            /**
             * Transfer some liquid free balance to another account.
             *
             * `transfer` will set the `FreeBalance` of the sender and receiver.
             * It will decrease the total issuance of the system by the
             * `TransferFee`. If the sender's account is below the existential
             * deposit as a result of the transfer, the account will be reaped.
             *
             * The dispatch origin for this call must be `Signed` by the
             * transactor.
             *
             * - `dest`: The recipient of the transfer.
             * - `currency_id`: currency type.
             * - `amount`: free balance amount to tranfer.
             **/
            transfer: AugmentedSubmittable<
                (
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    currencyId: u128 | AnyNumber | Uint8Array,
                    amount: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, u128, Compact<u128>]
            >;
            /**
             * Transfer all remaining balance to the given account.
             *
             * NOTE: This function only attempts to transfer _transferable_
             * balances. This means that any locked, reserved, or existential
             * deposits (when `keep_alive` is `true`), will not be transferred by
             * this function. To ensure that this function results in a killed
             * account, you might need to prepare the account by removing any
             * reference counters, storage deposits, etc...
             *
             * The dispatch origin for this call must be `Signed` by the
             * transactor.
             *
             * - `dest`: The recipient of the transfer.
             * - `currency_id`: currency type.
             * - `keep_alive`: A boolean to determine if the `transfer_all`
             * operation should send all of the funds the account has, causing
             * the sender account to be killed (false), or transfer everything
             * except at least the existential deposit, which will guarantee to
             * keep the sender account alive (true).
             **/
            transferAll: AugmentedSubmittable<
                (
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    currencyId: u128 | AnyNumber | Uint8Array,
                    keepAlive: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, u128, bool]
            >;
            /**
             * Same as the [`transfer`] call, but with a check that the transfer
             * will not kill the origin account.
             *
             * 99% of the time you want [`transfer`] instead.
             *
             * The dispatch origin for this call must be `Signed` by the
             * transactor.
             *
             * - `dest`: The recipient of the transfer.
             * - `currency_id`: currency type.
             * - `amount`: free balance amount to tranfer.
             **/
            transferKeepAlive: AugmentedSubmittable<
                (
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    currencyId: u128 | AnyNumber | Uint8Array,
                    amount: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, u128, Compact<u128>]
            >;
        };
        treasury: {
            /**
             * Approve a proposal. At a later time, the proposal will be allocated to the beneficiary
             * and the original deposit will be returned.
             *
             * May only be called from `T::ApproveOrigin`.
             *
             * ## Complexity
             * - O(1).
             **/
            approveProposal: AugmentedSubmittable<
                (
                    proposalId: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Put forward a suggestion for spending. A deposit proportional to the value
             * is reserved and slashed if the proposal is rejected. It is returned once the
             * proposal is awarded.
             *
             * ## Complexity
             * - O(1)
             **/
            proposeSpend: AugmentedSubmittable<
                (
                    value: Compact<u128> | AnyNumber | Uint8Array,
                    beneficiary:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u128>, MultiAddress]
            >;
            /**
             * Reject a proposed spend. The original deposit will be slashed.
             *
             * May only be called from `T::RejectOrigin`.
             *
             * ## Complexity
             * - O(1)
             **/
            rejectProposal: AugmentedSubmittable<
                (
                    proposalId: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Force a previously approved proposal to be removed from the approval queue.
             * The original deposit will no longer be returned.
             *
             * May only be called from `T::RejectOrigin`.
             * - `proposal_id`: The index of a proposal
             *
             * ## Complexity
             * - O(A) where `A` is the number of approvals
             *
             * Errors:
             * - `ProposalNotApproved`: The `proposal_id` supplied was not found in the approval queue,
             * i.e., the proposal has not been approved. This could also mean the proposal does not
             * exist altogether, thus there is no way it would have been approved in the first place.
             **/
            removeApproval: AugmentedSubmittable<
                (
                    proposalId: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Propose and approve a spend of treasury funds.
             *
             * - `origin`: Must be `SpendOrigin` with the `Success` value being at least `amount`.
             * - `amount`: The amount to be transferred from the treasury to the `beneficiary`.
             * - `beneficiary`: The destination account for the transfer.
             *
             * NOTE: For record-keeping purposes, the proposer is deemed to be equivalent to the
             * beneficiary.
             **/
            spend: AugmentedSubmittable<
                (
                    amount: Compact<u128> | AnyNumber | Uint8Array,
                    beneficiary:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u128>, MultiAddress]
            >;
        };
        utility: {
            /**
             * Send a call through an indexed pseudonym of the sender.
             *
             * Filter from origin are passed along. The call will be dispatched with an origin which
             * use the same filter as the origin of this call.
             *
             * NOTE: If you need to ensure that any account-based filtering is not honored (i.e.
             * because you expect `proxy` to have been used prior in the call stack and you do not want
             * the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`
             * in the Multisig pallet instead.
             *
             * NOTE: Prior to version *12, this was called `as_limited_sub`.
             *
             * The dispatch origin for this call must be _Signed_.
             **/
            asDerivative: AugmentedSubmittable<
                (
                    index: u16 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Call]
            >;
            /**
             * Send a batch of dispatch calls.
             *
             * May be called from any origin except `None`.
             *
             * - `calls`: The calls to be dispatched from the same origin. The number of call must not
             * exceed the constant: `batched_calls_limit` (available in constant metadata).
             *
             * If origin is root then the calls are dispatched without checking origin filter. (This
             * includes bypassing `frame_system::Config::BaseCallFilter`).
             *
             * ## Complexity
             * - O(C) where C is the number of calls to be batched.
             *
             * This will return `Ok` in all circumstances. To determine the success of the batch, an
             * event is deposited. If a call failed and the batch was interrupted, then the
             * `BatchInterrupted` event is deposited, along with the number of successful calls made
             * and the error of the failed call. If all were successful, then the `BatchCompleted`
             * event is deposited.
             **/
            batch: AugmentedSubmittable<
                (
                    calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /**
             * Send a batch of dispatch calls and atomically execute them.
             * The whole transaction will rollback and fail if any of the calls failed.
             *
             * May be called from any origin except `None`.
             *
             * - `calls`: The calls to be dispatched from the same origin. The number of call must not
             * exceed the constant: `batched_calls_limit` (available in constant metadata).
             *
             * If origin is root then the calls are dispatched without checking origin filter. (This
             * includes bypassing `frame_system::Config::BaseCallFilter`).
             *
             * ## Complexity
             * - O(C) where C is the number of calls to be batched.
             **/
            batchAll: AugmentedSubmittable<
                (
                    calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /**
             * Dispatches a function call with a provided origin.
             *
             * The dispatch origin for this call must be _Root_.
             *
             * ## Complexity
             * - O(1).
             **/
            dispatchAs: AugmentedSubmittable<
                (
                    asOrigin:
                        | RococoParachainRuntimeOriginCaller
                        | { system: any }
                        | { Void: any }
                        | { Council: any }
                        | { TechnicalCommittee: any }
                        | { PolkadotXcm: any }
                        | { CumulusXcm: any }
                        | string
                        | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [RococoParachainRuntimeOriginCaller, Call]
            >;
            /**
             * Send a batch of dispatch calls.
             * Unlike `batch`, it allows errors and won't interrupt.
             *
             * May be called from any origin except `None`.
             *
             * - `calls`: The calls to be dispatched from the same origin. The number of call must not
             * exceed the constant: `batched_calls_limit` (available in constant metadata).
             *
             * If origin is root then the calls are dispatch without checking origin filter. (This
             * includes bypassing `frame_system::Config::BaseCallFilter`).
             *
             * ## Complexity
             * - O(C) where C is the number of calls to be batched.
             **/
            forceBatch: AugmentedSubmittable<
                (
                    calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /**
             * Dispatch a function call with a specified weight.
             *
             * This function does not check the weight of the call, and instead allows the
             * Root origin to specify the weight of the call.
             *
             * The dispatch origin for this call must be _Root_.
             **/
            withWeight: AugmentedSubmittable<
                (
                    call: Call | IMethod | string | Uint8Array,
                    weight:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Call, SpWeightsWeightV2Weight]
            >;
        };
        vcManagement: {
            activateSchema: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    index: u64 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, u64]
            >;
            addSchema: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    id: Bytes | string | Uint8Array,
                    content: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, Bytes, Bytes]
            >;
            addVcRegistryItem: AugmentedSubmittable<
                (
                    index: H256 | string | Uint8Array,
                    subject: AccountId32 | string | Uint8Array,
                    assertion:
                        | CorePrimitivesAssertion
                        | { A1: any }
                        | { A2: any }
                        | { A3: any }
                        | { A4: any }
                        | { A5: any }
                        | { A6: any }
                        | { A7: any }
                        | { A8: any }
                        | { A9: any }
                        | { A10: any }
                        | { A11: any }
                        | { A13: any }
                        | string
                        | Uint8Array,
                    hash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, AccountId32, CorePrimitivesAssertion, H256]
            >;
            clearVcRegistry: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            disableSchema: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    index: u64 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, u64]
            >;
            disableVc: AugmentedSubmittable<
                (index: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            removeVcRegistryItem: AugmentedSubmittable<
                (index: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            requestVc: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    assertion:
                        | CorePrimitivesAssertion
                        | { A1: any }
                        | { A2: any }
                        | { A3: any }
                        | { A4: any }
                        | { A5: any }
                        | { A6: any }
                        | { A7: any }
                        | { A8: any }
                        | { A9: any }
                        | { A10: any }
                        | { A11: any }
                        | { A13: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, CorePrimitivesAssertion]
            >;
            revokeSchema: AugmentedSubmittable<
                (
                    shard: H256 | string | Uint8Array,
                    index: u64 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, u64]
            >;
            revokeVc: AugmentedSubmittable<
                (index: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            setAdmin: AugmentedSubmittable<
                (updated: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            someError: AugmentedSubmittable<
                (
                    account: Option<AccountId32> | null | Uint8Array | AccountId32 | string,
                    error:
                        | CorePrimitivesErrorVcmpError
                        | { RequestVCFailed: any }
                        | { UnclassifiedError: any }
                        | string
                        | Uint8Array,
                    reqExtHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Option<AccountId32>, CorePrimitivesErrorVcmpError, H256]
            >;
            /**
             * ---------------------------------------------------
             * The following extrinsics are supposed to be called by TEE only
             * ---------------------------------------------------
             **/
            vcIssued: AugmentedSubmittable<
                (
                    account: AccountId32 | string | Uint8Array,
                    assertion:
                        | CorePrimitivesAssertion
                        | { A1: any }
                        | { A2: any }
                        | { A3: any }
                        | { A4: any }
                        | { A5: any }
                        | { A6: any }
                        | { A7: any }
                        | { A8: any }
                        | { A9: any }
                        | { A10: any }
                        | { A11: any }
                        | { A13: any }
                        | string
                        | Uint8Array,
                    index: H256 | string | Uint8Array,
                    hash: H256 | string | Uint8Array,
                    vc:
                        | CorePrimitivesKeyAesOutput
                        | { ciphertext?: any; aad?: any; nonce?: any }
                        | string
                        | Uint8Array,
                    reqExtHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, CorePrimitivesAssertion, H256, H256, CorePrimitivesKeyAesOutput, H256]
            >;
        };
        vcmpExtrinsicWhitelist: {
            /**
             * Adds a new group member
             **/
            addGroupMember: AugmentedSubmittable<
                (v: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Batch adding of new group members
             **/
            batchAddGroupMembers: AugmentedSubmittable<
                (
                    vs: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /**
             * Batch Removing existing group members
             **/
            batchRemoveGroupMembers: AugmentedSubmittable<
                (
                    vs: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /**
             * Removes an existing group members
             **/
            removeGroupMember: AugmentedSubmittable<
                (v: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Swith GroupControlOn off
             **/
            switchGroupControlOff: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Swith GroupControlOn on
             **/
            switchGroupControlOn: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
        };
        vesting: {
            /**
             * Force a vested transfer.
             *
             * The dispatch origin for this call must be _Root_.
             *
             * - `source`: The account whose funds should be transferred.
             * - `target`: The account that should be transferred the vested funds.
             * - `schedule`: The vesting schedule attached to the transfer.
             *
             * Emits `VestingCreated`.
             *
             * NOTE: This will unlock all schedules through the current block.
             *
             * ## Complexity
             * - `O(1)`.
             **/
            forceVestedTransfer: AugmentedSubmittable<
                (
                    source:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    schedule:
                        | PalletVestingVestingInfo
                        | { locked?: any; perBlock?: any; startingBlock?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress, PalletVestingVestingInfo]
            >;
            /**
             * Merge two vesting schedules together, creating a new vesting schedule that unlocks over
             * the highest possible start and end blocks. If both schedules have already started the
             * current block will be used as the schedule start; with the caveat that if one schedule
             * is finished by the current block, the other will be treated as the new merged schedule,
             * unmodified.
             *
             * NOTE: If `schedule1_index == schedule2_index` this is a no-op.
             * NOTE: This will unlock all schedules through the current block prior to merging.
             * NOTE: If both schedules have ended by the current block, no new schedule will be created
             * and both will be removed.
             *
             * Merged schedule attributes:
             * - `starting_block`: `MAX(schedule1.starting_block, scheduled2.starting_block,
             * current_block)`.
             * - `ending_block`: `MAX(schedule1.ending_block, schedule2.ending_block)`.
             * - `locked`: `schedule1.locked_at(current_block) + schedule2.locked_at(current_block)`.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `schedule1_index`: index of the first schedule to merge.
             * - `schedule2_index`: index of the second schedule to merge.
             **/
            mergeSchedules: AugmentedSubmittable<
                (
                    schedule1Index: u32 | AnyNumber | Uint8Array,
                    schedule2Index: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Unlock any vested funds of the sender account.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have funds still
             * locked under this pallet.
             *
             * Emits either `VestingCompleted` or `VestingUpdated`.
             *
             * ## Complexity
             * - `O(1)`.
             **/
            vest: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Create a vested transfer.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `target`: The account receiving the vested funds.
             * - `schedule`: The vesting schedule attached to the transfer.
             *
             * Emits `VestingCreated`.
             *
             * NOTE: This will unlock all schedules through the current block.
             *
             * ## Complexity
             * - `O(1)`.
             **/
            vestedTransfer: AugmentedSubmittable<
                (
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    schedule:
                        | PalletVestingVestingInfo
                        | { locked?: any; perBlock?: any; startingBlock?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, PalletVestingVestingInfo]
            >;
            /**
             * Unlock any vested funds of a `target` account.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `target`: The account whose vested funds should be unlocked. Must have funds still
             * locked under this pallet.
             *
             * Emits either `VestingCompleted` or `VestingUpdated`.
             *
             * ## Complexity
             * - `O(1)`.
             **/
            vestOther: AugmentedSubmittable<
                (
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
        };
        xcmpQueue: {
            /**
             * Resumes all XCM executions for the XCMP queue.
             *
             * Note that this function doesn't change the status of the in/out bound channels.
             *
             * - `origin`: Must pass `ControllerOrigin`.
             **/
            resumeXcmExecution: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Services a single overweight XCM.
             *
             * - `origin`: Must pass `ExecuteOverweightOrigin`.
             * - `index`: The index of the overweight XCM to service
             * - `weight_limit`: The amount of weight that XCM execution may take.
             *
             * Errors:
             * - `BadOverweightIndex`: XCM under `index` is not found in the `Overweight` storage map.
             * - `BadXcm`: XCM under `index` cannot be properly decoded into a valid XCM format.
             * - `WeightOverLimit`: XCM execution may use greater `weight_limit`.
             *
             * Events:
             * - `OverweightServiced`: On success.
             **/
            serviceOverweight: AugmentedSubmittable<
                (
                    index: u64 | AnyNumber | Uint8Array,
                    weightLimit:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, SpWeightsWeightV2Weight]
            >;
            /**
             * Suspends all XCM executions for the XCMP queue, regardless of the sender's origin.
             *
             * - `origin`: Must pass `ControllerOrigin`.
             **/
            suspendXcmExecution: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Overwrites the number of pages of messages which must be in the queue after which we drop any further
             * messages from the channel.
             *
             * - `origin`: Must pass `Root`.
             * - `new`: Desired value for `QueueConfigData.drop_threshold`
             **/
            updateDropThreshold: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Overwrites the number of pages of messages which the queue must be reduced to before it signals that
             * message sending may recommence after it has been suspended.
             *
             * - `origin`: Must pass `Root`.
             * - `new`: Desired value for `QueueConfigData.resume_threshold`
             **/
            updateResumeThreshold: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Overwrites the number of pages of messages which must be in the queue for the other side to be told to
             * suspend their sending.
             *
             * - `origin`: Must pass `Root`.
             * - `new`: Desired value for `QueueConfigData.suspend_value`
             **/
            updateSuspendThreshold: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Overwrites the amount of remaining weight under which we stop processing messages.
             *
             * - `origin`: Must pass `Root`.
             * - `new`: Desired value for `QueueConfigData.threshold_weight`
             **/
            updateThresholdWeight: AugmentedSubmittable<
                (
                    updated:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpWeightsWeightV2Weight]
            >;
            /**
             * Overwrites the speed to which the available weight approaches the maximum weight.
             * A lower number results in a faster progression. A value of 1 makes the entire weight available initially.
             *
             * - `origin`: Must pass `Root`.
             * - `new`: Desired value for `QueueConfigData.weight_restrict_decay`.
             **/
            updateWeightRestrictDecay: AugmentedSubmittable<
                (
                    updated:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpWeightsWeightV2Weight]
            >;
            /**
             * Overwrite the maximum amount of weight any individual message may consume.
             * Messages above this weight go into the overweight queue and may only be serviced explicitly.
             *
             * - `origin`: Must pass `Root`.
             * - `new`: Desired value for `QueueConfigData.xcmp_max_individual_weight`.
             **/
            updateXcmpMaxIndividualWeight: AugmentedSubmittable<
                (
                    updated:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpWeightsWeightV2Weight]
            >;
        };
        xTokens: {
            /**
             * Transfer native currencies.
             *
             * `dest_weight_limit` is the weight for XCM execution on the dest
             * chain, and it would be charged from the transferred assets. If set
             * below requirements, the execution may fail and assets wouldn't be
             * received.
             *
             * It's a no-op if any error on local XCM execution or message sending.
             * Note sending assets out per se doesn't guarantee they would be
             * received. Receiving depends on if the XCM message could be delivered
             * by the network, and if the receiving chain would handle
             * messages correctly.
             **/
            transfer: AugmentedSubmittable<
                (
                    currencyId:
                        | RuntimeCommonXcmImplCurrencyId
                        | { SelfReserve: any }
                        | { ParachainReserve: any }
                        | string
                        | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array,
                    dest:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    destWeightLimit:
                        | XcmV3WeightLimit
                        | { Unlimited: any }
                        | { Limited: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [RuntimeCommonXcmImplCurrencyId, u128, XcmVersionedMultiLocation, XcmV3WeightLimit]
            >;
            /**
             * Transfer `MultiAsset`.
             *
             * `dest_weight_limit` is the weight for XCM execution on the dest
             * chain, and it would be charged from the transferred assets. If set
             * below requirements, the execution may fail and assets wouldn't be
             * received.
             *
             * It's a no-op if any error on local XCM execution or message sending.
             * Note sending assets out per se doesn't guarantee they would be
             * received. Receiving depends on if the XCM message could be delivered
             * by the network, and if the receiving chain would handle
             * messages correctly.
             **/
            transferMultiasset: AugmentedSubmittable<
                (
                    asset: XcmVersionedMultiAsset | { V2: any } | { V3: any } | string | Uint8Array,
                    dest:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    destWeightLimit:
                        | XcmV3WeightLimit
                        | { Unlimited: any }
                        | { Limited: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiAsset, XcmVersionedMultiLocation, XcmV3WeightLimit]
            >;
            /**
             * Transfer several `MultiAsset` specifying the item to be used as fee
             *
             * `dest_weight_limit` is the weight for XCM execution on the dest
             * chain, and it would be charged from the transferred assets. If set
             * below requirements, the execution may fail and assets wouldn't be
             * received.
             *
             * `fee_item` is index of the MultiAssets that we want to use for
             * payment
             *
             * It's a no-op if any error on local XCM execution or message sending.
             * Note sending assets out per se doesn't guarantee they would be
             * received. Receiving depends on if the XCM message could be delivered
             * by the network, and if the receiving chain would handle
             * messages correctly.
             **/
            transferMultiassets: AugmentedSubmittable<
                (
                    assets:
                        | XcmVersionedMultiAssets
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    feeItem: u32 | AnyNumber | Uint8Array,
                    dest:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    destWeightLimit:
                        | XcmV3WeightLimit
                        | { Unlimited: any }
                        | { Limited: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiAssets, u32, XcmVersionedMultiLocation, XcmV3WeightLimit]
            >;
            /**
             * Transfer `MultiAsset` specifying the fee and amount as separate.
             *
             * `dest_weight_limit` is the weight for XCM execution on the dest
             * chain, and it would be charged from the transferred assets. If set
             * below requirements, the execution may fail and assets wouldn't be
             * received.
             *
             * `fee` is the multiasset to be spent to pay for execution in
             * destination chain. Both fee and amount will be subtracted form the
             * callers balance For now we only accept fee and asset having the same
             * `MultiLocation` id.
             *
             * If `fee` is not high enough to cover for the execution costs in the
             * destination chain, then the assets will be trapped in the
             * destination chain
             *
             * It's a no-op if any error on local XCM execution or message sending.
             * Note sending assets out per se doesn't guarantee they would be
             * received. Receiving depends on if the XCM message could be delivered
             * by the network, and if the receiving chain would handle
             * messages correctly.
             **/
            transferMultiassetWithFee: AugmentedSubmittable<
                (
                    asset: XcmVersionedMultiAsset | { V2: any } | { V3: any } | string | Uint8Array,
                    fee: XcmVersionedMultiAsset | { V2: any } | { V3: any } | string | Uint8Array,
                    dest:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    destWeightLimit:
                        | XcmV3WeightLimit
                        | { Unlimited: any }
                        | { Limited: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [
                    XcmVersionedMultiAsset,
                    XcmVersionedMultiAsset,
                    XcmVersionedMultiLocation,
                    XcmV3WeightLimit
                ]
            >;
            /**
             * Transfer several currencies specifying the item to be used as fee
             *
             * `dest_weight_limit` is the weight for XCM execution on the dest
             * chain, and it would be charged from the transferred assets. If set
             * below requirements, the execution may fail and assets wouldn't be
             * received.
             *
             * `fee_item` is index of the currencies tuple that we want to use for
             * payment
             *
             * It's a no-op if any error on local XCM execution or message sending.
             * Note sending assets out per se doesn't guarantee they would be
             * received. Receiving depends on if the XCM message could be delivered
             * by the network, and if the receiving chain would handle
             * messages correctly.
             **/
            transferMulticurrencies: AugmentedSubmittable<
                (
                    currencies:
                        | Vec<ITuple<[RuntimeCommonXcmImplCurrencyId, u128]>>
                        | [
                              (
                                  | RuntimeCommonXcmImplCurrencyId
                                  | { SelfReserve: any }
                                  | { ParachainReserve: any }
                                  | string
                                  | Uint8Array
                              ),
                              u128 | AnyNumber | Uint8Array
                          ][],
                    feeItem: u32 | AnyNumber | Uint8Array,
                    dest:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    destWeightLimit:
                        | XcmV3WeightLimit
                        | { Unlimited: any }
                        | { Limited: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [
                    Vec<ITuple<[RuntimeCommonXcmImplCurrencyId, u128]>>,
                    u32,
                    XcmVersionedMultiLocation,
                    XcmV3WeightLimit
                ]
            >;
            /**
             * Transfer native currencies specifying the fee and amount as
             * separate.
             *
             * `dest_weight_limit` is the weight for XCM execution on the dest
             * chain, and it would be charged from the transferred assets. If set
             * below requirements, the execution may fail and assets wouldn't be
             * received.
             *
             * `fee` is the amount to be spent to pay for execution in destination
             * chain. Both fee and amount will be subtracted form the callers
             * balance.
             *
             * If `fee` is not high enough to cover for the execution costs in the
             * destination chain, then the assets will be trapped in the
             * destination chain
             *
             * It's a no-op if any error on local XCM execution or message sending.
             * Note sending assets out per se doesn't guarantee they would be
             * received. Receiving depends on if the XCM message could be delivered
             * by the network, and if the receiving chain would handle
             * messages correctly.
             **/
            transferWithFee: AugmentedSubmittable<
                (
                    currencyId:
                        | RuntimeCommonXcmImplCurrencyId
                        | { SelfReserve: any }
                        | { ParachainReserve: any }
                        | string
                        | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array,
                    fee: u128 | AnyNumber | Uint8Array,
                    dest:
                        | XcmVersionedMultiLocation
                        | { V2: any }
                        | { V3: any }
                        | string
                        | Uint8Array,
                    destWeightLimit:
                        | XcmV3WeightLimit
                        | { Unlimited: any }
                        | { Limited: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [
                    RuntimeCommonXcmImplCurrencyId,
                    u128,
                    u128,
                    XcmVersionedMultiLocation,
                    XcmV3WeightLimit
                ]
            >;
        };
    } // AugmentedSubmittables
} // declare module
