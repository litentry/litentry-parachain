import { CorePrimitivesAssertion, CorePrimitivesNetworkWeb3Network } from 'parachain-api';
import type { Codec } from '@polkadot/types-codec/types';
import type { U8aLike } from '@polkadot/util/types';

type AssertionGenericPayload = string | Array<string | number | Codec | U8aLike> | Record<string, unknown>;

import vip3Json from './vip3-credential-test.json' assert { type: 'json' };
import achainableJson from './achainable-credential-test.json' assert { type: 'json' };
import noderealJson from './nodereal-credential-test.json' assert { type: 'json' };
import discordJson from './discord-credential-test.json' assert { type: 'json' };
import litentryJson from './litentry-credential-test.json' assert { type: 'json' };
import twitterJson from './twitter-credential-test.json' assert { type: 'json' };
import oneblockJson from './oneblock-credential-test.json' assert { type: 'json' };
export const vip3CredentialJson = vip3Json as unknown as CredentialDefinition[];
export const achainableCredentialJson = achainableJson as unknown as CredentialDefinition[];
export const noderealCredentialJson = noderealJson as unknown as CredentialDefinition[];
export const discordCredentialJson = discordJson as unknown as CredentialDefinition[];
export const litentryCredentialJson = litentryJson as unknown as CredentialDefinition[];
export const twitterCredentialJson = twitterJson as unknown as CredentialDefinition[];
export const oneblockCredentialJson = oneblockJson as unknown as CredentialDefinition[];
export const credentialsJson = [
    ...vip3CredentialJson,
    ...achainableCredentialJson,
    ...noderealCredentialJson,
    ...litentryCredentialJson,
    ...twitterCredentialJson,
    ...oneblockCredentialJson,
    ...discordCredentialJson
];

export interface CredentialDefinition {
    id: string;
    name: string;
    description: string;
    assertion: {
        id: CorePrimitivesAssertion['type'];
        payload: AssertionGenericPayload;
    };
    dataProvider: string;
    network: CorePrimitivesNetworkWeb3Network['type'];
    mockDid: string;
    mockWeb3Network: string;
    expectedCredentialValue: boolean;
}
