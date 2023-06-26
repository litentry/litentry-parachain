// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/types/registry';

import type {
    CorePrimitivesAssertion,
    CorePrimitivesAssertionIndexingNetwork,
    CorePrimitivesErrorErrorDetail,
    CorePrimitivesErrorImpError,
    CorePrimitivesErrorVcmpError,
    CorePrimitivesKeyAesOutput,
    CumulusPalletDmpQueueCall,
    CumulusPalletDmpQueueConfigData,
    CumulusPalletDmpQueueError,
    CumulusPalletDmpQueueEvent,
    CumulusPalletDmpQueuePageIndexData,
    CumulusPalletParachainSystemCall,
    CumulusPalletParachainSystemError,
    CumulusPalletParachainSystemEvent,
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot,
    CumulusPalletXcmCall,
    CumulusPalletXcmError,
    CumulusPalletXcmEvent,
    CumulusPalletXcmOrigin,
    CumulusPalletXcmpQueueCall,
    CumulusPalletXcmpQueueError,
    CumulusPalletXcmpQueueEvent,
    CumulusPalletXcmpQueueInboundChannelDetails,
    CumulusPalletXcmpQueueInboundState,
    CumulusPalletXcmpQueueOutboundChannelDetails,
    CumulusPalletXcmpQueueOutboundState,
    CumulusPalletXcmpQueueQueueConfigData,
    CumulusPrimitivesParachainInherentParachainInherentData,
    FrameSupportDispatchDispatchClass,
    FrameSupportDispatchDispatchInfo,
    FrameSupportDispatchPays,
    FrameSupportDispatchPerDispatchClassU32,
    FrameSupportDispatchPerDispatchClassWeight,
    FrameSupportDispatchPerDispatchClassWeightsPerClass,
    FrameSupportDispatchRawOrigin,
    FrameSupportPalletId,
    FrameSupportPreimagesBounded,
    FrameSupportTokensMiscBalanceStatus,
    FrameSystemAccountInfo,
    FrameSystemCall,
    FrameSystemError,
    FrameSystemEvent,
    FrameSystemEventRecord,
    FrameSystemExtensionsCheckGenesis,
    FrameSystemExtensionsCheckNonZeroSender,
    FrameSystemExtensionsCheckNonce,
    FrameSystemExtensionsCheckSpecVersion,
    FrameSystemExtensionsCheckTxVersion,
    FrameSystemExtensionsCheckWeight,
    FrameSystemLastRuntimeUpgradeInfo,
    FrameSystemLimitsBlockLength,
    FrameSystemLimitsBlockWeights,
    FrameSystemLimitsWeightsPerClass,
    FrameSystemPhase,
    MockTeePrimitivesIdentity,
    MockTeePrimitivesIdentityAddress20,
    MockTeePrimitivesIdentityAddress32,
    MockTeePrimitivesIdentityEvmNetwork,
    MockTeePrimitivesIdentitySubstrateNetwork,
    MockTeePrimitivesIdentityWeb2Network,
    OrmlTokensAccountData,
    OrmlTokensBalanceLock,
    OrmlTokensModuleCall,
    OrmlTokensModuleError,
    OrmlTokensModuleEvent,
    OrmlTokensReserveData,
    OrmlXtokensModuleCall,
    OrmlXtokensModuleError,
    OrmlXtokensModuleEvent,
    PalletAssetManagerAssetMetadata,
    PalletAssetManagerCall,
    PalletAssetManagerError,
    PalletAssetManagerEvent,
    PalletBalancesAccountData,
    PalletBalancesBalanceLock,
    PalletBalancesCall,
    PalletBalancesError,
    PalletBalancesEvent,
    PalletBalancesReasons,
    PalletBalancesReserveData,
    PalletBountiesBounty,
    PalletBountiesBountyStatus,
    PalletBountiesCall,
    PalletBountiesError,
    PalletBountiesEvent,
    PalletBridgeBridgeEvent,
    PalletBridgeCall,
    PalletBridgeError,
    PalletBridgeEvent,
    PalletBridgeProposalStatus,
    PalletBridgeProposalVotes,
    PalletBridgeTransferCall,
    PalletBridgeTransferError,
    PalletBridgeTransferEvent,
    PalletCollectiveCall,
    PalletCollectiveError,
    PalletCollectiveEvent,
    PalletCollectiveRawOrigin,
    PalletCollectiveVotes,
    PalletDemocracyCall,
    PalletDemocracyConviction,
    PalletDemocracyDelegations,
    PalletDemocracyError,
    PalletDemocracyEvent,
    PalletDemocracyMetadataOwner,
    PalletDemocracyReferendumInfo,
    PalletDemocracyReferendumStatus,
    PalletDemocracyTally,
    PalletDemocracyVoteAccountVote,
    PalletDemocracyVotePriorLock,
    PalletDemocracyVoteThreshold,
    PalletDemocracyVoteVoting,
    PalletDrop3Call,
    PalletDrop3Error,
    PalletDrop3Event,
    PalletDrop3RewardPool,
    PalletExtrinsicFilterCall,
    PalletExtrinsicFilterError,
    PalletExtrinsicFilterEvent,
    PalletExtrinsicFilterOperationalMode,
    PalletGroupCall,
    PalletGroupError,
    PalletGroupEvent,
    PalletIdentityBitFlags,
    PalletIdentityCall,
    PalletIdentityError,
    PalletIdentityEvent,
    PalletIdentityIdentityField,
    PalletIdentityIdentityInfo,
    PalletIdentityJudgement,
    PalletIdentityManagementCall,
    PalletIdentityManagementError,
    PalletIdentityManagementEvent,
    PalletIdentityManagementMockCall,
    PalletIdentityManagementMockError,
    PalletIdentityManagementMockEvent,
    PalletIdentityManagementMockIdentityContext,
    PalletIdentityRegistrarInfo,
    PalletIdentityRegistration,
    PalletMembershipCall,
    PalletMembershipError,
    PalletMembershipEvent,
    PalletMultisigCall,
    PalletMultisigError,
    PalletMultisigEvent,
    PalletMultisigMultisig,
    PalletMultisigTimepoint,
    PalletParachainStakingAutoCompoundAutoCompoundConfig,
    PalletParachainStakingBond,
    PalletParachainStakingBondWithAutoCompound,
    PalletParachainStakingCall,
    PalletParachainStakingCandidateBondLessRequest,
    PalletParachainStakingCandidateMetadata,
    PalletParachainStakingCapacityStatus,
    PalletParachainStakingCollatorSnapshot,
    PalletParachainStakingCollatorStatus,
    PalletParachainStakingDelayedPayout,
    PalletParachainStakingDelegationRequestsCancelledScheduledRequest,
    PalletParachainStakingDelegationRequestsDelegationAction,
    PalletParachainStakingDelegationRequestsScheduledRequest,
    PalletParachainStakingDelegations,
    PalletParachainStakingDelegator,
    PalletParachainStakingDelegatorAdded,
    PalletParachainStakingDelegatorStatus,
    PalletParachainStakingError,
    PalletParachainStakingEvent,
    PalletParachainStakingInflationInflationInfo,
    PalletParachainStakingParachainBondConfig,
    PalletParachainStakingRoundInfo,
    PalletParachainStakingSetOrderedSet,
    PalletPreimageCall,
    PalletPreimageError,
    PalletPreimageEvent,
    PalletPreimageRequestStatus,
    PalletProxyAnnouncement,
    PalletProxyCall,
    PalletProxyError,
    PalletProxyEvent,
    PalletProxyProxyDefinition,
    PalletSchedulerCall,
    PalletSchedulerError,
    PalletSchedulerEvent,
    PalletSchedulerScheduled,
    PalletSessionCall,
    PalletSessionError,
    PalletSessionEvent,
    PalletSidechainCall,
    PalletSidechainError,
    PalletSidechainEvent,
    PalletSudoCall,
    PalletSudoError,
    PalletSudoEvent,
    PalletTeeracleCall,
    PalletTeeracleError,
    PalletTeeracleEvent,
    PalletTeerexCall,
    PalletTeerexError,
    PalletTeerexEvent,
    PalletTimestampCall,
    PalletTipsCall,
    PalletTipsError,
    PalletTipsEvent,
    PalletTipsOpenTip,
    PalletTransactionPaymentChargeTransactionPayment,
    PalletTransactionPaymentEvent,
    PalletTransactionPaymentReleases,
    PalletTreasuryCall,
    PalletTreasuryError,
    PalletTreasuryEvent,
    PalletTreasuryProposal,
    PalletUtilityCall,
    PalletUtilityError,
    PalletUtilityEvent,
    PalletVcManagementCall,
    PalletVcManagementError,
    PalletVcManagementEvent,
    PalletVcManagementSchemaVcSchema,
    PalletVcManagementVcContext,
    PalletVcManagementVcContextStatus,
    PalletVestingCall,
    PalletVestingError,
    PalletVestingEvent,
    PalletVestingReleases,
    PalletVestingVestingInfo,
    PalletXcmCall,
    PalletXcmError,
    PalletXcmEvent,
    PalletXcmOrigin,
    PalletXcmQueryStatus,
    PalletXcmRemoteLockedFungibleRecord,
    PalletXcmVersionMigrationStage,
    ParachainInfoCall,
    PolkadotCorePrimitivesInboundDownwardMessage,
    PolkadotCorePrimitivesInboundHrmpMessage,
    PolkadotCorePrimitivesOutboundHrmpMessage,
    PolkadotParachainPrimitivesXcmpMessageFormat,
    PolkadotPrimitivesV2AbridgedHostConfiguration,
    PolkadotPrimitivesV2AbridgedHrmpChannel,
    PolkadotPrimitivesV2PersistedValidationData,
    PolkadotPrimitivesV2UpgradeRestriction,
    RococoParachainRuntimeOriginCaller,
    RococoParachainRuntimeProxyType,
    RococoParachainRuntimeRuntime,
    RococoParachainRuntimeSessionKeys,
    RuntimeCommonXcmImplCurrencyId,
    SidechainPrimitivesSidechainBlockConfirmation,
    SpArithmeticArithmeticError,
    SpConsensusAuraSr25519AppSr25519Public,
    SpCoreCryptoKeyTypeId,
    SpCoreEcdsaSignature,
    SpCoreEd25519Signature,
    SpCoreSr25519Public,
    SpCoreSr25519Signature,
    SpCoreVoid,
    SpRuntimeDigest,
    SpRuntimeDigestDigestItem,
    SpRuntimeDispatchError,
    SpRuntimeModuleError,
    SpRuntimeMultiSignature,
    SpRuntimeTokenError,
    SpRuntimeTransactionalError,
    SpTrieStorageProof,
    SpVersionRuntimeVersion,
    SpWeightsRuntimeDbWeight,
    SpWeightsWeightV2Weight,
    SubstrateFixedFixedU64,
    TeerexPrimitivesEnclave,
    TeerexPrimitivesQeTcb,
    TeerexPrimitivesQuotingEnclave,
    TeerexPrimitivesRequest,
    TeerexPrimitivesSgxBuildMode,
    TeerexPrimitivesSgxEnclaveMetadata,
    TeerexPrimitivesTcbInfoOnChain,
    TeerexPrimitivesTcbVersionStatus,
    TypenumBitB0,
    TypenumBitB1,
    TypenumUIntUInt,
    TypenumUIntUTerm,
    TypenumUintUTerm,
    XcmDoubleEncoded,
    XcmV2BodyId,
    XcmV2BodyPart,
    XcmV2Instruction,
    XcmV2Junction,
    XcmV2MultiAsset,
    XcmV2MultiLocation,
    XcmV2MultiassetAssetId,
    XcmV2MultiassetAssetInstance,
    XcmV2MultiassetFungibility,
    XcmV2MultiassetMultiAssetFilter,
    XcmV2MultiassetMultiAssets,
    XcmV2MultiassetWildFungibility,
    XcmV2MultiassetWildMultiAsset,
    XcmV2MultilocationJunctions,
    XcmV2NetworkId,
    XcmV2OriginKind,
    XcmV2Response,
    XcmV2TraitsError,
    XcmV2WeightLimit,
    XcmV2Xcm,
    XcmV3Instruction,
    XcmV3Junction,
    XcmV3JunctionBodyId,
    XcmV3JunctionBodyPart,
    XcmV3JunctionNetworkId,
    XcmV3Junctions,
    XcmV3MaybeErrorCode,
    XcmV3MultiAsset,
    XcmV3MultiLocation,
    XcmV3MultiassetAssetId,
    XcmV3MultiassetAssetInstance,
    XcmV3MultiassetFungibility,
    XcmV3MultiassetMultiAssetFilter,
    XcmV3MultiassetMultiAssets,
    XcmV3MultiassetWildFungibility,
    XcmV3MultiassetWildMultiAsset,
    XcmV3PalletInfo,
    XcmV3QueryResponseInfo,
    XcmV3Response,
    XcmV3TraitsError,
    XcmV3TraitsOutcome,
    XcmV3WeightLimit,
    XcmV3Xcm,
    XcmVersionedAssetId,
    XcmVersionedMultiAsset,
    XcmVersionedMultiAssets,
    XcmVersionedMultiLocation,
    XcmVersionedResponse,
    XcmVersionedXcm,
} from '@polkadot/types/lookup';

