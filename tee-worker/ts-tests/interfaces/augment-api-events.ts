// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/events';

import type { ApiTypes, AugmentedEvent } from '@polkadot/api-base/types';
import type { Bytes, Null, Option, Result, Text, U8aFixed, Vec, bool, u128, u16, u32, u64 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H256 } from '@polkadot/types/interfaces/runtime';
import type { ClaimsPrimitivesEthereumAddress, CorePrimitivesAssertion, CorePrimitivesErrorErrorDetail, CorePrimitivesKeyAesOutput, FrameSupportDispatchDispatchInfo, FrameSupportTokensMiscBalanceStatus, IntegriteeNodeRuntimeProxyType, PalletMultisigTimepoint, SpFinalityGrandpaAppPublic, SpRuntimeDispatchError, SubstrateFixedFixedU64 } from '@polkadot/types/lookup';

export type __AugmentedEvent<ApiType extends ApiTypes> = AugmentedEvent<ApiType>;

declare module '@polkadot/api-base/types/events' {
  interface AugmentedEvents<ApiType extends ApiTypes> {
    balances: {
      /**
       * A balance was set by root.
       **/
      BalanceSet: AugmentedEvent<ApiType, [who: AccountId32, free: u128, reserved: u128], { who: AccountId32, free: u128, reserved: u128 }>;
      /**
       * Some amount was deposited (e.g. for transaction fees).
       **/
      Deposit: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * An account was removed whose balance was non-zero but below ExistentialDeposit,
       * resulting in an outright loss.
       **/
      DustLost: AugmentedEvent<ApiType, [account: AccountId32, amount: u128], { account: AccountId32, amount: u128 }>;
      /**
       * An account was created with some free balance.
       **/
      Endowed: AugmentedEvent<ApiType, [account: AccountId32, freeBalance: u128], { account: AccountId32, freeBalance: u128 }>;
      /**
       * Some balance was reserved (moved from free to reserved).
       **/
      Reserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some balance was moved from the reserve of the first account to the second account.
       * Final argument indicates the destination balance type.
       **/
      ReserveRepatriated: AugmentedEvent<ApiType, [from: AccountId32, to: AccountId32, amount: u128, destinationStatus: FrameSupportTokensMiscBalanceStatus], { from: AccountId32, to: AccountId32, amount: u128, destinationStatus: FrameSupportTokensMiscBalanceStatus }>;
      /**
       * Some amount was removed from the account (e.g. for misbehavior).
       **/
      Slashed: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Transfer succeeded.
       **/
      Transfer: AugmentedEvent<ApiType, [from: AccountId32, to: AccountId32, amount: u128], { from: AccountId32, to: AccountId32, amount: u128 }>;
      /**
       * Some balance was unreserved (moved from reserved to free).
       **/
      Unreserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some amount was withdrawn from the account (e.g. for transaction fees).
       **/
      Withdraw: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    claims: {
      /**
       * Someone claimed some TEERs. `[who, ethereum_address, amount]`
       **/
      Claimed: AugmentedEvent<ApiType, [AccountId32, ClaimsPrimitivesEthereumAddress, u128]>;
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
      Closed: AugmentedEvent<ApiType, [proposalHash: H256, yes: u32, no: u32], { proposalHash: H256, yes: u32, no: u32 }>;
      /**
       * A motion was not approved by the required threshold.
       **/
      Disapproved: AugmentedEvent<ApiType, [proposalHash: H256], { proposalHash: H256 }>;
      /**
       * A motion was executed; result will be `Ok` if it returned without error.
       **/
      Executed: AugmentedEvent<ApiType, [proposalHash: H256, result: Result<Null, SpRuntimeDispatchError>], { proposalHash: H256, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A single member did some action; result will be `Ok` if it returned without error.
       **/
      MemberExecuted: AugmentedEvent<ApiType, [proposalHash: H256, result: Result<Null, SpRuntimeDispatchError>], { proposalHash: H256, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A motion (given hash) has been proposed (by given account) with a threshold (given
       * `MemberCount`).
       **/
      Proposed: AugmentedEvent<ApiType, [account: AccountId32, proposalIndex: u32, proposalHash: H256, threshold: u32], { account: AccountId32, proposalIndex: u32, proposalHash: H256, threshold: u32 }>;
      /**
       * A motion (given hash) has been voted on by given account, leaving
       * a tally (yes votes and no votes given respectively as `MemberCount`).
       **/
      Voted: AugmentedEvent<ApiType, [account: AccountId32, proposalHash: H256, voted: bool, yes: u32, no: u32], { account: AccountId32, proposalHash: H256, voted: bool, yes: u32, no: u32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    grandpa: {
      /**
       * New authority set has been applied.
       **/
      NewAuthorities: AugmentedEvent<ApiType, [authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>], { authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>> }>;
      /**
       * Current authority set has been paused.
       **/
      Paused: AugmentedEvent<ApiType, []>;
      /**
       * Current authority set has been resumed.
       **/
      Resumed: AugmentedEvent<ApiType, []>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    identityManagement: {
      CreateIdentityFailed: AugmentedEvent<ApiType, [account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256], { account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256 }>;
      CreateIdentityRequested: AugmentedEvent<ApiType, [shard: H256], { shard: H256 }>;
      DelegateeAdded: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
      DelegateeRemoved: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
      IdentityCreated: AugmentedEvent<ApiType, [account: AccountId32, identity: CorePrimitivesKeyAesOutput, code: CorePrimitivesKeyAesOutput, reqExtHash: H256], { account: AccountId32, identity: CorePrimitivesKeyAesOutput, code: CorePrimitivesKeyAesOutput, reqExtHash: H256 }>;
      IdentityRemoved: AugmentedEvent<ApiType, [account: AccountId32, identity: CorePrimitivesKeyAesOutput, reqExtHash: H256], { account: AccountId32, identity: CorePrimitivesKeyAesOutput, reqExtHash: H256 }>;
      IdentityVerified: AugmentedEvent<ApiType, [account: AccountId32, identity: CorePrimitivesKeyAesOutput, idGraph: CorePrimitivesKeyAesOutput, reqExtHash: H256], { account: AccountId32, identity: CorePrimitivesKeyAesOutput, idGraph: CorePrimitivesKeyAesOutput, reqExtHash: H256 }>;
      ImportScheduledEnclaveFailed: AugmentedEvent<ApiType, []>;
      RemoveIdentityFailed: AugmentedEvent<ApiType, [account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256], { account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256 }>;
      RemoveIdentityRequested: AugmentedEvent<ApiType, [shard: H256], { shard: H256 }>;
      SetUserShieldingKeyFailed: AugmentedEvent<ApiType, [account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256], { account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256 }>;
      SetUserShieldingKeyRequested: AugmentedEvent<ApiType, [shard: H256], { shard: H256 }>;
      UnclassifiedError: AugmentedEvent<ApiType, [account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256], { account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256 }>;
      UserShieldingKeySet: AugmentedEvent<ApiType, [account: AccountId32, idGraph: CorePrimitivesKeyAesOutput, reqExtHash: H256], { account: AccountId32, idGraph: CorePrimitivesKeyAesOutput, reqExtHash: H256 }>;
      VerifyIdentityFailed: AugmentedEvent<ApiType, [account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256], { account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256 }>;
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
      MultisigApproval: AugmentedEvent<ApiType, [approving: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed], { approving: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed }>;
      /**
       * A multisig operation has been cancelled.
       **/
      MultisigCancelled: AugmentedEvent<ApiType, [cancelling: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed], { cancelling: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed }>;
      /**
       * A multisig operation has been executed.
       **/
      MultisigExecuted: AugmentedEvent<ApiType, [approving: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed, result: Result<Null, SpRuntimeDispatchError>], { approving: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A new multisig operation has begun.
       **/
      NewMultisig: AugmentedEvent<ApiType, [approving: AccountId32, multisig: AccountId32, callHash: U8aFixed], { approving: AccountId32, multisig: AccountId32, callHash: U8aFixed }>;
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
      Announced: AugmentedEvent<ApiType, [real: AccountId32, proxy: AccountId32, callHash: H256], { real: AccountId32, proxy: AccountId32, callHash: H256 }>;
      /**
       * A proxy was added.
       **/
      ProxyAdded: AugmentedEvent<ApiType, [delegator: AccountId32, delegatee: AccountId32, proxyType: IntegriteeNodeRuntimeProxyType, delay: u32], { delegator: AccountId32, delegatee: AccountId32, proxyType: IntegriteeNodeRuntimeProxyType, delay: u32 }>;
      /**
       * A proxy was executed correctly, with the given.
       **/
      ProxyExecuted: AugmentedEvent<ApiType, [result: Result<Null, SpRuntimeDispatchError>], { result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A proxy was removed.
       **/
      ProxyRemoved: AugmentedEvent<ApiType, [delegator: AccountId32, delegatee: AccountId32, proxyType: IntegriteeNodeRuntimeProxyType, delay: u32], { delegator: AccountId32, delegatee: AccountId32, proxyType: IntegriteeNodeRuntimeProxyType, delay: u32 }>;
      /**
       * A pure account has been created by new proxy with given
       * disambiguation index and proxy type.
       **/
      PureCreated: AugmentedEvent<ApiType, [pure: AccountId32, who: AccountId32, proxyType: IntegriteeNodeRuntimeProxyType, disambiguationIndex: u16], { pure: AccountId32, who: AccountId32, proxyType: IntegriteeNodeRuntimeProxyType, disambiguationIndex: u16 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    scheduler: {
      /**
       * The call for the provided hash was not found so the task has been aborted.
       **/
      CallUnavailable: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<U8aFixed>], { task: ITuple<[u32, u32]>, id: Option<U8aFixed> }>;
      /**
       * Canceled some task.
       **/
      Canceled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32, index: u32 }>;
      /**
       * Dispatched some task.
       **/
      Dispatched: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<U8aFixed>, result: Result<Null, SpRuntimeDispatchError>], { task: ITuple<[u32, u32]>, id: Option<U8aFixed>, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * The given task was unable to be renewed since the agenda is full at that block.
       **/
      PeriodicFailed: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<U8aFixed>], { task: ITuple<[u32, u32]>, id: Option<U8aFixed> }>;
      /**
       * The given task can never be executed since it is overweight.
       **/
      PermanentlyOverweight: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<U8aFixed>], { task: ITuple<[u32, u32]>, id: Option<U8aFixed> }>;
      /**
       * Scheduled some task.
       **/
      Scheduled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32, index: u32 }>;
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
      Sudid: AugmentedEvent<ApiType, [sudoResult: Result<Null, SpRuntimeDispatchError>], { sudoResult: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A sudo just took place. \[result\]
       **/
      SudoAsDone: AugmentedEvent<ApiType, [sudoResult: Result<Null, SpRuntimeDispatchError>], { sudoResult: Result<Null, SpRuntimeDispatchError> }>;
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
      ExtrinsicFailed: AugmentedEvent<ApiType, [dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportDispatchDispatchInfo], { dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportDispatchDispatchInfo }>;
      /**
       * An extrinsic completed successfully.
       **/
      ExtrinsicSuccess: AugmentedEvent<ApiType, [dispatchInfo: FrameSupportDispatchDispatchInfo], { dispatchInfo: FrameSupportDispatchDispatchInfo }>;
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
      Remarked: AugmentedEvent<ApiType, [sender: AccountId32, hash_: H256], { sender: AccountId32, hash_: H256 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    teeracle: {
      AddedToWhitelist: AugmentedEvent<ApiType, [Text, U8aFixed]>;
      ExchangeRateDeleted: AugmentedEvent<ApiType, [Text, Text]>;
      /**
       * The exchange rate of trading pair was set/updated with value from source.
       * \[data_source], [trading_pair], [new value\]
       **/
      ExchangeRateUpdated: AugmentedEvent<ApiType, [Text, Text, Option<SubstrateFixedFixedU64>]>;
      OracleUpdated: AugmentedEvent<ApiType, [Text, Text]>;
      RemovedFromWhitelist: AugmentedEvent<ApiType, [Text, U8aFixed]>;
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
      PublishedHash: AugmentedEvent<ApiType, [mrEnclave: U8aFixed, hash_: H256, data: Bytes], { mrEnclave: U8aFixed, hash_: H256, data: Bytes }>;
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
    transactionPayment: {
      /**
       * A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,
       * has been paid by `who`.
       **/
      TransactionFeePaid: AugmentedEvent<ApiType, [who: AccountId32, actualFee: u128, tip: u128], { who: AccountId32, actualFee: u128, tip: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    treasury: {
      /**
       * Some funds have been allocated.
       **/
      Awarded: AugmentedEvent<ApiType, [proposalIndex: u32, award: u128, account: AccountId32], { proposalIndex: u32, award: u128, account: AccountId32 }>;
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
      Rejected: AugmentedEvent<ApiType, [proposalIndex: u32, slashed: u128], { proposalIndex: u32, slashed: u128 }>;
      /**
       * Spending has finished; this is the amount that rolls over until next spend.
       **/
      Rollover: AugmentedEvent<ApiType, [rolloverBalance: u128], { rolloverBalance: u128 }>;
      /**
       * A new spend proposal has been approved.
       **/
      SpendApproved: AugmentedEvent<ApiType, [proposalIndex: u32, amount: u128, beneficiary: AccountId32], { proposalIndex: u32, amount: u128, beneficiary: AccountId32 }>;
      /**
       * We have ended a spend period and will now allocate funds.
       **/
      Spending: AugmentedEvent<ApiType, [budgetRemaining: u128], { budgetRemaining: u128 }>;
      /**
       * The inactive funds of the pallet have been updated.
       **/
      UpdatedInactive: AugmentedEvent<ApiType, [reactivated: u128, deactivated: u128], { reactivated: u128, deactivated: u128 }>;
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
      BatchInterrupted: AugmentedEvent<ApiType, [index: u32, error: SpRuntimeDispatchError], { index: u32, error: SpRuntimeDispatchError }>;
      /**
       * A call was dispatched.
       **/
      DispatchedAs: AugmentedEvent<ApiType, [result: Result<Null, SpRuntimeDispatchError>], { result: Result<Null, SpRuntimeDispatchError> }>;
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
      AdminChanged: AugmentedEvent<ApiType, [oldAdmin: Option<AccountId32>, newAdmin: Option<AccountId32>], { oldAdmin: Option<AccountId32>, newAdmin: Option<AccountId32> }>;
      RequestVCFailed: AugmentedEvent<ApiType, [account: Option<AccountId32>, assertion: CorePrimitivesAssertion, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256], { account: Option<AccountId32>, assertion: CorePrimitivesAssertion, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256 }>;
      SchemaActivated: AugmentedEvent<ApiType, [account: AccountId32, shard: H256, index: u64], { account: AccountId32, shard: H256, index: u64 }>;
      SchemaDisabled: AugmentedEvent<ApiType, [account: AccountId32, shard: H256, index: u64], { account: AccountId32, shard: H256, index: u64 }>;
      SchemaIssued: AugmentedEvent<ApiType, [account: AccountId32, shard: H256, index: u64], { account: AccountId32, shard: H256, index: u64 }>;
      SchemaRevoked: AugmentedEvent<ApiType, [account: AccountId32, shard: H256, index: u64], { account: AccountId32, shard: H256, index: u64 }>;
      UnclassifiedError: AugmentedEvent<ApiType, [account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256], { account: Option<AccountId32>, detail: CorePrimitivesErrorErrorDetail, reqExtHash: H256 }>;
      VCDisabled: AugmentedEvent<ApiType, [account: AccountId32, index: H256], { account: AccountId32, index: H256 }>;
      VCIssued: AugmentedEvent<ApiType, [account: AccountId32, assertion: CorePrimitivesAssertion, index: H256, vc: CorePrimitivesKeyAesOutput, reqExtHash: H256], { account: AccountId32, assertion: CorePrimitivesAssertion, index: H256, vc: CorePrimitivesKeyAesOutput, reqExtHash: H256 }>;
      VCRegistryCleared: AugmentedEvent<ApiType, []>;
      VCRegistryItemAdded: AugmentedEvent<ApiType, [account: AccountId32, assertion: CorePrimitivesAssertion, index: H256], { account: AccountId32, assertion: CorePrimitivesAssertion, index: H256 }>;
      VCRegistryItemRemoved: AugmentedEvent<ApiType, [index: H256], { index: H256 }>;
      VCRequested: AugmentedEvent<ApiType, [account: AccountId32, shard: H256, assertion: CorePrimitivesAssertion], { account: AccountId32, shard: H256, assertion: CorePrimitivesAssertion }>;
      VCRevoked: AugmentedEvent<ApiType, [account: AccountId32, index: H256], { account: AccountId32, index: H256 }>;
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
      VestingUpdated: AugmentedEvent<ApiType, [account: AccountId32, unvested: u128], { account: AccountId32, unvested: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
  } // AugmentedEvents
} // declare module
