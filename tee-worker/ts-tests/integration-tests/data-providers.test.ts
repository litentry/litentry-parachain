import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { buildIdentityFromKeypair, decryptWithAes, initIntegrationTestContext, PolkadotSigner } from './common/utils';
import { assertIsInSidechainBlock } from './common/utils/assertion';
import {
    getSidechainNonce,
    getTeeShieldingKey,
    sendRequestFromTrustedCall,
    createSignedTrustedCallRequestVc,
} from './common/di-utils'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/common-types';
import { CorePrimitivesIdentity, RequestVCResult } from 'parachain-api';
import { aesKey } from './common/call';
import { $ as zx } from 'zx';
import { credentialDefinitionMap } from './common/credential-definitions';
import { subscribeToEventsWithExtHash } from './common/transactions';
describe('Test Vc (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubstrateIdentity: CorePrimitivesIdentity = undefined as any;

    // CLIENT="${CLIENT_BIN} -p ${LITENTRY_RPC_PORT} -P ${WORKER_1_PORT} -u ${LITENTRY_RPC_URL} -U ${WORKER_1_URL}"

    const binPath = process.env.BINARY_DIR + '/litentry-cli';
    const client = binPath + ' -p ' + 9912 + ' -P ' + 2011 + ' -u ' + 'ws://litentry-node' + ' -U ' + 'wss://litentry-worker-1';
    
    const aliceAddressFormat = '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d';
    const reqExtHash = '0x0000000000000000000000000000000000000000000000000000000000000000';
    let argvId = '';
    this.timeout(6000000);
    before(async () => {
        context = await initIntegrationTestContext(process.env.WORKER_ENDPOINT!, process.env.NODE_ENDPOINT!);
        teeShieldingKey = await getTeeShieldingKey(context);
        aliceSubstrateIdentity = await buildIdentityFromKeypair(
            new PolkadotSigner(context.substrateWallet.alice),
            context
        );
    });

    // usage example:
    // pnpm run test-data-providers:local --id=vip3-membership-card-gold for single test
    // pnpm run test-data-providers:local for all tests
    const argv = process.argv.indexOf('--id');
    argvId = process.argv[argv + 1];

    async function linkIdentityViaCli(id: string) {
        const eventsPromise = subscribeToEventsWithExtHash(reqExtHash, context);
        try {
            await zx`${client} trusted -d link-identity did:litentry:substrate:${aliceAddressFormat}\
                  did:${credentialDefinitionMap[id].mockDid}\
                  ${credentialDefinitionMap[id].mockWeb3Network}`;
        } catch (error: any) {
            console.log(`Exit code: ${error.exitCode}`);
            console.log(`Error: ${error.stderr}`);
            throw error;
        }

        const events = (await eventsPromise).map(({ event }) => event);
        assert.equal(events.length, 1);
    }

    // eslint-disable-next-line no-prototype-builtins
    if (argvId && credentialDefinitionMap.hasOwnProperty(argvId)) {
        step(`linking identity-${credentialDefinitionMap[argvId].mockDid} via cli`, async function () {
            await linkIdentityViaCli(argvId);
        });
    } else {
        Object.keys(credentialDefinitionMap).forEach((id) => {
            step(`linking identity-${credentialDefinitionMap[id].mockAddress} via cli`, async function () {
                await linkIdentityViaCli(id);
            });
        });
    }

    Object.keys(credentialDefinitionMap).forEach((id) => {
        step(`request vc for ${credentialDefinitionMap[id]}`, async function () {
            const assertion = {
                [credentialDefinitionMap[id].assertion.id]: credentialDefinitionMap[id].assertion.payload,
            };
            console.log(assertion);

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

            const vcResults = context.api.createType('RequestVCResult', res.value) as unknown as RequestVCResult;
            const decryptVcPayload = decryptWithAes(aesKey, vcResults.vc_payload, 'utf-8').replace('0x', '');
            const vcPayloadJson = JSON.parse(decryptVcPayload);

            assert.equal(
                vcPayloadJson.credentialSubject.values[0],
                credentialDefinitionMap[id].expectedCredentialValue
            );
        });
    });
});
