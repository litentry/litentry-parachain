// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/errors';

import type { ApiTypes, AugmentedError } from '@polkadot/api-base/types';

export type __AugmentedError<ApiType extends ApiTypes> = AugmentedError<ApiType>;

declare module '@polkadot/api-base/types/errors' {
    interface AugmentedErrors<ApiType extends ApiTypes> {
        assetManager: {
            AssetAlreadyExists: AugmentedError<ApiType>;
            AssetIdDoesNotExist: AugmentedError<ApiType>;
            AssetIdLimitReached: AugmentedError<ApiType>;
            AssetTypeDoesNotExist: AugmentedError<ApiType>;
            DefaultAssetTypeRemoved: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        balances: {
            /**
             * Beneficiary account must pre-exist
             **/
            DeadAccount: AugmentedError<ApiType>;
            /**
             * Value too low to create account due to existential deposit
             **/
            ExistentialDeposit: AugmentedError<ApiType>;
            /**
             * A vesting schedule already exists for this account
             **/
            ExistingVestingSchedule: AugmentedError<ApiType>;
            /**
             * Balance too low to send value.
             **/
            InsufficientBalance: AugmentedError<ApiType>;
            /**
             * Transfer/payment would kill account
             **/
            KeepAlive: AugmentedError<ApiType>;
            /**
             * Account liquidity restrictions prevent withdrawal
             **/
            LiquidityRestrictions: AugmentedError<ApiType>;
            /**
             * Number of named reserves exceed MaxReserves
             **/
            TooManyReserves: AugmentedError<ApiType>;
            /**
             * Vesting balance too high to send value
             **/
            VestingBalance: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        bounties: {
            /**
             * The bounty cannot be closed because it has active child bounties.
             **/
            HasActiveChildBounty: AugmentedError<ApiType>;
            /**
             * Proposer's balance is too low.
             **/
            InsufficientProposersBalance: AugmentedError<ApiType>;
            /**
             * Invalid bounty fee.
             **/
            InvalidFee: AugmentedError<ApiType>;
            /**
             * No proposal or bounty at that index.
             **/
            InvalidIndex: AugmentedError<ApiType>;
            /**
             * Invalid bounty value.
             **/
            InvalidValue: AugmentedError<ApiType>;
            /**
             * A bounty payout is pending.
             * To cancel the bounty, you must unassign and slash the curator.
             **/
            PendingPayout: AugmentedError<ApiType>;
            /**
             * The bounties cannot be claimed/closed because it's still in the countdown period.
             **/
            Premature: AugmentedError<ApiType>;
            /**
             * The reason given is just too big.
             **/
            ReasonTooBig: AugmentedError<ApiType>;
            /**
             * Require bounty curator.
             **/
            RequireCurator: AugmentedError<ApiType>;
            /**
             * Too many approvals are already queued.
             **/
            TooManyQueued: AugmentedError<ApiType>;
            /**
             * The bounty status is unexpected.
             **/
            UnexpectedStatus: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        bridgeTransfer: {
            InvalidCommand: AugmentedError<ApiType>;
            InvalidResourceId: AugmentedError<ApiType>;
            OverFlow: AugmentedError<ApiType>;
            ReachMaximumSupply: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        chainBridge: {
            CannotPayAsFee: AugmentedError<ApiType>;
            /**
             * Chain has already been enabled
             **/
            ChainAlreadyWhitelisted: AugmentedError<ApiType>;
            /**
             * Interactions with this chain is not permitted
             **/
            ChainNotWhitelisted: AugmentedError<ApiType>;
            /**
             * No fee with the ID was found
             **/
            FeeDoesNotExist: AugmentedError<ApiType>;
            /**
             * Too expensive fee for withdrawn asset
             **/
            FeeTooExpensive: AugmentedError<ApiType>;
            /**
             * Balance too low to withdraw
             **/
            InsufficientBalance: AugmentedError<ApiType>;
            /**
             * Provided chain Id is not valid
             **/
            InvalidChainId: AugmentedError<ApiType>;
            /**
             * Relayer threshold cannot be 0
             **/
            InvalidThreshold: AugmentedError<ApiType>;
            /**
             * Protected operation, must be performed by relayer
             **/
            MustBeRelayer: AugmentedError<ApiType>;
            NonceOverFlow: AugmentedError<ApiType>;
            /**
             * Proposal has either failed or succeeded
             **/
            ProposalAlreadyComplete: AugmentedError<ApiType>;
            /**
             * A proposal with these parameters has already been submitted
             **/
            ProposalAlreadyExists: AugmentedError<ApiType>;
            /**
             * No proposal with the ID was found
             **/
            ProposalDoesNotExist: AugmentedError<ApiType>;
            /**
             * Lifetime of proposal has been exceeded
             **/
            ProposalExpired: AugmentedError<ApiType>;
            /**
             * Cannot complete proposal, needs more votes
             **/
            ProposalNotComplete: AugmentedError<ApiType>;
            /**
             * Relayer already in set
             **/
            RelayerAlreadyExists: AugmentedError<ApiType>;
            /**
             * Relayer has already submitted some vote for this proposal
             **/
            RelayerAlreadyVoted: AugmentedError<ApiType>;
            /**
             * Provided accountId is not a relayer
             **/
            RelayerInvalid: AugmentedError<ApiType>;
            /**
             * Resource ID provided isn't mapped to anything
             **/
            ResourceDoesNotExist: AugmentedError<ApiType>;
            /**
             * Relayer threshold not set
             **/
            ThresholdNotSet: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        council: {
            /**
             * Members are already initialized!
             **/
            AlreadyInitialized: AugmentedError<ApiType>;
            /**
             * Duplicate proposals not allowed
             **/
            DuplicateProposal: AugmentedError<ApiType>;
            /**
             * Duplicate vote ignored
             **/
            DuplicateVote: AugmentedError<ApiType>;
            /**
             * Account is not a member
             **/
            NotMember: AugmentedError<ApiType>;
            /**
             * Proposal must exist
             **/
            ProposalMissing: AugmentedError<ApiType>;
            /**
             * The close call was made too early, before the end of the voting.
             **/
            TooEarly: AugmentedError<ApiType>;
            /**
             * There can only be a maximum of `MaxProposals` active proposals.
             **/
            TooManyProposals: AugmentedError<ApiType>;
            /**
             * Mismatched index
             **/
            WrongIndex: AugmentedError<ApiType>;
            /**
             * The given length bound for the proposal was too low.
             **/
            WrongProposalLength: AugmentedError<ApiType>;
            /**
             * The given weight bound for the proposal was too low.
             **/
            WrongProposalWeight: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        councilMembership: {
            /**
             * Already a member.
             **/
            AlreadyMember: AugmentedError<ApiType>;
            /**
             * Not a member.
             **/
            NotMember: AugmentedError<ApiType>;
            /**
             * Too many members.
             **/
            TooManyMembers: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        cumulusXcm: {
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        democracy: {
            /**
             * Cannot cancel the same proposal twice
             **/
            AlreadyCanceled: AugmentedError<ApiType>;
            /**
             * The account is already delegating.
             **/
            AlreadyDelegating: AugmentedError<ApiType>;
            /**
             * Identity may not veto a proposal twice
             **/
            AlreadyVetoed: AugmentedError<ApiType>;
            /**
             * Proposal already made
             **/
            DuplicateProposal: AugmentedError<ApiType>;
            /**
             * The instant referendum origin is currently disallowed.
             **/
            InstantNotAllowed: AugmentedError<ApiType>;
            /**
             * Too high a balance was provided that the account cannot afford.
             **/
            InsufficientFunds: AugmentedError<ApiType>;
            /**
             * Invalid hash
             **/
            InvalidHash: AugmentedError<ApiType>;
            /**
             * Maximum number of votes reached.
             **/
            MaxVotesReached: AugmentedError<ApiType>;
            /**
             * No proposals waiting
             **/
            NoneWaiting: AugmentedError<ApiType>;
            /**
             * Delegation to oneself makes no sense.
             **/
            Nonsense: AugmentedError<ApiType>;
            /**
             * The actor has no permission to conduct the action.
             **/
            NoPermission: AugmentedError<ApiType>;
            /**
             * No external proposal
             **/
            NoProposal: AugmentedError<ApiType>;
            /**
             * The account is not currently delegating.
             **/
            NotDelegating: AugmentedError<ApiType>;
            /**
             * Next external proposal not simple majority
             **/
            NotSimpleMajority: AugmentedError<ApiType>;
            /**
             * The given account did not vote on the referendum.
             **/
            NotVoter: AugmentedError<ApiType>;
            /**
             * The preimage does not exist.
             **/
            PreimageNotExist: AugmentedError<ApiType>;
            /**
             * Proposal still blacklisted
             **/
            ProposalBlacklisted: AugmentedError<ApiType>;
            /**
             * Proposal does not exist
             **/
            ProposalMissing: AugmentedError<ApiType>;
            /**
             * Vote given for invalid referendum
             **/
            ReferendumInvalid: AugmentedError<ApiType>;
            /**
             * Maximum number of items reached.
             **/
            TooMany: AugmentedError<ApiType>;
            /**
             * Value too low
             **/
            ValueLow: AugmentedError<ApiType>;
            /**
             * The account currently has votes attached to it and the operation cannot succeed until
             * these are removed, either through `unvote` or `reap_vote`.
             **/
            VotesExist: AugmentedError<ApiType>;
            /**
             * Voting period too low
             **/
            VotingPeriodLow: AugmentedError<ApiType>;
            /**
             * Invalid upper bound.
             **/
            WrongUpperBound: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        dmpQueue: {
            /**
             * The amount of weight given is possibly not enough for executing the message.
             **/
            OverLimit: AugmentedError<ApiType>;
            /**
             * The message index given is unknown.
             **/
            Unknown: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        drop3: {
            /**
             * Error when the remaning of a reward pool is not enough
             **/
            InsufficientRemain: AugmentedError<ApiType>;
            /**
             * Error when the sender doesn't have enough reserved balance
             **/
            InsufficientReservedBalance: AugmentedError<ApiType>;
            /**
             * Error when start_at < end_at when proposing reward pool
             **/
            InvalidProposedBlock: AugmentedError<ApiType>;
            /**
             * Error when `total` amount is 0 when proposing reward pool
             **/
            InvalidTotalBalance: AugmentedError<ApiType>;
            /**
             * Error when a reward pool can't be found
             **/
            NoSuchRewardPool: AugmentedError<ApiType>;
            /**
             * Error when no vacant PoolId can be acquired
             **/
            NoVacantPoolId: AugmentedError<ApiType>;
            /**
             * Error when the caller account is not the admin
             **/
            RequireAdmin: AugmentedError<ApiType>;
            /**
             * Error when the caller account is not the reward pool owner or admin
             **/
            RequireAdminOrRewardPoolOwner: AugmentedError<ApiType>;
            /**
             * Error when the caller account is not the reward pool owner
             **/
            RequireRewardPoolOwner: AugmentedError<ApiType>;
            /**
             * Error when the reward pool is first approved then rejected
             **/
            RewardPoolAlreadyApproved: AugmentedError<ApiType>;
            /**
             * Error when the reward pool is runing before `start_at`
             **/
            RewardPoolRanTooEarly: AugmentedError<ApiType>;
            /**
             * Error when the reward pool is runing after `end_at`
             **/
            RewardPoolRanTooLate: AugmentedError<ApiType>;
            /**
             * Error when the reward pool is stopped
             **/
            RewardPoolStopped: AugmentedError<ApiType>;
            /**
             * Error when the reward pool is unapproved
             **/
            RewardPoolUnapproved: AugmentedError<ApiType>;
            /**
             * Error of unexpected unmoved amount when calling repatriate_reserved
             **/
            UnexpectedUnMovedAmount: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        extrinsicFilter: {
            /**
             * Error when a given extrinsic cannot be blocked (e.g. this pallet)
             **/
            CannotBlock: AugmentedError<ApiType>;
            /**
             * Error during conversion bytes to utf8 string
             **/
            CannotConvertToString: AugmentedError<ApiType>;
            /**
             * Error when trying to block extrinsic more than once
             **/
            ExtrinsicAlreadyBlocked: AugmentedError<ApiType>;
            /**
             * Error when trying to unblock a non-existent extrinsic
             **/
            ExtrinsicNotBlocked: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        identityManagement: {
            /**
             * a delegatee doesn't exist
             **/
            DelegateeNotExist: AugmentedError<ApiType>;
            /**
             * a `create_identity` request from unauthorised user
             **/
            UnauthorisedUser: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        identityManagementMock: {
            /**
             * the challenge code doesn't exist
             **/
            ChallengeCodeNotExist: AugmentedError<ApiType>;
            /**
             * creating the prime identity manually is disallowed
             **/
            CreatePrimeIdentityNotAllowed: AugmentedError<ApiType>;
            /**
             * the creation request block is zero
             **/
            CreationRequestBlockZero: AugmentedError<ApiType>;
            /**
             * a delegatee doesn't exist
             **/
            DelegateeNotExist: AugmentedError<ApiType>;
            /**
             * identity already verified when creating an identity
             **/
            IdentityAlreadyVerified: AugmentedError<ApiType>;
            /**
             * identity not exist when removing an identity
             **/
            IdentityNotExist: AugmentedError<ApiType>;
            /**
             * fail to recover evm address
             **/
            RecoverEvmAddressFailed: AugmentedError<ApiType>;
            /**
             * recover substrate pubkey failed using ecdsa
             **/
            RecoverSubstratePubkeyFailed: AugmentedError<ApiType>;
            /**
             * Error when decrypting using TEE'shielding key
             **/
            ShieldingKeyDecryptionFailed: AugmentedError<ApiType>;
            /**
             * no shielding key for a given AccountId
             **/
            ShieldingKeyNotExist: AugmentedError<ApiType>;
            /**
             * a `create_identity` request from unauthorised user
             **/
            UnauthorisedUser: AugmentedError<ApiType>;
            /**
             * the message in validation data is unexpected
             **/
            UnexpectedMessage: AugmentedError<ApiType>;
            /**
             * a verification reqeust comes too early
             **/
            VerificationRequestTooEarly: AugmentedError<ApiType>;
            /**
             * a verification reqeust comes too late
             **/
            VerificationRequestTooLate: AugmentedError<ApiType>;
            /**
             * verify evm signature failed
             **/
            VerifyEvmSignatureFailed: AugmentedError<ApiType>;
            /**
             * verify substrate signature failed
             **/
            VerifySubstrateSignatureFailed: AugmentedError<ApiType>;
            /**
             * unexpected decoded type
             **/
            WrongDecodedType: AugmentedError<ApiType>;
            /**
             * wrong identity type
             **/
            WrongIdentityType: AugmentedError<ApiType>;
            /**
             * wrong signature type
             **/
            WrongSignatureType: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        impExtrinsicWhitelist: {
            /**
             * Group memeber already in set
             **/
            GroupMemberAlreadyExists: AugmentedError<ApiType>;
            /**
             * Provided accountId is not a Group member
             **/
            GroupMemberInvalid: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        multisig: {
            /**
             * Call is already approved by this signatory.
             **/
            AlreadyApproved: AugmentedError<ApiType>;
            /**
             * The data to be stored is already stored.
             **/
            AlreadyStored: AugmentedError<ApiType>;
            /**
             * The maximum weight information provided was too low.
             **/
            MaxWeightTooLow: AugmentedError<ApiType>;
            /**
             * Threshold must be 2 or greater.
             **/
            MinimumThreshold: AugmentedError<ApiType>;
            /**
             * Call doesn't need any (more) approvals.
             **/
            NoApprovalsNeeded: AugmentedError<ApiType>;
            /**
             * Multisig operation not found when attempting to cancel.
             **/
            NotFound: AugmentedError<ApiType>;
            /**
             * No timepoint was given, yet the multisig operation is already underway.
             **/
            NoTimepoint: AugmentedError<ApiType>;
            /**
             * Only the account that originally created the multisig is able to cancel it.
             **/
            NotOwner: AugmentedError<ApiType>;
            /**
             * The sender was contained in the other signatories; it shouldn't be.
             **/
            SenderInSignatories: AugmentedError<ApiType>;
            /**
             * The signatories were provided out of order; they should be ordered.
             **/
            SignatoriesOutOfOrder: AugmentedError<ApiType>;
            /**
             * There are too few signatories in the list.
             **/
            TooFewSignatories: AugmentedError<ApiType>;
            /**
             * There are too many signatories in the list.
             **/
            TooManySignatories: AugmentedError<ApiType>;
            /**
             * A timepoint was given, yet no multisig operation is underway.
             **/
            UnexpectedTimepoint: AugmentedError<ApiType>;
            /**
             * A different timepoint was given to the multisig operation that is underway.
             **/
            WrongTimepoint: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        parachainIdentity: {
            /**
             * Account ID is already named.
             **/
            AlreadyClaimed: AugmentedError<ApiType>;
            /**
             * Empty index.
             **/
            EmptyIndex: AugmentedError<ApiType>;
            /**
             * Fee is changed.
             **/
            FeeChanged: AugmentedError<ApiType>;
            /**
             * The index is invalid.
             **/
            InvalidIndex: AugmentedError<ApiType>;
            /**
             * Invalid judgement.
             **/
            InvalidJudgement: AugmentedError<ApiType>;
            /**
             * The target is invalid.
             **/
            InvalidTarget: AugmentedError<ApiType>;
            /**
             * The provided judgement was for a different identity.
             **/
            JudgementForDifferentIdentity: AugmentedError<ApiType>;
            /**
             * Judgement given.
             **/
            JudgementGiven: AugmentedError<ApiType>;
            /**
             * Error that occurs when there is an issue paying for judgement.
             **/
            JudgementPaymentFailed: AugmentedError<ApiType>;
            /**
             * No identity found.
             **/
            NoIdentity: AugmentedError<ApiType>;
            /**
             * Account isn't found.
             **/
            NotFound: AugmentedError<ApiType>;
            /**
             * Account isn't named.
             **/
            NotNamed: AugmentedError<ApiType>;
            /**
             * Sub-account isn't owned by sender.
             **/
            NotOwned: AugmentedError<ApiType>;
            /**
             * Sender is not a sub-account.
             **/
            NotSub: AugmentedError<ApiType>;
            /**
             * Sticky judgement.
             **/
            StickyJudgement: AugmentedError<ApiType>;
            /**
             * Too many additional fields.
             **/
            TooManyFields: AugmentedError<ApiType>;
            /**
             * Maximum amount of registrars reached. Cannot add any more.
             **/
            TooManyRegistrars: AugmentedError<ApiType>;
            /**
             * Too many subs-accounts.
             **/
            TooManySubAccounts: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        parachainStaking: {
            AlreadyActive: AugmentedError<ApiType>;
            AlreadyDelegatedCandidate: AugmentedError<ApiType>;
            AlreadyOffline: AugmentedError<ApiType>;
            CandidateAlreadyLeaving: AugmentedError<ApiType>;
            CandidateBondBelowMin: AugmentedError<ApiType>;
            CandidateCannotLeaveYet: AugmentedError<ApiType>;
            CandidateDNE: AugmentedError<ApiType>;
            CandidateExists: AugmentedError<ApiType>;
            CandidateNotLeaving: AugmentedError<ApiType>;
            CandidateUnauthorized: AugmentedError<ApiType>;
            CannotDelegateIfLeaving: AugmentedError<ApiType>;
            CannotDelegateLessThanOrEqualToLowestBottomWhenFull: AugmentedError<ApiType>;
            CannotGoOnlineIfLeaving: AugmentedError<ApiType>;
            CannotSetBelowMin: AugmentedError<ApiType>;
            DelegationBelowMin: AugmentedError<ApiType>;
            DelegationDNE: AugmentedError<ApiType>;
            DelegatorAlreadyLeaving: AugmentedError<ApiType>;
            DelegatorBondBelowMin: AugmentedError<ApiType>;
            DelegatorCannotLeaveYet: AugmentedError<ApiType>;
            DelegatorDNE: AugmentedError<ApiType>;
            DelegatorDNEInDelegatorSet: AugmentedError<ApiType>;
            DelegatorDNEinTopNorBottom: AugmentedError<ApiType>;
            DelegatorExists: AugmentedError<ApiType>;
            DelegatorNotLeaving: AugmentedError<ApiType>;
            ExceedMaxDelegationsPerDelegator: AugmentedError<ApiType>;
            InsufficientBalance: AugmentedError<ApiType>;
            InvalidSchedule: AugmentedError<ApiType>;
            NoWritingSameValue: AugmentedError<ApiType>;
            PendingCandidateRequestAlreadyExists: AugmentedError<ApiType>;
            PendingCandidateRequestNotDueYet: AugmentedError<ApiType>;
            PendingCandidateRequestsDNE: AugmentedError<ApiType>;
            PendingDelegationRequestAlreadyExists: AugmentedError<ApiType>;
            PendingDelegationRequestDNE: AugmentedError<ApiType>;
            PendingDelegationRequestNotDueYet: AugmentedError<ApiType>;
            PendingDelegationRevoke: AugmentedError<ApiType>;
            RoundLengthMustBeGreaterThanTotalSelectedCollators: AugmentedError<ApiType>;
            TooLowCandidateCountWeightHintCancelLeaveCandidates: AugmentedError<ApiType>;
            TooLowCandidateDelegationCountToLeaveCandidates: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        parachainSystem: {
            /**
             * The inherent which supplies the host configuration did not run this block
             **/
            HostConfigurationNotAvailable: AugmentedError<ApiType>;
            /**
             * No code upgrade has been authorized.
             **/
            NothingAuthorized: AugmentedError<ApiType>;
            /**
             * No validation function upgrade is currently scheduled.
             **/
            NotScheduled: AugmentedError<ApiType>;
            /**
             * Attempt to upgrade validation function while existing upgrade pending
             **/
            OverlappingUpgrades: AugmentedError<ApiType>;
            /**
             * Polkadot currently prohibits this parachain from upgrading its validation function
             **/
            ProhibitedByPolkadot: AugmentedError<ApiType>;
            /**
             * The supplied validation function has compiled into a blob larger than Polkadot is
             * willing to run
             **/
            TooBig: AugmentedError<ApiType>;
            /**
             * The given code upgrade has not been authorized.
             **/
            Unauthorized: AugmentedError<ApiType>;
            /**
             * The inherent which supplies the validation data did not run this block
             **/
            ValidationDataNotAvailable: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        polkadotXcm: {
            /**
             * The given account is not an identifiable sovereign account for any location.
             **/
            AccountNotSovereign: AugmentedError<ApiType>;
            /**
             * The location is invalid since it already has a subscription from us.
             **/
            AlreadySubscribed: AugmentedError<ApiType>;
            /**
             * The given location could not be used (e.g. because it cannot be expressed in the
             * desired version of XCM).
             **/
            BadLocation: AugmentedError<ApiType>;
            /**
             * The version of the `Versioned` value used is not able to be interpreted.
             **/
            BadVersion: AugmentedError<ApiType>;
            /**
             * Could not re-anchor the assets to declare the fees for the destination chain.
             **/
            CannotReanchor: AugmentedError<ApiType>;
            /**
             * The destination `MultiLocation` provided cannot be inverted.
             **/
            DestinationNotInvertible: AugmentedError<ApiType>;
            /**
             * The assets to be sent are empty.
             **/
            Empty: AugmentedError<ApiType>;
            /**
             * The operation required fees to be paid which the initiator could not meet.
             **/
            FeesNotMet: AugmentedError<ApiType>;
            /**
             * The message execution fails the filter.
             **/
            Filtered: AugmentedError<ApiType>;
            /**
             * The unlock operation cannot succeed because there are still users of the lock.
             **/
            InUse: AugmentedError<ApiType>;
            /**
             * Invalid asset for the operation.
             **/
            InvalidAsset: AugmentedError<ApiType>;
            /**
             * Origin is invalid for sending.
             **/
            InvalidOrigin: AugmentedError<ApiType>;
            /**
             * A remote lock with the corresponding data could not be found.
             **/
            LockNotFound: AugmentedError<ApiType>;
            /**
             * The owner does not own (all) of the asset that they wish to do the operation on.
             **/
            LowBalance: AugmentedError<ApiType>;
            /**
             * The referenced subscription could not be found.
             **/
            NoSubscription: AugmentedError<ApiType>;
            /**
             * There was some other issue (i.e. not to do with routing) in sending the message. Perhaps
             * a lack of space for buffering the message.
             **/
            SendFailure: AugmentedError<ApiType>;
            /**
             * Too many assets have been attempted for transfer.
             **/
            TooManyAssets: AugmentedError<ApiType>;
            /**
             * The asset owner has too many locks on the asset.
             **/
            TooManyLocks: AugmentedError<ApiType>;
            /**
             * The desired destination was unreachable, generally because there is a no way of routing
             * to it.
             **/
            Unreachable: AugmentedError<ApiType>;
            /**
             * The message's weight could not be determined.
             **/
            UnweighableMessage: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        preimage: {
            /**
             * Preimage has already been noted on-chain.
             **/
            AlreadyNoted: AugmentedError<ApiType>;
            /**
             * The user is not authorized to perform this action.
             **/
            NotAuthorized: AugmentedError<ApiType>;
            /**
             * The preimage cannot be removed since it has not yet been noted.
             **/
            NotNoted: AugmentedError<ApiType>;
            /**
             * The preimage request cannot be removed since no outstanding requests exist.
             **/
            NotRequested: AugmentedError<ApiType>;
            /**
             * A preimage may not be removed when there are outstanding requests.
             **/
            Requested: AugmentedError<ApiType>;
            /**
             * Preimage is too large to store on-chain.
             **/
            TooBig: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        proxy: {
            /**
             * Account is already a proxy.
             **/
            Duplicate: AugmentedError<ApiType>;
            /**
             * Call may not be made by proxy because it may escalate its privileges.
             **/
            NoPermission: AugmentedError<ApiType>;
            /**
             * Cannot add self as proxy.
             **/
            NoSelfProxy: AugmentedError<ApiType>;
            /**
             * Proxy registration not found.
             **/
            NotFound: AugmentedError<ApiType>;
            /**
             * Sender is not a proxy of the account to be proxied.
             **/
            NotProxy: AugmentedError<ApiType>;
            /**
             * There are too many proxies registered or too many announcements pending.
             **/
            TooMany: AugmentedError<ApiType>;
            /**
             * Announcement, if made at all, was made too recently.
             **/
            Unannounced: AugmentedError<ApiType>;
            /**
             * A call which is incompatible with the proxy type's filter was attempted.
             **/
            Unproxyable: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        scheduler: {
            /**
             * Failed to schedule a call
             **/
            FailedToSchedule: AugmentedError<ApiType>;
            /**
             * Attempt to use a non-named function on a named task.
             **/
            Named: AugmentedError<ApiType>;
            /**
             * Cannot find the scheduled call.
             **/
            NotFound: AugmentedError<ApiType>;
            /**
             * Reschedule failed because it does not change scheduled time.
             **/
            RescheduleNoChange: AugmentedError<ApiType>;
            /**
             * Given target block number is in the past.
             **/
            TargetBlockNumberInPast: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        session: {
            /**
             * Registered duplicate key.
             **/
            DuplicatedKey: AugmentedError<ApiType>;
            /**
             * Invalid ownership proof.
             **/
            InvalidProof: AugmentedError<ApiType>;
            /**
             * Key setting account is not live, so it's impossible to associate keys.
             **/
            NoAccount: AugmentedError<ApiType>;
            /**
             * No associated validator ID for account.
             **/
            NoAssociatedValidatorId: AugmentedError<ApiType>;
            /**
             * No keys are associated with this account.
             **/
            NoKeys: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        sidechain: {
            /**
             * The value for the next finalization candidate is invalid.
             **/
            InvalidNextFinalizationCandidateBlockNumber: AugmentedError<ApiType>;
            /**
             * A proposed block is unexpected.
             **/
            ReceivedUnexpectedSidechainBlock: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        sudo: {
            /**
             * Sender must be the Sudo account
             **/
            RequireSudo: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        system: {
            /**
             * The origin filter prevent the call to be dispatched.
             **/
            CallFiltered: AugmentedError<ApiType>;
            /**
             * Failed to extract the runtime version from the new runtime.
             *
             * Either calling `Core_version` or decoding `RuntimeVersion` failed.
             **/
            FailedToExtractRuntimeVersion: AugmentedError<ApiType>;
            /**
             * The name of specification does not match between the current runtime
             * and the new runtime.
             **/
            InvalidSpecName: AugmentedError<ApiType>;
            /**
             * Suicide called when the account has non-default composite data.
             **/
            NonDefaultComposite: AugmentedError<ApiType>;
            /**
             * There is a non-zero reference count preventing the account from being purged.
             **/
            NonZeroRefCount: AugmentedError<ApiType>;
            /**
             * The specification version is not allowed to decrease between the current runtime
             * and the new runtime.
             **/
            SpecVersionNeedsToIncrease: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        technicalCommittee: {
            /**
             * Members are already initialized!
             **/
            AlreadyInitialized: AugmentedError<ApiType>;
            /**
             * Duplicate proposals not allowed
             **/
            DuplicateProposal: AugmentedError<ApiType>;
            /**
             * Duplicate vote ignored
             **/
            DuplicateVote: AugmentedError<ApiType>;
            /**
             * Account is not a member
             **/
            NotMember: AugmentedError<ApiType>;
            /**
             * Proposal must exist
             **/
            ProposalMissing: AugmentedError<ApiType>;
            /**
             * The close call was made too early, before the end of the voting.
             **/
            TooEarly: AugmentedError<ApiType>;
            /**
             * There can only be a maximum of `MaxProposals` active proposals.
             **/
            TooManyProposals: AugmentedError<ApiType>;
            /**
             * Mismatched index
             **/
            WrongIndex: AugmentedError<ApiType>;
            /**
             * The given length bound for the proposal was too low.
             **/
            WrongProposalLength: AugmentedError<ApiType>;
            /**
             * The given weight bound for the proposal was too low.
             **/
            WrongProposalWeight: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        technicalCommitteeMembership: {
            /**
             * Already a member.
             **/
            AlreadyMember: AugmentedError<ApiType>;
            /**
             * Not a member.
             **/
            NotMember: AugmentedError<ApiType>;
            /**
             * Too many members.
             **/
            TooManyMembers: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        teeracle: {
            DataSourceStringTooLong: AugmentedError<ApiType>;
            InvalidCurrency: AugmentedError<ApiType>;
            OracleBlobTooBig: AugmentedError<ApiType>;
            OracleDataNameStringTooLong: AugmentedError<ApiType>;
            ReleaseAlreadyWhitelisted: AugmentedError<ApiType>;
            ReleaseNotWhitelisted: AugmentedError<ApiType>;
            /**
             * Too many MrEnclave in the whitelist.
             **/
            ReleaseWhitelistOverflow: AugmentedError<ApiType>;
            TradingPairStringTooLong: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        teerex: {
            /**
             * The provided collateral data is invalid
             **/
            CollateralInvalid: AugmentedError<ApiType>;
            /**
             * The length of the `data` passed to `publish_hash` exceeds the limit.
             **/
            DataTooLong: AugmentedError<ApiType>;
            /**
             * No enclave is registered.
             **/
            EmptyEnclaveRegistry: AugmentedError<ApiType>;
            /**
             * The enclave is not registered.
             **/
            EnclaveIsNotRegistered: AugmentedError<ApiType>;
            /**
             * Enclave not in the scheduled list, therefore unexpected.
             **/
            EnclaveNotInSchedule: AugmentedError<ApiType>;
            /**
             * Failed to decode enclave signer.
             **/
            EnclaveSignerDecodeError: AugmentedError<ApiType>;
            /**
             * The worker url is too long.
             **/
            EnclaveUrlTooLong: AugmentedError<ApiType>;
            /**
             * The Remote Attestation report is too long.
             **/
            RaReportTooLong: AugmentedError<ApiType>;
            RemoteAttestationTooOld: AugmentedError<ApiType>;
            /**
             * Verifying RA report failed.
             **/
            RemoteAttestationVerificationFailed: AugmentedError<ApiType>;
            /**
             * This operation needs the admin permission
             **/
            RequireAdmin: AugmentedError<ApiType>;
            /**
             * Can not found the desired scheduled enclave.
             **/
            ScheduledEnclaveNotExist: AugmentedError<ApiType>;
            /**
             * Sender does not match attested enclave in report.
             **/
            SenderIsNotAttestedEnclave: AugmentedError<ApiType>;
            /**
             * The enclave cannot attest, because its building mode is not allowed.
             **/
            SgxModeNotAllowed: AugmentedError<ApiType>;
            /**
             * The number of `extra_topics` passed to `publish_hash` exceeds the limit.
             **/
            TooManyTopics: AugmentedError<ApiType>;
            /**
             * The bonding account doesn't match the enclave.
             **/
            WrongMrenclaveForBondingAccount: AugmentedError<ApiType>;
            /**
             * The shard doesn't match the enclave.
             **/
            WrongMrenclaveForShard: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        tips: {
            /**
             * The tip was already found/started.
             **/
            AlreadyKnown: AugmentedError<ApiType>;
            /**
             * The account attempting to retract the tip is not the finder of the tip.
             **/
            NotFinder: AugmentedError<ApiType>;
            /**
             * The tip cannot be claimed/closed because it's still in the countdown period.
             **/
            Premature: AugmentedError<ApiType>;
            /**
             * The reason given is just too big.
             **/
            ReasonTooBig: AugmentedError<ApiType>;
            /**
             * The tip cannot be claimed/closed because there are not enough tippers yet.
             **/
            StillOpen: AugmentedError<ApiType>;
            /**
             * The tip hash is unknown.
             **/
            UnknownTip: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        tokens: {
            /**
             * Cannot convert Amount into Balance type
             **/
            AmountIntoBalanceFailed: AugmentedError<ApiType>;
            /**
             * The balance is too low
             **/
            BalanceTooLow: AugmentedError<ApiType>;
            /**
             * Beneficiary account must pre-exist
             **/
            DeadAccount: AugmentedError<ApiType>;
            /**
             * Value too low to create account due to existential deposit
             **/
            ExistentialDeposit: AugmentedError<ApiType>;
            /**
             * Transfer/payment would kill account
             **/
            KeepAlive: AugmentedError<ApiType>;
            /**
             * Failed because liquidity restrictions due to locking
             **/
            LiquidityRestrictions: AugmentedError<ApiType>;
            /**
             * Failed because the maximum locks was exceeded
             **/
            MaxLocksExceeded: AugmentedError<ApiType>;
            TooManyReserves: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        treasury: {
            /**
             * The spend origin is valid but the amount it is allowed to spend is lower than the
             * amount to be spent.
             **/
            InsufficientPermission: AugmentedError<ApiType>;
            /**
             * Proposer's balance is too low.
             **/
            InsufficientProposersBalance: AugmentedError<ApiType>;
            /**
             * No proposal or bounty at that index.
             **/
            InvalidIndex: AugmentedError<ApiType>;
            /**
             * Proposal has not been approved.
             **/
            ProposalNotApproved: AugmentedError<ApiType>;
            /**
             * Too many approvals in the queue.
             **/
            TooManyApprovals: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        utility: {
            /**
             * Too many calls batched.
             **/
            TooManyCalls: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        vcManagement: {
            LengthMismatch: AugmentedError<ApiType>;
            /**
             * Error when the caller account is not the admin
             **/
            RequireAdmin: AugmentedError<ApiType>;
            /**
             * Schema is active
             **/
            SchemaAlreadyActivated: AugmentedError<ApiType>;
            /**
             * Schema is already disabled
             **/
            SchemaAlreadyDisabled: AugmentedError<ApiType>;
            SchemaIndexOverFlow: AugmentedError<ApiType>;
            /**
             * Schema not exists
             **/
            SchemaNotExists: AugmentedError<ApiType>;
            /**
             * The VC is already disabled
             **/
            VCAlreadyDisabled: AugmentedError<ApiType>;
            /**
             * the VC already exists
             **/
            VCAlreadyExists: AugmentedError<ApiType>;
            /**
             * the ID doesn't exist
             **/
            VCNotExist: AugmentedError<ApiType>;
            /**
             * The requester doesn't have the permission (because of subject mismatch)
             **/
            VCSubjectMismatch: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        vcmpExtrinsicWhitelist: {
            /**
             * Group memeber already in set
             **/
            GroupMemberAlreadyExists: AugmentedError<ApiType>;
            /**
             * Provided accountId is not a Group member
             **/
            GroupMemberInvalid: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        vesting: {
            /**
             * Amount being transferred is too low to create a vesting schedule.
             **/
            AmountLow: AugmentedError<ApiType>;
            /**
             * The account already has `MaxVestingSchedules` count of schedules and thus
             * cannot add another one. Consider merging existing schedules in order to add another.
             **/
            AtMaxVestingSchedules: AugmentedError<ApiType>;
            /**
             * Failed to create a new schedule because some parameter was invalid.
             **/
            InvalidScheduleParams: AugmentedError<ApiType>;
            /**
             * The account given is not vesting.
             **/
            NotVesting: AugmentedError<ApiType>;
            /**
             * An index was out of bounds of the vesting schedules.
             **/
            ScheduleIndexOutOfBounds: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        xcmpQueue: {
            /**
             * Bad overweight index.
             **/
            BadOverweightIndex: AugmentedError<ApiType>;
            /**
             * Bad XCM data.
             **/
            BadXcm: AugmentedError<ApiType>;
            /**
             * Bad XCM origin.
             **/
            BadXcmOrigin: AugmentedError<ApiType>;
            /**
             * Failed to send XCM message.
             **/
            FailedToSend: AugmentedError<ApiType>;
            /**
             * Provided weight is possibly not enough to execute the message.
             **/
            WeightOverLimit: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        xTokens: {
            /**
             * Asset has no reserve location.
             **/
            AssetHasNoReserve: AugmentedError<ApiType>;
            /**
             * The specified index does not exist in a MultiAssets struct.
             **/
            AssetIndexNonExistent: AugmentedError<ApiType>;
            /**
             * The version of the `Versioned` value used is not able to be
             * interpreted.
             **/
            BadVersion: AugmentedError<ApiType>;
            /**
             * Could not re-anchor the assets to declare the fees for the
             * destination chain.
             **/
            CannotReanchor: AugmentedError<ApiType>;
            /**
             * The destination `MultiLocation` provided cannot be inverted.
             **/
            DestinationNotInvertible: AugmentedError<ApiType>;
            /**
             * We tried sending distinct asset and fee but they have different
             * reserve chains.
             **/
            DistinctReserveForAssetAndFee: AugmentedError<ApiType>;
            /**
             * Fee is not enough.
             **/
            FeeNotEnough: AugmentedError<ApiType>;
            /**
             * Could not get ancestry of asset reserve location.
             **/
            InvalidAncestry: AugmentedError<ApiType>;
            /**
             * The MultiAsset is invalid.
             **/
            InvalidAsset: AugmentedError<ApiType>;
            /**
             * Invalid transfer destination.
             **/
            InvalidDest: AugmentedError<ApiType>;
            /**
             * MinXcmFee not registered for certain reserve location
             **/
            MinXcmFeeNotDefined: AugmentedError<ApiType>;
            /**
             * Not cross-chain transfer.
             **/
            NotCrossChainTransfer: AugmentedError<ApiType>;
            /**
             * Currency is not cross-chain transferable.
             **/
            NotCrossChainTransferableCurrency: AugmentedError<ApiType>;
            /**
             * Not supported MultiLocation
             **/
            NotSupportedMultiLocation: AugmentedError<ApiType>;
            /**
             * The number of assets to be sent is over the maximum.
             **/
            TooManyAssetsBeingSent: AugmentedError<ApiType>;
            /**
             * The message's weight could not be determined.
             **/
            UnweighableMessage: AugmentedError<ApiType>;
            /**
             * XCM execution failed.
             **/
            XcmExecutionFailed: AugmentedError<ApiType>;
            /**
             * The transfering asset amount is zero.
             **/
            ZeroAmount: AugmentedError<ApiType>;
            /**
             * The fee is zero.
             **/
            ZeroFee: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
    } // AugmentedErrors
} // declare module
