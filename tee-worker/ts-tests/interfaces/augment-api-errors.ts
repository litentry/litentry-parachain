// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/errors';

import type { ApiTypes, AugmentedError } from '@polkadot/api-base/types';

export type __AugmentedError<ApiType extends ApiTypes> = AugmentedError<ApiType>;

declare module '@polkadot/api-base/types/errors' {
    interface AugmentedErrors<ApiType extends ApiTypes> {
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
        claims: {
            /**
             * Invalid Ethereum signature.
             **/
            InvalidEthereumSignature: AugmentedError<ApiType>;
            /**
             * A needed statement was not included.
             **/
            InvalidStatement: AugmentedError<ApiType>;
            /**
             * There's not enough in the pot to pay out some unvested amount. Generally implies a logic
             * error.
             **/
            PotUnderflow: AugmentedError<ApiType>;
            /**
             * Account ID sending transaction has no claim.
             **/
            SenderHasNoClaim: AugmentedError<ApiType>;
            /**
             * Ethereum address has no claim.
             **/
            SignerHasNoClaim: AugmentedError<ApiType>;
            /**
             * The account already has a vested balance.
             **/
            VestedBalanceExists: AugmentedError<ApiType>;
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
        grandpa: {
            /**
             * Attempt to signal GRANDPA change with one already pending.
             **/
            ChangePending: AugmentedError<ApiType>;
            /**
             * A given equivocation report is valid but already previously reported.
             **/
            DuplicateOffenceReport: AugmentedError<ApiType>;
            /**
             * An equivocation proof provided as part of an equivocation report is invalid.
             **/
            InvalidEquivocationProof: AugmentedError<ApiType>;
            /**
             * A key ownership proof provided as part of an equivocation report is invalid.
             **/
            InvalidKeyOwnershipProof: AugmentedError<ApiType>;
            /**
             * Attempt to signal GRANDPA pause when the authority set isn't live
             * (either paused or already pending pause).
             **/
            PauseFailed: AugmentedError<ApiType>;
            /**
             * Attempt to signal GRANDPA resume when the authority set isn't paused
             * (either live or already pending resume).
             **/
            ResumeFailed: AugmentedError<ApiType>;
            /**
             * Cannot signal forced change so soon after last.
             **/
            TooSoon: AugmentedError<ApiType>;
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
    } // AugmentedErrors
} // declare module
