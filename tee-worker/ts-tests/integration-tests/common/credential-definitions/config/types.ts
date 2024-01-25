import { CorePrimitivesAssertion, CorePrimitivesNetworkWeb3Network } from 'parachain-api';
import type { Codec } from '@polkadot/types-codec/types';
import type { U8aLike } from '@polkadot/util/types';
import { DataProvider } from './data-providers';

type AssertionGenericPayload = string | Array<string | number | Codec | U8aLike> | Record<string, unknown>;

export interface CredentialDefinition {
    id: string;
    name: string;
    description: string;
    assertion: {
        id: CorePrimitivesAssertion['type'];
        payload: AssertionGenericPayload;
    };
    dataProvider: DataProvider;
    network: CorePrimitivesNetworkWeb3Network['type'];
    mockDid: string;
    mockWeb3Network: string;
}
