import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { EraIndex } from '@polkadot/types/interfaces';
import type { DeriveEraPoints } from '../types';
export declare function _erasPoints(instanceId: string, api: ApiInterfaceRx): (eras: EraIndex[], withActive: boolean) => Observable<DeriveEraPoints[]>;
export declare function erasPoints(instanceId: string, api: ApiInterfaceRx): (withActive?: boolean) => Observable<DeriveEraPoints[]>;
