import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { Hash } from '@polkadot/types/interfaces';
import type { DeriveProposalImage } from '../types';
export declare function preimage(instanceId: string, api: ApiInterfaceRx): (hash: Hash) => Observable<DeriveProposalImage | undefined>;
