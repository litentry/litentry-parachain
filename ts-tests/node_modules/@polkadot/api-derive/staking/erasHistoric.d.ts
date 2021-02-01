import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { EraIndex } from '@polkadot/types/interfaces';
export declare function erasHistoric(instanceId: string, api: ApiInterfaceRx): (withActive: boolean) => Observable<EraIndex[]>;
