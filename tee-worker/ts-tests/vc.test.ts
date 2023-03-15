import { describeLitentry, checkVc } from './utils';
import { step } from 'mocha-steps';
import { setUserShieldingKey, requestVCs, disableVCs, revokeVCs } from './indirect_calls';
import { Assertion } from './type-definitions';
import { assert } from 'chai';
import { u8aToHex } from '@polkadot/util';
import { HexString } from '@polkadot/util/types';
import { blake2AsHex } from '@polkadot/util-crypto';
import { requestErrorVCs, disableErrorVCs, revokeErrorVCs } from './indirect_error_calls';

const assertion = <Assertion>{
    A1: 'A1',
    A2: ['A2'],
    A3: ['A3', 'A3', 'A3'],
    A4: [10],
    A7: [10],
    A8: ['litentry'],
    A10: [10],
    A11: [10],
};
describeLitentry('VC test', async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    var indexList: HexString[] = [];
    step('set user shielding key', async function () {
        const who = await setUserShieldingKey(context, context.defaultSigner[0], aesKey, true);
        assert.equal(who, u8aToHex(context.defaultSigner[0].addressRaw), 'check caller error');
    });
    step('Request VC', async () => {
        // request all vc
        const res = (await requestVCs(
            context,
            context.defaultSigner[0],
            aesKey,
            true,
            context.mrEnclave,
            assertion
        )) as {
            account: HexString;
            index: HexString;
            vc: HexString;
        }[];

        for (let k = 0; k < res.length; k++) {
            const vcString = res[k].vc.replace('0x', '');
            const vcObj = JSON.parse(vcString);
            console.log('---------VC json----------', vcObj);

            const vcProof = vcObj.proof;

            const registry = (await context.substrate.query.vcManagement.vcRegistry(res[k].index)) as any;
            assert.equal(registry.toHuman()!['status'], 'Active', 'check registry error');

            const vcHash = blake2AsHex(Buffer.from(vcString));
            assert.equal(vcHash, registry.toHuman()!['hash_'], 'check vc json hash error');

            //check vc
            const vcValid = await checkVc(vcObj, res[k].index, vcProof, context.substrate);
            assert.equal(vcValid, true, 'check vc error');
            indexList.push(res[k].index);
        }
    });
    step('Request Error VC', async () => {
        const resp_request_error = (await requestErrorVCs(
            context,
            context.defaultSigner[1],
            aesKey,
            true,
            context.mrEnclave,
            assertion
        )) as any;

        resp_request_error.map((item: any) => {
            const result = item.toHuman().data.reason;

            assert(
                result.search('User shielding key is missing') !== -1,
                'requestVc should fail with reason `User shielding key is missing`'
            );
        });
    });
    step('Disable VC', async () => {
        const res = (await disableVCs(context, context.defaultSigner[0], aesKey, true, indexList)) as HexString[];
        for (let k = 0; k < res.length; k++) {
            assert.equal(res[k], indexList[k], 'check index error');
            const registry = (await context.substrate.query.vcManagement.vcRegistry(indexList[k])) as any;
            assert.equal(registry.toHuman()!['status'], 'Disabled');
        }
    });
    step('Disable error VC', async () => {
        //Bob dont't request VC before
        const resp_disable_error = (await disableErrorVCs(
            context,
            context.defaultSigner[0],
            true,
            indexList
        )) as HexString[];
        for (let k = 0; k < resp_disable_error.length; k++) {
            assert.equal(resp_disable_error[k], 'vcManagement.VCAlreadyDisabled', 'check disableVc  error');
        }
    });

    step('Revoke VC', async () => {
        const res = (await revokeVCs(context, context.defaultSigner[0], aesKey, true, indexList)) as HexString[];
        for (let k = 0; k < res.length; k++) {
            assert.equal(res[k], indexList[k], 'check index error');
            const registry = (await context.substrate.query.vcManagement.vcRegistry(indexList[k])) as any;
            assert.equal(registry.toHuman(), null);
        }
    });

    step('Revoke Error VC', async () => {
        const resp_revoke_error = (await revokeErrorVCs(
            context,
            context.defaultSigner[0],
            true,
            indexList
        )) as string[];
        for (let k = 0; k < resp_revoke_error.length; k++) {
            assert.equal(resp_revoke_error[k], 'vcManagement.VCNotExist', 'check revokeVc  error');
        }
    });
});
