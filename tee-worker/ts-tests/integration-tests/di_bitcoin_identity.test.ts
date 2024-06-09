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
    sendRsaRequestFromGetter,
    sendRequestFromTrustedCall,
    sendAesRequestFromGetter,
} from './common/di-utils'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import type { LitentryValidationData, Web3Network, CorePrimitivesIdentity } from 'parachain-api';
import { type Bytes, type Vec } from '@polkadot/types';
import type { HexString } from '@polkadot/util/types';
import { hexToU8a } from '@polkadot/util';

describe('Test Identity (bitcoin direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceBitcoinIdentity: CorePrimitivesIdentity = undefined as any;
    let aliceEvmIdentity: CorePrimitivesIdentity;
    let bobBitcoinIdentity: CorePrimitivesIdentity;
    let currentNonce = 0;

    // Alice links:
    // - alice's evm identity
    // - bob's bitcoin identity
    const linkIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
        validation: LitentryValidationData;
        networks: Bytes | Vec<Web3Network>;
    }[] = [];

    const deactivateIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
    }[] = [];

    const activateIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
    }[] = [];

    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_ENDPOINT!, // @fixme evil assertion; centralize env access
            process.env.NODE_ENDPOINT! // @fixme evil assertion; centralize env access
        );
        teeShieldingKey = await getTeeShieldingKey(context);
        aliceBitcoinIdentity = await context.web3Wallets.bitcoin.Alice.getIdentity(context);
        aliceEvmIdentity = await context.web3Wallets.evm.Alice.getIdentity(context);
        bobBitcoinIdentity = await context.web3Wallets.bitcoin.Bob.getIdentity(context);
        currentNonce = (await getSidechainNonce(context, aliceBitcoinIdentity)).toNumber();
    });

    step('check idGraph from sidechain storage before linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.bitcoin.Alice,
            aliceBitcoinIdentity
        );
        const res = await sendAesRequestFromGetter(context, teeShieldingKey, hexToU8a(aesKey), idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);
        assert.lengthOf(idGraph, 0);
    });

    step('linking identities (alice bitcoin account)', async function () {
        const aliceEvmNonce = currentNonce++;
        const aliceEvmValidation = await buildValidations(
            context,
            aliceBitcoinIdentity,
            aliceEvmIdentity,
            aliceEvmNonce,
            'ethereum',
            context.web3Wallets.evm.Alice
        );
        const aliceEvmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);
        linkIdentityRequestParams.push({
            nonce: aliceEvmNonce,
            identity: aliceEvmIdentity,
            validation: aliceEvmValidation,
            networks: aliceEvmNetworks,
        });

        // link another bitcoin account
        const bobBitcoinNonce = currentNonce++;
        const bobBitcoinValidation = await buildValidations(
            context,
            aliceBitcoinIdentity,
            bobBitcoinIdentity,
            bobBitcoinNonce,
            'bitcoin',
            context.web3Wallets.bitcoin.Bob
        );
        const bobBitcoinNetowrks = context.api.createType('Vec<Web3Network>', ['BitcoinP2tr']);
        linkIdentityRequestParams.push({
            nonce: bobBitcoinNonce,
            identity: bobBitcoinIdentity,
            validation: bobBitcoinValidation,
            networks: bobBitcoinNetowrks,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [
                [aliceBitcoinIdentity, true],
                [aliceEvmIdentity, true],
            ],
            [[bobBitcoinIdentity, true]],
        ];

        let counter = 0;
        for (const { nonce, identity, validation, networks } of linkIdentityRequestParams) {
            counter++;
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.bitcoin.Alice,
                aliceBitcoinIdentity,
                identity.toHex(),
                validation.toHex(),
                networks.toHex(),
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
                    aliceBitcoinIdentity,
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
            context.web3Wallets.bitcoin.Alice,
            aliceBitcoinIdentity
        );
        const res = await sendRsaRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // according to the order of linkIdentityRequestParams
        const expectedWeb3Networks = [['Ethereum', 'Bsc'], ['BitcoinP2tr']];
        let currentIndex = 0;

        for (const { identity } of linkIdentityRequestParams) {
            const identityDump = JSON.stringify(identity.toHuman(), null, 4);
            console.debug(`checking identity: ${identityDump}`);
            const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity));
            assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
            const [, idGraphNodeContext] = idGraphNode!;

            const web3networks = idGraphNode![1].web3networks.toHuman();
            assert.deepEqual(web3networks, expectedWeb3Networks[currentIndex]);

            assert.equal(
                idGraphNodeContext.status.toString(),
                'Active',
                `status should be active for identity: ${identityDump}`
            );
            console.debug('active ✅');

            currentIndex++;
        }

        await assertIdGraphHash(context, teeShieldingKey, aliceBitcoinIdentity, idGraph);
    });
    step('deactivating identity(alice bitcoin account)', async function () {
        const aliceEvmNonce = currentNonce++;

        deactivateIdentityRequestParams.push({
            nonce: aliceEvmNonce,
            identity: aliceEvmIdentity,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [[[aliceEvmIdentity, false]]];

        for (const { nonce, identity } of deactivateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const deactivateIdentityCall = await createSignedTrustedCallDeactivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.bitcoin.Alice,
                aliceBitcoinIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, deactivateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceBitcoinIdentity,
                    res,
                    'DeactivateIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('deactivateIdentityCall', res);
            assert.lengthOf(idGraphHashResults, 1);
        }
    });

    step('check idGraph from sidechain storage after deactivating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.bitcoin.Alice,
            aliceBitcoinIdentity
        );
        const res = await sendRsaRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        for (const { identity } of deactivateIdentityRequestParams) {
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

        await assertIdGraphHash(context, teeShieldingKey, aliceBitcoinIdentity, idGraph);
    });

    step('activating identity(alice bitcoin account)', async function () {
        const aliceEvmNonce = currentNonce++;

        activateIdentityRequestParams.push({
            nonce: aliceEvmNonce,
            identity: aliceEvmIdentity,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [[[aliceEvmIdentity, true]]];

        for (const { nonce, identity } of activateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const activateIdentityCall = await createSignedTrustedCallActivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.bitcoin.Alice,

                aliceBitcoinIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, activateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceBitcoinIdentity,
                    res,
                    'ActivateIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('activateIdentityCall', res);
        }
        assert.lengthOf(idGraphHashResults, 1);
    });

    step('check idGraph from sidechain storage after activating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.bitcoin.Alice,
            aliceBitcoinIdentity
        );
        const res = await sendRsaRequestFromGetter(context, teeShieldingKey, idGraphGetter);
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

        await assertIdGraphHash(context, teeShieldingKey, aliceBitcoinIdentity, idGraph);
    });
    step('check sidechain nonce', async function () {
        await sleep(20);
        const nonce = await getSidechainNonce(context, aliceBitcoinIdentity);
        assert.equal(nonce.toNumber(), currentNonce);
    });
});
