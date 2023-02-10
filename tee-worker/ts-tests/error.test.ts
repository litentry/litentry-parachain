import { describeLitentry } from './utils';
import { step } from 'mocha-steps';
import { setErrorUserShieldingKey } from './indirect_error_calls';
import { assert } from 'chai';
import { u8aToHex } from '@polkadot/util';

describeLitentry('setErrorUserShieldingKey test', async (context) => {
    const errorAseKey = '0xError';
    step('set user shielding key', async function () {
        const result = await setErrorUserShieldingKey(context, context.defaultSigner[0], errorAseKey, true);
        assert.equal(
            result,
            'SetUserShieldingKeyHandlingFailed',
            'result is not equal to SetUserShieldingKeyHandlingFailed'
        );
    });
});
