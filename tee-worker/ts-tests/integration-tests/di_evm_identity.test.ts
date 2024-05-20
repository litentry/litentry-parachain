import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import {
    buildValidations,
    initIntegrationTestContext,
    assertIdGraphMutationResult,
    assertIdGraphHash,
    sleep,
} from './common/utils';
import { assertIsInSidechainBlock } from './common/utils/assertion';
import {
    createSignedTrustedCallLinkIdentity,
    createSignedTrustedGetterIdGraph,
    createSignedTrustedCallDeactivateIdentity,
    createSignedTrustedCallActivateIdentity,
    decodeIdGraph,
    getSidechainNonce,
    getTeeShieldingKey,
    sendRequestFromTrustedCall,
    sendAesRequestFromGetter,
} from './common/di-utils'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import type { LitentryValidationData, CorePrimitivesIdentity } from 'parachain-api';
import type { HexString } from '@polkadot/util/types';
import { hexToU8a } from '@polkadot/util';

describe('Test Identity (evm direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceEvmIdentity: CorePrimitivesIdentity = undefined as any;
    let bobEvmIdentity: CorePrimitivesIdentity;
    let currentNonce = 0;

    // Alice links:
    // - a `mock_user` twitter
    // - alice's evm identity
    // - eve's substrate identity (as alice can't link her own substrate again)
    const linkIdentityRequestParams: {
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

        aliceEvmIdentity = await context.web3Wallets.evm.Alice.getIdentity(context);
        bobEvmIdentity = await context.web3Wallets.evm.Bob.getIdentity(context);
        currentNonce = (await getSidechainNonce(context, aliceEvmIdentity)).toNumber();
    });

    step('check idGraph from sidechain storage before linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.evm.Alice,
            aliceEvmIdentity
        );
        const res = await sendAesRequestFromGetter(context, teeShieldingKey, hexToU8a(aesKey), idGraphGetter);

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        assert.lengthOf(idGraph, 0);
    });

    step('linking identities (alice evm account)', async function () {
        const bobEvmNonce = currentNonce++;
        const bobEvmValidation = await buildValidations(
            context,
            aliceEvmIdentity,
            bobEvmIdentity,
            bobEvmNonce,
            'ethereum',
            context.web3Wallets.evm.Bob
        );
        linkIdentityRequestParams.push({
            nonce: bobEvmNonce,
            identity: bobEvmIdentity,
            validation: bobEvmValidation,
        });

        const eveSubstrateNonce = currentNonce++;

        const eveSubstrateIdentity = await context.web3Wallets.substrate.Eve.getIdentity(context);
        const eveSubstrateValidation = await buildValidations(
            context,
            aliceEvmIdentity,
            eveSubstrateIdentity,
            eveSubstrateNonce,
            'substrate',
            context.web3Wallets.substrate.Eve
        );
        linkIdentityRequestParams.push({
            nonce: eveSubstrateNonce,
            identity: eveSubstrateIdentity,
            validation: eveSubstrateValidation,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [
                [aliceEvmIdentity, true],
                [bobEvmIdentity, true],
            ],
            [[eveSubstrateIdentity, true]],
        ];

        let counter = 0;
        for (const { nonce, identity, validation } of linkIdentityRequestParams) {
            counter++;
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.evm.Alice,
                aliceEvmIdentity,
                identity.toHex(),
                validation.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier,
                {
                    withWrappedBytes: false,
                    withPrefix: counter % 2 === 0, // alternate per entry
                }
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceEvmIdentity,
                    res,
                    'LinkIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('linkIdentityCall', res);
        }
        assert.lengthOf(idGraphHashResults, 2);
    });

    step('check user sidechain storage after linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.evm.Alice,
            aliceEvmIdentity
        );
        const res = await sendAesRequestFromGetter(context, teeShieldingKey, hexToU8a(aesKey), idGraphGetter);

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        for (const { identity } of linkIdentityRequestParams) {
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

        await assertIdGraphHash(context, teeShieldingKey, aliceEvmIdentity, idGraph);
    });
    step('deactivating identity(alice evm account)', async function () {
        const deactivateIdentityRequestParams: {
            nonce: number;
            identity: CorePrimitivesIdentity;
        }[] = [];

        const bobEvmNonce = currentNonce++;

        deactivateIdentityRequestParams.push({
            nonce: bobEvmNonce,
            identity: bobEvmIdentity,
        });

        const eveSubstrateNonce = currentNonce++;

        const eveSubstrateIdentity = await context.web3Wallets.substrate.Eve.getIdentity(context);
        deactivateIdentityRequestParams.push({
            nonce: eveSubstrateNonce,
            identity: eveSubstrateIdentity,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [[bobEvmIdentity, false]],
            [[eveSubstrateIdentity, false]],
        ];

        for (const { nonce, identity } of deactivateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const deactivateIdentityCall = await createSignedTrustedCallDeactivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.evm.Alice,
                aliceEvmIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, deactivateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceEvmIdentity,
                    res,
                    'DeactivateIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('deactivateIdentityCall', res);
        }
        assert.lengthOf(idGraphHashResults, 2);
    });

    step('check idGraph from sidechain storage after deactivating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.evm.Alice,
            aliceEvmIdentity
        );
        const res = await sendAesRequestFromGetter(context, teeShieldingKey, hexToU8a(aesKey), idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        for (const { identity } of linkIdentityRequestParams) {
            const identityDump = JSON.stringify(identity.toHuman(), null, 4);
            console.debug(`checking identity: ${identityDump}`);
            const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity));
            assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
            const [, idGraphNodeContext] = idGraphNode!;

            assert.equal(
                idGraphNodeContext.status.toString(),
                'Inactive',
                `status should be Inactive for identity: ${identityDump}`
            );
            console.debug('inactive ✅');
        }

        await assertIdGraphHash(context, teeShieldingKey, aliceEvmIdentity, idGraph);
    });
    step('activating identity(alice evm account)', async function () {
        const activateIdentityRequestParams: {
            nonce: number;
            identity: CorePrimitivesIdentity;
        }[] = [];

        const bobEvmNonce = currentNonce++;

        activateIdentityRequestParams.push({
            nonce: bobEvmNonce,
            identity: bobEvmIdentity,
        });

        const eveSubstrateNonce = currentNonce++;

        const eveSubstrateIdentity = await context.web3Wallets.substrate.Eve.getIdentity(context);

        activateIdentityRequestParams.push({
            nonce: eveSubstrateNonce,
            identity: eveSubstrateIdentity,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [[bobEvmIdentity, true]],
            [[eveSubstrateIdentity, true]],
        ];

        for (const { nonce, identity } of activateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const activateIdentityCall = await createSignedTrustedCallActivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.evm.Alice,
                aliceEvmIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, activateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceEvmIdentity,
                    res,
                    'ActivateIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('activateIdentityCall', res);
        }
        assert.lengthOf(idGraphHashResults, 2);
    });

    step('check idGraph from sidechain storage after activating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.evm.Alice,

            aliceEvmIdentity
        );
        const res = await sendAesRequestFromGetter(context, teeShieldingKey, hexToU8a(aesKey), idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        for (const { identity } of linkIdentityRequestParams) {
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

        await assertIdGraphHash(context, teeShieldingKey, aliceEvmIdentity, idGraph);
    });
    step('check sidechain nonce', async function () {
        await sleep(20);
        const nonce = await getSidechainNonce(context, aliceEvmIdentity);
        assert.equal(nonce.toNumber(), currentNonce);
    });
});
