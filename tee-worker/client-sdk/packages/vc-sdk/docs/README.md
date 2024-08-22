@litentry/vc-sdk

# @litentry/vc-sdk

## Table of contents

### Type Aliases

- [Network](README.md#network)
- [ValidationResult](README.md#validationresult)
- [ValidationResultDetail](README.md#validationresultdetail)
- [VerifiableCredentialLike](README.md#verifiablecredentiallike)

### Variables

- [NETWORKS](README.md#networks)

### Functions

- [validateEnclaveRegistry](README.md#validateenclaveregistry)
- [validateVc](README.md#validatevc)
- [validateVcWithTrustedTeeDevEnclave](README.md#validatevcwithtrustedteedevenclave)

## Type Aliases

### Network

Ƭ **Network**: keyof typeof [`NETWORKS`](README.md#networks)

#### Defined in

[lib/validator.types.ts:4](https://github.com/litentry/client-sdk/blob/main/lib/validator.types.ts#L4)

___

### ValidationResult

Ƭ **ValidationResult**: `Object`

Represents the overall validation result for a Verifiable Credential (VC).

#### Type declaration

| Name | Type | Description |
| :------ | :------ | :------ |
| `detail` | [`ValidationResultDetail`](README.md#validationresultdetail) | Represents the whole validation result detail. |
| `isValid` | `boolean` | Represents the whole validation result status. If is true, means all fields of the detail are true, otherwise any one of it is not true. The caller should use this field to determine whether the VC is valid. |

#### Defined in

[lib/validator.types.ts:33](https://github.com/litentry/client-sdk/blob/main/lib/validator.types.ts#L33)

___

### ValidationResultDetail

Ƭ **ValidationResultDetail**: `Object`

Defines the details of the validation result for each component of the Verifiable Credential (VC).

#### Type declaration

| Name | Type | Description |
| :------ | :------ | :------ |
| `enclaveRegistry?` | ``true`` \| `string` | Represents the validation result (vcPubkey and mrEnclave) for the Enclave registry. If validation succeeds, it's true; if validation fails, it's an error message. The vcPubkey from Enclave registry must be same as issuer.id in VC JSON. The mrEnclave from Enclave registry must be same as issuer.mrenclave in VC JSON. |
| `vcSignature?` | ``true`` \| `string` | Represents the validation result for the VC signature. If validation succeeds, it's true; if validation fails, it's an error message. Use issuer.id in VC JSON as vcPubkey, proof.proofValue in VC JSON as signature to verify VC JSON. |

#### Defined in

[lib/validator.types.ts:9](https://github.com/litentry/client-sdk/blob/main/lib/validator.types.ts#L9)

___

### VerifiableCredentialLike

Ƭ **VerifiableCredentialLike**: `Record`\<`string`, `unknown`\> & \{ `@context`: `string` ; `credentialSubject`: `Record`\<`string`, `unknown`\> ; `id`: `string` ; `issuer`: \{ `id`: `string` ; `mrenclave`: `string` ; `name`: `string`  } ; `parachainBlockNumber?`: `number` ; `proof`: \{ `proofValue`: `string` ; `verificationMethod`: `string`  } ; `sidechainBlockNumber?`: `number` ; `type`: `string`[]  }

#### Defined in

[lib/validator.ts:11](https://github.com/litentry/client-sdk/blob/main/lib/validator.ts#L11)

## Variables

### NETWORKS

• `Const` **NETWORKS**: `Object`

Use to create WsProvider according to network.

#### Type declaration

| Name | Type |
| :------ | :------ |
| `litentry` | ``"wss://rpc.litentry-parachain.litentry.io"`` |
| `litentry-internal` | ``"wss://tee-internal.litentry.io"`` |
| `litentry-rococo` | ``"wss://rpc.rococo-parachain-sg.litentry.io"`` |
| `litentry-staging` | ``"wss://tee-staging.litentry.io"`` |

#### Defined in

[lib/validator.constants.ts:4](https://github.com/litentry/client-sdk/blob/main/lib/validator.constants.ts#L4)

## Functions

### validateEnclaveRegistry

▸ **validateEnclaveRegistry**(`api`, `vc`): `Promise`\<``true`` \| `string`\>

Validates that the VC was valid at the time it was issued.

#### Parameters

| Name | Type |
| :------ | :------ |
| `api` | `ApiPromise` |
| `vc` | [`VerifiableCredentialLike`](README.md#verifiablecredentiallike) |

#### Returns

`Promise`\<``true`` \| `string`\>

#### Defined in

[lib/validator.ts:231](https://github.com/litentry/client-sdk/blob/main/lib/validator.ts#L231)

___

### validateVc

▸ **validateVc**(`api`, `vc`): `Promise`\<[`ValidationResult`](README.md#validationresult)\>

Validate Verifiable Credential (VC)

### How to find the wallet account address in VC

The id of VC's credentialSubject is the encoded wallet account address, using code below to encode:

```typescript
import { decodeAddress } from '@polkadot/util-crypto';
import { u8aToHex } from '@polkadot/util';

const address = u8aToHex(decodeAddress(walletAccountAddress));
const credentialSubjectId = address.substring(2);
```

With the code above:
- If your Polkadot account address is `5CwPfqmormPx9wJ4ASq7ikwdJeRonoUZX9SxwUtm1px9L72W`, the credentialSubjectId will be `26a84b380d8c3226d69f9ae6e482aa6669ed34c6371c52c4dfb48596913d6f28`.
- If your Metamask account address is `0xC620b3e5BEBedA952A8AD18b83Dc4Cf3Dc9CAF4b`, the credentialSubjectId will be `c620b3e5bebeda952a8ad18b83dc4cf3dc9caf4b`.

### What the validation function do

- The validation function can only verify that the VC was issued by Litentry.
- The VC's credentialSubject can be Substrate or EVM account that is support by Litentry.

### What the validation function can't do

- The validation function cannot validate that the VC's credentialSubject is the current wallet account. It's SDK's consumer's responsibility to validate the id of VC's credentialSubject is equal to the wallet address.

### How to use

```typescript
import { WsProvider, ApiPromise } from '@polkadot/api';
import { validateVc, NETWORKS } from '@litentry/vc-sdk';

const api: ApiPromise = await ApiPromise.create({
  provider: new WsProvider(NETWORKS['litentry-staging'])
});
const vcJson = '{"@context": "https://www.w3.org/2018/credentials/v1", "type": "VerifiableCredential", "issuer": "https://example.com/issuer", "subject": "did:example:123", "credentialStatus": "https://example.com/status"}';
const result = await validateVc(api, vcJson);

// isValid is false if any field value of the result.detail is not true
if (!result.isValid) {
  // true or error message
  console.log('vcJson: ', result.detail.vcJson);
  // true or error message
  console.log('vcRegistry: ', result.detail.vcRegistry);
  // true or error message
  console.log('vcSignature: ', result.detail.vcSignature);
  // true or error message
  console.log('enclaveRegistry: ', result.detail.enclaveRegistry);
}
```

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `api` | `ApiPromise` | The instance of ApiPromise. |
| `vc` | `string` | The VC JSON string that needs to be validated. |

#### Returns

`Promise`\<[`ValidationResult`](README.md#validationresult)\>

The validation result.

#### Defined in

[lib/validator.ts:86](https://github.com/litentry/client-sdk/blob/main/lib/validator.ts#L86)

___

### validateVcWithTrustedTeeDevEnclave

▸ **validateVcWithTrustedTeeDevEnclave**(`api`, `vc`): ``true`` \| `string`

Validates the VC information against the past enclave registry.
These Enclave may no longer be online but these are Enclave that issued the VC and we trust.

exported for testing purposes.

#### Parameters

| Name | Type |
| :------ | :------ |
| `api` | `ApiPromise` |
| `vc` | [`VerifiableCredentialLike`](README.md#verifiablecredentiallike) |

#### Returns

``true`` \| `string`

#### Defined in

[lib/validator.ts:297](https://github.com/litentry/client-sdk/blob/main/lib/validator.ts#L297)
