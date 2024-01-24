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
import type { IntegrationTestContext } from './common/common-types';
import { CorePrimitivesIdentity } from 'parachain-api';
import { LitentryValidationData, Web3Network } from 'parachain-api';
import { Vec } from '@polkadot/types';


const assertion = {
    VIP3MembershipCard:'gold'
}
describe('Test Vc (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubstrateIdentity: CorePrimitivesIdentity = undefined as any;
    // const client= process.env.BINARY_DIR+'/litentry-cli';
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

    step(`create idGrapgh via cli`, async function () { 

        // console.log(client);
        
    });
    step(`request vc`, async function () {
        console.log(aliceSubstrateIdentity.toHuman());
        
        // let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubstrateIdentity)).toNumber();
        // const getNextNonce = () => currentNonce++;
        // const nonce = getNextNonce();
        // const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        // // console.log(`request vc ${Object.keys(assertion)[0]} for Alice ... Assertion description: ${description}`);
        // const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, context);

        // const requestVcCall = await createSignedTrustedCallRequestVc(
        //     context.api,
        //     context.mrEnclave,
        //     context.api.createType('Index', nonce),
        //     new PolkadotSigner(context.substrateWallet.alice),
        //     aliceSubstrateIdentity,
        //     context.api.createType('Assertion', assertion).toHex(),
        //     context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
        //     requestIdentifier
        // );

        // const res = await sendRequestFromTrustedCall(context, teeShieldingKey, requestVcCall);
        // await assertIsInSidechainBlock(`${Object.keys(assertion)[0]} requestVcCall`, res);
        // const events = await eventsPromise;
        // const vcIssuedEvents = events
        //     .map(({ event }) => event)
        //     .filter(({ section, method }) => section === 'vcManagement' && method === 'VCIssued');

        // assert.equal(
        //     vcIssuedEvents.length,
        //     1,
        //     `vcIssuedEvents.length != 1, please check the ${Object.keys(assertion)[0]} call`
        // );
        // await assertVc(context, aliceSubstrateIdentity, res.value);
    })
});
