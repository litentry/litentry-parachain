import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { DeriveSociety } from '../types';
/**
 * @description Get the overall info for a society
 */
export declare function info(instanceId: string, api: ApiInterfaceRx): () => Observable<DeriveSociety>;
