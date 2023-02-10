import { describeLitentry } from './utils';
import { step } from 'mocha-steps';
import { setErrorUserShieldingKey, createErrorIdentity } from './indirect_error_calls';
import { assert } from 'chai';

describeLitentry('setErrorUserShieldingKey test', async (context) => {
    const errorAseKey = '0xError';
    const errorCiphertext = '0xError';
    step('set error user shielding key', async function () {
        const result = await setErrorUserShieldingKey(context, context.defaultSigner[0], errorAseKey, true);
        assert.equal(
            result,
            'SetUserShieldingKeyHandlingFailed',
            'result is not equal to SetUserShieldingKeyHandlingFailed'
        );
    });

    step('create error identity', async function () {
        //The simulation generates the wrong Ciphertext
        const result = await createErrorIdentity(context, context.defaultSigner[0], true, errorCiphertext);
        assert.equal(result, 'CreateIdentityHandlingFailed', 'result is not equal to CreateIdentityHandlingFailed');
    });
});
