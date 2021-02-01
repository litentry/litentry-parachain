import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { DeriveSessionIndexes } from '../types';
export declare function indexes(instanceId: string, api: ApiInterfaceRx): () => Observable<DeriveSessionIndexes>;
