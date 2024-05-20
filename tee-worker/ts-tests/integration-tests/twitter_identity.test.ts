import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import {
    assertIdGraphMutationResult,
    assertIdGraphHash,
    buildIdentityHelper,
    initIntegrationTestContext,
    buildWeb2Validation,
} from './common/utils';
import { assertIsInSidechainBlock } from './common/utils/assertion';
import {
    createSignedTrustedCallLinkIdentity,
    createSignedTrustedGetterIdGraph,
    decodeIdGraph,
    getSidechainNonce,
    getTeeShieldingKey,
    sendRsaRequestFromGetter,
    sendRequestFromTrustedCall,
    sendAesRequestFromGetter,
} from './common/di-utils'; // @fixme move to a better place
import { sleep } from './common/utils';
import { aesKey, sendRequest, decodeRpcBytesAsString } from './common/call';
import { createJsonRpcRequest, nextRequestId } from './common/helpers';
import type { IntegrationTestContext } from './common/common-types';
import type { LitentryValidationData, CorePrimitivesIdentity } from 'parachain-api';
import type { HexString } from '@polkadot/util/types';
import { hexToU8a } from '@polkadot/util';

describe('Test Twitter Identity (direct invocation)', function () {
    let context: IntegrationTestContext;
    let teeShieldingKey: KeyObject;
    let aliceSubstrateIdentity: CorePrimitivesIdentity;
    let bobSubstrateIdentity: CorePrimitivesIdentity;
    let aliceCurrentNonce = 0;
    let bobCurrentNonce = 0;

    const aliceLinkIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
        validation: LitentryValidationData;
    }[] = [];

    const bobLinkIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
        validation: LitentryValidationData;
    }[] = [];

    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_ENDPOINT!, // @fixme evil assertion; centralize env access
            process.env.NODE_ENDPOINT! // @fixme evil assertion; centralize env access
        );
        teeShieldingKey = await getTeeShieldingKey(context);

        aliceSubstrateIdentity = await context.web3Wallets.substrate.Alice.getIdentity(context);
        bobSubstrateIdentity = await context.web3Wallets.substrate.Bob.getIdentity(context);

        aliceCurrentNonce = (await getSidechainNonce(context, aliceSubstrateIdentity)).toNumber();
        bobCurrentNonce = (await getSidechainNonce(context, bobSubstrateIdentity)).toNumber();
    });

    step('check alice idgraph from sidechain storage before linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.substrate.Alice,
            aliceSubstrateIdentity
        );
        const res = await sendAesRequestFromGetter(context, teeShieldingKey, hexToU8a(aesKey), idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        assert.lengthOf(idGraph, 0);
    });

    step('check bob idgraph from sidechain storage before linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.substrate.Bob,
            bobSubstrateIdentity
        );
        const res = await sendRsaRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        assert.lengthOf(idGraph, 0);
    });

    step('linking twitter identity with public tweet verification (alice)', async function () {
        const nonce = aliceCurrentNonce++;
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const twitterValidation = await buildWeb2Validation({
            identityType: 'Twitter',
            context,
            signerIdentitity: aliceSubstrateIdentity,
            linkIdentity: twitterIdentity,
            verificationType: 'PublicTweet',
            validationNonce: nonce,
        });

        aliceLinkIdentityRequestParams.push({
            nonce,
            identity: twitterIdentity,
            validation: twitterValidation,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [
                [aliceSubstrateIdentity, true],
                [twitterIdentity, true],
            ],
        ];

        for (const { nonce, identity, validation } of aliceLinkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.substrate.Alice,
                aliceSubstrateIdentity,
                identity.toHex(),
                validation.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier,
                {
                    withWrappedBytes: false,
                    withPrefix: false,
                }
            );
            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);

            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceSubstrateIdentity,
                    res,
                    'LinkIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);

            await assertIsInSidechainBlock('linkIdentityCall', res);
        }
        assert.lengthOf(idGraphHashResults, 1);
    });

    step('linking twitter identity with oauth2 verification (bob)', async function () {
        // Generate oauth code verifier on the enclave for the user
        const did = 'did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48';
        const request = createJsonRpcRequest(
            'identity_getTwitterAuthorizeUrl',
            [did, 'http://127.0.0.1:3000/callback'],
            nextRequestId(context)
        );
        const response = await sendRequest(context.tee, request, context.api);
        const authorizeUrl = decodeRpcBytesAsString(response.value);
        const state = authorizeUrl.split('state=')[1].split('&')[0];

        const nonce = bobCurrentNonce++;
        const twitterIdentity = await buildIdentityHelper('mock_user_me', 'Twitter', context);
        const twitterValidation = await buildWeb2Validation({
            identityType: 'Twitter',
            context,
            signerIdentitity: bobSubstrateIdentity,
            linkIdentity: twitterIdentity,
            validationNonce: nonce,
            verificationType: 'OAuth2',
            oauthState: state,
        });

        bobLinkIdentityRequestParams.push({
            nonce,
            identity: twitterIdentity,
            validation: twitterValidation,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [
                [bobSubstrateIdentity, true],
                [twitterIdentity, true],
            ],
        ];

        for (const { nonce, identity, validation } of bobLinkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.substrate.Bob,
                bobSubstrateIdentity,
                identity.toHex(),
                validation.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier,
                {
                    withWrappedBytes: false,
                    withPrefix: true,
                }
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);

            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    bobSubstrateIdentity,
                    res,
                    'LinkIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);

            await assertIsInSidechainBlock('linkIdentityCall', res);
        }
        assert.lengthOf(idGraphHashResults, 1);
    });

    step('check users sidechain storage after linking (alice)', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.substrate.Alice,
            aliceSubstrateIdentity
        );
        const res = await sendRsaRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        for (const { identity } of aliceLinkIdentityRequestParams) {
            const identityDump = JSON.stringify(identity.toHuman(), null, 4);
            console.debug(`checking identity: ${identityDump}`);
            const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity));
            assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
            const [, idGraphNodeContext] = idGraphNode!;

            assert.equal(
                idGraphNodeContext.status.toString(),
                'Active',
                `status should be active for identity: ${identityDump}`
            );
            console.debug('active ✅');
        }

        await assertIdGraphHash(context, teeShieldingKey, aliceSubstrateIdentity, idGraph);
    });

    step('check users sidechain storage after linking (bob)', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.substrate.Bob,
            bobSubstrateIdentity
        );
        const res = await sendAesRequestFromGetter(context, teeShieldingKey, hexToU8a(aesKey), idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        for (const { identity } of bobLinkIdentityRequestParams) {
            const identityDump = JSON.stringify(identity.toHuman(), null, 4);
            console.debug(`checking identity: ${identityDump}`);
            const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity));
            assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
            const [, idGraphNodeContext] = idGraphNode!;

            assert.equal(
                idGraphNodeContext.status.toString(),
                'Active',
                `status should be active for identity: ${identityDump}`
            );
            console.debug('active ✅');
        }

        await assertIdGraphHash(context, teeShieldingKey, bobSubstrateIdentity, idGraph);
    });

    step('check sidechain nonce', async function () {
        await sleep(20);

        const aliceNonce = await getSidechainNonce(context, aliceSubstrateIdentity);
        assert.equal(aliceNonce.toNumber(), aliceCurrentNonce);

        const bobNonce = await getSidechainNonce(context, bobSubstrateIdentity);
        assert.equal(bobNonce.toNumber(), bobCurrentNonce);
    });
});
