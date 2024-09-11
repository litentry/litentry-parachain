import { CorePrimitivesAssertion, CorePrimitivesNetworkWeb3Network } from 'parachain-api';
import type { Codec } from '@polkadot/types-codec/types';
import type { U8aLike } from '@polkadot/util/types';

type AssertionGenericPayload = string | Array<string | number | Codec | U8aLike> | Record<string, unknown>;

import vip3Json from './vip3.json' assert { type: 'json' };
import achainableJson from './achainable.json' assert { type: 'json' };
import noderealJson from './nodereal.json' assert { type: 'json' };
import discordJson from './discord.json' assert { type: 'json' };
import litentryJson from './litentry.json' assert { type: 'json' };
import twitterJson from './twitter.json' assert { type: 'json' };
import oneblockJson from './oneblock.json' assert { type: 'json' };
import geniidataJson from './geniidata.json' assert { type: 'json' };
import moralisJson from './moralis.json' assert { type: 'json' };
import blockchaininfoJson from './blockchaininfo.json' assert { type: 'json' };
export const vip3 = vip3Json as unknown as CredentialDefinition[];
export const achainable = achainableJson as unknown as CredentialDefinition[];
export const nodereal = noderealJson as unknown as CredentialDefinition[];
export const discord = discordJson as unknown as CredentialDefinition[];
export const litentry = litentryJson as unknown as CredentialDefinition[];
export const twitter = twitterJson as unknown as CredentialDefinition[];
export const oneblock = oneblockJson as unknown as CredentialDefinition[];
export const geniidata = geniidataJson as unknown as CredentialDefinition[];
export const moralis = moralisJson as unknown as CredentialDefinition[];
export const blockchaininfo = blockchaininfoJson as unknown as CredentialDefinition[];
export const credentialsJson = [
    // ...nodereal,
    // ...geniidata,
    ...moralis,
    ...achainable,
    ...litentry,
    ...twitter,
    ...oneblock,
    ...discord,
    ...vip3,
    ...blockchaininfo,
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
