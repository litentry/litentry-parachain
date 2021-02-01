import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { DeriveStakingKeys } from './types';
export declare function keys(instanceId: string, api: ApiInterfaceRx): (stashId: Uint8Array | string) => Observable<DeriveStakingKeys>;
export declare function keysMulti(instanceId: string, api: ApiInterfaceRx): (stashIds: (Uint8Array | string)[]) => Observable<DeriveStakingKeys[]>;
