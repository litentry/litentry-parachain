const { ApiPromise, WsProvider } = require("@polkadot/api");
const { cryptoWaitReady } = require('@polkadot/util-crypto');
import colors from "colors";
//fetchApi is used to fetch data from the chain
const wsFetchProvider = new WsProvider("wss://tee-staging.litentry.io:443");

//defaultAPI is used to send transactions to the chain
const wsDefaultProvider = new WsProvider("ws://localhost:9944");

export const initApi = async () => {
    console.log(colors.green("init api..."))
    const fetchApi = await ApiPromise.create({ provider: wsFetchProvider });
    const defaultAPI = await ApiPromise.create({ provider: wsDefaultProvider });
    await cryptoWaitReady();
    console.log(colors.green("api is ready"))

    return { fetchApi, defaultAPI };
}


