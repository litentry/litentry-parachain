import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { u8aToHex, u8aToString } from '@polkadot/util';
import {
    assertIdGraphMutationResult,
    assertIdGraphHash,
    assertWorkerError,
    buildIdentityHelper,
    buildValidations,
    initIntegrationTestContext,
    buildWeb2Validation,
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
    createSignedTrustedCall,
} from './common/di-utils'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import type { LitentryValidationData, CorePrimitivesIdentity } from 'parachain-api';
import { ethers } from 'ethers';
import type { HexString } from '@polkadot/util/types';
import { sleep } from './common/utils';

describe('Test Identity (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubstrateIdentity: CorePrimitivesIdentity = undefined as any;
    let bobSubstrateIdentity: CorePrimitivesIdentity = undefined as any;
    let charlieSubstrateIdentity: CorePrimitivesIdentity = undefined as any;
    let aliceCurrentNonce = 0;
    let bobCurrentNonce = 0;
    let charlieCurrentNonce = 0;
    // Alice links:
    // - a `mock_user` twitter
    // - alice's evm identity
    // - eve's substrate identity (as alice can't link her own substrate again)
    // - alice's bitcoin identity
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
        aliceSubstrateIdentity = await context.web3Wallets.substrate.Alice.getIdentity(context);
        bobSubstrateIdentity = await context.web3Wallets.substrate.Bob.getIdentity(context);
        charlieSubstrateIdentity = await context.web3Wallets.substrate.Charlie.getIdentity(context);
        aliceCurrentNonce = (await getSidechainNonce(context, aliceSubstrateIdentity)).toNumber();
        bobCurrentNonce = (await getSidechainNonce(context, bobSubstrateIdentity)).toNumber();
        charlieCurrentNonce = (await getSidechainNonce(context, charlieSubstrateIdentity)).toNumber();
    });

    step('check idgraph from sidechain storage before linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.substrate.Alice,
            aliceSubstrateIdentity
        );
        const res = await sendRsaRequestFromGetter(context, teeShieldingKey, idGraphGetter);

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        assert.lengthOf(idGraph, 0);
    });

    step('linking identities (alice)', async function () {
        const twitterNonce = aliceCurrentNonce++;

        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);

        const twitterValidation = await buildWeb2Validation({
            identityType: 'Twitter',
            context,
            signerIdentitity: aliceSubstrateIdentity,
            linkIdentity: twitterIdentity,
            verificationType: 'PublicTweet',
            validationNonce: twitterNonce,
        });
        linkIdentityRequestParams.push({
            nonce: twitterNonce,
            identity: twitterIdentity,
            validation: twitterValidation,
        });

        const evmNonce = aliceCurrentNonce++;

        const evmIdentity = await context.web3Wallets.evm.Alice.getIdentity(context);
        const evmValidation = await buildValidations(
            context,
            aliceSubstrateIdentity,
            evmIdentity,
            evmNonce,
            'ethereum',
            context.web3Wallets.evm.Alice
        );
        linkIdentityRequestParams.push({
            nonce: evmNonce,
            identity: evmIdentity,
            validation: evmValidation,
        });

        const eveSubstrateNonce = aliceCurrentNonce++;
        const eveSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.web3Wallets.substrate.Eve.getAddressRaw()),
            'Substrate',
            context
        );
        const eveSubstrateValidation = await buildValidations(
            context,
            aliceSubstrateIdentity,
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

        const bitcoinNonce = aliceCurrentNonce++;
        const bitcoinIdentity = await buildIdentityHelper(
            u8aToHex(context.web3Wallets.bitcoin.Alice.getAddressRaw()),
            'Bitcoin',
            context
        );
        console.log('bitcoin id: ', bitcoinIdentity.toHuman());
        const bitcoinValidation = await buildValidations(
            context,
            aliceSubstrateIdentity,
            bitcoinIdentity,
            bitcoinNonce,
            'bitcoin',
            context.web3Wallets.bitcoin.Alice
        );
        linkIdentityRequestParams.push({
            nonce: bitcoinNonce,
            identity: bitcoinIdentity,
            validation: bitcoinValidation,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [
                [aliceSubstrateIdentity, true],
                [twitterIdentity, true],
            ],
            [[evmIdentity, true]],
            [[eveSubstrateIdentity, true]],
            [[bitcoinIdentity, true]],
        ];

        let counter = 0;
        for (const { nonce, identity, validation } of linkIdentityRequestParams) {
            counter++;
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
                    withPrefix: counter % 2 === 0, // alternate per entry
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
        assert.lengthOf(idGraphHashResults, 4);
    });

    step('check user sidechain storage after linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.substrate.Alice,

            aliceSubstrateIdentity
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

        await assertIdGraphHash(context, teeShieldingKey, aliceSubstrateIdentity, idGraph);
    });

    step('linking identity with wrong signature', async function () {
        const evmIdentity = await context.web3Wallets.evm.Alice.getIdentity(context);
        const evmNonce = aliceCurrentNonce++;

        // random wrong msg
        const wrongMsg = '0x693d9131808e7a8574c7ea5eb7813bdf356223263e61fa8fe2ee8e434508bc75';
        const evmSignature = await context.web3Wallets.evm.Alice.sign(ethers.utils.arrayify(wrongMsg));

        const evmValidationData = {
            Web3Validation: {
                Evm: {
                    message: wrongMsg as HexString,
                    signature: {
                        Ethereum: u8aToHex(evmSignature),
                    },
                },
            },
        };
        const encodedVerifyIdentityValidation = context.api.createType('LitentryValidationData', evmValidationData);
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;

        const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', evmNonce),
            context.web3Wallets.substrate.Alice,
            aliceSubstrateIdentity,
            evmIdentity.toHex(),
            encodedVerifyIdentityValidation.toHex(),
            context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
            requestIdentifier
        );
        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);

        assert.isTrue(res.do_watch.isFalse);
        assert.isTrue(res.status.asTrustedOperationStatus[0].isInvalid);
        assertWorkerError(
            context,
            (v) => {
                assert.isTrue(v.isLinkIdentityFailed, `expected LinkIdentityFailed, received ${v.type} instead`);
                assert.isTrue(
                    v.asLinkIdentityFailed.isUnexpectedMessage,
                    `expected UnexpectedMessage, received ${v.asLinkIdentityFailed.type} instead`
                );
            },
            res
        );
    });

    step('linking already linked identity', async function () {
        const twitterNonce = aliceCurrentNonce++;

        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const twitterValidation = await buildWeb2Validation({
            identityType: 'Twitter',
            context,
            signerIdentitity: aliceSubstrateIdentity,
            linkIdentity: twitterIdentity,
            verificationType: 'PublicTweet',
            validationNonce: twitterNonce,
        });

        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', twitterNonce),
            context.web3Wallets.substrate.Alice,
            aliceSubstrateIdentity,
            twitterIdentity.toHex(),
            twitterValidation.toHex(),
            context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
            requestIdentifier
        );
        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);

        assert.isTrue(res.do_watch.isFalse);
        assert.isTrue(res.status.asTrustedOperationStatus[0].isInvalid);
        assertWorkerError(
            context,
            (v) => {
                assert.isTrue(v.isLinkIdentityFailed, `expected LinkIdentityFailed, received ${v.type} instead`);
                assert.isTrue(
                    v.asLinkIdentityFailed.isStfError,
                    `expected StfError, received ${v.asLinkIdentityFailed.type} instead`
                );
                assert.equal(u8aToString(v.asLinkIdentityFailed.asStfError), 'IdentityAlreadyLinked');
            },
            res
        );
    });

    step('deactivating linked identities', async function () {
        const deactivateIdentityRequestParams: {
            nonce: number;
            identity: CorePrimitivesIdentity;
        }[] = [];

        const twitterNonce = aliceCurrentNonce++;
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);

        deactivateIdentityRequestParams.push({
            nonce: twitterNonce,
            identity: twitterIdentity,
        });

        const evmNonce = aliceCurrentNonce++;
        const evmIdentity = await context.web3Wallets.evm.Alice.getIdentity(context);

        deactivateIdentityRequestParams.push({
            nonce: evmNonce,
            identity: evmIdentity,
        });

        const eveSubstrateNonce = aliceCurrentNonce++;
        const eveSubstrateIdentity = await context.web3Wallets.substrate.Eve.getIdentity(context);

        deactivateIdentityRequestParams.push({
            nonce: eveSubstrateNonce,
            identity: eveSubstrateIdentity,
        });

        const bitcoinNonce = aliceCurrentNonce++;

        const bitcoinIdentity = await context.web3Wallets.bitcoin.Alice.getIdentity(context);

        deactivateIdentityRequestParams.push({
            nonce: bitcoinNonce,
            identity: bitcoinIdentity,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [[twitterIdentity, false]],
            [[evmIdentity, false]],
            [[eveSubstrateIdentity, false]],
            [[bitcoinIdentity, false]],
        ];

        for (const { nonce, identity } of deactivateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const deactivateIdentityCall = await createSignedTrustedCallDeactivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.substrate.Alice,
                aliceSubstrateIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, deactivateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceSubstrateIdentity,
                    res,
                    'DeactivateIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('deactivateIdentityCall', res);
        }
        assert.lengthOf(idGraphHashResults, 4);
    });

    step('check idgraph from sidechain storage after deactivating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.substrate.Alice,
            aliceSubstrateIdentity
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
                'Inactive',
                `status should be Inactive for identity: ${identityDump}`
            );
            console.debug('inactive ✅');
        }

        await assertIdGraphHash(context, teeShieldingKey, aliceSubstrateIdentity, idGraph);
    });
    step('activating linked identities', async function () {
        const activateIdentityRequestParams: {
            nonce: number;
            identity: CorePrimitivesIdentity;
        }[] = [];

        const twitterNonce = aliceCurrentNonce++;
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);

        activateIdentityRequestParams.push({
            nonce: twitterNonce,
            identity: twitterIdentity,
        });

        const evmNonce = aliceCurrentNonce++;
        const evmIdentity = await context.web3Wallets.evm.Alice.getIdentity(context);

        activateIdentityRequestParams.push({
            nonce: evmNonce,
            identity: evmIdentity,
        });

        const eveSubstrateNonce = aliceCurrentNonce++;
        const eveSubstrateIdentity = await context.web3Wallets.substrate.Eve.getIdentity(context);

        activateIdentityRequestParams.push({
            nonce: eveSubstrateNonce,
            identity: eveSubstrateIdentity,
        });

        const bitcoinNonce = aliceCurrentNonce++;
        const bitcoinIdentity = await context.web3Wallets.bitcoin.Alice.getIdentity(context);
        activateIdentityRequestParams.push({
            nonce: bitcoinNonce,
            identity: bitcoinIdentity,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [[twitterIdentity, true]],
            [[evmIdentity, true]],
            [[eveSubstrateIdentity, true]],
            [[bitcoinIdentity, true]],
        ];

        for (const { nonce, identity } of activateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const activateIdentityCall = await createSignedTrustedCallActivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.substrate.Alice,
                aliceSubstrateIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, activateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceSubstrateIdentity,
                    res,
                    'ActivateIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('activateIdentityCall', res);
        }
        assert.lengthOf(idGraphHashResults, 4);
    });

    step('check idgraph from sidechain storage after activating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            context.web3Wallets.substrate.Alice,
            aliceSubstrateIdentity
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

        await assertIdGraphHash(context, teeShieldingKey, aliceSubstrateIdentity, idGraph);
    });

    step('deactivate prime identity', async function () {
        // deactivating prime identity should be possible and create the IDGraph if one doesn't exist already
        const deactivateIdentityRequestParams: {
            nonce: number;
            identity: CorePrimitivesIdentity;
        }[] = [];

        deactivateIdentityRequestParams.push({
            nonce: bobCurrentNonce++,
            identity: bobSubstrateIdentity,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [[[bobSubstrateIdentity, false]]];

        for (const { nonce, identity } of deactivateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const deactivateIdentityCall = await createSignedTrustedCallDeactivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.substrate.Bob,
                bobSubstrateIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, deactivateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    bobSubstrateIdentity,
                    res,
                    'DeactivateIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('deactivateIdentityCall', res);
        }
        assert.lengthOf(idGraphHashResults, 1);
    });

    step('linking invalid identity with different identities', async function () {
        let currentNonce = (await getSidechainNonce(context, bobSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const twitterNonce = getNextNonce();
        const aliceEvmNonce = getNextNonce();
        const aliceEvmIdentity = await context.web3Wallets.evm.Alice.getIdentity(context);
        const aliceEvmValidation = await buildValidations(
            context,
            bobSubstrateIdentity,
            aliceEvmIdentity,
            aliceEvmNonce,
            'ethereum',
            context.web3Wallets.evm.Bob
        );

        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;

        const linkIdentityCall = await createSignedTrustedCall(
            context.api,
            [
                'link_identity',
                '(LitentryIdentity, LitentryIdentity, LitentryIdentity, LitentryValidationData, Option<RequestAesKey>, H256)',
            ],
            context.web3Wallets.substrate.Bob,
            context.mrEnclave,

            context.api.createType('Index', twitterNonce),

            [
                bobSubstrateIdentity.toHuman(),
                aliceEvmIdentity.toHuman(),
                twitterIdentity,
                aliceEvmValidation,
                aesKey,
                requestIdentifier,
            ]
        );

        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);

        assert.isTrue(res.do_watch.isFalse);
        assert.isTrue(res.status.asTrustedOperationStatus[0].isInvalid);
        console.log('linkInvalidIdentity call returned', res.toHuman());

        assertWorkerError(
            context,
            (v) => {
                assert.isTrue(v.isLinkIdentityFailed, `expected LinkIdentityFailed, received ${v.type} instead`);
            },
            res
        );
    });
    step('check sidechain nonce', async function () {
        await sleep(20);
        const aliceNonce = await getSidechainNonce(context, aliceSubstrateIdentity);
        assert.equal(aliceNonce.toNumber(), aliceCurrentNonce);
    });
});
