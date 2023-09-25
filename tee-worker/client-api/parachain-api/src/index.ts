import '@polkadot/api/augment';
import '@polkadot/types-augment';
import '@polkadot/types/lookup';

import '../build/interfaces/types-lookup.js';
export * from '@polkadot/api/types';
export * from '@polkadot/api';
export * from '@polkadot/types/lookup';
export * from '../build/interfaces';

import { identity } from '../build/interfaces/definitions';

export const definitions = identity;
