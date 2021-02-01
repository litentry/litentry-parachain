import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { AccountId } from '@polkadot/types/interfaces';
import type { DeriveStakingValidators } from '../types';
export declare function nextElected(instanceId: string, api: ApiInterfaceRx): () => Observable<AccountId[]>;
/**
 * @description Retrieve latest list of validators
 */
export declare function validators(instanceId: string, api: ApiInterfaceRx): () => Observable<DeriveStakingValidators>;
