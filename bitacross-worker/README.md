#  BitAcross worker

This repository contains code for BitAcross offchain worker. The main responsibility of the worker is to 
store custodian wallets and sign transactions submitted by relayers.

## Wallets

Supported wallets: 
* ethereum (ecdsa based on secp256k1) 
* bitcoin (schnorr based on secp256k1)

Wallets (private keys) are generated during the initialization (on first startup) and sealed to encrypted file using Intel Protected File System while public keys are published on parachain's bitacross pallet in compressed SEC1-encoded format.  


## Transaction signing

Signing requests are processed by a dedicated JSON-RPC `bitacross_submitRequest` method and results in raw signature bytes. Only requests signed by registered relayers are permitted.

Typescript code related to the RPC integration and can be found in [tee-worker's ts-tests](https://github.com/litentry/litentry-parachain/blob/a6b78ed68396280655271f9cd30e17535d54da81/tee-worker/ts-tests/integration-tests/common/di-utils.ts).

Rust code used in CLI module can also be used as a reference and can be found [here](https://github.com/litentry/litentry-parachain/blob/a6b78ed68396280655271f9cd30e17535d54da81/bitacross-worker/cli/src/trusted_base_cli/commands/bitacross/utils.rs).


### Step by step guide for request preparing/sending and response handling. 

1. Prepare `DirectCall`, for example `SignBitcoin` variant which will reflect bitcoin's transaction signing request. Generate 256-bit AES-GCM as request enc/dec key. The first parameter is relayer identity, second generated aes key and third is transaction payload to sign.

```rust
pub enum DirectCall {
	SignBitcoin(Identity, RequestAesKey, Vec<u8>),
	SignEthereum(Identity, RequestAesKey, Vec<u8>),
}
```

2. Prepare `DirectCallSigned`. Scale encode created direct call from step 1, append scale encoded mrenclave and shard identifier (use mrenclave) to it, do a Blake2 256-bit hash of it and sign it using relayer's private key, then prepare struct containing direct call and signature. Mrenclave can be obtained from parachain's teebag pallet enclave registry storage.

```Rust
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct DirectCallSigned {
	pub call: DirectCall,
	pub signature: LitentryMultiSignature,
}
```

3. Prepare `AesRequest`. Fill payload property with aes encrypted scale encoded `DirectCallSigned` prepared in previous step. Get worker's shielding key and use it to encrypt aes key from step 1 and fill `key` property with it. Fill shard identifier (use mrenclave again). Shielding key can be obtained from parachain's teebag pallet enclave registry storage.

```rust
pub struct AesRequest {
	pub shard: ShardIdentifier,
	pub key: Vec<u8>,
	pub payload: AesOutput,
}
``` 

4. Prepare `RpcRequest`. Scale encode `AesRequest` prepared in previous step, turn the bytes into hex representation and put in `params` vec. Generate locally unique `id`. Use following consts for `jsonrpc` and `method`: `"2.0"`, `bitacross_submitRequest` 

```rust
pub struct RpcRequest {
	pub jsonrpc: String,
	pub method: String,
	pub params: Vec<String>,
	pub id: Id,
}
```

5. Send prepared request to worker's jsonrpc endpoint, in the following example `websocat` is used to send request to locally run bitacross worker.

```bash
echo '{"jsonrpc":"2.0","method":"bitacross_submitRequest","params":["0x6565c4529cd2af40f89e5d526c6e890019a2fd33cfdc9ee3cd14a0bf1427a61601065c22cde40abe4ad0550a4beba5d05a55380117a57824a57c5949a472fb0639d1ebb1baff0f5453e222418844044ed75352f9a76b4f3fd57f8db4deabf4074eb552784b32c1a881ac27d143148e06a3607455ebafb7dd3ab1669013502bfd7b840d6698363015f55fede5275dfe7d05827315301772e4b75bf745f74b71c443b97b7d22010d54b89fcc1105cbfc72a58dfbd4c10e34ef6019dad859abafdb4f82118f5f339255cb5d2400243bc2e982b4c60341572b6253e0815ed90de74b64145aef8d8304a576ba11c73421b9c86a053619908c475be5d223acc942460afb7e248836f58d2e639d3e32365bbc7ba9fe838b3329db6432fce3427569523f513e7cc82098db4ccaf024a286ad94e6be775ba1f9e918f0867e20a8dbb409232ba297878eff52740e705f59dab2a1c5827d1f8bf7adfa7cdf9e345c16fda757016337f398201af14c820782dac82bc9c5f8df93c917cba29f89e5a1e323dafcf2465e258f1d6dcf9808e5202e6fa3766433981f619c580b831c0d49eed759a0ca1555021c688b72490ffd3f4391c60c04ba904d83aa9497cce62eb6d0e55124692c5124fabfabd70ab366ba81d152f2299ba99021a3705754d64d2b9455229d6ecd730a120a1003abe432a060e40931ad9eb3199cbb09a6b2c84af35735b51628d80210369c0f902905f7e7902d6787673691f2e923b6bc001cfa56f3568e95a95f1f084cd69e658e42c96e317cebc17d54de13f08a0fb007008777e7510d0aa8d124271afe"],"id":1}' | websocat -n1 -k -B 99999999 wss://localhost:2000
{"jsonrpc":"2.0","result":"0x7d014101d9197274039df1280452819ede02d0867aa57185251d19c9e2c74bd22d1f3b8e1db031b068e3ee7229631a804c1d03e2d9af7055851ac9609dae5e8c7c8dccf4961b31a5cc98bfa356fa262a7376525300a2c03d6f2b59e28578fdcee00000","id":1}
```

6. Get result from response `result` field. It's a hex representation of scale encoded `RpcReturnValue`.  In case of success, the signature can be obtained from `value` property (it's aes encrypted).

Types definitions:

```rust
pub type RequestAesKey = [u8; 32];

#[derive(
Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen, EnumIter, Ord, PartialOrd,
)]
pub enum Identity {
    // web2
    #[codec(index = 0)]
    Twitter(IdentityString),
    #[codec(index = 1)]
    Discord(IdentityString),
    #[codec(index = 2)]
    Github(IdentityString),

    // web3
    #[codec(index = 3)]
    Substrate(Address32),
    #[codec(index = 4)]
    Evm(Address20),
    // bitcoin addresses are derived (one-way hash) from the pubkey
    // by using `Address33` as the Identity handle, it requires that pubkey
    // is retrievable by the wallet API when verifying the bitcoin account.
    // e.g. unisat-wallet: https://docs.unisat.io/dev/unisat-developer-service/unisat-wallet#getpublickey
    #[codec(index = 5)]
    Bitcoin(Address33),
}

pub enum LitentryMultiSignature {
    /// An Ed25519 signature.
    #[codec(index = 0)]
    Ed25519(ed25519::Signature),
    /// An Sr25519 signature.
    #[codec(index = 1)]
    Sr25519(sr25519::Signature),
    /// An ECDSA/SECP256k1 signature.
    #[codec(index = 2)]
    Ecdsa(ecdsa::Signature),
    /// An ECDSA/keccak256 signature. An Ethereum signature. hash message with keccak256
    #[codec(index = 3)]
    Ethereum(EthereumSignature),
    /// Same as above, but the payload bytes are prepended with a readable prefix and `0x`
    #[codec(index = 4)]
    EthereumPrettified(EthereumSignature),
    /// Bitcoin signed message, a hex-encoded string of original &[u8] message, without `0x` prefix
    #[codec(index = 5)]
    Bitcoin(BitcoinSignature),
    /// Same as above, but the payload bytes are prepended with a readable prefix and `0x`
    #[codec(index = 6)]
    BitcoinPrettified(BitcoinSignature),
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub struct EthereumSignature(pub [u8; 65]);

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub struct BitcoinSignature(pub [u8; 65]);

#[derive(
Encode, Decode, Copy, Clone, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen, Ord, PartialOrd,
)]
pub struct Address20([u8; 20]);

#[derive(
Encode, Decode, Copy, Clone, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen, Ord, PartialOrd,
)]
pub struct Address32([u8; 32]);

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen, PartialOrd, Ord)]
pub struct Address33([u8; 33]);

#[derive(Debug, Default, Clone, Eq, PartialEq, Encode, Decode)]
pub struct AesOutput {
    pub ciphertext: Vec<u8>,
    pub aad: Vec<u8>,
    pub nonce: RequestAesKeyNonce, // IV
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub struct RpcReturnValue {
    pub value: Vec<u8>,
    pub do_watch: bool,
    pub status: DirectRequestStatus,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode, Eq)]
pub enum DirectRequestStatus {
    /// Direct request was successfully executed
    #[codec(index = 0)]
    Ok,
    /// Trusted Call Status
    /// Litentry: embed the top hash here - TODO - use generic type?
    #[codec(index = 1)]
    TrustedOperationStatus(TrustedOperationStatus, H256),
    /// Direct request could not be executed
    #[codec(index = 2)]
    Error,
    #[codec(index = 3)]
    Processing
}
```

### Using CLI

There are two commands related to transaction signing:

* request-direct-call-sign-bitcoin
* request-direct-call-sign-ethereum

They take single argument representing raw payload bytes to sign.

#### Example usage

```bash
./bitacross-cli trusted -m 7ppBUcnjGir4szRHCG59p2dTnbtRwKRbLZPpR32ACjbK request-direct-call-sign-bitcoin 00
```

### Obtaining data from parachain's teebag pallet

Mrencalve, worker's url and public shielding key can be obtained during the runtime from parachain's teebag pallet registry. 

The following gif ilustrates how it can be done manually:

![demo](./assets/teebag_registry.gif)

These values can also be obtained programmatically using substrate's `state_getStorage` RPC method. See [this](https://docs.substrate.io/build/remote-procedure-calls/) documentation for more information.