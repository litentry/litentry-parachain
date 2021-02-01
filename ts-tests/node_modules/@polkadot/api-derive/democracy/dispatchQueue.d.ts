import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { DeriveDispatch } from '../types';
export declare function dispatchQueue(instanceId: string, api: ApiInterfaceRx): () => Observable<DeriveDispatch[]>;
