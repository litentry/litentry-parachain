import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { AccountId } from '@polkadot/types/interfaces';
import type { DeriveSocietyMember } from '../types';
/**
 * @description Get the member info for a society
 */
export declare function member(instanceId: string, api: ApiInterfaceRx): (accountId: AccountId) => Observable<DeriveSocietyMember>;
