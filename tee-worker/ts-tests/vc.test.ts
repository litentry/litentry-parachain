import { describeLitentry } from './utils';
import { step } from 'mocha-steps';
import { requestVC, setUserShieldingKey } from './indirect_calls';
import { Assertion } from './type-definitions';
import { assert } from 'chai';
import { u8aToHex } from '@polkadot/util';
const assertion = <Assertion>{
    A1: 'A1',
    A2: ['A2', 'A2'],
    A3: ['A3', 'A3'],
    A4: [10, 'A4'],
    A7: [10, 10],
    A8: 'A8',
    A10: [10, 10],
};
describeLitentry('VC test', async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    step('set user shielding key', async function () {
        const who = await setUserShieldingKey(context, context.defaultSigner[0], aesKey, true);
        assert.equal(who, u8aToHex(context.defaultSigner[0].addressRaw), 'check caller error');
    });
    step('Request VC', async () => {
        for (const key in assertion) {
            const eventData = await requestVC(context, context.defaultSigner[0], aesKey, true, context.shard, {
                [key]: assertion[key as keyof Assertion],
            });
            assert(eventData![0] == u8aToHex(context.defaultSigner[0].addressRaw) && eventData![1] && eventData![2]);
            const registry = (await context.substrate.query.vcManagement.vcRegistry(eventData![1])) as any;
            assert.equal(registry.toHuman()!['status'], 'Active');
        }
    });
});
