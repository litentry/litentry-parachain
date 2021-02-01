import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { DeriveProposalExternal } from '../types';
export declare function nextExternal(instanceId: string, api: ApiInterfaceRx): () => Observable<DeriveProposalExternal | null>;
