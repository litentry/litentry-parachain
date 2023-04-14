
//run:npx ts-node transfer-to-whitelist.ts
import * as XLSX from 'xlsx';
import { initApi } from "./initApis";
import colors from 'colors';
let whiteList: any;

//100 token
const transferAmount = '100000000000000';

function transfer(api: any) {

    let txs: any = [];
    for (let index = 0; index < whiteList.length; index++) {
        try {
            let tx = api.tx.balances.transfer(whiteList[index], transferAmount);
            txs.push(tx);
        } catch (error: any) {
            //maybe invalied address or other error remove it from whitelist
            console.log(colors.red('transfer Error: '), `${error.message}. Removing ${whiteList[index]} from whiteList.`);
            whiteList.splice(index, 1);
            index--;
        }
    }
    const transfer_hex =
        api.tx.utility.batchAll(txs)
            .toHex()

    return transfer_hex
}

async function main() {
    //download whitelist from google sheet (https://docs.google.com/spreadsheets/d/1QD0gVraqDDOkdJk-vhLMZEbdnbAOt_ogiJ3uHau_1Kw/edit#gid=950765040)
    //put the whitelist in the same folder as this script
    //read whitelist from excel
    const workbook = XLSX.readFile('Whitelist R1 & R2.xlsx');

    //read sheet 'Whitelisted Addresses for R1'
    const sheet = workbook.Sheets['Whitelisted Addresses for R1'];

    //read the second column of the sheet and skip the first row
    whiteList = XLSX.utils.sheet_to_json(sheet, { header: 1 })
        .map((row: any) => row[1]).slice(1);

    const { defaultAPI } = await initApi();

    const transfer_hex = transfer(defaultAPI);
    console.log(colors.green('transfer_hex'), transfer_hex);
    console.log(colors.green('done'));
    process.exit()

}

main();