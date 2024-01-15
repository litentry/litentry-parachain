import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { u8aToHex, bufferToU8a } from '@polkadot/util';
import { buildIdentityFromKeypair, initIntegrationTestContext, PolkadotSigner } from './common/utils';
import { assertIsInSidechainBlock, assertVc } from './common/utils/assertion';
import {
    getSidechainNonce,
    createSignedTrustedCallLinkIdentity,
    getTeeShieldingKey,
    sendRequestFromTrustedCall,
    createSignedTrustedCallRequestVc,
} from './common/di-utils'; // @fixme move to a better place
import { buildIdentityHelper, buildValidations } from './common/utils';
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import { CorePrimitivesIdentity } from 'parachain-api';
import { subscribeToEventsWithExtHash } from './common/transactions';
import { defaultAssertions, unconfiguredAssertions } from './common/utils/vc-helper';
import { LitentryValidationData, Web3Network } from 'parachain-api';
import { Vec } from '@polkadot/types';

describe('Test Vc (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubstrateIdentity: CorePrimitivesIdentity = undefined as any;

    // Alice links:
    // - a `mock_user` twitter
    // - alice's evm identity
    // - alice's bitcoin identity]
    //
    // We need this linking to not have empty eligible identities for any vc request
    const linkIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
        validation: LitentryValidationData;
        networks: Vec<Web3Network>;
    }[] = [];
    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_ENDPOINT!, // @fixme evil assertion; centralize env access
            process.env.NODE_ENDPOINT! // @fixme evil assertion; centralize env access
        );
        teeShieldingKey = await getTeeShieldingKey(context);
        aliceSubstrateIdentity = await buildIdentityFromKeypair(
            new PolkadotSigner(context.substrateWallet.alice),
            context
        );
    });

});
