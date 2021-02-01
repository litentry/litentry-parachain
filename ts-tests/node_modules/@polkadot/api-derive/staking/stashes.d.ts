import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { AccountId } from '@polkadot/types/interfaces';
/**
 * @description Retrieve the list of all validator stashes
 */
export declare function stashes(instanceId: string, api: ApiInterfaceRx): () => Observable<AccountId[]>;
