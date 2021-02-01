import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { EraIndex } from '@polkadot/types/interfaces';
import type { DeriveEraRewards } from '../types';
export declare function _erasRewards(instanceId: string, api: ApiInterfaceRx): (eras: EraIndex[], withActive: boolean) => Observable<DeriveEraRewards[]>;
export declare function erasRewards(instanceId: string, api: ApiInterfaceRx): (withActive?: boolean) => Observable<DeriveEraRewards[]>;
