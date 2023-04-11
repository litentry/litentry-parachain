
//run:npx ts-node setup-enclave-script.ts

import { initApi } from "./index";
const { hexToU8a } = require('@polkadot/util');

//100 token
const transferAmount = '100000000000000';
const enclaveAccount = '5EtChN6SSfL7E13fEqKQn4jZztDtZkSiRqfFTv2BKufL5fzZ'
const mrenclave = 'a552654d1733c4054a3c7e5e86adf26b5d65c072b57b2550fe763821ebac54c6'
function transfer(api: any) {
    const transfer_hex =
        api.tx.balances.transfer(enclaveAccount, transferAmount).toHex();
    return transfer_hex
}
async function updateScheduledEnclave(api: any) {
    return api.tx.sudo.sudo(
        api.tx.teerex.updateScheduledEnclave(
            0, hexToU8a(`0x${mrenclave}`)
        )
    ).toHex()
}
async function main() {

    const { syncApi } = await initApi();

    const transfer_hex = transfer(syncApi);
    console.log('transfer_hex', transfer_hex);

    const updateScheduledEnclave_hex = await updateScheduledEnclave(syncApi);
    console.log('updateScheduledEnclave_hex', updateScheduledEnclave_hex);

    console.log("done");
    process.exit()

}

main();
