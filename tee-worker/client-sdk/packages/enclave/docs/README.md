@litentry/enclave

# @litentry/enclave

## Table of contents

### References

- [getIdGraphHash](README.md#getidgraphhash)

### Namespaces

- [request](modules/request.md)

### Classes

- [Enclave](classes/Enclave.md)

### Type Aliases

- [DiscordOAuth2Proof](README.md#discordoauth2proof)
- [DiscordProof](README.md#discordproof)
- [TwitterOAuth2Proof](README.md#twitteroauth2proof)
- [TwitterProof](README.md#twitterproof)
- [Web3Proof](README.md#web3proof)

### Variables

- [enclave](README.md#enclave)

### Functions

- [calculateIdGraphHash](README.md#calculateidgraphhash)
- [createKeyAesOutputType](README.md#createkeyaesoutputtype)
- [createLitentryIdentityType](README.md#createlitentryidentitytype)
- [createLitentryValidationDataType](README.md#createlitentryvalidationdatatype)
- [createRequestType](README.md#createrequesttype)
- [createTrustedCallType](README.md#createtrustedcalltype)
- [toPublicKey](README.md#topublickey)

## References

### getIdGraphHash

Re-exports [getIdGraphHash](modules/request.md#getidgraphhash)

## Type Aliases

### DiscordOAuth2Proof

Ƭ **DiscordOAuth2Proof**: `Object`

Ownership proof for Discord accounts using oAuth2

**`See`**

createLitentryValidationDataType

#### Type declaration

| Name | Type |
| :------ | :------ |
| `code` | `string` |
| `redirectUri` | `string` |

#### Defined in

[lib/type-creators/validation-data.ts:58](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/validation-data.ts#L58)

___

### DiscordProof

Ƭ **DiscordProof**: `Object`

Ownership proof for Discord accounts

**`See`**

createLitentryValidationDataType

#### Type declaration

| Name | Type |
| :------ | :------ |
| `channelId` | `string` |
| `guildId` | `string` |
| `messageId` | `string` |

#### Defined in

[lib/type-creators/validation-data.ts:48](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/validation-data.ts#L48)

___

### TwitterOAuth2Proof

Ƭ **TwitterOAuth2Proof**: `Object`

Ownership proof for Twitter accounts using oAuth2

**`See`**

createLitentryValidationDataType

#### Type declaration

| Name | Type |
| :------ | :------ |
| `code` | `string` |
| `redirectUri` | `string` |
| `state` | `string` |

#### Defined in

[lib/type-creators/validation-data.ts:37](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/validation-data.ts#L37)

___

### TwitterProof

Ƭ **TwitterProof**: `Object`

Ownership proof for Twitter accounts

**`See`**

createLitentryValidationDataType

#### Type declaration

| Name | Type |
| :------ | :------ |
| `tweetId` | `string` |

#### Defined in

[lib/type-creators/validation-data.ts:30](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/validation-data.ts#L30)

___

### Web3Proof

Ƭ **Web3Proof**: `Object`

Ownership proof for Web3 accounts (Substrate, EVM, Bitcoin).

Bitcoin signatures are base64-encoded strings. Substrate and EVM signatures are hex-encoded strings.

**`See`**

createLitentryIdentityType

#### Type declaration

| Name | Type |
| :------ | :------ |
| `message` | `string` |
| `signature` | \`0x$\{string}\` \| `string` |

#### Defined in

[lib/type-creators/validation-data.ts:20](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/validation-data.ts#L20)

## Variables

### enclave

• `Const` **enclave**: [`Enclave`](classes/Enclave.md)

Enclave instance

This an instance of the Enclave class.

**`See`**

Enclave

**`Example`**

```ts
import { enclave } from '@litentry/identity-hub';

const shard = await enclave.getShard(api);
const key = await enclave.getKey(api);
const encrypted = await enclave.encrypt(api, { cleartext: new Uint8Array([1, 2, 3]) });
const response = await enclave.send({
 jsonrpc: '2.0',
 method: 'author_submitAndWatch',
 params: ['0x123']
});
```

#### Defined in

[lib/enclave.ts:298](https://github.com/litentry/client-sdk/blob/develop/lib/enclave.ts#L298)

## Functions

### calculateIdGraphHash

▸ **calculateIdGraphHash**(`idGraph`): \`0x$\{string}\`

Returns the hash of the given id graph. It matches the hash used in the Litentry Parachain.

#### Parameters

| Name | Type |
| :------ | :------ |
| `idGraph` | `IdGraph` |

#### Returns

\`0x$\{string}\`

#### Defined in

[lib/util/calculate-id-graph-hash.ts:9](https://github.com/litentry/client-sdk/blob/develop/lib/util/calculate-id-graph-hash.ts#L9)

___

### createKeyAesOutputType

▸ **createKeyAesOutputType**(`registry`, `data`): `AesOutput`

Creates a KeyAesOutput sidechain type.

Heads-up: ensure data.ciphertext is in hex format. Using Uint may cause a bytes out range error.

#### Parameters

| Name | Type |
| :------ | :------ |
| `registry` | `Registry` |
| `data` | \`0x$\{string}\` \| \{ `aad`: \`0x$\{string}\` \| `Uint8Array` ; `ciphertext`: \`0x$\{string}\` ; `nonce`: \`0x$\{string}\` \| `Uint8Array`  } |

#### Returns

`AesOutput`

**`Example`**

build from object
```ts
const identity = createKeyAesOutputType(registry, {
 ciphertext: '0x...',
 nonce: '0x...',
 aad: '0x...',
});
```

build from hex string
```ts
const identity = createKeyAesOutputType(registry, `0x...`);
```

#### Defined in

[lib/type-creators/key-aes-output.ts:27](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/key-aes-output.ts#L27)

___

### createLitentryIdentityType

▸ **createLitentryIdentityType**(`registry`, `data`): `LitentryIdentity`

Creates a LitentryIdentity chain type.

Notice that addresses and handles are not fully validated. This struct shouldn't be relied on for validation.

For Bitcoin, the compressed public key is expected (begins with 02 or 03).

For Solana, the address string is expected to be a base58-encoded or hex-encoded string.

For Substrate, the address is expected to be a SS58-encoded or hex-encoded address.

#### Parameters

| Name | Type |
| :------ | :------ |
| `registry` | `Registry` |
| `data` | `Object` |
| `data.addressOrHandle` | `string` |
| `data.type` | ``"Solana"`` \| ``"Twitter"`` \| ``"Discord"`` \| ``"Github"`` \| ``"Substrate"`` \| ``"Evm"`` \| ``"Bitcoin"`` |

#### Returns

`LitentryIdentity`

**`Example`**

```ts
const substrateIdentity = createLitentryIdentityType(registry, {
 addressOrHandle: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
 type: 'Substrate',
});

const twitterIdentity = createLitentryIdentityType(registry, {
 addressOrHandle: 'my-twitter-handle',
 type: 'Twitter',
});
```

#### Defined in

[lib/type-creators/litentry-identity.ts:31](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/litentry-identity.ts#L31)

___

### createLitentryValidationDataType

▸ **createLitentryValidationDataType**\<`IIdentityType`\>(`registry`, `identityDescriptor`, `proof`): `LitentryValidationData`

Creates the LitentryValidationData given the identity network and its type.

The proof to pass depends on the identity network (IIdentityType):
- Web3: Web3Proof
- Twitter: TwitterProof
- Discord: DiscordProof

#### Type parameters

| Name | Type |
| :------ | :------ |
| `IIdentityType` | extends ``"Solana"`` \| ``"Twitter"`` \| ``"Discord"`` \| ``"Github"`` \| ``"Substrate"`` \| ``"Evm"`` \| ``"Bitcoin"`` |

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `registry` | `Registry` | Litentry Parachain API's type registry |
| `identityDescriptor` | `Object` | - |
| `identityDescriptor.addressOrHandle` | `string` | The address or handle of the identity |
| `identityDescriptor.type` | `IIdentityType` | The identity type |
| `proof` | `IIdentityType` extends ``"Discord"`` ? [`DiscordProof`](README.md#discordproof) \| [`DiscordOAuth2Proof`](README.md#discordoauth2proof) : `IIdentityType` extends ``"Twitter"`` ? [`TwitterProof`](README.md#twitterproof) \| [`TwitterOAuth2Proof`](README.md#twitteroauth2proof) : [`Web3Proof`](README.md#web3proof) | The ownership proof |

#### Returns

`LitentryValidationData`

**`Example`**

Web3
```ts
import { createLitentryValidationDataType } from '@litentry/enclave';
import type { Web3Proof } from '@litentry/enclave';

const userAddress = '0x123';

const proof: Web3Proof = {
  signature: '0x123',
  message: '0x123',
}

const validationData = createLitentryValidationDataType(
  registry,
  {
    addressOrHandle: userAddress,
    type: 'Evm',
  },
  proof,
);
```

**`Example`**

Twitter
```ts
import { createLitentryValidationDataType } from '@litentry/enclave';
import type { TwitterProof } from '@litentry/enclave';

const userHandle = '@litentry';

const proof: TwitterProof = {
  // Both twitter.com and x.com are valid
  tweetId: 'https://twitter.com/0x123/status/123',
};

const validationData = createLitentryValidationDataType(
  registry,
  {
    addressOrHandle: userHandle,
    type: 'Twitter',
  },
  proof,
);
```

#### Defined in

[lib/type-creators/validation-data.ts:116](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/validation-data.ts#L116)

___

### createRequestType

▸ **createRequestType**(`api`, `data`): `Promise`\<`AesRequest`\>

Creates a Request struct type for the `TrustedCall` operation.

A shielding key is generated and used to encrypt the `TrustedCall` operation and communicated
to the enclave to protect the data for transportation.

The shielding key is encrypted using the Enclave's shielding key and attached in the Request.

#### Parameters

| Name | Type |
| :------ | :------ |
| `api` | `ApiPromise` |
| `data` | `Object` |
| `data.call` | `TrustedCall` |
| `data.nonce` | `Index` |
| `data.shard` | `Uint8Array` |
| `data.signature` | `string` |
| `data.who` | `LitentryIdentity` |

#### Returns

`Promise`\<`AesRequest`\>

#### Defined in

[lib/type-creators/request.ts:30](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/request.ts#L30)

___

### createTrustedCallType

▸ **createTrustedCallType**(`registry`, `data`): `Promise`\<\{ `call`: `TrustedCall` ; `key`: `CryptoKey`  }\>

Creates the TrustedCall for the given method and provide the `param's` types expected for them.

Heads-up:
This must match the Rust implementation of the TrustedCall

#### Parameters

| Name | Type |
| :------ | :------ |
| `registry` | `Registry` |
| `data` | `Object` |
| `data.method` | ``"link_identity"`` |
| `data.params` | `LinkIdentityParams` |

#### Returns

`Promise`\<\{ `call`: `TrustedCall` ; `key`: `CryptoKey`  }\>

**`See`**

 - https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/app-libs/stf/src/trusted_call.rs

Similarly, our types definitions must match also.
 - https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/client-api/parachain-api/prepare-build/interfaces/trusted_operations/definitions.ts

#### Defined in

[lib/type-creators/trusted-call.ts:69](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/trusted-call.ts#L69)

▸ **createTrustedCallType**(`registry`, `data`): `Promise`\<\{ `call`: `TrustedCall` ; `key`: `CryptoKey`  }\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `registry` | `Registry` |
| `data` | `Object` |
| `data.method` | ``"set_identity_networks"`` |
| `data.params` | `SetIdentityNetworksParams` |

#### Returns

`Promise`\<\{ `call`: `TrustedCall` ; `key`: `CryptoKey`  }\>

#### Defined in

[lib/type-creators/trusted-call.ts:76](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/trusted-call.ts#L76)

▸ **createTrustedCallType**(`registry`, `data`): `Promise`\<\{ `call`: `TrustedCall` ; `key`: `CryptoKey`  }\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `registry` | `Registry` |
| `data` | `Object` |
| `data.method` | ``"request_vc"`` |
| `data.params` | `RequestVcParams` |

#### Returns

`Promise`\<\{ `call`: `TrustedCall` ; `key`: `CryptoKey`  }\>

#### Defined in

[lib/type-creators/trusted-call.ts:83](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/trusted-call.ts#L83)

▸ **createTrustedCallType**(`registry`, `data`): `Promise`\<\{ `call`: `TrustedCall` ; `key`: `CryptoKey`  }\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `registry` | `Registry` |
| `data` | `Object` |
| `data.method` | ``"request_batch_vc"`` |
| `data.params` | `RequestBatchVcParams` |

#### Returns

`Promise`\<\{ `call`: `TrustedCall` ; `key`: `CryptoKey`  }\>

#### Defined in

[lib/type-creators/trusted-call.ts:90](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/trusted-call.ts#L90)

___

### toPublicKey

▸ **toPublicKey**(`address`): \`0x$\{string}\`

Returns the public key from a given substrate address.

#### Parameters

| Name | Type |
| :------ | :------ |
| `address` | `string` |

#### Returns

\`0x$\{string}\`

**`Throws`**

if the address is not a valid substrate address

#### Defined in

[lib/type-creators/litentry-identity.ts:78](https://github.com/litentry/client-sdk/blob/develop/lib/type-creators/litentry-identity.ts#L78)
