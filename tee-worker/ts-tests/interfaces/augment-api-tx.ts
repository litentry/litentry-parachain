// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/submittable';

import type {
    ApiTypes,
    AugmentedSubmittable,
    SubmittableExtrinsic,
    SubmittableExtrinsicFunction,
} from '@polkadot/api-base/types';
import type { Bytes, Compact, Option, Text, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { AnyNumber, IMethod, ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, Call, H256, MultiAddress } from '@polkadot/types/interfaces/runtime';
import type {
    ClaimsPrimitivesEcdsaSignature,
    ClaimsPrimitivesEthereumAddress,
    ClaimsPrimitivesStatementKind,
    CorePrimitivesAssertion,
    CorePrimitivesErrorImpError,
    CorePrimitivesErrorVcmpError,
    CorePrimitivesKeyAesOutput,
    IntegriteeNodeRuntimeOriginCaller,
    IntegriteeNodeRuntimeProxyType,
    PalletMultisigTimepoint,
    PalletVestingVestingInfo,
    SpCoreVoid,
    SpFinalityGrandpaEquivocationProof,
    SpWeightsWeightV2Weight,
    SubstrateFixedFixedU64,
    TeerexPrimitivesRequest,
} from '@polkadot/types/lookup';

export type __AugmentedSubmittable = AugmentedSubmittable<() => unknown>;
export type __SubmittableExtrinsic<ApiType extends ApiTypes> = SubmittableExtrinsic<ApiType>;
export type __SubmittableExtrinsicFunction<ApiType extends ApiTypes> = SubmittableExtrinsicFunction<ApiType>;

declare module '@polkadot/api-base/types/submittable' {
    interface AugmentedSubmittables<ApiType extends ApiTypes> {
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        claims: {
            /**
             * Attest to a statement, needed to finalize the claims process.
             *
             * WARNING: Insecure unless your chain includes `PrevalidateAttests` as a `SignedExtension`.
             *
             * Unsigned Validation:
             * A call to attest is deemed valid if the sender has a `Preclaim` registered
             * and provides a `statement` which is expected for the account.
             *
             * Parameters:
             * - `statement`: The identity of the statement which is being attested to in the signature.
             *
             * <weight>
             * The weight of this call is invariant over the input parameters.
             * Weight includes logic to do pre-validation on `attest` call.
             *
             * Total Complexity: O(1)
             * </weight>
             **/
            attest: AugmentedSubmittable<
                (statement: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Make a claim to collect your TEERs.
             *
             * The dispatch origin for this call must be _None_.
             *
             * Unsigned Validation:
             * A call to claim is deemed valid if the signature provided matches
             * the expected signed message of:
             *
             * > Ethereum Signed Message:
             * > (configured prefix string)(address)
             *
             * and `address` matches the `dest` account.
             *
             * Parameters:
             * - `dest`: The destination account to payout the claim.
             * - `ethereum_signature`: The signature of an ethereum signed message
             * matching the format described above.
             *
             * <weight>
             * The weight of this call is invariant over the input parameters.
             * Weight includes logic to validate unsigned `claim` call.
             *
             * Total Complexity: O(1)
             * </weight>
             **/
            claim: AugmentedSubmittable<
                (
                    dest: AccountId32 | string | Uint8Array,
                    ethereumSignature: ClaimsPrimitivesEcdsaSignature | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, ClaimsPrimitivesEcdsaSignature]
            >;
            /**
             * Make a claim to collect your TEERs by signing a statement.
             *
             * The dispatch origin for this call must be _None_.
             *
             * Unsigned Validation:
             * A call to `claim_attest` is deemed valid if the signature provided matches
             * the expected signed message of:
             *
             * > Ethereum Signed Message:
             * > (configured prefix string)(address)(statement)
             *
             * and `address` matches the `dest` account; the `statement` must match that which is
             * expected according to your purchase arrangement.
             *
             * Parameters:
             * - `dest`: The destination account to payout the claim.
             * - `ethereum_signature`: The signature of an ethereum signed message
             * matching the format described above.
             * - `statement`: The identity of the statement which is being attested to in the signature.
             *
             * <weight>
             * The weight of this call is invariant over the input parameters.
             * Weight includes logic to validate unsigned `claim_attest` call.
             *
             * Total Complexity: O(1)
             * </weight>
             **/
            claimAttest: AugmentedSubmittable<
                (
                    dest: AccountId32 | string | Uint8Array,
                    ethereumSignature: ClaimsPrimitivesEcdsaSignature | string | Uint8Array,
                    statement: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, ClaimsPrimitivesEcdsaSignature, Bytes]
            >;
            /**
             * Mint a new claim to collect TEERs.
             *
             * The dispatch origin for this call must be _Root_.
             *
             * Parameters:
             * - `who`: The Ethereum address allowed to collect this claim.
             * - `value`: The number of TEERs that will be claimed.
             * - `vesting_schedule`: An optional vesting schedule for these TEERs.
             *
             * <weight>
             * The weight of this call is invariant over the input parameters.
             * We assume worst case that both vesting and statement is being inserted.
             *
             * Total Complexity: O(1)
             * </weight>
             **/
            mintClaim: AugmentedSubmittable<
                (
                    who: ClaimsPrimitivesEthereumAddress | string | Uint8Array,
                    value: u128 | AnyNumber | Uint8Array,
                    vestingSchedule:
                        | Option<ITuple<[u128, u128, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u128, u128, u32]>
                        | [u128 | AnyNumber | Uint8Array, u128 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    statement:
                        | Option<ClaimsPrimitivesStatementKind>
                        | null
                        | Uint8Array
                        | ClaimsPrimitivesStatementKind
                        | 'Regular'
                        | 'Saft'
                        | number
                ) => SubmittableExtrinsic<ApiType>,
                [
                    ClaimsPrimitivesEthereumAddress,
                    u128,
                    Option<ITuple<[u128, u128, u32]>>,
                    Option<ClaimsPrimitivesStatementKind>
                ]
            >;
            moveClaim: AugmentedSubmittable<
                (
                    old: ClaimsPrimitivesEthereumAddress | string | Uint8Array,
                    updated: ClaimsPrimitivesEthereumAddress | string | Uint8Array,
                    maybePreclaim: Option<AccountId32> | null | Uint8Array | AccountId32 | string
                ) => SubmittableExtrinsic<ApiType>,
                [ClaimsPrimitivesEthereumAddress, ClaimsPrimitivesEthereumAddress, Option<AccountId32>]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        grandpa: {
            /**
             * Note that the current authority set of the GRANDPA finality gadget has stalled.
             *
             * This will trigger a forced authority set change at the beginning of the next session, to
             * be enacted `delay` blocks after that. The `delay` should be high enough to safely assume
             * that the block signalling the forced change will not be re-orged e.g. 1000 blocks.
             * The block production rate (which may be slowed down because of finality lagging) should
             * be taken into account when choosing the `delay`. The GRANDPA voters based on the new
             * authority will start voting on top of `best_finalized_block_number` for new finalized
             * blocks. `best_finalized_block_number` should be the highest of the latest finalized
             * block of all validators of the new authority set.
             *
             * Only callable by root.
             **/
            noteStalled: AugmentedSubmittable<
                (
                    delay: u32 | AnyNumber | Uint8Array,
                    bestFinalizedBlockNumber: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Report voter equivocation/misbehavior. This method will verify the
             * equivocation proof and validate the given key ownership proof
             * against the extracted offender. If both are valid, the offence
             * will be reported.
             **/
            reportEquivocation: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpFinalityGrandpaEquivocationProof
                        | { setId?: any; equivocation?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof: SpCoreVoid | null
                ) => SubmittableExtrinsic<ApiType>,
                [SpFinalityGrandpaEquivocationProof, SpCoreVoid]
            >;
            /**
             * Report voter equivocation/misbehavior. This method will verify the
             * equivocation proof and validate the given key ownership proof
             * against the extracted offender. If both are valid, the offence
             * will be reported.
             *
             * This extrinsic must be called unsigned and it is expected that only
             * block authors will call it (validated in `ValidateUnsigned`), as such
             * if the block author is defined it will be defined as the equivocation
             * reporter.
             **/
            reportEquivocationUnsigned: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpFinalityGrandpaEquivocationProof
                        | { setId?: any; equivocation?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof: SpCoreVoid | null
                ) => SubmittableExtrinsic<ApiType>,
                [SpFinalityGrandpaEquivocationProof, SpCoreVoid]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
                (vs: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /**
             * Batch Removing existing group members
             **/
            batchRemoveGroupMembers: AugmentedSubmittable<
                (vs: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
                    maxWeight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Vec<AccountId32>, Option<PalletMultisigTimepoint>, U8aFixed, SpWeightsWeightV2Weight]
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
                    maxWeight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Vec<AccountId32>, Option<PalletMultisigTimepoint>, Call, SpWeightsWeightV2Weight]
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
                    timepoint: PalletMultisigTimepoint | { height?: any; index?: any } | string | Uint8Array,
                    callHash: U8aFixed | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Vec<AccountId32>, PalletMultisigTimepoint, U8aFixed]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
                        | IntegriteeNodeRuntimeProxyType
                        | 'Any'
                        | 'NonTransfer'
                        | 'Governance'
                        | 'CancelProxy'
                        | number
                        | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, IntegriteeNodeRuntimeProxyType, u32]
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
                        | IntegriteeNodeRuntimeProxyType
                        | 'Any'
                        | 'NonTransfer'
                        | 'Governance'
                        | 'CancelProxy'
                        | number
                        | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array,
                    index: u16 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [IntegriteeNodeRuntimeProxyType, u32, u16]
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
                        | IntegriteeNodeRuntimeProxyType
                        | 'Any'
                        | 'NonTransfer'
                        | 'Governance'
                        | 'CancelProxy'
                        | number
                        | Uint8Array,
                    index: u16 | AnyNumber | Uint8Array,
                    height: Compact<u32> | AnyNumber | Uint8Array,
                    extIndex: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, IntegriteeNodeRuntimeProxyType, u16, Compact<u32>, Compact<u32>]
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
                        | Option<IntegriteeNodeRuntimeProxyType>
                        | null
                        | Uint8Array
                        | IntegriteeNodeRuntimeProxyType
                        | 'Any'
                        | 'NonTransfer'
                        | 'Governance'
                        | 'CancelProxy'
                        | number,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Option<IntegriteeNodeRuntimeProxyType>, Call]
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
                        | Option<IntegriteeNodeRuntimeProxyType>
                        | null
                        | Uint8Array
                        | IntegriteeNodeRuntimeProxyType
                        | 'Any'
                        | 'NonTransfer'
                        | 'Governance'
                        | 'CancelProxy'
                        | number,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress, Option<IntegriteeNodeRuntimeProxyType>, Call]
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
                        | IntegriteeNodeRuntimeProxyType
                        | 'Any'
                        | 'NonTransfer'
                        | 'Governance'
                        | 'CancelProxy'
                        | number
                        | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, IntegriteeNodeRuntimeProxyType, u32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
                    weight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Call, SpWeightsWeightV2Weight]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
                (keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
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
                    items: Vec<ITuple<[Bytes, Bytes]>> | [Bytes | string | Uint8Array, Bytes | string | Uint8Array][]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<ITuple<[Bytes, Bytes]>>]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        teeracle: {
            addToWhitelist: AugmentedSubmittable<
                (dataSource: Text | string, mrenclave: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Text, U8aFixed]
            >;
            removeFromWhitelist: AugmentedSubmittable<
                (dataSource: Text | string, mrenclave: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Text, U8aFixed]
            >;
            updateExchangeRate: AugmentedSubmittable<
                (
                    dataSource: Text | string,
                    tradingPair: Text | string,
                    newValue:
                        | Option<SubstrateFixedFixedU64>
                        | null
                        | Uint8Array
                        | SubstrateFixedFixedU64
                        | { bits?: any }
                        | string
                ) => SubmittableExtrinsic<ApiType>,
                [Text, Text, Option<SubstrateFixedFixedU64>]
            >;
            updateOracle: AugmentedSubmittable<
                (
                    oracleName: Text | string,
                    dataSource: Text | string,
                    newBlob: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Text, Text, Bytes]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        teerex: {
            callWorker: AugmentedSubmittable<
                (
                    request: TeerexPrimitivesRequest | { shard?: any; cyphertext?: any } | string | Uint8Array
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
                    blockNumber: u32 | AnyNumber | Uint8Array,
                    trustedCallsMerkleRoot: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, u32, H256]
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
                (sidechainBlockNumber: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
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
                (timeout: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
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
                    sidechainBlockNumber: u64 | AnyNumber | Uint8Array,
                    mrEnclave: U8aFixed | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, U8aFixed]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
                (proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
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
                (proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
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
                (proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
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
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
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
                        | IntegriteeNodeRuntimeOriginCaller
                        | { system: any }
                        | { Void: any }
                        | { Council: any }
                        | string
                        | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [IntegriteeNodeRuntimeOriginCaller, Call]
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
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
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
                    weight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Call, SpWeightsWeightV2Weight]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
                    vc: CorePrimitivesKeyAesOutput | { ciphertext?: any; aad?: any; nonce?: any } | string | Uint8Array,
                    reqExtHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, CorePrimitivesAssertion, H256, H256, CorePrimitivesKeyAesOutput, H256]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
                (vs: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /**
             * Batch Removing existing group members
             **/
            batchRemoveGroupMembers: AugmentedSubmittable<
                (vs: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
    } // AugmentedSubmittables
} // declare module
