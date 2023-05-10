## This folder contains a simple example of sending direct invocation to the tee worker.

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

-   the event in sidechain is not yet supported (i.e. will not be included in the block)
-   some requests (e.g. VC request, identity verification) are async handled, the trusted call being included in a sidechain block doesn't mean the processing is complete, the request was routed to another thread for async processing.
-   currently the `value` represents the trusted operation hash, to change it to the encoded event (e.g. IdentityCreated) with parameters, the code needs to be adjusted in depth: not an easy task

4. the subscription of parachain is not implemented in this demo - it's verified via polkadot-js UI in my tests

### TODOs:

-   the `WorkerRpcReturnValue` type is manually defined, we need to use the generated type - they differ a bit
-   more direct calls should be implemented in rust
