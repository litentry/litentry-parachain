# Integritee CLI client
Interact with the Integritee chain and workers from the command line

Includes
* keystore (incompatible with polkadot js app json)
* basic balance transfer
* Integritee-specific calls

## examples
```
> ./litentry-cli transfer //Bob //Alice 12345
> ./litentry-cli -u ws://127.0.0.1 list-workers
number of workers registered: 1
Enclave 1
   AccountId: 5HN8RGEiJuc9iNA3vfiYj7Lk6ULWzBZXvSDheohBu3usSUqn
   MRENCLAVE: 4GMb72Acyg8hnnnGEJ89jZK5zxNC4LvSe2ME96wLRV6J
   RA timestamp: 2022-03-16 10:43:12.001 UTC
   URL: wss://127.0.0.1:2345
```

## housekeeping tasks

populate all TCBinfo's Intel has published
```
../target/release/litentry-cli register-tcb-info //Alice --fmspc 00606a000000
../target/release/litentry-cli register-tcb-info //Alice --all
```
