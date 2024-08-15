import './lib/interfaces/types-lookup';
import './lib/interfaces/augment-types';
import './lib/interfaces/augment-api';

export * from './lib/interfaces/definitions';
export * from './lib/interfaces/types';

// Handy types
import type {
  LitentryIdentity,
  Web3Network,
} from './lib/interfaces/identityManagement/types';

export type SubstrateNetwork = Extract<
  Web3Network['type'],
  | 'Polkadot'
  | 'Kusama'
  | 'Litentry'
  | 'Litmus'
  | 'LitentryRococo'
  | 'Khala'
  | 'SubstrateTestnet'
>;

export type EvmNetwork = Extract<
  Web3Network['type'],
  'Ethereum' | 'Bsc' | 'Polygon' | 'Arbitrum' | 'Combo'
>;

export type SolanaNetwork = Extract<Web3Network['type'], 'Solana'>;

export type BitcoinNetwork = Exclude<
  Web3Network['type'],
  SubstrateNetwork | EvmNetwork | SolanaNetwork
>;

export type Web2Network = Exclude<
  LitentryIdentity['type'],
  'Substrate' | 'Evm' | 'Bitcoin' | 'Solana'
>;

/**
 * Identities that can be used as prime identity to own an idGraph.
 */
export type PrimeIdentity = Extract<
  LitentryIdentity['type'],
  'Substrate' | 'Evm' | 'Bitcoin' | 'Solana'
>;
