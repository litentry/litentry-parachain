export type Maybe<T> = null | undefined | T;

export type JsonRpcRequest = {
  jsonrpc: string;
  method: string;
  params: Array<string>;
  /**
   * Heads-up: it should work with string and any number but we found out
   * that responses are not coming back if the number is too big or a string :sad-panda:
   * @see Enclave Reverse Proxy `/apps/identity-hub/pages/api/enclave.ts`
   */
  id?: number;
};

export type JsonRpcResponse = {
  jsonrpc: string;
  id: number | string;
  result: string;
};

/**
 * Substrate SS58 address, Substrate public key or EVM address
 */
export type SubstrateOrEvmOrBtcAddress = string | `0x${string}`;
