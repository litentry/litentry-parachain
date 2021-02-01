import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { ReferendumInfoFinished } from '@polkadot/types/interfaces';
export declare function referendumsFinished(instanceId: string, api: ApiInterfaceRx): () => Observable<ReferendumInfoFinished[]>;
