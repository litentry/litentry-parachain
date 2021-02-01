import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { DeriveReferendumExt } from '../types';
export declare function referendums(instanceId: string, api: ApiInterfaceRx): () => Observable<DeriveReferendumExt[]>;
