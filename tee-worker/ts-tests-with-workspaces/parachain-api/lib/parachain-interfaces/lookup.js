"use strict";
// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */
Object.defineProperty(exports, "__esModule", { value: true });
/* eslint-disable sort-keys */
exports.default = {
    /**
     * Lookup3: frame_system::AccountInfo<Index, pallet_balances::AccountData<Balance>>
     **/
    FrameSystemAccountInfo: {
        nonce: 'u32',
        consumers: 'u32',
        providers: 'u32',
        sufficients: 'u32',
        data: 'PalletBalancesAccountData',
    },
    /**
     * Lookup5: pallet_balances::AccountData<Balance>
     **/
    PalletBalancesAccountData: {
        free: 'u128',
        reserved: 'u128',
        miscFrozen: 'u128',
        feeFrozen: 'u128',
    },
    /**
     * Lookup7: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
     **/
    FrameSupportDispatchPerDispatchClassWeight: {
        normal: 'SpWeightsWeightV2Weight',
        operational: 'SpWeightsWeightV2Weight',
        mandatory: 'SpWeightsWeightV2Weight',
    },
    /**
     * Lookup8: sp_weights::weight_v2::Weight
     **/
    SpWeightsWeightV2Weight: {
        refTime: 'Compact<u64>',
        proofSize: 'Compact<u64>',
    },
    /**
     * Lookup13: sp_runtime::generic::digest::Digest
     **/
    SpRuntimeDigest: {
        logs: 'Vec<SpRuntimeDigestDigestItem>',
    },
    /**
     * Lookup15: sp_runtime::generic::digest::DigestItem
     **/
    SpRuntimeDigestDigestItem: {
        _enum: {
            Other: 'Bytes',
            __Unused1: 'Null',
            __Unused2: 'Null',
            __Unused3: 'Null',
            Consensus: '([u8;4],Bytes)',
            Seal: '([u8;4],Bytes)',
            PreRuntime: '([u8;4],Bytes)',
            __Unused7: 'Null',
            RuntimeEnvironmentUpdated: 'Null',
        },
    },
    /**
     * Lookup18: frame_system::EventRecord<rococo_parachain_runtime::RuntimeEvent, primitive_types::H256>
     **/
    FrameSystemEventRecord: {
        phase: 'FrameSystemPhase',
        event: 'Event',
        topics: 'Vec<H256>',
    },
    /**
     * Lookup20: frame_system::pallet::Event<T>
     **/
    FrameSystemEvent: {
        _enum: {
            ExtrinsicSuccess: {
                dispatchInfo: 'FrameSupportDispatchDispatchInfo',
            },
            ExtrinsicFailed: {
                dispatchError: 'SpRuntimeDispatchError',
                dispatchInfo: 'FrameSupportDispatchDispatchInfo',
            },
            CodeUpdated: 'Null',
            NewAccount: {
                account: 'AccountId32',
            },
            KilledAccount: {
                account: 'AccountId32',
            },
            Remarked: {
                _alias: {
                    hash_: 'hash',
                },
                sender: 'AccountId32',
                hash_: 'H256',
            },
        },
    },
    /**
     * Lookup21: frame_support::dispatch::DispatchInfo
     **/
    FrameSupportDispatchDispatchInfo: {
        weight: 'SpWeightsWeightV2Weight',
        class: 'FrameSupportDispatchDispatchClass',
        paysFee: 'FrameSupportDispatchPays',
    },
    /**
     * Lookup22: frame_support::dispatch::DispatchClass
     **/
    FrameSupportDispatchDispatchClass: {
        _enum: ['Normal', 'Operational', 'Mandatory'],
    },
    /**
     * Lookup23: frame_support::dispatch::Pays
     **/
    FrameSupportDispatchPays: {
        _enum: ['Yes', 'No'],
    },
    /**
     * Lookup24: sp_runtime::DispatchError
     **/
    SpRuntimeDispatchError: {
        _enum: {
            Other: 'Null',
            CannotLookup: 'Null',
            BadOrigin: 'Null',
            Module: 'SpRuntimeModuleError',
            ConsumerRemaining: 'Null',
            NoProviders: 'Null',
            TooManyConsumers: 'Null',
            Token: 'SpRuntimeTokenError',
            Arithmetic: 'SpArithmeticArithmeticError',
            Transactional: 'SpRuntimeTransactionalError',
            Exhausted: 'Null',
            Corruption: 'Null',
            Unavailable: 'Null',
        },
    },
    /**
     * Lookup25: sp_runtime::ModuleError
     **/
    SpRuntimeModuleError: {
        index: 'u8',
        error: '[u8;4]',
    },
    /**
     * Lookup26: sp_runtime::TokenError
     **/
    SpRuntimeTokenError: {
        _enum: ['NoFunds', 'WouldDie', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported'],
    },
    /**
     * Lookup27: sp_arithmetic::ArithmeticError
     **/
    SpArithmeticArithmeticError: {
        _enum: ['Underflow', 'Overflow', 'DivisionByZero'],
    },
    /**
     * Lookup28: sp_runtime::TransactionalError
     **/
    SpRuntimeTransactionalError: {
        _enum: ['LimitReached', 'NoLayer'],
    },
    /**
     * Lookup29: pallet_scheduler::pallet::Event<T>
     **/
    PalletSchedulerEvent: {
        _enum: {
            Scheduled: {
                when: 'u32',
                index: 'u32',
            },
            Canceled: {
                when: 'u32',
                index: 'u32',
            },
            Dispatched: {
                task: '(u32,u32)',
                id: 'Option<[u8;32]>',
                result: 'Result<Null, SpRuntimeDispatchError>',
            },
            CallUnavailable: {
                task: '(u32,u32)',
                id: 'Option<[u8;32]>',
            },
            PeriodicFailed: {
                task: '(u32,u32)',
                id: 'Option<[u8;32]>',
            },
            PermanentlyOverweight: {
                task: '(u32,u32)',
                id: 'Option<[u8;32]>',
            },
        },
    },
    /**
     * Lookup34: pallet_utility::pallet::Event
     **/
    PalletUtilityEvent: {
        _enum: {
            BatchInterrupted: {
                index: 'u32',
                error: 'SpRuntimeDispatchError',
            },
            BatchCompleted: 'Null',
            BatchCompletedWithErrors: 'Null',
            ItemCompleted: 'Null',
            ItemFailed: {
                error: 'SpRuntimeDispatchError',
            },
            DispatchedAs: {
                result: 'Result<Null, SpRuntimeDispatchError>',
            },
        },
    },
    /**
     * Lookup35: pallet_multisig::pallet::Event<T>
     **/
    PalletMultisigEvent: {
        _enum: {
            NewMultisig: {
                approving: 'AccountId32',
                multisig: 'AccountId32',
                callHash: '[u8;32]',
            },
            MultisigApproval: {
                approving: 'AccountId32',
                timepoint: 'PalletMultisigTimepoint',
                multisig: 'AccountId32',
                callHash: '[u8;32]',
            },
            MultisigExecuted: {
                approving: 'AccountId32',
                timepoint: 'PalletMultisigTimepoint',
                multisig: 'AccountId32',
                callHash: '[u8;32]',
                result: 'Result<Null, SpRuntimeDispatchError>',
            },
            MultisigCancelled: {
                cancelling: 'AccountId32',
                timepoint: 'PalletMultisigTimepoint',
                multisig: 'AccountId32',
                callHash: '[u8;32]',
            },
        },
    },
    /**
     * Lookup36: pallet_multisig::Timepoint<BlockNumber>
     **/
    PalletMultisigTimepoint: {
        height: 'u32',
        index: 'u32',
    },
    /**
     * Lookup37: pallet_proxy::pallet::Event<T>
     **/
    PalletProxyEvent: {
        _enum: {
            ProxyExecuted: {
                result: 'Result<Null, SpRuntimeDispatchError>',
            },
            PureCreated: {
                pure: 'AccountId32',
                who: 'AccountId32',
                proxyType: 'RococoParachainRuntimeProxyType',
                disambiguationIndex: 'u16',
            },
            Announced: {
                real: 'AccountId32',
                proxy: 'AccountId32',
                callHash: 'H256',
            },
            ProxyAdded: {
                delegator: 'AccountId32',
                delegatee: 'AccountId32',
                proxyType: 'RococoParachainRuntimeProxyType',
                delay: 'u32',
            },
            ProxyRemoved: {
                delegator: 'AccountId32',
                delegatee: 'AccountId32',
                proxyType: 'RococoParachainRuntimeProxyType',
                delay: 'u32',
            },
        },
    },
    /**
     * Lookup38: rococo_parachain_runtime::ProxyType
     **/
    RococoParachainRuntimeProxyType: {
        _enum: ['Any', 'NonTransfer', 'CancelProxy', 'Collator', 'Governance'],
    },
    /**
     * Lookup40: pallet_preimage::pallet::Event<T>
     **/
    PalletPreimageEvent: {
        _enum: {
            Noted: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
            },
            Requested: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
            },
            Cleared: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
            },
        },
    },
    /**
     * Lookup41: pallet_balances::pallet::Event<T, I>
     **/
    PalletBalancesEvent: {
        _enum: {
            Endowed: {
                account: 'AccountId32',
                freeBalance: 'u128',
            },
            DustLost: {
                account: 'AccountId32',
                amount: 'u128',
            },
            Transfer: {
                from: 'AccountId32',
                to: 'AccountId32',
                amount: 'u128',
            },
            BalanceSet: {
                who: 'AccountId32',
                free: 'u128',
                reserved: 'u128',
            },
            Reserved: {
                who: 'AccountId32',
                amount: 'u128',
            },
            Unreserved: {
                who: 'AccountId32',
                amount: 'u128',
            },
            ReserveRepatriated: {
                from: 'AccountId32',
                to: 'AccountId32',
                amount: 'u128',
                destinationStatus: 'FrameSupportTokensMiscBalanceStatus',
            },
            Deposit: {
                who: 'AccountId32',
                amount: 'u128',
            },
            Withdraw: {
                who: 'AccountId32',
                amount: 'u128',
            },
            Slashed: {
                who: 'AccountId32',
                amount: 'u128',
            },
        },
    },
    /**
     * Lookup42: frame_support::traits::tokens::misc::BalanceStatus
     **/
    FrameSupportTokensMiscBalanceStatus: {
        _enum: ['Free', 'Reserved'],
    },
    /**
     * Lookup43: pallet_vesting::pallet::Event<T>
     **/
    PalletVestingEvent: {
        _enum: {
            VestingUpdated: {
                account: 'AccountId32',
                unvested: 'u128',
            },
            VestingCompleted: {
                account: 'AccountId32',
            },
        },
    },
    /**
     * Lookup44: pallet_transaction_payment::pallet::Event<T>
     **/
    PalletTransactionPaymentEvent: {
        _enum: {
            TransactionFeePaid: {
                who: 'AccountId32',
                actualFee: 'u128',
                tip: 'u128',
            },
        },
    },
    /**
     * Lookup45: pallet_treasury::pallet::Event<T, I>
     **/
    PalletTreasuryEvent: {
        _enum: {
            Proposed: {
                proposalIndex: 'u32',
            },
            Spending: {
                budgetRemaining: 'u128',
            },
            Awarded: {
                proposalIndex: 'u32',
                award: 'u128',
                account: 'AccountId32',
            },
            Rejected: {
                proposalIndex: 'u32',
                slashed: 'u128',
            },
            Burnt: {
                burntFunds: 'u128',
            },
            Rollover: {
                rolloverBalance: 'u128',
            },
            Deposit: {
                value: 'u128',
            },
            SpendApproved: {
                proposalIndex: 'u32',
                amount: 'u128',
                beneficiary: 'AccountId32',
            },
            UpdatedInactive: {
                reactivated: 'u128',
                deactivated: 'u128',
            },
        },
    },
    /**
     * Lookup46: pallet_democracy::pallet::Event<T>
     **/
    PalletDemocracyEvent: {
        _enum: {
            Proposed: {
                proposalIndex: 'u32',
                deposit: 'u128',
            },
            Tabled: {
                proposalIndex: 'u32',
                deposit: 'u128',
            },
            ExternalTabled: 'Null',
            Started: {
                refIndex: 'u32',
                threshold: 'PalletDemocracyVoteThreshold',
            },
            Passed: {
                refIndex: 'u32',
            },
            NotPassed: {
                refIndex: 'u32',
            },
            Cancelled: {
                refIndex: 'u32',
            },
            Delegated: {
                who: 'AccountId32',
                target: 'AccountId32',
            },
            Undelegated: {
                account: 'AccountId32',
            },
            Vetoed: {
                who: 'AccountId32',
                proposalHash: 'H256',
                until: 'u32',
            },
            Blacklisted: {
                proposalHash: 'H256',
            },
            Voted: {
                voter: 'AccountId32',
                refIndex: 'u32',
                vote: 'PalletDemocracyVoteAccountVote',
            },
            Seconded: {
                seconder: 'AccountId32',
                propIndex: 'u32',
            },
            ProposalCanceled: {
                propIndex: 'u32',
            },
            MetadataSet: {
                _alias: {
                    hash_: 'hash',
                },
                owner: 'PalletDemocracyMetadataOwner',
                hash_: 'H256',
            },
            MetadataCleared: {
                _alias: {
                    hash_: 'hash',
                },
                owner: 'PalletDemocracyMetadataOwner',
                hash_: 'H256',
            },
            MetadataTransferred: {
                _alias: {
                    hash_: 'hash',
                },
                prevOwner: 'PalletDemocracyMetadataOwner',
                owner: 'PalletDemocracyMetadataOwner',
                hash_: 'H256',
            },
        },
    },
    /**
     * Lookup47: pallet_democracy::vote_threshold::VoteThreshold
     **/
    PalletDemocracyVoteThreshold: {
        _enum: ['SuperMajorityApprove', 'SuperMajorityAgainst', 'SimpleMajority'],
    },
    /**
     * Lookup48: pallet_democracy::vote::AccountVote<Balance>
     **/
    PalletDemocracyVoteAccountVote: {
        _enum: {
            Standard: {
                vote: 'Vote',
                balance: 'u128',
            },
            Split: {
                aye: 'u128',
                nay: 'u128',
            },
        },
    },
    /**
     * Lookup50: pallet_democracy::types::MetadataOwner
     **/
    PalletDemocracyMetadataOwner: {
        _enum: {
            External: 'Null',
            Proposal: 'u32',
            Referendum: 'u32',
        },
    },
    /**
     * Lookup51: pallet_collective::pallet::Event<T, I>
     **/
    PalletCollectiveEvent: {
        _enum: {
            Proposed: {
                account: 'AccountId32',
                proposalIndex: 'u32',
                proposalHash: 'H256',
                threshold: 'u32',
            },
            Voted: {
                account: 'AccountId32',
                proposalHash: 'H256',
                voted: 'bool',
                yes: 'u32',
                no: 'u32',
            },
            Approved: {
                proposalHash: 'H256',
            },
            Disapproved: {
                proposalHash: 'H256',
            },
            Executed: {
                proposalHash: 'H256',
                result: 'Result<Null, SpRuntimeDispatchError>',
            },
            MemberExecuted: {
                proposalHash: 'H256',
                result: 'Result<Null, SpRuntimeDispatchError>',
            },
            Closed: {
                proposalHash: 'H256',
                yes: 'u32',
                no: 'u32',
            },
        },
    },
    /**
     * Lookup53: pallet_membership::pallet::Event<T, I>
     **/
    PalletMembershipEvent: {
        _enum: ['MemberAdded', 'MemberRemoved', 'MembersSwapped', 'MembersReset', 'KeyChanged', 'Dummy'],
    },
    /**
     * Lookup56: pallet_bounties::pallet::Event<T, I>
     **/
    PalletBountiesEvent: {
        _enum: {
            BountyProposed: {
                index: 'u32',
            },
            BountyRejected: {
                index: 'u32',
                bond: 'u128',
            },
            BountyBecameActive: {
                index: 'u32',
            },
            BountyAwarded: {
                index: 'u32',
                beneficiary: 'AccountId32',
            },
            BountyClaimed: {
                index: 'u32',
                payout: 'u128',
                beneficiary: 'AccountId32',
            },
            BountyCanceled: {
                index: 'u32',
            },
            BountyExtended: {
                index: 'u32',
            },
        },
    },
    /**
     * Lookup57: pallet_tips::pallet::Event<T, I>
     **/
    PalletTipsEvent: {
        _enum: {
            NewTip: {
                tipHash: 'H256',
            },
            TipClosing: {
                tipHash: 'H256',
            },
            TipClosed: {
                tipHash: 'H256',
                who: 'AccountId32',
                payout: 'u128',
            },
            TipRetracted: {
                tipHash: 'H256',
            },
            TipSlashed: {
                tipHash: 'H256',
                finder: 'AccountId32',
                deposit: 'u128',
            },
        },
    },
    /**
     * Lookup58: pallet_identity::pallet::Event<T>
     **/
    PalletIdentityEvent: {
        _enum: {
            IdentitySet: {
                who: 'AccountId32',
            },
            IdentityCleared: {
                who: 'AccountId32',
                deposit: 'u128',
            },
            IdentityKilled: {
                who: 'AccountId32',
                deposit: 'u128',
            },
            JudgementRequested: {
                who: 'AccountId32',
                registrarIndex: 'u32',
            },
            JudgementUnrequested: {
                who: 'AccountId32',
                registrarIndex: 'u32',
            },
            JudgementGiven: {
                target: 'AccountId32',
                registrarIndex: 'u32',
            },
            RegistrarAdded: {
                registrarIndex: 'u32',
            },
            SubIdentityAdded: {
                sub: 'AccountId32',
                main: 'AccountId32',
                deposit: 'u128',
            },
            SubIdentityRemoved: {
                sub: 'AccountId32',
                main: 'AccountId32',
                deposit: 'u128',
            },
            SubIdentityRevoked: {
                sub: 'AccountId32',
                main: 'AccountId32',
                deposit: 'u128',
            },
        },
    },
    /**
     * Lookup59: cumulus_pallet_parachain_system::pallet::Event<T>
     **/
    CumulusPalletParachainSystemEvent: {
        _enum: {
            ValidationFunctionStored: 'Null',
            ValidationFunctionApplied: {
                relayChainBlockNum: 'u32',
            },
            ValidationFunctionDiscarded: 'Null',
            UpgradeAuthorized: {
                codeHash: 'H256',
            },
            DownwardMessagesReceived: {
                count: 'u32',
            },
            DownwardMessagesProcessed: {
                weightUsed: 'SpWeightsWeightV2Weight',
                dmqHead: 'H256',
            },
            UpwardMessageSent: {
                messageHash: 'Option<[u8;32]>',
            },
        },
    },
    /**
     * Lookup60: pallet_session::pallet::Event
     **/
    PalletSessionEvent: {
        _enum: {
            NewSession: {
                sessionIndex: 'u32',
            },
        },
    },
    /**
     * Lookup61: pallet_parachain_staking::pallet::Event<T>
     **/
    PalletParachainStakingEvent: {
        _enum: {
            NewRound: {
                startingBlock: 'u32',
                round: 'u32',
                selectedCollatorsNumber: 'u32',
                totalBalance: 'u128',
            },
            JoinedCollatorCandidates: {
                account: 'AccountId32',
                amountLocked: 'u128',
                newTotalAmtLocked: 'u128',
            },
            CollatorChosen: {
                round: 'u32',
                collatorAccount: 'AccountId32',
                totalExposedAmount: 'u128',
            },
            CandidateBondLessRequested: {
                candidate: 'AccountId32',
                amountToDecrease: 'u128',
                executeRound: 'u32',
            },
            CandidateBondedMore: {
                candidate: 'AccountId32',
                amount: 'u128',
                newTotalBond: 'u128',
            },
            CandidateBondedLess: {
                candidate: 'AccountId32',
                amount: 'u128',
                newBond: 'u128',
            },
            CandidateWentOffline: {
                candidate: 'AccountId32',
            },
            CandidateBackOnline: {
                candidate: 'AccountId32',
            },
            CandidateScheduledExit: {
                exitAllowedRound: 'u32',
                candidate: 'AccountId32',
                scheduledExit: 'u32',
            },
            CancelledCandidateExit: {
                candidate: 'AccountId32',
            },
            CancelledCandidateBondLess: {
                candidate: 'AccountId32',
                amount: 'u128',
                executeRound: 'u32',
            },
            CandidateLeft: {
                exCandidate: 'AccountId32',
                unlockedAmount: 'u128',
                newTotalAmtLocked: 'u128',
            },
            DelegationDecreaseScheduled: {
                delegator: 'AccountId32',
                candidate: 'AccountId32',
                amountToDecrease: 'u128',
                executeRound: 'u32',
            },
            DelegationIncreased: {
                delegator: 'AccountId32',
                candidate: 'AccountId32',
                amount: 'u128',
                inTop: 'bool',
            },
            DelegationDecreased: {
                delegator: 'AccountId32',
                candidate: 'AccountId32',
                amount: 'u128',
                inTop: 'bool',
            },
            DelegatorExitScheduled: {
                round: 'u32',
                delegator: 'AccountId32',
                scheduledExit: 'u32',
            },
            DelegationRevocationScheduled: {
                round: 'u32',
                delegator: 'AccountId32',
                candidate: 'AccountId32',
                scheduledExit: 'u32',
            },
            DelegatorLeft: {
                delegator: 'AccountId32',
                unstakedAmount: 'u128',
            },
            DelegationRevoked: {
                delegator: 'AccountId32',
                candidate: 'AccountId32',
                unstakedAmount: 'u128',
            },
            DelegationKicked: {
                delegator: 'AccountId32',
                candidate: 'AccountId32',
                unstakedAmount: 'u128',
            },
            DelegatorExitCancelled: {
                delegator: 'AccountId32',
            },
            CancelledDelegationRequest: {
                delegator: 'AccountId32',
                cancelledRequest: 'PalletParachainStakingDelegationRequestsCancelledScheduledRequest',
                collator: 'AccountId32',
            },
            Delegation: {
                delegator: 'AccountId32',
                lockedAmount: 'u128',
                candidate: 'AccountId32',
                delegatorPosition: 'PalletParachainStakingDelegatorAdded',
                autoCompound: 'Percent',
            },
            DelegatorLeftCandidate: {
                delegator: 'AccountId32',
                candidate: 'AccountId32',
                unstakedAmount: 'u128',
                totalCandidateStaked: 'u128',
            },
            Rewarded: {
                account: 'AccountId32',
                rewards: 'u128',
            },
            ReservedForParachainBond: {
                account: 'AccountId32',
                value: 'u128',
            },
            ParachainBondAccountSet: {
                _alias: {
                    new_: 'new',
                },
                old: 'AccountId32',
                new_: 'AccountId32',
            },
            ParachainBondReservePercentSet: {
                _alias: {
                    new_: 'new',
                },
                old: 'Percent',
                new_: 'Percent',
            },
            InflationSet: {
                annualMin: 'Perbill',
                annualIdeal: 'Perbill',
                annualMax: 'Perbill',
                roundMin: 'Perbill',
                roundIdeal: 'Perbill',
                roundMax: 'Perbill',
            },
            StakeExpectationsSet: {
                expectMin: 'u128',
                expectIdeal: 'u128',
                expectMax: 'u128',
            },
            TotalSelectedSet: {
                _alias: {
                    new_: 'new',
                },
                old: 'u32',
                new_: 'u32',
            },
            CollatorCommissionSet: {
                _alias: {
                    new_: 'new',
                },
                old: 'Perbill',
                new_: 'Perbill',
            },
            BlocksPerRoundSet: {
                _alias: {
                    new_: 'new',
                },
                currentRound: 'u32',
                firstBlock: 'u32',
                old: 'u32',
                new_: 'u32',
                newPerRoundInflationMin: 'Perbill',
                newPerRoundInflationIdeal: 'Perbill',
                newPerRoundInflationMax: 'Perbill',
            },
            CandidateWhiteListAdded: {
                candidate: 'AccountId32',
            },
            CandidateWhiteListRemoved: {
                candidate: 'AccountId32',
            },
            AutoCompoundSet: {
                candidate: 'AccountId32',
                delegator: 'AccountId32',
                value: 'Percent',
            },
            Compounded: {
                candidate: 'AccountId32',
                delegator: 'AccountId32',
                amount: 'u128',
            },
        },
    },
    /**
     * Lookup62: pallet_parachain_staking::delegation_requests::CancelledScheduledRequest<Balance>
     **/
    PalletParachainStakingDelegationRequestsCancelledScheduledRequest: {
        whenExecutable: 'u32',
        action: 'PalletParachainStakingDelegationRequestsDelegationAction',
    },
    /**
     * Lookup63: pallet_parachain_staking::delegation_requests::DelegationAction<Balance>
     **/
    PalletParachainStakingDelegationRequestsDelegationAction: {
        _enum: {
            Revoke: 'u128',
            Decrease: 'u128',
        },
    },
    /**
     * Lookup64: pallet_parachain_staking::types::DelegatorAdded<B>
     **/
    PalletParachainStakingDelegatorAdded: {
        _enum: {
            AddedToTop: {
                newTotal: 'u128',
            },
            AddedToBottom: 'Null',
        },
    },
    /**
     * Lookup67: cumulus_pallet_xcmp_queue::pallet::Event<T>
     **/
    CumulusPalletXcmpQueueEvent: {
        _enum: {
            Success: {
                messageHash: 'Option<[u8;32]>',
                weight: 'SpWeightsWeightV2Weight',
            },
            Fail: {
                messageHash: 'Option<[u8;32]>',
                error: 'XcmV3TraitsError',
                weight: 'SpWeightsWeightV2Weight',
            },
            BadVersion: {
                messageHash: 'Option<[u8;32]>',
            },
            BadFormat: {
                messageHash: 'Option<[u8;32]>',
            },
            XcmpMessageSent: {
                messageHash: 'Option<[u8;32]>',
            },
            OverweightEnqueued: {
                sender: 'u32',
                sentAt: 'u32',
                index: 'u64',
                required: 'SpWeightsWeightV2Weight',
            },
            OverweightServiced: {
                index: 'u64',
                used: 'SpWeightsWeightV2Weight',
            },
        },
    },
    /**
     * Lookup68: xcm::v3::traits::Error
     **/
    XcmV3TraitsError: {
        _enum: {
            Overflow: 'Null',
            Unimplemented: 'Null',
            UntrustedReserveLocation: 'Null',
            UntrustedTeleportLocation: 'Null',
            LocationFull: 'Null',
            LocationNotInvertible: 'Null',
            BadOrigin: 'Null',
            InvalidLocation: 'Null',
            AssetNotFound: 'Null',
            FailedToTransactAsset: 'Null',
            NotWithdrawable: 'Null',
            LocationCannotHold: 'Null',
            ExceedsMaxMessageSize: 'Null',
            DestinationUnsupported: 'Null',
            Transport: 'Null',
            Unroutable: 'Null',
            UnknownClaim: 'Null',
            FailedToDecode: 'Null',
            MaxWeightInvalid: 'Null',
            NotHoldingFees: 'Null',
            TooExpensive: 'Null',
            Trap: 'u64',
            ExpectationFalse: 'Null',
            PalletNotFound: 'Null',
            NameMismatch: 'Null',
            VersionIncompatible: 'Null',
            HoldingWouldOverflow: 'Null',
            ExportError: 'Null',
            ReanchorFailed: 'Null',
            NoDeal: 'Null',
            FeesNotMet: 'Null',
            LockError: 'Null',
            NoPermission: 'Null',
            Unanchored: 'Null',
            NotDepositable: 'Null',
            UnhandledXcmVersion: 'Null',
            WeightLimitReached: 'SpWeightsWeightV2Weight',
            Barrier: 'Null',
            WeightNotComputable: 'Null',
            ExceedsStackLimit: 'Null',
        },
    },
    /**
     * Lookup70: pallet_xcm::pallet::Event<T>
     **/
    PalletXcmEvent: {
        _enum: {
            Attempted: 'XcmV3TraitsOutcome',
            Sent: '(XcmV3MultiLocation,XcmV3MultiLocation,XcmV3Xcm)',
            UnexpectedResponse: '(XcmV3MultiLocation,u64)',
            ResponseReady: '(u64,XcmV3Response)',
            Notified: '(u64,u8,u8)',
            NotifyOverweight: '(u64,u8,u8,SpWeightsWeightV2Weight,SpWeightsWeightV2Weight)',
            NotifyDispatchError: '(u64,u8,u8)',
            NotifyDecodeFailed: '(u64,u8,u8)',
            InvalidResponder: '(XcmV3MultiLocation,u64,Option<XcmV3MultiLocation>)',
            InvalidResponderVersion: '(XcmV3MultiLocation,u64)',
            ResponseTaken: 'u64',
            AssetsTrapped: '(H256,XcmV3MultiLocation,XcmVersionedMultiAssets)',
            VersionChangeNotified: '(XcmV3MultiLocation,u32,XcmV3MultiassetMultiAssets)',
            SupportedVersionChanged: '(XcmV3MultiLocation,u32)',
            NotifyTargetSendFail: '(XcmV3MultiLocation,u64,XcmV3TraitsError)',
            NotifyTargetMigrationFail: '(XcmVersionedMultiLocation,u64)',
            InvalidQuerierVersion: '(XcmV3MultiLocation,u64)',
            InvalidQuerier: '(XcmV3MultiLocation,u64,XcmV3MultiLocation,Option<XcmV3MultiLocation>)',
            VersionNotifyStarted: '(XcmV3MultiLocation,XcmV3MultiassetMultiAssets)',
            VersionNotifyRequested: '(XcmV3MultiLocation,XcmV3MultiassetMultiAssets)',
            VersionNotifyUnrequested: '(XcmV3MultiLocation,XcmV3MultiassetMultiAssets)',
            FeesPaid: '(XcmV3MultiLocation,XcmV3MultiassetMultiAssets)',
            AssetsClaimed: '(H256,XcmV3MultiLocation,XcmVersionedMultiAssets)',
        },
    },
    /**
     * Lookup71: xcm::v3::traits::Outcome
     **/
    XcmV3TraitsOutcome: {
        _enum: {
            Complete: 'SpWeightsWeightV2Weight',
            Incomplete: '(SpWeightsWeightV2Weight,XcmV3TraitsError)',
            Error: 'XcmV3TraitsError',
        },
    },
    /**
     * Lookup72: xcm::v3::multilocation::MultiLocation
     **/
    XcmV3MultiLocation: {
        parents: 'u8',
        interior: 'XcmV3Junctions',
    },
    /**
     * Lookup73: xcm::v3::junctions::Junctions
     **/
    XcmV3Junctions: {
        _enum: {
            Here: 'Null',
            X1: 'XcmV3Junction',
            X2: '(XcmV3Junction,XcmV3Junction)',
            X3: '(XcmV3Junction,XcmV3Junction,XcmV3Junction)',
            X4: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
            X5: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
            X6: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
            X7: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
            X8: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
        },
    },
    /**
     * Lookup74: xcm::v3::junction::Junction
     **/
    XcmV3Junction: {
        _enum: {
            Parachain: 'Compact<u32>',
            AccountId32: {
                network: 'Option<XcmV3JunctionNetworkId>',
                id: '[u8;32]',
            },
            AccountIndex64: {
                network: 'Option<XcmV3JunctionNetworkId>',
                index: 'Compact<u64>',
            },
            AccountKey20: {
                network: 'Option<XcmV3JunctionNetworkId>',
                key: '[u8;20]',
            },
            PalletInstance: 'u8',
            GeneralIndex: 'Compact<u128>',
            GeneralKey: {
                length: 'u8',
                data: '[u8;32]',
            },
            OnlyChild: 'Null',
            Plurality: {
                id: 'XcmV3JunctionBodyId',
                part: 'XcmV3JunctionBodyPart',
            },
            GlobalConsensus: 'XcmV3JunctionNetworkId',
        },
    },
    /**
     * Lookup77: xcm::v3::junction::NetworkId
     **/
    XcmV3JunctionNetworkId: {
        _enum: {
            ByGenesis: '[u8;32]',
            ByFork: {
                blockNumber: 'u64',
                blockHash: '[u8;32]',
            },
            Polkadot: 'Null',
            Kusama: 'Null',
            Westend: 'Null',
            Rococo: 'Null',
            Wococo: 'Null',
            Ethereum: {
                chainId: 'Compact<u64>',
            },
            BitcoinCore: 'Null',
            BitcoinCash: 'Null',
        },
    },
    /**
     * Lookup80: xcm::v3::junction::BodyId
     **/
    XcmV3JunctionBodyId: {
        _enum: {
            Unit: 'Null',
            Moniker: '[u8;4]',
            Index: 'Compact<u32>',
            Executive: 'Null',
            Technical: 'Null',
            Legislative: 'Null',
            Judicial: 'Null',
            Defense: 'Null',
            Administration: 'Null',
            Treasury: 'Null',
        },
    },
    /**
     * Lookup81: xcm::v3::junction::BodyPart
     **/
    XcmV3JunctionBodyPart: {
        _enum: {
            Voice: 'Null',
            Members: {
                count: 'Compact<u32>',
            },
            Fraction: {
                nom: 'Compact<u32>',
                denom: 'Compact<u32>',
            },
            AtLeastProportion: {
                nom: 'Compact<u32>',
                denom: 'Compact<u32>',
            },
            MoreThanProportion: {
                nom: 'Compact<u32>',
                denom: 'Compact<u32>',
            },
        },
    },
    /**
     * Lookup82: xcm::v3::Xcm<Call>
     **/
    XcmV3Xcm: 'Vec<XcmV3Instruction>',
    /**
     * Lookup84: xcm::v3::Instruction<Call>
     **/
    XcmV3Instruction: {
        _enum: {
            WithdrawAsset: 'XcmV3MultiassetMultiAssets',
            ReserveAssetDeposited: 'XcmV3MultiassetMultiAssets',
            ReceiveTeleportedAsset: 'XcmV3MultiassetMultiAssets',
            QueryResponse: {
                queryId: 'Compact<u64>',
                response: 'XcmV3Response',
                maxWeight: 'SpWeightsWeightV2Weight',
                querier: 'Option<XcmV3MultiLocation>',
            },
            TransferAsset: {
                assets: 'XcmV3MultiassetMultiAssets',
                beneficiary: 'XcmV3MultiLocation',
            },
            TransferReserveAsset: {
                assets: 'XcmV3MultiassetMultiAssets',
                dest: 'XcmV3MultiLocation',
                xcm: 'XcmV3Xcm',
            },
            Transact: {
                originKind: 'XcmV2OriginKind',
                requireWeightAtMost: 'SpWeightsWeightV2Weight',
                call: 'XcmDoubleEncoded',
            },
            HrmpNewChannelOpenRequest: {
                sender: 'Compact<u32>',
                maxMessageSize: 'Compact<u32>',
                maxCapacity: 'Compact<u32>',
            },
            HrmpChannelAccepted: {
                recipient: 'Compact<u32>',
            },
            HrmpChannelClosing: {
                initiator: 'Compact<u32>',
                sender: 'Compact<u32>',
                recipient: 'Compact<u32>',
            },
            ClearOrigin: 'Null',
            DescendOrigin: 'XcmV3Junctions',
            ReportError: 'XcmV3QueryResponseInfo',
            DepositAsset: {
                assets: 'XcmV3MultiassetMultiAssetFilter',
                beneficiary: 'XcmV3MultiLocation',
            },
            DepositReserveAsset: {
                assets: 'XcmV3MultiassetMultiAssetFilter',
                dest: 'XcmV3MultiLocation',
                xcm: 'XcmV3Xcm',
            },
            ExchangeAsset: {
                give: 'XcmV3MultiassetMultiAssetFilter',
                want: 'XcmV3MultiassetMultiAssets',
                maximal: 'bool',
            },
            InitiateReserveWithdraw: {
                assets: 'XcmV3MultiassetMultiAssetFilter',
                reserve: 'XcmV3MultiLocation',
                xcm: 'XcmV3Xcm',
            },
            InitiateTeleport: {
                assets: 'XcmV3MultiassetMultiAssetFilter',
                dest: 'XcmV3MultiLocation',
                xcm: 'XcmV3Xcm',
            },
            ReportHolding: {
                responseInfo: 'XcmV3QueryResponseInfo',
                assets: 'XcmV3MultiassetMultiAssetFilter',
            },
            BuyExecution: {
                fees: 'XcmV3MultiAsset',
                weightLimit: 'XcmV3WeightLimit',
            },
            RefundSurplus: 'Null',
            SetErrorHandler: 'XcmV3Xcm',
            SetAppendix: 'XcmV3Xcm',
            ClearError: 'Null',
            ClaimAsset: {
                assets: 'XcmV3MultiassetMultiAssets',
                ticket: 'XcmV3MultiLocation',
            },
            Trap: 'Compact<u64>',
            SubscribeVersion: {
                queryId: 'Compact<u64>',
                maxResponseWeight: 'SpWeightsWeightV2Weight',
            },
            UnsubscribeVersion: 'Null',
            BurnAsset: 'XcmV3MultiassetMultiAssets',
            ExpectAsset: 'XcmV3MultiassetMultiAssets',
            ExpectOrigin: 'Option<XcmV3MultiLocation>',
            ExpectError: 'Option<(u32,XcmV3TraitsError)>',
            ExpectTransactStatus: 'XcmV3MaybeErrorCode',
            QueryPallet: {
                moduleName: 'Bytes',
                responseInfo: 'XcmV3QueryResponseInfo',
            },
            ExpectPallet: {
                index: 'Compact<u32>',
                name: 'Bytes',
                moduleName: 'Bytes',
                crateMajor: 'Compact<u32>',
                minCrateMinor: 'Compact<u32>',
            },
            ReportTransactStatus: 'XcmV3QueryResponseInfo',
            ClearTransactStatus: 'Null',
            UniversalOrigin: 'XcmV3Junction',
            ExportMessage: {
                network: 'XcmV3JunctionNetworkId',
                destination: 'XcmV3Junctions',
                xcm: 'XcmV3Xcm',
            },
            LockAsset: {
                asset: 'XcmV3MultiAsset',
                unlocker: 'XcmV3MultiLocation',
            },
            UnlockAsset: {
                asset: 'XcmV3MultiAsset',
                target: 'XcmV3MultiLocation',
            },
            NoteUnlockable: {
                asset: 'XcmV3MultiAsset',
                owner: 'XcmV3MultiLocation',
            },
            RequestUnlock: {
                asset: 'XcmV3MultiAsset',
                locker: 'XcmV3MultiLocation',
            },
            SetFeesMode: {
                jitWithdraw: 'bool',
            },
            SetTopic: '[u8;32]',
            ClearTopic: 'Null',
            AliasOrigin: 'XcmV3MultiLocation',
            UnpaidExecution: {
                weightLimit: 'XcmV3WeightLimit',
                checkOrigin: 'Option<XcmV3MultiLocation>',
            },
        },
    },
    /**
     * Lookup85: xcm::v3::multiasset::MultiAssets
     **/
    XcmV3MultiassetMultiAssets: 'Vec<XcmV3MultiAsset>',
    /**
     * Lookup87: xcm::v3::multiasset::MultiAsset
     **/
    XcmV3MultiAsset: {
        id: 'XcmV3MultiassetAssetId',
        fun: 'XcmV3MultiassetFungibility',
    },
    /**
     * Lookup88: xcm::v3::multiasset::AssetId
     **/
    XcmV3MultiassetAssetId: {
        _enum: {
            Concrete: 'XcmV3MultiLocation',
            Abstract: '[u8;32]',
        },
    },
    /**
     * Lookup89: xcm::v3::multiasset::Fungibility
     **/
    XcmV3MultiassetFungibility: {
        _enum: {
            Fungible: 'Compact<u128>',
            NonFungible: 'XcmV3MultiassetAssetInstance',
        },
    },
    /**
     * Lookup90: xcm::v3::multiasset::AssetInstance
     **/
    XcmV3MultiassetAssetInstance: {
        _enum: {
            Undefined: 'Null',
            Index: 'Compact<u128>',
            Array4: '[u8;4]',
            Array8: '[u8;8]',
            Array16: '[u8;16]',
            Array32: '[u8;32]',
        },
    },
    /**
     * Lookup93: xcm::v3::Response
     **/
    XcmV3Response: {
        _enum: {
            Null: 'Null',
            Assets: 'XcmV3MultiassetMultiAssets',
            ExecutionResult: 'Option<(u32,XcmV3TraitsError)>',
            Version: 'u32',
            PalletsInfo: 'Vec<XcmV3PalletInfo>',
            DispatchResult: 'XcmV3MaybeErrorCode',
        },
    },
    /**
     * Lookup97: xcm::v3::PalletInfo
     **/
    XcmV3PalletInfo: {
        index: 'Compact<u32>',
        name: 'Bytes',
        moduleName: 'Bytes',
        major: 'Compact<u32>',
        minor: 'Compact<u32>',
        patch: 'Compact<u32>',
    },
    /**
     * Lookup100: xcm::v3::MaybeErrorCode
     **/
    XcmV3MaybeErrorCode: {
        _enum: {
            Success: 'Null',
            Error: 'Bytes',
            TruncatedError: 'Bytes',
        },
    },
    /**
     * Lookup103: xcm::v2::OriginKind
     **/
    XcmV2OriginKind: {
        _enum: ['Native', 'SovereignAccount', 'Superuser', 'Xcm'],
    },
    /**
     * Lookup104: xcm::double_encoded::DoubleEncoded<T>
     **/
    XcmDoubleEncoded: {
        encoded: 'Bytes',
    },
    /**
     * Lookup105: xcm::v3::QueryResponseInfo
     **/
    XcmV3QueryResponseInfo: {
        destination: 'XcmV3MultiLocation',
        queryId: 'Compact<u64>',
        maxWeight: 'SpWeightsWeightV2Weight',
    },
    /**
     * Lookup106: xcm::v3::multiasset::MultiAssetFilter
     **/
    XcmV3MultiassetMultiAssetFilter: {
        _enum: {
            Definite: 'XcmV3MultiassetMultiAssets',
            Wild: 'XcmV3MultiassetWildMultiAsset',
        },
    },
    /**
     * Lookup107: xcm::v3::multiasset::WildMultiAsset
     **/
    XcmV3MultiassetWildMultiAsset: {
        _enum: {
            All: 'Null',
            AllOf: {
                id: 'XcmV3MultiassetAssetId',
                fun: 'XcmV3MultiassetWildFungibility',
            },
            AllCounted: 'Compact<u32>',
            AllOfCounted: {
                id: 'XcmV3MultiassetAssetId',
                fun: 'XcmV3MultiassetWildFungibility',
                count: 'Compact<u32>',
            },
        },
    },
    /**
     * Lookup108: xcm::v3::multiasset::WildFungibility
     **/
    XcmV3MultiassetWildFungibility: {
        _enum: ['Fungible', 'NonFungible'],
    },
    /**
     * Lookup109: xcm::v3::WeightLimit
     **/
    XcmV3WeightLimit: {
        _enum: {
            Unlimited: 'Null',
            Limited: 'SpWeightsWeightV2Weight',
        },
    },
    /**
     * Lookup110: xcm::VersionedMultiAssets
     **/
    XcmVersionedMultiAssets: {
        _enum: {
            __Unused0: 'Null',
            V2: 'XcmV2MultiassetMultiAssets',
            __Unused2: 'Null',
            V3: 'XcmV3MultiassetMultiAssets',
        },
    },
    /**
     * Lookup111: xcm::v2::multiasset::MultiAssets
     **/
    XcmV2MultiassetMultiAssets: 'Vec<XcmV2MultiAsset>',
    /**
     * Lookup113: xcm::v2::multiasset::MultiAsset
     **/
    XcmV2MultiAsset: {
        id: 'XcmV2MultiassetAssetId',
        fun: 'XcmV2MultiassetFungibility',
    },
    /**
     * Lookup114: xcm::v2::multiasset::AssetId
     **/
    XcmV2MultiassetAssetId: {
        _enum: {
            Concrete: 'XcmV2MultiLocation',
            Abstract: 'Bytes',
        },
    },
    /**
     * Lookup115: xcm::v2::multilocation::MultiLocation
     **/
    XcmV2MultiLocation: {
        parents: 'u8',
        interior: 'XcmV2MultilocationJunctions',
    },
    /**
     * Lookup116: xcm::v2::multilocation::Junctions
     **/
    XcmV2MultilocationJunctions: {
        _enum: {
            Here: 'Null',
            X1: 'XcmV2Junction',
            X2: '(XcmV2Junction,XcmV2Junction)',
            X3: '(XcmV2Junction,XcmV2Junction,XcmV2Junction)',
            X4: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
            X5: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
            X6: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
            X7: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
            X8: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
        },
    },
    /**
     * Lookup117: xcm::v2::junction::Junction
     **/
    XcmV2Junction: {
        _enum: {
            Parachain: 'Compact<u32>',
            AccountId32: {
                network: 'XcmV2NetworkId',
                id: '[u8;32]',
            },
            AccountIndex64: {
                network: 'XcmV2NetworkId',
                index: 'Compact<u64>',
            },
            AccountKey20: {
                network: 'XcmV2NetworkId',
                key: '[u8;20]',
            },
            PalletInstance: 'u8',
            GeneralIndex: 'Compact<u128>',
            GeneralKey: 'Bytes',
            OnlyChild: 'Null',
            Plurality: {
                id: 'XcmV2BodyId',
                part: 'XcmV2BodyPart',
            },
        },
    },
    /**
     * Lookup118: xcm::v2::NetworkId
     **/
    XcmV2NetworkId: {
        _enum: {
            Any: 'Null',
            Named: 'Bytes',
            Polkadot: 'Null',
            Kusama: 'Null',
        },
    },
    /**
     * Lookup120: xcm::v2::BodyId
     **/
    XcmV2BodyId: {
        _enum: {
            Unit: 'Null',
            Named: 'Bytes',
            Index: 'Compact<u32>',
            Executive: 'Null',
            Technical: 'Null',
            Legislative: 'Null',
            Judicial: 'Null',
            Defense: 'Null',
            Administration: 'Null',
            Treasury: 'Null',
        },
    },
    /**
     * Lookup121: xcm::v2::BodyPart
     **/
    XcmV2BodyPart: {
        _enum: {
            Voice: 'Null',
            Members: {
                count: 'Compact<u32>',
            },
            Fraction: {
                nom: 'Compact<u32>',
                denom: 'Compact<u32>',
            },
            AtLeastProportion: {
                nom: 'Compact<u32>',
                denom: 'Compact<u32>',
            },
            MoreThanProportion: {
                nom: 'Compact<u32>',
                denom: 'Compact<u32>',
            },
        },
    },
    /**
     * Lookup122: xcm::v2::multiasset::Fungibility
     **/
    XcmV2MultiassetFungibility: {
        _enum: {
            Fungible: 'Compact<u128>',
            NonFungible: 'XcmV2MultiassetAssetInstance',
        },
    },
    /**
     * Lookup123: xcm::v2::multiasset::AssetInstance
     **/
    XcmV2MultiassetAssetInstance: {
        _enum: {
            Undefined: 'Null',
            Index: 'Compact<u128>',
            Array4: '[u8;4]',
            Array8: '[u8;8]',
            Array16: '[u8;16]',
            Array32: '[u8;32]',
            Blob: 'Bytes',
        },
    },
    /**
     * Lookup124: xcm::VersionedMultiLocation
     **/
    XcmVersionedMultiLocation: {
        _enum: {
            __Unused0: 'Null',
            V2: 'XcmV2MultiLocation',
            __Unused2: 'Null',
            V3: 'XcmV3MultiLocation',
        },
    },
    /**
     * Lookup125: cumulus_pallet_xcm::pallet::Event<T>
     **/
    CumulusPalletXcmEvent: {
        _enum: {
            InvalidFormat: '[u8;32]',
            UnsupportedVersion: '[u8;32]',
            ExecutedDownward: '([u8;32],XcmV3TraitsOutcome)',
        },
    },
    /**
     * Lookup126: cumulus_pallet_dmp_queue::pallet::Event<T>
     **/
    CumulusPalletDmpQueueEvent: {
        _enum: {
            InvalidFormat: {
                messageId: '[u8;32]',
            },
            UnsupportedVersion: {
                messageId: '[u8;32]',
            },
            ExecutedDownward: {
                messageId: '[u8;32]',
                outcome: 'XcmV3TraitsOutcome',
            },
            WeightExhausted: {
                messageId: '[u8;32]',
                remainingWeight: 'SpWeightsWeightV2Weight',
                requiredWeight: 'SpWeightsWeightV2Weight',
            },
            OverweightEnqueued: {
                messageId: '[u8;32]',
                overweightIndex: 'u64',
                requiredWeight: 'SpWeightsWeightV2Weight',
            },
            OverweightServiced: {
                overweightIndex: 'u64',
                weightUsed: 'SpWeightsWeightV2Weight',
            },
            MaxMessagesExhausted: {
                messageId: '[u8;32]',
            },
        },
    },
    /**
     * Lookup127: orml_xtokens::module::Event<T>
     **/
    OrmlXtokensModuleEvent: {
        _enum: {
            TransferredMultiAssets: {
                sender: 'AccountId32',
                assets: 'XcmV3MultiassetMultiAssets',
                fee: 'XcmV3MultiAsset',
                dest: 'XcmV3MultiLocation',
            },
        },
    },
    /**
     * Lookup128: orml_tokens::module::Event<T>
     **/
    OrmlTokensModuleEvent: {
        _enum: {
            Endowed: {
                currencyId: 'u128',
                who: 'AccountId32',
                amount: 'u128',
            },
            DustLost: {
                currencyId: 'u128',
                who: 'AccountId32',
                amount: 'u128',
            },
            Transfer: {
                currencyId: 'u128',
                from: 'AccountId32',
                to: 'AccountId32',
                amount: 'u128',
            },
            Reserved: {
                currencyId: 'u128',
                who: 'AccountId32',
                amount: 'u128',
            },
            Unreserved: {
                currencyId: 'u128',
                who: 'AccountId32',
                amount: 'u128',
            },
            ReserveRepatriated: {
                currencyId: 'u128',
                from: 'AccountId32',
                to: 'AccountId32',
                amount: 'u128',
                status: 'FrameSupportTokensMiscBalanceStatus',
            },
            BalanceSet: {
                currencyId: 'u128',
                who: 'AccountId32',
                free: 'u128',
                reserved: 'u128',
            },
            TotalIssuanceSet: {
                currencyId: 'u128',
                amount: 'u128',
            },
            Withdrawn: {
                currencyId: 'u128',
                who: 'AccountId32',
                amount: 'u128',
            },
            Slashed: {
                currencyId: 'u128',
                who: 'AccountId32',
                freeAmount: 'u128',
                reservedAmount: 'u128',
            },
            Deposited: {
                currencyId: 'u128',
                who: 'AccountId32',
                amount: 'u128',
            },
            LockSet: {
                lockId: '[u8;8]',
                currencyId: 'u128',
                who: 'AccountId32',
                amount: 'u128',
            },
            LockRemoved: {
                lockId: '[u8;8]',
                currencyId: 'u128',
                who: 'AccountId32',
            },
            Locked: {
                currencyId: 'u128',
                who: 'AccountId32',
                amount: 'u128',
            },
            Unlocked: {
                currencyId: 'u128',
                who: 'AccountId32',
                amount: 'u128',
            },
        },
    },
    /**
     * Lookup129: pallet_bridge::pallet::Event<T>
     **/
    PalletBridgeEvent: {
        _enum: {
            RelayerThresholdChanged: 'u32',
            ChainWhitelisted: 'u8',
            RelayerAdded: 'AccountId32',
            RelayerRemoved: 'AccountId32',
            FungibleTransfer: '(u8,u64,[u8;32],u128,Bytes)',
            NonFungibleTransfer: '(u8,u64,[u8;32],Bytes,Bytes,Bytes)',
            GenericTransfer: '(u8,u64,[u8;32],Bytes)',
            VoteFor: '(u8,u64,AccountId32)',
            VoteAgainst: '(u8,u64,AccountId32)',
            ProposalApproved: '(u8,u64)',
            ProposalRejected: '(u8,u64)',
            ProposalSucceeded: '(u8,u64)',
            ProposalFailed: '(u8,u64)',
            FeeUpdated: {
                destId: 'u8',
                fee: 'u128',
            },
        },
    },
    /**
     * Lookup130: pallet_bridge_transfer::pallet::Event<T>
     **/
    PalletBridgeTransferEvent: {
        _enum: {
            MaximumIssuanceChanged: {
                oldValue: 'u128',
            },
            NativeTokenMinted: {
                to: 'AccountId32',
                amount: 'u128',
            },
        },
    },
    /**
     * Lookup131: pallet_drop3::pallet::Event<T>
     **/
    PalletDrop3Event: {
        _enum: {
            AdminChanged: {
                oldAdmin: 'Option<AccountId32>',
            },
            BalanceSlashed: {
                who: 'AccountId32',
                amount: 'u128',
            },
            RewardPoolApproved: {
                id: 'u64',
            },
            RewardPoolRejected: {
                id: 'u64',
            },
            RewardPoolStarted: {
                id: 'u64',
            },
            RewardPoolStopped: {
                id: 'u64',
            },
            RewardPoolRemoved: {
                id: 'u64',
                name: 'Bytes',
                owner: 'AccountId32',
            },
            RewardPoolProposed: {
                id: 'u64',
                name: 'Bytes',
                owner: 'AccountId32',
            },
            RewardSent: {
                to: 'AccountId32',
                amount: 'u128',
            },
        },
    },
    /**
     * Lookup133: pallet_extrinsic_filter::pallet::Event<T>
     **/
    PalletExtrinsicFilterEvent: {
        _enum: {
            ModeSet: {
                newMode: 'PalletExtrinsicFilterOperationalMode',
            },
            ExtrinsicsBlocked: {
                palletNameBytes: 'Bytes',
                functionNameBytes: 'Option<Bytes>',
            },
            ExtrinsicsUnblocked: {
                palletNameBytes: 'Bytes',
                functionNameBytes: 'Option<Bytes>',
            },
        },
    },
    /**
     * Lookup134: pallet_extrinsic_filter::OperationalMode
     **/
    PalletExtrinsicFilterOperationalMode: {
        _enum: ['Normal', 'Safe', 'Test'],
    },
    /**
     * Lookup136: pallet_identity_management::pallet::Event<T>
     **/
    PalletIdentityManagementEvent: {
        _enum: {
            DelegateeAdded: {
                account: 'AccountId32',
            },
            DelegateeRemoved: {
                account: 'AccountId32',
            },
            CreateIdentityRequested: {
                shard: 'H256',
            },
            RemoveIdentityRequested: {
                shard: 'H256',
            },
            VerifyIdentityRequested: {
                shard: 'H256',
            },
            SetUserShieldingKeyRequested: {
                shard: 'H256',
            },
            UserShieldingKeySet: {
                account: 'AccountId32',
                idGraph: 'CorePrimitivesKeyAesOutput',
                reqExtHash: 'H256',
            },
            IdentityCreated: {
                account: 'AccountId32',
                identity: 'CorePrimitivesKeyAesOutput',
                code: 'CorePrimitivesKeyAesOutput',
                reqExtHash: 'H256',
            },
            IdentityRemoved: {
                account: 'AccountId32',
                identity: 'CorePrimitivesKeyAesOutput',
                reqExtHash: 'H256',
            },
            IdentityVerified: {
                account: 'AccountId32',
                identity: 'CorePrimitivesKeyAesOutput',
                idGraph: 'CorePrimitivesKeyAesOutput',
                reqExtHash: 'H256',
            },
            SetUserShieldingKeyFailed: {
                account: 'Option<AccountId32>',
                detail: 'CorePrimitivesErrorErrorDetail',
                reqExtHash: 'H256',
            },
            CreateIdentityFailed: {
                account: 'Option<AccountId32>',
                detail: 'CorePrimitivesErrorErrorDetail',
                reqExtHash: 'H256',
            },
            RemoveIdentityFailed: {
                account: 'Option<AccountId32>',
                detail: 'CorePrimitivesErrorErrorDetail',
                reqExtHash: 'H256',
            },
            VerifyIdentityFailed: {
                account: 'Option<AccountId32>',
                detail: 'CorePrimitivesErrorErrorDetail',
                reqExtHash: 'H256',
            },
            ImportScheduledEnclaveFailed: 'Null',
            UnclassifiedError: {
                account: 'Option<AccountId32>',
                detail: 'CorePrimitivesErrorErrorDetail',
                reqExtHash: 'H256',
            },
        },
    },
    /**
     * Lookup137: core_primitives::key::AesOutput
     **/
    CorePrimitivesKeyAesOutput: {
        ciphertext: 'Bytes',
        aad: 'Bytes',
        nonce: '[u8;12]',
    },
    /**
     * Lookup139: core_primitives::error::ErrorDetail
     **/
    CorePrimitivesErrorErrorDetail: {
        _enum: {
            ImportError: 'Null',
            StfError: 'Bytes',
            SendStfRequestFailed: 'Null',
            ChallengeCodeNotFound: 'Null',
            UserShieldingKeyNotFound: 'Null',
            ParseError: 'Null',
            DataProviderError: 'Bytes',
            InvalidIdentity: 'Null',
            WrongWeb2Handle: 'Null',
            UnexpectedMessage: 'Null',
            WrongSignatureType: 'Null',
            VerifySubstrateSignatureFailed: 'Null',
            VerifyEvmSignatureFailed: 'Null',
            RecoverEvmAddressFailed: 'Null',
        },
    },
    /**
     * Lookup141: pallet_asset_manager::pallet::Event<T>
     **/
    PalletAssetManagerEvent: {
        _enum: {
            ForeignAssetMetadataUpdated: {
                assetId: 'u128',
                metadata: 'PalletAssetManagerAssetMetadata',
            },
            ForeignAssetTrackerUpdated: {
                oldAssetTracker: 'u128',
                newAssetTracker: 'u128',
            },
            ForeignAssetTypeRegistered: {
                assetId: 'u128',
                assetType: 'RuntimeCommonXcmImplCurrencyId',
            },
            ForeignAssetTypeRemoved: {
                assetId: 'u128',
                removedAssetType: 'RuntimeCommonXcmImplCurrencyId',
                defaultAssetType: 'RuntimeCommonXcmImplCurrencyId',
            },
            UnitsPerSecondChanged: {
                assetId: 'u128',
                unitsPerSecond: 'u128',
            },
        },
    },
    /**
     * Lookup142: pallet_asset_manager::pallet::AssetMetadata<Balance>
     **/
    PalletAssetManagerAssetMetadata: {
        name: 'Bytes',
        symbol: 'Bytes',
        decimals: 'u8',
        minimalBalance: 'u128',
        isFrozen: 'bool',
    },
    /**
     * Lookup143: runtime_common::xcm_impl::CurrencyId<rococo_parachain_runtime::Runtime>
     **/
    RuntimeCommonXcmImplCurrencyId: {
        _enum: {
            SelfReserve: 'Null',
            ParachainReserve: 'XcmV3MultiLocation',
        },
    },
    /**
     * Lookup144: rococo_parachain_runtime::Runtime
     **/
    RococoParachainRuntimeRuntime: 'Null',
    /**
     * Lookup145: pallet_vc_management::pallet::Event<T>
     **/
    PalletVcManagementEvent: {
        _enum: {
            VCRequested: {
                account: 'AccountId32',
                shard: 'H256',
                assertion: 'CorePrimitivesAssertion',
            },
            VCDisabled: {
                account: 'AccountId32',
                index: 'H256',
            },
            VCRevoked: {
                account: 'AccountId32',
                index: 'H256',
            },
            VCIssued: {
                account: 'AccountId32',
                assertion: 'CorePrimitivesAssertion',
                index: 'H256',
                vc: 'CorePrimitivesKeyAesOutput',
                reqExtHash: 'H256',
            },
            AdminChanged: {
                oldAdmin: 'Option<AccountId32>',
                newAdmin: 'Option<AccountId32>',
            },
            SchemaIssued: {
                account: 'AccountId32',
                shard: 'H256',
                index: 'u64',
            },
            SchemaDisabled: {
                account: 'AccountId32',
                shard: 'H256',
                index: 'u64',
            },
            SchemaActivated: {
                account: 'AccountId32',
                shard: 'H256',
                index: 'u64',
            },
            SchemaRevoked: {
                account: 'AccountId32',
                shard: 'H256',
                index: 'u64',
            },
            RequestVCFailed: {
                account: 'Option<AccountId32>',
                assertion: 'CorePrimitivesAssertion',
                detail: 'CorePrimitivesErrorErrorDetail',
                reqExtHash: 'H256',
            },
            UnclassifiedError: {
                account: 'Option<AccountId32>',
                detail: 'CorePrimitivesErrorErrorDetail',
                reqExtHash: 'H256',
            },
            VCRegistryItemAdded: {
                account: 'AccountId32',
                assertion: 'CorePrimitivesAssertion',
                index: 'H256',
            },
            VCRegistryItemRemoved: {
                index: 'H256',
            },
            VCRegistryCleared: 'Null',
        },
    },
    /**
     * Lookup146: core_primitives::assertion::Assertion
     **/
    CorePrimitivesAssertion: {
        _enum: {
            A1: 'Null',
            A2: 'Bytes',
            A3: '(Bytes,Bytes,Bytes)',
            A4: 'Bytes',
            A5: 'Bytes',
            A6: 'Null',
            A7: 'Bytes',
            A8: 'Vec<CorePrimitivesAssertionIndexingNetwork>',
            A9: 'Null',
            A10: 'Bytes',
            A11: 'Bytes',
            A13: 'u32',
        },
    },
    /**
     * Lookup149: core_primitives::assertion::IndexingNetwork
     **/
    CorePrimitivesAssertionIndexingNetwork: {
        _enum: ['Litentry', 'Litmus', 'Polkadot', 'Kusama', 'Khala', 'Ethereum'],
    },
    /**
     * Lookup151: pallet_group::pallet::Event<T, I>
     **/
    PalletGroupEvent: {
        _enum: {
            GroupMemberAdded: 'AccountId32',
            GroupMemberRemoved: 'AccountId32',
        },
    },
    /**
     * Lookup153: pallet_teerex::pallet::Event<T>
     **/
    PalletTeerexEvent: {
        _enum: {
            AdminChanged: {
                oldAdmin: 'Option<AccountId32>',
            },
            AddedEnclave: '(AccountId32,Bytes)',
            RemovedEnclave: 'AccountId32',
            Forwarded: 'H256',
            ShieldFunds: 'Bytes',
            UnshieldedFunds: 'AccountId32',
            ProcessedParentchainBlock: '(AccountId32,H256,H256,u32)',
            SetHeartbeatTimeout: 'u64',
            UpdatedScheduledEnclave: '(u64,[u8;32])',
            RemovedScheduledEnclave: 'u64',
            PublishedHash: {
                _alias: {
                    hash_: 'hash',
                },
                mrEnclave: '[u8;32]',
                hash_: 'H256',
                data: 'Bytes',
            },
        },
    },
    /**
     * Lookup154: pallet_sidechain::pallet::Event<T>
     **/
    PalletSidechainEvent: {
        _enum: {
            ProposedSidechainBlock: '(AccountId32,H256)',
            FinalizedSidechainBlock: '(AccountId32,H256)',
        },
    },
    /**
     * Lookup155: pallet_teeracle::pallet::Event<T>
     **/
    PalletTeeracleEvent: {
        _enum: {
            ExchangeRateUpdated: '(Bytes,Bytes,Option<SubstrateFixedFixedU64>)',
            ExchangeRateDeleted: '(Bytes,Bytes)',
            OracleUpdated: '(Bytes,Bytes)',
            AddedToWhitelist: '(Bytes,[u8;32])',
            RemovedFromWhitelist: '(Bytes,[u8;32])',
        },
    },
    /**
     * Lookup157: substrate_fixed::FixedU64<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>>
     **/
    SubstrateFixedFixedU64: {
        bits: 'u64',
    },
    /**
     * Lookup162: typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>
     **/
    TypenumUIntUInt: {
        msb: 'TypenumUIntUTerm',
        lsb: 'TypenumBitB0',
    },
    /**
     * Lookup163: typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>
     **/
    TypenumUIntUTerm: {
        msb: 'TypenumUintUTerm',
        lsb: 'TypenumBitB1',
    },
    /**
     * Lookup164: typenum::uint::UTerm
     **/
    TypenumUintUTerm: 'Null',
    /**
     * Lookup165: typenum::bit::B1
     **/
    TypenumBitB1: 'Null',
    /**
     * Lookup166: typenum::bit::B0
     **/
    TypenumBitB0: 'Null',
    /**
     * Lookup167: pallet_identity_management_mock::pallet::Event<T>
     **/
    PalletIdentityManagementMockEvent: {
        _enum: {
            DelegateeAdded: {
                account: 'AccountId32',
            },
            DelegateeRemoved: {
                account: 'AccountId32',
            },
            CreateIdentityRequested: {
                shard: 'H256',
            },
            RemoveIdentityRequested: {
                shard: 'H256',
            },
            VerifyIdentityRequested: {
                shard: 'H256',
            },
            SetUserShieldingKeyRequested: {
                shard: 'H256',
            },
            UserShieldingKeySetPlain: {
                account: 'AccountId32',
            },
            UserShieldingKeySet: {
                account: 'AccountId32',
            },
            IdentityCreatedPlain: {
                account: 'AccountId32',
                identity: 'MockTeePrimitivesIdentity',
                code: '[u8;16]',
                idGraph: 'Vec<(MockTeePrimitivesIdentity,PalletIdentityManagementMockIdentityContext)>',
            },
            IdentityCreated: {
                account: 'AccountId32',
                identity: 'CorePrimitivesKeyAesOutput',
                code: 'CorePrimitivesKeyAesOutput',
                idGraph: 'CorePrimitivesKeyAesOutput',
            },
            IdentityRemovedPlain: {
                account: 'AccountId32',
                identity: 'MockTeePrimitivesIdentity',
                idGraph: 'Vec<(MockTeePrimitivesIdentity,PalletIdentityManagementMockIdentityContext)>',
            },
            IdentityRemoved: {
                account: 'AccountId32',
                identity: 'CorePrimitivesKeyAesOutput',
                idGraph: 'CorePrimitivesKeyAesOutput',
            },
            IdentityVerifiedPlain: {
                account: 'AccountId32',
                identity: 'MockTeePrimitivesIdentity',
                idGraph: 'Vec<(MockTeePrimitivesIdentity,PalletIdentityManagementMockIdentityContext)>',
            },
            IdentityVerified: {
                account: 'AccountId32',
                identity: 'CorePrimitivesKeyAesOutput',
                idGraph: 'CorePrimitivesKeyAesOutput',
            },
            SomeError: {
                func: 'Bytes',
                error: 'Bytes',
            },
        },
    },
    /**
     * Lookup168: mock_tee_primitives::identity::Identity
     **/
    MockTeePrimitivesIdentity: {
        _enum: {
            Substrate: {
                network: 'MockTeePrimitivesIdentitySubstrateNetwork',
                address: 'MockTeePrimitivesIdentityAddress32',
            },
            Evm: {
                network: 'MockTeePrimitivesIdentityEvmNetwork',
                address: 'MockTeePrimitivesIdentityAddress20',
            },
            Web2: {
                network: 'MockTeePrimitivesIdentityWeb2Network',
                address: 'Bytes',
            },
        },
    },
    /**
     * Lookup169: mock_tee_primitives::identity::SubstrateNetwork
     **/
    MockTeePrimitivesIdentitySubstrateNetwork: {
        _enum: ['Polkadot', 'Kusama', 'Litentry', 'Litmus', 'LitentryRococo'],
    },
    /**
     * Lookup170: mock_tee_primitives::identity::Address32
     **/
    MockTeePrimitivesIdentityAddress32: '[u8;32]',
    /**
     * Lookup171: mock_tee_primitives::identity::EvmNetwork
     **/
    MockTeePrimitivesIdentityEvmNetwork: {
        _enum: ['Ethereum', 'BSC'],
    },
    /**
     * Lookup172: mock_tee_primitives::identity::Address20
     **/
    MockTeePrimitivesIdentityAddress20: '[u8;20]',
    /**
     * Lookup173: mock_tee_primitives::identity::Web2Network
     **/
    MockTeePrimitivesIdentityWeb2Network: {
        _enum: ['Twitter', 'Discord', 'Github'],
    },
    /**
     * Lookup176: pallet_identity_management_mock::identity_context::IdentityContext<T>
     **/
    PalletIdentityManagementMockIdentityContext: {
        metadata: 'Option<Bytes>',
        creationRequestBlock: 'Option<u32>',
        verificationRequestBlock: 'Option<u32>',
        isVerified: 'bool',
    },
    /**
     * Lookup180: pallet_sudo::pallet::Event<T>
     **/
    PalletSudoEvent: {
        _enum: {
            Sudid: {
                sudoResult: 'Result<Null, SpRuntimeDispatchError>',
            },
            KeyChanged: {
                oldSudoer: 'Option<AccountId32>',
            },
            SudoAsDone: {
                sudoResult: 'Result<Null, SpRuntimeDispatchError>',
            },
        },
    },
    /**
     * Lookup181: frame_system::Phase
     **/
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: 'u32',
            Finalization: 'Null',
            Initialization: 'Null',
        },
    },
    /**
     * Lookup184: frame_system::LastRuntimeUpgradeInfo
     **/
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: 'Compact<u32>',
        specName: 'Text',
    },
    /**
     * Lookup186: frame_system::pallet::Call<T>
     **/
    FrameSystemCall: {
        _enum: {
            remark: {
                remark: 'Bytes',
            },
            set_heap_pages: {
                pages: 'u64',
            },
            set_code: {
                code: 'Bytes',
            },
            set_code_without_checks: {
                code: 'Bytes',
            },
            set_storage: {
                items: 'Vec<(Bytes,Bytes)>',
            },
            kill_storage: {
                _alias: {
                    keys_: 'keys',
                },
                keys_: 'Vec<Bytes>',
            },
            kill_prefix: {
                prefix: 'Bytes',
                subkeys: 'u32',
            },
            remark_with_event: {
                remark: 'Bytes',
            },
        },
    },
    /**
     * Lookup190: frame_system::limits::BlockWeights
     **/
    FrameSystemLimitsBlockWeights: {
        baseBlock: 'SpWeightsWeightV2Weight',
        maxBlock: 'SpWeightsWeightV2Weight',
        perClass: 'FrameSupportDispatchPerDispatchClassWeightsPerClass',
    },
    /**
     * Lookup191: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
     **/
    FrameSupportDispatchPerDispatchClassWeightsPerClass: {
        normal: 'FrameSystemLimitsWeightsPerClass',
        operational: 'FrameSystemLimitsWeightsPerClass',
        mandatory: 'FrameSystemLimitsWeightsPerClass',
    },
    /**
     * Lookup192: frame_system::limits::WeightsPerClass
     **/
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: 'SpWeightsWeightV2Weight',
        maxExtrinsic: 'Option<SpWeightsWeightV2Weight>',
        maxTotal: 'Option<SpWeightsWeightV2Weight>',
        reserved: 'Option<SpWeightsWeightV2Weight>',
    },
    /**
     * Lookup194: frame_system::limits::BlockLength
     **/
    FrameSystemLimitsBlockLength: {
        max: 'FrameSupportDispatchPerDispatchClassU32',
    },
    /**
     * Lookup195: frame_support::dispatch::PerDispatchClass<T>
     **/
    FrameSupportDispatchPerDispatchClassU32: {
        normal: 'u32',
        operational: 'u32',
        mandatory: 'u32',
    },
    /**
     * Lookup196: sp_weights::RuntimeDbWeight
     **/
    SpWeightsRuntimeDbWeight: {
        read: 'u64',
        write: 'u64',
    },
    /**
     * Lookup197: sp_version::RuntimeVersion
     **/
    SpVersionRuntimeVersion: {
        specName: 'Text',
        implName: 'Text',
        authoringVersion: 'u32',
        specVersion: 'u32',
        implVersion: 'u32',
        apis: 'Vec<([u8;8],u32)>',
        transactionVersion: 'u32',
        stateVersion: 'u8',
    },
    /**
     * Lookup201: frame_system::pallet::Error<T>
     **/
    FrameSystemError: {
        _enum: [
            'InvalidSpecName',
            'SpecVersionNeedsToIncrease',
            'FailedToExtractRuntimeVersion',
            'NonDefaultComposite',
            'NonZeroRefCount',
            'CallFiltered',
        ],
    },
    /**
     * Lookup202: pallet_timestamp::pallet::Call<T>
     **/
    PalletTimestampCall: {
        _enum: {
            set: {
                now: 'Compact<u64>',
            },
        },
    },
    /**
     * Lookup205: pallet_scheduler::Scheduled<Name, frame_support::traits::preimages::Bounded<rococo_parachain_runtime::RuntimeCall>, BlockNumber, rococo_parachain_runtime::OriginCaller, sp_core::crypto::AccountId32>
     **/
    PalletSchedulerScheduled: {
        maybeId: 'Option<[u8;32]>',
        priority: 'u8',
        call: 'FrameSupportPreimagesBounded',
        maybePeriodic: 'Option<(u32,u32)>',
        origin: 'RococoParachainRuntimeOriginCaller',
    },
    /**
     * Lookup206: frame_support::traits::preimages::Bounded<rococo_parachain_runtime::RuntimeCall>
     **/
    FrameSupportPreimagesBounded: {
        _enum: {
            Legacy: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
            },
            Inline: 'Bytes',
            Lookup: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
                len: 'u32',
            },
        },
    },
    /**
     * Lookup208: pallet_scheduler::pallet::Call<T>
     **/
    PalletSchedulerCall: {
        _enum: {
            schedule: {
                when: 'u32',
                maybePeriodic: 'Option<(u32,u32)>',
                priority: 'u8',
                call: 'Call',
            },
            cancel: {
                when: 'u32',
                index: 'u32',
            },
            schedule_named: {
                id: '[u8;32]',
                when: 'u32',
                maybePeriodic: 'Option<(u32,u32)>',
                priority: 'u8',
                call: 'Call',
            },
            cancel_named: {
                id: '[u8;32]',
            },
            schedule_after: {
                after: 'u32',
                maybePeriodic: 'Option<(u32,u32)>',
                priority: 'u8',
                call: 'Call',
            },
            schedule_named_after: {
                id: '[u8;32]',
                after: 'u32',
                maybePeriodic: 'Option<(u32,u32)>',
                priority: 'u8',
                call: 'Call',
            },
        },
    },
    /**
     * Lookup210: pallet_utility::pallet::Call<T>
     **/
    PalletUtilityCall: {
        _enum: {
            batch: {
                calls: 'Vec<Call>',
            },
            as_derivative: {
                index: 'u16',
                call: 'Call',
            },
            batch_all: {
                calls: 'Vec<Call>',
            },
            dispatch_as: {
                asOrigin: 'RococoParachainRuntimeOriginCaller',
                call: 'Call',
            },
            force_batch: {
                calls: 'Vec<Call>',
            },
            with_weight: {
                call: 'Call',
                weight: 'SpWeightsWeightV2Weight',
            },
        },
    },
    /**
     * Lookup212: rococo_parachain_runtime::OriginCaller
     **/
    RococoParachainRuntimeOriginCaller: {
        _enum: {
            system: 'FrameSupportDispatchRawOrigin',
            __Unused1: 'Null',
            __Unused2: 'Null',
            __Unused3: 'Null',
            __Unused4: 'Null',
            Void: 'SpCoreVoid',
            __Unused6: 'Null',
            __Unused7: 'Null',
            __Unused8: 'Null',
            __Unused9: 'Null',
            __Unused10: 'Null',
            __Unused11: 'Null',
            __Unused12: 'Null',
            __Unused13: 'Null',
            __Unused14: 'Null',
            __Unused15: 'Null',
            __Unused16: 'Null',
            __Unused17: 'Null',
            __Unused18: 'Null',
            __Unused19: 'Null',
            __Unused20: 'Null',
            __Unused21: 'Null',
            Council: 'PalletCollectiveRawOrigin',
            __Unused23: 'Null',
            TechnicalCommittee: 'PalletCollectiveRawOrigin',
            __Unused25: 'Null',
            __Unused26: 'Null',
            __Unused27: 'Null',
            __Unused28: 'Null',
            __Unused29: 'Null',
            __Unused30: 'Null',
            __Unused31: 'Null',
            __Unused32: 'Null',
            __Unused33: 'Null',
            __Unused34: 'Null',
            __Unused35: 'Null',
            __Unused36: 'Null',
            __Unused37: 'Null',
            __Unused38: 'Null',
            __Unused39: 'Null',
            __Unused40: 'Null',
            __Unused41: 'Null',
            __Unused42: 'Null',
            __Unused43: 'Null',
            __Unused44: 'Null',
            __Unused45: 'Null',
            __Unused46: 'Null',
            __Unused47: 'Null',
            __Unused48: 'Null',
            __Unused49: 'Null',
            __Unused50: 'Null',
            PolkadotXcm: 'PalletXcmOrigin',
            CumulusXcm: 'CumulusPalletXcmOrigin',
        },
    },
    /**
     * Lookup213: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
     **/
    FrameSupportDispatchRawOrigin: {
        _enum: {
            Root: 'Null',
            Signed: 'AccountId32',
            None: 'Null',
        },
    },
    /**
     * Lookup214: pallet_collective::RawOrigin<sp_core::crypto::AccountId32, I>
     **/
    PalletCollectiveRawOrigin: {
        _enum: {
            Members: '(u32,u32)',
            Member: 'AccountId32',
            _Phantom: 'Null',
        },
    },
    /**
     * Lookup216: pallet_xcm::pallet::Origin
     **/
    PalletXcmOrigin: {
        _enum: {
            Xcm: 'XcmV3MultiLocation',
            Response: 'XcmV3MultiLocation',
        },
    },
    /**
     * Lookup217: cumulus_pallet_xcm::pallet::Origin
     **/
    CumulusPalletXcmOrigin: {
        _enum: {
            Relay: 'Null',
            SiblingParachain: 'u32',
        },
    },
    /**
     * Lookup218: sp_core::Void
     **/
    SpCoreVoid: 'Null',
    /**
     * Lookup219: pallet_multisig::pallet::Call<T>
     **/
    PalletMultisigCall: {
        _enum: {
            as_multi_threshold_1: {
                otherSignatories: 'Vec<AccountId32>',
                call: 'Call',
            },
            as_multi: {
                threshold: 'u16',
                otherSignatories: 'Vec<AccountId32>',
                maybeTimepoint: 'Option<PalletMultisigTimepoint>',
                call: 'Call',
                maxWeight: 'SpWeightsWeightV2Weight',
            },
            approve_as_multi: {
                threshold: 'u16',
                otherSignatories: 'Vec<AccountId32>',
                maybeTimepoint: 'Option<PalletMultisigTimepoint>',
                callHash: '[u8;32]',
                maxWeight: 'SpWeightsWeightV2Weight',
            },
            cancel_as_multi: {
                threshold: 'u16',
                otherSignatories: 'Vec<AccountId32>',
                timepoint: 'PalletMultisigTimepoint',
                callHash: '[u8;32]',
            },
        },
    },
    /**
     * Lookup222: pallet_proxy::pallet::Call<T>
     **/
    PalletProxyCall: {
        _enum: {
            proxy: {
                real: 'MultiAddress',
                forceProxyType: 'Option<RococoParachainRuntimeProxyType>',
                call: 'Call',
            },
            add_proxy: {
                delegate: 'MultiAddress',
                proxyType: 'RococoParachainRuntimeProxyType',
                delay: 'u32',
            },
            remove_proxy: {
                delegate: 'MultiAddress',
                proxyType: 'RococoParachainRuntimeProxyType',
                delay: 'u32',
            },
            remove_proxies: 'Null',
            create_pure: {
                proxyType: 'RococoParachainRuntimeProxyType',
                delay: 'u32',
                index: 'u16',
            },
            kill_pure: {
                spawner: 'MultiAddress',
                proxyType: 'RococoParachainRuntimeProxyType',
                index: 'u16',
                height: 'Compact<u32>',
                extIndex: 'Compact<u32>',
            },
            announce: {
                real: 'MultiAddress',
                callHash: 'H256',
            },
            remove_announcement: {
                real: 'MultiAddress',
                callHash: 'H256',
            },
            reject_announcement: {
                delegate: 'MultiAddress',
                callHash: 'H256',
            },
            proxy_announced: {
                delegate: 'MultiAddress',
                real: 'MultiAddress',
                forceProxyType: 'Option<RococoParachainRuntimeProxyType>',
                call: 'Call',
            },
        },
    },
    /**
     * Lookup226: pallet_preimage::pallet::Call<T>
     **/
    PalletPreimageCall: {
        _enum: {
            note_preimage: {
                bytes: 'Bytes',
            },
            unnote_preimage: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
            },
            request_preimage: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
            },
            unrequest_preimage: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
            },
        },
    },
    /**
     * Lookup227: pallet_balances::pallet::Call<T, I>
     **/
    PalletBalancesCall: {
        _enum: {
            transfer: {
                dest: 'MultiAddress',
                value: 'Compact<u128>',
            },
            set_balance: {
                who: 'MultiAddress',
                newFree: 'Compact<u128>',
                newReserved: 'Compact<u128>',
            },
            force_transfer: {
                source: 'MultiAddress',
                dest: 'MultiAddress',
                value: 'Compact<u128>',
            },
            transfer_keep_alive: {
                dest: 'MultiAddress',
                value: 'Compact<u128>',
            },
            transfer_all: {
                dest: 'MultiAddress',
                keepAlive: 'bool',
            },
            force_unreserve: {
                who: 'MultiAddress',
                amount: 'u128',
            },
        },
    },
    /**
     * Lookup228: pallet_vesting::pallet::Call<T>
     **/
    PalletVestingCall: {
        _enum: {
            vest: 'Null',
            vest_other: {
                target: 'MultiAddress',
            },
            vested_transfer: {
                target: 'MultiAddress',
                schedule: 'PalletVestingVestingInfo',
            },
            force_vested_transfer: {
                source: 'MultiAddress',
                target: 'MultiAddress',
                schedule: 'PalletVestingVestingInfo',
            },
            merge_schedules: {
                schedule1Index: 'u32',
                schedule2Index: 'u32',
            },
        },
    },
    /**
     * Lookup229: pallet_vesting::vesting_info::VestingInfo<Balance, BlockNumber>
     **/
    PalletVestingVestingInfo: {
        locked: 'u128',
        perBlock: 'u128',
        startingBlock: 'u32',
    },
    /**
     * Lookup230: pallet_treasury::pallet::Call<T, I>
     **/
    PalletTreasuryCall: {
        _enum: {
            propose_spend: {
                value: 'Compact<u128>',
                beneficiary: 'MultiAddress',
            },
            reject_proposal: {
                proposalId: 'Compact<u32>',
            },
            approve_proposal: {
                proposalId: 'Compact<u32>',
            },
            spend: {
                amount: 'Compact<u128>',
                beneficiary: 'MultiAddress',
            },
            remove_approval: {
                proposalId: 'Compact<u32>',
            },
        },
    },
    /**
     * Lookup231: pallet_democracy::pallet::Call<T>
     **/
    PalletDemocracyCall: {
        _enum: {
            propose: {
                proposal: 'FrameSupportPreimagesBounded',
                value: 'Compact<u128>',
            },
            second: {
                proposal: 'Compact<u32>',
            },
            vote: {
                refIndex: 'Compact<u32>',
                vote: 'PalletDemocracyVoteAccountVote',
            },
            emergency_cancel: {
                refIndex: 'u32',
            },
            external_propose: {
                proposal: 'FrameSupportPreimagesBounded',
            },
            external_propose_majority: {
                proposal: 'FrameSupportPreimagesBounded',
            },
            external_propose_default: {
                proposal: 'FrameSupportPreimagesBounded',
            },
            fast_track: {
                proposalHash: 'H256',
                votingPeriod: 'u32',
                delay: 'u32',
            },
            veto_external: {
                proposalHash: 'H256',
            },
            cancel_referendum: {
                refIndex: 'Compact<u32>',
            },
            delegate: {
                to: 'MultiAddress',
                conviction: 'PalletDemocracyConviction',
                balance: 'u128',
            },
            undelegate: 'Null',
            clear_public_proposals: 'Null',
            unlock: {
                target: 'MultiAddress',
            },
            remove_vote: {
                index: 'u32',
            },
            remove_other_vote: {
                target: 'MultiAddress',
                index: 'u32',
            },
            blacklist: {
                proposalHash: 'H256',
                maybeRefIndex: 'Option<u32>',
            },
            cancel_proposal: {
                propIndex: 'Compact<u32>',
            },
            set_metadata: {
                owner: 'PalletDemocracyMetadataOwner',
                maybeHash: 'Option<H256>',
            },
        },
    },
    /**
     * Lookup232: pallet_democracy::conviction::Conviction
     **/
    PalletDemocracyConviction: {
        _enum: ['None', 'Locked1x', 'Locked2x', 'Locked3x', 'Locked4x', 'Locked5x', 'Locked6x'],
    },
    /**
     * Lookup234: pallet_collective::pallet::Call<T, I>
     **/
    PalletCollectiveCall: {
        _enum: {
            set_members: {
                newMembers: 'Vec<AccountId32>',
                prime: 'Option<AccountId32>',
                oldCount: 'u32',
            },
            execute: {
                proposal: 'Call',
                lengthBound: 'Compact<u32>',
            },
            propose: {
                threshold: 'Compact<u32>',
                proposal: 'Call',
                lengthBound: 'Compact<u32>',
            },
            vote: {
                proposal: 'H256',
                index: 'Compact<u32>',
                approve: 'bool',
            },
            close_old_weight: {
                proposalHash: 'H256',
                index: 'Compact<u32>',
                proposalWeightBound: 'Compact<u64>',
                lengthBound: 'Compact<u32>',
            },
            disapprove_proposal: {
                proposalHash: 'H256',
            },
            close: {
                proposalHash: 'H256',
                index: 'Compact<u32>',
                proposalWeightBound: 'SpWeightsWeightV2Weight',
                lengthBound: 'Compact<u32>',
            },
        },
    },
    /**
     * Lookup237: pallet_membership::pallet::Call<T, I>
     **/
    PalletMembershipCall: {
        _enum: {
            add_member: {
                who: 'MultiAddress',
            },
            remove_member: {
                who: 'MultiAddress',
            },
            swap_member: {
                remove: 'MultiAddress',
                add: 'MultiAddress',
            },
            reset_members: {
                members: 'Vec<AccountId32>',
            },
            change_key: {
                _alias: {
                    new_: 'new',
                },
                new_: 'MultiAddress',
            },
            set_prime: {
                who: 'MultiAddress',
            },
            clear_prime: 'Null',
        },
    },
    /**
     * Lookup240: pallet_bounties::pallet::Call<T, I>
     **/
    PalletBountiesCall: {
        _enum: {
            propose_bounty: {
                value: 'Compact<u128>',
                description: 'Bytes',
            },
            approve_bounty: {
                bountyId: 'Compact<u32>',
            },
            propose_curator: {
                bountyId: 'Compact<u32>',
                curator: 'MultiAddress',
                fee: 'Compact<u128>',
            },
            unassign_curator: {
                bountyId: 'Compact<u32>',
            },
            accept_curator: {
                bountyId: 'Compact<u32>',
            },
            award_bounty: {
                bountyId: 'Compact<u32>',
                beneficiary: 'MultiAddress',
            },
            claim_bounty: {
                bountyId: 'Compact<u32>',
            },
            close_bounty: {
                bountyId: 'Compact<u32>',
            },
            extend_bounty_expiry: {
                bountyId: 'Compact<u32>',
                remark: 'Bytes',
            },
        },
    },
    /**
     * Lookup241: pallet_tips::pallet::Call<T, I>
     **/
    PalletTipsCall: {
        _enum: {
            report_awesome: {
                reason: 'Bytes',
                who: 'MultiAddress',
            },
            retract_tip: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
            },
            tip_new: {
                reason: 'Bytes',
                who: 'MultiAddress',
                tipValue: 'Compact<u128>',
            },
            tip: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
                tipValue: 'Compact<u128>',
            },
            close_tip: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
            },
            slash_tip: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
            },
        },
    },
    /**
     * Lookup242: pallet_identity::pallet::Call<T>
     **/
    PalletIdentityCall: {
        _enum: {
            add_registrar: {
                account: 'MultiAddress',
            },
            set_identity: {
                info: 'PalletIdentityIdentityInfo',
            },
            set_subs: {
                subs: 'Vec<(AccountId32,Data)>',
            },
            clear_identity: 'Null',
            request_judgement: {
                regIndex: 'Compact<u32>',
                maxFee: 'Compact<u128>',
            },
            cancel_request: {
                regIndex: 'u32',
            },
            set_fee: {
                index: 'Compact<u32>',
                fee: 'Compact<u128>',
            },
            set_account_id: {
                _alias: {
                    new_: 'new',
                },
                index: 'Compact<u32>',
                new_: 'MultiAddress',
            },
            set_fields: {
                index: 'Compact<u32>',
                fields: 'PalletIdentityBitFlags',
            },
            provide_judgement: {
                regIndex: 'Compact<u32>',
                target: 'MultiAddress',
                judgement: 'PalletIdentityJudgement',
                identity: 'H256',
            },
            kill_identity: {
                target: 'MultiAddress',
            },
            add_sub: {
                sub: 'MultiAddress',
                data: 'Data',
            },
            rename_sub: {
                sub: 'MultiAddress',
                data: 'Data',
            },
            remove_sub: {
                sub: 'MultiAddress',
            },
            quit_sub: 'Null',
        },
    },
    /**
     * Lookup243: pallet_identity::types::IdentityInfo<FieldLimit>
     **/
    PalletIdentityIdentityInfo: {
        additional: 'Vec<(Data,Data)>',
        display: 'Data',
        legal: 'Data',
        web: 'Data',
        riot: 'Data',
        email: 'Data',
        pgpFingerprint: 'Option<[u8;20]>',
        image: 'Data',
        twitter: 'Data',
    },
    /**
     * Lookup278: pallet_identity::types::BitFlags<pallet_identity::types::IdentityField>
     **/
    PalletIdentityBitFlags: {
        _bitLength: 64,
        Display: 1,
        Legal: 2,
        Web: 4,
        Riot: 8,
        Email: 16,
        PgpFingerprint: 32,
        Image: 64,
        Twitter: 128,
    },
    /**
     * Lookup279: pallet_identity::types::IdentityField
     **/
    PalletIdentityIdentityField: {
        _enum: [
            '__Unused0',
            'Display',
            'Legal',
            '__Unused3',
            'Web',
            '__Unused5',
            '__Unused6',
            '__Unused7',
            'Riot',
            '__Unused9',
            '__Unused10',
            '__Unused11',
            '__Unused12',
            '__Unused13',
            '__Unused14',
            '__Unused15',
            'Email',
            '__Unused17',
            '__Unused18',
            '__Unused19',
            '__Unused20',
            '__Unused21',
            '__Unused22',
            '__Unused23',
            '__Unused24',
            '__Unused25',
            '__Unused26',
            '__Unused27',
            '__Unused28',
            '__Unused29',
            '__Unused30',
            '__Unused31',
            'PgpFingerprint',
            '__Unused33',
            '__Unused34',
            '__Unused35',
            '__Unused36',
            '__Unused37',
            '__Unused38',
            '__Unused39',
            '__Unused40',
            '__Unused41',
            '__Unused42',
            '__Unused43',
            '__Unused44',
            '__Unused45',
            '__Unused46',
            '__Unused47',
            '__Unused48',
            '__Unused49',
            '__Unused50',
            '__Unused51',
            '__Unused52',
            '__Unused53',
            '__Unused54',
            '__Unused55',
            '__Unused56',
            '__Unused57',
            '__Unused58',
            '__Unused59',
            '__Unused60',
            '__Unused61',
            '__Unused62',
            '__Unused63',
            'Image',
            '__Unused65',
            '__Unused66',
            '__Unused67',
            '__Unused68',
            '__Unused69',
            '__Unused70',
            '__Unused71',
            '__Unused72',
            '__Unused73',
            '__Unused74',
            '__Unused75',
            '__Unused76',
            '__Unused77',
            '__Unused78',
            '__Unused79',
            '__Unused80',
            '__Unused81',
            '__Unused82',
            '__Unused83',
            '__Unused84',
            '__Unused85',
            '__Unused86',
            '__Unused87',
            '__Unused88',
            '__Unused89',
            '__Unused90',
            '__Unused91',
            '__Unused92',
            '__Unused93',
            '__Unused94',
            '__Unused95',
            '__Unused96',
            '__Unused97',
            '__Unused98',
            '__Unused99',
            '__Unused100',
            '__Unused101',
            '__Unused102',
            '__Unused103',
            '__Unused104',
            '__Unused105',
            '__Unused106',
            '__Unused107',
            '__Unused108',
            '__Unused109',
            '__Unused110',
            '__Unused111',
            '__Unused112',
            '__Unused113',
            '__Unused114',
            '__Unused115',
            '__Unused116',
            '__Unused117',
            '__Unused118',
            '__Unused119',
            '__Unused120',
            '__Unused121',
            '__Unused122',
            '__Unused123',
            '__Unused124',
            '__Unused125',
            '__Unused126',
            '__Unused127',
            'Twitter',
        ],
    },
    /**
     * Lookup280: pallet_identity::types::Judgement<Balance>
     **/
    PalletIdentityJudgement: {
        _enum: {
            Unknown: 'Null',
            FeePaid: 'u128',
            Reasonable: 'Null',
            KnownGood: 'Null',
            OutOfDate: 'Null',
            LowQuality: 'Null',
            Erroneous: 'Null',
        },
    },
    /**
     * Lookup281: cumulus_pallet_parachain_system::pallet::Call<T>
     **/
    CumulusPalletParachainSystemCall: {
        _enum: {
            set_validation_data: {
                data: 'CumulusPrimitivesParachainInherentParachainInherentData',
            },
            sudo_send_upward_message: {
                message: 'Bytes',
            },
            authorize_upgrade: {
                codeHash: 'H256',
            },
            enact_authorized_upgrade: {
                code: 'Bytes',
            },
        },
    },
    /**
     * Lookup282: cumulus_primitives_parachain_inherent::ParachainInherentData
     **/
    CumulusPrimitivesParachainInherentParachainInherentData: {
        validationData: 'PolkadotPrimitivesV2PersistedValidationData',
        relayChainState: 'SpTrieStorageProof',
        downwardMessages: 'Vec<PolkadotCorePrimitivesInboundDownwardMessage>',
        horizontalMessages: 'BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>',
    },
    /**
     * Lookup283: polkadot_primitives::v2::PersistedValidationData<primitive_types::H256, N>
     **/
    PolkadotPrimitivesV2PersistedValidationData: {
        parentHead: 'Bytes',
        relayParentNumber: 'u32',
        relayParentStorageRoot: 'H256',
        maxPovSize: 'u32',
    },
    /**
     * Lookup285: sp_trie::storage_proof::StorageProof
     **/
    SpTrieStorageProof: {
        trieNodes: 'BTreeSet<Bytes>',
    },
    /**
     * Lookup288: polkadot_core_primitives::InboundDownwardMessage<BlockNumber>
     **/
    PolkadotCorePrimitivesInboundDownwardMessage: {
        sentAt: 'u32',
        msg: 'Bytes',
    },
    /**
     * Lookup291: polkadot_core_primitives::InboundHrmpMessage<BlockNumber>
     **/
    PolkadotCorePrimitivesInboundHrmpMessage: {
        sentAt: 'u32',
        data: 'Bytes',
    },
    /**
     * Lookup294: parachain_info::pallet::Call<T>
     **/
    ParachainInfoCall: 'Null',
    /**
     * Lookup295: pallet_session::pallet::Call<T>
     **/
    PalletSessionCall: {
        _enum: {
            set_keys: {
                _alias: {
                    keys_: 'keys',
                },
                keys_: 'RococoParachainRuntimeSessionKeys',
                proof: 'Bytes',
            },
            purge_keys: 'Null',
        },
    },
    /**
     * Lookup296: rococo_parachain_runtime::SessionKeys
     **/
    RococoParachainRuntimeSessionKeys: {
        aura: 'SpConsensusAuraSr25519AppSr25519Public',
    },
    /**
     * Lookup297: sp_consensus_aura::sr25519::app_sr25519::Public
     **/
    SpConsensusAuraSr25519AppSr25519Public: 'SpCoreSr25519Public',
    /**
     * Lookup298: sp_core::sr25519::Public
     **/
    SpCoreSr25519Public: '[u8;32]',
    /**
     * Lookup299: pallet_parachain_staking::pallet::Call<T>
     **/
    PalletParachainStakingCall: {
        _enum: {
            set_staking_expectations: {
                expectations: {
                    min: 'u128',
                    ideal: 'u128',
                    max: 'u128',
                },
            },
            set_inflation: {
                schedule: {
                    min: 'Perbill',
                    ideal: 'Perbill',
                    max: 'Perbill',
                },
            },
            set_parachain_bond_account: {
                _alias: {
                    new_: 'new',
                },
                new_: 'AccountId32',
            },
            set_parachain_bond_reserve_percent: {
                _alias: {
                    new_: 'new',
                },
                new_: 'Percent',
            },
            set_total_selected: {
                _alias: {
                    new_: 'new',
                },
                new_: 'u32',
            },
            set_collator_commission: {
                _alias: {
                    new_: 'new',
                },
                new_: 'Perbill',
            },
            set_blocks_per_round: {
                _alias: {
                    new_: 'new',
                },
                new_: 'u32',
            },
            add_candidates_whitelist: {
                candidate: 'AccountId32',
            },
            remove_candidates_whitelist: {
                candidate: 'AccountId32',
            },
            join_candidates: {
                bond: 'u128',
            },
            schedule_leave_candidates: 'Null',
            execute_leave_candidates: {
                candidate: 'AccountId32',
            },
            cancel_leave_candidates: 'Null',
            go_offline: 'Null',
            go_online: 'Null',
            candidate_bond_more: {
                more: 'u128',
            },
            schedule_candidate_bond_less: {
                less: 'u128',
            },
            execute_candidate_bond_less: {
                candidate: 'AccountId32',
            },
            cancel_candidate_bond_less: 'Null',
            delegate: {
                candidate: 'AccountId32',
                amount: 'u128',
            },
            delegate_with_auto_compound: {
                candidate: 'AccountId32',
                amount: 'u128',
                autoCompound: 'Percent',
            },
            schedule_leave_delegators: 'Null',
            execute_leave_delegators: {
                delegator: 'AccountId32',
            },
            cancel_leave_delegators: 'Null',
            schedule_revoke_delegation: {
                collator: 'AccountId32',
            },
            delegator_bond_more: {
                candidate: 'AccountId32',
                more: 'u128',
            },
            schedule_delegator_bond_less: {
                candidate: 'AccountId32',
                less: 'u128',
            },
            execute_delegation_request: {
                delegator: 'AccountId32',
                candidate: 'AccountId32',
            },
            cancel_delegation_request: {
                candidate: 'AccountId32',
            },
            set_auto_compound: {
                candidate: 'AccountId32',
                value: 'Percent',
            },
        },
    },
    /**
     * Lookup302: cumulus_pallet_xcmp_queue::pallet::Call<T>
     **/
    CumulusPalletXcmpQueueCall: {
        _enum: {
            service_overweight: {
                index: 'u64',
                weightLimit: 'SpWeightsWeightV2Weight',
            },
            suspend_xcm_execution: 'Null',
            resume_xcm_execution: 'Null',
            update_suspend_threshold: {
                _alias: {
                    new_: 'new',
                },
                new_: 'u32',
            },
            update_drop_threshold: {
                _alias: {
                    new_: 'new',
                },
                new_: 'u32',
            },
            update_resume_threshold: {
                _alias: {
                    new_: 'new',
                },
                new_: 'u32',
            },
            update_threshold_weight: {
                _alias: {
                    new_: 'new',
                },
                new_: 'SpWeightsWeightV2Weight',
            },
            update_weight_restrict_decay: {
                _alias: {
                    new_: 'new',
                },
                new_: 'SpWeightsWeightV2Weight',
            },
            update_xcmp_max_individual_weight: {
                _alias: {
                    new_: 'new',
                },
                new_: 'SpWeightsWeightV2Weight',
            },
        },
    },
    /**
     * Lookup303: pallet_xcm::pallet::Call<T>
     **/
    PalletXcmCall: {
        _enum: {
            send: {
                dest: 'XcmVersionedMultiLocation',
                message: 'XcmVersionedXcm',
            },
            teleport_assets: {
                dest: 'XcmVersionedMultiLocation',
                beneficiary: 'XcmVersionedMultiLocation',
                assets: 'XcmVersionedMultiAssets',
                feeAssetItem: 'u32',
            },
            reserve_transfer_assets: {
                dest: 'XcmVersionedMultiLocation',
                beneficiary: 'XcmVersionedMultiLocation',
                assets: 'XcmVersionedMultiAssets',
                feeAssetItem: 'u32',
            },
            execute: {
                message: 'XcmVersionedXcm',
                maxWeight: 'SpWeightsWeightV2Weight',
            },
            force_xcm_version: {
                location: 'XcmV3MultiLocation',
                xcmVersion: 'u32',
            },
            force_default_xcm_version: {
                maybeXcmVersion: 'Option<u32>',
            },
            force_subscribe_version_notify: {
                location: 'XcmVersionedMultiLocation',
            },
            force_unsubscribe_version_notify: {
                location: 'XcmVersionedMultiLocation',
            },
            limited_reserve_transfer_assets: {
                dest: 'XcmVersionedMultiLocation',
                beneficiary: 'XcmVersionedMultiLocation',
                assets: 'XcmVersionedMultiAssets',
                feeAssetItem: 'u32',
                weightLimit: 'XcmV3WeightLimit',
            },
            limited_teleport_assets: {
                dest: 'XcmVersionedMultiLocation',
                beneficiary: 'XcmVersionedMultiLocation',
                assets: 'XcmVersionedMultiAssets',
                feeAssetItem: 'u32',
                weightLimit: 'XcmV3WeightLimit',
            },
        },
    },
    /**
     * Lookup304: xcm::VersionedXcm<RuntimeCall>
     **/
    XcmVersionedXcm: {
        _enum: {
            __Unused0: 'Null',
            __Unused1: 'Null',
            V2: 'XcmV2Xcm',
            V3: 'XcmV3Xcm',
        },
    },
    /**
     * Lookup305: xcm::v2::Xcm<RuntimeCall>
     **/
    XcmV2Xcm: 'Vec<XcmV2Instruction>',
    /**
     * Lookup307: xcm::v2::Instruction<RuntimeCall>
     **/
    XcmV2Instruction: {
        _enum: {
            WithdrawAsset: 'XcmV2MultiassetMultiAssets',
            ReserveAssetDeposited: 'XcmV2MultiassetMultiAssets',
            ReceiveTeleportedAsset: 'XcmV2MultiassetMultiAssets',
            QueryResponse: {
                queryId: 'Compact<u64>',
                response: 'XcmV2Response',
                maxWeight: 'Compact<u64>',
            },
            TransferAsset: {
                assets: 'XcmV2MultiassetMultiAssets',
                beneficiary: 'XcmV2MultiLocation',
            },
            TransferReserveAsset: {
                assets: 'XcmV2MultiassetMultiAssets',
                dest: 'XcmV2MultiLocation',
                xcm: 'XcmV2Xcm',
            },
            Transact: {
                originType: 'XcmV2OriginKind',
                requireWeightAtMost: 'Compact<u64>',
                call: 'XcmDoubleEncoded',
            },
            HrmpNewChannelOpenRequest: {
                sender: 'Compact<u32>',
                maxMessageSize: 'Compact<u32>',
                maxCapacity: 'Compact<u32>',
            },
            HrmpChannelAccepted: {
                recipient: 'Compact<u32>',
            },
            HrmpChannelClosing: {
                initiator: 'Compact<u32>',
                sender: 'Compact<u32>',
                recipient: 'Compact<u32>',
            },
            ClearOrigin: 'Null',
            DescendOrigin: 'XcmV2MultilocationJunctions',
            ReportError: {
                queryId: 'Compact<u64>',
                dest: 'XcmV2MultiLocation',
                maxResponseWeight: 'Compact<u64>',
            },
            DepositAsset: {
                assets: 'XcmV2MultiassetMultiAssetFilter',
                maxAssets: 'Compact<u32>',
                beneficiary: 'XcmV2MultiLocation',
            },
            DepositReserveAsset: {
                assets: 'XcmV2MultiassetMultiAssetFilter',
                maxAssets: 'Compact<u32>',
                dest: 'XcmV2MultiLocation',
                xcm: 'XcmV2Xcm',
            },
            ExchangeAsset: {
                give: 'XcmV2MultiassetMultiAssetFilter',
                receive: 'XcmV2MultiassetMultiAssets',
            },
            InitiateReserveWithdraw: {
                assets: 'XcmV2MultiassetMultiAssetFilter',
                reserve: 'XcmV2MultiLocation',
                xcm: 'XcmV2Xcm',
            },
            InitiateTeleport: {
                assets: 'XcmV2MultiassetMultiAssetFilter',
                dest: 'XcmV2MultiLocation',
                xcm: 'XcmV2Xcm',
            },
            QueryHolding: {
                queryId: 'Compact<u64>',
                dest: 'XcmV2MultiLocation',
                assets: 'XcmV2MultiassetMultiAssetFilter',
                maxResponseWeight: 'Compact<u64>',
            },
            BuyExecution: {
                fees: 'XcmV2MultiAsset',
                weightLimit: 'XcmV2WeightLimit',
            },
            RefundSurplus: 'Null',
            SetErrorHandler: 'XcmV2Xcm',
            SetAppendix: 'XcmV2Xcm',
            ClearError: 'Null',
            ClaimAsset: {
                assets: 'XcmV2MultiassetMultiAssets',
                ticket: 'XcmV2MultiLocation',
            },
            Trap: 'Compact<u64>',
            SubscribeVersion: {
                queryId: 'Compact<u64>',
                maxResponseWeight: 'Compact<u64>',
            },
            UnsubscribeVersion: 'Null',
        },
    },
    /**
     * Lookup308: xcm::v2::Response
     **/
    XcmV2Response: {
        _enum: {
            Null: 'Null',
            Assets: 'XcmV2MultiassetMultiAssets',
            ExecutionResult: 'Option<(u32,XcmV2TraitsError)>',
            Version: 'u32',
        },
    },
    /**
     * Lookup311: xcm::v2::traits::Error
     **/
    XcmV2TraitsError: {
        _enum: {
            Overflow: 'Null',
            Unimplemented: 'Null',
            UntrustedReserveLocation: 'Null',
            UntrustedTeleportLocation: 'Null',
            MultiLocationFull: 'Null',
            MultiLocationNotInvertible: 'Null',
            BadOrigin: 'Null',
            InvalidLocation: 'Null',
            AssetNotFound: 'Null',
            FailedToTransactAsset: 'Null',
            NotWithdrawable: 'Null',
            LocationCannotHold: 'Null',
            ExceedsMaxMessageSize: 'Null',
            DestinationUnsupported: 'Null',
            Transport: 'Null',
            Unroutable: 'Null',
            UnknownClaim: 'Null',
            FailedToDecode: 'Null',
            MaxWeightInvalid: 'Null',
            NotHoldingFees: 'Null',
            TooExpensive: 'Null',
            Trap: 'u64',
            UnhandledXcmVersion: 'Null',
            WeightLimitReached: 'u64',
            Barrier: 'Null',
            WeightNotComputable: 'Null',
        },
    },
    /**
     * Lookup312: xcm::v2::multiasset::MultiAssetFilter
     **/
    XcmV2MultiassetMultiAssetFilter: {
        _enum: {
            Definite: 'XcmV2MultiassetMultiAssets',
            Wild: 'XcmV2MultiassetWildMultiAsset',
        },
    },
    /**
     * Lookup313: xcm::v2::multiasset::WildMultiAsset
     **/
    XcmV2MultiassetWildMultiAsset: {
        _enum: {
            All: 'Null',
            AllOf: {
                id: 'XcmV2MultiassetAssetId',
                fun: 'XcmV2MultiassetWildFungibility',
            },
        },
    },
    /**
     * Lookup314: xcm::v2::multiasset::WildFungibility
     **/
    XcmV2MultiassetWildFungibility: {
        _enum: ['Fungible', 'NonFungible'],
    },
    /**
     * Lookup315: xcm::v2::WeightLimit
     **/
    XcmV2WeightLimit: {
        _enum: {
            Unlimited: 'Null',
            Limited: 'Compact<u64>',
        },
    },
    /**
     * Lookup324: cumulus_pallet_xcm::pallet::Call<T>
     **/
    CumulusPalletXcmCall: 'Null',
    /**
     * Lookup325: cumulus_pallet_dmp_queue::pallet::Call<T>
     **/
    CumulusPalletDmpQueueCall: {
        _enum: {
            service_overweight: {
                index: 'u64',
                weightLimit: 'SpWeightsWeightV2Weight',
            },
        },
    },
    /**
     * Lookup326: orml_xtokens::module::Call<T>
     **/
    OrmlXtokensModuleCall: {
        _enum: {
            transfer: {
                currencyId: 'RuntimeCommonXcmImplCurrencyId',
                amount: 'u128',
                dest: 'XcmVersionedMultiLocation',
                destWeightLimit: 'XcmV3WeightLimit',
            },
            transfer_multiasset: {
                asset: 'XcmVersionedMultiAsset',
                dest: 'XcmVersionedMultiLocation',
                destWeightLimit: 'XcmV3WeightLimit',
            },
            transfer_with_fee: {
                currencyId: 'RuntimeCommonXcmImplCurrencyId',
                amount: 'u128',
                fee: 'u128',
                dest: 'XcmVersionedMultiLocation',
                destWeightLimit: 'XcmV3WeightLimit',
            },
            transfer_multiasset_with_fee: {
                asset: 'XcmVersionedMultiAsset',
                fee: 'XcmVersionedMultiAsset',
                dest: 'XcmVersionedMultiLocation',
                destWeightLimit: 'XcmV3WeightLimit',
            },
            transfer_multicurrencies: {
                currencies: 'Vec<(RuntimeCommonXcmImplCurrencyId,u128)>',
                feeItem: 'u32',
                dest: 'XcmVersionedMultiLocation',
                destWeightLimit: 'XcmV3WeightLimit',
            },
            transfer_multiassets: {
                assets: 'XcmVersionedMultiAssets',
                feeItem: 'u32',
                dest: 'XcmVersionedMultiLocation',
                destWeightLimit: 'XcmV3WeightLimit',
            },
        },
    },
    /**
     * Lookup327: xcm::VersionedMultiAsset
     **/
    XcmVersionedMultiAsset: {
        _enum: {
            __Unused0: 'Null',
            V2: 'XcmV2MultiAsset',
            __Unused2: 'Null',
            V3: 'XcmV3MultiAsset',
        },
    },
    /**
     * Lookup330: orml_tokens::module::Call<T>
     **/
    OrmlTokensModuleCall: {
        _enum: {
            transfer: {
                dest: 'MultiAddress',
                currencyId: 'u128',
                amount: 'Compact<u128>',
            },
            transfer_all: {
                dest: 'MultiAddress',
                currencyId: 'u128',
                keepAlive: 'bool',
            },
            transfer_keep_alive: {
                dest: 'MultiAddress',
                currencyId: 'u128',
                amount: 'Compact<u128>',
            },
            force_transfer: {
                source: 'MultiAddress',
                dest: 'MultiAddress',
                currencyId: 'u128',
                amount: 'Compact<u128>',
            },
            set_balance: {
                who: 'MultiAddress',
                currencyId: 'u128',
                newFree: 'Compact<u128>',
                newReserved: 'Compact<u128>',
            },
        },
    },
    /**
     * Lookup331: pallet_bridge::pallet::Call<T>
     **/
    PalletBridgeCall: {
        _enum: {
            set_threshold: {
                threshold: 'u32',
            },
            set_resource: {
                id: '[u8;32]',
                method: 'Bytes',
            },
            remove_resource: {
                id: '[u8;32]',
            },
            whitelist_chain: {
                id: 'u8',
            },
            add_relayer: {
                v: 'AccountId32',
            },
            remove_relayer: {
                v: 'AccountId32',
            },
            update_fee: {
                destId: 'u8',
                fee: 'u128',
            },
            acknowledge_proposal: {
                nonce: 'u64',
                srcId: 'u8',
                rId: '[u8;32]',
                call: 'Call',
            },
            reject_proposal: {
                nonce: 'u64',
                srcId: 'u8',
                rId: '[u8;32]',
                call: 'Call',
            },
            eval_vote_state: {
                nonce: 'u64',
                srcId: 'u8',
                prop: 'Call',
            },
        },
    },
    /**
     * Lookup332: pallet_bridge_transfer::pallet::Call<T>
     **/
    PalletBridgeTransferCall: {
        _enum: {
            transfer_native: {
                amount: 'u128',
                recipient: 'Bytes',
                destId: 'u8',
            },
            transfer: {
                to: 'AccountId32',
                amount: 'u128',
                rid: '[u8;32]',
            },
            set_maximum_issuance: {
                maximumIssuance: 'u128',
            },
            set_external_balances: {
                externalBalances: 'u128',
            },
        },
    },
    /**
     * Lookup333: pallet_drop3::pallet::Call<T>
     **/
    PalletDrop3Call: {
        _enum: {
            set_admin: {
                _alias: {
                    new_: 'new',
                },
                new_: 'AccountId32',
            },
            approve_reward_pool: {
                id: 'u64',
            },
            reject_reward_pool: {
                id: 'u64',
            },
            start_reward_pool: {
                id: 'u64',
            },
            stop_reward_pool: {
                id: 'u64',
            },
            close_reward_pool: {
                id: 'u64',
            },
            propose_reward_pool: {
                name: 'Bytes',
                total: 'u128',
                startAt: 'u32',
                endAt: 'u32',
            },
            send_reward: {
                id: 'u64',
                to: 'AccountId32',
                amount: 'u128',
            },
        },
    },
    /**
     * Lookup334: pallet_extrinsic_filter::pallet::Call<T>
     **/
    PalletExtrinsicFilterCall: {
        _enum: {
            set_mode: {
                mode: 'PalletExtrinsicFilterOperationalMode',
            },
            block_extrinsics: {
                palletNameBytes: 'Bytes',
                functionNameBytes: 'Option<Bytes>',
            },
            unblock_extrinsics: {
                palletNameBytes: 'Bytes',
                functionNameBytes: 'Option<Bytes>',
            },
        },
    },
    /**
     * Lookup335: pallet_identity_management::pallet::Call<T>
     **/
    PalletIdentityManagementCall: {
        _enum: {
            add_delegatee: {
                account: 'AccountId32',
            },
            remove_delegatee: {
                account: 'AccountId32',
            },
            set_user_shielding_key: {
                shard: 'H256',
                encryptedKey: 'Bytes',
            },
            create_identity: {
                shard: 'H256',
                user: 'AccountId32',
                encryptedIdentity: 'Bytes',
                encryptedMetadata: 'Option<Bytes>',
            },
            remove_identity: {
                shard: 'H256',
                encryptedIdentity: 'Bytes',
            },
            verify_identity: {
                shard: 'H256',
                encryptedIdentity: 'Bytes',
                encryptedValidationData: 'Bytes',
            },
            __Unused6: 'Null',
            __Unused7: 'Null',
            __Unused8: 'Null',
            __Unused9: 'Null',
            __Unused10: 'Null',
            __Unused11: 'Null',
            __Unused12: 'Null',
            __Unused13: 'Null',
            __Unused14: 'Null',
            __Unused15: 'Null',
            __Unused16: 'Null',
            __Unused17: 'Null',
            __Unused18: 'Null',
            __Unused19: 'Null',
            __Unused20: 'Null',
            __Unused21: 'Null',
            __Unused22: 'Null',
            __Unused23: 'Null',
            __Unused24: 'Null',
            __Unused25: 'Null',
            __Unused26: 'Null',
            __Unused27: 'Null',
            __Unused28: 'Null',
            __Unused29: 'Null',
            user_shielding_key_set: {
                account: 'AccountId32',
                idGraph: 'CorePrimitivesKeyAesOutput',
                reqExtHash: 'H256',
            },
            identity_created: {
                account: 'AccountId32',
                identity: 'CorePrimitivesKeyAesOutput',
                code: 'CorePrimitivesKeyAesOutput',
                reqExtHash: 'H256',
            },
            identity_removed: {
                account: 'AccountId32',
                identity: 'CorePrimitivesKeyAesOutput',
                reqExtHash: 'H256',
            },
            identity_verified: {
                account: 'AccountId32',
                identity: 'CorePrimitivesKeyAesOutput',
                idGraph: 'CorePrimitivesKeyAesOutput',
                reqExtHash: 'H256',
            },
            some_error: {
                account: 'Option<AccountId32>',
                error: 'CorePrimitivesErrorImpError',
                reqExtHash: 'H256',
            },
        },
    },
    /**
     * Lookup336: core_primitives::error::IMPError
     **/
    CorePrimitivesErrorImpError: {
        _enum: {
            SetUserShieldingKeyFailed: 'CorePrimitivesErrorErrorDetail',
            CreateIdentityFailed: 'CorePrimitivesErrorErrorDetail',
            RemoveIdentityFailed: 'CorePrimitivesErrorErrorDetail',
            VerifyIdentityFailed: 'CorePrimitivesErrorErrorDetail',
            ImportScheduledEnclaveFailed: 'Null',
            UnclassifiedError: 'CorePrimitivesErrorErrorDetail',
        },
    },
    /**
     * Lookup337: pallet_asset_manager::pallet::Call<T>
     **/
    PalletAssetManagerCall: {
        _enum: {
            register_foreign_asset_type: {
                assetType: 'RuntimeCommonXcmImplCurrencyId',
                metadata: 'PalletAssetManagerAssetMetadata',
            },
            update_foreign_asset_metadata: {
                assetId: 'u128',
                metadata: 'PalletAssetManagerAssetMetadata',
            },
            set_asset_units_per_second: {
                assetId: 'u128',
                unitsPerSecond: 'u128',
            },
            add_asset_type: {
                assetId: 'u128',
                newAssetType: 'RuntimeCommonXcmImplCurrencyId',
            },
            remove_asset_type: {
                assetType: 'RuntimeCommonXcmImplCurrencyId',
                newDefaultAssetType: 'Option<RuntimeCommonXcmImplCurrencyId>',
            },
        },
    },
    /**
     * Lookup339: pallet_vc_management::pallet::Call<T>
     **/
    PalletVcManagementCall: {
        _enum: {
            request_vc: {
                shard: 'H256',
                assertion: 'CorePrimitivesAssertion',
            },
            disable_vc: {
                index: 'H256',
            },
            revoke_vc: {
                index: 'H256',
            },
            set_admin: {
                _alias: {
                    new_: 'new',
                },
                new_: 'AccountId32',
            },
            add_schema: {
                shard: 'H256',
                id: 'Bytes',
                content: 'Bytes',
            },
            disable_schema: {
                shard: 'H256',
                index: 'u64',
            },
            activate_schema: {
                shard: 'H256',
                index: 'u64',
            },
            revoke_schema: {
                shard: 'H256',
                index: 'u64',
            },
            add_vc_registry_item: {
                _alias: {
                    hash_: 'hash',
                },
                index: 'H256',
                subject: 'AccountId32',
                assertion: 'CorePrimitivesAssertion',
                hash_: 'H256',
            },
            remove_vc_registry_item: {
                index: 'H256',
            },
            clear_vc_registry: 'Null',
            __Unused11: 'Null',
            __Unused12: 'Null',
            __Unused13: 'Null',
            __Unused14: 'Null',
            __Unused15: 'Null',
            __Unused16: 'Null',
            __Unused17: 'Null',
            __Unused18: 'Null',
            __Unused19: 'Null',
            __Unused20: 'Null',
            __Unused21: 'Null',
            __Unused22: 'Null',
            __Unused23: 'Null',
            __Unused24: 'Null',
            __Unused25: 'Null',
            __Unused26: 'Null',
            __Unused27: 'Null',
            __Unused28: 'Null',
            __Unused29: 'Null',
            vc_issued: {
                _alias: {
                    hash_: 'hash',
                },
                account: 'AccountId32',
                assertion: 'CorePrimitivesAssertion',
                index: 'H256',
                hash_: 'H256',
                vc: 'CorePrimitivesKeyAesOutput',
                reqExtHash: 'H256',
            },
            some_error: {
                account: 'Option<AccountId32>',
                error: 'CorePrimitivesErrorVcmpError',
                reqExtHash: 'H256',
            },
        },
    },
    /**
     * Lookup340: core_primitives::error::VCMPError
     **/
    CorePrimitivesErrorVcmpError: {
        _enum: {
            RequestVCFailed: '(CorePrimitivesAssertion,CorePrimitivesErrorErrorDetail)',
            UnclassifiedError: 'CorePrimitivesErrorErrorDetail',
        },
    },
    /**
     * Lookup341: pallet_group::pallet::Call<T, I>
     **/
    PalletGroupCall: {
        _enum: {
            add_group_member: {
                v: 'AccountId32',
            },
            batch_add_group_members: {
                vs: 'Vec<AccountId32>',
            },
            remove_group_member: {
                v: 'AccountId32',
            },
            batch_remove_group_members: {
                vs: 'Vec<AccountId32>',
            },
            switch_group_control_on: 'Null',
            switch_group_control_off: 'Null',
        },
    },
    /**
     * Lookup343: pallet_teerex::pallet::Call<T>
     **/
    PalletTeerexCall: {
        _enum: {
            register_enclave: {
                raReport: 'Bytes',
                workerUrl: 'Bytes',
                shieldingKey: 'Option<Bytes>',
                vcPubkey: 'Option<Bytes>',
            },
            unregister_enclave: 'Null',
            call_worker: {
                request: 'TeerexPrimitivesRequest',
            },
            confirm_processed_parentchain_block: {
                blockHash: 'H256',
                blockNumber: 'u32',
                trustedCallsMerkleRoot: 'H256',
            },
            shield_funds: {
                incognitoAccountEncrypted: 'Bytes',
                amount: 'u128',
                bondingAccount: 'AccountId32',
            },
            unshield_funds: {
                publicAccount: 'AccountId32',
                amount: 'u128',
                bondingAccount: 'AccountId32',
                callHash: 'H256',
            },
            set_heartbeat_timeout: {
                timeout: 'u64',
            },
            register_dcap_enclave: {
                dcapQuote: 'Bytes',
                workerUrl: 'Bytes',
            },
            update_scheduled_enclave: {
                sidechainBlockNumber: 'u64',
                mrEnclave: '[u8;32]',
            },
            register_quoting_enclave: {
                enclaveIdentity: 'Bytes',
                signature: 'Bytes',
                certificateChain: 'Bytes',
            },
            remove_scheduled_enclave: {
                sidechainBlockNumber: 'u64',
            },
            register_tcb_info: {
                tcbInfo: 'Bytes',
                signature: 'Bytes',
                certificateChain: 'Bytes',
            },
            publish_hash: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
                extraTopics: 'Vec<H256>',
                data: 'Bytes',
            },
            set_admin: {
                _alias: {
                    new_: 'new',
                },
                new_: 'AccountId32',
            },
        },
    },
    /**
     * Lookup344: teerex_primitives::Request
     **/
    TeerexPrimitivesRequest: {
        shard: 'H256',
        cyphertext: 'Bytes',
    },
    /**
     * Lookup345: pallet_sidechain::pallet::Call<T>
     **/
    PalletSidechainCall: {
        _enum: {
            confirm_imported_sidechain_block: {
                shardId: 'H256',
                blockNumber: 'u64',
                nextFinalizationCandidateBlockNumber: 'u64',
                blockHeaderHash: 'H256',
            },
        },
    },
    /**
     * Lookup346: pallet_teeracle::pallet::Call<T>
     **/
    PalletTeeracleCall: {
        _enum: {
            add_to_whitelist: {
                dataSource: 'Bytes',
                mrenclave: '[u8;32]',
            },
            remove_from_whitelist: {
                dataSource: 'Bytes',
                mrenclave: '[u8;32]',
            },
            update_oracle: {
                oracleName: 'Bytes',
                dataSource: 'Bytes',
                newBlob: 'Bytes',
            },
            update_exchange_rate: {
                dataSource: 'Bytes',
                tradingPair: 'Bytes',
                newValue: 'Option<SubstrateFixedFixedU64>',
            },
        },
    },
    /**
     * Lookup348: pallet_identity_management_mock::pallet::Call<T>
     **/
    PalletIdentityManagementMockCall: {
        _enum: {
            add_delegatee: {
                account: 'AccountId32',
            },
            remove_delegatee: {
                account: 'AccountId32',
            },
            set_user_shielding_key: {
                shard: 'H256',
                encryptedKey: 'Bytes',
            },
            create_identity: {
                shard: 'H256',
                user: 'AccountId32',
                encryptedIdentity: 'Bytes',
                encryptedMetadata: 'Option<Bytes>',
            },
            remove_identity: {
                shard: 'H256',
                encryptedIdentity: 'Bytes',
            },
            verify_identity: {
                shard: 'H256',
                encryptedIdentity: 'Bytes',
                encryptedValidationData: 'Bytes',
            },
            user_shielding_key_set: {
                account: 'AccountId32',
            },
            identity_created: {
                account: 'AccountId32',
                identity: 'CorePrimitivesKeyAesOutput',
                code: 'CorePrimitivesKeyAesOutput',
                idGraph: 'CorePrimitivesKeyAesOutput',
            },
            identity_removed: {
                account: 'AccountId32',
                identity: 'CorePrimitivesKeyAesOutput',
                idGraph: 'CorePrimitivesKeyAesOutput',
            },
            identity_verified: {
                account: 'AccountId32',
                identity: 'CorePrimitivesKeyAesOutput',
                idGraph: 'CorePrimitivesKeyAesOutput',
            },
            some_error: {
                func: 'Bytes',
                error: 'Bytes',
            },
        },
    },
    /**
     * Lookup349: pallet_sudo::pallet::Call<T>
     **/
    PalletSudoCall: {
        _enum: {
            sudo: {
                call: 'Call',
            },
            sudo_unchecked_weight: {
                call: 'Call',
                weight: 'SpWeightsWeightV2Weight',
            },
            set_key: {
                _alias: {
                    new_: 'new',
                },
                new_: 'MultiAddress',
            },
            sudo_as: {
                who: 'MultiAddress',
                call: 'Call',
            },
        },
    },
    /**
     * Lookup352: pallet_scheduler::pallet::Error<T>
     **/
    PalletSchedulerError: {
        _enum: ['FailedToSchedule', 'NotFound', 'TargetBlockNumberInPast', 'RescheduleNoChange', 'Named'],
    },
    /**
     * Lookup353: pallet_utility::pallet::Error<T>
     **/
    PalletUtilityError: {
        _enum: ['TooManyCalls'],
    },
    /**
     * Lookup355: pallet_multisig::Multisig<BlockNumber, Balance, sp_core::crypto::AccountId32, MaxApprovals>
     **/
    PalletMultisigMultisig: {
        when: 'PalletMultisigTimepoint',
        deposit: 'u128',
        depositor: 'AccountId32',
        approvals: 'Vec<AccountId32>',
    },
    /**
     * Lookup357: pallet_multisig::pallet::Error<T>
     **/
    PalletMultisigError: {
        _enum: [
            'MinimumThreshold',
            'AlreadyApproved',
            'NoApprovalsNeeded',
            'TooFewSignatories',
            'TooManySignatories',
            'SignatoriesOutOfOrder',
            'SenderInSignatories',
            'NotFound',
            'NotOwner',
            'NoTimepoint',
            'WrongTimepoint',
            'UnexpectedTimepoint',
            'MaxWeightTooLow',
            'AlreadyStored',
        ],
    },
    /**
     * Lookup360: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, rococo_parachain_runtime::ProxyType, BlockNumber>
     **/
    PalletProxyProxyDefinition: {
        delegate: 'AccountId32',
        proxyType: 'RococoParachainRuntimeProxyType',
        delay: 'u32',
    },
    /**
     * Lookup364: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber>
     **/
    PalletProxyAnnouncement: {
        real: 'AccountId32',
        callHash: 'H256',
        height: 'u32',
    },
    /**
     * Lookup366: pallet_proxy::pallet::Error<T>
     **/
    PalletProxyError: {
        _enum: [
            'TooMany',
            'NotFound',
            'NotProxy',
            'Unproxyable',
            'Duplicate',
            'NoPermission',
            'Unannounced',
            'NoSelfProxy',
        ],
    },
    /**
     * Lookup367: pallet_preimage::RequestStatus<sp_core::crypto::AccountId32, Balance>
     **/
    PalletPreimageRequestStatus: {
        _enum: {
            Unrequested: {
                deposit: '(AccountId32,u128)',
                len: 'u32',
            },
            Requested: {
                deposit: 'Option<(AccountId32,u128)>',
                count: 'u32',
                len: 'Option<u32>',
            },
        },
    },
    /**
     * Lookup372: pallet_preimage::pallet::Error<T>
     **/
    PalletPreimageError: {
        _enum: ['TooBig', 'AlreadyNoted', 'NotAuthorized', 'NotNoted', 'Requested', 'NotRequested'],
    },
    /**
     * Lookup374: pallet_balances::BalanceLock<Balance>
     **/
    PalletBalancesBalanceLock: {
        id: '[u8;8]',
        amount: 'u128',
        reasons: 'PalletBalancesReasons',
    },
    /**
     * Lookup375: pallet_balances::Reasons
     **/
    PalletBalancesReasons: {
        _enum: ['Fee', 'Misc', 'All'],
    },
    /**
     * Lookup378: pallet_balances::ReserveData<ReserveIdentifier, Balance>
     **/
    PalletBalancesReserveData: {
        id: '[u8;8]',
        amount: 'u128',
    },
    /**
     * Lookup380: pallet_balances::pallet::Error<T, I>
     **/
    PalletBalancesError: {
        _enum: [
            'VestingBalance',
            'LiquidityRestrictions',
            'InsufficientBalance',
            'ExistentialDeposit',
            'KeepAlive',
            'ExistingVestingSchedule',
            'DeadAccount',
            'TooManyReserves',
        ],
    },
    /**
     * Lookup383: pallet_vesting::Releases
     **/
    PalletVestingReleases: {
        _enum: ['V0', 'V1'],
    },
    /**
     * Lookup384: pallet_vesting::pallet::Error<T>
     **/
    PalletVestingError: {
        _enum: [
            'NotVesting',
            'AtMaxVestingSchedules',
            'AmountLow',
            'ScheduleIndexOutOfBounds',
            'InvalidScheduleParams',
        ],
    },
    /**
     * Lookup386: pallet_transaction_payment::Releases
     **/
    PalletTransactionPaymentReleases: {
        _enum: ['V1Ancient', 'V2'],
    },
    /**
     * Lookup387: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
     **/
    PalletTreasuryProposal: {
        proposer: 'AccountId32',
        value: 'u128',
        beneficiary: 'AccountId32',
        bond: 'u128',
    },
    /**
     * Lookup392: frame_support::PalletId
     **/
    FrameSupportPalletId: '[u8;8]',
    /**
     * Lookup393: pallet_treasury::pallet::Error<T, I>
     **/
    PalletTreasuryError: {
        _enum: [
            'InsufficientProposersBalance',
            'InvalidIndex',
            'TooManyApprovals',
            'InsufficientPermission',
            'ProposalNotApproved',
        ],
    },
    /**
     * Lookup398: pallet_democracy::types::ReferendumInfo<BlockNumber, frame_support::traits::preimages::Bounded<rococo_parachain_runtime::RuntimeCall>, Balance>
     **/
    PalletDemocracyReferendumInfo: {
        _enum: {
            Ongoing: 'PalletDemocracyReferendumStatus',
            Finished: {
                approved: 'bool',
                end: 'u32',
            },
        },
    },
    /**
     * Lookup399: pallet_democracy::types::ReferendumStatus<BlockNumber, frame_support::traits::preimages::Bounded<rococo_parachain_runtime::RuntimeCall>, Balance>
     **/
    PalletDemocracyReferendumStatus: {
        end: 'u32',
        proposal: 'FrameSupportPreimagesBounded',
        threshold: 'PalletDemocracyVoteThreshold',
        delay: 'u32',
        tally: 'PalletDemocracyTally',
    },
    /**
     * Lookup400: pallet_democracy::types::Tally<Balance>
     **/
    PalletDemocracyTally: {
        ayes: 'u128',
        nays: 'u128',
        turnout: 'u128',
    },
    /**
     * Lookup401: pallet_democracy::vote::Voting<Balance, sp_core::crypto::AccountId32, BlockNumber, MaxVotes>
     **/
    PalletDemocracyVoteVoting: {
        _enum: {
            Direct: {
                votes: 'Vec<(u32,PalletDemocracyVoteAccountVote)>',
                delegations: 'PalletDemocracyDelegations',
                prior: 'PalletDemocracyVotePriorLock',
            },
            Delegating: {
                balance: 'u128',
                target: 'AccountId32',
                conviction: 'PalletDemocracyConviction',
                delegations: 'PalletDemocracyDelegations',
                prior: 'PalletDemocracyVotePriorLock',
            },
        },
    },
    /**
     * Lookup405: pallet_democracy::types::Delegations<Balance>
     **/
    PalletDemocracyDelegations: {
        votes: 'u128',
        capital: 'u128',
    },
    /**
     * Lookup406: pallet_democracy::vote::PriorLock<BlockNumber, Balance>
     **/
    PalletDemocracyVotePriorLock: '(u32,u128)',
    /**
     * Lookup409: pallet_democracy::pallet::Error<T>
     **/
    PalletDemocracyError: {
        _enum: [
            'ValueLow',
            'ProposalMissing',
            'AlreadyCanceled',
            'DuplicateProposal',
            'ProposalBlacklisted',
            'NotSimpleMajority',
            'InvalidHash',
            'NoProposal',
            'AlreadyVetoed',
            'ReferendumInvalid',
            'NoneWaiting',
            'NotVoter',
            'NoPermission',
            'AlreadyDelegating',
            'InsufficientFunds',
            'NotDelegating',
            'VotesExist',
            'InstantNotAllowed',
            'Nonsense',
            'WrongUpperBound',
            'MaxVotesReached',
            'TooMany',
            'VotingPeriodLow',
            'PreimageNotExist',
        ],
    },
    /**
     * Lookup411: pallet_collective::Votes<sp_core::crypto::AccountId32, BlockNumber>
     **/
    PalletCollectiveVotes: {
        index: 'u32',
        threshold: 'u32',
        ayes: 'Vec<AccountId32>',
        nays: 'Vec<AccountId32>',
        end: 'u32',
    },
    /**
     * Lookup412: pallet_collective::pallet::Error<T, I>
     **/
    PalletCollectiveError: {
        _enum: [
            'NotMember',
            'DuplicateProposal',
            'ProposalMissing',
            'WrongIndex',
            'DuplicateVote',
            'AlreadyInitialized',
            'TooEarly',
            'TooManyProposals',
            'WrongProposalWeight',
            'WrongProposalLength',
        ],
    },
    /**
     * Lookup414: pallet_membership::pallet::Error<T, I>
     **/
    PalletMembershipError: {
        _enum: ['AlreadyMember', 'NotMember', 'TooManyMembers'],
    },
    /**
     * Lookup417: pallet_bounties::Bounty<sp_core::crypto::AccountId32, Balance, BlockNumber>
     **/
    PalletBountiesBounty: {
        proposer: 'AccountId32',
        value: 'u128',
        fee: 'u128',
        curatorDeposit: 'u128',
        bond: 'u128',
        status: 'PalletBountiesBountyStatus',
    },
    /**
     * Lookup418: pallet_bounties::BountyStatus<sp_core::crypto::AccountId32, BlockNumber>
     **/
    PalletBountiesBountyStatus: {
        _enum: {
            Proposed: 'Null',
            Approved: 'Null',
            Funded: 'Null',
            CuratorProposed: {
                curator: 'AccountId32',
            },
            Active: {
                curator: 'AccountId32',
                updateDue: 'u32',
            },
            PendingPayout: {
                curator: 'AccountId32',
                beneficiary: 'AccountId32',
                unlockAt: 'u32',
            },
        },
    },
    /**
     * Lookup420: pallet_bounties::pallet::Error<T, I>
     **/
    PalletBountiesError: {
        _enum: [
            'InsufficientProposersBalance',
            'InvalidIndex',
            'ReasonTooBig',
            'UnexpectedStatus',
            'RequireCurator',
            'InvalidValue',
            'InvalidFee',
            'PendingPayout',
            'Premature',
            'HasActiveChildBounty',
            'TooManyQueued',
        ],
    },
    /**
     * Lookup421: pallet_tips::OpenTip<sp_core::crypto::AccountId32, Balance, BlockNumber, primitive_types::H256>
     **/
    PalletTipsOpenTip: {
        reason: 'H256',
        who: 'AccountId32',
        finder: 'AccountId32',
        deposit: 'u128',
        closes: 'Option<u32>',
        tips: 'Vec<(AccountId32,u128)>',
        findersFee: 'bool',
    },
    /**
     * Lookup423: pallet_tips::pallet::Error<T, I>
     **/
    PalletTipsError: {
        _enum: ['ReasonTooBig', 'AlreadyKnown', 'UnknownTip', 'NotFinder', 'StillOpen', 'Premature'],
    },
    /**
     * Lookup424: pallet_identity::types::Registration<Balance, MaxJudgements, MaxAdditionalFields>
     **/
    PalletIdentityRegistration: {
        judgements: 'Vec<(u32,PalletIdentityJudgement)>',
        deposit: 'u128',
        info: 'PalletIdentityIdentityInfo',
    },
    /**
     * Lookup431: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32>
     **/
    PalletIdentityRegistrarInfo: {
        account: 'AccountId32',
        fee: 'u128',
        fields: 'PalletIdentityBitFlags',
    },
    /**
     * Lookup433: pallet_identity::pallet::Error<T>
     **/
    PalletIdentityError: {
        _enum: [
            'TooManySubAccounts',
            'NotFound',
            'NotNamed',
            'EmptyIndex',
            'FeeChanged',
            'NoIdentity',
            'StickyJudgement',
            'JudgementGiven',
            'InvalidJudgement',
            'InvalidIndex',
            'InvalidTarget',
            'TooManyFields',
            'TooManyRegistrars',
            'AlreadyClaimed',
            'NotSub',
            'NotOwned',
            'JudgementForDifferentIdentity',
            'JudgementPaymentFailed',
        ],
    },
    /**
     * Lookup435: polkadot_primitives::v2::UpgradeRestriction
     **/
    PolkadotPrimitivesV2UpgradeRestriction: {
        _enum: ['Present'],
    },
    /**
     * Lookup436: cumulus_pallet_parachain_system::relay_state_snapshot::MessagingStateSnapshot
     **/
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: {
        dmqMqcHead: 'H256',
        relayDispatchQueueSize: '(u32,u32)',
        ingressChannels: 'Vec<(u32,PolkadotPrimitivesV2AbridgedHrmpChannel)>',
        egressChannels: 'Vec<(u32,PolkadotPrimitivesV2AbridgedHrmpChannel)>',
    },
    /**
     * Lookup439: polkadot_primitives::v2::AbridgedHrmpChannel
     **/
    PolkadotPrimitivesV2AbridgedHrmpChannel: {
        maxCapacity: 'u32',
        maxTotalSize: 'u32',
        maxMessageSize: 'u32',
        msgCount: 'u32',
        totalSize: 'u32',
        mqcHead: 'Option<H256>',
    },
    /**
     * Lookup440: polkadot_primitives::v2::AbridgedHostConfiguration
     **/
    PolkadotPrimitivesV2AbridgedHostConfiguration: {
        maxCodeSize: 'u32',
        maxHeadDataSize: 'u32',
        maxUpwardQueueCount: 'u32',
        maxUpwardQueueSize: 'u32',
        maxUpwardMessageSize: 'u32',
        maxUpwardMessageNumPerCandidate: 'u32',
        hrmpMaxMessageNumPerCandidate: 'u32',
        validationUpgradeCooldown: 'u32',
        validationUpgradeDelay: 'u32',
    },
    /**
     * Lookup446: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain::primitives::Id>
     **/
    PolkadotCorePrimitivesOutboundHrmpMessage: {
        recipient: 'u32',
        data: 'Bytes',
    },
    /**
     * Lookup447: cumulus_pallet_parachain_system::pallet::Error<T>
     **/
    CumulusPalletParachainSystemError: {
        _enum: [
            'OverlappingUpgrades',
            'ProhibitedByPolkadot',
            'TooBig',
            'ValidationDataNotAvailable',
            'HostConfigurationNotAvailable',
            'NotScheduled',
            'NothingAuthorized',
            'Unauthorized',
        ],
    },
    /**
     * Lookup451: sp_core::crypto::KeyTypeId
     **/
    SpCoreCryptoKeyTypeId: '[u8;4]',
    /**
     * Lookup452: pallet_session::pallet::Error<T>
     **/
    PalletSessionError: {
        _enum: ['InvalidProof', 'NoAssociatedValidatorId', 'DuplicatedKey', 'NoKeys', 'NoAccount'],
    },
    /**
     * Lookup456: pallet_parachain_staking::types::ParachainBondConfig<sp_core::crypto::AccountId32>
     **/
    PalletParachainStakingParachainBondConfig: {
        account: 'AccountId32',
        percent: 'Percent',
    },
    /**
     * Lookup457: pallet_parachain_staking::types::RoundInfo<BlockNumber>
     **/
    PalletParachainStakingRoundInfo: {
        current: 'u32',
        first: 'u32',
        length: 'u32',
    },
    /**
     * Lookup458: pallet_parachain_staking::types::Delegator<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingDelegator: {
        id: 'AccountId32',
        delegations: 'PalletParachainStakingSetOrderedSet',
        total: 'u128',
        lessTotal: 'u128',
        status: 'PalletParachainStakingDelegatorStatus',
    },
    /**
     * Lookup459: pallet_parachain_staking::set::OrderedSet<pallet_parachain_staking::types::Bond<sp_core::crypto::AccountId32, Balance>>
     **/
    PalletParachainStakingSetOrderedSet: 'Vec<PalletParachainStakingBond>',
    /**
     * Lookup460: pallet_parachain_staking::types::Bond<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingBond: {
        owner: 'AccountId32',
        amount: 'u128',
    },
    /**
     * Lookup462: pallet_parachain_staking::types::DelegatorStatus
     **/
    PalletParachainStakingDelegatorStatus: {
        _enum: ['Active'],
    },
    /**
     * Lookup463: pallet_parachain_staking::types::CandidateMetadata<Balance>
     **/
    PalletParachainStakingCandidateMetadata: {
        bond: 'u128',
        delegationCount: 'u32',
        totalCounted: 'u128',
        lowestTopDelegationAmount: 'u128',
        highestBottomDelegationAmount: 'u128',
        lowestBottomDelegationAmount: 'u128',
        topCapacity: 'PalletParachainStakingCapacityStatus',
        bottomCapacity: 'PalletParachainStakingCapacityStatus',
        request: 'Option<PalletParachainStakingCandidateBondLessRequest>',
        status: 'PalletParachainStakingCollatorStatus',
    },
    /**
     * Lookup464: pallet_parachain_staking::types::CapacityStatus
     **/
    PalletParachainStakingCapacityStatus: {
        _enum: ['Full', 'Empty', 'Partial'],
    },
    /**
     * Lookup466: pallet_parachain_staking::types::CandidateBondLessRequest<Balance>
     **/
    PalletParachainStakingCandidateBondLessRequest: {
        amount: 'u128',
        whenExecutable: 'u32',
    },
    /**
     * Lookup467: pallet_parachain_staking::types::CollatorStatus
     **/
    PalletParachainStakingCollatorStatus: {
        _enum: {
            Active: 'Null',
            Idle: 'Null',
            Leaving: 'u32',
        },
    },
    /**
     * Lookup469: pallet_parachain_staking::delegation_requests::ScheduledRequest<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingDelegationRequestsScheduledRequest: {
        delegator: 'AccountId32',
        whenExecutable: 'u32',
        action: 'PalletParachainStakingDelegationRequestsDelegationAction',
    },
    /**
     * Lookup471: pallet_parachain_staking::auto_compound::AutoCompoundConfig<sp_core::crypto::AccountId32>
     **/
    PalletParachainStakingAutoCompoundAutoCompoundConfig: {
        delegator: 'AccountId32',
        value: 'Percent',
    },
    /**
     * Lookup472: pallet_parachain_staking::types::Delegations<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingDelegations: {
        delegations: 'Vec<PalletParachainStakingBond>',
        total: 'u128',
    },
    /**
     * Lookup474: pallet_parachain_staking::types::CollatorSnapshot<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingCollatorSnapshot: {
        bond: 'u128',
        delegations: 'Vec<PalletParachainStakingBondWithAutoCompound>',
        total: 'u128',
    },
    /**
     * Lookup476: pallet_parachain_staking::types::BondWithAutoCompound<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingBondWithAutoCompound: {
        owner: 'AccountId32',
        amount: 'u128',
        autoCompound: 'Percent',
    },
    /**
     * Lookup477: pallet_parachain_staking::types::DelayedPayout<Balance>
     **/
    PalletParachainStakingDelayedPayout: {
        roundIssuance: 'u128',
        totalStakingReward: 'u128',
        collatorCommission: 'Perbill',
    },
    /**
     * Lookup478: pallet_parachain_staking::inflation::InflationInfo<Balance>
     **/
    PalletParachainStakingInflationInflationInfo: {
        expect: {
            min: 'u128',
            ideal: 'u128',
            max: 'u128',
        },
        annual: {
            min: 'Perbill',
            ideal: 'Perbill',
            max: 'Perbill',
        },
        round: {
            min: 'Perbill',
            ideal: 'Perbill',
            max: 'Perbill',
        },
    },
    /**
     * Lookup479: pallet_parachain_staking::pallet::Error<T>
     **/
    PalletParachainStakingError: {
        _enum: [
            'DelegatorDNE',
            'DelegatorDNEinTopNorBottom',
            'DelegatorDNEInDelegatorSet',
            'CandidateDNE',
            'DelegationDNE',
            'DelegatorExists',
            'CandidateExists',
            'CandidateBondBelowMin',
            'InsufficientBalance',
            'DelegatorBondBelowMin',
            'DelegationBelowMin',
            'AlreadyOffline',
            'AlreadyActive',
            'DelegatorAlreadyLeaving',
            'DelegatorNotLeaving',
            'DelegatorCannotLeaveYet',
            'CannotDelegateIfLeaving',
            'CandidateAlreadyLeaving',
            'CandidateNotLeaving',
            'CandidateCannotLeaveYet',
            'CannotGoOnlineIfLeaving',
            'ExceedMaxDelegationsPerDelegator',
            'AlreadyDelegatedCandidate',
            'InvalidSchedule',
            'CannotSetBelowMin',
            'RoundLengthMustBeGreaterThanTotalSelectedCollators',
            'NoWritingSameValue',
            'TooLowCandidateCountWeightHintCancelLeaveCandidates',
            'TooLowCandidateDelegationCountToLeaveCandidates',
            'PendingCandidateRequestsDNE',
            'PendingCandidateRequestAlreadyExists',
            'PendingCandidateRequestNotDueYet',
            'PendingDelegationRequestDNE',
            'PendingDelegationRequestAlreadyExists',
            'PendingDelegationRequestNotDueYet',
            'CannotDelegateLessThanOrEqualToLowestBottomWhenFull',
            'PendingDelegationRevoke',
            'CandidateUnauthorized',
        ],
    },
    /**
     * Lookup481: cumulus_pallet_xcmp_queue::InboundChannelDetails
     **/
    CumulusPalletXcmpQueueInboundChannelDetails: {
        sender: 'u32',
        state: 'CumulusPalletXcmpQueueInboundState',
        messageMetadata: 'Vec<(u32,PolkadotParachainPrimitivesXcmpMessageFormat)>',
    },
    /**
     * Lookup482: cumulus_pallet_xcmp_queue::InboundState
     **/
    CumulusPalletXcmpQueueInboundState: {
        _enum: ['Ok', 'Suspended'],
    },
    /**
     * Lookup485: polkadot_parachain::primitives::XcmpMessageFormat
     **/
    PolkadotParachainPrimitivesXcmpMessageFormat: {
        _enum: ['ConcatenatedVersionedXcm', 'ConcatenatedEncodedBlob', 'Signals'],
    },
    /**
     * Lookup488: cumulus_pallet_xcmp_queue::OutboundChannelDetails
     **/
    CumulusPalletXcmpQueueOutboundChannelDetails: {
        recipient: 'u32',
        state: 'CumulusPalletXcmpQueueOutboundState',
        signalsExist: 'bool',
        firstIndex: 'u16',
        lastIndex: 'u16',
    },
    /**
     * Lookup489: cumulus_pallet_xcmp_queue::OutboundState
     **/
    CumulusPalletXcmpQueueOutboundState: {
        _enum: ['Ok', 'Suspended'],
    },
    /**
     * Lookup491: cumulus_pallet_xcmp_queue::QueueConfigData
     **/
    CumulusPalletXcmpQueueQueueConfigData: {
        suspendThreshold: 'u32',
        dropThreshold: 'u32',
        resumeThreshold: 'u32',
        thresholdWeight: 'SpWeightsWeightV2Weight',
        weightRestrictDecay: 'SpWeightsWeightV2Weight',
        xcmpMaxIndividualWeight: 'SpWeightsWeightV2Weight',
    },
    /**
     * Lookup493: cumulus_pallet_xcmp_queue::pallet::Error<T>
     **/
    CumulusPalletXcmpQueueError: {
        _enum: ['FailedToSend', 'BadXcmOrigin', 'BadXcm', 'BadOverweightIndex', 'WeightOverLimit'],
    },
    /**
     * Lookup494: pallet_xcm::pallet::QueryStatus<BlockNumber>
     **/
    PalletXcmQueryStatus: {
        _enum: {
            Pending: {
                responder: 'XcmVersionedMultiLocation',
                maybeMatchQuerier: 'Option<XcmVersionedMultiLocation>',
                maybeNotify: 'Option<(u8,u8)>',
                timeout: 'u32',
            },
            VersionNotifier: {
                origin: 'XcmVersionedMultiLocation',
                isActive: 'bool',
            },
            Ready: {
                response: 'XcmVersionedResponse',
                at: 'u32',
            },
        },
    },
    /**
     * Lookup498: xcm::VersionedResponse
     **/
    XcmVersionedResponse: {
        _enum: {
            __Unused0: 'Null',
            __Unused1: 'Null',
            V2: 'XcmV2Response',
            V3: 'XcmV3Response',
        },
    },
    /**
     * Lookup504: pallet_xcm::pallet::VersionMigrationStage
     **/
    PalletXcmVersionMigrationStage: {
        _enum: {
            MigrateSupportedVersion: 'Null',
            MigrateVersionNotifiers: 'Null',
            NotifyCurrentTargets: 'Option<Bytes>',
            MigrateAndNotifyOldTargets: 'Null',
        },
    },
    /**
     * Lookup506: xcm::VersionedAssetId
     **/
    XcmVersionedAssetId: {
        _enum: {
            __Unused0: 'Null',
            __Unused1: 'Null',
            __Unused2: 'Null',
            V3: 'XcmV3MultiassetAssetId',
        },
    },
    /**
     * Lookup507: pallet_xcm::pallet::RemoteLockedFungibleRecord
     **/
    PalletXcmRemoteLockedFungibleRecord: {
        amount: 'u128',
        owner: 'XcmVersionedMultiLocation',
        locker: 'XcmVersionedMultiLocation',
        users: 'u32',
    },
    /**
     * Lookup511: pallet_xcm::pallet::Error<T>
     **/
    PalletXcmError: {
        _enum: [
            'Unreachable',
            'SendFailure',
            'Filtered',
            'UnweighableMessage',
            'DestinationNotInvertible',
            'Empty',
            'CannotReanchor',
            'TooManyAssets',
            'InvalidOrigin',
            'BadVersion',
            'BadLocation',
            'NoSubscription',
            'AlreadySubscribed',
            'InvalidAsset',
            'LowBalance',
            'TooManyLocks',
            'AccountNotSovereign',
            'FeesNotMet',
            'LockNotFound',
            'InUse',
        ],
    },
    /**
     * Lookup512: cumulus_pallet_xcm::pallet::Error<T>
     **/
    CumulusPalletXcmError: 'Null',
    /**
     * Lookup513: cumulus_pallet_dmp_queue::ConfigData
     **/
    CumulusPalletDmpQueueConfigData: {
        maxIndividual: 'SpWeightsWeightV2Weight',
    },
    /**
     * Lookup514: cumulus_pallet_dmp_queue::PageIndexData
     **/
    CumulusPalletDmpQueuePageIndexData: {
        beginUsed: 'u32',
        endUsed: 'u32',
        overweightCount: 'u64',
    },
    /**
     * Lookup517: cumulus_pallet_dmp_queue::pallet::Error<T>
     **/
    CumulusPalletDmpQueueError: {
        _enum: ['Unknown', 'OverLimit'],
    },
    /**
     * Lookup518: orml_xtokens::module::Error<T>
     **/
    OrmlXtokensModuleError: {
        _enum: [
            'AssetHasNoReserve',
            'NotCrossChainTransfer',
            'InvalidDest',
            'NotCrossChainTransferableCurrency',
            'UnweighableMessage',
            'XcmExecutionFailed',
            'CannotReanchor',
            'InvalidAncestry',
            'InvalidAsset',
            'DestinationNotInvertible',
            'BadVersion',
            'DistinctReserveForAssetAndFee',
            'ZeroFee',
            'ZeroAmount',
            'TooManyAssetsBeingSent',
            'AssetIndexNonExistent',
            'FeeNotEnough',
            'NotSupportedMultiLocation',
            'MinXcmFeeNotDefined',
        ],
    },
    /**
     * Lookup520: orml_tokens::BalanceLock<Balance>
     **/
    OrmlTokensBalanceLock: {
        id: '[u8;8]',
        amount: 'u128',
    },
    /**
     * Lookup522: orml_tokens::AccountData<Balance>
     **/
    OrmlTokensAccountData: {
        free: 'u128',
        reserved: 'u128',
        frozen: 'u128',
    },
    /**
     * Lookup524: orml_tokens::ReserveData<ReserveIdentifier, Balance>
     **/
    OrmlTokensReserveData: {
        id: '[u8;8]',
        amount: 'u128',
    },
    /**
     * Lookup526: orml_tokens::module::Error<T>
     **/
    OrmlTokensModuleError: {
        _enum: [
            'BalanceTooLow',
            'AmountIntoBalanceFailed',
            'LiquidityRestrictions',
            'MaxLocksExceeded',
            'KeepAlive',
            'ExistentialDeposit',
            'DeadAccount',
            'TooManyReserves',
        ],
    },
    /**
     * Lookup529: pallet_bridge::pallet::ProposalVotes<sp_core::crypto::AccountId32, BlockNumber>
     **/
    PalletBridgeProposalVotes: {
        votesFor: 'Vec<AccountId32>',
        votesAgainst: 'Vec<AccountId32>',
        status: 'PalletBridgeProposalStatus',
        expiry: 'u32',
    },
    /**
     * Lookup530: pallet_bridge::pallet::ProposalStatus
     **/
    PalletBridgeProposalStatus: {
        _enum: ['Initiated', 'Approved', 'Rejected'],
    },
    /**
     * Lookup532: pallet_bridge::pallet::BridgeEvent
     **/
    PalletBridgeBridgeEvent: {
        _enum: {
            FungibleTransfer: '(u8,u64,[u8;32],u128,Bytes)',
            NonFungibleTransfer: '(u8,u64,[u8;32],Bytes,Bytes,Bytes)',
            GenericTransfer: '(u8,u64,[u8;32],Bytes)',
        },
    },
    /**
     * Lookup533: pallet_bridge::pallet::Error<T>
     **/
    PalletBridgeError: {
        _enum: [
            'ThresholdNotSet',
            'InvalidChainId',
            'InvalidThreshold',
            'ChainNotWhitelisted',
            'ChainAlreadyWhitelisted',
            'ResourceDoesNotExist',
            'RelayerAlreadyExists',
            'RelayerInvalid',
            'MustBeRelayer',
            'RelayerAlreadyVoted',
            'ProposalAlreadyExists',
            'ProposalDoesNotExist',
            'ProposalNotComplete',
            'ProposalAlreadyComplete',
            'ProposalExpired',
            'FeeTooExpensive',
            'FeeDoesNotExist',
            'InsufficientBalance',
            'CannotPayAsFee',
            'NonceOverFlow',
        ],
    },
    /**
     * Lookup535: pallet_bridge_transfer::pallet::Error<T>
     **/
    PalletBridgeTransferError: {
        _enum: ['InvalidCommand', 'InvalidResourceId', 'ReachMaximumSupply', 'OverFlow'],
    },
    /**
     * Lookup536: pallet_drop3::RewardPool<PoolId, bounded_collections::bounded_vec::BoundedVec<T, S>, sp_core::crypto::AccountId32, Balance, BlockNumber>
     **/
    PalletDrop3RewardPool: {
        id: 'u64',
        name: 'Bytes',
        owner: 'AccountId32',
        total: 'u128',
        remain: 'u128',
        createAt: 'u32',
        startAt: 'u32',
        endAt: 'u32',
        started: 'bool',
        approved: 'bool',
    },
    /**
     * Lookup538: pallet_drop3::pallet::Error<T>
     **/
    PalletDrop3Error: {
        _enum: [
            'RequireAdmin',
            'RequireRewardPoolOwner',
            'RequireAdminOrRewardPoolOwner',
            'NoSuchRewardPool',
            'InsufficientReservedBalance',
            'InvalidTotalBalance',
            'InsufficientRemain',
            'InvalidProposedBlock',
            'RewardPoolUnapproved',
            'RewardPoolAlreadyApproved',
            'RewardPoolStopped',
            'RewardPoolRanTooEarly',
            'RewardPoolRanTooLate',
            'UnexpectedUnMovedAmount',
            'NoVacantPoolId',
        ],
    },
    /**
     * Lookup539: pallet_extrinsic_filter::pallet::Error<T>
     **/
    PalletExtrinsicFilterError: {
        _enum: ['CannotBlock', 'CannotConvertToString', 'ExtrinsicAlreadyBlocked', 'ExtrinsicNotBlocked'],
    },
    /**
     * Lookup540: pallet_identity_management::pallet::Error<T>
     **/
    PalletIdentityManagementError: {
        _enum: ['DelegateeNotExist', 'UnauthorisedUser'],
    },
    /**
     * Lookup541: pallet_asset_manager::pallet::Error<T>
     **/
    PalletAssetManagerError: {
        _enum: [
            'AssetAlreadyExists',
            'AssetTypeDoesNotExist',
            'AssetIdDoesNotExist',
            'DefaultAssetTypeRemoved',
            'AssetIdLimitReached',
        ],
    },
    /**
     * Lookup542: pallet_vc_management::vc_context::VCContext<T>
     **/
    PalletVcManagementVcContext: {
        _alias: {
            hash_: 'hash',
        },
        subject: 'AccountId32',
        assertion: 'CorePrimitivesAssertion',
        hash_: 'H256',
        status: 'PalletVcManagementVcContextStatus',
    },
    /**
     * Lookup543: pallet_vc_management::vc_context::Status
     **/
    PalletVcManagementVcContextStatus: {
        _enum: ['Active', 'Disabled'],
    },
    /**
     * Lookup544: pallet_vc_management::schema::VCSchema<T>
     **/
    PalletVcManagementSchemaVcSchema: {
        id: 'Bytes',
        author: 'AccountId32',
        content: 'Bytes',
        status: 'PalletVcManagementVcContextStatus',
    },
    /**
     * Lookup546: pallet_vc_management::pallet::Error<T>
     **/
    PalletVcManagementError: {
        _enum: [
            'VCAlreadyExists',
            'VCNotExist',
            'VCSubjectMismatch',
            'VCAlreadyDisabled',
            'RequireAdmin',
            'SchemaNotExists',
            'SchemaAlreadyDisabled',
            'SchemaAlreadyActivated',
            'SchemaIndexOverFlow',
            'LengthMismatch',
        ],
    },
    /**
     * Lookup547: pallet_group::pallet::Error<T, I>
     **/
    PalletGroupError: {
        _enum: ['GroupMemberAlreadyExists', 'GroupMemberInvalid'],
    },
    /**
     * Lookup549: teerex_primitives::Enclave<sp_core::crypto::AccountId32, Url>
     **/
    TeerexPrimitivesEnclave: {
        pubkey: 'AccountId32',
        mrEnclave: '[u8;32]',
        timestamp: 'u64',
        url: 'Bytes',
        shieldingKey: 'Option<Bytes>',
        vcPubkey: 'Option<Bytes>',
        sgxMode: 'TeerexPrimitivesSgxBuildMode',
        sgxMetadata: 'TeerexPrimitivesSgxEnclaveMetadata',
    },
    /**
     * Lookup550: teerex_primitives::SgxBuildMode
     **/
    TeerexPrimitivesSgxBuildMode: {
        _enum: ['Debug', 'Production'],
    },
    /**
     * Lookup551: teerex_primitives::SgxEnclaveMetadata
     **/
    TeerexPrimitivesSgxEnclaveMetadata: {
        quote: 'Bytes',
        quoteSig: 'Bytes',
        quoteCert: 'Bytes',
    },
    /**
     * Lookup552: teerex_primitives::QuotingEnclave
     **/
    TeerexPrimitivesQuotingEnclave: {
        issueDate: 'u64',
        nextUpdate: 'u64',
        miscselect: '[u8;4]',
        miscselectMask: '[u8;4]',
        attributes: '[u8;16]',
        attributesMask: '[u8;16]',
        mrsigner: '[u8;32]',
        isvprodid: 'u16',
        tcb: 'Vec<TeerexPrimitivesQeTcb>',
    },
    /**
     * Lookup554: teerex_primitives::QeTcb
     **/
    TeerexPrimitivesQeTcb: {
        isvsvn: 'u16',
    },
    /**
     * Lookup555: teerex_primitives::TcbInfoOnChain
     **/
    TeerexPrimitivesTcbInfoOnChain: {
        issueDate: 'u64',
        nextUpdate: 'u64',
        tcbLevels: 'Vec<TeerexPrimitivesTcbVersionStatus>',
    },
    /**
     * Lookup557: teerex_primitives::TcbVersionStatus
     **/
    TeerexPrimitivesTcbVersionStatus: {
        cpusvn: '[u8;16]',
        pcesvn: 'u16',
    },
    /**
     * Lookup558: pallet_teerex::pallet::Error<T>
     **/
    PalletTeerexError: {
        _enum: [
            'RequireAdmin',
            'EnclaveSignerDecodeError',
            'SenderIsNotAttestedEnclave',
            'RemoteAttestationVerificationFailed',
            'RemoteAttestationTooOld',
            'SgxModeNotAllowed',
            'EnclaveIsNotRegistered',
            'WrongMrenclaveForBondingAccount',
            'WrongMrenclaveForShard',
            'EnclaveUrlTooLong',
            'RaReportTooLong',
            'EmptyEnclaveRegistry',
            'ScheduledEnclaveNotExist',
            'EnclaveNotInSchedule',
            'CollateralInvalid',
            'TooManyTopics',
            'DataTooLong',
        ],
    },
    /**
     * Lookup559: sidechain_primitives::SidechainBlockConfirmation
     **/
    SidechainPrimitivesSidechainBlockConfirmation: {
        blockNumber: 'u64',
        blockHeaderHash: 'H256',
    },
    /**
     * Lookup560: pallet_sidechain::pallet::Error<T>
     **/
    PalletSidechainError: {
        _enum: ['ReceivedUnexpectedSidechainBlock', 'InvalidNextFinalizationCandidateBlockNumber'],
    },
    /**
     * Lookup563: pallet_teeracle::pallet::Error<T>
     **/
    PalletTeeracleError: {
        _enum: [
            'InvalidCurrency',
            'ReleaseWhitelistOverflow',
            'ReleaseNotWhitelisted',
            'ReleaseAlreadyWhitelisted',
            'TradingPairStringTooLong',
            'OracleDataNameStringTooLong',
            'DataSourceStringTooLong',
            'OracleBlobTooBig',
        ],
    },
    /**
     * Lookup565: pallet_identity_management_mock::pallet::Error<T>
     **/
    PalletIdentityManagementMockError: {
        _enum: [
            'DelegateeNotExist',
            'UnauthorisedUser',
            'ShieldingKeyDecryptionFailed',
            'WrongDecodedType',
            'IdentityAlreadyVerified',
            'IdentityNotExist',
            'CreatePrimeIdentityNotAllowed',
            'ShieldingKeyNotExist',
            'VerificationRequestTooEarly',
            'VerificationRequestTooLate',
            'VerifySubstrateSignatureFailed',
            'RecoverSubstratePubkeyFailed',
            'VerifyEvmSignatureFailed',
            'CreationRequestBlockZero',
            'ChallengeCodeNotExist',
            'WrongSignatureType',
            'WrongIdentityType',
            'RecoverEvmAddressFailed',
            'UnexpectedMessage',
        ],
    },
    /**
     * Lookup566: pallet_sudo::pallet::Error<T>
     **/
    PalletSudoError: {
        _enum: ['RequireSudo'],
    },
    /**
     * Lookup568: sp_runtime::MultiSignature
     **/
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: 'SpCoreEd25519Signature',
            Sr25519: 'SpCoreSr25519Signature',
            Ecdsa: 'SpCoreEcdsaSignature',
        },
    },
    /**
     * Lookup569: sp_core::ed25519::Signature
     **/
    SpCoreEd25519Signature: '[u8;64]',
    /**
     * Lookup571: sp_core::sr25519::Signature
     **/
    SpCoreSr25519Signature: '[u8;64]',
    /**
     * Lookup572: sp_core::ecdsa::Signature
     **/
    SpCoreEcdsaSignature: '[u8;65]',
    /**
     * Lookup575: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
     **/
    FrameSystemExtensionsCheckNonZeroSender: 'Null',
    /**
     * Lookup576: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
     **/
    FrameSystemExtensionsCheckSpecVersion: 'Null',
    /**
     * Lookup577: frame_system::extensions::check_tx_version::CheckTxVersion<T>
     **/
    FrameSystemExtensionsCheckTxVersion: 'Null',
    /**
     * Lookup578: frame_system::extensions::check_genesis::CheckGenesis<T>
     **/
    FrameSystemExtensionsCheckGenesis: 'Null',
    /**
     * Lookup581: frame_system::extensions::check_nonce::CheckNonce<T>
     **/
    FrameSystemExtensionsCheckNonce: 'Compact<u32>',
    /**
     * Lookup582: frame_system::extensions::check_weight::CheckWeight<T>
     **/
    FrameSystemExtensionsCheckWeight: 'Null',
    /**
     * Lookup583: pallet_transaction_payment::ChargeTransactionPayment<T>
     **/
    PalletTransactionPaymentChargeTransactionPayment: 'Compact<u128>',
};
