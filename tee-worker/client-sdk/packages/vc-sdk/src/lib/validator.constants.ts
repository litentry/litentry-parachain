/**
 * Use to create WsProvider according to network.
 */
export const NETWORKS = {
  'litentry-internal': 'wss://tee-internal.litentry.io',
  'litentry-staging': 'wss://tee-staging.litentry.io',
  'litentry-rococo': 'wss://rpc.rococo-parachain-sg.litentry.io',
  litentry: 'wss://rpc.litentry-parachain.litentry.io',
} as const;
