
//run: npx ts-node move-vcregistry-snapshot.ts

import { initApi } from "./initApis";
const fs = require("fs");
const path = require("path");
const prettier = require("prettier");
import colors from 'colors';

//set the maximal calls are 500 per batch
const BATCH_SIZE = 500;
async function encodeExtrinsic() {
    const { fetchApi, defaultAPI } = await initApi();
    console.log(colors.green('get vcRegistry entries...'))

    const entries = await fetchApi.query.vcManagement.vcRegistry.entries();
    const data = entries.map((res: any) => {
        return { index: res[0].toHuman(), vc: res[1].toHuman() };
    });

    const filename = `VCRegistry-${new Date().toISOString().slice(0, 10)}.json`;
    const filepath = path.join(__dirname, filename);
    const formattedData = prettier.format(JSON.stringify(data), {
        parser: "json",
        printWidth: 120,
        tabWidth: 2,
        singleQuote: true,
        trailingComma: "es5",
    });
    fs.writeFileSync(filepath, formattedData);
    console.log(colors.green(`Data saved to ${filename} successfully.`));

    let txs: any[] = [];
    console.log(colors.green('vcRegistry data length'), data.length);
    let i = 0
    while (data.length > 0) {
        const batch = data.splice(0, BATCH_SIZE);
        const batchTxs = batch.map((entry: any) =>
            defaultAPI.tx.vcManagement.addVcRegistryItem(
                entry.index[0],
                entry.vc.subject,
                entry.vc.assertion,
                entry.vc.hash_
            )
        );
        txs = txs.concat(batchTxs);

        if (data.length === 0 || txs.length >= BATCH_SIZE) {
            i++
            const extrinsics = defaultAPI.tx.utility.batch(batchTxs);
            const sudoExtrinsic = defaultAPI.tx.sudo.sudo(extrinsics);
            console.log(colors.green(`extrinsic ${i} encode`), sudoExtrinsic.toHex());
            txs = [];
        }
    }
    console.log(colors.green('done'));
    process.exit();
}

encodeExtrinsic();