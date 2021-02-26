# ocw manual test instruction
1. start the local token server
    cp token-server/.env.example .env
    update the API tokens in .env
    source .env
    target/release/litentry-token-server
2. start the litentry node
    target/release/litentry-node --dev
3. create ocw session account and send to ocw module via curl command
   bash-5.0$ subkey generate

  Secret phrase `loop high amazing chat tennis auto denial attend type quit liquid tonight` is account:
  Secret seed:      0xad9e7d8233eff5b32ebdf1cfd6d2007f0bfa7c73f7d2d7e60f95dbd642a8af54
  Public key (hex): 0x8c35b97c56099cf3b5c631d1f296abbb11289857e74a8f60936290080d56da6d
  Account ID:       0x8c35b97c56099cf3b5c631d1f296abbb11289857e74a8f60936290080d56da6d
  SS58 Address:     5FEYX9NES9mAJt1Xg4WebmHWywxyeGQK8G3oEBXtyfZrRePX

$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d \
  '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"author_insertKey",
    "params": [
      "ocw!",
      "loop high amazing chat tennis auto denial attend type quit liquid tonight",
      "0x8c35b97c56099cf3b5c631d1f296abbb11289857e74a8f60936290080d56da6d"
    ]
  }'
4. transfer some token to ocw account in UI
   transaction -> balances -> transfer -> from Alice to 5FEYX9NES9mAJt1Xg4WebmHWywxyeGQK8G3oEBXtyfZrRePX
5. link eth account to Alice
   transaction -> AccountLinderModule -> linkEth

    eth address 0x4d88dc5d528a33e4b8be579e9476715f60060582
    block number 10000
    r 0x318400f0f9bd15f0d8842870b510e996dffc944b77111ded03a4255c66e82d42
    s 0x7132e765d5e6bb21ba046dbb98e28bb28cb2bebe0c8aced2c547aca60a554892
    v 0x1c
6. call asset claim in ocw module
   transaction -> OffchainWorker -> AssetClaim
7. after 5 blocks check the balance 
   state -> OffchainWorker -> accountBalance 
   its eth balance should be 0.5 eth.
8. check the balance of ocw account
   its balance should be incresed 1 dot.