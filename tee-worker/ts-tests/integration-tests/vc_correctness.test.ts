import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { decryptWithAes, initIntegrationTestContext, PolkadotSigner } from './common/utils';
import { randomSubstrateWallet } from './common/helpers';
import { assertIsInSidechainBlock } from './common/utils/assertion';
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
import { subscribeToEventsWithExtHash } from './common/transactions';
import { KeyringPair } from '@polkadot/keyring/types';
import { u8aToHex } from '@polkadot/util';
import { CredentialDefinition, credentialsJson } from './common/credential-json';
import { byId } from '@litentry/chaindata';

// Change this to the environment you want to test
const chain = byId['litentry-dev'];

describe('Test Vc (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    const substrateIdentities: CorePrimitivesIdentity[] = [];

    const clientDir = process.env.LITENTRY_CLI_DIR;
    const reqExtHash = '0x0000000000000000000000000000000000000000000000000000000000000000';
    const keyringPairs: KeyringPair[] = [];
    let argvId = '';

    const nodeEndpoint: string = chain.rpcs[0].url;
    const enclaveEndpoint: string = chain.enclaveRpcs[0].url;
    console.log(`[node] ${nodeEndpoint}`);
    console.log(`[worker] ${enclaveEndpoint}`);

    const teeDevNodePort = 443;
    const teeDevWorkerPort = 443;
    const errorArray: { id: string; index: number; assertion: any; error: any }[] = [];
    this.timeout(6000000);
    before(async () => {
        context = await initIntegrationTestContext(enclaveEndpoint, nodeEndpoint);
        teeShieldingKey = await getTeeShieldingKey(context);
    });

    // usage example:
    // `pnpm run test-data-providers:local --id=vip3-membership-card-gold` for single test
    // `pnpm run test-data-providers:local` for all tests
    const idIndex = process.argv.indexOf('--id');
    argvId = process.argv[idIndex + 1];
    const { protocol: workerProtocal, hostname: workerHostname } = new URL(enclaveEndpoint);
    const { protocol: nodeProtocal, hostname: nodeHostname } = new URL(nodeEndpoint);

    async function linkIdentityViaCli(id: string) {
        const credentialDefinitions = credentialsJson.find((item) => item.id === id) as CredentialDefinition;
        console.log(`linking identity-${credentialDefinitions.mockDid} via cli`);

        const keyringPair = randomSubstrateWallet();
        keyringPairs.push(keyringPair);
        const formatAddress = u8aToHex(keyringPair.publicKey);
        const substrateIdentity = await new PolkadotSigner(keyringPair).getIdentity(context);
        substrateIdentities.push(substrateIdentity);
        const eventsPromise = subscribeToEventsWithExtHash(reqExtHash, context);
        try {
            // CLIENT = "$CLIENT_BIN -p $NPORT -P $WORKER1PORT -u $NODEURL -U $WORKER1URL"
            const commandPromise = zx`${clientDir} -p ${teeDevNodePort} -P ${teeDevWorkerPort} -u ${
                nodeProtocal + nodeHostname
            } -U ${workerProtocal + workerHostname}\
                  trusted -d link-identity did:litentry:substrate:${formatAddress}\
                  did:${credentialDefinitions.mockDid}\
                  ${credentialDefinitions.mockWeb3Network}`;

            await commandPromise;
        } catch (error: any) {
            console.log(`Exit code: ${error.exitCode}`);
            console.log(`Error: ${error.stderr}`);
            throw error;
        }

        const events = (await eventsPromise).map(({ event }) => event);
        assert.equal(events.length, 1);
    }

    async function requestVc(id: string, index: number) {
        try {
            const credentialDefinitions = credentialsJson.find((item) => item.id === id) as CredentialDefinition;
            const assertion = {
                [credentialDefinitions.assertion.id]: credentialDefinitions.assertion.payload,
            };
            console.log('vc description: ', credentialDefinitions.description);

            console.log('assertion: ', assertion);

            let currentNonce = (await getSidechainNonce(context, substrateIdentities[index])).toNumber();
            const getNextNonce = () => currentNonce++;
            const nonce = getNextNonce();

            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const requestVcCall = await createSignedTrustedCallRequestVc(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new PolkadotSigner(keyringPairs[index]),
                substrateIdentities[index],
                context.api.createType('Assertion', assertion).toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );
            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, requestVcCall);
            await assertIsInSidechainBlock(`${Object.keys(assertion)[0]} requestVcCall`, res);

            const vcResults = context.api.createType('RequestVCResult', res.value);
            const decryptVcPayload = decryptWithAes(aesKey, vcResults.vc_payload, 'utf-8').replace('0x', '');
            const vcPayloadJson = JSON.parse(decryptVcPayload);
            console.log('vcPayload: ', vcPayloadJson);

            assert.equal(
                vcPayloadJson.credentialSubject.values[0],
                credentialDefinitions.expectedCredentialValue,
                "credential value doesn't match, please check the credential json expectedCredentialValue"
            );
        } catch (error) {
            // Sometimes unstable dataprovider can cause interruptions in the testing process. We expect errors in the test to be stored and specific error information to be thrown out after the end.
            const credentialDefinitions = credentialsJson.find((item) => item.id === id) as CredentialDefinition;

            errorArray.push({
                id: id,
                index: index,
                assertion: credentialDefinitions.assertion.payload,
                error: error,
            });
            console.error(`Error in requestVc for id ${id} at index ${index}:`, error);
        }
    }

    if (argvId && credentialsJson.find((item) => item.id === argvId)) {
        step(`link identity && request vc with specific credentials for ${argvId}`, async function () {
            await linkIdentityViaCli(argvId);
            await requestVc(argvId, 0);
        });
    } else {
        credentialsJson.forEach(({ id }, index) => {
            step(`link identity && request vc with all credentials for ${id}`, async function () {
                await linkIdentityViaCli(id);
                await requestVc(id, index);
            });
        });
    }
    after(async function () {
        if (errorArray.length > 0) {
            console.log('errorArray:', errorArray);
            throw new Error(`${errorArray.length} tests failed. See above for details.`);
        }
    });
});
