import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { DeriveReferendum } from '../types';
export declare function referendumsActive(instanceId: string, api: ApiInterfaceRx): () => Observable<DeriveReferendum[]>;
