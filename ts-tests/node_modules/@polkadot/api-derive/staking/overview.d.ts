import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { DeriveStakingOverview } from '../types';
/**
 * @description Retrieve the staking overview, including elected and points earned
 */
export declare function overview(instanceId: string, api: ApiInterfaceRx): () => Observable<DeriveStakingOverview>;
