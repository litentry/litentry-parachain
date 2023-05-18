import '../config';

// in order to handle self-signed certificates we need to turn off the validation
// TODO add self signed certificate ??
process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

export * from './assertion';
export * from './common';
export * from './context';
export * from './crypto';
export * from './identity';
export * from './storage';
export * from './test';
