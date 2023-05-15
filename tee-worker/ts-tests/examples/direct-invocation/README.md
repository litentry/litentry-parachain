This folder contains a simple example of sending direct invocation to the tee worker

### How to run the demo
1. launch the parachain and worker:
```
cd <path-to-tee-worker>
./local-setup/launch.py local-setup/github-action-config-one-worker.json
```

2. run the ts demo
```
cd <path-to-tee-worker/ts-tests/examples/direct-invocation>
ts-node index.ts
```

### Notes

1. the raw RPC response is the standard JSONRPC response. E.g.

```
{"jsonrpc":"2.0","result":"0x6d6574610b6c185379737....","id":29}
```

The field `result` contains the hex-encoded `WorkerRpcReturnValue` struct, after decoding to that type it should be able to be represented as JSON, example:

```
{
  value: '0x20dc135fc04a6ef0e1a885dea7b0cdb4b0bd2ea1eb8d108ca2e6638ac0eec423',
  do_watch: false,
  status: {
    trustedOperationStatus: {
      inSidechainBlock: '0x0b6389bd951f8075d6662ddc35d03f2978d0667d8433557cc551868517e58670'
    }
  }
}
```

-   `value` field contains the real response content in an opaque hex-encoded way. The consumer must know to which type it should be decoded.
-   `do_watch` indicates if there're more (streamed) responses coming
-   `status` has 3 options: `ok`, `trustedOperationStatus`, or `error`:
    -   `ok` is returned when the request is a "normal" request, e.g. getting TEE's shielding key
    -   `trustedOperationStatus` is a dedicated status for a trusted operation submission/subscription request to indicate its status in the pool, similar to transaction pool status in the parachain

2. there's no extrinsic hash as it's not extrinsic, instead a random H256 value needs to be generated and sent along with the request to pair the response. However, it might be not strictly needed depending on how the ws channel is used (if it's used exclusively for a single request).

3. The execution result of a sidechain state mutation (e.g. creating an identity) is returned via parachain event. There's room to improve it but it's kept like the old way because:
   - the event in sidechain is not yet supported (i.e. will not be included in the block)
   - some requests (e.g. VC request, identity verification) are async handled, the trusted call being included in a sidechain block doesn't mean the processing is complete, the request was routed to another thread for async processing.
   - currently the `value` represents the trusted operation hash, to change it to the encoded event (e.g. IdentityCreated) with parameters, the code needs to be adjusted in depth: not an easy task

4. the subscription of parachain is not implemented in this demo - it's verified via polkadot-js UI and worker-logs in my tests

```
Send direct setUserShieldingKey call... hash: 0x82fa3970886ed11cdc8a7f98b48df20c9d939fefe86e0d225b47dd35d1aa8d8e
response status:  { trustedOperationStatus: { submitted: null } }
response status:  {
  trustedOperationStatus: {
    inSidechainBlock: '0xe4c9a63a26635e53087ff2cc0aadca480ae59cb83223f1be5cb1dab225f83c07'
  }
}
setUserShieldingKey call returned
Send direct createIdentity call... hash: 0xb355245c0f58533b127ed5196ddaaf5d521a0b96d332b9d020c5289c4716519c
response status:  { trustedOperationStatus: { submitted: null } }
response status:  {
  trustedOperationStatus: {
    inSidechainBlock: '0x6487f58b3523dd1ab133a00f0bffc8b578ac6ce8778da60e57656357cec76504'
  }
}
```
<img width="899" alt="image" src="https://github.com/litentry/litentry-parachain/assets/7630809/2f0f70cc-c25f-4069-b8c1-0376ab954a77">

<img width="862" alt="image" src="https://github.com/litentry/litentry-parachain/assets/7630809/489b3ccc-a22b-4aa8-9b8a-ef82914ad181">

### TODOs:

-   the `WorkerRpcReturnValue` type is manually defined, we need to use the generated type - they differ a bit
-   more direct calls should be implemented in rust
