Example golang code for connecting to bitacross worker and requesting ethereum signature through trusted direct rpc.

It connects to worker's trusted direct rpc endpoint exposed at `wss://localhost:2000` and requests `SignEthereum` direct call using JSON-RPC 2.0 protocol.
Direct call is signed by ethereum keypair.

### Running
Specify worker's trusted rpc port as first argument

`go run example --port 2000`