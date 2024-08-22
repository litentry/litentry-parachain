[@litentry/enclave](../README.md) / Enclave

# Class: Enclave

This is a singleton class to mainly hold the Enclave's Shielding Key and Shard.

With this class you can:
- Retrieve the Enclave's Shielding Key. (1)
- Retrieve the Enclave's MREnclave value which is used as the Shard value. (1)
- Encrypt data using the Enclave's Shielding Key.
- Send request to the Enclave. This is also known as Direct Invocation. (2)

(1) This is done by querying the Parachain. Its shielding key can be retrieved directly but
since the MREnclave cannot we opted to leave both from the Parachain in the meantime.

(2) This is done by using a reverse proxy API. By default, it expects the proxy API to be
running on `/api/enclave`. See `createEnclaveHttpProxyHandler` for more details.

**`Example`**

```ts
import { Enclave } from '@litentry/enclave';

const enclave = new Enclave(); // same as the `enclave` variable exported from this module
const shard = await enclave.getShard(api);
const key = await enclave.getKey(api);

console.log({ shard, key });

// Encrypt data using the Enclave's Shielding Key
const encrypted = await enclave.encrypt(api, { cleartext: new Uint8Array([1, 2, 3]) });

// Send request to the Enclave directly. This is also known as Direct Invocation.
const response = await enclave.send({
 jsonrpc: '2.0',
 method: 'author_submitAndWatch',
 params: ['0x123']
});
```

## Table of contents

### Constructors

- [constructor](Enclave.md#constructor)

### Properties

- [#key](Enclave.md##key)
- [#shard](Enclave.md##shard)
- [#instance](Enclave.md##instance)

### Methods

- [encrypt](Enclave.md#encrypt)
- [getKey](Enclave.md#getkey)
- [getShard](Enclave.md#getshard)
- [retrieveKeyAndShard](Enclave.md#retrievekeyandshard)
- [send](Enclave.md#send)

## Constructors

### constructor

• **new Enclave**(): [`Enclave`](Enclave.md)

#### Returns

[`Enclave`](Enclave.md)

#### Defined in

[lib/enclave.ts:66](https://github.com/litentry/client-sdk/blob/develop/lib/enclave.ts#L66)

## Properties

### #key

• `Private` **#key**: ``null`` \| `CryptoKey` = `null`

#### Defined in

[lib/enclave.ts:63](https://github.com/litentry/client-sdk/blob/develop/lib/enclave.ts#L63)

___

### #shard

• `Private` **#shard**: ``null`` \| \`0x$\{string}\` = `null`

#### Defined in

[lib/enclave.ts:64](https://github.com/litentry/client-sdk/blob/develop/lib/enclave.ts#L64)

___

### #instance

▪ `Static` `Private` **#instance**: [`Enclave`](Enclave.md)

#### Defined in

[lib/enclave.ts:62](https://github.com/litentry/client-sdk/blob/develop/lib/enclave.ts#L62)

## Methods

### encrypt

▸ **encrypt**(`api`, `args`): `Promise`\<\{ `ciphertext`: `Uint8Array`  }\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `api` | `ApiPromise` |
| `args` | `Object` |
| `args.cleartext` | `Uint8Array` |

#### Returns

`Promise`\<\{ `ciphertext`: `Uint8Array`  }\>

#### Defined in

[lib/enclave.ts:88](https://github.com/litentry/client-sdk/blob/develop/lib/enclave.ts#L88)

___

### getKey

▸ **getKey**(`api`): `Promise`\<`CryptoKey`\>

Get the Enclave's Shielding Key.

The value will be held in memory for the duration of the session.

#### Parameters

| Name | Type |
| :------ | :------ |
| `api` | `ApiPromise` |

#### Returns

`Promise`\<`CryptoKey`\>

#### Defined in

[lib/enclave.ts:112](https://github.com/litentry/client-sdk/blob/develop/lib/enclave.ts#L112)

___

### getShard

▸ **getShard**(`api`): `Promise`\<\`0x$\{string}\`\>

Get the Enclave's Shard. Also referred as MREnclave.

The value will be held in memory for the duration of the session.

#### Parameters

| Name | Type |
| :------ | :------ |
| `api` | `ApiPromise` |

#### Returns

`Promise`\<\`0x$\{string}\`\>

#### Defined in

[lib/enclave.ts:123](https://github.com/litentry/client-sdk/blob/develop/lib/enclave.ts#L123)

___

### retrieveKeyAndShard

▸ **retrieveKeyAndShard**(`api`): `Promise`\<\{ `key`: `CryptoKey` ; `shard`: \`0x$\{string}\`  }\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `api` | `ApiPromise` |

#### Returns

`Promise`\<\{ `key`: `CryptoKey` ; `shard`: \`0x$\{string}\`  }\>

#### Defined in

[lib/enclave.ts:74](https://github.com/litentry/client-sdk/blob/develop/lib/enclave.ts#L74)

___

### send

▸ **send**(`api`, `_payload`, `subscribeFn?`): `Promise`\<`WorkerRpcReturnValue`[]\>

Send requests to the Enclave.

The subscribeFn is a callback that will be called for every message received from the Enclave.

The Enclave WebSocket will be closed after the response is completed. A long-lived connection
is not offered but should be feasible.

For single messages, it will throw an error if the response contains an error.

#### Parameters

| Name | Type |
| :------ | :------ |
| `api` | `ApiPromise` |
| `_payload` | `JsonRpcRequest` |
| `subscribeFn?` | (`message`: `WorkerRpcReturnValue`, `partialResult`: `WorkerRpcReturnValue`[]) => `Promise`\<`void`\> |

#### Returns

`Promise`\<`WorkerRpcReturnValue`[]\>

#### Defined in

[lib/enclave.ts:140](https://github.com/litentry/client-sdk/blob/develop/lib/enclave.ts#L140)
