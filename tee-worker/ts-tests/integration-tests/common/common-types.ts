import { ApiPromise } from 'parachain-api';
import { KeyObject } from 'crypto';
import WebSocketAsPromised from 'websocket-as-promised';
import { Metadata, TypeRegistry } from '@polkadot/types';
import { Wallet } from 'ethers';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { HexString } from '@polkadot/util/types';
import bitcore from 'bitcore-lib';

// If there are types already defined in the client-api, please avoid redefining these types.
// Instead, make every effort to use the types that have been generated within the client-api.
interface EthersWalletItem {
    [key: string]: Wallet;
}
interface SubstrateWalletItem {
    [key: string]: KeyringPair;
}
interface BitcoinWalletItem {
    [key: string]: bitcore.PrivateKey;
}
export type IntegrationTestContext = {
    tee: WebSocketAsPromised;
    api: ApiPromise;
    teeShieldingKey: KeyObject;
    mrEnclave: HexString;
    ethersWallet: EthersWalletItem;
    substrateWallet: SubstrateWalletItem;
    bitcoinWallet: BitcoinWalletItem;
    sidechainMetaData: Metadata;
    sidechainRegistry: TypeRegistry;
    chainIdentifier: number;
    requestId: number;
};

export type Web3Wallets = {
    substrateWallet: KeyringPair;
    evmWallet: Wallet;
    bitcoinWallet: bitcore.PrivateKey;
};

export type JsonRpcRequest = {
    jsonrpc: string;
    method: string;
    params: any;
    id: number;
};
