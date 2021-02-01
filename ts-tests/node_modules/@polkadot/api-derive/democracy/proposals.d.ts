import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { DeriveProposal } from '../types';
export declare function proposals(instanceId: string, api: ApiInterfaceRx): () => Observable<DeriveProposal[]>;
