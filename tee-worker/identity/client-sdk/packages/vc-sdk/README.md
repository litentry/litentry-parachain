# @litentry/vc-sdk

This SDK provides the common functionality to help dApps parse and validate Litentry issued Verifiable Credentials.

A live example of this SDK's consumer application is identity-hub (https://idhub.litentry.io) where users request and generate their VCs and reveal their obtained VCs and share the relevant VCs should they consent to 3rd party applications.

## Install

```bash
npm install @litentry/parachain-api @litentry/sidechain-api @litentry/vc-sdk
```

## Examples

You can find more elaborated examples about the usage of this package on https://github.com/litentry/client-sdk-examples.

Below you will find some code snippets to guide you through the main functionalities.

### Validate Verifiable Credential (VC)

Interaction with the Litentry parachain and TEE to validate whether the VC is an active VC issued by the Litentry.

#### What the validateVc function do

- The validateVc function can only verify that the VC was issued by Litentry.
- The VC's credentialSubject can be Substrate or EVM account that is support by Litentry.

#### What the validateVc function can't do

- The validateVc function cannot validate that the VC's credentialSubject is the current wallet account. It's SDK's consumer's responsibility to validate the id of VC's credentialSubject is equal to the wallet address.

#### Example

```typescript
import { WsProvider, ApiPromise } from '@polkadot/api';
import { validateVc } from '@litentry/vc-sdk';

const api: ApiPromise = await ApiPromise.create({
  provider: new WsProvider('wss://tee-prod.litentry.io'),
});
// vc json string
const vcJson =
  '{"@context": "https://www.w3.org/2018/credentials/v1", "type": "VerifiableCredential", "issuer": "https://example.com/issuer", "subject": "did:example:123", "credentialStatus": "https://example.com/status"}';
const result = await validateVc(api, vcJson);

// isValid is false if any field value of the result.detail is not true
if (!result.isValid) {
  // true or error message
  console.log('vcSignature: ', result.detail.vcSignature);
  // true or error message
  console.log('enclaveRegistry: ', result.detail.enclaveRegistry);
}
```

Note for simplicity, in above example it's assumed that SDK consumer application has to provide the api instance of ApiPromise and manage its lifecycle as well.
