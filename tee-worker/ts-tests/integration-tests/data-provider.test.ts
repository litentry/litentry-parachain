import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { buildIdentityFromKeypair, initIntegrationTestContext, PolkadotSigner, sleep } from './common/utils';
import { assertIsInSidechainBlock, assertVc } from './common/utils/assertion';
import {
    getSidechainNonce,
    getTeeShieldingKey,
    sendRequestFromTrustedCall,
    createSignedTrustedCallRequestVc,
} from './common/di-utils'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/common-types';
import { CorePrimitivesIdentity } from 'parachain-api';
import { aesKey } from './common/call';
import { $ as zx } from 'zx';
import { credentialDefinitionMap } from './common/credential-definitions';
import { subscribeToEventsWithExtHash } from './common/transactions';
describe('Test Vc (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubstrateIdentity: CorePrimitivesIdentity = undefined as any;
    const client = process.env.BINARY_DIR + '/litentry-cli';
    const aliceAddressFormat = '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d';
    const reqExtHash = '0x0000000000000000000000000000000000000000000000000000000000000000';
    this.timeout(6000000);
    before(async () => {
        context = await initIntegrationTestContext(process.env.WORKER_ENDPOINT!, process.env.NODE_ENDPOINT!);
        teeShieldingKey = await getTeeShieldingKey(context);
        aliceSubstrateIdentity = await buildIdentityFromKeypair(
            new PolkadotSigner(context.substrateWallet.alice),
            context
        );
    });

    step(`create idGrapgh via cli`, async function () {
        // todo: get process args from command line
        const eventsPromise = subscribeToEventsWithExtHash(reqExtHash, context);

        try {
            const linkResult = await zx`${client} trusted -d link-identity did:litentry:substrate:${aliceAddressFormat}\
            did:${credentialDefinitionMap['vip3-membership-card-gold'].mockDid}\
            ${credentialDefinitionMap['vip3-membership-card-gold'].mockWeb3Network}`;

            console.log(linkResult);
        } catch (error: any) {
            console.log(`Exit code: ${error.exitCode}`);
            console.log(`Error: ${error.stderr}`);
            throw error;
        }

        const events = (await eventsPromise).map(({ event }) => event);
        assert.equal(events.length, 1);
        // todo: listen to event
    });

    step(`request vc`, async function () {
        const assertion = {
            [credentialDefinitionMap['vip3-membership-card-gold'].assertion.id]:
                credentialDefinitionMap['vip3-membership-card-gold'].assertion.payload,
        };
        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, aliceSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;
        const nonce = getNextNonce();
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const requestVcCall = await createSignedTrustedCallRequestVc(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', nonce),
            new PolkadotSigner(context.substrateWallet.alice),
            aliceSubstrateIdentity,
            context.api.createType('Assertion', assertion).toHex(),
            context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
            requestIdentifier
        );
        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, requestVcCall);
        await assertIsInSidechainBlock(`${Object.keys(assertion)[0]} requestVcCall`, res);
        await assertVc(context, aliceSubstrateIdentity, res.value);
    });
});
