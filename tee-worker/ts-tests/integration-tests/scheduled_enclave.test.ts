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
});
