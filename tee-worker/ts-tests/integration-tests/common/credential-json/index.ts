import { CorePrimitivesAssertion, CorePrimitivesNetworkWeb3Network } from 'parachain-api';
import type { Codec } from '@polkadot/types-codec/types';
import type { U8aLike } from '@polkadot/util/types';
type DataProvider = {
    id: string;
    name: string;
    url: string;
};

type AssertionGenericPayload = string | Array<string | number | Codec | U8aLike> | Record<string, unknown>;

import vip3Json from './vip3-credential-test.json' assert { type: 'json' };
export const vip3CredentialJson = vip3Json as unknown as CredentialDefinition[];

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
    expectedCredentialValue: boolean;
}
