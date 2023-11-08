import "@polkadot/api-base/types/submittable";
import type { ApiTypes, AugmentedSubmittable, SubmittableExtrinsic, SubmittableExtrinsicFunction } from "@polkadot/api-base/types";
import type { Bytes, Compact, Vec, bool, u128, u32, u64 } from "@polkadot/types-codec";
import type { AnyNumber, IMethod, ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, Call, MultiAddress } from "@polkadot/types/interfaces/runtime";
import type { CorePrimitivesNetworkWeb3Network, LitentryPrimitivesIdentity, SpRuntimeHeader, SpWeightsWeightV2Weight } from "@polkadot/types/lookup";
export type __AugmentedSubmittable = AugmentedSubmittable<() => unknown>;
export type __SubmittableExtrinsic<ApiType extends ApiTypes> = SubmittableExtrinsic<ApiType>;
export type __SubmittableExtrinsicFunction<ApiType extends ApiTypes> = SubmittableExtrinsicFunction<ApiType>;
declare module "@polkadot/api-base/types/submittable" {
    interface AugmentedSubmittables<ApiType extends ApiTypes> {
        balances: {
            /**
             * Set the regular balance of a given account.
             *
             * The dispatch origin for this call is `root`.
             **/
            forceSetBalance: AugmentedSubmittable<(who: MultiAddress | {
                Id: any;
            } | {
                Index: any;
            } | {
                Raw: any;
            } | {
                Address32: any;
            } | {
                Address20: any;
            } | string | Uint8Array, newFree: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                MultiAddress,
                Compact<u128>
            ]>;
            /**
             * Exactly as `transfer_allow_death`, except the origin must be root and the source account
             * may be specified.
             **/
            forceTransfer: AugmentedSubmittable<(source: MultiAddress | {
                Id: any;
            } | {
                Index: any;
            } | {
                Raw: any;
            } | {
                Address32: any;
            } | {
                Address20: any;
            } | string | Uint8Array, dest: MultiAddress | {
                Id: any;
            } | {
                Index: any;
            } | {
                Raw: any;
            } | {
                Address32: any;
            } | {
                Address20: any;
            } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                MultiAddress,
                MultiAddress,
                Compact<u128>
            ]>;
            /**
             * Unreserve some balance from a user by force.
             *
             * Can only be called by ROOT.
             **/
            forceUnreserve: AugmentedSubmittable<(who: MultiAddress | {
                Id: any;
            } | {
                Index: any;
            } | {
                Raw: any;
            } | {
                Address32: any;
            } | {
                Address20: any;
            } | string | Uint8Array, amount: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                MultiAddress,
                u128
            ]>;
            /**
             * Set the regular balance of a given account; it also takes a reserved balance but this
             * must be the same as the account's current reserved balance.
             *
             * The dispatch origin for this call is `root`.
             *
             * WARNING: This call is DEPRECATED! Use `force_set_balance` instead.
             **/
            setBalanceDeprecated: AugmentedSubmittable<(who: MultiAddress | {
                Id: any;
            } | {
                Index: any;
            } | {
                Raw: any;
            } | {
                Address32: any;
            } | {
                Address20: any;
            } | string | Uint8Array, newFree: Compact<u128> | AnyNumber | Uint8Array, oldReserved: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                MultiAddress,
                Compact<u128>,
                Compact<u128>
            ]>;
            /**
             * Alias for `transfer_allow_death`, provided only for name-wise compatibility.
             *
             * WARNING: DEPRECATED! Will be released in approximately 3 months.
             **/
            transfer: AugmentedSubmittable<(dest: MultiAddress | {
                Id: any;
            } | {
                Index: any;
            } | {
                Raw: any;
            } | {
                Address32: any;
            } | {
                Address20: any;
            } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                MultiAddress,
                Compact<u128>
            ]>;
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
             * keep the sender account alive (true).
             **/
            transferAll: AugmentedSubmittable<(dest: MultiAddress | {
                Id: any;
            } | {
                Index: any;
            } | {
                Raw: any;
            } | {
                Address32: any;
            } | {
                Address20: any;
            } | string | Uint8Array, keepAlive: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                MultiAddress,
                bool
            ]>;
            /**
             * Transfer some liquid free balance to another account.
             *
             * `transfer_allow_death` will set the `FreeBalance` of the sender and receiver.
             * If the sender's account is below the existential deposit as a result
             * of the transfer, the account will be reaped.
             *
             * The dispatch origin for this call must be `Signed` by the transactor.
             **/
            transferAllowDeath: AugmentedSubmittable<(dest: MultiAddress | {
                Id: any;
            } | {
                Index: any;
            } | {
                Raw: any;
            } | {
                Address32: any;
            } | {
                Address20: any;
            } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                MultiAddress,
                Compact<u128>
            ]>;
            /**
             * Same as the [`transfer_allow_death`] call, but with a check that the transfer will not
             * kill the origin account.
             *
             * 99% of the time you want [`transfer_allow_death`] instead.
             *
             * [`transfer_allow_death`]: struct.Pallet.html#method.transfer
             **/
            transferKeepAlive: AugmentedSubmittable<(dest: MultiAddress | {
                Id: any;
            } | {
                Index: any;
            } | {
                Raw: any;
            } | {
                Address32: any;
            } | {
                Address20: any;
            } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                MultiAddress,
                Compact<u128>
            ]>;
            /**
             * Upgrade a specified account.
             *
             * - `origin`: Must be `Signed`.
             * - `who`: The account to be upgraded.
             *
             * This will waive the transaction fee if at least all but 10% of the accounts needed to
             * be upgraded. (We let some not have to be upgraded just in order to allow for the
             * possibililty of churn).
             **/
            upgradeAccounts: AugmentedSubmittable<(who: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [
                Vec<AccountId32>
            ]>;
        };
        identityManagement: {
            activateIdentity: AugmentedSubmittable<(who: LitentryPrimitivesIdentity | {
                Twitter: any;
            } | {
                Discord: any;
            } | {
                Github: any;
            } | {
                Substrate: any;
            } | {
                Evm: any;
            } | string | Uint8Array, identity: LitentryPrimitivesIdentity | {
                Twitter: any;
            } | {
                Discord: any;
            } | {
                Github: any;
            } | {
                Substrate: any;
            } | {
                Evm: any;
            } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                LitentryPrimitivesIdentity,
                LitentryPrimitivesIdentity
            ]>;
            deactivateIdentity: AugmentedSubmittable<(who: LitentryPrimitivesIdentity | {
                Twitter: any;
            } | {
                Discord: any;
            } | {
                Github: any;
            } | {
                Substrate: any;
            } | {
                Evm: any;
            } | string | Uint8Array, identity: LitentryPrimitivesIdentity | {
                Twitter: any;
            } | {
                Discord: any;
            } | {
                Github: any;
            } | {
                Substrate: any;
            } | {
                Evm: any;
            } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                LitentryPrimitivesIdentity,
                LitentryPrimitivesIdentity
            ]>;
            linkIdentity: AugmentedSubmittable<(who: LitentryPrimitivesIdentity | {
                Twitter: any;
            } | {
                Discord: any;
            } | {
                Github: any;
            } | {
                Substrate: any;
            } | {
                Evm: any;
            } | string | Uint8Array, identity: LitentryPrimitivesIdentity | {
                Twitter: any;
            } | {
                Discord: any;
            } | {
                Github: any;
            } | {
                Substrate: any;
            } | {
                Evm: any;
            } | string | Uint8Array, web3networks: Vec<CorePrimitivesNetworkWeb3Network> | (CorePrimitivesNetworkWeb3Network | "Polkadot" | "Kusama" | "Litentry" | "Litmus" | "LitentryRococo" | "Khala" | "SubstrateTestnet" | "Ethereum" | "Bsc" | number | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [
                LitentryPrimitivesIdentity,
                LitentryPrimitivesIdentity,
                Vec<CorePrimitivesNetworkWeb3Network>
            ]>;
            setIdentityNetworks: AugmentedSubmittable<(who: LitentryPrimitivesIdentity | {
                Twitter: any;
            } | {
                Discord: any;
            } | {
                Github: any;
            } | {
                Substrate: any;
            } | {
                Evm: any;
            } | string | Uint8Array, identity: LitentryPrimitivesIdentity | {
                Twitter: any;
            } | {
                Discord: any;
            } | {
                Github: any;
            } | {
                Substrate: any;
            } | {
                Evm: any;
            } | string | Uint8Array, web3networks: Vec<CorePrimitivesNetworkWeb3Network> | (CorePrimitivesNetworkWeb3Network | "Polkadot" | "Kusama" | "Litentry" | "Litmus" | "LitentryRococo" | "Khala" | "SubstrateTestnet" | "Ethereum" | "Bsc" | number | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [
                LitentryPrimitivesIdentity,
                LitentryPrimitivesIdentity,
                Vec<CorePrimitivesNetworkWeb3Network>
            ]>;
        };
        parentchain: {
            setBlock: AugmentedSubmittable<(header: SpRuntimeHeader | {
                parentHash?: any;
                number?: any;
                stateRoot?: any;
                extrinsicsRoot?: any;
                digest?: any;
            } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                SpRuntimeHeader
            ]>;
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
            setKey: AugmentedSubmittable<(updated: MultiAddress | {
                Id: any;
            } | {
                Index: any;
            } | {
                Raw: any;
            } | {
                Address32: any;
            } | {
                Address20: any;
            } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                MultiAddress
            ]>;
            /**
             * Authenticates the sudo key and dispatches a function call with `Root` origin.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * ## Complexity
             * - O(1).
             **/
            sudo: AugmentedSubmittable<(call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call]>;
            /**
             * Authenticates the sudo key and dispatches a function call with `Signed` origin from
             * a given account.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * ## Complexity
             * - O(1).
             **/
            sudoAs: AugmentedSubmittable<(who: MultiAddress | {
                Id: any;
            } | {
                Index: any;
            } | {
                Raw: any;
            } | {
                Address32: any;
            } | {
                Address20: any;
            } | string | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                MultiAddress,
                Call
            ]>;
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
            sudoUncheckedWeight: AugmentedSubmittable<(call: Call | IMethod | string | Uint8Array, weight: SpWeightsWeightV2Weight | {
                refTime?: any;
                proofSize?: any;
            } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                Call,
                SpWeightsWeightV2Weight
            ]>;
        };
        system: {
            /**
             * Kill all storage items with a key that starts with the given prefix.
             *
             * **NOTE:** We rely on the Root origin to provide us the number of subkeys under
             * the prefix we are removing to accurately calculate the weight of this function.
             **/
            killPrefix: AugmentedSubmittable<(prefix: Bytes | string | Uint8Array, subkeys: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [
                Bytes,
                u32
            ]>;
            /**
             * Kill some items from storage.
             **/
            killStorage: AugmentedSubmittable<(keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [
                Vec<Bytes>
            ]>;
            /**
             * Make some on-chain remark.
             *
             * ## Complexity
             * - `O(1)`
             **/
            remark: AugmentedSubmittable<(remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
            /**
             * Make some on-chain remark and emit event.
             **/
            remarkWithEvent: AugmentedSubmittable<(remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
            /**
             * Set the new runtime code.
             *
             * ## Complexity
             * - `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`
             **/
            setCode: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
            /**
             * Set the new runtime code without doing any checks of the given `code`.
             *
             * ## Complexity
             * - `O(C)` where `C` length of `code`
             **/
            setCodeWithoutChecks: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
            /**
             * Set the number of pages in the WebAssembly environment's heap.
             **/
            setHeapPages: AugmentedSubmittable<(pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64]>;
            /**
             * Set some items of storage.
             **/
            setStorage: AugmentedSubmittable<(items: Vec<ITuple<[Bytes, Bytes]>> | [Bytes | string | Uint8Array, Bytes | string | Uint8Array][]) => SubmittableExtrinsic<ApiType>, [
                Vec<ITuple<[Bytes, Bytes]>>
            ]>;
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
            set: AugmentedSubmittable<(now: Compact<u64> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u64>]>;
        };
    }
}
//# sourceMappingURL=augment-api-tx.d.ts.map