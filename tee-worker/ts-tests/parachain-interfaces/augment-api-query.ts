// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/storage';

import type { ApiTypes, AugmentedQuery, QueryableStorageEntry } from '@polkadot/api-base/types';
import type { Data } from '@polkadot/types';
import type {
    BTreeMap,
    Bytes,
    Null,
    Option,
    U8aFixed,
    Vec,
    bool,
    u128,
    u16,
    u32,
    u64,
    u8,
} from '@polkadot/types-codec';
import type { AnyNumber, IMethod, ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, Call, H256, Perbill } from '@polkadot/types/interfaces/runtime';
import type {
    CumulusPalletDmpQueueConfigData,
    CumulusPalletDmpQueuePageIndexData,
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot,
    CumulusPalletXcmpQueueInboundChannelDetails,
    CumulusPalletXcmpQueueOutboundChannelDetails,
    CumulusPalletXcmpQueueQueueConfigData,
    FrameSupportDispatchPerDispatchClassWeight,
    FrameSupportPreimagesBounded,
    FrameSystemAccountInfo,
    FrameSystemEventRecord,
    FrameSystemLastRuntimeUpgradeInfo,
    FrameSystemPhase,
    MockTeePrimitivesIdentity,
    OrmlTokensAccountData,
    OrmlTokensBalanceLock,
    OrmlTokensReserveData,
    PalletAssetManagerAssetMetadata,
    PalletBalancesAccountData,
    PalletBalancesBalanceLock,
    PalletBalancesReserveData,
    PalletBountiesBounty,
    PalletBridgeBridgeEvent,
    PalletBridgeProposalVotes,
    PalletCollectiveVotes,
    PalletDemocracyMetadataOwner,
    PalletDemocracyReferendumInfo,
    PalletDemocracyVoteThreshold,
    PalletDemocracyVoteVoting,
    PalletDrop3RewardPool,
    PalletExtrinsicFilterOperationalMode,
    PalletIdentityManagementMockIdentityContext,
    PalletIdentityRegistrarInfo,
    PalletIdentityRegistration,
    PalletMultisigMultisig,
    PalletParachainStakingAutoCompoundAutoCompoundConfig,
    PalletParachainStakingBond,
    PalletParachainStakingCandidateMetadata,
    PalletParachainStakingCollatorSnapshot,
    PalletParachainStakingDelayedPayout,
    PalletParachainStakingDelegationRequestsScheduledRequest,
    PalletParachainStakingDelegations,
    PalletParachainStakingDelegator,
    PalletParachainStakingInflationInflationInfo,
    PalletParachainStakingParachainBondConfig,
    PalletParachainStakingRoundInfo,
    PalletParachainStakingSetOrderedSet,
    PalletPreimageRequestStatus,
    PalletProxyAnnouncement,
    PalletProxyProxyDefinition,
    PalletSchedulerScheduled,
    PalletTipsOpenTip,
    PalletTransactionPaymentReleases,
    PalletTreasuryProposal,
    PalletVcManagementSchemaVcSchema,
    PalletVcManagementVcContext,
    PalletVestingReleases,
    PalletVestingVestingInfo,
    PalletXcmQueryStatus,
    PalletXcmRemoteLockedFungibleRecord,
    PalletXcmVersionMigrationStage,
    PolkadotCorePrimitivesOutboundHrmpMessage,
    PolkadotPrimitivesV2AbridgedHostConfiguration,
    PolkadotPrimitivesV2PersistedValidationData,
    PolkadotPrimitivesV2UpgradeRestriction,
    RococoParachainRuntimeSessionKeys,
    RuntimeCommonXcmImplCurrencyId,
    SidechainPrimitivesSidechainBlockConfirmation,
    SpConsensusAuraSr25519AppSr25519Public,
    SpCoreCryptoKeyTypeId,
    SpRuntimeDigest,
    SpTrieStorageProof,
    SpWeightsWeightV2Weight,
    SubstrateFixedFixedU64,
    TeerexPrimitivesEnclave,
    TeerexPrimitivesQuotingEnclave,
    TeerexPrimitivesTcbInfoOnChain,
    XcmVersionedAssetId,
    XcmVersionedMultiLocation,
} from '@polkadot/types/lookup';
import type { Observable } from '@polkadot/types/types';

export type __AugmentedQuery<ApiType extends ApiTypes> = AugmentedQuery<ApiType, () => unknown>;
export type __QueryableStorageEntry<ApiType extends ApiTypes> = QueryableStorageEntry<ApiType>;

