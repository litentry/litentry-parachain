import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { initIntegrationTestContext } from './common/utils';
import { assertIsInSidechainBlock, assertVc } from './common/utils/assertion';
import {
    getSidechainNonce,
    createSignedTrustedCallLinkIdentity,
    getTeeShieldingKey,
    sendRequestFromTrustedCall,
    createSignedTrustedCallRequestVc,
} from './common/di-utils'; // @fixme move to a better place
import { buildValidations } from './common/utils';
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import type { CorePrimitivesIdentity } from 'parachain-api';
import type { LitentryValidationData, Web3Network } from 'parachain-api';
import type { Vec, Bytes } from '@polkadot/types';
import fs from 'fs';
import path from 'path';
import { assert } from 'chai';
import { genesisSubstrateWallet } from './common/helpers';
import { KeyringPair } from '@polkadot/keyring/types';
import { subscribeToEvents, subscribeToEventsWithExtHash } from './common/transactions';
import { encryptWithTeeShieldingKey } from './common/utils/crypto';
import { ethers } from 'ethers';
import { sleep } from './common/utils';
import { $ as zx } from 'zx';

describe('Test Vc (direct request)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubstrateIdentity: CorePrimitivesIdentity = undefined as any;

    let alice: KeyringPair = undefined as any;
    let contractBytecode = undefined as any;
    const clientDir = process.env.LITENTRY_CLI_DIR;

    const linkIdentityRequestParams: {
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
        alice = genesisSubstrateWallet('Alice');
    });

    step('loading tokenmapping contract bytecode', async function () {
        const file = path.resolve('./', './contracts-build-info/TokenMapping.sol/TokenMapping.json');
        const data = fs.readFileSync(file, 'utf8');
        contractBytecode = JSON.parse(data).bytecode.object;
        assert.isNotEmpty(contractBytecode);
    });

    step('deploying tokenmapping contract via parachain pallet', async function () {
        const secretValue = 'my-secrets-value';
        const secretEncoded = context.api.createType('String', secretValue).toU8a();
        const encryptedSecrets = encryptWithTeeShieldingKey(teeShieldingKey, secretEncoded);

        const secret = '0x' + encryptedSecrets.toString('hex');

        const assertionId = '0x0000000000000000000000000000000000000011';
        const createAssertionEventsPromise = subscribeToEvents('evmAssertions', 'AssertionCreated', context.api);

        await context.api.tx.evmAssertions.createAssertion(assertionId, contractBytecode, [secret]).signAndSend(alice);

        const event = (await createAssertionEventsPromise).map((e) => e);
        assert.equal(event.length, 1);
    });

    step('linking identities (alice) via cli', async function () {
        const {
            protocol: workerProtocal,
            hostname: workerHostname,
            port: workerPort,
        } = new URL(process.env.WORKER_ENDPOINT!);
        const { protocol: nodeProtocal, hostname: nodeHostname, port: nodePort } = new URL(process.env.NODE_ENDPOINT!);
        const reqExtHash = '0x0000000000000000000000000000000000000000000000000000000000000000';
        const eventsPromise = subscribeToEventsWithExtHash(reqExtHash, context);

        // alice linking a evm identity
        try {
            const commandPromise = zx`${clientDir} -p ${nodePort} -P ${workerPort} -u ${
                nodeProtocal + nodeHostname
            } -U ${workerProtocal + workerHostname}\
                  trusted -d link-identity did:litentry:substrate:0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\
                  did:litentry:evm:0x4B04b9166f471a72e067d68560a141a1d02332Ef\
                  bsc`;

            await commandPromise;
        } catch (error: any) {
            console.log(`Exit code: ${error.exitCode}`);
            console.log(`Error: ${error.stderr}`);
            throw error;
        }

        const events = (await eventsPromise).map(({ event }) => event);
        assert.equal(events.length, 1);
    });

    step('requesting VC for deployed contract', async function () {
        await sleep(30);
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const nonce = (await getSidechainNonce(context, aliceSubstrateIdentity)).toNumber();

        const abiCoder = new ethers.utils.AbiCoder();
        const encodedData = abiCoder.encode(['string'], ['bnb']);

        const assertion = {
            dynamic: [Uint8Array.from(Buffer.from('0000000000000000000000000000000000000011', 'hex')), encodedData],
        };

        const requestVcCall = await createSignedTrustedCallRequestVc(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', nonce),
            context.web3Wallets.substrate.Alice,
            aliceSubstrateIdentity,
            context.api.createType('Assertion', assertion).toHex(),
            context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
            requestIdentifier,
            {
                withWrappedBytes: false,
                withPrefix: true,
            }
        );

        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, requestVcCall);
        await assertIsInSidechainBlock(`${Object.keys(assertion)[0]} requestVcCall`, res);
        assertVc(context, aliceSubstrateIdentity, res.value);
    });
});
