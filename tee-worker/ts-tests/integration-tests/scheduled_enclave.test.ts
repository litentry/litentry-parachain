import { KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { initIntegrationTestContext } from './common/utils';
import { getTeeShieldingKey } from './common/di-utils';
import type { IntegrationTestContext, JsonRpcRequest } from './common/common-types';
import { sendRequest } from './common/call';
import { Keyring, type ApiPromise, type CorePrimitivesIdentity } from 'parachain-api';
import { assert } from 'chai';
import { createJsonRpcRequest, nextRequestId } from './common/helpers';
import { hexToU8a } from '@polkadot/util';
import { subscribeToEvents, waitForBlock } from './common/transactions';

async function setScheduledEnclave(api: ApiPromise, block: number, mrenclave: string) {
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    const tx = api.tx.teebag.setScheduledEnclave('Identity', block, hexToU8a(`0x${mrenclave}`));

    console.log('Schedule Enclave Extrinsic sent');
    return new Promise<{ block: string }>(async (resolve, reject) => {
        await tx.signAndSend(alice, (result) => {
            console.log(`Current status is ${result.status}`);
            if (result.status.isInBlock) {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
            } else if (result.status.isFinalized) {
                console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
                resolve({
                    block: result.status.asFinalized.toString(),
                });
            } else if (result.status.isInvalid) {
                reject(`Transaction is ${result.status}`);
            }
        });
    });
}

describe('Scheduled Enclave', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubstrateIdentity: CorePrimitivesIdentity = undefined as any;

    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_ENDPOINT!, // @fixme evil assertion; centralize env access
            process.env.NODE_ENDPOINT! // @fixme evil assertion; centralize env access
        );
        teeShieldingKey = await getTeeShieldingKey(context);
        aliceSubstrateIdentity = await context.web3Wallets.substrate.Alice.getIdentity(context);
    });

    step('state of scheduled enclave list', async function () {
        const request = createJsonRpcRequest('state_getScheduledEnclave', undefined, nextRequestId(context));
        const response = await sendRequest(context.tee, request, context.api);
        const scheduledEnclaveList = context.api.createType('Vec<(u64, [u8; 32])>', response.value).toJSON() as [
            number,
            string
        ][];

        assert.equal(1, scheduledEnclaveList.length);

        const [blockNumber, mrEnclave] = scheduledEnclaveList[0];
        assert.equal(blockNumber, 0);
        assert.equal(mrEnclave, context.mrEnclave);
    });

    step('setting new scheduled enclave', async function () {
        const buildRequest = (method: string, params?: any[]) =>
            createJsonRpcRequest(method, params, nextRequestId(context));
        const callRPC = async (request: JsonRpcRequest) => sendRequest(context.tee, request, context.api);

        let response = await callRPC(buildRequest('state_getScheduledEnclave'));
        let scheduledEnclaveList = context.api.createType('Vec<(u64, [u8; 32])>', response.value).toJSON() as [
            number,
            string
        ][];

        assert.equal(scheduledEnclaveList.length, 1);

        // set new mrenclave
        const lastBlock = await context.api.rpc.chain.getBlock();
        const expectedBlockNumber = lastBlock.block.header.number.toNumber() + 2;
        console.log(`expected mrenclave block number: ${expectedBlockNumber}`);

        const validMrEnclave = '97f516a61ff59c5eab74b8a9b1b7273d6986b9c0e6c479a4010e22402ca7cee6';

        await setScheduledEnclave(context.api, expectedBlockNumber, validMrEnclave);
        const events = await subscribeToEvents('teebag', 'ScheduledEnclaveSet', context.api);
        assert.equal(events.length, 1);

        await waitForBlock(context.api, expectedBlockNumber);

        response = await callRPC(buildRequest('state_getScheduledEnclave'));
        scheduledEnclaveList = context.api.createType('Vec<(u64, [u8; 32])>', response.value).toJSON() as [
            number,
            string
        ][];

        assert.equal(scheduledEnclaveList.length, 2);

        const [blockNumber, mrEnclave] = scheduledEnclaveList[1];
        assert.equal(blockNumber, expectedBlockNumber);
        assert.equal(mrEnclave, `0x${validMrEnclave}`);
    });
});
