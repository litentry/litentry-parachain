#!/bin/bash

# Parachain endpoint details
parachain_host="localhost"
parachain_port="9944"

# JSON-RPC request payload
request='{"jsonrpc":"2.0","id":1,"method":"chain_getHeader","params":[]}'

# Make the JSON-RPC request and retrieve the latest finalized block
response=$(curl -s -H "Content-Type: application/json" -d "$request" http://$parachain_host:$parachain_port)
hex_number=$(echo "$response" | grep -oP '(?<="number":")[^"]+')
dec_number=$(printf "%d" "$hex_number")


# Store the latest finalized block number in an environment variable
export LATEST_FINALIZED_BLOCK=${dec_number}

echo "Latest finalized block number: $LATEST_FINALIZED_BLOCK"