declare module '@polkadot/types/types/registry' {
    interface InterfaceTypes {
        CorePrimitivesAssertion: CorePrimitivesAssertion;
        CorePrimitivesAssertionIndexingNetwork: CorePrimitivesAssertionIndexingNetwork;
        CorePrimitivesErrorErrorDetail: CorePrimitivesErrorErrorDetail;
        CorePrimitivesErrorImpError: CorePrimitivesErrorImpError;
        CorePrimitivesErrorVcmpError: CorePrimitivesErrorVcmpError;
        CorePrimitivesKeyAesOutput: CorePrimitivesKeyAesOutput;
        CumulusPalletDmpQueueCall: CumulusPalletDmpQueueCall;
        CumulusPalletDmpQueueConfigData: CumulusPalletDmpQueueConfigData;
        CumulusPalletDmpQueueError: CumulusPalletDmpQueueError;
        CumulusPalletDmpQueueEvent: CumulusPalletDmpQueueEvent;
        CumulusPalletDmpQueuePageIndexData: CumulusPalletDmpQueuePageIndexData;
        CumulusPalletParachainSystemCall: CumulusPalletParachainSystemCall;
        CumulusPalletParachainSystemError: CumulusPalletParachainSystemError;
        CumulusPalletParachainSystemEvent: CumulusPalletParachainSystemEvent;
        CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot;
        CumulusPalletXcmCall: CumulusPalletXcmCall;
        CumulusPalletXcmError: CumulusPalletXcmError;
        CumulusPalletXcmEvent: CumulusPalletXcmEvent;
        CumulusPalletXcmOrigin: CumulusPalletXcmOrigin;
        CumulusPalletXcmpQueueCall: CumulusPalletXcmpQueueCall;
        CumulusPalletXcmpQueueError: CumulusPalletXcmpQueueError;
        CumulusPalletXcmpQueueEvent: CumulusPalletXcmpQueueEvent;
        CumulusPalletXcmpQueueInboundChannelDetails: CumulusPalletXcmpQueueInboundChannelDetails;
        CumulusPalletXcmpQueueInboundState: CumulusPalletXcmpQueueInboundState;
        CumulusPalletXcmpQueueOutboundChannelDetails: CumulusPalletXcmpQueueOutboundChannelDetails;
        CumulusPalletXcmpQueueOutboundState: CumulusPalletXcmpQueueOutboundState;
        CumulusPalletXcmpQueueQueueConfigData: CumulusPalletXcmpQueueQueueConfigData;
        CumulusPrimitivesParachainInherentParachainInherentData: CumulusPrimitivesParachainInherentParachainInherentData;
        FrameSupportDispatchDispatchClass: FrameSupportDispatchDispatchClass;
        FrameSupportDispatchDispatchInfo: FrameSupportDispatchDispatchInfo;
        FrameSupportDispatchPays: FrameSupportDispatchPays;
        FrameSupportDispatchPerDispatchClassU32: FrameSupportDispatchPerDispatchClassU32;
        FrameSupportDispatchPerDispatchClassWeight: FrameSupportDispatchPerDispatchClassWeight;
        FrameSupportDispatchPerDispatchClassWeightsPerClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
        FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
        FrameSupportPalletId: FrameSupportPalletId;
        FrameSupportPreimagesBounded: FrameSupportPreimagesBounded;
        FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
        FrameSystemAccountInfo: FrameSystemAccountInfo;
        FrameSystemCall: FrameSystemCall;
        FrameSystemError: FrameSystemError;
        FrameSystemEvent: FrameSystemEvent;
        FrameSystemEventRecord: FrameSystemEventRecord;
        FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
        FrameSystemExtensionsCheckNonZeroSender: FrameSystemExtensionsCheckNonZeroSender;
        FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
        FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
        FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
        FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
        FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
        FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
        FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
        FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
        FrameSystemPhase: FrameSystemPhase;
        MockTeePrimitivesIdentity: MockTeePrimitivesIdentity;
        MockTeePrimitivesIdentityAddress20: MockTeePrimitivesIdentityAddress20;
        MockTeePrimitivesIdentityAddress32: MockTeePrimitivesIdentityAddress32;
        MockTeePrimitivesIdentityEvmNetwork: MockTeePrimitivesIdentityEvmNetwork;
        MockTeePrimitivesIdentitySubstrateNetwork: MockTeePrimitivesIdentitySubstrateNetwork;
        MockTeePrimitivesIdentityWeb2Network: MockTeePrimitivesIdentityWeb2Network;
        OrmlTokensAccountData: OrmlTokensAccountData;
        OrmlTokensBalanceLock: OrmlTokensBalanceLock;
        OrmlTokensModuleCall: OrmlTokensModuleCall;
        OrmlTokensModuleError: OrmlTokensModuleError;
        OrmlTokensModuleEvent: OrmlTokensModuleEvent;
        OrmlTokensReserveData: OrmlTokensReserveData;
        OrmlXtokensModuleCall: OrmlXtokensModuleCall;
        OrmlXtokensModuleError: OrmlXtokensModuleError;
        OrmlXtokensModuleEvent: OrmlXtokensModuleEvent;
        PalletAssetManagerAssetMetadata: PalletAssetManagerAssetMetadata;
        PalletAssetManagerCall: PalletAssetManagerCall;
        PalletAssetManagerError: PalletAssetManagerError;
        PalletAssetManagerEvent: PalletAssetManagerEvent;
        PalletBalancesAccountData: PalletBalancesAccountData;
        PalletBalancesBalanceLock: PalletBalancesBalanceLock;
        PalletBalancesCall: PalletBalancesCall;
        PalletBalancesError: PalletBalancesError;
        PalletBalancesEvent: PalletBalancesEvent;
        PalletBalancesReasons: PalletBalancesReasons;
        PalletBalancesReserveData: PalletBalancesReserveData;
        PalletBountiesBounty: PalletBountiesBounty;
        PalletBountiesBountyStatus: PalletBountiesBountyStatus;
        PalletBountiesCall: PalletBountiesCall;
        PalletBountiesError: PalletBountiesError;
        PalletBountiesEvent: PalletBountiesEvent;
        PalletBridgeBridgeEvent: PalletBridgeBridgeEvent;
        PalletBridgeCall: PalletBridgeCall;
        PalletBridgeError: PalletBridgeError;
        PalletBridgeEvent: PalletBridgeEvent;
        PalletBridgeProposalStatus: PalletBridgeProposalStatus;
        PalletBridgeProposalVotes: PalletBridgeProposalVotes;
        PalletBridgeTransferCall: PalletBridgeTransferCall;
        PalletBridgeTransferError: PalletBridgeTransferError;
        PalletBridgeTransferEvent: PalletBridgeTransferEvent;
        PalletCollectiveCall: PalletCollectiveCall;
        PalletCollectiveError: PalletCollectiveError;
        PalletCollectiveEvent: PalletCollectiveEvent;
        PalletCollectiveRawOrigin: PalletCollectiveRawOrigin;
        PalletCollectiveVotes: PalletCollectiveVotes;
        PalletDemocracyCall: PalletDemocracyCall;
        PalletDemocracyConviction: PalletDemocracyConviction;
        PalletDemocracyDelegations: PalletDemocracyDelegations;
        PalletDemocracyError: PalletDemocracyError;
        PalletDemocracyEvent: PalletDemocracyEvent;
        PalletDemocracyMetadataOwner: PalletDemocracyMetadataOwner;
        PalletDemocracyReferendumInfo: PalletDemocracyReferendumInfo;
        PalletDemocracyReferendumStatus: PalletDemocracyReferendumStatus;
        PalletDemocracyTally: PalletDemocracyTally;
        PalletDemocracyVoteAccountVote: PalletDemocracyVoteAccountVote;
        PalletDemocracyVotePriorLock: PalletDemocracyVotePriorLock;
        PalletDemocracyVoteThreshold: PalletDemocracyVoteThreshold;
        PalletDemocracyVoteVoting: PalletDemocracyVoteVoting;
        PalletDrop3Call: PalletDrop3Call;
        PalletDrop3Error: PalletDrop3Error;
        PalletDrop3Event: PalletDrop3Event;
        PalletDrop3RewardPool: PalletDrop3RewardPool;
        PalletExtrinsicFilterCall: PalletExtrinsicFilterCall;
        PalletExtrinsicFilterError: PalletExtrinsicFilterError;
        PalletExtrinsicFilterEvent: PalletExtrinsicFilterEvent;
        PalletExtrinsicFilterOperationalMode: PalletExtrinsicFilterOperationalMode;
        PalletGroupCall: PalletGroupCall;
        PalletGroupError: PalletGroupError;
        PalletGroupEvent: PalletGroupEvent;
        PalletIdentityBitFlags: PalletIdentityBitFlags;
        PalletIdentityCall: PalletIdentityCall;
        PalletIdentityError: PalletIdentityError;
        PalletIdentityEvent: PalletIdentityEvent;
        PalletIdentityIdentityField: PalletIdentityIdentityField;
        PalletIdentityIdentityInfo: PalletIdentityIdentityInfo;
        PalletIdentityJudgement: PalletIdentityJudgement;
        PalletIdentityManagementCall: PalletIdentityManagementCall;
        PalletIdentityManagementError: PalletIdentityManagementError;
        PalletIdentityManagementEvent: PalletIdentityManagementEvent;
        PalletIdentityManagementMockCall: PalletIdentityManagementMockCall;
        PalletIdentityManagementMockError: PalletIdentityManagementMockError;
        PalletIdentityManagementMockEvent: PalletIdentityManagementMockEvent;
        PalletIdentityManagementMockIdentityContext: PalletIdentityManagementMockIdentityContext;
        PalletIdentityRegistrarInfo: PalletIdentityRegistrarInfo;
        PalletIdentityRegistration: PalletIdentityRegistration;
        PalletMembershipCall: PalletMembershipCall;
        PalletMembershipError: PalletMembershipError;
        PalletMembershipEvent: PalletMembershipEvent;
        PalletMultisigCall: PalletMultisigCall;
        PalletMultisigError: PalletMultisigError;
        PalletMultisigEvent: PalletMultisigEvent;
        PalletMultisigMultisig: PalletMultisigMultisig;
        PalletMultisigTimepoint: PalletMultisigTimepoint;
        PalletParachainStakingAutoCompoundAutoCompoundConfig: PalletParachainStakingAutoCompoundAutoCompoundConfig;
        PalletParachainStakingBond: PalletParachainStakingBond;
        PalletParachainStakingBondWithAutoCompound: PalletParachainStakingBondWithAutoCompound;
        PalletParachainStakingCall: PalletParachainStakingCall;
        PalletParachainStakingCandidateBondLessRequest: PalletParachainStakingCandidateBondLessRequest;
        PalletParachainStakingCandidateMetadata: PalletParachainStakingCandidateMetadata;
        PalletParachainStakingCapacityStatus: PalletParachainStakingCapacityStatus;
        PalletParachainStakingCollatorSnapshot: PalletParachainStakingCollatorSnapshot;
        PalletParachainStakingCollatorStatus: PalletParachainStakingCollatorStatus;
        PalletParachainStakingDelayedPayout: PalletParachainStakingDelayedPayout;
        PalletParachainStakingDelegationRequestsCancelledScheduledRequest: PalletParachainStakingDelegationRequestsCancelledScheduledRequest;
        PalletParachainStakingDelegationRequestsDelegationAction: PalletParachainStakingDelegationRequestsDelegationAction;
        PalletParachainStakingDelegationRequestsScheduledRequest: PalletParachainStakingDelegationRequestsScheduledRequest;
        PalletParachainStakingDelegations: PalletParachainStakingDelegations;
        PalletParachainStakingDelegator: PalletParachainStakingDelegator;
        PalletParachainStakingDelegatorAdded: PalletParachainStakingDelegatorAdded;
        PalletParachainStakingDelegatorStatus: PalletParachainStakingDelegatorStatus;
        PalletParachainStakingError: PalletParachainStakingError;
        PalletParachainStakingEvent: PalletParachainStakingEvent;
        PalletParachainStakingInflationInflationInfo: PalletParachainStakingInflationInflationInfo;
        PalletParachainStakingParachainBondConfig: PalletParachainStakingParachainBondConfig;
        PalletParachainStakingRoundInfo: PalletParachainStakingRoundInfo;
        PalletParachainStakingSetOrderedSet: PalletParachainStakingSetOrderedSet;
        PalletPreimageCall: PalletPreimageCall;
        PalletPreimageError: PalletPreimageError;
        PalletPreimageEvent: PalletPreimageEvent;
        PalletPreimageRequestStatus: PalletPreimageRequestStatus;
        PalletProxyAnnouncement: PalletProxyAnnouncement;
        PalletProxyCall: PalletProxyCall;
        PalletProxyError: PalletProxyError;
        PalletProxyEvent: PalletProxyEvent;
        PalletProxyProxyDefinition: PalletProxyProxyDefinition;
        PalletSchedulerCall: PalletSchedulerCall;
        PalletSchedulerError: PalletSchedulerError;
        PalletSchedulerEvent: PalletSchedulerEvent;
        PalletSchedulerScheduled: PalletSchedulerScheduled;
        PalletSessionCall: PalletSessionCall;
        PalletSessionError: PalletSessionError;
        PalletSessionEvent: PalletSessionEvent;
        PalletSidechainCall: PalletSidechainCall;
        PalletSidechainError: PalletSidechainError;
        PalletSidechainEvent: PalletSidechainEvent;
        PalletSudoCall: PalletSudoCall;
        PalletSudoError: PalletSudoError;
        PalletSudoEvent: PalletSudoEvent;
        PalletTeeracleCall: PalletTeeracleCall;
        PalletTeeracleError: PalletTeeracleError;
        PalletTeeracleEvent: PalletTeeracleEvent;
        PalletTeerexCall: PalletTeerexCall;
        PalletTeerexError: PalletTeerexError;
        PalletTeerexEvent: PalletTeerexEvent;
        PalletTimestampCall: PalletTimestampCall;
        PalletTipsCall: PalletTipsCall;
        PalletTipsError: PalletTipsError;
        PalletTipsEvent: PalletTipsEvent;
        PalletTipsOpenTip: PalletTipsOpenTip;
        PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
        PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
        PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
        PalletTreasuryCall: PalletTreasuryCall;
        PalletTreasuryError: PalletTreasuryError;
        PalletTreasuryEvent: PalletTreasuryEvent;
        PalletTreasuryProposal: PalletTreasuryProposal;
        PalletUtilityCall: PalletUtilityCall;
        PalletUtilityError: PalletUtilityError;
        PalletUtilityEvent: PalletUtilityEvent;
        PalletVcManagementCall: PalletVcManagementCall;
        PalletVcManagementError: PalletVcManagementError;
        PalletVcManagementEvent: PalletVcManagementEvent;
        PalletVcManagementSchemaVcSchema: PalletVcManagementSchemaVcSchema;
        PalletVcManagementVcContext: PalletVcManagementVcContext;
        PalletVcManagementVcContextStatus: PalletVcManagementVcContextStatus;
        PalletVestingCall: PalletVestingCall;
        PalletVestingError: PalletVestingError;
        PalletVestingEvent: PalletVestingEvent;
        PalletVestingReleases: PalletVestingReleases;
        PalletVestingVestingInfo: PalletVestingVestingInfo;
        PalletXcmCall: PalletXcmCall;
        PalletXcmError: PalletXcmError;
        PalletXcmEvent: PalletXcmEvent;
        PalletXcmOrigin: PalletXcmOrigin;
        PalletXcmQueryStatus: PalletXcmQueryStatus;
        PalletXcmRemoteLockedFungibleRecord: PalletXcmRemoteLockedFungibleRecord;
        PalletXcmVersionMigrationStage: PalletXcmVersionMigrationStage;
        ParachainInfoCall: ParachainInfoCall;
        PolkadotCorePrimitivesInboundDownwardMessage: PolkadotCorePrimitivesInboundDownwardMessage;
        PolkadotCorePrimitivesInboundHrmpMessage: PolkadotCorePrimitivesInboundHrmpMessage;
        PolkadotCorePrimitivesOutboundHrmpMessage: PolkadotCorePrimitivesOutboundHrmpMessage;
        PolkadotParachainPrimitivesXcmpMessageFormat: PolkadotParachainPrimitivesXcmpMessageFormat;
        PolkadotPrimitivesV2AbridgedHostConfiguration: PolkadotPrimitivesV2AbridgedHostConfiguration;
        PolkadotPrimitivesV2AbridgedHrmpChannel: PolkadotPrimitivesV2AbridgedHrmpChannel;
        PolkadotPrimitivesV2PersistedValidationData: PolkadotPrimitivesV2PersistedValidationData;
        PolkadotPrimitivesV2UpgradeRestriction: PolkadotPrimitivesV2UpgradeRestriction;
        RococoParachainRuntimeOriginCaller: RococoParachainRuntimeOriginCaller;
        RococoParachainRuntimeProxyType: RococoParachainRuntimeProxyType;
        RococoParachainRuntimeRuntime: RococoParachainRuntimeRuntime;
        RococoParachainRuntimeSessionKeys: RococoParachainRuntimeSessionKeys;
        RuntimeCommonXcmImplCurrencyId: RuntimeCommonXcmImplCurrencyId;
        SidechainPrimitivesSidechainBlockConfirmation: SidechainPrimitivesSidechainBlockConfirmation;
        SpArithmeticArithmeticError: SpArithmeticArithmeticError;
        SpConsensusAuraSr25519AppSr25519Public: SpConsensusAuraSr25519AppSr25519Public;
        SpCoreCryptoKeyTypeId: SpCoreCryptoKeyTypeId;
        SpCoreEcdsaSignature: SpCoreEcdsaSignature;
        SpCoreEd25519Signature: SpCoreEd25519Signature;
        SpCoreSr25519Public: SpCoreSr25519Public;
        SpCoreSr25519Signature: SpCoreSr25519Signature;
        SpCoreVoid: SpCoreVoid;
        SpRuntimeDigest: SpRuntimeDigest;
        SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
        SpRuntimeDispatchError: SpRuntimeDispatchError;
        SpRuntimeModuleError: SpRuntimeModuleError;
        SpRuntimeMultiSignature: SpRuntimeMultiSignature;
        SpRuntimeTokenError: SpRuntimeTokenError;
        SpRuntimeTransactionalError: SpRuntimeTransactionalError;
        SpTrieStorageProof: SpTrieStorageProof;
        SpVersionRuntimeVersion: SpVersionRuntimeVersion;
        SpWeightsRuntimeDbWeight: SpWeightsRuntimeDbWeight;
        SpWeightsWeightV2Weight: SpWeightsWeightV2Weight;
        SubstrateFixedFixedU64: SubstrateFixedFixedU64;
        TeerexPrimitivesEnclave: TeerexPrimitivesEnclave;
        TeerexPrimitivesQeTcb: TeerexPrimitivesQeTcb;
        TeerexPrimitivesQuotingEnclave: TeerexPrimitivesQuotingEnclave;
        TeerexPrimitivesRequest: TeerexPrimitivesRequest;
        TeerexPrimitivesSgxBuildMode: TeerexPrimitivesSgxBuildMode;
        TeerexPrimitivesSgxEnclaveMetadata: TeerexPrimitivesSgxEnclaveMetadata;
        TeerexPrimitivesTcbInfoOnChain: TeerexPrimitivesTcbInfoOnChain;
        TeerexPrimitivesTcbVersionStatus: TeerexPrimitivesTcbVersionStatus;
        TypenumBitB0: TypenumBitB0;
        TypenumBitB1: TypenumBitB1;
        TypenumUIntUInt: TypenumUIntUInt;
        TypenumUIntUTerm: TypenumUIntUTerm;
        TypenumUintUTerm: TypenumUintUTerm;
        XcmDoubleEncoded: XcmDoubleEncoded;
        XcmV2BodyId: XcmV2BodyId;
        XcmV2BodyPart: XcmV2BodyPart;
        XcmV2Instruction: XcmV2Instruction;
        XcmV2Junction: XcmV2Junction;
        XcmV2MultiAsset: XcmV2MultiAsset;
        XcmV2MultiLocation: XcmV2MultiLocation;
        XcmV2MultiassetAssetId: XcmV2MultiassetAssetId;
        XcmV2MultiassetAssetInstance: XcmV2MultiassetAssetInstance;
        XcmV2MultiassetFungibility: XcmV2MultiassetFungibility;
        XcmV2MultiassetMultiAssetFilter: XcmV2MultiassetMultiAssetFilter;
        XcmV2MultiassetMultiAssets: XcmV2MultiassetMultiAssets;
        XcmV2MultiassetWildFungibility: XcmV2MultiassetWildFungibility;
        XcmV2MultiassetWildMultiAsset: XcmV2MultiassetWildMultiAsset;
        XcmV2MultilocationJunctions: XcmV2MultilocationJunctions;
        XcmV2NetworkId: XcmV2NetworkId;
        XcmV2OriginKind: XcmV2OriginKind;
        XcmV2Response: XcmV2Response;
        XcmV2TraitsError: XcmV2TraitsError;
        XcmV2WeightLimit: XcmV2WeightLimit;
        XcmV2Xcm: XcmV2Xcm;
        XcmV3Instruction: XcmV3Instruction;
        XcmV3Junction: XcmV3Junction;
        XcmV3JunctionBodyId: XcmV3JunctionBodyId;
        XcmV3JunctionBodyPart: XcmV3JunctionBodyPart;
        XcmV3JunctionNetworkId: XcmV3JunctionNetworkId;
        XcmV3Junctions: XcmV3Junctions;
        XcmV3MaybeErrorCode: XcmV3MaybeErrorCode;
        XcmV3MultiAsset: XcmV3MultiAsset;
        XcmV3MultiLocation: XcmV3MultiLocation;
        XcmV3MultiassetAssetId: XcmV3MultiassetAssetId;
        XcmV3MultiassetAssetInstance: XcmV3MultiassetAssetInstance;
        XcmV3MultiassetFungibility: XcmV3MultiassetFungibility;
        XcmV3MultiassetMultiAssetFilter: XcmV3MultiassetMultiAssetFilter;
        XcmV3MultiassetMultiAssets: XcmV3MultiassetMultiAssets;
        XcmV3MultiassetWildFungibility: XcmV3MultiassetWildFungibility;
        XcmV3MultiassetWildMultiAsset: XcmV3MultiassetWildMultiAsset;
        XcmV3PalletInfo: XcmV3PalletInfo;
        XcmV3QueryResponseInfo: XcmV3QueryResponseInfo;
        XcmV3Response: XcmV3Response;
        XcmV3TraitsError: XcmV3TraitsError;
        XcmV3TraitsOutcome: XcmV3TraitsOutcome;
        XcmV3WeightLimit: XcmV3WeightLimit;
        XcmV3Xcm: XcmV3Xcm;
        XcmVersionedAssetId: XcmVersionedAssetId;
        XcmVersionedMultiAsset: XcmVersionedMultiAsset;
        XcmVersionedMultiAssets: XcmVersionedMultiAssets;
        XcmVersionedMultiLocation: XcmVersionedMultiLocation;
        XcmVersionedResponse: XcmVersionedResponse;
        XcmVersionedXcm: XcmVersionedXcm;
    } // InterfaceTypes
} // declare module
