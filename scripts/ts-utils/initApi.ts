const { ApiPromise, WsProvider } = require("@polkadot/api");
const { cryptoWaitReady } = require('@polkadot/util-crypto');
//fetchApi is used to fetch data from the chain
const wsFetchProvider = new WsProvider("wss://tee-staging.litentry.io:443");

//syncApi is used to send transactions to the chain
const wsSyncProvider = new WsProvider("ws://localhost:9944");

export const initApi = async () => {
    const fetchApi = await ApiPromise.create({ provider: wsFetchProvider });
    const syncApi = await ApiPromise.create({ provider: wsSyncProvider });
    await cryptoWaitReady();
    return { fetchApi, syncApi };
}


