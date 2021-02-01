import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { DeriveTreasuryProposals } from '../types';
/**
 * @description Retrieve all active and approved treasury proposals, along with their info
 */
export declare function proposals(instanceId: string, api: ApiInterfaceRx): () => Observable<DeriveTreasuryProposals>;
