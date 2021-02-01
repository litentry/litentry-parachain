import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { DeriveCouncilVotes } from '../types';
export declare function votes(instanceId: string, api: ApiInterfaceRx): () => Observable<DeriveCouncilVotes>;