declare module '@polkadot/api-base/types/storage' {
    interface AugmentedQueries<ApiType extends ApiTypes> {
        assetManager: {
            /**
             * The storages for AssetIdMetadata.
             * AssetIdMetadata: map AssetId => Option<AssetMetadata>
             **/
            assetIdMetadata: AugmentedQuery<
                ApiType,
                (arg: u128 | AnyNumber | Uint8Array) => Observable<Option<PalletAssetManagerAssetMetadata>>,
                [u128]
            > &
                QueryableStorageEntry<ApiType, [u128]>;
            /**
             * Mapping from an asset id to asset type.
             * This is mostly used when receiving transaction specifying an asset directly,
             * like transferring an asset from this chain to another.
             **/
            assetIdType: AugmentedQuery<
                ApiType,
                (arg: u128 | AnyNumber | Uint8Array) => Observable<Option<RuntimeCommonXcmImplCurrencyId>>,
                [u128]
            > &
                QueryableStorageEntry<ApiType, [u128]>;
            /**
             * Stores the units per second for local execution for a AssetType.
             * This is used to know how to charge for XCM execution in a particular asset
             * Not all assets might contain units per second, hence the different storage
             **/
            assetIdUnitsPerSecond: AugmentedQuery<
                ApiType,
                (arg: u128 | AnyNumber | Uint8Array) => Observable<u128>,
                [u128]
            > &
                QueryableStorageEntry<ApiType, [u128]>;
            /**
             * Reverse mapping of AssetIdType. Mapping from an asset type to an asset id.
             * This is mostly used when receiving a multilocation XCM message to retrieve
             * the corresponding asset in which tokens should me minted.
             **/
            assetTypeId: AugmentedQuery<
                ApiType,
                (
                    arg:
                        | RuntimeCommonXcmImplCurrencyId
                        | { SelfReserve: any }
                        | { ParachainReserve: any }
                        | string
                        | Uint8Array
                ) => Observable<Option<u128>>,
                [RuntimeCommonXcmImplCurrencyId]
            > &
                QueryableStorageEntry<ApiType, [RuntimeCommonXcmImplCurrencyId]>;
            /**
             * Stores the tracker of foreign assets id that have been
             * created so far
             **/
            foreignAssetTracker: AugmentedQuery<ApiType, () => Observable<u128>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        aura: {
            /**
             * The current authority set.
             **/
            authorities: AugmentedQuery<ApiType, () => Observable<Vec<SpConsensusAuraSr25519AppSr25519Public>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The current slot of this block.
             *
             * This will be set in `on_initialize`.
             **/
            currentSlot: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        auraExt: {
            /**
             * Serves as cache for the authorities.
             *
             * The authorities in AuRa are overwritten in `on_initialize` when we switch to a new session,
             * but we require the old authorities to verify the seal when validating a PoV. This will always
             * be updated to the latest AuRa authorities in `on_finalize`.
             **/
            authorities: AugmentedQuery<ApiType, () => Observable<Vec<SpConsensusAuraSr25519AppSr25519Public>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        authorship: {
            /**
             * Author of current block.
             **/
            author: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        balances: {
            /**
             * The Balances pallet example of storing the balance of an account.
             *
             * # Example
             *
             * ```nocompile
             * impl pallet_balances::Config for Runtime {
             * type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>
             * }
             * ```
             *
             * You can also store the balance of an account in the `System` pallet.
             *
             * # Example
             *
             * ```nocompile
             * impl pallet_balances::Config for Runtime {
             * type AccountStore = System
             * }
             * ```
             *
             * But this comes with tradeoffs, storing account balances in the system pallet stores
             * `frame_system` data alongside the account data contrary to storing account balances in the
             * `Balances` pallet, which uses a `StorageMap` to store balances data only.
             * NOTE: This is only used in the case that this pallet is used to store balances.
             **/
            account: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<PalletBalancesAccountData>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * The total units of outstanding deactivated balance in the system.
             **/
            inactiveIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Any liquidity locks on some account balances.
             * NOTE: Should only be accessed when setting, changing and freeing a lock.
             **/
            locks: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesBalanceLock>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Named reserves on some account balances.
             **/
            reserves: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesReserveData>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * The total units issued in the system.
             **/
            totalIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        bounties: {
            /**
             * Bounties that have been made.
             **/
            bounties: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletBountiesBounty>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Bounty indices that have been approved but not yet funded.
             **/
            bountyApprovals: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Number of bounty proposals that have been made.
             **/
            bountyCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The description of each bounty.
             **/
            bountyDescriptions: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<Bytes>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        bridgeTransfer: {
            bridgeBalances: AugmentedQuery<
                ApiType,
                (
                    arg1: U8aFixed | string | Uint8Array,
                    arg2: AccountId32 | string | Uint8Array
                ) => Observable<Option<u128>>,
                [U8aFixed, AccountId32]
            > &
                QueryableStorageEntry<ApiType, [U8aFixed, AccountId32]>;
            externalBalances: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            maximumIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        chainBridge: {
            bridgeEvents: AugmentedQuery<ApiType, () => Observable<Vec<PalletBridgeBridgeEvent>>, []> &
                QueryableStorageEntry<ApiType, []>;
            bridgeFee: AugmentedQuery<ApiType, (arg: u8 | AnyNumber | Uint8Array) => Observable<Option<u128>>, [u8]> &
                QueryableStorageEntry<ApiType, [u8]>;
            chainNonces: AugmentedQuery<ApiType, (arg: u8 | AnyNumber | Uint8Array) => Observable<Option<u64>>, [u8]> &
                QueryableStorageEntry<ApiType, [u8]>;
            relayerCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            relayers: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<bool>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            relayerThreshold: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            resources: AugmentedQuery<
                ApiType,
                (arg: U8aFixed | string | Uint8Array) => Observable<Option<Bytes>>,
                [U8aFixed]
            > &
                QueryableStorageEntry<ApiType, [U8aFixed]>;
            votes: AugmentedQuery<
                ApiType,
                (
                    arg1: u8 | AnyNumber | Uint8Array,
                    arg2: ITuple<[u64, Call]> | [u64 | AnyNumber | Uint8Array, Call | IMethod | string | Uint8Array]
                ) => Observable<Option<PalletBridgeProposalVotes>>,
                [u8, ITuple<[u64, Call]>]
            > &
                QueryableStorageEntry<ApiType, [u8, ITuple<[u64, Call]>]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        council: {
            /**
             * The current members of the collective. This is stored sorted (just by value).
             **/
            members: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The prime member that helps determine the default vote behavior in case of absentations.
             **/
            prime: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Proposals so far.
             **/
            proposalCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Actual proposal for a given hash, if it's current.
             **/
            proposalOf: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<Call>>, [H256]> &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * The hashes of the active proposals.
             **/
            proposals: AugmentedQuery<ApiType, () => Observable<Vec<H256>>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Votes on a given proposal, if it is ongoing.
             **/
            voting: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<PalletCollectiveVotes>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        councilMembership: {
            /**
             * The current membership, stored as an ordered Vec.
             **/
            members: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The current prime member, if one exists.
             **/
            prime: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        democracy: {
            /**
             * A record of who vetoed what. Maps proposal hash to a possible existent block number
             * (until when it may not be resubmitted) and who vetoed it.
             **/
            blacklist: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<ITuple<[u32, Vec<AccountId32>]>>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * Record of all proposals that have been subject to emergency cancellation.
             **/
            cancellations: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<bool>, [H256]> &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * Those who have locked a deposit.
             *
             * TWOX-NOTE: Safe, as increasing integer keys are safe.
             **/
            depositOf: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<ITuple<[Vec<AccountId32>, u128]>>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * True if the last referendum tabled was submitted externally. False if it was a public
             * proposal.
             **/
            lastTabledWasExternal: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The lowest referendum index representing an unbaked referendum. Equal to
             * `ReferendumCount` if there isn't a unbaked referendum.
             **/
            lowestUnbaked: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * General information concerning any proposal or referendum.
             * The `PreimageHash` refers to the preimage of the `Preimages` provider which can be a JSON
             * dump or IPFS hash of a JSON file.
             *
             * Consider a garbage collection for a metadata of finished referendums to `unrequest` (remove)
             * large preimages.
             **/
            metadataOf: AugmentedQuery<
                ApiType,
                (
                    arg:
                        | PalletDemocracyMetadataOwner
                        | { External: any }
                        | { Proposal: any }
                        | { Referendum: any }
                        | string
                        | Uint8Array
                ) => Observable<Option<H256>>,
                [PalletDemocracyMetadataOwner]
            > &
                QueryableStorageEntry<ApiType, [PalletDemocracyMetadataOwner]>;
            /**
             * The referendum to be tabled whenever it would be valid to table an external proposal.
             * This happens when a referendum needs to be tabled and one of two conditions are met:
             * - `LastTabledWasExternal` is `false`; or
             * - `PublicProps` is empty.
             **/
            nextExternal: AugmentedQuery<
                ApiType,
                () => Observable<Option<ITuple<[FrameSupportPreimagesBounded, PalletDemocracyVoteThreshold]>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The number of (public) proposals that have been made so far.
             **/
            publicPropCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The public proposals. Unsorted. The second item is the proposal.
             **/
            publicProps: AugmentedQuery<
                ApiType,
                () => Observable<Vec<ITuple<[u32, FrameSupportPreimagesBounded, AccountId32]>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The next free referendum index, aka the number of referenda started so far.
             **/
            referendumCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Information concerning any given referendum.
             *
             * TWOX-NOTE: SAFE as indexes are not under an attacker’s control.
             **/
            referendumInfoOf: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletDemocracyReferendumInfo>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * All votes for a particular voter. We store the balance for the number of votes that we
             * have recorded. The second item is the total amount of delegations, that will be added.
             *
             * TWOX-NOTE: SAFE as `AccountId`s are crypto hashes anyway.
             **/
            votingOf: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<PalletDemocracyVoteVoting>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        dmpQueue: {
            /**
             * The configuration.
             **/
            configuration: AugmentedQuery<ApiType, () => Observable<CumulusPalletDmpQueueConfigData>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Counter for the related counted storage map
             **/
            counterForOverweight: AugmentedQuery<ApiType, () => Observable<u32>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The overweight messages.
             **/
            overweight: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<ITuple<[u32, Bytes]>>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
            /**
             * The page index.
             **/
            pageIndex: AugmentedQuery<ApiType, () => Observable<CumulusPalletDmpQueuePageIndexData>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The queue pages.
             **/
            pages: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<ITuple<[u32, Bytes]>>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        drop3: {
            /**
             * The reward pool admin account
             * The reason why such an account is needed (other than just using ROOT) is for
             * fast processing of reward proposals, imagine later when sudo is removed
             **/
            admin: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            currentMaxPoolId: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Map for PoolId <> RewardPoolOwner
             **/
            rewardPoolOwners: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<AccountId32>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
            /**
             * Map for PoolId <> RewardPool
             **/
            rewardPools: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletDrop3RewardPool>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        extrinsicFilter: {
            /**
             * a tuple (pallet_name_bytes, Option<function_name_bytes>) to represent blocked extrinsics
             * if `Option<function_name_bytes>` is None, then all extrinsics in `pallet_name_bytes` are
             * blocked
             **/
            blockedExtrinsics: AugmentedQuery<
                ApiType,
                (
                    arg: ITuple<[Bytes, Bytes]> | [Bytes | string | Uint8Array, Bytes | string | Uint8Array]
                ) => Observable<Option<Null>>,
                [ITuple<[Bytes, Bytes]>]
            > &
                QueryableStorageEntry<ApiType, [ITuple<[Bytes, Bytes]>]>;
            /**
             * current mode, ValueQuery as it can't be None
             **/
            mode: AugmentedQuery<ApiType, () => Observable<PalletExtrinsicFilterOperationalMode>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        identityManagement: {
            /**
             * delegatees who are authorised to send extrinsics(currently only `create_identity`)
             * on behalf of the users
             **/
            delegatee: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<Null>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        identityManagementMock: {
            /**
             * challenge code is per Litentry account + identity
             **/
            challengeCodes: AugmentedQuery<
                ApiType,
                (
                    arg1: AccountId32 | string | Uint8Array,
                    arg2:
                        | MockTeePrimitivesIdentity
                        | { Substrate: any }
                        | { Evm: any }
                        | { Web2: any }
                        | string
                        | Uint8Array
                ) => Observable<Option<U8aFixed>>,
                [AccountId32, MockTeePrimitivesIdentity]
            > &
                QueryableStorageEntry<ApiType, [AccountId32, MockTeePrimitivesIdentity]>;
            /**
             * delegatees who are authorised to send extrinsics(currently only `create_identity`)
             * on behalf of the users
             **/
            delegatee: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<Null>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * ID graph is per Litentry account + identity
             **/
            idGraphs: AugmentedQuery<
                ApiType,
                (
                    arg1: AccountId32 | string | Uint8Array,
                    arg2:
                        | MockTeePrimitivesIdentity
                        | { Substrate: any }
                        | { Evm: any }
                        | { Web2: any }
                        | string
                        | Uint8Array
                ) => Observable<Option<PalletIdentityManagementMockIdentityContext>>,
                [AccountId32, MockTeePrimitivesIdentity]
            > &
                QueryableStorageEntry<ApiType, [AccountId32, MockTeePrimitivesIdentity]>;
            /**
             * user shielding key is per Litentry account
             **/
            userShieldingKeys: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<U8aFixed>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        impExtrinsicWhitelist: {
            groupControlOn: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            groupMembers: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<bool>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        multisig: {
            /**
             * The set of open multisig operations.
             **/
            multisigs: AugmentedQuery<
                ApiType,
                (
                    arg1: AccountId32 | string | Uint8Array,
                    arg2: U8aFixed | string | Uint8Array
                ) => Observable<Option<PalletMultisigMultisig>>,
                [AccountId32, U8aFixed]
            > &
                QueryableStorageEntry<ApiType, [AccountId32, U8aFixed]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        parachainIdentity: {
            /**
             * Information that is pertinent to identify the entity behind an account.
             *
             * TWOX-NOTE: OK ― `AccountId` is a secure hash.
             **/
            identityOf: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<PalletIdentityRegistration>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * The set of registrars. Not expected to get very big as can only be added through a
             * special origin (likely a council motion).
             *
             * The index into this can be cast to `RegistrarIndex` to get a valid value.
             **/
            registrars: AugmentedQuery<ApiType, () => Observable<Vec<Option<PalletIdentityRegistrarInfo>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Alternative "sub" identities of this account.
             *
             * The first item is the deposit, the second is a vector of the accounts.
             *
             * TWOX-NOTE: OK ― `AccountId` is a secure hash.
             **/
            subsOf: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<ITuple<[u128, Vec<AccountId32>]>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * The super-identity of an alternative "sub" identity together with its name, within that
             * context. If the account is not some other account's sub-identity, then just `None`.
             **/
            superOf: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<ITuple<[AccountId32, Data]>>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        parachainInfo: {
            parachainId: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        parachainStaking: {
            /**
             * Snapshot of collator delegation stake at the start of the round
             **/
            atStake: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: AccountId32 | string | Uint8Array
                ) => Observable<PalletParachainStakingCollatorSnapshot>,
                [u32, AccountId32]
            > &
                QueryableStorageEntry<ApiType, [u32, AccountId32]>;
            /**
             * Stores auto-compounding configuration per collator.
             **/
            autoCompoundingDelegations: AugmentedQuery<
                ApiType,
                (
                    arg: AccountId32 | string | Uint8Array
                ) => Observable<Vec<PalletParachainStakingAutoCompoundAutoCompoundConfig>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Points for each collator per round
             **/
            awardedPts: AugmentedQuery<
                ApiType,
                (arg1: u32 | AnyNumber | Uint8Array, arg2: AccountId32 | string | Uint8Array) => Observable<u32>,
                [u32, AccountId32]
            > &
                QueryableStorageEntry<ApiType, [u32, AccountId32]>;
            /**
             * Bottom delegations for collator candidate
             **/
            bottomDelegations: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<PalletParachainStakingDelegations>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Get collator candidate info associated with an account if account is candidate else None
             **/
            candidateInfo: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<PalletParachainStakingCandidateMetadata>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * The pool of collator candidates, each with their total backing stake
             **/
            candidatePool: AugmentedQuery<ApiType, () => Observable<Vec<PalletParachainStakingBond>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The whitelist of collation candidates.
             * This storage should be safe to delete after
             * we release the restriction
             **/
            candidates: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Commission percent taken off of rewards for all collators
             **/
            collatorCommission: AugmentedQuery<ApiType, () => Observable<Perbill>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Delayed payouts
             **/
            delayedPayouts: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletParachainStakingDelayedPayout>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Stores outstanding delegation requests per collator.
             **/
            delegationScheduledRequests: AugmentedQuery<
                ApiType,
                (
                    arg: AccountId32 | string | Uint8Array
                ) => Observable<Vec<PalletParachainStakingDelegationRequestsScheduledRequest>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Get delegator state associated with an account if account is delegating else None
             **/
            delegatorState: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<PalletParachainStakingDelegator>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Inflation configuration
             **/
            inflationConfig: AugmentedQuery<
                ApiType,
                () => Observable<PalletParachainStakingInflationInflationInfo>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Parachain bond config info { account, percent_of_inflation }
             **/
            parachainBondInfo: AugmentedQuery<
                ApiType,
                () => Observable<PalletParachainStakingParachainBondConfig>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Total points awarded to collators for block production in the round
             **/
            points: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<u32>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Current round index and next round scheduled transition
             **/
            round: AugmentedQuery<ApiType, () => Observable<PalletParachainStakingRoundInfo>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The collator candidates selected for the current round
             **/
            selectedCandidates: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Total counted stake for selected candidates in the round
             **/
            staked: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<u128>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Top delegations for collator candidate
             **/
            topDelegations: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<PalletParachainStakingDelegations>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Total capital locked by this staking pallet
             **/
            total: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The total candidates selected every round
             **/
            totalSelected: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        parachainSystem: {
            /**
             * The number of HRMP messages we observed in `on_initialize` and thus used that number for
             * announcing the weight of `on_initialize` and `on_finalize`.
             **/
            announcedHrmpMessagesPerCandidate: AugmentedQuery<ApiType, () => Observable<u32>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The next authorized upgrade, if there is one.
             **/
            authorizedUpgrade: AugmentedQuery<ApiType, () => Observable<Option<H256>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * A custom head data that should be returned as result of `validate_block`.
             *
             * See [`Pallet::set_custom_validation_head_data`] for more information.
             **/
            customValidationHeadData: AugmentedQuery<ApiType, () => Observable<Option<Bytes>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Were the validation data set to notify the relay chain?
             **/
            didSetValidationCode: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The parachain host configuration that was obtained from the relay parent.
             *
             * This field is meant to be updated each block with the validation data inherent. Therefore,
             * before processing of the inherent, e.g. in `on_initialize` this data may be stale.
             *
             * This data is also absent from the genesis.
             **/
            hostConfiguration: AugmentedQuery<
                ApiType,
                () => Observable<Option<PolkadotPrimitivesV2AbridgedHostConfiguration>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * HRMP messages that were sent in a block.
             *
             * This will be cleared in `on_initialize` of each new block.
             **/
            hrmpOutboundMessages: AugmentedQuery<
                ApiType,
                () => Observable<Vec<PolkadotCorePrimitivesOutboundHrmpMessage>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * HRMP watermark that was set in a block.
             *
             * This will be cleared in `on_initialize` of each new block.
             **/
            hrmpWatermark: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The last downward message queue chain head we have observed.
             *
             * This value is loaded before and saved after processing inbound downward messages carried
             * by the system inherent.
             **/
            lastDmqMqcHead: AugmentedQuery<ApiType, () => Observable<H256>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The message queue chain heads we have observed per each channel incoming channel.
             *
             * This value is loaded before and saved after processing inbound downward messages carried
             * by the system inherent.
             **/
            lastHrmpMqcHeads: AugmentedQuery<ApiType, () => Observable<BTreeMap<u32, H256>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The relay chain block number associated with the last parachain block.
             **/
            lastRelayChainBlockNumber: AugmentedQuery<ApiType, () => Observable<u32>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Validation code that is set by the parachain and is to be communicated to collator and
             * consequently the relay-chain.
             *
             * This will be cleared in `on_initialize` of each new block if no other pallet already set
             * the value.
             **/
            newValidationCode: AugmentedQuery<ApiType, () => Observable<Option<Bytes>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Upward messages that are still pending and not yet send to the relay chain.
             **/
            pendingUpwardMessages: AugmentedQuery<ApiType, () => Observable<Vec<Bytes>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * In case of a scheduled upgrade, this storage field contains the validation code to be applied.
             *
             * As soon as the relay chain gives us the go-ahead signal, we will overwrite the [`:code`][well_known_keys::CODE]
             * which will result the next block process with the new validation code. This concludes the upgrade process.
             *
             * [well_known_keys::CODE]: sp_core::storage::well_known_keys::CODE
             **/
            pendingValidationCode: AugmentedQuery<ApiType, () => Observable<Bytes>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Number of downward messages processed in a block.
             *
             * This will be cleared in `on_initialize` of each new block.
             **/
            processedDownwardMessages: AugmentedQuery<ApiType, () => Observable<u32>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The state proof for the last relay parent block.
             *
             * This field is meant to be updated each block with the validation data inherent. Therefore,
             * before processing of the inherent, e.g. in `on_initialize` this data may be stale.
             *
             * This data is also absent from the genesis.
             **/
            relayStateProof: AugmentedQuery<ApiType, () => Observable<Option<SpTrieStorageProof>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The snapshot of some state related to messaging relevant to the current parachain as per
             * the relay parent.
             *
             * This field is meant to be updated each block with the validation data inherent. Therefore,
             * before processing of the inherent, e.g. in `on_initialize` this data may be stale.
             *
             * This data is also absent from the genesis.
             **/
            relevantMessagingState: AugmentedQuery<
                ApiType,
                () => Observable<Option<CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The weight we reserve at the beginning of the block for processing DMP messages. This
             * overrides the amount set in the Config trait.
             **/
            reservedDmpWeightOverride: AugmentedQuery<ApiType, () => Observable<Option<SpWeightsWeightV2Weight>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The weight we reserve at the beginning of the block for processing XCMP messages. This
             * overrides the amount set in the Config trait.
             **/
            reservedXcmpWeightOverride: AugmentedQuery<ApiType, () => Observable<Option<SpWeightsWeightV2Weight>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * An option which indicates if the relay-chain restricts signalling a validation code upgrade.
             * In other words, if this is `Some` and [`NewValidationCode`] is `Some` then the produced
             * candidate will be invalid.
             *
             * This storage item is a mirror of the corresponding value for the current parachain from the
             * relay-chain. This value is ephemeral which means it doesn't hit the storage. This value is
             * set after the inherent.
             **/
            upgradeRestrictionSignal: AugmentedQuery<
                ApiType,
                () => Observable<Option<PolkadotPrimitivesV2UpgradeRestriction>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Upward messages that were sent in a block.
             *
             * This will be cleared in `on_initialize` of each new block.
             **/
            upwardMessages: AugmentedQuery<ApiType, () => Observable<Vec<Bytes>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The [`PersistedValidationData`] set for this block.
             * This value is expected to be set only once per block and it's never stored
             * in the trie.
             **/
            validationData: AugmentedQuery<
                ApiType,
                () => Observable<Option<PolkadotPrimitivesV2PersistedValidationData>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        polkadotXcm: {
            /**
             * The existing asset traps.
             *
             * Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of
             * times this pair has been trapped (usually just 1 if it exists at all).
             **/
            assetTraps: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<u32>, [H256]> &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * The current migration's stage, if any.
             **/
            currentMigration: AugmentedQuery<ApiType, () => Observable<Option<PalletXcmVersionMigrationStage>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Fungible assets which we know are locked on this chain.
             **/
            lockedFungibles: AugmentedQuery<
                ApiType,
                (
                    arg: AccountId32 | string | Uint8Array
                ) => Observable<Option<Vec<ITuple<[u128, XcmVersionedMultiLocation]>>>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * The ongoing queries.
             **/
            queries: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletXcmQueryStatus>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
            /**
             * The latest available query index.
             **/
            queryCounter: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Fungible assets which we know are locked on a remote chain.
             **/
            remoteLockedFungibles: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: AccountId32 | string | Uint8Array,
                    arg3: XcmVersionedAssetId | { V3: any } | string | Uint8Array
                ) => Observable<Option<PalletXcmRemoteLockedFungibleRecord>>,
                [u32, AccountId32, XcmVersionedAssetId]
            > &
                QueryableStorageEntry<ApiType, [u32, AccountId32, XcmVersionedAssetId]>;
            /**
             * Default version to encode XCM when latest version of destination is unknown. If `None`,
             * then the destinations whose XCM version is unknown are considered unreachable.
             **/
            safeXcmVersion: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The Latest versions that we know various locations support.
             **/
            supportedVersion: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array
                ) => Observable<Option<u32>>,
                [u32, XcmVersionedMultiLocation]
            > &
                QueryableStorageEntry<ApiType, [u32, XcmVersionedMultiLocation]>;
            /**
             * Destinations whose latest XCM version we would like to know. Duplicates not allowed, and
             * the `u32` counter is the number of times that a send to the destination has been attempted,
             * which is used as a prioritization.
             **/
            versionDiscoveryQueue: AugmentedQuery<
                ApiType,
                () => Observable<Vec<ITuple<[XcmVersionedMultiLocation, u32]>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * All locations that we have requested version notifications from.
             **/
            versionNotifiers: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array
                ) => Observable<Option<u64>>,
                [u32, XcmVersionedMultiLocation]
            > &
                QueryableStorageEntry<ApiType, [u32, XcmVersionedMultiLocation]>;
            /**
             * The target locations that are subscribed to our version changes, as well as the most recent
             * of our versions we informed them of.
             **/
            versionNotifyTargets: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array
                ) => Observable<Option<ITuple<[u64, SpWeightsWeightV2Weight, u32]>>>,
                [u32, XcmVersionedMultiLocation]
            > &
                QueryableStorageEntry<ApiType, [u32, XcmVersionedMultiLocation]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        preimage: {
            preimageFor: AugmentedQuery<
                ApiType,
                (
                    arg: ITuple<[H256, u32]> | [H256 | string | Uint8Array, u32 | AnyNumber | Uint8Array]
                ) => Observable<Option<Bytes>>,
                [ITuple<[H256, u32]>]
            > &
                QueryableStorageEntry<ApiType, [ITuple<[H256, u32]>]>;
            /**
             * The request status of a given hash.
             **/
            statusFor: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<PalletPreimageRequestStatus>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        proxy: {
            /**
             * The announcements made by the proxy (key).
             **/
            announcements: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<ITuple<[Vec<PalletProxyAnnouncement>, u128]>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * The set of account proxies. Maps the account which has delegated to the accounts
             * which are being delegated to, together with the amount held on deposit.
             **/
            proxies: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<ITuple<[Vec<PalletProxyProxyDefinition>, u128]>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        scheduler: {
            /**
             * Items to be executed, indexed by the block number that they should be executed on.
             **/
            agenda: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<Option<PalletSchedulerScheduled>>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            incompleteSince: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Lookup from a name to the block number and index of the task.
             *
             * For v3 -> v4 the previously unbounded identities are Blake2-256 hashed to form the v4
             * identities.
             **/
            lookup: AugmentedQuery<
                ApiType,
                (arg: U8aFixed | string | Uint8Array) => Observable<Option<ITuple<[u32, u32]>>>,
                [U8aFixed]
            > &
                QueryableStorageEntry<ApiType, [U8aFixed]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        session: {
            /**
             * Current index of the session.
             **/
            currentIndex: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Indices of disabled validators.
             *
             * The vec is always kept sorted so that we can find whether a given validator is
             * disabled using binary search. It gets cleared when `on_session_ending` returns
             * a new set of identities.
             **/
            disabledValidators: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The owner of a key. The key is the `KeyTypeId` + the encoded key.
             **/
            keyOwner: AugmentedQuery<
                ApiType,
                (
                    arg:
                        | ITuple<[SpCoreCryptoKeyTypeId, Bytes]>
                        | [SpCoreCryptoKeyTypeId | string | Uint8Array, Bytes | string | Uint8Array]
                ) => Observable<Option<AccountId32>>,
                [ITuple<[SpCoreCryptoKeyTypeId, Bytes]>]
            > &
                QueryableStorageEntry<ApiType, [ITuple<[SpCoreCryptoKeyTypeId, Bytes]>]>;
            /**
             * The next session keys for a validator.
             **/
            nextKeys: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<RococoParachainRuntimeSessionKeys>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * True if the underlying economic identities or weighting behind the validators
             * has changed in the queued validator set.
             **/
            queuedChanged: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The queued keys for the next session. When the next session begins, these keys
             * will be used to determine the validator's session keys.
             **/
            queuedKeys: AugmentedQuery<
                ApiType,
                () => Observable<Vec<ITuple<[AccountId32, RococoParachainRuntimeSessionKeys]>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The current set of validators.
             **/
            validators: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        sidechain: {
            latestSidechainBlockConfirmation: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<SidechainPrimitivesSidechainBlockConfirmation>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            sidechainBlockFinalizationCandidate: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<u64>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            workerForShard: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<u64>, [H256]> &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        sudo: {
            /**
             * The `AccountId` of the sudo key.
             **/
            key: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        system: {
            /**
             * The full account information for a particular account ID.
             **/
            account: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<FrameSystemAccountInfo>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Total length (in bytes) for all extrinsics put together, for the current block.
             **/
            allExtrinsicsLen: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Map of block numbers to block hashes.
             **/
            blockHash: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<H256>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * The current weight for the block.
             **/
            blockWeight: AugmentedQuery<ApiType, () => Observable<FrameSupportDispatchPerDispatchClassWeight>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Digest of the current block, also part of the block header.
             **/
            digest: AugmentedQuery<ApiType, () => Observable<SpRuntimeDigest>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The number of events in the `Events<T>` list.
             **/
            eventCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Events deposited for the current block.
             *
             * NOTE: The item is unbound and should therefore never be read on chain.
             * It could otherwise inflate the PoV size of a block.
             *
             * Events have a large in-memory size. Box the events to not go out-of-memory
             * just in case someone still reads them from within the runtime.
             **/
            events: AugmentedQuery<ApiType, () => Observable<Vec<FrameSystemEventRecord>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Mapping between a topic (represented by T::Hash) and a vector of indexes
             * of events in the `<Events<T>>` list.
             *
             * All topic vectors have deterministic storage locations depending on the topic. This
             * allows light-clients to leverage the changes trie storage tracking mechanism and
             * in case of changes fetch the list of events of interest.
             *
             * The value has the type `(T::BlockNumber, EventIndex)` because if we used only just
             * the `EventIndex` then in case if the topic has the same contents on the next block
             * no notification will be triggered thus the event might be lost.
             **/
            eventTopics: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Vec<ITuple<[u32, u32]>>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * The execution phase of the block.
             **/
            executionPhase: AugmentedQuery<ApiType, () => Observable<Option<FrameSystemPhase>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Total extrinsics count for the current block.
             **/
            extrinsicCount: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Extrinsics data for the current block (maps an extrinsic's index to its data).
             **/
            extrinsicData: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Bytes>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened.
             **/
            lastRuntimeUpgrade: AugmentedQuery<
                ApiType,
                () => Observable<Option<FrameSystemLastRuntimeUpgradeInfo>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The current block number being processed. Set by `execute_block`.
             **/
            number: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Hash of the previous block.
             **/
            parentHash: AugmentedQuery<ApiType, () => Observable<H256>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * True if we have upgraded so that AccountInfo contains three types of `RefCount`. False
             * (default) if not.
             **/
            upgradedToTripleRefCount: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * True if we have upgraded so that `type RefCount` is `u32`. False (default) if not.
             **/
            upgradedToU32RefCount: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        technicalCommittee: {
            /**
             * The current members of the collective. This is stored sorted (just by value).
             **/
            members: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The prime member that helps determine the default vote behavior in case of absentations.
             **/
            prime: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Proposals so far.
             **/
            proposalCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Actual proposal for a given hash, if it's current.
             **/
            proposalOf: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<Call>>, [H256]> &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * The hashes of the active proposals.
             **/
            proposals: AugmentedQuery<ApiType, () => Observable<Vec<H256>>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Votes on a given proposal, if it is ongoing.
             **/
            voting: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<PalletCollectiveVotes>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        technicalCommitteeMembership: {
            /**
             * The current membership, stored as an ordered Vec.
             **/
            members: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The current prime member, if one exists.
             **/
            prime: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        teeracle: {
            /**
             * Exchange rates chain's cryptocurrency/currency (trading pair) from different sources
             **/
            exchangeRates: AugmentedQuery<
                ApiType,
                (
                    arg1: Bytes | string | Uint8Array,
                    arg2: Bytes | string | Uint8Array
                ) => Observable<SubstrateFixedFixedU64>,
                [Bytes, Bytes]
            > &
                QueryableStorageEntry<ApiType, [Bytes, Bytes]>;
            oracleData: AugmentedQuery<
                ApiType,
                (arg1: Bytes | string | Uint8Array, arg2: Bytes | string | Uint8Array) => Observable<Bytes>,
                [Bytes, Bytes]
            > &
                QueryableStorageEntry<ApiType, [Bytes, Bytes]>;
            /**
             * whitelist of trusted oracle's releases for different data sources
             **/
            whitelists: AugmentedQuery<
                ApiType,
                (arg: Bytes | string | Uint8Array) => Observable<Vec<U8aFixed>>,
                [Bytes]
            > &
                QueryableStorageEntry<ApiType, [Bytes]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        teerex: {
            admin: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            allowSGXDebugMode: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            enclaveCount: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            enclaveIndex: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<u64>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            enclaveRegistry: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<TeerexPrimitivesEnclave>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
            executedCalls: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<u64>, [H256]> &
                QueryableStorageEntry<ApiType, [H256]>;
            heartbeatTimeout: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            quotingEnclaveRegistry: AugmentedQuery<ApiType, () => Observable<TeerexPrimitivesQuotingEnclave>, []> &
                QueryableStorageEntry<ApiType, []>;
            scheduledEnclave: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<U8aFixed>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
            tcbInfo: AugmentedQuery<
                ApiType,
                (arg: U8aFixed | string | Uint8Array) => Observable<TeerexPrimitivesTcbInfoOnChain>,
                [U8aFixed]
            > &
                QueryableStorageEntry<ApiType, [U8aFixed]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        timestamp: {
            /**
             * Did the timestamp get updated in this block?
             **/
            didUpdate: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Current time for the current block.
             **/
            now: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        tips: {
            /**
             * Simple preimage lookup from the reason's hash to the original data. Again, has an
             * insecure enumerable hash since the key is guaranteed to be the result of a secure hash.
             **/
            reasons: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<Bytes>>, [H256]> &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * TipsMap that are not yet completed. Keyed by the hash of `(reason, who)` from the value.
             * This has the insecure enumerable hash function since the key itself is already
             * guaranteed to be a secure hash.
             **/
            tips: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<PalletTipsOpenTip>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        tokens: {
            /**
             * The balance of a token type under an account.
             *
             * NOTE: If the total is ever zero, decrease account ref account.
             *
             * NOTE: This is only used in the case that this module is used to store
             * balances.
             **/
            accounts: AugmentedQuery<
                ApiType,
                (
                    arg1: AccountId32 | string | Uint8Array,
                    arg2: u128 | AnyNumber | Uint8Array
                ) => Observable<OrmlTokensAccountData>,
                [AccountId32, u128]
            > &
                QueryableStorageEntry<ApiType, [AccountId32, u128]>;
            /**
             * Any liquidity locks of a token type under an account.
             * NOTE: Should only be accessed when setting, changing and freeing a lock.
             **/
            locks: AugmentedQuery<
                ApiType,
                (
                    arg1: AccountId32 | string | Uint8Array,
                    arg2: u128 | AnyNumber | Uint8Array
                ) => Observable<Vec<OrmlTokensBalanceLock>>,
                [AccountId32, u128]
            > &
                QueryableStorageEntry<ApiType, [AccountId32, u128]>;
            /**
             * Named reserves on some account balances.
             **/
            reserves: AugmentedQuery<
                ApiType,
                (
                    arg1: AccountId32 | string | Uint8Array,
                    arg2: u128 | AnyNumber | Uint8Array
                ) => Observable<Vec<OrmlTokensReserveData>>,
                [AccountId32, u128]
            > &
                QueryableStorageEntry<ApiType, [AccountId32, u128]>;
            /**
             * The total issuance of a token type.
             **/
            totalIssuance: AugmentedQuery<ApiType, (arg: u128 | AnyNumber | Uint8Array) => Observable<u128>, [u128]> &
                QueryableStorageEntry<ApiType, [u128]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        transactionPayment: {
            nextFeeMultiplier: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            storageVersion: AugmentedQuery<ApiType, () => Observable<PalletTransactionPaymentReleases>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        treasury: {
            /**
             * Proposal indices that have been approved but not yet awarded.
             **/
            approvals: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The amount which has been reported as inactive to Currency.
             **/
            deactivated: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Number of proposals that have been made.
             **/
            proposalCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Proposals that have been made.
             **/
            proposals: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletTreasuryProposal>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        vcManagement: {
            admin: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            schemaRegistry: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletVcManagementSchemaVcSchema>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
            schemaRegistryIndex: AugmentedQuery<ApiType, () => Observable<u64>, []> &
                QueryableStorageEntry<ApiType, []>;
            vcRegistry: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<PalletVcManagementVcContext>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        vcmpExtrinsicWhitelist: {
            groupControlOn: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            groupMembers: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<bool>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        vesting: {
            /**
             * Storage version of the pallet.
             *
             * New networks start with latest version, as determined by the genesis build.
             **/
            storageVersion: AugmentedQuery<ApiType, () => Observable<PalletVestingReleases>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Information regarding the vesting of a given account.
             **/
            vesting: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<Vec<PalletVestingVestingInfo>>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        xcmpQueue: {
            /**
             * Counter for the related counted storage map
             **/
            counterForOverweight: AugmentedQuery<ApiType, () => Observable<u32>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Inbound aggregate XCMP messages. It can only be one per ParaId/block.
             **/
            inboundXcmpMessages: AugmentedQuery<
                ApiType,
                (arg1: u32 | AnyNumber | Uint8Array, arg2: u32 | AnyNumber | Uint8Array) => Observable<Bytes>,
                [u32, u32]
            > &
                QueryableStorageEntry<ApiType, [u32, u32]>;
            /**
             * Status of the inbound XCMP channels.
             **/
            inboundXcmpStatus: AugmentedQuery<
                ApiType,
                () => Observable<Vec<CumulusPalletXcmpQueueInboundChannelDetails>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The messages outbound in a given XCMP channel.
             **/
            outboundXcmpMessages: AugmentedQuery<
                ApiType,
                (arg1: u32 | AnyNumber | Uint8Array, arg2: u16 | AnyNumber | Uint8Array) => Observable<Bytes>,
                [u32, u16]
            > &
                QueryableStorageEntry<ApiType, [u32, u16]>;
            /**
             * The non-empty XCMP channels in order of becoming non-empty, and the index of the first
             * and last outbound message. If the two indices are equal, then it indicates an empty
             * queue and there must be a non-`Ok` `OutboundStatus`. We assume queues grow no greater
             * than 65535 items. Queue indices for normal messages begin at one; zero is reserved in
             * case of the need to send a high-priority signal message this block.
             * The bool is true if there is a signal message waiting to be sent.
             **/
            outboundXcmpStatus: AugmentedQuery<
                ApiType,
                () => Observable<Vec<CumulusPalletXcmpQueueOutboundChannelDetails>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The messages that exceeded max individual message weight budget.
             *
             * These message stay in this storage map until they are manually dispatched via
             * `service_overweight`.
             **/
            overweight: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<ITuple<[u32, u32, Bytes]>>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
            /**
             * The number of overweight messages ever recorded in `Overweight`. Also doubles as the next
             * available free overweight index.
             **/
            overweightCount: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The configuration which controls the dynamics of the outbound queue.
             **/
            queueConfig: AugmentedQuery<ApiType, () => Observable<CumulusPalletXcmpQueueQueueConfigData>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Whether or not the XCMP queue is suspended from executing incoming XCMs or not.
             **/
            queueSuspended: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Any signal messages waiting to be sent.
             **/
            signalMessages: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Bytes>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
    } // AugmentedQueries
} // declare module
