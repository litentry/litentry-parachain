import { KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { initIntegrationTestContext } from './common/utils';
import { getTeeShieldingKey } from './common/di-utils';
import type { IntegrationTestContext, JsonRpcRequest } from './common/common-types';
import { decodeRpcBytesAsString, sendRequest } from './common/call';
import type { CorePrimitivesIdentity } from 'parachain-api';
import { assert } from 'chai';
import { createJsonRpcRequest, nextRequestId } from './common/helpers';

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
        let setEnclaveResponse = await callRPC(
            buildRequest('state_setScheduledEnclave', [20, 'some invalid mrenclave'])
        );
        assert.include(decodeRpcBytesAsString(setEnclaveResponse.value), 'Failed to decode mrenclave');

        setEnclaveResponse = await callRPC(buildRequest('state_setScheduledEnclave', [20, '48656c6c6f20776f726c6421']));
        assert.include(
            decodeRpcBytesAsString(setEnclaveResponse.value),
            'mrenclave len mismatch, expected 32 bytes long'
        );

        // valid mutation
        const validParams = [20, '97f516a61ff59c5eab74b8a9b1b7273d6986b9c0e6c479a4010e22402ca7cee6'];
        await callRPC(buildRequest('state_setScheduledEnclave', validParams));

        // checking mutated state
        response = await callRPC(buildRequest('state_getScheduledEnclave'));
        scheduledEnclaveList = context.api.createType('Vec<(u64, [u8; 32])>', response.value).toJSON() as [
            number,
            string
        ][];

        assert.equal(scheduledEnclaveList.length, 2);

        const [blockNumber, mrEnclave] = scheduledEnclaveList[1];
        assert.equal(blockNumber, validParams[0]);
        assert.equal(mrEnclave, `0x${validParams[1]}`);
    });
});
