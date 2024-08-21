[@litentry/enclave](../README.md) / request

# Namespace: request

requests

## Table of contents

### Functions

- [createChallengeCode](request.md#createchallengecode)
- [getIdGraph](request.md#getidgraph)
- [getIdGraphHash](request.md#getidgraphhash)
- [getLastRegisteredEnclave](request.md#getlastregisteredenclave)
- [linkIdentity](request.md#linkidentity)
- [requestBatchVC](request.md#requestbatchvc)
- [setIdentityNetworks](request.md#setidentitynetworks)

## Functions

### createChallengeCode

▸ **createChallengeCode**(`api`, `args`): `Promise`\<`string`\>

Generates the challenge code to link an identity.

The challenge code is calculated from:

```
blake2_256(<enclaveNonce> + <primaryAccount> + <identityToLink>)
```

The output is a hex string. For Bitcoin `identity`, the hex's prefix `0x` is removed.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `api` | `ApiPromise` | Litentry Parachain API instance from Polkadot.js |
| `args` | `Object` | - |
| `args.identity` | `LitentryIdentity` | Identity to be linked. Use `createCorePrimitivesIdentityType` helper to create this struct |
| `args.who` | `LitentryIdentity` | The user's account. Use `createCorePrimitivesIdentityType` helper to create this struct |

#### Returns

`Promise`\<`string`\>

#### Defined in

[lib/requests/link-identity.request.ts:34](https://github.com/litentry/client-sdk/blob/develop/lib/requests/link-identity.request.ts#L34)

___

### getIdGraph

▸ **getIdGraph**(`api`, `data`): `Promise`\<\{ `payloadToSign`: `string` ; `send`: (`args`: \{ `signedPayload`: `string`  }) => `Promise`\<\{ `idGraph`: `IdGraph` ; `response`: `WorkerRpcReturnValue`  }\>  }\>

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `api` | `ApiPromise` | - |
| `data` | `Object` | - |
| `data.who` | `LitentryIdentity` | The user's account. Use `createLitentryIdentityType` helper to create this struct |

#### Returns

`Promise`\<\{ `payloadToSign`: `string` ; `send`: (`args`: \{ `signedPayload`: `string`  }) => `Promise`\<\{ `idGraph`: `IdGraph` ; `response`: `WorkerRpcReturnValue`  }\>  }\>

#### Defined in

[lib/requests/get-id-graph.request.ts:14](https://github.com/litentry/client-sdk/blob/develop/lib/requests/get-id-graph.request.ts#L14)

___

### getIdGraphHash

▸ **getIdGraphHash**(`api`, `«destructured»`): `Promise`\<`H256` \| ``null``\>

Retrieve the idGraphHash for a given identity.

#### Parameters

| Name | Type |
| :------ | :------ |
| `api` | `ApiPromise` |
| `«destructured»` | `Object` |
| › `who` | `LitentryIdentity` |

#### Returns

`Promise`\<`H256` \| ``null``\>

#### Defined in

[lib/requests/get-id-graph-hash.ts:13](https://github.com/litentry/client-sdk/blob/develop/lib/requests/get-id-graph-hash.ts#L13)

___

### getLastRegisteredEnclave

▸ **getLastRegisteredEnclave**(`api`, `workerType?`): `Promise`\<\{ `account`: `AccountId32` ; `enclave`: `PalletTeebagEnclave`  }\>

Return the Enclave registry information of the latest registered TEE worker.

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `api` | `ApiPromise` | `undefined` |
| `workerType` | ``"Identity"`` \| ``"BitAcross"`` | `'Identity'` |

#### Returns

`Promise`\<\{ `account`: `AccountId32` ; `enclave`: `PalletTeebagEnclave`  }\>

#### Defined in

[lib/requests/get-last-registered-enclave.ts:11](https://github.com/litentry/client-sdk/blob/develop/lib/requests/get-last-registered-enclave.ts#L11)

___

### linkIdentity

▸ **linkIdentity**(`api`, `data`): `Promise`\<\{ `payloadToSign`: `string` ; `send`: (`args`: \{ `signedPayload`: `string`  }) => `Promise`\<\{ `idGraphHash`: \`0x$\{string}\` ; `mutatedIdentities`: `IdGraph` ; `response`: `WorkerRpcReturnValue` ; `txHash`: `string`  }\> ; `txHash`: `string`  }\>

Link an identity to the user's account.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `api` | `ApiPromise` | Litentry Parachain API instance from Polkadot.js |
| `data` | `Object` | - |
| `data.identity` | `LitentryIdentity` | Identity to be linked. Use `createCorePrimitivesIdentityType` helper to create this struct |
| `data.networks` | (``"Polkadot"`` \| ``"Kusama"`` \| ``"Litentry"`` \| ``"Litmus"`` \| ``"LitentryRococo"`` \| ``"Khala"`` \| ``"SubstrateTestnet"`` \| ``"Ethereum"`` \| ``"Bsc"`` \| ``"BitcoinP2tr"`` \| ``"BitcoinP2pkh"`` \| ``"BitcoinP2sh"`` \| ``"BitcoinP2wpkh"`` \| ``"BitcoinP2wsh"`` \| ``"Polygon"`` \| ``"Arbitrum"`` \| ``"Solana"`` \| ``"Combo"``)[] | The networks to link the identity to, for web3 accounts |
| `data.validation` | `LitentryValidationData` | The ownership proof. Use `createLitentryValidationDataType` helper to create this struct |
| `data.who` | `LitentryIdentity` | The prime identity. Use `createCorePrimitivesIdentityType` helper to create this struct |

#### Returns

`Promise`\<\{ `payloadToSign`: `string` ; `send`: (`args`: \{ `signedPayload`: `string`  }) => `Promise`\<\{ `idGraphHash`: \`0x$\{string}\` ; `mutatedIdentities`: `IdGraph` ; `response`: `WorkerRpcReturnValue` ; `txHash`: `string`  }\> ; `txHash`: `string`  }\>

#### Defined in

[lib/requests/link-identity.request.ts:57](https://github.com/litentry/client-sdk/blob/develop/lib/requests/link-identity.request.ts#L57)

___

### requestBatchVC

▸ **requestBatchVC**(`api`, `data`): `Promise`\<\{ `payloadToSign`: `string` ; `send`: (`args`: \{ `signedPayload`: `string`  }, `subscribeFn?`: (`error`: `Error` \| ``null``, `data`: \{ `index`: `number` ; `partialResult`: `WorkerRpcReturnValue`[] ; `vcPayload`: `Uint8Array`  }) => `void`) => `Promise`\<\{ `response`: `WorkerRpcReturnValue`[] ; `txHash`: `string` ; `vcPayloads`: (`Uint8Array` \| `Error`)[]  }\> ; `txHash`: `string`  }\>

Request a Batch of Verifiable Credential (VC) from the Litentry Protocol.

The send's subscribeFn is optional and is used to process the VC payload as it comes in.

The final response array, contains WorkerRpcReturnValue as they come in from the Enclave.
Notice that the response array is not ordered. Decoding the `WorkerRpcReturnValue.value`
into `RequestVcResultOrError` will contain the index of the request and the payload or error.

The information about available assertions and their payload can be found in the
`Assertion` (`Assertion`) type.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `api` | `ApiPromise` | Litentry Parachain API instance from Polkadot.js |
| `data` | `Object` | - |
| `data.assertions` | `Assertion`[] | the assertions to be claimed. See `Assertion` type |
| `data.who` | `LitentryIdentity` | The user's account. Use `createLitentryIdentityType` helper to create this struct |

#### Returns

`Promise`\<\{ `payloadToSign`: `string` ; `send`: (`args`: \{ `signedPayload`: `string`  }, `subscribeFn?`: (`error`: `Error` \| ``null``, `data`: \{ `index`: `number` ; `partialResult`: `WorkerRpcReturnValue`[] ; `vcPayload`: `Uint8Array`  }) => `void`) => `Promise`\<\{ `response`: `WorkerRpcReturnValue`[] ; `txHash`: `string` ; `vcPayloads`: (`Uint8Array` \| `Error`)[]  }\> ; `txHash`: `string`  }\>

#### Defined in

[lib/requests/request-batch-vc.request.ts:35](https://github.com/litentry/client-sdk/blob/develop/lib/requests/request-batch-vc.request.ts#L35)

___

### setIdentityNetworks

▸ **setIdentityNetworks**(`api`, `data`): `Promise`\<\{ `payloadToSign`: `string` ; `send`: (`args`: \{ `signedPayload`: `string`  }) => `Promise`\<\{ `idGraphHash`: \`0x$\{string}\` ; `mutatedIdentities`: `IdGraph` ; `response`: `WorkerRpcReturnValue` ; `txHash`: `string`  }\> ; `txHash`: `string`  }\>

Set the networks for a Web3 Identity.

It allows to change the list of `networks` for an already linked web3 identity.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `api` | `ApiPromise` | Litentry Parachain API instance from Polkadot.js |
| `data` | `Object` | - |
| `data.identity` | `LitentryIdentity` | Identity to be modified. Use `createLitentryIdentityType` helper to create this struct |
| `data.networks` | (``"Polkadot"`` \| ``"Kusama"`` \| ``"Litentry"`` \| ``"Litmus"`` \| ``"LitentryRococo"`` \| ``"Khala"`` \| ``"SubstrateTestnet"`` \| ``"Ethereum"`` \| ``"Bsc"`` \| ``"BitcoinP2tr"`` \| ``"BitcoinP2pkh"`` \| ``"BitcoinP2sh"`` \| ``"BitcoinP2wpkh"`` \| ``"BitcoinP2wsh"`` \| ``"Polygon"`` \| ``"Arbitrum"`` \| ``"Solana"`` \| ``"Combo"``)[] | Networks to be set |
| `data.who` | `LitentryIdentity` | The user's account. Use `createLitentryIdentityType` helper to create this struct |

#### Returns

`Promise`\<\{ `payloadToSign`: `string` ; `send`: (`args`: \{ `signedPayload`: `string`  }) => `Promise`\<\{ `idGraphHash`: \`0x$\{string}\` ; `mutatedIdentities`: `IdGraph` ; `response`: `WorkerRpcReturnValue` ; `txHash`: `string`  }\> ; `txHash`: `string`  }\>

#### Defined in

[lib/requests/set-identity-networks.request.ts:26](https://github.com/litentry/client-sdk/blob/develop/lib/requests/set-identity-networks.request.ts#L26)
