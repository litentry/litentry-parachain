import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { DeriveSocietyCandidate } from '../types';
/**
 * @description Get the candidate info for a society
 */
export declare function candidates(instanceId: string, api: ApiInterfaceRx): () => Observable<DeriveSocietyCandidate[]>;
