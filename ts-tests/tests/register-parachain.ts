import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { loadConfig, launchRelayNodesAndParachainRegister } from './utils';

(async () => {
    console.log('Register parachain ...');
    const DEFAULT_CONFIG = loadConfig();
    const provider = new WsProvider(DEFAULT_CONFIG.relaynode_ws);
    const api = await ApiPromise.create({
        provider: provider,
        types: {
            // mapping the actual specified address format
            Address: 'MultiAddress',
            // mapping the lookup
            LookupSource: 'MultiAddress',
        },
    });

    await launchRelayNodesAndParachainRegister(api);
    await api.disconnect();
    provider.on('disconnected', () => {
        console.log('Disconnect from relaychain');
        process.exit(0);
    });
})();
