import { KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { initIntegrationTestContext } from './common/utils';
import { getTeeShieldingKey } from './common/di-utils';
import type { IntegrationTestContext, JsonRpcRequest } from './common/common-types';
import { sendRequest } from './common/call';
import { type CorePrimitivesIdentity } from 'parachain-api';
import { assert } from 'chai';
import { createJsonRpcRequest, nextRequestId } from './common/helpers';
import { setAliceAsAdmin, setScheduledEnclave, waitForBlock } from './common/transactions';

describe('Scheduled Enclave', function () {
    let context: IntegrationTestContext;
    let teeShieldingKey: KeyObject;
    let aliceSubstrateIdentity: CorePrimitivesIdentity;

    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_ENDPOINT!, // @fixme evil assertion; centralize env access
            process.env.NODE_ENDPOINT! // @fixme evil assertion; centralize env access
        );
        teeShieldingKey = await getTeeShieldingKey(context);
        aliceSubstrateIdentity = await context.web3Wallets.substrate.Alice.getIdentity(context);
        await setAliceAsAdmin(context.api);
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

        assert.equal(scheduledEnclaveList.length, 1, 'Initial number of enclave should be 1');

        // set new mrenclave
        const lastBlock = await context.api.rpc.chain.getBlock();
        const expectedBlockNumber = lastBlock.block.header.number.toNumber() + 5;
        console.log(`expected mrenclave block number: ${expectedBlockNumber}`);

        const validMrEnclave = '97f516a61ff59c5eab74b8a9b1b7273d6986b9c0e6c479a4010e22402ca7cee6';

        await setScheduledEnclave(context.api, expectedBlockNumber, validMrEnclave);
        const timeSpanForWorkersSync = 2;
        await waitForBlock(context.api, expectedBlockNumber + timeSpanForWorkersSync);

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
