
//run: npx ts-node move-vcregistry-snapshot-script.ts

import { initApi } from "./index";
const fs = require("fs");
const path = require("path");
const prettier = require("prettier");
async function encodeExtrinsic() {
    const { fetchApi, syncApi } = await initApi();
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
    fs.writeFile(filepath, formattedData, (err: any) => {
        if (err) {
            console.error(err);
            return;
        }
        console.log(`Data saved to ${filename} successfully.`);
    });
    let txs: any = [];
    for (let index = 0; index < data.length; index++) {
        let tx = syncApi.tx.vcManagement.addVcRegistryItem(
            data[index].index[0],
            data[index].vc.subject,
            data[index].vc.assertion,
            data[index].vc.hash_
        );
        txs.push(tx);
    }

    const extrinsic = syncApi.tx.sudo.sudo(syncApi.tx.utility.batch(txs));
    console.log("extrinsic encode", extrinsic.toHex());
    console.log("done");
    process.exit()
}

encodeExtrinsic();
