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
    sendRequestFromGetter,
    sendRequestFromTrustedCall,
} from './common/di-utils'; // @fixme move to a better place
import { sleep } from './common/utils';
import { aesKey } from './common/call';
import type { IntegrationTestContext } from './common/common-types';
import type { LitentryValidationData, Web3Network, CorePrimitivesIdentity } from 'parachain-api';
import type { Vec, Bytes } from '@polkadot/types';
import type { HexString } from '@polkadot/util/types';

describe('Test Discord Identity (direct invocation)', function () {
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
        networks: Bytes | Vec<Web3Network>;
    }[] = [];

    const bobLinkIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
        validation: LitentryValidationData;
        networks: Bytes | Vec<Web3Network>;
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
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        assert.lengthOf(idGraph, 0);
    });

    step('check bob idgraph from sidechain storage before linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.substrate.Bob,
            bobSubstrateIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        assert.lengthOf(idGraph, 0);
    });

    step('linking discord identity with public message verification (alice)', async function () {
        const nonce = aliceCurrentNonce++;
        const discordIdentity = await buildIdentityHelper('alice', 'Discord', context);
        const discordValidation = await buildWeb2Validation({
            identityType: 'Discord',
            context,
            signerIdentitity: aliceSubstrateIdentity,
            linkIdentity: discordIdentity,
            verificationType: 'PublicMessage',
            validationNonce: nonce,
        });
        const networks = context.api.createType('Vec<Web3Network>', []);

        aliceLinkIdentityRequestParams.push({
            nonce,
            identity: discordIdentity,
            validation: discordValidation,
            networks,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [
                [aliceSubstrateIdentity, true],
                [discordIdentity, true],
            ],
        ];

        for (const { nonce, identity, validation, networks } of aliceLinkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.substrate.Alice,
                aliceSubstrateIdentity,
                identity.toHex(),
                validation.toHex(),
                networks.toHex(),
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

    step('linking discord identity with oauth2 verification (bob)', async function () {
        const nonce = bobCurrentNonce++;
        const discordIdentity = await buildIdentityHelper('bob', 'Discord', context);
        const discordValidation = await buildWeb2Validation({
            identityType: 'Discord',
            context,
            signerIdentitity: bobSubstrateIdentity,
            linkIdentity: discordIdentity,
            validationNonce: nonce,
            verificationType: 'OAuth2',
        });
        const networks = context.api.createType('Vec<Web3Network>', []);

        bobLinkIdentityRequestParams.push({
            nonce,
            identity: discordIdentity,
            validation: discordValidation,
            networks,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [
                [bobSubstrateIdentity, true],
                [discordIdentity, true],
            ],
        ];

        for (const { nonce, identity, validation, networks } of bobLinkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.substrate.Bob,
                bobSubstrateIdentity,
                identity.toHex(),
                validation.toHex(),
                networks.toHex(),
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
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        for (const { identity } of aliceLinkIdentityRequestParams) {
            const identityDump = JSON.stringify(identity.toHuman(), null, 4);
            console.debug(`checking identity: ${identityDump}`);
            const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity));
            assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
            const [, idGraphNodeContext] = idGraphNode!;

            const web3networks = idGraphNode![1].web3networks.toHuman();
            assert.deepEqual(web3networks, []);

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
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        for (const { identity } of bobLinkIdentityRequestParams) {
            const identityDump = JSON.stringify(identity.toHuman(), null, 4);
            console.debug(`checking identity: ${identityDump}`);
            const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity));
            assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
            const [, idGraphNodeContext] = idGraphNode!;

            const web3networks = idGraphNode![1].web3networks.toHuman();
            assert.deepEqual(web3networks, []);

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
