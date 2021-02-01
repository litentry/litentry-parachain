import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { AccountId, Address } from '@polkadot/types/interfaces';
import type { DeriveAccountFlags } from '../types';
/**
 * @name info
 * @description Returns account membership flags
 */
export declare function flags(instanceId: string, api: ApiInterfaceRx): (address?: AccountId | Address | string | null) => Observable<DeriveAccountFlags>;
