// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/events';

import type { ApiTypes, AugmentedEvent } from '@polkadot/api-base/types';
import type { Bytes, Null, Option, Result, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H256, Perbill, Percent } from '@polkadot/types/interfaces/runtime';
import type {
    CorePrimitivesAssertion,
    CorePrimitivesErrorErrorDetail,
    CorePrimitivesKeyAesOutput,
    FrameSupportDispatchDispatchInfo,
    FrameSupportTokensMiscBalanceStatus,
    MockTeePrimitivesIdentity,
    PalletAssetManagerAssetMetadata,
    PalletDemocracyMetadataOwner,
    PalletDemocracyVoteAccountVote,
    PalletDemocracyVoteThreshold,
    PalletExtrinsicFilterOperationalMode,
    PalletIdentityManagementMockIdentityContext,
    PalletMultisigTimepoint,
    PalletParachainStakingDelegationRequestsCancelledScheduledRequest,
    PalletParachainStakingDelegatorAdded,
    RococoParachainRuntimeProxyType,
    RuntimeCommonXcmImplCurrencyId,
    SpRuntimeDispatchError,
    SpWeightsWeightV2Weight,
    SubstrateFixedFixedU64,
    XcmV3MultiAsset,
    XcmV3MultiLocation,
    XcmV3MultiassetMultiAssets,
    XcmV3Response,
    XcmV3TraitsError,
    XcmV3TraitsOutcome,
    XcmV3Xcm,
    XcmVersionedMultiAssets,
    XcmVersionedMultiLocation,
} from '@polkadot/types/lookup';

export type __AugmentedEvent<ApiType extends ApiTypes> = AugmentedEvent<ApiType>;

