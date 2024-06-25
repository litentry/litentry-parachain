import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { buildWeb2Validation, initIntegrationTestContext } from './common/utils';
import { assertIsInSidechainBlock, assertVc } from './common/utils/assertion';
import {
    getSidechainNonce,
    createSignedTrustedCallLinkIdentity,
    getTeeShieldingKey,
    sendRequestFromTrustedCall,
    createSignedTrustedCallRequestVc,
    createSignedTrustedCallRequestBatchVc,
} from './common/di-utils'; // @fixme move to a better place
import { buildIdentityHelper, buildValidations } from './common/utils';
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import type { CorePrimitivesIdentity, WorkerRpcReturnValue } from 'parachain-api';
import type { LitentryValidationData, Web3Network } from 'parachain-api';
import type { Vec, Bytes } from '@polkadot/types';
import fs from 'fs';
import path from 'path';
import { assert } from 'chai';
import crypto from 'crypto';
import { genesisSubstrateWallet } from './common/helpers';
import { KeyringPair } from '@polkadot/keyring/types';
import { subscribeToEvents } from './common/transactions';
import {ethers} from 'ethers';
describe('Test Vc (direct request)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubstrateIdentity: CorePrimitivesIdentity = undefined as any;

    let alice: KeyringPair = undefined as any;
    let contractBytecode = undefined as any;

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
        const file = path.resolve('./', './compile.json');
        const data = fs.readFileSync(file, 'utf8');
        // contractBytecode = JSON.parse(data).bytecode.object;
        contractBytecode = JSON.parse(data).bytecode;
        console.log(contractBytecode);

        assert.isNotEmpty(contractBytecode);
    });

    step('deploying tokenmapping contract via parachain pallet', async function () {
        const encryptedSecrets = crypto.publicEncrypt(teeShieldingKey, Buffer.from('c99123cec85643da879f63654f0c642f'));
        const secret = '0x' + encryptedSecrets.toString('hex');

        const assertionId = '0x0000000000000000000000000000000000000000';
        const createAssertionEventsPromise = subscribeToEvents('evmAssertions', 'AssertionCreated', context.api);
        await context.api.tx.evmAssertions.createAssertion(assertionId, contractBytecode, [secret]).signAndSend(alice);
        const event = (await createAssertionEventsPromise).map((e) => e);
        assert.equal(event.length, 1);
    });

    step('linking identities (alice)', async function () {
        let currentNonce = (await getSidechainNonce(context, aliceSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;
        const evmNonce = getNextNonce();
        const evmIdentity = await context.web3Wallets.evm.Alice.getIdentity(context);
        const evmValidation = await buildValidations(
            context,
            aliceSubstrateIdentity,
            evmIdentity,
            evmNonce,
            'ethereum',
            context.web3Wallets.evm.Alice
        );
        const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);
        linkIdentityRequestParams.push({
            nonce: evmNonce,
            identity: evmIdentity,
            validation: evmValidation,
            networks: evmNetworks,
        });

        let counter = 0;
        for (const { nonce, identity, validation, networks } of linkIdentityRequestParams) {
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
                networks.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier,
                {
                    withWrappedBytes: false,
                    withPrefix: counter % 2 === 0, // alternate per entry
                }
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);
            await assertIsInSidechainBlock('linkIdentityCall', res);
        }
    });

    step('requesting VC for deployed contract', async function () {
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const nonce = (await getSidechainNonce(context, aliceSubstrateIdentity)).toNumber();

        const abiCoder = new ethers.utils.AbiCoder();
        const encodedData = abiCoder.encode(['string'], ['bnb']);
        
        const assertion = {
            dynamic: [Uint8Array.from(Buffer.from('0x0000000000000000000000000000000000000000', 'hex')), encodedData],
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
    });
});
