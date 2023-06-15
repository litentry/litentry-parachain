declare const _default: {
    /**
     * Lookup3: frame_system::AccountInfo<Index, pallet_balances::AccountData<Balance>>
     **/
    FrameSystemAccountInfo: {
        nonce: string;
        consumers: string;
        providers: string;
        sufficients: string;
        data: string;
    };
    /**
     * Lookup5: pallet_balances::AccountData<Balance>
     **/
    PalletBalancesAccountData: {
        free: string;
        reserved: string;
        miscFrozen: string;
        feeFrozen: string;
    };
    /**
     * Lookup7: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
     **/
    FrameSupportDispatchPerDispatchClassWeight: {
        normal: string;
        operational: string;
        mandatory: string;
    };
    /**
     * Lookup8: sp_weights::weight_v2::Weight
     **/
    SpWeightsWeightV2Weight: {
        refTime: string;
        proofSize: string;
    };
    /**
     * Lookup13: sp_runtime::generic::digest::Digest
     **/
    SpRuntimeDigest: {
        logs: string;
    };
    /**
     * Lookup15: sp_runtime::generic::digest::DigestItem
     **/
    SpRuntimeDigestDigestItem: {
        _enum: {
            Other: string;
            __Unused1: string;
            __Unused2: string;
            __Unused3: string;
            Consensus: string;
            Seal: string;
            PreRuntime: string;
            __Unused7: string;
            RuntimeEnvironmentUpdated: string;
        };
    };
    /**
     * Lookup18: frame_system::EventRecord<rococo_parachain_runtime::RuntimeEvent, primitive_types::H256>
     **/
    FrameSystemEventRecord: {
        phase: string;
        event: string;
        topics: string;
    };
    /**
     * Lookup20: frame_system::pallet::Event<T>
     **/
    FrameSystemEvent: {
        _enum: {
            ExtrinsicSuccess: {
                dispatchInfo: string;
            };
            ExtrinsicFailed: {
                dispatchError: string;
                dispatchInfo: string;
            };
            CodeUpdated: string;
            NewAccount: {
                account: string;
            };
            KilledAccount: {
                account: string;
            };
            Remarked: {
                _alias: {
                    hash_: string;
                };
                sender: string;
                hash_: string;
            };
        };
    };
    /**
     * Lookup21: frame_support::dispatch::DispatchInfo
     **/
    FrameSupportDispatchDispatchInfo: {
        weight: string;
        class: string;
        paysFee: string;
    };
    /**
     * Lookup22: frame_support::dispatch::DispatchClass
     **/
    FrameSupportDispatchDispatchClass: {
        _enum: string[];
    };
    /**
     * Lookup23: frame_support::dispatch::Pays
     **/
    FrameSupportDispatchPays: {
        _enum: string[];
    };
    /**
     * Lookup24: sp_runtime::DispatchError
     **/
    SpRuntimeDispatchError: {
        _enum: {
            Other: string;
            CannotLookup: string;
            BadOrigin: string;
            Module: string;
            ConsumerRemaining: string;
            NoProviders: string;
            TooManyConsumers: string;
            Token: string;
            Arithmetic: string;
            Transactional: string;
            Exhausted: string;
            Corruption: string;
            Unavailable: string;
        };
    };
    /**
     * Lookup25: sp_runtime::ModuleError
     **/
    SpRuntimeModuleError: {
        index: string;
        error: string;
    };
    /**
     * Lookup26: sp_runtime::TokenError
     **/
    SpRuntimeTokenError: {
        _enum: string[];
    };
    /**
     * Lookup27: sp_arithmetic::ArithmeticError
     **/
    SpArithmeticArithmeticError: {
        _enum: string[];
    };
    /**
     * Lookup28: sp_runtime::TransactionalError
     **/
    SpRuntimeTransactionalError: {
        _enum: string[];
    };
    /**
     * Lookup29: pallet_scheduler::pallet::Event<T>
     **/
    PalletSchedulerEvent: {
        _enum: {
            Scheduled: {
                when: string;
                index: string;
            };
            Canceled: {
                when: string;
                index: string;
            };
            Dispatched: {
                task: string;
                id: string;
                result: string;
            };
            CallUnavailable: {
                task: string;
                id: string;
            };
            PeriodicFailed: {
                task: string;
                id: string;
            };
            PermanentlyOverweight: {
                task: string;
                id: string;
            };
        };
    };
    /**
     * Lookup34: pallet_utility::pallet::Event
     **/
    PalletUtilityEvent: {
        _enum: {
            BatchInterrupted: {
                index: string;
                error: string;
            };
            BatchCompleted: string;
            BatchCompletedWithErrors: string;
            ItemCompleted: string;
            ItemFailed: {
                error: string;
            };
            DispatchedAs: {
                result: string;
            };
        };
    };
    /**
     * Lookup35: pallet_multisig::pallet::Event<T>
     **/
    PalletMultisigEvent: {
        _enum: {
            NewMultisig: {
                approving: string;
                multisig: string;
                callHash: string;
            };
            MultisigApproval: {
                approving: string;
                timepoint: string;
                multisig: string;
                callHash: string;
            };
            MultisigExecuted: {
                approving: string;
                timepoint: string;
                multisig: string;
                callHash: string;
                result: string;
            };
            MultisigCancelled: {
                cancelling: string;
                timepoint: string;
                multisig: string;
                callHash: string;
            };
        };
    };
    /**
     * Lookup36: pallet_multisig::Timepoint<BlockNumber>
     **/
    PalletMultisigTimepoint: {
        height: string;
        index: string;
    };
    /**
     * Lookup37: pallet_proxy::pallet::Event<T>
     **/
    PalletProxyEvent: {
        _enum: {
            ProxyExecuted: {
                result: string;
            };
            PureCreated: {
                pure: string;
                who: string;
                proxyType: string;
                disambiguationIndex: string;
            };
            Announced: {
                real: string;
                proxy: string;
                callHash: string;
            };
            ProxyAdded: {
                delegator: string;
                delegatee: string;
                proxyType: string;
                delay: string;
            };
            ProxyRemoved: {
                delegator: string;
                delegatee: string;
                proxyType: string;
                delay: string;
            };
        };
    };
    /**
     * Lookup38: rococo_parachain_runtime::ProxyType
     **/
    RococoParachainRuntimeProxyType: {
        _enum: string[];
    };
    /**
     * Lookup40: pallet_preimage::pallet::Event<T>
     **/
    PalletPreimageEvent: {
        _enum: {
            Noted: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
            };
            Requested: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
            };
            Cleared: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
            };
        };
    };
    /**
     * Lookup41: pallet_balances::pallet::Event<T, I>
     **/
    PalletBalancesEvent: {
        _enum: {
            Endowed: {
                account: string;
                freeBalance: string;
            };
            DustLost: {
                account: string;
                amount: string;
            };
            Transfer: {
                from: string;
                to: string;
                amount: string;
            };
            BalanceSet: {
                who: string;
                free: string;
                reserved: string;
            };
            Reserved: {
                who: string;
                amount: string;
            };
            Unreserved: {
                who: string;
                amount: string;
            };
            ReserveRepatriated: {
                from: string;
                to: string;
                amount: string;
                destinationStatus: string;
            };
            Deposit: {
                who: string;
                amount: string;
            };
            Withdraw: {
                who: string;
                amount: string;
            };
            Slashed: {
                who: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup42: frame_support::traits::tokens::misc::BalanceStatus
     **/
    FrameSupportTokensMiscBalanceStatus: {
        _enum: string[];
    };
    /**
     * Lookup43: pallet_vesting::pallet::Event<T>
     **/
    PalletVestingEvent: {
        _enum: {
            VestingUpdated: {
                account: string;
                unvested: string;
            };
            VestingCompleted: {
                account: string;
            };
        };
    };
    /**
     * Lookup44: pallet_transaction_payment::pallet::Event<T>
     **/
    PalletTransactionPaymentEvent: {
        _enum: {
            TransactionFeePaid: {
                who: string;
                actualFee: string;
                tip: string;
            };
        };
    };
    /**
     * Lookup45: pallet_treasury::pallet::Event<T, I>
     **/
    PalletTreasuryEvent: {
        _enum: {
            Proposed: {
                proposalIndex: string;
            };
            Spending: {
                budgetRemaining: string;
            };
            Awarded: {
                proposalIndex: string;
                award: string;
                account: string;
            };
            Rejected: {
                proposalIndex: string;
                slashed: string;
            };
            Burnt: {
                burntFunds: string;
            };
            Rollover: {
                rolloverBalance: string;
            };
            Deposit: {
                value: string;
            };
            SpendApproved: {
                proposalIndex: string;
                amount: string;
                beneficiary: string;
            };
            UpdatedInactive: {
                reactivated: string;
                deactivated: string;
            };
        };
    };
    /**
     * Lookup46: pallet_democracy::pallet::Event<T>
     **/
    PalletDemocracyEvent: {
        _enum: {
            Proposed: {
                proposalIndex: string;
                deposit: string;
            };
            Tabled: {
                proposalIndex: string;
                deposit: string;
            };
            ExternalTabled: string;
            Started: {
                refIndex: string;
                threshold: string;
            };
            Passed: {
                refIndex: string;
            };
            NotPassed: {
                refIndex: string;
            };
            Cancelled: {
                refIndex: string;
            };
            Delegated: {
                who: string;
                target: string;
            };
            Undelegated: {
                account: string;
            };
            Vetoed: {
                who: string;
                proposalHash: string;
                until: string;
            };
            Blacklisted: {
                proposalHash: string;
            };
            Voted: {
                voter: string;
                refIndex: string;
                vote: string;
            };
            Seconded: {
                seconder: string;
                propIndex: string;
            };
            ProposalCanceled: {
                propIndex: string;
            };
            MetadataSet: {
                _alias: {
                    hash_: string;
                };
                owner: string;
                hash_: string;
            };
            MetadataCleared: {
                _alias: {
                    hash_: string;
                };
                owner: string;
                hash_: string;
            };
            MetadataTransferred: {
                _alias: {
                    hash_: string;
                };
                prevOwner: string;
                owner: string;
                hash_: string;
            };
        };
    };
    /**
     * Lookup47: pallet_democracy::vote_threshold::VoteThreshold
     **/
    PalletDemocracyVoteThreshold: {
        _enum: string[];
    };
    /**
     * Lookup48: pallet_democracy::vote::AccountVote<Balance>
     **/
    PalletDemocracyVoteAccountVote: {
        _enum: {
            Standard: {
                vote: string;
                balance: string;
            };
            Split: {
                aye: string;
                nay: string;
            };
        };
    };
    /**
     * Lookup50: pallet_democracy::types::MetadataOwner
     **/
    PalletDemocracyMetadataOwner: {
        _enum: {
            External: string;
            Proposal: string;
            Referendum: string;
        };
    };
    /**
     * Lookup51: pallet_collective::pallet::Event<T, I>
     **/
    PalletCollectiveEvent: {
        _enum: {
            Proposed: {
                account: string;
                proposalIndex: string;
                proposalHash: string;
                threshold: string;
            };
            Voted: {
                account: string;
                proposalHash: string;
                voted: string;
                yes: string;
                no: string;
            };
            Approved: {
                proposalHash: string;
            };
            Disapproved: {
                proposalHash: string;
            };
            Executed: {
                proposalHash: string;
                result: string;
            };
            MemberExecuted: {
                proposalHash: string;
                result: string;
            };
            Closed: {
                proposalHash: string;
                yes: string;
                no: string;
            };
        };
    };
    /**
     * Lookup53: pallet_membership::pallet::Event<T, I>
     **/
    PalletMembershipEvent: {
        _enum: string[];
    };
    /**
     * Lookup56: pallet_bounties::pallet::Event<T, I>
     **/
    PalletBountiesEvent: {
        _enum: {
            BountyProposed: {
                index: string;
            };
            BountyRejected: {
                index: string;
                bond: string;
            };
            BountyBecameActive: {
                index: string;
            };
            BountyAwarded: {
                index: string;
                beneficiary: string;
            };
            BountyClaimed: {
                index: string;
                payout: string;
                beneficiary: string;
            };
            BountyCanceled: {
                index: string;
            };
            BountyExtended: {
                index: string;
            };
        };
    };
    /**
     * Lookup57: pallet_tips::pallet::Event<T, I>
     **/
    PalletTipsEvent: {
        _enum: {
            NewTip: {
                tipHash: string;
            };
            TipClosing: {
                tipHash: string;
            };
            TipClosed: {
                tipHash: string;
                who: string;
                payout: string;
            };
            TipRetracted: {
                tipHash: string;
            };
            TipSlashed: {
                tipHash: string;
                finder: string;
                deposit: string;
            };
        };
    };
    /**
     * Lookup58: pallet_identity::pallet::Event<T>
     **/
    PalletIdentityEvent: {
        _enum: {
            IdentitySet: {
                who: string;
            };
            IdentityCleared: {
                who: string;
                deposit: string;
            };
            IdentityKilled: {
                who: string;
                deposit: string;
            };
            JudgementRequested: {
                who: string;
                registrarIndex: string;
            };
            JudgementUnrequested: {
                who: string;
                registrarIndex: string;
            };
            JudgementGiven: {
                target: string;
                registrarIndex: string;
            };
            RegistrarAdded: {
                registrarIndex: string;
            };
            SubIdentityAdded: {
                sub: string;
                main: string;
                deposit: string;
            };
            SubIdentityRemoved: {
                sub: string;
                main: string;
                deposit: string;
            };
            SubIdentityRevoked: {
                sub: string;
                main: string;
                deposit: string;
            };
        };
    };
    /**
     * Lookup59: cumulus_pallet_parachain_system::pallet::Event<T>
     **/
    CumulusPalletParachainSystemEvent: {
        _enum: {
            ValidationFunctionStored: string;
            ValidationFunctionApplied: {
                relayChainBlockNum: string;
            };
            ValidationFunctionDiscarded: string;
            UpgradeAuthorized: {
                codeHash: string;
            };
            DownwardMessagesReceived: {
                count: string;
            };
            DownwardMessagesProcessed: {
                weightUsed: string;
                dmqHead: string;
            };
            UpwardMessageSent: {
                messageHash: string;
            };
        };
    };
    /**
     * Lookup60: pallet_session::pallet::Event
     **/
    PalletSessionEvent: {
        _enum: {
            NewSession: {
                sessionIndex: string;
            };
        };
    };
    /**
     * Lookup61: pallet_parachain_staking::pallet::Event<T>
     **/
    PalletParachainStakingEvent: {
        _enum: {
            NewRound: {
                startingBlock: string;
                round: string;
                selectedCollatorsNumber: string;
                totalBalance: string;
            };
            JoinedCollatorCandidates: {
                account: string;
                amountLocked: string;
                newTotalAmtLocked: string;
            };
            CollatorChosen: {
                round: string;
                collatorAccount: string;
                totalExposedAmount: string;
            };
            CandidateBondLessRequested: {
                candidate: string;
                amountToDecrease: string;
                executeRound: string;
            };
            CandidateBondedMore: {
                candidate: string;
                amount: string;
                newTotalBond: string;
            };
            CandidateBondedLess: {
                candidate: string;
                amount: string;
                newBond: string;
            };
            CandidateWentOffline: {
                candidate: string;
            };
            CandidateBackOnline: {
                candidate: string;
            };
            CandidateScheduledExit: {
                exitAllowedRound: string;
                candidate: string;
                scheduledExit: string;
            };
            CancelledCandidateExit: {
                candidate: string;
            };
            CancelledCandidateBondLess: {
                candidate: string;
                amount: string;
                executeRound: string;
            };
            CandidateLeft: {
                exCandidate: string;
                unlockedAmount: string;
                newTotalAmtLocked: string;
            };
            DelegationDecreaseScheduled: {
                delegator: string;
                candidate: string;
                amountToDecrease: string;
                executeRound: string;
            };
            DelegationIncreased: {
                delegator: string;
                candidate: string;
                amount: string;
                inTop: string;
            };
            DelegationDecreased: {
                delegator: string;
                candidate: string;
                amount: string;
                inTop: string;
            };
            DelegatorExitScheduled: {
                round: string;
                delegator: string;
                scheduledExit: string;
            };
            DelegationRevocationScheduled: {
                round: string;
                delegator: string;
                candidate: string;
                scheduledExit: string;
            };
            DelegatorLeft: {
                delegator: string;
                unstakedAmount: string;
            };
            DelegationRevoked: {
                delegator: string;
                candidate: string;
                unstakedAmount: string;
            };
            DelegationKicked: {
                delegator: string;
                candidate: string;
                unstakedAmount: string;
            };
            DelegatorExitCancelled: {
                delegator: string;
            };
            CancelledDelegationRequest: {
                delegator: string;
                cancelledRequest: string;
                collator: string;
            };
            Delegation: {
                delegator: string;
                lockedAmount: string;
                candidate: string;
                delegatorPosition: string;
                autoCompound: string;
            };
            DelegatorLeftCandidate: {
                delegator: string;
                candidate: string;
                unstakedAmount: string;
                totalCandidateStaked: string;
            };
            Rewarded: {
                account: string;
                rewards: string;
            };
            ReservedForParachainBond: {
                account: string;
                value: string;
            };
            ParachainBondAccountSet: {
                _alias: {
                    new_: string;
                };
                old: string;
                new_: string;
            };
            ParachainBondReservePercentSet: {
                _alias: {
                    new_: string;
                };
                old: string;
                new_: string;
            };
            InflationSet: {
                annualMin: string;
                annualIdeal: string;
                annualMax: string;
                roundMin: string;
                roundIdeal: string;
                roundMax: string;
            };
            StakeExpectationsSet: {
                expectMin: string;
                expectIdeal: string;
                expectMax: string;
            };
            TotalSelectedSet: {
                _alias: {
                    new_: string;
                };
                old: string;
                new_: string;
            };
            CollatorCommissionSet: {
                _alias: {
                    new_: string;
                };
                old: string;
                new_: string;
            };
            BlocksPerRoundSet: {
                _alias: {
                    new_: string;
                };
                currentRound: string;
                firstBlock: string;
                old: string;
                new_: string;
                newPerRoundInflationMin: string;
                newPerRoundInflationIdeal: string;
                newPerRoundInflationMax: string;
            };
            CandidateWhiteListAdded: {
                candidate: string;
            };
            CandidateWhiteListRemoved: {
                candidate: string;
            };
            AutoCompoundSet: {
                candidate: string;
                delegator: string;
                value: string;
            };
            Compounded: {
                candidate: string;
                delegator: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup62: pallet_parachain_staking::delegation_requests::CancelledScheduledRequest<Balance>
     **/
    PalletParachainStakingDelegationRequestsCancelledScheduledRequest: {
        whenExecutable: string;
        action: string;
    };
    /**
     * Lookup63: pallet_parachain_staking::delegation_requests::DelegationAction<Balance>
     **/
    PalletParachainStakingDelegationRequestsDelegationAction: {
        _enum: {
            Revoke: string;
            Decrease: string;
        };
    };
    /**
     * Lookup64: pallet_parachain_staking::types::DelegatorAdded<B>
     **/
    PalletParachainStakingDelegatorAdded: {
        _enum: {
            AddedToTop: {
                newTotal: string;
            };
            AddedToBottom: string;
        };
    };
    /**
     * Lookup67: cumulus_pallet_xcmp_queue::pallet::Event<T>
     **/
    CumulusPalletXcmpQueueEvent: {
        _enum: {
            Success: {
                messageHash: string;
                weight: string;
            };
            Fail: {
                messageHash: string;
                error: string;
                weight: string;
            };
            BadVersion: {
                messageHash: string;
            };
            BadFormat: {
                messageHash: string;
            };
            XcmpMessageSent: {
                messageHash: string;
            };
            OverweightEnqueued: {
                sender: string;
                sentAt: string;
                index: string;
                required: string;
            };
            OverweightServiced: {
                index: string;
                used: string;
            };
        };
    };
    /**
     * Lookup68: xcm::v3::traits::Error
     **/
    XcmV3TraitsError: {
        _enum: {
            Overflow: string;
            Unimplemented: string;
            UntrustedReserveLocation: string;
            UntrustedTeleportLocation: string;
            LocationFull: string;
            LocationNotInvertible: string;
            BadOrigin: string;
            InvalidLocation: string;
            AssetNotFound: string;
            FailedToTransactAsset: string;
            NotWithdrawable: string;
            LocationCannotHold: string;
            ExceedsMaxMessageSize: string;
            DestinationUnsupported: string;
            Transport: string;
            Unroutable: string;
            UnknownClaim: string;
            FailedToDecode: string;
            MaxWeightInvalid: string;
            NotHoldingFees: string;
            TooExpensive: string;
            Trap: string;
            ExpectationFalse: string;
            PalletNotFound: string;
            NameMismatch: string;
            VersionIncompatible: string;
            HoldingWouldOverflow: string;
            ExportError: string;
            ReanchorFailed: string;
            NoDeal: string;
            FeesNotMet: string;
            LockError: string;
            NoPermission: string;
            Unanchored: string;
            NotDepositable: string;
            UnhandledXcmVersion: string;
            WeightLimitReached: string;
            Barrier: string;
            WeightNotComputable: string;
            ExceedsStackLimit: string;
        };
    };
    /**
     * Lookup70: pallet_xcm::pallet::Event<T>
     **/
    PalletXcmEvent: {
        _enum: {
            Attempted: string;
            Sent: string;
            UnexpectedResponse: string;
            ResponseReady: string;
            Notified: string;
            NotifyOverweight: string;
            NotifyDispatchError: string;
            NotifyDecodeFailed: string;
            InvalidResponder: string;
            InvalidResponderVersion: string;
            ResponseTaken: string;
            AssetsTrapped: string;
            VersionChangeNotified: string;
            SupportedVersionChanged: string;
            NotifyTargetSendFail: string;
            NotifyTargetMigrationFail: string;
            InvalidQuerierVersion: string;
            InvalidQuerier: string;
            VersionNotifyStarted: string;
            VersionNotifyRequested: string;
            VersionNotifyUnrequested: string;
            FeesPaid: string;
            AssetsClaimed: string;
        };
    };
    /**
     * Lookup71: xcm::v3::traits::Outcome
     **/
    XcmV3TraitsOutcome: {
        _enum: {
            Complete: string;
            Incomplete: string;
            Error: string;
        };
    };
    /**
     * Lookup72: xcm::v3::multilocation::MultiLocation
     **/
    XcmV3MultiLocation: {
        parents: string;
        interior: string;
    };
    /**
     * Lookup73: xcm::v3::junctions::Junctions
     **/
    XcmV3Junctions: {
        _enum: {
            Here: string;
            X1: string;
            X2: string;
            X3: string;
            X4: string;
            X5: string;
            X6: string;
            X7: string;
            X8: string;
        };
    };
    /**
     * Lookup74: xcm::v3::junction::Junction
     **/
    XcmV3Junction: {
        _enum: {
            Parachain: string;
            AccountId32: {
                network: string;
                id: string;
            };
            AccountIndex64: {
                network: string;
                index: string;
            };
            AccountKey20: {
                network: string;
                key: string;
            };
            PalletInstance: string;
            GeneralIndex: string;
            GeneralKey: {
                length: string;
                data: string;
            };
            OnlyChild: string;
            Plurality: {
                id: string;
                part: string;
            };
            GlobalConsensus: string;
        };
    };
    /**
     * Lookup77: xcm::v3::junction::NetworkId
     **/
    XcmV3JunctionNetworkId: {
        _enum: {
            ByGenesis: string;
            ByFork: {
                blockNumber: string;
                blockHash: string;
            };
            Polkadot: string;
            Kusama: string;
            Westend: string;
            Rococo: string;
            Wococo: string;
            Ethereum: {
                chainId: string;
            };
            BitcoinCore: string;
            BitcoinCash: string;
        };
    };
    /**
     * Lookup80: xcm::v3::junction::BodyId
     **/
    XcmV3JunctionBodyId: {
        _enum: {
            Unit: string;
            Moniker: string;
            Index: string;
            Executive: string;
            Technical: string;
            Legislative: string;
            Judicial: string;
            Defense: string;
            Administration: string;
            Treasury: string;
        };
    };
    /**
     * Lookup81: xcm::v3::junction::BodyPart
     **/
    XcmV3JunctionBodyPart: {
        _enum: {
            Voice: string;
            Members: {
                count: string;
            };
            Fraction: {
                nom: string;
                denom: string;
            };
            AtLeastProportion: {
                nom: string;
                denom: string;
            };
            MoreThanProportion: {
                nom: string;
                denom: string;
            };
        };
    };
    /**
     * Lookup82: xcm::v3::Xcm<Call>
     **/
    XcmV3Xcm: string;
    /**
     * Lookup84: xcm::v3::Instruction<Call>
     **/
    XcmV3Instruction: {
        _enum: {
            WithdrawAsset: string;
            ReserveAssetDeposited: string;
            ReceiveTeleportedAsset: string;
            QueryResponse: {
                queryId: string;
                response: string;
                maxWeight: string;
                querier: string;
            };
            TransferAsset: {
                assets: string;
                beneficiary: string;
            };
            TransferReserveAsset: {
                assets: string;
                dest: string;
                xcm: string;
            };
            Transact: {
                originKind: string;
                requireWeightAtMost: string;
                call: string;
            };
            HrmpNewChannelOpenRequest: {
                sender: string;
                maxMessageSize: string;
                maxCapacity: string;
            };
            HrmpChannelAccepted: {
                recipient: string;
            };
            HrmpChannelClosing: {
                initiator: string;
                sender: string;
                recipient: string;
            };
            ClearOrigin: string;
            DescendOrigin: string;
            ReportError: string;
            DepositAsset: {
                assets: string;
                beneficiary: string;
            };
            DepositReserveAsset: {
                assets: string;
                dest: string;
                xcm: string;
            };
            ExchangeAsset: {
                give: string;
                want: string;
                maximal: string;
            };
            InitiateReserveWithdraw: {
                assets: string;
                reserve: string;
                xcm: string;
            };
            InitiateTeleport: {
                assets: string;
                dest: string;
                xcm: string;
            };
            ReportHolding: {
                responseInfo: string;
                assets: string;
            };
            BuyExecution: {
                fees: string;
                weightLimit: string;
            };
            RefundSurplus: string;
            SetErrorHandler: string;
            SetAppendix: string;
            ClearError: string;
            ClaimAsset: {
                assets: string;
                ticket: string;
            };
            Trap: string;
            SubscribeVersion: {
                queryId: string;
                maxResponseWeight: string;
            };
            UnsubscribeVersion: string;
            BurnAsset: string;
            ExpectAsset: string;
            ExpectOrigin: string;
            ExpectError: string;
            ExpectTransactStatus: string;
            QueryPallet: {
                moduleName: string;
                responseInfo: string;
            };
            ExpectPallet: {
                index: string;
                name: string;
                moduleName: string;
                crateMajor: string;
                minCrateMinor: string;
            };
            ReportTransactStatus: string;
            ClearTransactStatus: string;
            UniversalOrigin: string;
            ExportMessage: {
                network: string;
                destination: string;
                xcm: string;
            };
            LockAsset: {
                asset: string;
                unlocker: string;
            };
            UnlockAsset: {
                asset: string;
                target: string;
            };
            NoteUnlockable: {
                asset: string;
                owner: string;
            };
            RequestUnlock: {
                asset: string;
                locker: string;
            };
            SetFeesMode: {
                jitWithdraw: string;
            };
            SetTopic: string;
            ClearTopic: string;
            AliasOrigin: string;
            UnpaidExecution: {
                weightLimit: string;
                checkOrigin: string;
            };
        };
    };
    /**
     * Lookup85: xcm::v3::multiasset::MultiAssets
     **/
    XcmV3MultiassetMultiAssets: string;
    /**
     * Lookup87: xcm::v3::multiasset::MultiAsset
     **/
    XcmV3MultiAsset: {
        id: string;
        fun: string;
    };
    /**
     * Lookup88: xcm::v3::multiasset::AssetId
     **/
    XcmV3MultiassetAssetId: {
        _enum: {
            Concrete: string;
            Abstract: string;
        };
    };
    /**
     * Lookup89: xcm::v3::multiasset::Fungibility
     **/
    XcmV3MultiassetFungibility: {
        _enum: {
            Fungible: string;
            NonFungible: string;
        };
    };
    /**
     * Lookup90: xcm::v3::multiasset::AssetInstance
     **/
    XcmV3MultiassetAssetInstance: {
        _enum: {
            Undefined: string;
            Index: string;
            Array4: string;
            Array8: string;
            Array16: string;
            Array32: string;
        };
    };
    /**
     * Lookup93: xcm::v3::Response
     **/
    XcmV3Response: {
        _enum: {
            Null: string;
            Assets: string;
            ExecutionResult: string;
            Version: string;
            PalletsInfo: string;
            DispatchResult: string;
        };
    };
    /**
     * Lookup97: xcm::v3::PalletInfo
     **/
    XcmV3PalletInfo: {
        index: string;
        name: string;
        moduleName: string;
        major: string;
        minor: string;
        patch: string;
    };
    /**
     * Lookup100: xcm::v3::MaybeErrorCode
     **/
    XcmV3MaybeErrorCode: {
        _enum: {
            Success: string;
            Error: string;
            TruncatedError: string;
        };
    };
    /**
     * Lookup103: xcm::v2::OriginKind
     **/
    XcmV2OriginKind: {
        _enum: string[];
    };
    /**
     * Lookup104: xcm::double_encoded::DoubleEncoded<T>
     **/
    XcmDoubleEncoded: {
        encoded: string;
    };
    /**
     * Lookup105: xcm::v3::QueryResponseInfo
     **/
    XcmV3QueryResponseInfo: {
        destination: string;
        queryId: string;
        maxWeight: string;
    };
    /**
     * Lookup106: xcm::v3::multiasset::MultiAssetFilter
     **/
    XcmV3MultiassetMultiAssetFilter: {
        _enum: {
            Definite: string;
            Wild: string;
        };
    };
    /**
     * Lookup107: xcm::v3::multiasset::WildMultiAsset
     **/
    XcmV3MultiassetWildMultiAsset: {
        _enum: {
            All: string;
            AllOf: {
                id: string;
                fun: string;
            };
            AllCounted: string;
            AllOfCounted: {
                id: string;
                fun: string;
                count: string;
            };
        };
    };
    /**
     * Lookup108: xcm::v3::multiasset::WildFungibility
     **/
    XcmV3MultiassetWildFungibility: {
        _enum: string[];
    };
    /**
     * Lookup109: xcm::v3::WeightLimit
     **/
    XcmV3WeightLimit: {
        _enum: {
            Unlimited: string;
            Limited: string;
        };
    };
    /**
     * Lookup110: xcm::VersionedMultiAssets
     **/
    XcmVersionedMultiAssets: {
        _enum: {
            __Unused0: string;
            V2: string;
            __Unused2: string;
            V3: string;
        };
    };
    /**
     * Lookup111: xcm::v2::multiasset::MultiAssets
     **/
    XcmV2MultiassetMultiAssets: string;
    /**
     * Lookup113: xcm::v2::multiasset::MultiAsset
     **/
    XcmV2MultiAsset: {
        id: string;
        fun: string;
    };
    /**
     * Lookup114: xcm::v2::multiasset::AssetId
     **/
    XcmV2MultiassetAssetId: {
        _enum: {
            Concrete: string;
            Abstract: string;
        };
    };
    /**
     * Lookup115: xcm::v2::multilocation::MultiLocation
     **/
    XcmV2MultiLocation: {
        parents: string;
        interior: string;
    };
    /**
     * Lookup116: xcm::v2::multilocation::Junctions
     **/
    XcmV2MultilocationJunctions: {
        _enum: {
            Here: string;
            X1: string;
            X2: string;
            X3: string;
            X4: string;
            X5: string;
            X6: string;
            X7: string;
            X8: string;
        };
    };
    /**
     * Lookup117: xcm::v2::junction::Junction
     **/
    XcmV2Junction: {
        _enum: {
            Parachain: string;
            AccountId32: {
                network: string;
                id: string;
            };
            AccountIndex64: {
                network: string;
                index: string;
            };
            AccountKey20: {
                network: string;
                key: string;
            };
            PalletInstance: string;
            GeneralIndex: string;
            GeneralKey: string;
            OnlyChild: string;
            Plurality: {
                id: string;
                part: string;
            };
        };
    };
    /**
     * Lookup118: xcm::v2::NetworkId
     **/
    XcmV2NetworkId: {
        _enum: {
            Any: string;
            Named: string;
            Polkadot: string;
            Kusama: string;
        };
    };
    /**
     * Lookup120: xcm::v2::BodyId
     **/
    XcmV2BodyId: {
        _enum: {
            Unit: string;
            Named: string;
            Index: string;
            Executive: string;
            Technical: string;
            Legislative: string;
            Judicial: string;
            Defense: string;
            Administration: string;
            Treasury: string;
        };
    };
    /**
     * Lookup121: xcm::v2::BodyPart
     **/
    XcmV2BodyPart: {
        _enum: {
            Voice: string;
            Members: {
                count: string;
            };
            Fraction: {
                nom: string;
                denom: string;
            };
            AtLeastProportion: {
                nom: string;
                denom: string;
            };
            MoreThanProportion: {
                nom: string;
                denom: string;
            };
        };
    };
    /**
     * Lookup122: xcm::v2::multiasset::Fungibility
     **/
    XcmV2MultiassetFungibility: {
        _enum: {
            Fungible: string;
            NonFungible: string;
        };
    };
    /**
     * Lookup123: xcm::v2::multiasset::AssetInstance
     **/
    XcmV2MultiassetAssetInstance: {
        _enum: {
            Undefined: string;
            Index: string;
            Array4: string;
            Array8: string;
            Array16: string;
            Array32: string;
            Blob: string;
        };
    };
    /**
     * Lookup124: xcm::VersionedMultiLocation
     **/
    XcmVersionedMultiLocation: {
        _enum: {
            __Unused0: string;
            V2: string;
            __Unused2: string;
            V3: string;
        };
    };
    /**
     * Lookup125: cumulus_pallet_xcm::pallet::Event<T>
     **/
    CumulusPalletXcmEvent: {
        _enum: {
            InvalidFormat: string;
            UnsupportedVersion: string;
            ExecutedDownward: string;
        };
    };
    /**
     * Lookup126: cumulus_pallet_dmp_queue::pallet::Event<T>
     **/
    CumulusPalletDmpQueueEvent: {
        _enum: {
            InvalidFormat: {
                messageId: string;
            };
            UnsupportedVersion: {
                messageId: string;
            };
            ExecutedDownward: {
                messageId: string;
                outcome: string;
            };
            WeightExhausted: {
                messageId: string;
                remainingWeight: string;
                requiredWeight: string;
            };
            OverweightEnqueued: {
                messageId: string;
                overweightIndex: string;
                requiredWeight: string;
            };
            OverweightServiced: {
                overweightIndex: string;
                weightUsed: string;
            };
            MaxMessagesExhausted: {
                messageId: string;
            };
        };
    };
    /**
     * Lookup127: orml_xtokens::module::Event<T>
     **/
    OrmlXtokensModuleEvent: {
        _enum: {
            TransferredMultiAssets: {
                sender: string;
                assets: string;
                fee: string;
                dest: string;
            };
        };
    };
    /**
     * Lookup128: orml_tokens::module::Event<T>
     **/
    OrmlTokensModuleEvent: {
        _enum: {
            Endowed: {
                currencyId: string;
                who: string;
                amount: string;
            };
            DustLost: {
                currencyId: string;
                who: string;
                amount: string;
            };
            Transfer: {
                currencyId: string;
                from: string;
                to: string;
                amount: string;
            };
            Reserved: {
                currencyId: string;
                who: string;
                amount: string;
            };
            Unreserved: {
                currencyId: string;
                who: string;
                amount: string;
            };
            ReserveRepatriated: {
                currencyId: string;
                from: string;
                to: string;
                amount: string;
                status: string;
            };
            BalanceSet: {
                currencyId: string;
                who: string;
                free: string;
                reserved: string;
            };
            TotalIssuanceSet: {
                currencyId: string;
                amount: string;
            };
            Withdrawn: {
                currencyId: string;
                who: string;
                amount: string;
            };
            Slashed: {
                currencyId: string;
                who: string;
                freeAmount: string;
                reservedAmount: string;
            };
            Deposited: {
                currencyId: string;
                who: string;
                amount: string;
            };
            LockSet: {
                lockId: string;
                currencyId: string;
                who: string;
                amount: string;
            };
            LockRemoved: {
                lockId: string;
                currencyId: string;
                who: string;
            };
            Locked: {
                currencyId: string;
                who: string;
                amount: string;
            };
            Unlocked: {
                currencyId: string;
                who: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup129: pallet_bridge::pallet::Event<T>
     **/
    PalletBridgeEvent: {
        _enum: {
            RelayerThresholdChanged: string;
            ChainWhitelisted: string;
            RelayerAdded: string;
            RelayerRemoved: string;
            FungibleTransfer: string;
            NonFungibleTransfer: string;
            GenericTransfer: string;
            VoteFor: string;
            VoteAgainst: string;
            ProposalApproved: string;
            ProposalRejected: string;
            ProposalSucceeded: string;
            ProposalFailed: string;
            FeeUpdated: {
                destId: string;
                fee: string;
            };
        };
    };
    /**
     * Lookup130: pallet_bridge_transfer::pallet::Event<T>
     **/
    PalletBridgeTransferEvent: {
        _enum: {
            MaximumIssuanceChanged: {
                oldValue: string;
            };
            NativeTokenMinted: {
                to: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup131: pallet_drop3::pallet::Event<T>
     **/
    PalletDrop3Event: {
        _enum: {
            AdminChanged: {
                oldAdmin: string;
            };
            BalanceSlashed: {
                who: string;
                amount: string;
            };
            RewardPoolApproved: {
                id: string;
            };
            RewardPoolRejected: {
                id: string;
            };
            RewardPoolStarted: {
                id: string;
            };
            RewardPoolStopped: {
                id: string;
            };
            RewardPoolRemoved: {
                id: string;
                name: string;
                owner: string;
            };
            RewardPoolProposed: {
                id: string;
                name: string;
                owner: string;
            };
            RewardSent: {
                to: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup133: pallet_extrinsic_filter::pallet::Event<T>
     **/
    PalletExtrinsicFilterEvent: {
        _enum: {
            ModeSet: {
                newMode: string;
            };
            ExtrinsicsBlocked: {
                palletNameBytes: string;
                functionNameBytes: string;
            };
            ExtrinsicsUnblocked: {
                palletNameBytes: string;
                functionNameBytes: string;
            };
        };
    };
    /**
     * Lookup134: pallet_extrinsic_filter::OperationalMode
     **/
    PalletExtrinsicFilterOperationalMode: {
        _enum: string[];
    };
    /**
     * Lookup136: pallet_identity_management::pallet::Event<T>
     **/
    PalletIdentityManagementEvent: {
        _enum: {
            DelegateeAdded: {
                account: string;
            };
            DelegateeRemoved: {
                account: string;
            };
            CreateIdentityRequested: {
                shard: string;
            };
            RemoveIdentityRequested: {
                shard: string;
            };
            VerifyIdentityRequested: {
                shard: string;
            };
            SetUserShieldingKeyRequested: {
                shard: string;
            };
            UserShieldingKeySet: {
                account: string;
                idGraph: string;
                reqExtHash: string;
            };
            IdentityCreated: {
                account: string;
                identity: string;
                code: string;
                reqExtHash: string;
            };
            IdentityRemoved: {
                account: string;
                identity: string;
                reqExtHash: string;
            };
            IdentityVerified: {
                account: string;
                identity: string;
                idGraph: string;
                reqExtHash: string;
            };
            SetUserShieldingKeyFailed: {
                account: string;
                detail: string;
                reqExtHash: string;
            };
            CreateIdentityFailed: {
                account: string;
                detail: string;
                reqExtHash: string;
            };
            RemoveIdentityFailed: {
                account: string;
                detail: string;
                reqExtHash: string;
            };
            VerifyIdentityFailed: {
                account: string;
                detail: string;
                reqExtHash: string;
            };
            ImportScheduledEnclaveFailed: string;
            UnclassifiedError: {
                account: string;
                detail: string;
                reqExtHash: string;
            };
        };
    };
    /**
     * Lookup137: core_primitives::key::AesOutput
     **/
    CorePrimitivesKeyAesOutput: {
        ciphertext: string;
        aad: string;
        nonce: string;
    };
    /**
     * Lookup139: core_primitives::error::ErrorDetail
     **/
    CorePrimitivesErrorErrorDetail: {
        _enum: {
            ImportError: string;
            StfError: string;
            SendStfRequestFailed: string;
            ChallengeCodeNotFound: string;
            UserShieldingKeyNotFound: string;
            ParseError: string;
            DataProviderError: string;
            InvalidIdentity: string;
            WrongWeb2Handle: string;
            UnexpectedMessage: string;
            WrongSignatureType: string;
            VerifySubstrateSignatureFailed: string;
            VerifyEvmSignatureFailed: string;
            RecoverEvmAddressFailed: string;
        };
    };
    /**
     * Lookup141: pallet_asset_manager::pallet::Event<T>
     **/
    PalletAssetManagerEvent: {
        _enum: {
            ForeignAssetMetadataUpdated: {
                assetId: string;
                metadata: string;
            };
            ForeignAssetTrackerUpdated: {
                oldAssetTracker: string;
                newAssetTracker: string;
            };
            ForeignAssetTypeRegistered: {
                assetId: string;
                assetType: string;
            };
            ForeignAssetTypeRemoved: {
                assetId: string;
                removedAssetType: string;
                defaultAssetType: string;
            };
            UnitsPerSecondChanged: {
                assetId: string;
                unitsPerSecond: string;
            };
        };
    };
    /**
     * Lookup142: pallet_asset_manager::pallet::AssetMetadata<Balance>
     **/
    PalletAssetManagerAssetMetadata: {
        name: string;
        symbol: string;
        decimals: string;
        minimalBalance: string;
        isFrozen: string;
    };
    /**
     * Lookup143: runtime_common::xcm_impl::CurrencyId<rococo_parachain_runtime::Runtime>
     **/
    RuntimeCommonXcmImplCurrencyId: {
        _enum: {
            SelfReserve: string;
            ParachainReserve: string;
        };
    };
    /**
     * Lookup144: rococo_parachain_runtime::Runtime
     **/
    RococoParachainRuntimeRuntime: string;
    /**
     * Lookup145: pallet_vc_management::pallet::Event<T>
     **/
    PalletVcManagementEvent: {
        _enum: {
            VCRequested: {
                account: string;
                shard: string;
                assertion: string;
            };
            VCDisabled: {
                account: string;
                index: string;
            };
            VCRevoked: {
                account: string;
                index: string;
            };
            VCIssued: {
                account: string;
                assertion: string;
                index: string;
                vc: string;
                reqExtHash: string;
            };
            AdminChanged: {
                oldAdmin: string;
                newAdmin: string;
            };
            SchemaIssued: {
                account: string;
                shard: string;
                index: string;
            };
            SchemaDisabled: {
                account: string;
                shard: string;
                index: string;
            };
            SchemaActivated: {
                account: string;
                shard: string;
                index: string;
            };
            SchemaRevoked: {
                account: string;
                shard: string;
                index: string;
            };
            RequestVCFailed: {
                account: string;
                assertion: string;
                detail: string;
                reqExtHash: string;
            };
            UnclassifiedError: {
                account: string;
                detail: string;
                reqExtHash: string;
            };
            VCRegistryItemAdded: {
                account: string;
                assertion: string;
                index: string;
            };
            VCRegistryItemRemoved: {
                index: string;
            };
            VCRegistryCleared: string;
        };
    };
    /**
     * Lookup146: core_primitives::assertion::Assertion
     **/
    CorePrimitivesAssertion: {
        _enum: {
            A1: string;
            A2: string;
            A3: string;
            A4: string;
            A5: string;
            A6: string;
            A7: string;
            A8: string;
            A9: string;
            A10: string;
            A11: string;
            A13: string;
        };
    };
    /**
     * Lookup149: core_primitives::assertion::IndexingNetwork
     **/
    CorePrimitivesAssertionIndexingNetwork: {
        _enum: string[];
    };
    /**
     * Lookup151: pallet_group::pallet::Event<T, I>
     **/
    PalletGroupEvent: {
        _enum: {
            GroupMemberAdded: string;
            GroupMemberRemoved: string;
        };
    };
    /**
     * Lookup153: pallet_teerex::pallet::Event<T>
     **/
    PalletTeerexEvent: {
        _enum: {
            AdminChanged: {
                oldAdmin: string;
            };
            AddedEnclave: string;
            RemovedEnclave: string;
            Forwarded: string;
            ShieldFunds: string;
            UnshieldedFunds: string;
            ProcessedParentchainBlock: string;
            SetHeartbeatTimeout: string;
            UpdatedScheduledEnclave: string;
            RemovedScheduledEnclave: string;
            PublishedHash: {
                _alias: {
                    hash_: string;
                };
                mrEnclave: string;
                hash_: string;
                data: string;
            };
        };
    };
    /**
     * Lookup154: pallet_sidechain::pallet::Event<T>
     **/
    PalletSidechainEvent: {
        _enum: {
            ProposedSidechainBlock: string;
            FinalizedSidechainBlock: string;
        };
    };
    /**
     * Lookup155: pallet_teeracle::pallet::Event<T>
     **/
    PalletTeeracleEvent: {
        _enum: {
            ExchangeRateUpdated: string;
            ExchangeRateDeleted: string;
            OracleUpdated: string;
            AddedToWhitelist: string;
            RemovedFromWhitelist: string;
        };
    };
    /**
     * Lookup157: substrate_fixed::FixedU64<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>>
     **/
    SubstrateFixedFixedU64: {
        bits: string;
    };
    /**
     * Lookup162: typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>
     **/
    TypenumUIntUInt: {
        msb: string;
        lsb: string;
    };
    /**
     * Lookup163: typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>
     **/
    TypenumUIntUTerm: {
        msb: string;
        lsb: string;
    };
    /**
     * Lookup164: typenum::uint::UTerm
     **/
    TypenumUintUTerm: string;
    /**
     * Lookup165: typenum::bit::B1
     **/
    TypenumBitB1: string;
    /**
     * Lookup166: typenum::bit::B0
     **/
    TypenumBitB0: string;
    /**
     * Lookup167: pallet_identity_management_mock::pallet::Event<T>
     **/
    PalletIdentityManagementMockEvent: {
        _enum: {
            DelegateeAdded: {
                account: string;
            };
            DelegateeRemoved: {
                account: string;
            };
            CreateIdentityRequested: {
                shard: string;
            };
            RemoveIdentityRequested: {
                shard: string;
            };
            VerifyIdentityRequested: {
                shard: string;
            };
            SetUserShieldingKeyRequested: {
                shard: string;
            };
            UserShieldingKeySetPlain: {
                account: string;
            };
            UserShieldingKeySet: {
                account: string;
            };
            IdentityCreatedPlain: {
                account: string;
                identity: string;
                code: string;
                idGraph: string;
            };
            IdentityCreated: {
                account: string;
                identity: string;
                code: string;
                idGraph: string;
            };
            IdentityRemovedPlain: {
                account: string;
                identity: string;
                idGraph: string;
            };
            IdentityRemoved: {
                account: string;
                identity: string;
                idGraph: string;
            };
            IdentityVerifiedPlain: {
                account: string;
                identity: string;
                idGraph: string;
            };
            IdentityVerified: {
                account: string;
                identity: string;
                idGraph: string;
            };
            SomeError: {
                func: string;
                error: string;
            };
        };
    };
    /**
     * Lookup168: mock_tee_primitives::identity::Identity
     **/
    MockTeePrimitivesIdentity: {
        _enum: {
            Substrate: {
                network: string;
                address: string;
            };
            Evm: {
                network: string;
                address: string;
            };
            Web2: {
                network: string;
                address: string;
            };
        };
    };
    /**
     * Lookup169: mock_tee_primitives::identity::SubstrateNetwork
     **/
    MockTeePrimitivesIdentitySubstrateNetwork: {
        _enum: string[];
    };
    /**
     * Lookup170: mock_tee_primitives::identity::Address32
     **/
    MockTeePrimitivesIdentityAddress32: string;
    /**
     * Lookup171: mock_tee_primitives::identity::EvmNetwork
     **/
    MockTeePrimitivesIdentityEvmNetwork: {
        _enum: string[];
    };
    /**
     * Lookup172: mock_tee_primitives::identity::Address20
     **/
    MockTeePrimitivesIdentityAddress20: string;
    /**
     * Lookup173: mock_tee_primitives::identity::Web2Network
     **/
    MockTeePrimitivesIdentityWeb2Network: {
        _enum: string[];
    };
    /**
     * Lookup176: pallet_identity_management_mock::identity_context::IdentityContext<T>
     **/
    PalletIdentityManagementMockIdentityContext: {
        metadata: string;
        creationRequestBlock: string;
        verificationRequestBlock: string;
        isVerified: string;
    };
    /**
     * Lookup180: pallet_sudo::pallet::Event<T>
     **/
    PalletSudoEvent: {
        _enum: {
            Sudid: {
                sudoResult: string;
            };
            KeyChanged: {
                oldSudoer: string;
            };
            SudoAsDone: {
                sudoResult: string;
            };
        };
    };
    /**
     * Lookup181: frame_system::Phase
     **/
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: string;
            Finalization: string;
            Initialization: string;
        };
    };
    /**
     * Lookup184: frame_system::LastRuntimeUpgradeInfo
     **/
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: string;
        specName: string;
    };
    /**
     * Lookup186: frame_system::pallet::Call<T>
     **/
    FrameSystemCall: {
        _enum: {
            remark: {
                remark: string;
            };
            set_heap_pages: {
                pages: string;
            };
            set_code: {
                code: string;
            };
            set_code_without_checks: {
                code: string;
            };
            set_storage: {
                items: string;
            };
            kill_storage: {
                _alias: {
                    keys_: string;
                };
                keys_: string;
            };
            kill_prefix: {
                prefix: string;
                subkeys: string;
            };
            remark_with_event: {
                remark: string;
            };
        };
    };
    /**
     * Lookup190: frame_system::limits::BlockWeights
     **/
    FrameSystemLimitsBlockWeights: {
        baseBlock: string;
        maxBlock: string;
        perClass: string;
    };
    /**
     * Lookup191: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
     **/
    FrameSupportDispatchPerDispatchClassWeightsPerClass: {
        normal: string;
        operational: string;
        mandatory: string;
    };
    /**
     * Lookup192: frame_system::limits::WeightsPerClass
     **/
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: string;
        maxExtrinsic: string;
        maxTotal: string;
        reserved: string;
    };
    /**
     * Lookup194: frame_system::limits::BlockLength
     **/
    FrameSystemLimitsBlockLength: {
        max: string;
    };
    /**
     * Lookup195: frame_support::dispatch::PerDispatchClass<T>
     **/
    FrameSupportDispatchPerDispatchClassU32: {
        normal: string;
        operational: string;
        mandatory: string;
    };
    /**
     * Lookup196: sp_weights::RuntimeDbWeight
     **/
    SpWeightsRuntimeDbWeight: {
        read: string;
        write: string;
    };
    /**
     * Lookup197: sp_version::RuntimeVersion
     **/
    SpVersionRuntimeVersion: {
        specName: string;
        implName: string;
        authoringVersion: string;
        specVersion: string;
        implVersion: string;
        apis: string;
        transactionVersion: string;
        stateVersion: string;
    };
    /**
     * Lookup201: frame_system::pallet::Error<T>
     **/
    FrameSystemError: {
        _enum: string[];
    };
    /**
     * Lookup202: pallet_timestamp::pallet::Call<T>
     **/
    PalletTimestampCall: {
        _enum: {
            set: {
                now: string;
            };
        };
    };
    /**
     * Lookup205: pallet_scheduler::Scheduled<Name, frame_support::traits::preimages::Bounded<rococo_parachain_runtime::RuntimeCall>, BlockNumber, rococo_parachain_runtime::OriginCaller, sp_core::crypto::AccountId32>
     **/
    PalletSchedulerScheduled: {
        maybeId: string;
        priority: string;
        call: string;
        maybePeriodic: string;
        origin: string;
    };
    /**
     * Lookup206: frame_support::traits::preimages::Bounded<rococo_parachain_runtime::RuntimeCall>
     **/
    FrameSupportPreimagesBounded: {
        _enum: {
            Legacy: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
            };
            Inline: string;
            Lookup: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
                len: string;
            };
        };
    };
    /**
     * Lookup208: pallet_scheduler::pallet::Call<T>
     **/
    PalletSchedulerCall: {
        _enum: {
            schedule: {
                when: string;
                maybePeriodic: string;
                priority: string;
                call: string;
            };
            cancel: {
                when: string;
                index: string;
            };
            schedule_named: {
                id: string;
                when: string;
                maybePeriodic: string;
                priority: string;
                call: string;
            };
            cancel_named: {
                id: string;
            };
            schedule_after: {
                after: string;
                maybePeriodic: string;
                priority: string;
                call: string;
            };
            schedule_named_after: {
                id: string;
                after: string;
                maybePeriodic: string;
                priority: string;
                call: string;
            };
        };
    };
    /**
     * Lookup210: pallet_utility::pallet::Call<T>
     **/
    PalletUtilityCall: {
        _enum: {
            batch: {
                calls: string;
            };
            as_derivative: {
                index: string;
                call: string;
            };
            batch_all: {
                calls: string;
            };
            dispatch_as: {
                asOrigin: string;
                call: string;
            };
            force_batch: {
                calls: string;
            };
            with_weight: {
                call: string;
                weight: string;
            };
        };
    };
    /**
     * Lookup212: rococo_parachain_runtime::OriginCaller
     **/
    RococoParachainRuntimeOriginCaller: {
        _enum: {
            system: string;
            __Unused1: string;
            __Unused2: string;
            __Unused3: string;
            __Unused4: string;
            Void: string;
            __Unused6: string;
            __Unused7: string;
            __Unused8: string;
            __Unused9: string;
            __Unused10: string;
            __Unused11: string;
            __Unused12: string;
            __Unused13: string;
            __Unused14: string;
            __Unused15: string;
            __Unused16: string;
            __Unused17: string;
            __Unused18: string;
            __Unused19: string;
            __Unused20: string;
            __Unused21: string;
            Council: string;
            __Unused23: string;
            TechnicalCommittee: string;
            __Unused25: string;
            __Unused26: string;
            __Unused27: string;
            __Unused28: string;
            __Unused29: string;
            __Unused30: string;
            __Unused31: string;
            __Unused32: string;
            __Unused33: string;
            __Unused34: string;
            __Unused35: string;
            __Unused36: string;
            __Unused37: string;
            __Unused38: string;
            __Unused39: string;
            __Unused40: string;
            __Unused41: string;
            __Unused42: string;
            __Unused43: string;
            __Unused44: string;
            __Unused45: string;
            __Unused46: string;
            __Unused47: string;
            __Unused48: string;
            __Unused49: string;
            __Unused50: string;
            PolkadotXcm: string;
            CumulusXcm: string;
        };
    };
    /**
     * Lookup213: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
     **/
    FrameSupportDispatchRawOrigin: {
        _enum: {
            Root: string;
            Signed: string;
            None: string;
        };
    };
    /**
     * Lookup214: pallet_collective::RawOrigin<sp_core::crypto::AccountId32, I>
     **/
    PalletCollectiveRawOrigin: {
        _enum: {
            Members: string;
            Member: string;
            _Phantom: string;
        };
    };
    /**
     * Lookup216: pallet_xcm::pallet::Origin
     **/
    PalletXcmOrigin: {
        _enum: {
            Xcm: string;
            Response: string;
        };
    };
    /**
     * Lookup217: cumulus_pallet_xcm::pallet::Origin
     **/
    CumulusPalletXcmOrigin: {
        _enum: {
            Relay: string;
            SiblingParachain: string;
        };
    };
    /**
     * Lookup218: sp_core::Void
     **/
    SpCoreVoid: string;
    /**
     * Lookup219: pallet_multisig::pallet::Call<T>
     **/
    PalletMultisigCall: {
        _enum: {
            as_multi_threshold_1: {
                otherSignatories: string;
                call: string;
            };
            as_multi: {
                threshold: string;
                otherSignatories: string;
                maybeTimepoint: string;
                call: string;
                maxWeight: string;
            };
            approve_as_multi: {
                threshold: string;
                otherSignatories: string;
                maybeTimepoint: string;
                callHash: string;
                maxWeight: string;
            };
            cancel_as_multi: {
                threshold: string;
                otherSignatories: string;
                timepoint: string;
                callHash: string;
            };
        };
    };
    /**
     * Lookup222: pallet_proxy::pallet::Call<T>
     **/
    PalletProxyCall: {
        _enum: {
            proxy: {
                real: string;
                forceProxyType: string;
                call: string;
            };
            add_proxy: {
                delegate: string;
                proxyType: string;
                delay: string;
            };
            remove_proxy: {
                delegate: string;
                proxyType: string;
                delay: string;
            };
            remove_proxies: string;
            create_pure: {
                proxyType: string;
                delay: string;
                index: string;
            };
            kill_pure: {
                spawner: string;
                proxyType: string;
                index: string;
                height: string;
                extIndex: string;
            };
            announce: {
                real: string;
                callHash: string;
            };
            remove_announcement: {
                real: string;
                callHash: string;
            };
            reject_announcement: {
                delegate: string;
                callHash: string;
            };
            proxy_announced: {
                delegate: string;
                real: string;
                forceProxyType: string;
                call: string;
            };
        };
    };
    /**
     * Lookup226: pallet_preimage::pallet::Call<T>
     **/
    PalletPreimageCall: {
        _enum: {
            note_preimage: {
                bytes: string;
            };
            unnote_preimage: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
            };
            request_preimage: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
            };
            unrequest_preimage: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
            };
        };
    };
    /**
     * Lookup227: pallet_balances::pallet::Call<T, I>
     **/
    PalletBalancesCall: {
        _enum: {
            transfer: {
                dest: string;
                value: string;
            };
            set_balance: {
                who: string;
                newFree: string;
                newReserved: string;
            };
            force_transfer: {
                source: string;
                dest: string;
                value: string;
            };
            transfer_keep_alive: {
                dest: string;
                value: string;
            };
            transfer_all: {
                dest: string;
                keepAlive: string;
            };
            force_unreserve: {
                who: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup228: pallet_vesting::pallet::Call<T>
     **/
    PalletVestingCall: {
        _enum: {
            vest: string;
            vest_other: {
                target: string;
            };
            vested_transfer: {
                target: string;
                schedule: string;
            };
            force_vested_transfer: {
                source: string;
                target: string;
                schedule: string;
            };
            merge_schedules: {
                schedule1Index: string;
                schedule2Index: string;
            };
        };
    };
    /**
     * Lookup229: pallet_vesting::vesting_info::VestingInfo<Balance, BlockNumber>
     **/
    PalletVestingVestingInfo: {
        locked: string;
        perBlock: string;
        startingBlock: string;
    };
    /**
     * Lookup230: pallet_treasury::pallet::Call<T, I>
     **/
    PalletTreasuryCall: {
        _enum: {
            propose_spend: {
                value: string;
                beneficiary: string;
            };
            reject_proposal: {
                proposalId: string;
            };
            approve_proposal: {
                proposalId: string;
            };
            spend: {
                amount: string;
                beneficiary: string;
            };
            remove_approval: {
                proposalId: string;
            };
        };
    };
    /**
     * Lookup231: pallet_democracy::pallet::Call<T>
     **/
    PalletDemocracyCall: {
        _enum: {
            propose: {
                proposal: string;
                value: string;
            };
            second: {
                proposal: string;
            };
            vote: {
                refIndex: string;
                vote: string;
            };
            emergency_cancel: {
                refIndex: string;
            };
            external_propose: {
                proposal: string;
            };
            external_propose_majority: {
                proposal: string;
            };
            external_propose_default: {
                proposal: string;
            };
            fast_track: {
                proposalHash: string;
                votingPeriod: string;
                delay: string;
            };
            veto_external: {
                proposalHash: string;
            };
            cancel_referendum: {
                refIndex: string;
            };
            delegate: {
                to: string;
                conviction: string;
                balance: string;
            };
            undelegate: string;
            clear_public_proposals: string;
            unlock: {
                target: string;
            };
            remove_vote: {
                index: string;
            };
            remove_other_vote: {
                target: string;
                index: string;
            };
            blacklist: {
                proposalHash: string;
                maybeRefIndex: string;
            };
            cancel_proposal: {
                propIndex: string;
            };
            set_metadata: {
                owner: string;
                maybeHash: string;
            };
        };
    };
    /**
     * Lookup232: pallet_democracy::conviction::Conviction
     **/
    PalletDemocracyConviction: {
        _enum: string[];
    };
    /**
     * Lookup234: pallet_collective::pallet::Call<T, I>
     **/
    PalletCollectiveCall: {
        _enum: {
            set_members: {
                newMembers: string;
                prime: string;
                oldCount: string;
            };
            execute: {
                proposal: string;
                lengthBound: string;
            };
            propose: {
                threshold: string;
                proposal: string;
                lengthBound: string;
            };
            vote: {
                proposal: string;
                index: string;
                approve: string;
            };
            close_old_weight: {
                proposalHash: string;
                index: string;
                proposalWeightBound: string;
                lengthBound: string;
            };
            disapprove_proposal: {
                proposalHash: string;
            };
            close: {
                proposalHash: string;
                index: string;
                proposalWeightBound: string;
                lengthBound: string;
            };
        };
    };
    /**
     * Lookup237: pallet_membership::pallet::Call<T, I>
     **/
    PalletMembershipCall: {
        _enum: {
            add_member: {
                who: string;
            };
            remove_member: {
                who: string;
            };
            swap_member: {
                remove: string;
                add: string;
            };
            reset_members: {
                members: string;
            };
            change_key: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_prime: {
                who: string;
            };
            clear_prime: string;
        };
    };
    /**
     * Lookup240: pallet_bounties::pallet::Call<T, I>
     **/
    PalletBountiesCall: {
        _enum: {
            propose_bounty: {
                value: string;
                description: string;
            };
            approve_bounty: {
                bountyId: string;
            };
            propose_curator: {
                bountyId: string;
                curator: string;
                fee: string;
            };
            unassign_curator: {
                bountyId: string;
            };
            accept_curator: {
                bountyId: string;
            };
            award_bounty: {
                bountyId: string;
                beneficiary: string;
            };
            claim_bounty: {
                bountyId: string;
            };
            close_bounty: {
                bountyId: string;
            };
            extend_bounty_expiry: {
                bountyId: string;
                remark: string;
            };
        };
    };
    /**
     * Lookup241: pallet_tips::pallet::Call<T, I>
     **/
    PalletTipsCall: {
        _enum: {
            report_awesome: {
                reason: string;
                who: string;
            };
            retract_tip: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
            };
            tip_new: {
                reason: string;
                who: string;
                tipValue: string;
            };
            tip: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
                tipValue: string;
            };
            close_tip: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
            };
            slash_tip: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
            };
        };
    };
    /**
     * Lookup242: pallet_identity::pallet::Call<T>
     **/
    PalletIdentityCall: {
        _enum: {
            add_registrar: {
                account: string;
            };
            set_identity: {
                info: string;
            };
            set_subs: {
                subs: string;
            };
            clear_identity: string;
            request_judgement: {
                regIndex: string;
                maxFee: string;
            };
            cancel_request: {
                regIndex: string;
            };
            set_fee: {
                index: string;
                fee: string;
            };
            set_account_id: {
                _alias: {
                    new_: string;
                };
                index: string;
                new_: string;
            };
            set_fields: {
                index: string;
                fields: string;
            };
            provide_judgement: {
                regIndex: string;
                target: string;
                judgement: string;
                identity: string;
            };
            kill_identity: {
                target: string;
            };
            add_sub: {
                sub: string;
                data: string;
            };
            rename_sub: {
                sub: string;
                data: string;
            };
            remove_sub: {
                sub: string;
            };
            quit_sub: string;
        };
    };
    /**
     * Lookup243: pallet_identity::types::IdentityInfo<FieldLimit>
     **/
    PalletIdentityIdentityInfo: {
        additional: string;
        display: string;
        legal: string;
        web: string;
        riot: string;
        email: string;
        pgpFingerprint: string;
        image: string;
        twitter: string;
    };
    /**
     * Lookup278: pallet_identity::types::BitFlags<pallet_identity::types::IdentityField>
     **/
    PalletIdentityBitFlags: {
        _bitLength: number;
        Display: number;
        Legal: number;
        Web: number;
        Riot: number;
        Email: number;
        PgpFingerprint: number;
        Image: number;
        Twitter: number;
    };
    /**
     * Lookup279: pallet_identity::types::IdentityField
     **/
    PalletIdentityIdentityField: {
        _enum: string[];
    };
    /**
     * Lookup280: pallet_identity::types::Judgement<Balance>
     **/
    PalletIdentityJudgement: {
        _enum: {
            Unknown: string;
            FeePaid: string;
            Reasonable: string;
            KnownGood: string;
            OutOfDate: string;
            LowQuality: string;
            Erroneous: string;
        };
    };
    /**
     * Lookup281: cumulus_pallet_parachain_system::pallet::Call<T>
     **/
    CumulusPalletParachainSystemCall: {
        _enum: {
            set_validation_data: {
                data: string;
            };
            sudo_send_upward_message: {
                message: string;
            };
            authorize_upgrade: {
                codeHash: string;
            };
            enact_authorized_upgrade: {
                code: string;
            };
        };
    };
    /**
     * Lookup282: cumulus_primitives_parachain_inherent::ParachainInherentData
     **/
    CumulusPrimitivesParachainInherentParachainInherentData: {
        validationData: string;
        relayChainState: string;
        downwardMessages: string;
        horizontalMessages: string;
    };
    /**
     * Lookup283: polkadot_primitives::v2::PersistedValidationData<primitive_types::H256, N>
     **/
    PolkadotPrimitivesV2PersistedValidationData: {
        parentHead: string;
        relayParentNumber: string;
        relayParentStorageRoot: string;
        maxPovSize: string;
    };
    /**
     * Lookup285: sp_trie::storage_proof::StorageProof
     **/
    SpTrieStorageProof: {
        trieNodes: string;
    };
    /**
     * Lookup288: polkadot_core_primitives::InboundDownwardMessage<BlockNumber>
     **/
    PolkadotCorePrimitivesInboundDownwardMessage: {
        sentAt: string;
        msg: string;
    };
    /**
     * Lookup291: polkadot_core_primitives::InboundHrmpMessage<BlockNumber>
     **/
    PolkadotCorePrimitivesInboundHrmpMessage: {
        sentAt: string;
        data: string;
    };
    /**
     * Lookup294: parachain_info::pallet::Call<T>
     **/
    ParachainInfoCall: string;
    /**
     * Lookup295: pallet_session::pallet::Call<T>
     **/
    PalletSessionCall: {
        _enum: {
            set_keys: {
                _alias: {
                    keys_: string;
                };
                keys_: string;
                proof: string;
            };
            purge_keys: string;
        };
    };
    /**
     * Lookup296: rococo_parachain_runtime::SessionKeys
     **/
    RococoParachainRuntimeSessionKeys: {
        aura: string;
    };
    /**
     * Lookup297: sp_consensus_aura::sr25519::app_sr25519::Public
     **/
    SpConsensusAuraSr25519AppSr25519Public: string;
    /**
     * Lookup298: sp_core::sr25519::Public
     **/
    SpCoreSr25519Public: string;
    /**
     * Lookup299: pallet_parachain_staking::pallet::Call<T>
     **/
    PalletParachainStakingCall: {
        _enum: {
            set_staking_expectations: {
                expectations: {
                    min: string;
                    ideal: string;
                    max: string;
                };
            };
            set_inflation: {
                schedule: {
                    min: string;
                    ideal: string;
                    max: string;
                };
            };
            set_parachain_bond_account: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_parachain_bond_reserve_percent: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_total_selected: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_collator_commission: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_blocks_per_round: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            add_candidates_whitelist: {
                candidate: string;
            };
            remove_candidates_whitelist: {
                candidate: string;
            };
            join_candidates: {
                bond: string;
            };
            schedule_leave_candidates: string;
            execute_leave_candidates: {
                candidate: string;
            };
            cancel_leave_candidates: string;
            go_offline: string;
            go_online: string;
            candidate_bond_more: {
                more: string;
            };
            schedule_candidate_bond_less: {
                less: string;
            };
            execute_candidate_bond_less: {
                candidate: string;
            };
            cancel_candidate_bond_less: string;
            delegate: {
                candidate: string;
                amount: string;
            };
            delegate_with_auto_compound: {
                candidate: string;
                amount: string;
                autoCompound: string;
            };
            schedule_leave_delegators: string;
            execute_leave_delegators: {
                delegator: string;
            };
            cancel_leave_delegators: string;
            schedule_revoke_delegation: {
                collator: string;
            };
            delegator_bond_more: {
                candidate: string;
                more: string;
            };
            schedule_delegator_bond_less: {
                candidate: string;
                less: string;
            };
            execute_delegation_request: {
                delegator: string;
                candidate: string;
            };
            cancel_delegation_request: {
                candidate: string;
            };
            set_auto_compound: {
                candidate: string;
                value: string;
            };
        };
    };
    /**
     * Lookup302: cumulus_pallet_xcmp_queue::pallet::Call<T>
     **/
    CumulusPalletXcmpQueueCall: {
        _enum: {
            service_overweight: {
                index: string;
                weightLimit: string;
            };
            suspend_xcm_execution: string;
            resume_xcm_execution: string;
            update_suspend_threshold: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            update_drop_threshold: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            update_resume_threshold: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            update_threshold_weight: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            update_weight_restrict_decay: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            update_xcmp_max_individual_weight: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
        };
    };
    /**
     * Lookup303: pallet_xcm::pallet::Call<T>
     **/
    PalletXcmCall: {
        _enum: {
            send: {
                dest: string;
                message: string;
            };
            teleport_assets: {
                dest: string;
                beneficiary: string;
                assets: string;
                feeAssetItem: string;
            };
            reserve_transfer_assets: {
                dest: string;
                beneficiary: string;
                assets: string;
                feeAssetItem: string;
            };
            execute: {
                message: string;
                maxWeight: string;
            };
            force_xcm_version: {
                location: string;
                xcmVersion: string;
            };
            force_default_xcm_version: {
                maybeXcmVersion: string;
            };
            force_subscribe_version_notify: {
                location: string;
            };
            force_unsubscribe_version_notify: {
                location: string;
            };
            limited_reserve_transfer_assets: {
                dest: string;
                beneficiary: string;
                assets: string;
                feeAssetItem: string;
                weightLimit: string;
            };
            limited_teleport_assets: {
                dest: string;
                beneficiary: string;
                assets: string;
                feeAssetItem: string;
                weightLimit: string;
            };
        };
    };
    /**
     * Lookup304: xcm::VersionedXcm<RuntimeCall>
     **/
    XcmVersionedXcm: {
        _enum: {
            __Unused0: string;
            __Unused1: string;
            V2: string;
            V3: string;
        };
    };
    /**
     * Lookup305: xcm::v2::Xcm<RuntimeCall>
     **/
    XcmV2Xcm: string;
    /**
     * Lookup307: xcm::v2::Instruction<RuntimeCall>
     **/
    XcmV2Instruction: {
        _enum: {
            WithdrawAsset: string;
            ReserveAssetDeposited: string;
            ReceiveTeleportedAsset: string;
            QueryResponse: {
                queryId: string;
                response: string;
                maxWeight: string;
            };
            TransferAsset: {
                assets: string;
                beneficiary: string;
            };
            TransferReserveAsset: {
                assets: string;
                dest: string;
                xcm: string;
            };
            Transact: {
                originType: string;
                requireWeightAtMost: string;
                call: string;
            };
            HrmpNewChannelOpenRequest: {
                sender: string;
                maxMessageSize: string;
                maxCapacity: string;
            };
            HrmpChannelAccepted: {
                recipient: string;
            };
            HrmpChannelClosing: {
                initiator: string;
                sender: string;
                recipient: string;
            };
            ClearOrigin: string;
            DescendOrigin: string;
            ReportError: {
                queryId: string;
                dest: string;
                maxResponseWeight: string;
            };
            DepositAsset: {
                assets: string;
                maxAssets: string;
                beneficiary: string;
            };
            DepositReserveAsset: {
                assets: string;
                maxAssets: string;
                dest: string;
                xcm: string;
            };
            ExchangeAsset: {
                give: string;
                receive: string;
            };
            InitiateReserveWithdraw: {
                assets: string;
                reserve: string;
                xcm: string;
            };
            InitiateTeleport: {
                assets: string;
                dest: string;
                xcm: string;
            };
            QueryHolding: {
                queryId: string;
                dest: string;
                assets: string;
                maxResponseWeight: string;
            };
            BuyExecution: {
                fees: string;
                weightLimit: string;
            };
            RefundSurplus: string;
            SetErrorHandler: string;
            SetAppendix: string;
            ClearError: string;
            ClaimAsset: {
                assets: string;
                ticket: string;
            };
            Trap: string;
            SubscribeVersion: {
                queryId: string;
                maxResponseWeight: string;
            };
            UnsubscribeVersion: string;
        };
    };
    /**
     * Lookup308: xcm::v2::Response
     **/
    XcmV2Response: {
        _enum: {
            Null: string;
            Assets: string;
            ExecutionResult: string;
            Version: string;
        };
    };
    /**
     * Lookup311: xcm::v2::traits::Error
     **/
    XcmV2TraitsError: {
        _enum: {
            Overflow: string;
            Unimplemented: string;
            UntrustedReserveLocation: string;
            UntrustedTeleportLocation: string;
            MultiLocationFull: string;
            MultiLocationNotInvertible: string;
            BadOrigin: string;
            InvalidLocation: string;
            AssetNotFound: string;
            FailedToTransactAsset: string;
            NotWithdrawable: string;
            LocationCannotHold: string;
            ExceedsMaxMessageSize: string;
            DestinationUnsupported: string;
            Transport: string;
            Unroutable: string;
            UnknownClaim: string;
            FailedToDecode: string;
            MaxWeightInvalid: string;
            NotHoldingFees: string;
            TooExpensive: string;
            Trap: string;
            UnhandledXcmVersion: string;
            WeightLimitReached: string;
            Barrier: string;
            WeightNotComputable: string;
        };
    };
    /**
     * Lookup312: xcm::v2::multiasset::MultiAssetFilter
     **/
    XcmV2MultiassetMultiAssetFilter: {
        _enum: {
            Definite: string;
            Wild: string;
        };
    };
    /**
     * Lookup313: xcm::v2::multiasset::WildMultiAsset
     **/
    XcmV2MultiassetWildMultiAsset: {
        _enum: {
            All: string;
            AllOf: {
                id: string;
                fun: string;
            };
        };
    };
    /**
     * Lookup314: xcm::v2::multiasset::WildFungibility
     **/
    XcmV2MultiassetWildFungibility: {
        _enum: string[];
    };
    /**
     * Lookup315: xcm::v2::WeightLimit
     **/
    XcmV2WeightLimit: {
        _enum: {
            Unlimited: string;
            Limited: string;
        };
    };
    /**
     * Lookup324: cumulus_pallet_xcm::pallet::Call<T>
     **/
    CumulusPalletXcmCall: string;
    /**
     * Lookup325: cumulus_pallet_dmp_queue::pallet::Call<T>
     **/
    CumulusPalletDmpQueueCall: {
        _enum: {
            service_overweight: {
                index: string;
                weightLimit: string;
            };
        };
    };
    /**
     * Lookup326: orml_xtokens::module::Call<T>
     **/
    OrmlXtokensModuleCall: {
        _enum: {
            transfer: {
                currencyId: string;
                amount: string;
                dest: string;
                destWeightLimit: string;
            };
            transfer_multiasset: {
                asset: string;
                dest: string;
                destWeightLimit: string;
            };
            transfer_with_fee: {
                currencyId: string;
                amount: string;
                fee: string;
                dest: string;
                destWeightLimit: string;
            };
            transfer_multiasset_with_fee: {
                asset: string;
                fee: string;
                dest: string;
                destWeightLimit: string;
            };
            transfer_multicurrencies: {
                currencies: string;
                feeItem: string;
                dest: string;
                destWeightLimit: string;
            };
            transfer_multiassets: {
                assets: string;
                feeItem: string;
                dest: string;
                destWeightLimit: string;
            };
        };
    };
    /**
     * Lookup327: xcm::VersionedMultiAsset
     **/
    XcmVersionedMultiAsset: {
        _enum: {
            __Unused0: string;
            V2: string;
            __Unused2: string;
            V3: string;
        };
    };
    /**
     * Lookup330: orml_tokens::module::Call<T>
     **/
    OrmlTokensModuleCall: {
        _enum: {
            transfer: {
                dest: string;
                currencyId: string;
                amount: string;
            };
            transfer_all: {
                dest: string;
                currencyId: string;
                keepAlive: string;
            };
            transfer_keep_alive: {
                dest: string;
                currencyId: string;
                amount: string;
            };
            force_transfer: {
                source: string;
                dest: string;
                currencyId: string;
                amount: string;
            };
            set_balance: {
                who: string;
                currencyId: string;
                newFree: string;
                newReserved: string;
            };
        };
    };
    /**
     * Lookup331: pallet_bridge::pallet::Call<T>
     **/
    PalletBridgeCall: {
        _enum: {
            set_threshold: {
                threshold: string;
            };
            set_resource: {
                id: string;
                method: string;
            };
            remove_resource: {
                id: string;
            };
            whitelist_chain: {
                id: string;
            };
            add_relayer: {
                v: string;
            };
            remove_relayer: {
                v: string;
            };
            update_fee: {
                destId: string;
                fee: string;
            };
            acknowledge_proposal: {
                nonce: string;
                srcId: string;
                rId: string;
                call: string;
            };
            reject_proposal: {
                nonce: string;
                srcId: string;
                rId: string;
                call: string;
            };
            eval_vote_state: {
                nonce: string;
                srcId: string;
                prop: string;
            };
        };
    };
    /**
     * Lookup332: pallet_bridge_transfer::pallet::Call<T>
     **/
    PalletBridgeTransferCall: {
        _enum: {
            transfer_native: {
                amount: string;
                recipient: string;
                destId: string;
            };
            transfer: {
                to: string;
                amount: string;
                rid: string;
            };
            set_maximum_issuance: {
                maximumIssuance: string;
            };
            set_external_balances: {
                externalBalances: string;
            };
        };
    };
    /**
     * Lookup333: pallet_drop3::pallet::Call<T>
     **/
    PalletDrop3Call: {
        _enum: {
            set_admin: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            approve_reward_pool: {
                id: string;
            };
            reject_reward_pool: {
                id: string;
            };
            start_reward_pool: {
                id: string;
            };
            stop_reward_pool: {
                id: string;
            };
            close_reward_pool: {
                id: string;
            };
            propose_reward_pool: {
                name: string;
                total: string;
                startAt: string;
                endAt: string;
            };
            send_reward: {
                id: string;
                to: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup334: pallet_extrinsic_filter::pallet::Call<T>
     **/
    PalletExtrinsicFilterCall: {
        _enum: {
            set_mode: {
                mode: string;
            };
            block_extrinsics: {
                palletNameBytes: string;
                functionNameBytes: string;
            };
            unblock_extrinsics: {
                palletNameBytes: string;
                functionNameBytes: string;
            };
        };
    };
    /**
     * Lookup335: pallet_identity_management::pallet::Call<T>
     **/
    PalletIdentityManagementCall: {
        _enum: {
            add_delegatee: {
                account: string;
            };
            remove_delegatee: {
                account: string;
            };
            set_user_shielding_key: {
                shard: string;
                encryptedKey: string;
            };
            create_identity: {
                shard: string;
                user: string;
                encryptedIdentity: string;
                encryptedMetadata: string;
            };
            remove_identity: {
                shard: string;
                encryptedIdentity: string;
            };
            verify_identity: {
                shard: string;
                encryptedIdentity: string;
                encryptedValidationData: string;
            };
            __Unused6: string;
            __Unused7: string;
            __Unused8: string;
            __Unused9: string;
            __Unused10: string;
            __Unused11: string;
            __Unused12: string;
            __Unused13: string;
            __Unused14: string;
            __Unused15: string;
            __Unused16: string;
            __Unused17: string;
            __Unused18: string;
            __Unused19: string;
            __Unused20: string;
            __Unused21: string;
            __Unused22: string;
            __Unused23: string;
            __Unused24: string;
            __Unused25: string;
            __Unused26: string;
            __Unused27: string;
            __Unused28: string;
            __Unused29: string;
            user_shielding_key_set: {
                account: string;
                idGraph: string;
                reqExtHash: string;
            };
            identity_created: {
                account: string;
                identity: string;
                code: string;
                reqExtHash: string;
            };
            identity_removed: {
                account: string;
                identity: string;
                reqExtHash: string;
            };
            identity_verified: {
                account: string;
                identity: string;
                idGraph: string;
                reqExtHash: string;
            };
            some_error: {
                account: string;
                error: string;
                reqExtHash: string;
            };
        };
    };
    /**
     * Lookup336: core_primitives::error::IMPError
     **/
    CorePrimitivesErrorImpError: {
        _enum: {
            SetUserShieldingKeyFailed: string;
            CreateIdentityFailed: string;
            RemoveIdentityFailed: string;
            VerifyIdentityFailed: string;
            ImportScheduledEnclaveFailed: string;
            UnclassifiedError: string;
        };
    };
    /**
     * Lookup337: pallet_asset_manager::pallet::Call<T>
     **/
    PalletAssetManagerCall: {
        _enum: {
            register_foreign_asset_type: {
                assetType: string;
                metadata: string;
            };
            update_foreign_asset_metadata: {
                assetId: string;
                metadata: string;
            };
            set_asset_units_per_second: {
                assetId: string;
                unitsPerSecond: string;
            };
            add_asset_type: {
                assetId: string;
                newAssetType: string;
            };
            remove_asset_type: {
                assetType: string;
                newDefaultAssetType: string;
            };
        };
    };
    /**
     * Lookup339: pallet_vc_management::pallet::Call<T>
     **/
    PalletVcManagementCall: {
        _enum: {
            request_vc: {
                shard: string;
                assertion: string;
            };
            disable_vc: {
                index: string;
            };
            revoke_vc: {
                index: string;
            };
            set_admin: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            add_schema: {
                shard: string;
                id: string;
                content: string;
            };
            disable_schema: {
                shard: string;
                index: string;
            };
            activate_schema: {
                shard: string;
                index: string;
            };
            revoke_schema: {
                shard: string;
                index: string;
            };
            add_vc_registry_item: {
                _alias: {
                    hash_: string;
                };
                index: string;
                subject: string;
                assertion: string;
                hash_: string;
            };
            remove_vc_registry_item: {
                index: string;
            };
            clear_vc_registry: string;
            __Unused11: string;
            __Unused12: string;
            __Unused13: string;
            __Unused14: string;
            __Unused15: string;
            __Unused16: string;
            __Unused17: string;
            __Unused18: string;
            __Unused19: string;
            __Unused20: string;
            __Unused21: string;
            __Unused22: string;
            __Unused23: string;
            __Unused24: string;
            __Unused25: string;
            __Unused26: string;
            __Unused27: string;
            __Unused28: string;
            __Unused29: string;
            vc_issued: {
                _alias: {
                    hash_: string;
                };
                account: string;
                assertion: string;
                index: string;
                hash_: string;
                vc: string;
                reqExtHash: string;
            };
            some_error: {
                account: string;
                error: string;
                reqExtHash: string;
            };
        };
    };
    /**
     * Lookup340: core_primitives::error::VCMPError
     **/
    CorePrimitivesErrorVcmpError: {
        _enum: {
            RequestVCFailed: string;
            UnclassifiedError: string;
        };
    };
    /**
     * Lookup341: pallet_group::pallet::Call<T, I>
     **/
    PalletGroupCall: {
        _enum: {
            add_group_member: {
                v: string;
            };
            batch_add_group_members: {
                vs: string;
            };
            remove_group_member: {
                v: string;
            };
            batch_remove_group_members: {
                vs: string;
            };
            switch_group_control_on: string;
            switch_group_control_off: string;
        };
    };
    /**
     * Lookup343: pallet_teerex::pallet::Call<T>
     **/
    PalletTeerexCall: {
        _enum: {
            register_enclave: {
                raReport: string;
                workerUrl: string;
                shieldingKey: string;
                vcPubkey: string;
            };
            unregister_enclave: string;
            call_worker: {
                request: string;
            };
            confirm_processed_parentchain_block: {
                blockHash: string;
                blockNumber: string;
                trustedCallsMerkleRoot: string;
            };
            shield_funds: {
                incognitoAccountEncrypted: string;
                amount: string;
                bondingAccount: string;
            };
            unshield_funds: {
                publicAccount: string;
                amount: string;
                bondingAccount: string;
                callHash: string;
            };
            set_heartbeat_timeout: {
                timeout: string;
            };
            register_dcap_enclave: {
                dcapQuote: string;
                workerUrl: string;
            };
            update_scheduled_enclave: {
                sidechainBlockNumber: string;
                mrEnclave: string;
            };
            register_quoting_enclave: {
                enclaveIdentity: string;
                signature: string;
                certificateChain: string;
            };
            remove_scheduled_enclave: {
                sidechainBlockNumber: string;
            };
            register_tcb_info: {
                tcbInfo: string;
                signature: string;
                certificateChain: string;
            };
            publish_hash: {
                _alias: {
                    hash_: string;
                };
                hash_: string;
                extraTopics: string;
                data: string;
            };
            set_admin: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
        };
    };
    /**
     * Lookup344: teerex_primitives::Request
     **/
    TeerexPrimitivesRequest: {
        shard: string;
        cyphertext: string;
    };
    /**
     * Lookup345: pallet_sidechain::pallet::Call<T>
     **/
    PalletSidechainCall: {
        _enum: {
            confirm_imported_sidechain_block: {
                shardId: string;
                blockNumber: string;
                nextFinalizationCandidateBlockNumber: string;
                blockHeaderHash: string;
            };
        };
    };
    /**
     * Lookup346: pallet_teeracle::pallet::Call<T>
     **/
    PalletTeeracleCall: {
        _enum: {
            add_to_whitelist: {
                dataSource: string;
                mrenclave: string;
            };
            remove_from_whitelist: {
                dataSource: string;
                mrenclave: string;
            };
            update_oracle: {
                oracleName: string;
                dataSource: string;
                newBlob: string;
            };
            update_exchange_rate: {
                dataSource: string;
                tradingPair: string;
                newValue: string;
            };
        };
    };
    /**
     * Lookup348: pallet_identity_management_mock::pallet::Call<T>
     **/
    PalletIdentityManagementMockCall: {
        _enum: {
            add_delegatee: {
                account: string;
            };
            remove_delegatee: {
                account: string;
            };
            set_user_shielding_key: {
                shard: string;
                encryptedKey: string;
            };
            create_identity: {
                shard: string;
                user: string;
                encryptedIdentity: string;
                encryptedMetadata: string;
            };
            remove_identity: {
                shard: string;
                encryptedIdentity: string;
            };
            verify_identity: {
                shard: string;
                encryptedIdentity: string;
                encryptedValidationData: string;
            };
            user_shielding_key_set: {
                account: string;
            };
            identity_created: {
                account: string;
                identity: string;
                code: string;
                idGraph: string;
            };
            identity_removed: {
                account: string;
                identity: string;
                idGraph: string;
            };
            identity_verified: {
                account: string;
                identity: string;
                idGraph: string;
            };
            some_error: {
                func: string;
                error: string;
            };
        };
    };
    /**
     * Lookup349: pallet_sudo::pallet::Call<T>
     **/
    PalletSudoCall: {
        _enum: {
            sudo: {
                call: string;
            };
            sudo_unchecked_weight: {
                call: string;
                weight: string;
            };
            set_key: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            sudo_as: {
                who: string;
                call: string;
            };
        };
    };
    /**
     * Lookup352: pallet_scheduler::pallet::Error<T>
     **/
    PalletSchedulerError: {
        _enum: string[];
    };
    /**
     * Lookup353: pallet_utility::pallet::Error<T>
     **/
    PalletUtilityError: {
        _enum: string[];
    };
    /**
     * Lookup355: pallet_multisig::Multisig<BlockNumber, Balance, sp_core::crypto::AccountId32, MaxApprovals>
     **/
    PalletMultisigMultisig: {
        when: string;
        deposit: string;
        depositor: string;
        approvals: string;
    };
    /**
     * Lookup357: pallet_multisig::pallet::Error<T>
     **/
    PalletMultisigError: {
        _enum: string[];
    };
    /**
     * Lookup360: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, rococo_parachain_runtime::ProxyType, BlockNumber>
     **/
    PalletProxyProxyDefinition: {
        delegate: string;
        proxyType: string;
        delay: string;
    };
    /**
     * Lookup364: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber>
     **/
    PalletProxyAnnouncement: {
        real: string;
        callHash: string;
        height: string;
    };
    /**
     * Lookup366: pallet_proxy::pallet::Error<T>
     **/
    PalletProxyError: {
        _enum: string[];
    };
    /**
     * Lookup367: pallet_preimage::RequestStatus<sp_core::crypto::AccountId32, Balance>
     **/
    PalletPreimageRequestStatus: {
        _enum: {
            Unrequested: {
                deposit: string;
                len: string;
            };
            Requested: {
                deposit: string;
                count: string;
                len: string;
            };
        };
    };
    /**
     * Lookup372: pallet_preimage::pallet::Error<T>
     **/
    PalletPreimageError: {
        _enum: string[];
    };
    /**
     * Lookup374: pallet_balances::BalanceLock<Balance>
     **/
    PalletBalancesBalanceLock: {
        id: string;
        amount: string;
        reasons: string;
    };
    /**
     * Lookup375: pallet_balances::Reasons
     **/
    PalletBalancesReasons: {
        _enum: string[];
    };
    /**
     * Lookup378: pallet_balances::ReserveData<ReserveIdentifier, Balance>
     **/
    PalletBalancesReserveData: {
        id: string;
        amount: string;
    };
    /**
     * Lookup380: pallet_balances::pallet::Error<T, I>
     **/
    PalletBalancesError: {
        _enum: string[];
    };
    /**
     * Lookup383: pallet_vesting::Releases
     **/
    PalletVestingReleases: {
        _enum: string[];
    };
    /**
     * Lookup384: pallet_vesting::pallet::Error<T>
     **/
    PalletVestingError: {
        _enum: string[];
    };
    /**
     * Lookup386: pallet_transaction_payment::Releases
     **/
    PalletTransactionPaymentReleases: {
        _enum: string[];
    };
    /**
     * Lookup387: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
     **/
    PalletTreasuryProposal: {
        proposer: string;
        value: string;
        beneficiary: string;
        bond: string;
    };
    /**
     * Lookup392: frame_support::PalletId
     **/
    FrameSupportPalletId: string;
    /**
     * Lookup393: pallet_treasury::pallet::Error<T, I>
     **/
    PalletTreasuryError: {
        _enum: string[];
    };
    /**
     * Lookup398: pallet_democracy::types::ReferendumInfo<BlockNumber, frame_support::traits::preimages::Bounded<rococo_parachain_runtime::RuntimeCall>, Balance>
     **/
    PalletDemocracyReferendumInfo: {
        _enum: {
            Ongoing: string;
            Finished: {
                approved: string;
                end: string;
            };
        };
    };
    /**
     * Lookup399: pallet_democracy::types::ReferendumStatus<BlockNumber, frame_support::traits::preimages::Bounded<rococo_parachain_runtime::RuntimeCall>, Balance>
     **/
    PalletDemocracyReferendumStatus: {
        end: string;
        proposal: string;
        threshold: string;
        delay: string;
        tally: string;
    };
    /**
     * Lookup400: pallet_democracy::types::Tally<Balance>
     **/
    PalletDemocracyTally: {
        ayes: string;
        nays: string;
        turnout: string;
    };
    /**
     * Lookup401: pallet_democracy::vote::Voting<Balance, sp_core::crypto::AccountId32, BlockNumber, MaxVotes>
     **/
    PalletDemocracyVoteVoting: {
        _enum: {
            Direct: {
                votes: string;
                delegations: string;
                prior: string;
            };
            Delegating: {
                balance: string;
                target: string;
                conviction: string;
                delegations: string;
                prior: string;
            };
        };
    };
    /**
     * Lookup405: pallet_democracy::types::Delegations<Balance>
     **/
    PalletDemocracyDelegations: {
        votes: string;
        capital: string;
    };
    /**
     * Lookup406: pallet_democracy::vote::PriorLock<BlockNumber, Balance>
     **/
    PalletDemocracyVotePriorLock: string;
    /**
     * Lookup409: pallet_democracy::pallet::Error<T>
     **/
    PalletDemocracyError: {
        _enum: string[];
    };
    /**
     * Lookup411: pallet_collective::Votes<sp_core::crypto::AccountId32, BlockNumber>
     **/
    PalletCollectiveVotes: {
        index: string;
        threshold: string;
        ayes: string;
        nays: string;
        end: string;
    };
    /**
     * Lookup412: pallet_collective::pallet::Error<T, I>
     **/
    PalletCollectiveError: {
        _enum: string[];
    };
    /**
     * Lookup414: pallet_membership::pallet::Error<T, I>
     **/
    PalletMembershipError: {
        _enum: string[];
    };
    /**
     * Lookup417: pallet_bounties::Bounty<sp_core::crypto::AccountId32, Balance, BlockNumber>
     **/
    PalletBountiesBounty: {
        proposer: string;
        value: string;
        fee: string;
        curatorDeposit: string;
        bond: string;
        status: string;
    };
    /**
     * Lookup418: pallet_bounties::BountyStatus<sp_core::crypto::AccountId32, BlockNumber>
     **/
    PalletBountiesBountyStatus: {
        _enum: {
            Proposed: string;
            Approved: string;
            Funded: string;
            CuratorProposed: {
                curator: string;
            };
            Active: {
                curator: string;
                updateDue: string;
            };
            PendingPayout: {
                curator: string;
                beneficiary: string;
                unlockAt: string;
            };
        };
    };
    /**
     * Lookup420: pallet_bounties::pallet::Error<T, I>
     **/
    PalletBountiesError: {
        _enum: string[];
    };
    /**
     * Lookup421: pallet_tips::OpenTip<sp_core::crypto::AccountId32, Balance, BlockNumber, primitive_types::H256>
     **/
    PalletTipsOpenTip: {
        reason: string;
        who: string;
        finder: string;
        deposit: string;
        closes: string;
        tips: string;
        findersFee: string;
    };
    /**
     * Lookup423: pallet_tips::pallet::Error<T, I>
     **/
    PalletTipsError: {
        _enum: string[];
    };
    /**
     * Lookup424: pallet_identity::types::Registration<Balance, MaxJudgements, MaxAdditionalFields>
     **/
    PalletIdentityRegistration: {
        judgements: string;
        deposit: string;
        info: string;
    };
    /**
     * Lookup431: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32>
     **/
    PalletIdentityRegistrarInfo: {
        account: string;
        fee: string;
        fields: string;
    };
    /**
     * Lookup433: pallet_identity::pallet::Error<T>
     **/
    PalletIdentityError: {
        _enum: string[];
    };
    /**
     * Lookup435: polkadot_primitives::v2::UpgradeRestriction
     **/
    PolkadotPrimitivesV2UpgradeRestriction: {
        _enum: string[];
    };
    /**
     * Lookup436: cumulus_pallet_parachain_system::relay_state_snapshot::MessagingStateSnapshot
     **/
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: {
        dmqMqcHead: string;
        relayDispatchQueueSize: string;
        ingressChannels: string;
        egressChannels: string;
    };
    /**
     * Lookup439: polkadot_primitives::v2::AbridgedHrmpChannel
     **/
    PolkadotPrimitivesV2AbridgedHrmpChannel: {
        maxCapacity: string;
        maxTotalSize: string;
        maxMessageSize: string;
        msgCount: string;
        totalSize: string;
        mqcHead: string;
    };
    /**
     * Lookup440: polkadot_primitives::v2::AbridgedHostConfiguration
     **/
    PolkadotPrimitivesV2AbridgedHostConfiguration: {
        maxCodeSize: string;
        maxHeadDataSize: string;
        maxUpwardQueueCount: string;
        maxUpwardQueueSize: string;
        maxUpwardMessageSize: string;
        maxUpwardMessageNumPerCandidate: string;
        hrmpMaxMessageNumPerCandidate: string;
        validationUpgradeCooldown: string;
        validationUpgradeDelay: string;
    };
    /**
     * Lookup446: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain::primitives::Id>
     **/
    PolkadotCorePrimitivesOutboundHrmpMessage: {
        recipient: string;
        data: string;
    };
    /**
     * Lookup447: cumulus_pallet_parachain_system::pallet::Error<T>
     **/
    CumulusPalletParachainSystemError: {
        _enum: string[];
    };
    /**
     * Lookup451: sp_core::crypto::KeyTypeId
     **/
    SpCoreCryptoKeyTypeId: string;
    /**
     * Lookup452: pallet_session::pallet::Error<T>
     **/
    PalletSessionError: {
        _enum: string[];
    };
    /**
     * Lookup456: pallet_parachain_staking::types::ParachainBondConfig<sp_core::crypto::AccountId32>
     **/
    PalletParachainStakingParachainBondConfig: {
        account: string;
        percent: string;
    };
    /**
     * Lookup457: pallet_parachain_staking::types::RoundInfo<BlockNumber>
     **/
    PalletParachainStakingRoundInfo: {
        current: string;
        first: string;
        length: string;
    };
    /**
     * Lookup458: pallet_parachain_staking::types::Delegator<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingDelegator: {
        id: string;
        delegations: string;
        total: string;
        lessTotal: string;
        status: string;
    };
    /**
     * Lookup459: pallet_parachain_staking::set::OrderedSet<pallet_parachain_staking::types::Bond<sp_core::crypto::AccountId32, Balance>>
     **/
    PalletParachainStakingSetOrderedSet: string;
    /**
     * Lookup460: pallet_parachain_staking::types::Bond<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingBond: {
        owner: string;
        amount: string;
    };
    /**
     * Lookup462: pallet_parachain_staking::types::DelegatorStatus
     **/
    PalletParachainStakingDelegatorStatus: {
        _enum: string[];
    };
    /**
     * Lookup463: pallet_parachain_staking::types::CandidateMetadata<Balance>
     **/
    PalletParachainStakingCandidateMetadata: {
        bond: string;
        delegationCount: string;
        totalCounted: string;
        lowestTopDelegationAmount: string;
        highestBottomDelegationAmount: string;
        lowestBottomDelegationAmount: string;
        topCapacity: string;
        bottomCapacity: string;
        request: string;
        status: string;
    };
    /**
     * Lookup464: pallet_parachain_staking::types::CapacityStatus
     **/
    PalletParachainStakingCapacityStatus: {
        _enum: string[];
    };
    /**
     * Lookup466: pallet_parachain_staking::types::CandidateBondLessRequest<Balance>
     **/
    PalletParachainStakingCandidateBondLessRequest: {
        amount: string;
        whenExecutable: string;
    };
    /**
     * Lookup467: pallet_parachain_staking::types::CollatorStatus
     **/
    PalletParachainStakingCollatorStatus: {
        _enum: {
            Active: string;
            Idle: string;
            Leaving: string;
        };
    };
    /**
     * Lookup469: pallet_parachain_staking::delegation_requests::ScheduledRequest<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingDelegationRequestsScheduledRequest: {
        delegator: string;
        whenExecutable: string;
        action: string;
    };
    /**
     * Lookup471: pallet_parachain_staking::auto_compound::AutoCompoundConfig<sp_core::crypto::AccountId32>
     **/
    PalletParachainStakingAutoCompoundAutoCompoundConfig: {
        delegator: string;
        value: string;
    };
    /**
     * Lookup472: pallet_parachain_staking::types::Delegations<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingDelegations: {
        delegations: string;
        total: string;
    };
    /**
     * Lookup474: pallet_parachain_staking::types::CollatorSnapshot<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingCollatorSnapshot: {
        bond: string;
        delegations: string;
        total: string;
    };
    /**
     * Lookup476: pallet_parachain_staking::types::BondWithAutoCompound<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingBondWithAutoCompound: {
        owner: string;
        amount: string;
        autoCompound: string;
    };
    /**
     * Lookup477: pallet_parachain_staking::types::DelayedPayout<Balance>
     **/
    PalletParachainStakingDelayedPayout: {
        roundIssuance: string;
        totalStakingReward: string;
        collatorCommission: string;
    };
    /**
     * Lookup478: pallet_parachain_staking::inflation::InflationInfo<Balance>
     **/
    PalletParachainStakingInflationInflationInfo: {
        expect: {
            min: string;
            ideal: string;
            max: string;
        };
        annual: {
            min: string;
            ideal: string;
            max: string;
        };
        round: {
            min: string;
            ideal: string;
            max: string;
        };
    };
    /**
     * Lookup479: pallet_parachain_staking::pallet::Error<T>
     **/
    PalletParachainStakingError: {
        _enum: string[];
    };
    /**
     * Lookup481: cumulus_pallet_xcmp_queue::InboundChannelDetails
     **/
    CumulusPalletXcmpQueueInboundChannelDetails: {
        sender: string;
        state: string;
        messageMetadata: string;
    };
    /**
     * Lookup482: cumulus_pallet_xcmp_queue::InboundState
     **/
    CumulusPalletXcmpQueueInboundState: {
        _enum: string[];
    };
    /**
     * Lookup485: polkadot_parachain::primitives::XcmpMessageFormat
     **/
    PolkadotParachainPrimitivesXcmpMessageFormat: {
        _enum: string[];
    };
    /**
     * Lookup488: cumulus_pallet_xcmp_queue::OutboundChannelDetails
     **/
    CumulusPalletXcmpQueueOutboundChannelDetails: {
        recipient: string;
        state: string;
        signalsExist: string;
        firstIndex: string;
        lastIndex: string;
    };
    /**
     * Lookup489: cumulus_pallet_xcmp_queue::OutboundState
     **/
    CumulusPalletXcmpQueueOutboundState: {
        _enum: string[];
    };
    /**
     * Lookup491: cumulus_pallet_xcmp_queue::QueueConfigData
     **/
    CumulusPalletXcmpQueueQueueConfigData: {
        suspendThreshold: string;
        dropThreshold: string;
        resumeThreshold: string;
        thresholdWeight: string;
        weightRestrictDecay: string;
        xcmpMaxIndividualWeight: string;
    };
    /**
     * Lookup493: cumulus_pallet_xcmp_queue::pallet::Error<T>
     **/
    CumulusPalletXcmpQueueError: {
        _enum: string[];
    };
    /**
     * Lookup494: pallet_xcm::pallet::QueryStatus<BlockNumber>
     **/
    PalletXcmQueryStatus: {
        _enum: {
            Pending: {
                responder: string;
                maybeMatchQuerier: string;
                maybeNotify: string;
                timeout: string;
            };
            VersionNotifier: {
                origin: string;
                isActive: string;
            };
            Ready: {
                response: string;
                at: string;
            };
        };
    };
    /**
     * Lookup498: xcm::VersionedResponse
     **/
    XcmVersionedResponse: {
        _enum: {
            __Unused0: string;
            __Unused1: string;
            V2: string;
            V3: string;
        };
    };
    /**
     * Lookup504: pallet_xcm::pallet::VersionMigrationStage
     **/
    PalletXcmVersionMigrationStage: {
        _enum: {
            MigrateSupportedVersion: string;
            MigrateVersionNotifiers: string;
            NotifyCurrentTargets: string;
            MigrateAndNotifyOldTargets: string;
        };
    };
    /**
     * Lookup506: xcm::VersionedAssetId
     **/
    XcmVersionedAssetId: {
        _enum: {
            __Unused0: string;
            __Unused1: string;
            __Unused2: string;
            V3: string;
        };
    };
    /**
     * Lookup507: pallet_xcm::pallet::RemoteLockedFungibleRecord
     **/
    PalletXcmRemoteLockedFungibleRecord: {
        amount: string;
        owner: string;
        locker: string;
        users: string;
    };
    /**
     * Lookup511: pallet_xcm::pallet::Error<T>
     **/
    PalletXcmError: {
        _enum: string[];
    };
    /**
     * Lookup512: cumulus_pallet_xcm::pallet::Error<T>
     **/
    CumulusPalletXcmError: string;
    /**
     * Lookup513: cumulus_pallet_dmp_queue::ConfigData
     **/
    CumulusPalletDmpQueueConfigData: {
        maxIndividual: string;
    };
    /**
     * Lookup514: cumulus_pallet_dmp_queue::PageIndexData
     **/
    CumulusPalletDmpQueuePageIndexData: {
        beginUsed: string;
        endUsed: string;
        overweightCount: string;
    };
    /**
     * Lookup517: cumulus_pallet_dmp_queue::pallet::Error<T>
     **/
    CumulusPalletDmpQueueError: {
        _enum: string[];
    };
    /**
     * Lookup518: orml_xtokens::module::Error<T>
     **/
    OrmlXtokensModuleError: {
        _enum: string[];
    };
    /**
     * Lookup520: orml_tokens::BalanceLock<Balance>
     **/
    OrmlTokensBalanceLock: {
        id: string;
        amount: string;
    };
    /**
     * Lookup522: orml_tokens::AccountData<Balance>
     **/
    OrmlTokensAccountData: {
        free: string;
        reserved: string;
        frozen: string;
    };
    /**
     * Lookup524: orml_tokens::ReserveData<ReserveIdentifier, Balance>
     **/
    OrmlTokensReserveData: {
        id: string;
        amount: string;
    };
    /**
     * Lookup526: orml_tokens::module::Error<T>
     **/
    OrmlTokensModuleError: {
        _enum: string[];
    };
    /**
     * Lookup529: pallet_bridge::pallet::ProposalVotes<sp_core::crypto::AccountId32, BlockNumber>
     **/
    PalletBridgeProposalVotes: {
        votesFor: string;
        votesAgainst: string;
        status: string;
        expiry: string;
    };
    /**
     * Lookup530: pallet_bridge::pallet::ProposalStatus
     **/
    PalletBridgeProposalStatus: {
        _enum: string[];
    };
    /**
     * Lookup532: pallet_bridge::pallet::BridgeEvent
     **/
    PalletBridgeBridgeEvent: {
        _enum: {
            FungibleTransfer: string;
            NonFungibleTransfer: string;
            GenericTransfer: string;
        };
    };
    /**
     * Lookup533: pallet_bridge::pallet::Error<T>
     **/
    PalletBridgeError: {
        _enum: string[];
    };
    /**
     * Lookup535: pallet_bridge_transfer::pallet::Error<T>
     **/
    PalletBridgeTransferError: {
        _enum: string[];
    };
    /**
     * Lookup536: pallet_drop3::RewardPool<PoolId, bounded_collections::bounded_vec::BoundedVec<T, S>, sp_core::crypto::AccountId32, Balance, BlockNumber>
     **/
    PalletDrop3RewardPool: {
        id: string;
        name: string;
        owner: string;
        total: string;
        remain: string;
        createAt: string;
        startAt: string;
        endAt: string;
        started: string;
        approved: string;
    };
    /**
     * Lookup538: pallet_drop3::pallet::Error<T>
     **/
    PalletDrop3Error: {
        _enum: string[];
    };
    /**
     * Lookup539: pallet_extrinsic_filter::pallet::Error<T>
     **/
    PalletExtrinsicFilterError: {
        _enum: string[];
    };
    /**
     * Lookup540: pallet_identity_management::pallet::Error<T>
     **/
    PalletIdentityManagementError: {
        _enum: string[];
    };
    /**
     * Lookup541: pallet_asset_manager::pallet::Error<T>
     **/
    PalletAssetManagerError: {
        _enum: string[];
    };
    /**
     * Lookup542: pallet_vc_management::vc_context::VCContext<T>
     **/
    PalletVcManagementVcContext: {
        _alias: {
            hash_: string;
        };
        subject: string;
        assertion: string;
        hash_: string;
        status: string;
    };
    /**
     * Lookup543: pallet_vc_management::vc_context::Status
     **/
    PalletVcManagementVcContextStatus: {
        _enum: string[];
    };
    /**
     * Lookup544: pallet_vc_management::schema::VCSchema<T>
     **/
    PalletVcManagementSchemaVcSchema: {
        id: string;
        author: string;
        content: string;
        status: string;
    };
    /**
     * Lookup546: pallet_vc_management::pallet::Error<T>
     **/
    PalletVcManagementError: {
        _enum: string[];
    };
    /**
     * Lookup547: pallet_group::pallet::Error<T, I>
     **/
    PalletGroupError: {
        _enum: string[];
    };
    /**
     * Lookup549: teerex_primitives::Enclave<sp_core::crypto::AccountId32, Url>
     **/
    TeerexPrimitivesEnclave: {
        pubkey: string;
        mrEnclave: string;
        timestamp: string;
        url: string;
        shieldingKey: string;
        vcPubkey: string;
        sgxMode: string;
        sgxMetadata: string;
    };
    /**
     * Lookup550: teerex_primitives::SgxBuildMode
     **/
    TeerexPrimitivesSgxBuildMode: {
        _enum: string[];
    };
    /**
     * Lookup551: teerex_primitives::SgxEnclaveMetadata
     **/
    TeerexPrimitivesSgxEnclaveMetadata: {
        quote: string;
        quoteSig: string;
        quoteCert: string;
    };
    /**
     * Lookup552: teerex_primitives::QuotingEnclave
     **/
    TeerexPrimitivesQuotingEnclave: {
        issueDate: string;
        nextUpdate: string;
        miscselect: string;
        miscselectMask: string;
        attributes: string;
        attributesMask: string;
        mrsigner: string;
        isvprodid: string;
        tcb: string;
    };
    /**
     * Lookup554: teerex_primitives::QeTcb
     **/
    TeerexPrimitivesQeTcb: {
        isvsvn: string;
    };
    /**
     * Lookup555: teerex_primitives::TcbInfoOnChain
     **/
    TeerexPrimitivesTcbInfoOnChain: {
        issueDate: string;
        nextUpdate: string;
        tcbLevels: string;
    };
    /**
     * Lookup557: teerex_primitives::TcbVersionStatus
     **/
    TeerexPrimitivesTcbVersionStatus: {
        cpusvn: string;
        pcesvn: string;
    };
    /**
     * Lookup558: pallet_teerex::pallet::Error<T>
     **/
    PalletTeerexError: {
        _enum: string[];
    };
    /**
     * Lookup559: sidechain_primitives::SidechainBlockConfirmation
     **/
    SidechainPrimitivesSidechainBlockConfirmation: {
        blockNumber: string;
        blockHeaderHash: string;
    };
    /**
     * Lookup560: pallet_sidechain::pallet::Error<T>
     **/
    PalletSidechainError: {
        _enum: string[];
    };
    /**
     * Lookup563: pallet_teeracle::pallet::Error<T>
     **/
    PalletTeeracleError: {
        _enum: string[];
    };
    /**
     * Lookup565: pallet_identity_management_mock::pallet::Error<T>
     **/
    PalletIdentityManagementMockError: {
        _enum: string[];
    };
    /**
     * Lookup566: pallet_sudo::pallet::Error<T>
     **/
    PalletSudoError: {
        _enum: string[];
    };
    /**
     * Lookup568: sp_runtime::MultiSignature
     **/
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: string;
            Sr25519: string;
            Ecdsa: string;
        };
    };
    /**
     * Lookup569: sp_core::ed25519::Signature
     **/
    SpCoreEd25519Signature: string;
    /**
     * Lookup571: sp_core::sr25519::Signature
     **/
    SpCoreSr25519Signature: string;
    /**
     * Lookup572: sp_core::ecdsa::Signature
     **/
    SpCoreEcdsaSignature: string;
    /**
     * Lookup575: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
     **/
    FrameSystemExtensionsCheckNonZeroSender: string;
    /**
     * Lookup576: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
     **/
    FrameSystemExtensionsCheckSpecVersion: string;
    /**
     * Lookup577: frame_system::extensions::check_tx_version::CheckTxVersion<T>
     **/
    FrameSystemExtensionsCheckTxVersion: string;
    /**
     * Lookup578: frame_system::extensions::check_genesis::CheckGenesis<T>
     **/
    FrameSystemExtensionsCheckGenesis: string;
    /**
     * Lookup581: frame_system::extensions::check_nonce::CheckNonce<T>
     **/
    FrameSystemExtensionsCheckNonce: string;
    /**
     * Lookup582: frame_system::extensions::check_weight::CheckWeight<T>
     **/
    FrameSystemExtensionsCheckWeight: string;
    /**
     * Lookup583: pallet_transaction_payment::ChargeTransactionPayment<T>
     **/
    PalletTransactionPaymentChargeTransactionPayment: string;
};
export default _default;