declare module '@polkadot/api-base/types/events' {
    interface AugmentedEvents<ApiType extends ApiTypes> {
        assetManager: {
            /**
             * The foreign asset updated.
             **/
            ForeignAssetMetadataUpdated: AugmentedEvent<
                ApiType,
                [assetId: u128, metadata: PalletAssetManagerAssetMetadata],
                { assetId: u128; metadata: PalletAssetManagerAssetMetadata }
            >;
            /**
             * AssetTracker manipulated
             **/
            ForeignAssetTrackerUpdated: AugmentedEvent<
                ApiType,
                [oldAssetTracker: u128, newAssetTracker: u128],
                { oldAssetTracker: u128; newAssetTracker: u128 }
            >;
            /**
             * New asset with the asset manager is registered
             **/
            ForeignAssetTypeRegistered: AugmentedEvent<
                ApiType,
                [assetId: u128, assetType: RuntimeCommonXcmImplCurrencyId],
                { assetId: u128; assetType: RuntimeCommonXcmImplCurrencyId }
            >;
            /**
             * New Event gives the info about involved asset_id, removed asset_type, and the new
             * default asset_id and asset_type mapping after removal
             **/
            ForeignAssetTypeRemoved: AugmentedEvent<
                ApiType,
                [
                    assetId: u128,
                    removedAssetType: RuntimeCommonXcmImplCurrencyId,
                    defaultAssetType: RuntimeCommonXcmImplCurrencyId
                ],
                {
                    assetId: u128;
                    removedAssetType: RuntimeCommonXcmImplCurrencyId;
                    defaultAssetType: RuntimeCommonXcmImplCurrencyId;
                }
            >;
            /**
             * Changed the amount of units we
             * are charging per execution second for a given asset
             **/
            UnitsPerSecondChanged: AugmentedEvent<
                ApiType,
                [assetId: u128, unitsPerSecond: u128],
                { assetId: u128; unitsPerSecond: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        balances: {
            /**
             * A balance was set by root.
             **/
            BalanceSet: AugmentedEvent<
                ApiType,
                [who: AccountId32, free: u128, reserved: u128],
                { who: AccountId32; free: u128; reserved: u128 }
            >;
            /**
             * Some amount was deposited (e.g. for transaction fees).
             **/
            Deposit: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * An account was removed whose balance was non-zero but below ExistentialDeposit,
             * resulting in an outright loss.
             **/
            DustLost: AugmentedEvent<
                ApiType,
                [account: AccountId32, amount: u128],
                { account: AccountId32; amount: u128 }
            >;
            /**
             * An account was created with some free balance.
             **/
            Endowed: AugmentedEvent<
                ApiType,
                [account: AccountId32, freeBalance: u128],
                { account: AccountId32; freeBalance: u128 }
            >;
            /**
             * Some balance was reserved (moved from free to reserved).
             **/
            Reserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Some balance was moved from the reserve of the first account to the second account.
             * Final argument indicates the destination balance type.
             **/
            ReserveRepatriated: AugmentedEvent<
                ApiType,
                [
                    from: AccountId32,
                    to: AccountId32,
                    amount: u128,
                    destinationStatus: FrameSupportTokensMiscBalanceStatus
                ],
                {
                    from: AccountId32;
                    to: AccountId32;
                    amount: u128;
                    destinationStatus: FrameSupportTokensMiscBalanceStatus;
                }
            >;
            /**
             * Some amount was removed from the account (e.g. for misbehavior).
             **/
            Slashed: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Transfer succeeded.
             **/
            Transfer: AugmentedEvent<
                ApiType,
                [from: AccountId32, to: AccountId32, amount: u128],
                { from: AccountId32; to: AccountId32; amount: u128 }
            >;
            /**
             * Some balance was unreserved (moved from reserved to free).
             **/
            Unreserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Some amount was withdrawn from the account (e.g. for transaction fees).
             **/
            Withdraw: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        bounties: {
            /**
             * A bounty is awarded to a beneficiary.
             **/
            BountyAwarded: AugmentedEvent<
                ApiType,
                [index: u32, beneficiary: AccountId32],
                { index: u32; beneficiary: AccountId32 }
            >;
            /**
             * A bounty proposal is funded and became active.
             **/
            BountyBecameActive: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /**
             * A bounty is cancelled.
             **/
            BountyCanceled: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /**
             * A bounty is claimed by beneficiary.
             **/
            BountyClaimed: AugmentedEvent<
                ApiType,
                [index: u32, payout: u128, beneficiary: AccountId32],
                { index: u32; payout: u128; beneficiary: AccountId32 }
            >;
            /**
             * A bounty expiry is extended.
             **/
            BountyExtended: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /**
             * New bounty proposal.
             **/
            BountyProposed: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /**
             * A bounty proposal was rejected; funds were slashed.
             **/
            BountyRejected: AugmentedEvent<ApiType, [index: u32, bond: u128], { index: u32; bond: u128 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        bridgeTransfer: {
            /**
             * MaximumIssuance was changed
             **/
            MaximumIssuanceChanged: AugmentedEvent<ApiType, [oldValue: u128], { oldValue: u128 }>;
            /**
             * A certain amount of native tokens was minted
             **/
            NativeTokenMinted: AugmentedEvent<
                ApiType,
                [to: AccountId32, amount: u128],
                { to: AccountId32; amount: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        chainBridge: {
            /**
             * Chain now available for transfers (chain_id)
             **/
            ChainWhitelisted: AugmentedEvent<ApiType, [u8]>;
            /**
             * Update bridge transfer fee
             **/
            FeeUpdated: AugmentedEvent<ApiType, [destId: u8, fee: u128], { destId: u8; fee: u128 }>;
            /**
             * FungibleTransfer is for relaying fungibles (dest_id, nonce, resource_id, amount,
             * recipient)
             **/
            FungibleTransfer: AugmentedEvent<ApiType, [u8, u64, U8aFixed, u128, Bytes]>;
            /**
             * GenericTransfer is for a generic data payload (dest_id, nonce, resource_id, metadata)
             **/
            GenericTransfer: AugmentedEvent<ApiType, [u8, u64, U8aFixed, Bytes]>;
            /**
             * NonFungibleTransfer is for relaying NFTs (dest_id, nonce, resource_id, token_id,
             * recipient, metadata)
             **/
            NonFungibleTransfer: AugmentedEvent<ApiType, [u8, u64, U8aFixed, Bytes, Bytes, Bytes]>;
            /**
             * Voting successful for a proposal
             **/
            ProposalApproved: AugmentedEvent<ApiType, [u8, u64]>;
            /**
             * Execution of call failed
             **/
            ProposalFailed: AugmentedEvent<ApiType, [u8, u64]>;
            /**
             * Voting rejected a proposal
             **/
            ProposalRejected: AugmentedEvent<ApiType, [u8, u64]>;
            /**
             * Execution of call succeeded
             **/
            ProposalSucceeded: AugmentedEvent<ApiType, [u8, u64]>;
            /**
             * Relayer added to set
             **/
            RelayerAdded: AugmentedEvent<ApiType, [AccountId32]>;
            /**
             * Relayer removed from set
             **/
            RelayerRemoved: AugmentedEvent<ApiType, [AccountId32]>;
            /**
             * Vote threshold has changed (new_threshold)
             **/
            RelayerThresholdChanged: AugmentedEvent<ApiType, [u32]>;
            /**
             * Vot submitted against proposal
             **/
            VoteAgainst: AugmentedEvent<ApiType, [u8, u64, AccountId32]>;
            /**
             * Vote submitted in favour of proposal
             **/
            VoteFor: AugmentedEvent<ApiType, [u8, u64, AccountId32]>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        council: {
            /**
             * A motion was approved by the required threshold.
             **/
            Approved: AugmentedEvent<ApiType, [proposalHash: H256], { proposalHash: H256 }>;
            /**
             * A proposal was closed because its threshold was reached or after its duration was up.
             **/
            Closed: AugmentedEvent<
                ApiType,
                [proposalHash: H256, yes: u32, no: u32],
                { proposalHash: H256; yes: u32; no: u32 }
            >;
            /**
             * A motion was not approved by the required threshold.
             **/
            Disapproved: AugmentedEvent<ApiType, [proposalHash: H256], { proposalHash: H256 }>;
            /**
             * A motion was executed; result will be `Ok` if it returned without error.
             **/
            Executed: AugmentedEvent<
                ApiType,
                [proposalHash: H256, result: Result<Null, SpRuntimeDispatchError>],
                { proposalHash: H256; result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A single member did some action; result will be `Ok` if it returned without error.
             **/
            MemberExecuted: AugmentedEvent<
                ApiType,
                [proposalHash: H256, result: Result<Null, SpRuntimeDispatchError>],
                { proposalHash: H256; result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A motion (given hash) has been proposed (by given account) with a threshold (given
             * `MemberCount`).
             **/
            Proposed: AugmentedEvent<
                ApiType,
                [account: AccountId32, proposalIndex: u32, proposalHash: H256, threshold: u32],
                { account: AccountId32; proposalIndex: u32; proposalHash: H256; threshold: u32 }
            >;
            /**
             * A motion (given hash) has been voted on by given account, leaving
             * a tally (yes votes and no votes given respectively as `MemberCount`).
             **/
            Voted: AugmentedEvent<
                ApiType,
                [account: AccountId32, proposalHash: H256, voted: bool, yes: u32, no: u32],
                { account: AccountId32; proposalHash: H256; voted: bool; yes: u32; no: u32 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        councilMembership: {
            /**
             * Phantom member, never used.
             **/
            Dummy: AugmentedEvent<ApiType, []>;
            /**
             * One of the members' keys changed.
             **/
            KeyChanged: AugmentedEvent<ApiType, []>;
            /**
             * The given member was added; see the transaction for who.
             **/
            MemberAdded: AugmentedEvent<ApiType, []>;
            /**
             * The given member was removed; see the transaction for who.
             **/
            MemberRemoved: AugmentedEvent<ApiType, []>;
            /**
             * The membership was reset; see the transaction for who the new set is.
             **/
            MembersReset: AugmentedEvent<ApiType, []>;
            /**
             * Two members were swapped; see the transaction for who.
             **/
            MembersSwapped: AugmentedEvent<ApiType, []>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        cumulusXcm: {
            /**
             * Downward message executed with the given outcome.
             * \[ id, outcome \]
             **/
            ExecutedDownward: AugmentedEvent<ApiType, [U8aFixed, XcmV3TraitsOutcome]>;
            /**
             * Downward message is invalid XCM.
             * \[ id \]
             **/
            InvalidFormat: AugmentedEvent<ApiType, [U8aFixed]>;
            /**
             * Downward message is unsupported version of XCM.
             * \[ id \]
             **/
            UnsupportedVersion: AugmentedEvent<ApiType, [U8aFixed]>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        democracy: {
            /**
             * A proposal_hash has been blacklisted permanently.
             **/
            Blacklisted: AugmentedEvent<ApiType, [proposalHash: H256], { proposalHash: H256 }>;
            /**
             * A referendum has been cancelled.
             **/
            Cancelled: AugmentedEvent<ApiType, [refIndex: u32], { refIndex: u32 }>;
            /**
             * An account has delegated their vote to another account.
             **/
            Delegated: AugmentedEvent<
                ApiType,
                [who: AccountId32, target: AccountId32],
                { who: AccountId32; target: AccountId32 }
            >;
            /**
             * An external proposal has been tabled.
             **/
            ExternalTabled: AugmentedEvent<ApiType, []>;
            /**
             * Metadata for a proposal or a referendum has been cleared.
             **/
            MetadataCleared: AugmentedEvent<
                ApiType,
                [owner: PalletDemocracyMetadataOwner, hash_: H256],
                { owner: PalletDemocracyMetadataOwner; hash_: H256 }
            >;
            /**
             * Metadata for a proposal or a referendum has been set.
             **/
            MetadataSet: AugmentedEvent<
                ApiType,
                [owner: PalletDemocracyMetadataOwner, hash_: H256],
                { owner: PalletDemocracyMetadataOwner; hash_: H256 }
            >;
            /**
             * Metadata has been transferred to new owner.
             **/
            MetadataTransferred: AugmentedEvent<
                ApiType,
                [prevOwner: PalletDemocracyMetadataOwner, owner: PalletDemocracyMetadataOwner, hash_: H256],
                { prevOwner: PalletDemocracyMetadataOwner; owner: PalletDemocracyMetadataOwner; hash_: H256 }
            >;
            /**
             * A proposal has been rejected by referendum.
             **/
            NotPassed: AugmentedEvent<ApiType, [refIndex: u32], { refIndex: u32 }>;
            /**
             * A proposal has been approved by referendum.
             **/
            Passed: AugmentedEvent<ApiType, [refIndex: u32], { refIndex: u32 }>;
            /**
             * A proposal got canceled.
             **/
            ProposalCanceled: AugmentedEvent<ApiType, [propIndex: u32], { propIndex: u32 }>;
            /**
             * A motion has been proposed by a public account.
             **/
            Proposed: AugmentedEvent<
                ApiType,
                [proposalIndex: u32, deposit: u128],
                { proposalIndex: u32; deposit: u128 }
            >;
            /**
             * An account has secconded a proposal
             **/
            Seconded: AugmentedEvent<
                ApiType,
                [seconder: AccountId32, propIndex: u32],
                { seconder: AccountId32; propIndex: u32 }
            >;
            /**
             * A referendum has begun.
             **/
            Started: AugmentedEvent<
                ApiType,
                [refIndex: u32, threshold: PalletDemocracyVoteThreshold],
                { refIndex: u32; threshold: PalletDemocracyVoteThreshold }
            >;
            /**
             * A public proposal has been tabled for referendum vote.
             **/
            Tabled: AugmentedEvent<ApiType, [proposalIndex: u32, deposit: u128], { proposalIndex: u32; deposit: u128 }>;
            /**
             * An account has cancelled a previous delegation operation.
             **/
            Undelegated: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            /**
             * An external proposal has been vetoed.
             **/
            Vetoed: AugmentedEvent<
                ApiType,
                [who: AccountId32, proposalHash: H256, until: u32],
                { who: AccountId32; proposalHash: H256; until: u32 }
            >;
            /**
             * An account has voted in a referendum
             **/
            Voted: AugmentedEvent<
                ApiType,
                [voter: AccountId32, refIndex: u32, vote: PalletDemocracyVoteAccountVote],
                { voter: AccountId32; refIndex: u32; vote: PalletDemocracyVoteAccountVote }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        dmpQueue: {
            /**
             * Downward message executed with the given outcome.
             **/
            ExecutedDownward: AugmentedEvent<
                ApiType,
                [messageId: U8aFixed, outcome: XcmV3TraitsOutcome],
                { messageId: U8aFixed; outcome: XcmV3TraitsOutcome }
            >;
            /**
             * Downward message is invalid XCM.
             **/
            InvalidFormat: AugmentedEvent<ApiType, [messageId: U8aFixed], { messageId: U8aFixed }>;
            /**
             * The maximum number of downward messages was.
             **/
            MaxMessagesExhausted: AugmentedEvent<ApiType, [messageId: U8aFixed], { messageId: U8aFixed }>;
            /**
             * Downward message is overweight and was placed in the overweight queue.
             **/
            OverweightEnqueued: AugmentedEvent<
                ApiType,
                [messageId: U8aFixed, overweightIndex: u64, requiredWeight: SpWeightsWeightV2Weight],
                { messageId: U8aFixed; overweightIndex: u64; requiredWeight: SpWeightsWeightV2Weight }
            >;
            /**
             * Downward message from the overweight queue was executed.
             **/
            OverweightServiced: AugmentedEvent<
                ApiType,
                [overweightIndex: u64, weightUsed: SpWeightsWeightV2Weight],
                { overweightIndex: u64; weightUsed: SpWeightsWeightV2Weight }
            >;
            /**
             * Downward message is unsupported version of XCM.
             **/
            UnsupportedVersion: AugmentedEvent<ApiType, [messageId: U8aFixed], { messageId: U8aFixed }>;
            /**
             * The weight limit for handling downward messages was reached.
             **/
            WeightExhausted: AugmentedEvent<
                ApiType,
                [
                    messageId: U8aFixed,
                    remainingWeight: SpWeightsWeightV2Weight,
                    requiredWeight: SpWeightsWeightV2Weight
                ],
                {
                    messageId: U8aFixed;
                    remainingWeight: SpWeightsWeightV2Weight;
                    requiredWeight: SpWeightsWeightV2Weight;
                }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        drop3: {
            /**
             * Admin acccount was changed, the \[ old admin \] is provided
             **/
            AdminChanged: AugmentedEvent<ApiType, [oldAdmin: Option<AccountId32>], { oldAdmin: Option<AccountId32> }>;
            /**
             * An \[ amount \] balance of \[ who \] was slashed
             **/
            BalanceSlashed: AugmentedEvent<
                ApiType,
                [who: AccountId32, amount: u128],
                { who: AccountId32; amount: u128 }
            >;
            /**
             * A reward pool with \[ id \] was approved by admin
             **/
            RewardPoolApproved: AugmentedEvent<ApiType, [id: u64], { id: u64 }>;
            /**
             * A reward pool with \[ id, name, owner \] was proposed
             **/
            RewardPoolProposed: AugmentedEvent<
                ApiType,
                [id: u64, name: Bytes, owner: AccountId32],
                { id: u64; name: Bytes; owner: AccountId32 }
            >;
            /**
             * A reward pool with \[ id \] was rejected by admin
             **/
            RewardPoolRejected: AugmentedEvent<ApiType, [id: u64], { id: u64 }>;
            /**
             * A reward pool with \[ id, name, owner \] was removed, either by admin or owner
             **/
            RewardPoolRemoved: AugmentedEvent<
                ApiType,
                [id: u64, name: Bytes, owner: AccountId32],
                { id: u64; name: Bytes; owner: AccountId32 }
            >;
            /**
             * A reward pool with \[ id \] was started, either by admin or owner
             **/
            RewardPoolStarted: AugmentedEvent<ApiType, [id: u64], { id: u64 }>;
            /**
             * A reward pool with \[ id \] was stopped, either by admin or owner
             **/
            RewardPoolStopped: AugmentedEvent<ApiType, [id: u64], { id: u64 }>;
            /**
             * An \[ amount \] of reward was sent to \[ to \]
             **/
            RewardSent: AugmentedEvent<ApiType, [to: AccountId32, amount: u128], { to: AccountId32; amount: u128 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        extrinsicFilter: {
            /**
             * some extrinsics are blocked
             **/
            ExtrinsicsBlocked: AugmentedEvent<
                ApiType,
                [palletNameBytes: Bytes, functionNameBytes: Option<Bytes>],
                { palletNameBytes: Bytes; functionNameBytes: Option<Bytes> }
            >;
            /**
             * some extrinsics are unblocked
             **/
            ExtrinsicsUnblocked: AugmentedEvent<
                ApiType,
                [palletNameBytes: Bytes, functionNameBytes: Option<Bytes>],
                { palletNameBytes: Bytes; functionNameBytes: Option<Bytes> }
            >;
            /**
             * a new mode was just sent
             **/
            ModeSet: AugmentedEvent<
                ApiType,
                [newMode: PalletExtrinsicFilterOperationalMode],
                { newMode: PalletExtrinsicFilterOperationalMode }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        identityManagement: {
            CreateIdentityFailed: AugmentedEvent<
                ApiType,
                [account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256],
                { account: Option<AccountId32>; detail: CorePrimitivesErrorErrorDetail; reqExtHash: H256 }
            >;
            CreateIdentityRequested: AugmentedEvent<ApiType, [shard: H256], { shard: H256 }>;
            DelegateeAdded: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            DelegateeRemoved: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            IdentityCreated: AugmentedEvent<
                ApiType,
                [
                    account: AccountId32,
                    identity: CorePrimitivesKeyAesOutput,
                    code: CorePrimitivesKeyAesOutput,
                    reqExtHash: H256
                ],
                {
                    account: AccountId32;
                    identity: CorePrimitivesKeyAesOutput;
                    code: CorePrimitivesKeyAesOutput;
                    reqExtHash: H256;
                }
            >;
            IdentityRemoved: AugmentedEvent<
                ApiType,
                [account: AccountId32, identity: CorePrimitivesKeyAesOutput, reqExtHash: H256],
                { account: AccountId32; identity: CorePrimitivesKeyAesOutput; reqExtHash: H256 }
            >;
            IdentityVerified: AugmentedEvent<
                ApiType,
                [
                    account: AccountId32,
                    identity: CorePrimitivesKeyAesOutput,
                    idGraph: CorePrimitivesKeyAesOutput,
                    reqExtHash: H256
                ],
                {
                    account: AccountId32;
                    identity: CorePrimitivesKeyAesOutput;
                    idGraph: CorePrimitivesKeyAesOutput;
                    reqExtHash: H256;
                }
            >;
            ImportScheduledEnclaveFailed: AugmentedEvent<ApiType, []>;
            RemoveIdentityFailed: AugmentedEvent<
                ApiType,
                [account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256],
                { account: Option<AccountId32>; detail: CorePrimitivesErrorErrorDetail; reqExtHash: H256 }
            >;
            RemoveIdentityRequested: AugmentedEvent<ApiType, [shard: H256], { shard: H256 }>;
            SetUserShieldingKeyFailed: AugmentedEvent<
                ApiType,
                [account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256],
                { account: Option<AccountId32>; detail: CorePrimitivesErrorErrorDetail; reqExtHash: H256 }
            >;
            SetUserShieldingKeyRequested: AugmentedEvent<ApiType, [shard: H256], { shard: H256 }>;
            UnclassifiedError: AugmentedEvent<
                ApiType,
                [account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256],
                { account: Option<AccountId32>; detail: CorePrimitivesErrorErrorDetail; reqExtHash: H256 }
            >;
            UserShieldingKeySet: AugmentedEvent<
                ApiType,
                [account: AccountId32, idGraph: CorePrimitivesKeyAesOutput, reqExtHash: H256],
                { account: AccountId32; idGraph: CorePrimitivesKeyAesOutput; reqExtHash: H256 }
            >;
            VerifyIdentityFailed: AugmentedEvent<
                ApiType,
                [account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256],
                { account: Option<AccountId32>; detail: CorePrimitivesErrorErrorDetail; reqExtHash: H256 }
            >;
            VerifyIdentityRequested: AugmentedEvent<ApiType, [shard: H256], { shard: H256 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        identityManagementMock: {
            CreateIdentityRequested: AugmentedEvent<ApiType, [shard: H256], { shard: H256 }>;
            DelegateeAdded: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            DelegateeRemoved: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            IdentityCreated: AugmentedEvent<
                ApiType,
                [
                    account: AccountId32,
                    identity: CorePrimitivesKeyAesOutput,
                    code: CorePrimitivesKeyAesOutput,
                    idGraph: CorePrimitivesKeyAesOutput
                ],
                {
                    account: AccountId32;
                    identity: CorePrimitivesKeyAesOutput;
                    code: CorePrimitivesKeyAesOutput;
                    idGraph: CorePrimitivesKeyAesOutput;
                }
            >;
            IdentityCreatedPlain: AugmentedEvent<
                ApiType,
                [
                    account: AccountId32,
                    identity: MockTeePrimitivesIdentity,
                    code: U8aFixed,
                    idGraph: Vec<ITuple<[MockTeePrimitivesIdentity, PalletIdentityManagementMockIdentityContext]>>
                ],
                {
                    account: AccountId32;
                    identity: MockTeePrimitivesIdentity;
                    code: U8aFixed;
                    idGraph: Vec<ITuple<[MockTeePrimitivesIdentity, PalletIdentityManagementMockIdentityContext]>>;
                }
            >;
            IdentityRemoved: AugmentedEvent<
                ApiType,
                [account: AccountId32, identity: CorePrimitivesKeyAesOutput, idGraph: CorePrimitivesKeyAesOutput],
                { account: AccountId32; identity: CorePrimitivesKeyAesOutput; idGraph: CorePrimitivesKeyAesOutput }
            >;
            IdentityRemovedPlain: AugmentedEvent<
                ApiType,
                [
                    account: AccountId32,
                    identity: MockTeePrimitivesIdentity,
                    idGraph: Vec<ITuple<[MockTeePrimitivesIdentity, PalletIdentityManagementMockIdentityContext]>>
                ],
                {
                    account: AccountId32;
                    identity: MockTeePrimitivesIdentity;
                    idGraph: Vec<ITuple<[MockTeePrimitivesIdentity, PalletIdentityManagementMockIdentityContext]>>;
                }
            >;
            IdentityVerified: AugmentedEvent<
                ApiType,
                [account: AccountId32, identity: CorePrimitivesKeyAesOutput, idGraph: CorePrimitivesKeyAesOutput],
                { account: AccountId32; identity: CorePrimitivesKeyAesOutput; idGraph: CorePrimitivesKeyAesOutput }
            >;
            IdentityVerifiedPlain: AugmentedEvent<
                ApiType,
                [
                    account: AccountId32,
                    identity: MockTeePrimitivesIdentity,
                    idGraph: Vec<ITuple<[MockTeePrimitivesIdentity, PalletIdentityManagementMockIdentityContext]>>
                ],
                {
                    account: AccountId32;
                    identity: MockTeePrimitivesIdentity;
                    idGraph: Vec<ITuple<[MockTeePrimitivesIdentity, PalletIdentityManagementMockIdentityContext]>>;
                }
            >;
            RemoveIdentityRequested: AugmentedEvent<ApiType, [shard: H256], { shard: H256 }>;
            SetUserShieldingKeyRequested: AugmentedEvent<ApiType, [shard: H256], { shard: H256 }>;
            SomeError: AugmentedEvent<ApiType, [func: Bytes, error: Bytes], { func: Bytes; error: Bytes }>;
            UserShieldingKeySet: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            UserShieldingKeySetPlain: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            VerifyIdentityRequested: AugmentedEvent<ApiType, [shard: H256], { shard: H256 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        impExtrinsicWhitelist: {
            /**
             * Group member added to set
             **/
            GroupMemberAdded: AugmentedEvent<ApiType, [AccountId32]>;
            /**
             * Group member removed from set
             **/
            GroupMemberRemoved: AugmentedEvent<ApiType, [AccountId32]>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        multisig: {
            /**
             * A multisig operation has been approved by someone.
             **/
            MultisigApproval: AugmentedEvent<
                ApiType,
                [approving: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed],
                {
                    approving: AccountId32;
                    timepoint: PalletMultisigTimepoint;
                    multisig: AccountId32;
                    callHash: U8aFixed;
                }
            >;
            /**
             * A multisig operation has been cancelled.
             **/
            MultisigCancelled: AugmentedEvent<
                ApiType,
                [
                    cancelling: AccountId32,
                    timepoint: PalletMultisigTimepoint,
                    multisig: AccountId32,
                    callHash: U8aFixed
                ],
                {
                    cancelling: AccountId32;
                    timepoint: PalletMultisigTimepoint;
                    multisig: AccountId32;
                    callHash: U8aFixed;
                }
            >;
            /**
             * A multisig operation has been executed.
             **/
            MultisigExecuted: AugmentedEvent<
                ApiType,
                [
                    approving: AccountId32,
                    timepoint: PalletMultisigTimepoint,
                    multisig: AccountId32,
                    callHash: U8aFixed,
                    result: Result<Null, SpRuntimeDispatchError>
                ],
                {
                    approving: AccountId32;
                    timepoint: PalletMultisigTimepoint;
                    multisig: AccountId32;
                    callHash: U8aFixed;
                    result: Result<Null, SpRuntimeDispatchError>;
                }
            >;
            /**
             * A new multisig operation has begun.
             **/
            NewMultisig: AugmentedEvent<
                ApiType,
                [approving: AccountId32, multisig: AccountId32, callHash: U8aFixed],
                { approving: AccountId32; multisig: AccountId32; callHash: U8aFixed }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        parachainIdentity: {
            /**
             * A name was cleared, and the given balance returned.
             **/
            IdentityCleared: AugmentedEvent<
                ApiType,
                [who: AccountId32, deposit: u128],
                { who: AccountId32; deposit: u128 }
            >;
            /**
             * A name was removed and the given balance slashed.
             **/
            IdentityKilled: AugmentedEvent<
                ApiType,
                [who: AccountId32, deposit: u128],
                { who: AccountId32; deposit: u128 }
            >;
            /**
             * A name was set or reset (which will remove all judgements).
             **/
            IdentitySet: AugmentedEvent<ApiType, [who: AccountId32], { who: AccountId32 }>;
            /**
             * A judgement was given by a registrar.
             **/
            JudgementGiven: AugmentedEvent<
                ApiType,
                [target: AccountId32, registrarIndex: u32],
                { target: AccountId32; registrarIndex: u32 }
            >;
            /**
             * A judgement was asked from a registrar.
             **/
            JudgementRequested: AugmentedEvent<
                ApiType,
                [who: AccountId32, registrarIndex: u32],
                { who: AccountId32; registrarIndex: u32 }
            >;
            /**
             * A judgement request was retracted.
             **/
            JudgementUnrequested: AugmentedEvent<
                ApiType,
                [who: AccountId32, registrarIndex: u32],
                { who: AccountId32; registrarIndex: u32 }
            >;
            /**
             * A registrar was added.
             **/
            RegistrarAdded: AugmentedEvent<ApiType, [registrarIndex: u32], { registrarIndex: u32 }>;
            /**
             * A sub-identity was added to an identity and the deposit paid.
             **/
            SubIdentityAdded: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32, deposit: u128],
                { sub: AccountId32; main: AccountId32; deposit: u128 }
            >;
            /**
             * A sub-identity was removed from an identity and the deposit freed.
             **/
            SubIdentityRemoved: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32, deposit: u128],
                { sub: AccountId32; main: AccountId32; deposit: u128 }
            >;
            /**
             * A sub-identity was cleared, and the given deposit repatriated from the
             * main identity account to the sub-identity account.
             **/
            SubIdentityRevoked: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32, deposit: u128],
                { sub: AccountId32; main: AccountId32; deposit: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        parachainStaking: {
            /**
             * Auto-compounding reward percent was set for a delegation.
             **/
            AutoCompoundSet: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, value: Percent],
                { candidate: AccountId32; delegator: AccountId32; value: Percent }
            >;
            /**
             * Set blocks per round
             **/
            BlocksPerRoundSet: AugmentedEvent<
                ApiType,
                [
                    currentRound: u32,
                    firstBlock: u32,
                    old: u32,
                    new_: u32,
                    newPerRoundInflationMin: Perbill,
                    newPerRoundInflationIdeal: Perbill,
                    newPerRoundInflationMax: Perbill
                ],
                {
                    currentRound: u32;
                    firstBlock: u32;
                    old: u32;
                    new_: u32;
                    newPerRoundInflationMin: Perbill;
                    newPerRoundInflationIdeal: Perbill;
                    newPerRoundInflationMax: Perbill;
                }
            >;
            /**
             * Cancelled request to decrease candidate's bond.
             **/
            CancelledCandidateBondLess: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, amount: u128, executeRound: u32],
                { candidate: AccountId32; amount: u128; executeRound: u32 }
            >;
            /**
             * Cancelled request to leave the set of candidates.
             **/
            CancelledCandidateExit: AugmentedEvent<ApiType, [candidate: AccountId32], { candidate: AccountId32 }>;
            /**
             * Cancelled request to change an existing delegation.
             **/
            CancelledDelegationRequest: AugmentedEvent<
                ApiType,
                [
                    delegator: AccountId32,
                    cancelledRequest: PalletParachainStakingDelegationRequestsCancelledScheduledRequest,
                    collator: AccountId32
                ],
                {
                    delegator: AccountId32;
                    cancelledRequest: PalletParachainStakingDelegationRequestsCancelledScheduledRequest;
                    collator: AccountId32;
                }
            >;
            /**
             * Candidate rejoins the set of collator candidates.
             **/
            CandidateBackOnline: AugmentedEvent<ApiType, [candidate: AccountId32], { candidate: AccountId32 }>;
            /**
             * Candidate has decreased a self bond.
             **/
            CandidateBondedLess: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, amount: u128, newBond: u128],
                { candidate: AccountId32; amount: u128; newBond: u128 }
            >;
            /**
             * Candidate has increased a self bond.
             **/
            CandidateBondedMore: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, amount: u128, newTotalBond: u128],
                { candidate: AccountId32; amount: u128; newTotalBond: u128 }
            >;
            /**
             * Candidate requested to decrease a self bond.
             **/
            CandidateBondLessRequested: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, amountToDecrease: u128, executeRound: u32],
                { candidate: AccountId32; amountToDecrease: u128; executeRound: u32 }
            >;
            /**
             * Candidate has left the set of candidates.
             **/
            CandidateLeft: AugmentedEvent<
                ApiType,
                [exCandidate: AccountId32, unlockedAmount: u128, newTotalAmtLocked: u128],
                { exCandidate: AccountId32; unlockedAmount: u128; newTotalAmtLocked: u128 }
            >;
            /**
             * Candidate has requested to leave the set of candidates.
             **/
            CandidateScheduledExit: AugmentedEvent<
                ApiType,
                [exitAllowedRound: u32, candidate: AccountId32, scheduledExit: u32],
                { exitAllowedRound: u32; candidate: AccountId32; scheduledExit: u32 }
            >;
            /**
             * Candidate temporarily leave the set of collator candidates without unbonding.
             **/
            CandidateWentOffline: AugmentedEvent<ApiType, [candidate: AccountId32], { candidate: AccountId32 }>;
            CandidateWhiteListAdded: AugmentedEvent<ApiType, [candidate: AccountId32], { candidate: AccountId32 }>;
            CandidateWhiteListRemoved: AugmentedEvent<ApiType, [candidate: AccountId32], { candidate: AccountId32 }>;
            /**
             * Candidate selected for collators. Total Exposed Amount includes all delegations.
             **/
            CollatorChosen: AugmentedEvent<
                ApiType,
                [round: u32, collatorAccount: AccountId32, totalExposedAmount: u128],
                { round: u32; collatorAccount: AccountId32; totalExposedAmount: u128 }
            >;
            /**
             * Set collator commission to this value.
             **/
            CollatorCommissionSet: AugmentedEvent<
                ApiType,
                [old: Perbill, new_: Perbill],
                { old: Perbill; new_: Perbill }
            >;
            /**
             * Compounded a portion of rewards towards the delegation.
             **/
            Compounded: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, amount: u128],
                { candidate: AccountId32; delegator: AccountId32; amount: u128 }
            >;
            /**
             * New delegation (increase of the existing one).
             **/
            Delegation: AugmentedEvent<
                ApiType,
                [
                    delegator: AccountId32,
                    lockedAmount: u128,
                    candidate: AccountId32,
                    delegatorPosition: PalletParachainStakingDelegatorAdded,
                    autoCompound: Percent
                ],
                {
                    delegator: AccountId32;
                    lockedAmount: u128;
                    candidate: AccountId32;
                    delegatorPosition: PalletParachainStakingDelegatorAdded;
                    autoCompound: Percent;
                }
            >;
            DelegationDecreased: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, candidate: AccountId32, amount: u128, inTop: bool],
                { delegator: AccountId32; candidate: AccountId32; amount: u128; inTop: bool }
            >;
            /**
             * Delegator requested to decrease a bond for the collator candidate.
             **/
            DelegationDecreaseScheduled: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, candidate: AccountId32, amountToDecrease: u128, executeRound: u32],
                { delegator: AccountId32; candidate: AccountId32; amountToDecrease: u128; executeRound: u32 }
            >;
            DelegationIncreased: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, candidate: AccountId32, amount: u128, inTop: bool],
                { delegator: AccountId32; candidate: AccountId32; amount: u128; inTop: bool }
            >;
            /**
             * Delegation kicked.
             **/
            DelegationKicked: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, candidate: AccountId32, unstakedAmount: u128],
                { delegator: AccountId32; candidate: AccountId32; unstakedAmount: u128 }
            >;
            /**
             * Delegator requested to revoke delegation.
             **/
            DelegationRevocationScheduled: AugmentedEvent<
                ApiType,
                [round: u32, delegator: AccountId32, candidate: AccountId32, scheduledExit: u32],
                { round: u32; delegator: AccountId32; candidate: AccountId32; scheduledExit: u32 }
            >;
            /**
             * Delegation revoked.
             **/
            DelegationRevoked: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, candidate: AccountId32, unstakedAmount: u128],
                { delegator: AccountId32; candidate: AccountId32; unstakedAmount: u128 }
            >;
            /**
             * Cancelled a pending request to exit the set of delegators.
             **/
            DelegatorExitCancelled: AugmentedEvent<ApiType, [delegator: AccountId32], { delegator: AccountId32 }>;
            /**
             * Delegator requested to leave the set of delegators.
             **/
            DelegatorExitScheduled: AugmentedEvent<
                ApiType,
                [round: u32, delegator: AccountId32, scheduledExit: u32],
                { round: u32; delegator: AccountId32; scheduledExit: u32 }
            >;
            /**
             * Delegator has left the set of delegators.
             **/
            DelegatorLeft: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, unstakedAmount: u128],
                { delegator: AccountId32; unstakedAmount: u128 }
            >;
            /**
             * Delegation from candidate state has been remove.
             **/
            DelegatorLeftCandidate: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, candidate: AccountId32, unstakedAmount: u128, totalCandidateStaked: u128],
                { delegator: AccountId32; candidate: AccountId32; unstakedAmount: u128; totalCandidateStaked: u128 }
            >;
            /**
             * Annual inflation input (first 3) was used to derive new per-round inflation (last 3)
             **/
            InflationSet: AugmentedEvent<
                ApiType,
                [
                    annualMin: Perbill,
                    annualIdeal: Perbill,
                    annualMax: Perbill,
                    roundMin: Perbill,
                    roundIdeal: Perbill,
                    roundMax: Perbill
                ],
                {
                    annualMin: Perbill;
                    annualIdeal: Perbill;
                    annualMax: Perbill;
                    roundMin: Perbill;
                    roundIdeal: Perbill;
                    roundMax: Perbill;
                }
            >;
            /**
             * Account joined the set of collator candidates.
             **/
            JoinedCollatorCandidates: AugmentedEvent<
                ApiType,
                [account: AccountId32, amountLocked: u128, newTotalAmtLocked: u128],
                { account: AccountId32; amountLocked: u128; newTotalAmtLocked: u128 }
            >;
            /**
             * Started new round.
             **/
            NewRound: AugmentedEvent<
                ApiType,
                [startingBlock: u32, round: u32, selectedCollatorsNumber: u32, totalBalance: u128],
                { startingBlock: u32; round: u32; selectedCollatorsNumber: u32; totalBalance: u128 }
            >;
            /**
             * Account (re)set for parachain bond treasury.
             **/
            ParachainBondAccountSet: AugmentedEvent<
                ApiType,
                [old: AccountId32, new_: AccountId32],
                { old: AccountId32; new_: AccountId32 }
            >;
            /**
             * Percent of inflation reserved for parachain bond (re)set.
             **/
            ParachainBondReservePercentSet: AugmentedEvent<
                ApiType,
                [old: Percent, new_: Percent],
                { old: Percent; new_: Percent }
            >;
            /**
             * Transferred to account which holds funds reserved for parachain bond.
             **/
            ReservedForParachainBond: AugmentedEvent<
                ApiType,
                [account: AccountId32, value: u128],
                { account: AccountId32; value: u128 }
            >;
            /**
             * Paid the account (delegator or collator) the balance as liquid rewards.
             **/
            Rewarded: AugmentedEvent<
                ApiType,
                [account: AccountId32, rewards: u128],
                { account: AccountId32; rewards: u128 }
            >;
            /**
             * Staking expectations set.
             **/
            StakeExpectationsSet: AugmentedEvent<
                ApiType,
                [expectMin: u128, expectIdeal: u128, expectMax: u128],
                { expectMin: u128; expectIdeal: u128; expectMax: u128 }
            >;
            /**
             * Set total selected candidates to this value.
             **/
            TotalSelectedSet: AugmentedEvent<ApiType, [old: u32, new_: u32], { old: u32; new_: u32 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        parachainSystem: {
            /**
             * Downward messages were processed using the given weight.
             **/
            DownwardMessagesProcessed: AugmentedEvent<
                ApiType,
                [weightUsed: SpWeightsWeightV2Weight, dmqHead: H256],
                { weightUsed: SpWeightsWeightV2Weight; dmqHead: H256 }
            >;
            /**
             * Some downward messages have been received and will be processed.
             **/
            DownwardMessagesReceived: AugmentedEvent<ApiType, [count: u32], { count: u32 }>;
            /**
             * An upgrade has been authorized.
             **/
            UpgradeAuthorized: AugmentedEvent<ApiType, [codeHash: H256], { codeHash: H256 }>;
            /**
             * An upward message was sent to the relay chain.
             **/
            UpwardMessageSent: AugmentedEvent<
                ApiType,
                [messageHash: Option<U8aFixed>],
                { messageHash: Option<U8aFixed> }
            >;
            /**
             * The validation function was applied as of the contained relay chain block number.
             **/
            ValidationFunctionApplied: AugmentedEvent<ApiType, [relayChainBlockNum: u32], { relayChainBlockNum: u32 }>;
            /**
             * The relay-chain aborted the upgrade process.
             **/
            ValidationFunctionDiscarded: AugmentedEvent<ApiType, []>;
            /**
             * The validation function has been scheduled to apply.
             **/
            ValidationFunctionStored: AugmentedEvent<ApiType, []>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        polkadotXcm: {
            /**
             * Some assets have been claimed from an asset trap
             *
             * \[ hash, origin, assets \]
             **/
            AssetsClaimed: AugmentedEvent<ApiType, [H256, XcmV3MultiLocation, XcmVersionedMultiAssets]>;
            /**
             * Some assets have been placed in an asset trap.
             *
             * \[ hash, origin, assets \]
             **/
            AssetsTrapped: AugmentedEvent<ApiType, [H256, XcmV3MultiLocation, XcmVersionedMultiAssets]>;
            /**
             * Execution of an XCM message was attempted.
             *
             * \[ outcome \]
             **/
            Attempted: AugmentedEvent<ApiType, [XcmV3TraitsOutcome]>;
            /**
             * Fees were paid from a location for an operation (often for using `SendXcm`).
             *
             * \[ paying location, fees \]
             **/
            FeesPaid: AugmentedEvent<ApiType, [XcmV3MultiLocation, XcmV3MultiassetMultiAssets]>;
            /**
             * Expected query response has been received but the querier location of the response does
             * not match the expected. The query remains registered for a later, valid, response to
             * be received and acted upon.
             *
             * \[ origin location, id, expected querier, maybe actual querier \]
             **/
            InvalidQuerier: AugmentedEvent<
                ApiType,
                [XcmV3MultiLocation, u64, XcmV3MultiLocation, Option<XcmV3MultiLocation>]
            >;
            /**
             * Expected query response has been received but the expected querier location placed in
             * storage by this runtime previously cannot be decoded. The query remains registered.
             *
             * This is unexpected (since a location placed in storage in a previously executing
             * runtime should be readable prior to query timeout) and dangerous since the possibly
             * valid response will be dropped. Manual governance intervention is probably going to be
             * needed.
             *
             * \[ origin location, id \]
             **/
            InvalidQuerierVersion: AugmentedEvent<ApiType, [XcmV3MultiLocation, u64]>;
            /**
             * Expected query response has been received but the origin location of the response does
             * not match that expected. The query remains registered for a later, valid, response to
             * be received and acted upon.
             *
             * \[ origin location, id, expected location \]
             **/
            InvalidResponder: AugmentedEvent<ApiType, [XcmV3MultiLocation, u64, Option<XcmV3MultiLocation>]>;
            /**
             * Expected query response has been received but the expected origin location placed in
             * storage by this runtime previously cannot be decoded. The query remains registered.
             *
             * This is unexpected (since a location placed in storage in a previously executing
             * runtime should be readable prior to query timeout) and dangerous since the possibly
             * valid response will be dropped. Manual governance intervention is probably going to be
             * needed.
             *
             * \[ origin location, id \]
             **/
            InvalidResponderVersion: AugmentedEvent<ApiType, [XcmV3MultiLocation, u64]>;
            /**
             * Query response has been received and query is removed. The registered notification has
             * been dispatched and executed successfully.
             *
             * \[ id, pallet index, call index \]
             **/
            Notified: AugmentedEvent<ApiType, [u64, u8, u8]>;
            /**
             * Query response has been received and query is removed. The dispatch was unable to be
             * decoded into a `Call`; this might be due to dispatch function having a signature which
             * is not `(origin, QueryId, Response)`.
             *
             * \[ id, pallet index, call index \]
             **/
            NotifyDecodeFailed: AugmentedEvent<ApiType, [u64, u8, u8]>;
            /**
             * Query response has been received and query is removed. There was a general error with
             * dispatching the notification call.
             *
             * \[ id, pallet index, call index \]
             **/
            NotifyDispatchError: AugmentedEvent<ApiType, [u64, u8, u8]>;
            /**
             * Query response has been received and query is removed. The registered notification could
             * not be dispatched because the dispatch weight is greater than the maximum weight
             * originally budgeted by this runtime for the query result.
             *
             * \[ id, pallet index, call index, actual weight, max budgeted weight \]
             **/
            NotifyOverweight: AugmentedEvent<ApiType, [u64, u8, u8, SpWeightsWeightV2Weight, SpWeightsWeightV2Weight]>;
            /**
             * A given location which had a version change subscription was dropped owing to an error
             * migrating the location to our new XCM format.
             *
             * \[ location, query ID \]
             **/
            NotifyTargetMigrationFail: AugmentedEvent<ApiType, [XcmVersionedMultiLocation, u64]>;
            /**
             * A given location which had a version change subscription was dropped owing to an error
             * sending the notification to it.
             *
             * \[ location, query ID, error \]
             **/
            NotifyTargetSendFail: AugmentedEvent<ApiType, [XcmV3MultiLocation, u64, XcmV3TraitsError]>;
            /**
             * Query response has been received and is ready for taking with `take_response`. There is
             * no registered notification call.
             *
             * \[ id, response \]
             **/
            ResponseReady: AugmentedEvent<ApiType, [u64, XcmV3Response]>;
            /**
             * Received query response has been read and removed.
             *
             * \[ id \]
             **/
            ResponseTaken: AugmentedEvent<ApiType, [u64]>;
            /**
             * A XCM message was sent.
             *
             * \[ origin, destination, message \]
             **/
            Sent: AugmentedEvent<ApiType, [XcmV3MultiLocation, XcmV3MultiLocation, XcmV3Xcm]>;
            /**
             * The supported version of a location has been changed. This might be through an
             * automatic notification or a manual intervention.
             *
             * \[ location, XCM version \]
             **/
            SupportedVersionChanged: AugmentedEvent<ApiType, [XcmV3MultiLocation, u32]>;
            /**
             * Query response received which does not match a registered query. This may be because a
             * matching query was never registered, it may be because it is a duplicate response, or
             * because the query timed out.
             *
             * \[ origin location, id \]
             **/
            UnexpectedResponse: AugmentedEvent<ApiType, [XcmV3MultiLocation, u64]>;
            /**
             * An XCM version change notification message has been attempted to be sent.
             *
             * The cost of sending it (borne by the chain) is included.
             *
             * \[ destination, result, cost \]
             **/
            VersionChangeNotified: AugmentedEvent<ApiType, [XcmV3MultiLocation, u32, XcmV3MultiassetMultiAssets]>;
            /**
             * We have requested that a remote chain sends us XCM version change notifications.
             *
             * \[ destination location, cost \]
             **/
            VersionNotifyRequested: AugmentedEvent<ApiType, [XcmV3MultiLocation, XcmV3MultiassetMultiAssets]>;
            /**
             * A remote has requested XCM version change notification from us and we have honored it.
             * A version information message is sent to them and its cost is included.
             *
             * \[ destination location, cost \]
             **/
            VersionNotifyStarted: AugmentedEvent<ApiType, [XcmV3MultiLocation, XcmV3MultiassetMultiAssets]>;
            /**
             * We have requested that a remote chain stops sending us XCM version change notifications.
             *
             * \[ destination location, cost \]
             **/
            VersionNotifyUnrequested: AugmentedEvent<ApiType, [XcmV3MultiLocation, XcmV3MultiassetMultiAssets]>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        preimage: {
            /**
             * A preimage has ben cleared.
             **/
            Cleared: AugmentedEvent<ApiType, [hash_: H256], { hash_: H256 }>;
            /**
             * A preimage has been noted.
             **/
            Noted: AugmentedEvent<ApiType, [hash_: H256], { hash_: H256 }>;
            /**
             * A preimage has been requested.
             **/
            Requested: AugmentedEvent<ApiType, [hash_: H256], { hash_: H256 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        proxy: {
            /**
             * An announcement was placed to make a call in the future.
             **/
            Announced: AugmentedEvent<
                ApiType,
                [real: AccountId32, proxy: AccountId32, callHash: H256],
                { real: AccountId32; proxy: AccountId32; callHash: H256 }
            >;
            /**
             * A proxy was added.
             **/
            ProxyAdded: AugmentedEvent<
                ApiType,
                [
                    delegator: AccountId32,
                    delegatee: AccountId32,
                    proxyType: RococoParachainRuntimeProxyType,
                    delay: u32
                ],
                {
                    delegator: AccountId32;
                    delegatee: AccountId32;
                    proxyType: RococoParachainRuntimeProxyType;
                    delay: u32;
                }
            >;
            /**
             * A proxy was executed correctly, with the given.
             **/
            ProxyExecuted: AugmentedEvent<
                ApiType,
                [result: Result<Null, SpRuntimeDispatchError>],
                { result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A proxy was removed.
             **/
            ProxyRemoved: AugmentedEvent<
                ApiType,
                [
                    delegator: AccountId32,
                    delegatee: AccountId32,
                    proxyType: RococoParachainRuntimeProxyType,
                    delay: u32
                ],
                {
                    delegator: AccountId32;
                    delegatee: AccountId32;
                    proxyType: RococoParachainRuntimeProxyType;
                    delay: u32;
                }
            >;
            /**
             * A pure account has been created by new proxy with given
             * disambiguation index and proxy type.
             **/
            PureCreated: AugmentedEvent<
                ApiType,
                [
                    pure: AccountId32,
                    who: AccountId32,
                    proxyType: RococoParachainRuntimeProxyType,
                    disambiguationIndex: u16
                ],
                {
                    pure: AccountId32;
                    who: AccountId32;
                    proxyType: RococoParachainRuntimeProxyType;
                    disambiguationIndex: u16;
                }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        scheduler: {
            /**
             * The call for the provided hash was not found so the task has been aborted.
             **/
            CallUnavailable: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed> }
            >;
            /**
             * Canceled some task.
             **/
            Canceled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32; index: u32 }>;
            /**
             * Dispatched some task.
             **/
            Dispatched: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>, result: Result<Null, SpRuntimeDispatchError>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed>; result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * The given task was unable to be renewed since the agenda is full at that block.
             **/
            PeriodicFailed: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed> }
            >;
            /**
             * The given task can never be executed since it is overweight.
             **/
            PermanentlyOverweight: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed> }
            >;
            /**
             * Scheduled some task.
             **/
            Scheduled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32; index: u32 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        session: {
            /**
             * New session has happened. Note that the argument is the session index, not the
             * block number as the type might suggest.
             **/
            NewSession: AugmentedEvent<ApiType, [sessionIndex: u32], { sessionIndex: u32 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        sidechain: {
            FinalizedSidechainBlock: AugmentedEvent<ApiType, [AccountId32, H256]>;
            ProposedSidechainBlock: AugmentedEvent<ApiType, [AccountId32, H256]>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        sudo: {
            /**
             * The \[sudoer\] just switched identity; the old key is supplied if one existed.
             **/
            KeyChanged: AugmentedEvent<ApiType, [oldSudoer: Option<AccountId32>], { oldSudoer: Option<AccountId32> }>;
            /**
             * A sudo just took place. \[result\]
             **/
            Sudid: AugmentedEvent<
                ApiType,
                [sudoResult: Result<Null, SpRuntimeDispatchError>],
                { sudoResult: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A sudo just took place. \[result\]
             **/
            SudoAsDone: AugmentedEvent<
                ApiType,
                [sudoResult: Result<Null, SpRuntimeDispatchError>],
                { sudoResult: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        system: {
            /**
             * `:code` was updated.
             **/
            CodeUpdated: AugmentedEvent<ApiType, []>;
            /**
             * An extrinsic failed.
             **/
            ExtrinsicFailed: AugmentedEvent<
                ApiType,
                [dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportDispatchDispatchInfo],
                { dispatchError: SpRuntimeDispatchError; dispatchInfo: FrameSupportDispatchDispatchInfo }
            >;
            /**
             * An extrinsic completed successfully.
             **/
            ExtrinsicSuccess: AugmentedEvent<
                ApiType,
                [dispatchInfo: FrameSupportDispatchDispatchInfo],
                { dispatchInfo: FrameSupportDispatchDispatchInfo }
            >;
            /**
             * An account was reaped.
             **/
            KilledAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            /**
             * A new account was created.
             **/
            NewAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            /**
             * On on-chain remark happened.
             **/
            Remarked: AugmentedEvent<ApiType, [sender: AccountId32, hash_: H256], { sender: AccountId32; hash_: H256 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        technicalCommittee: {
            /**
             * A motion was approved by the required threshold.
             **/
            Approved: AugmentedEvent<ApiType, [proposalHash: H256], { proposalHash: H256 }>;
            /**
             * A proposal was closed because its threshold was reached or after its duration was up.
             **/
            Closed: AugmentedEvent<
                ApiType,
                [proposalHash: H256, yes: u32, no: u32],
                { proposalHash: H256; yes: u32; no: u32 }
            >;
            /**
             * A motion was not approved by the required threshold.
             **/
            Disapproved: AugmentedEvent<ApiType, [proposalHash: H256], { proposalHash: H256 }>;
            /**
             * A motion was executed; result will be `Ok` if it returned without error.
             **/
            Executed: AugmentedEvent<
                ApiType,
                [proposalHash: H256, result: Result<Null, SpRuntimeDispatchError>],
                { proposalHash: H256; result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A single member did some action; result will be `Ok` if it returned without error.
             **/
            MemberExecuted: AugmentedEvent<
                ApiType,
                [proposalHash: H256, result: Result<Null, SpRuntimeDispatchError>],
                { proposalHash: H256; result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A motion (given hash) has been proposed (by given account) with a threshold (given
             * `MemberCount`).
             **/
            Proposed: AugmentedEvent<
                ApiType,
                [account: AccountId32, proposalIndex: u32, proposalHash: H256, threshold: u32],
                { account: AccountId32; proposalIndex: u32; proposalHash: H256; threshold: u32 }
            >;
            /**
             * A motion (given hash) has been voted on by given account, leaving
             * a tally (yes votes and no votes given respectively as `MemberCount`).
             **/
            Voted: AugmentedEvent<
                ApiType,
                [account: AccountId32, proposalHash: H256, voted: bool, yes: u32, no: u32],
                { account: AccountId32; proposalHash: H256; voted: bool; yes: u32; no: u32 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        technicalCommitteeMembership: {
            /**
             * Phantom member, never used.
             **/
            Dummy: AugmentedEvent<ApiType, []>;
            /**
             * One of the members' keys changed.
             **/
            KeyChanged: AugmentedEvent<ApiType, []>;
            /**
             * The given member was added; see the transaction for who.
             **/
            MemberAdded: AugmentedEvent<ApiType, []>;
            /**
             * The given member was removed; see the transaction for who.
             **/
            MemberRemoved: AugmentedEvent<ApiType, []>;
            /**
             * The membership was reset; see the transaction for who the new set is.
             **/
            MembersReset: AugmentedEvent<ApiType, []>;
            /**
             * Two members were swapped; see the transaction for who.
             **/
            MembersSwapped: AugmentedEvent<ApiType, []>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        teeracle: {
            AddedToWhitelist: AugmentedEvent<ApiType, [Bytes, U8aFixed]>;
            ExchangeRateDeleted: AugmentedEvent<ApiType, [Bytes, Bytes]>;
            /**
             * The exchange rate of trading pair was set/updated with value from source.
             * \[data_source], [trading_pair], [new value\]
             **/
            ExchangeRateUpdated: AugmentedEvent<ApiType, [Bytes, Bytes, Option<SubstrateFixedFixedU64>]>;
            OracleUpdated: AugmentedEvent<ApiType, [Bytes, Bytes]>;
            RemovedFromWhitelist: AugmentedEvent<ApiType, [Bytes, U8aFixed]>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        teerex: {
            AddedEnclave: AugmentedEvent<ApiType, [AccountId32, Bytes]>;
            AdminChanged: AugmentedEvent<ApiType, [oldAdmin: Option<AccountId32>], { oldAdmin: Option<AccountId32> }>;
            Forwarded: AugmentedEvent<ApiType, [H256]>;
            ProcessedParentchainBlock: AugmentedEvent<ApiType, [AccountId32, H256, H256, u32]>;
            /**
             * An enclave with [mr_enclave] has published some [hash] with some metadata [data].
             **/
            PublishedHash: AugmentedEvent<
                ApiType,
                [mrEnclave: U8aFixed, hash_: H256, data: Bytes],
                { mrEnclave: U8aFixed; hash_: H256; data: Bytes }
            >;
            RemovedEnclave: AugmentedEvent<ApiType, [AccountId32]>;
            RemovedScheduledEnclave: AugmentedEvent<ApiType, [u64]>;
            SetHeartbeatTimeout: AugmentedEvent<ApiType, [u64]>;
            ShieldFunds: AugmentedEvent<ApiType, [Bytes]>;
            UnshieldedFunds: AugmentedEvent<ApiType, [AccountId32]>;
            UpdatedScheduledEnclave: AugmentedEvent<ApiType, [u64, U8aFixed]>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        tips: {
            /**
             * A new tip suggestion has been opened.
             **/
            NewTip: AugmentedEvent<ApiType, [tipHash: H256], { tipHash: H256 }>;
            /**
             * A tip suggestion has been closed.
             **/
            TipClosed: AugmentedEvent<
                ApiType,
                [tipHash: H256, who: AccountId32, payout: u128],
                { tipHash: H256; who: AccountId32; payout: u128 }
            >;
            /**
             * A tip suggestion has reached threshold and is closing.
             **/
            TipClosing: AugmentedEvent<ApiType, [tipHash: H256], { tipHash: H256 }>;
            /**
             * A tip suggestion has been retracted.
             **/
            TipRetracted: AugmentedEvent<ApiType, [tipHash: H256], { tipHash: H256 }>;
            /**
             * A tip suggestion has been slashed.
             **/
            TipSlashed: AugmentedEvent<
                ApiType,
                [tipHash: H256, finder: AccountId32, deposit: u128],
                { tipHash: H256; finder: AccountId32; deposit: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        tokens: {
            /**
             * A balance was set by root.
             **/
            BalanceSet: AugmentedEvent<
                ApiType,
                [currencyId: u128, who: AccountId32, free: u128, reserved: u128],
                { currencyId: u128; who: AccountId32; free: u128; reserved: u128 }
            >;
            /**
             * Deposited some balance into an account
             **/
            Deposited: AugmentedEvent<
                ApiType,
                [currencyId: u128, who: AccountId32, amount: u128],
                { currencyId: u128; who: AccountId32; amount: u128 }
            >;
            /**
             * An account was removed whose balance was non-zero but below
             * ExistentialDeposit, resulting in an outright loss.
             **/
            DustLost: AugmentedEvent<
                ApiType,
                [currencyId: u128, who: AccountId32, amount: u128],
                { currencyId: u128; who: AccountId32; amount: u128 }
            >;
            /**
             * An account was created with some free balance.
             **/
            Endowed: AugmentedEvent<
                ApiType,
                [currencyId: u128, who: AccountId32, amount: u128],
                { currencyId: u128; who: AccountId32; amount: u128 }
            >;
            /**
             * Some free balance was locked.
             **/
            Locked: AugmentedEvent<
                ApiType,
                [currencyId: u128, who: AccountId32, amount: u128],
                { currencyId: u128; who: AccountId32; amount: u128 }
            >;
            /**
             * Some locked funds were unlocked
             **/
            LockRemoved: AugmentedEvent<
                ApiType,
                [lockId: U8aFixed, currencyId: u128, who: AccountId32],
                { lockId: U8aFixed; currencyId: u128; who: AccountId32 }
            >;
            /**
             * Some funds are locked
             **/
            LockSet: AugmentedEvent<
                ApiType,
                [lockId: U8aFixed, currencyId: u128, who: AccountId32, amount: u128],
                { lockId: U8aFixed; currencyId: u128; who: AccountId32; amount: u128 }
            >;
            /**
             * Some balance was reserved (moved from free to reserved).
             **/
            Reserved: AugmentedEvent<
                ApiType,
                [currencyId: u128, who: AccountId32, amount: u128],
                { currencyId: u128; who: AccountId32; amount: u128 }
            >;
            /**
             * Some reserved balance was repatriated (moved from reserved to
             * another account).
             **/
            ReserveRepatriated: AugmentedEvent<
                ApiType,
                [
                    currencyId: u128,
                    from: AccountId32,
                    to: AccountId32,
                    amount: u128,
                    status: FrameSupportTokensMiscBalanceStatus
                ],
                {
                    currencyId: u128;
                    from: AccountId32;
                    to: AccountId32;
                    amount: u128;
                    status: FrameSupportTokensMiscBalanceStatus;
                }
            >;
            /**
             * Some balances were slashed (e.g. due to mis-behavior)
             **/
            Slashed: AugmentedEvent<
                ApiType,
                [currencyId: u128, who: AccountId32, freeAmount: u128, reservedAmount: u128],
                { currencyId: u128; who: AccountId32; freeAmount: u128; reservedAmount: u128 }
            >;
            /**
             * The total issuance of an currency has been set
             **/
            TotalIssuanceSet: AugmentedEvent<
                ApiType,
                [currencyId: u128, amount: u128],
                { currencyId: u128; amount: u128 }
            >;
            /**
             * Transfer succeeded.
             **/
            Transfer: AugmentedEvent<
                ApiType,
                [currencyId: u128, from: AccountId32, to: AccountId32, amount: u128],
                { currencyId: u128; from: AccountId32; to: AccountId32; amount: u128 }
            >;
            /**
             * Some locked balance was freed.
             **/
            Unlocked: AugmentedEvent<
                ApiType,
                [currencyId: u128, who: AccountId32, amount: u128],
                { currencyId: u128; who: AccountId32; amount: u128 }
            >;
            /**
             * Some balance was unreserved (moved from reserved to free).
             **/
            Unreserved: AugmentedEvent<
                ApiType,
                [currencyId: u128, who: AccountId32, amount: u128],
                { currencyId: u128; who: AccountId32; amount: u128 }
            >;
            /**
             * Some balances were withdrawn (e.g. pay for transaction fee)
             **/
            Withdrawn: AugmentedEvent<
                ApiType,
                [currencyId: u128, who: AccountId32, amount: u128],
                { currencyId: u128; who: AccountId32; amount: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        transactionPayment: {
            /**
             * A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,
             * has been paid by `who`.
             **/
            TransactionFeePaid: AugmentedEvent<
                ApiType,
                [who: AccountId32, actualFee: u128, tip: u128],
                { who: AccountId32; actualFee: u128; tip: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        treasury: {
            /**
             * Some funds have been allocated.
             **/
            Awarded: AugmentedEvent<
                ApiType,
                [proposalIndex: u32, award: u128, account: AccountId32],
                { proposalIndex: u32; award: u128; account: AccountId32 }
            >;
            /**
             * Some of our funds have been burnt.
             **/
            Burnt: AugmentedEvent<ApiType, [burntFunds: u128], { burntFunds: u128 }>;
            /**
             * Some funds have been deposited.
             **/
            Deposit: AugmentedEvent<ApiType, [value: u128], { value: u128 }>;
            /**
             * New proposal.
             **/
            Proposed: AugmentedEvent<ApiType, [proposalIndex: u32], { proposalIndex: u32 }>;
            /**
             * A proposal was rejected; funds were slashed.
             **/
            Rejected: AugmentedEvent<
                ApiType,
                [proposalIndex: u32, slashed: u128],
                { proposalIndex: u32; slashed: u128 }
            >;
            /**
             * Spending has finished; this is the amount that rolls over until next spend.
             **/
            Rollover: AugmentedEvent<ApiType, [rolloverBalance: u128], { rolloverBalance: u128 }>;
            /**
             * A new spend proposal has been approved.
             **/
            SpendApproved: AugmentedEvent<
                ApiType,
                [proposalIndex: u32, amount: u128, beneficiary: AccountId32],
                { proposalIndex: u32; amount: u128; beneficiary: AccountId32 }
            >;
            /**
             * We have ended a spend period and will now allocate funds.
             **/
            Spending: AugmentedEvent<ApiType, [budgetRemaining: u128], { budgetRemaining: u128 }>;
            /**
             * The inactive funds of the pallet have been updated.
             **/
            UpdatedInactive: AugmentedEvent<
                ApiType,
                [reactivated: u128, deactivated: u128],
                { reactivated: u128; deactivated: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        utility: {
            /**
             * Batch of dispatches completed fully with no error.
             **/
            BatchCompleted: AugmentedEvent<ApiType, []>;
            /**
             * Batch of dispatches completed but has errors.
             **/
            BatchCompletedWithErrors: AugmentedEvent<ApiType, []>;
            /**
             * Batch of dispatches did not complete fully. Index of first failing dispatch given, as
             * well as the error.
             **/
            BatchInterrupted: AugmentedEvent<
                ApiType,
                [index: u32, error: SpRuntimeDispatchError],
                { index: u32; error: SpRuntimeDispatchError }
            >;
            /**
             * A call was dispatched.
             **/
            DispatchedAs: AugmentedEvent<
                ApiType,
                [result: Result<Null, SpRuntimeDispatchError>],
                { result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A single item within a Batch of dispatches has completed with no error.
             **/
            ItemCompleted: AugmentedEvent<ApiType, []>;
            /**
             * A single item within a Batch of dispatches has completed with error.
             **/
            ItemFailed: AugmentedEvent<ApiType, [error: SpRuntimeDispatchError], { error: SpRuntimeDispatchError }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        vcManagement: {
            AdminChanged: AugmentedEvent<
                ApiType,
                [oldAdmin: Option<AccountId32>, newAdmin: Option<AccountId32>],
                { oldAdmin: Option<AccountId32>; newAdmin: Option<AccountId32> }
            >;
            RequestVCFailed: AugmentedEvent<
                ApiType,
                [
                    account: Option<AccountId32>,
                    assertion: CorePrimitivesAssertion,
                    detail: CorePrimitivesErrorErrorDetail,
                    reqExtHash: H256
                ],
                {
                    account: Option<AccountId32>;
                    assertion: CorePrimitivesAssertion;
                    detail: CorePrimitivesErrorErrorDetail;
                    reqExtHash: H256;
                }
            >;
            SchemaActivated: AugmentedEvent<
                ApiType,
                [account: AccountId32, shard: H256, index: u64],
                { account: AccountId32; shard: H256; index: u64 }
            >;
            SchemaDisabled: AugmentedEvent<
                ApiType,
                [account: AccountId32, shard: H256, index: u64],
                { account: AccountId32; shard: H256; index: u64 }
            >;
            SchemaIssued: AugmentedEvent<
                ApiType,
                [account: AccountId32, shard: H256, index: u64],
                { account: AccountId32; shard: H256; index: u64 }
            >;
            SchemaRevoked: AugmentedEvent<
                ApiType,
                [account: AccountId32, shard: H256, index: u64],
                { account: AccountId32; shard: H256; index: u64 }
            >;
            UnclassifiedError: AugmentedEvent<
                ApiType,
                [account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256],
                { account: Option<AccountId32>; detail: CorePrimitivesErrorErrorDetail; reqExtHash: H256 }
            >;
            VCDisabled: AugmentedEvent<
                ApiType,
                [account: AccountId32, index: H256],
                { account: AccountId32; index: H256 }
            >;
            VCIssued: AugmentedEvent<
                ApiType,
                [
                    account: AccountId32,
                    assertion: CorePrimitivesAssertion,
                    index: H256,
                    vc: CorePrimitivesKeyAesOutput,
                    reqExtHash: H256
                ],
                {
                    account: AccountId32;
                    assertion: CorePrimitivesAssertion;
                    index: H256;
                    vc: CorePrimitivesKeyAesOutput;
                    reqExtHash: H256;
                }
            >;
            VCRegistryCleared: AugmentedEvent<ApiType, []>;
            VCRegistryItemAdded: AugmentedEvent<
                ApiType,
                [account: AccountId32, assertion: CorePrimitivesAssertion, index: H256],
                { account: AccountId32; assertion: CorePrimitivesAssertion; index: H256 }
            >;
            VCRegistryItemRemoved: AugmentedEvent<ApiType, [index: H256], { index: H256 }>;
            VCRequested: AugmentedEvent<
                ApiType,
                [account: AccountId32, shard: H256, assertion: CorePrimitivesAssertion],
                { account: AccountId32; shard: H256; assertion: CorePrimitivesAssertion }
            >;
            VCRevoked: AugmentedEvent<
                ApiType,
                [account: AccountId32, index: H256],
                { account: AccountId32; index: H256 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        vcmpExtrinsicWhitelist: {
            /**
             * Group member added to set
             **/
            GroupMemberAdded: AugmentedEvent<ApiType, [AccountId32]>;
            /**
             * Group member removed from set
             **/
            GroupMemberRemoved: AugmentedEvent<ApiType, [AccountId32]>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        vesting: {
            /**
             * An \[account\] has become fully vested.
             **/
            VestingCompleted: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            /**
             * The amount vested has been updated. This could indicate a change in funds available.
             * The balance given is the amount which is left unvested (and thus locked).
             **/
            VestingUpdated: AugmentedEvent<
                ApiType,
                [account: AccountId32, unvested: u128],
                { account: AccountId32; unvested: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        xcmpQueue: {
            /**
             * Bad XCM format used.
             **/
            BadFormat: AugmentedEvent<ApiType, [messageHash: Option<U8aFixed>], { messageHash: Option<U8aFixed> }>;
            /**
             * Bad XCM version used.
             **/
            BadVersion: AugmentedEvent<ApiType, [messageHash: Option<U8aFixed>], { messageHash: Option<U8aFixed> }>;
            /**
             * Some XCM failed.
             **/
            Fail: AugmentedEvent<
                ApiType,
                [messageHash: Option<U8aFixed>, error: XcmV3TraitsError, weight: SpWeightsWeightV2Weight],
                { messageHash: Option<U8aFixed>; error: XcmV3TraitsError; weight: SpWeightsWeightV2Weight }
            >;
            /**
             * An XCM exceeded the individual message weight budget.
             **/
            OverweightEnqueued: AugmentedEvent<
                ApiType,
                [sender: u32, sentAt: u32, index: u64, required: SpWeightsWeightV2Weight],
                { sender: u32; sentAt: u32; index: u64; required: SpWeightsWeightV2Weight }
            >;
            /**
             * An XCM from the overweight queue was executed with the given actual weight used.
             **/
            OverweightServiced: AugmentedEvent<
                ApiType,
                [index: u64, used: SpWeightsWeightV2Weight],
                { index: u64; used: SpWeightsWeightV2Weight }
            >;
            /**
             * Some XCM was executed ok.
             **/
            Success: AugmentedEvent<
                ApiType,
                [messageHash: Option<U8aFixed>, weight: SpWeightsWeightV2Weight],
                { messageHash: Option<U8aFixed>; weight: SpWeightsWeightV2Weight }
            >;
            /**
             * An HRMP message was sent to a sibling parachain.
             **/
            XcmpMessageSent: AugmentedEvent<
                ApiType,
                [messageHash: Option<U8aFixed>],
                { messageHash: Option<U8aFixed> }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        xTokens: {
            /**
             * Transferred `MultiAsset` with fee.
             **/
            TransferredMultiAssets: AugmentedEvent<
                ApiType,
                [
                    sender: AccountId32,
                    assets: XcmV3MultiassetMultiAssets,
                    fee: XcmV3MultiAsset,
                    dest: XcmV3MultiLocation
                ],
                {
                    sender: AccountId32;
                    assets: XcmV3MultiassetMultiAssets;
                    fee: XcmV3MultiAsset;
                    dest: XcmV3MultiLocation;
                }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
    } // AugmentedEvents
} // declare module
