#!/bin/bash

# Default values
CHAIN_NAME=""
CONTRACT_NAME=""
SECRET_VALUES=()

# Parse command-line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --chain) CHAIN_NAME="$2"; shift ;;
        --contract) CONTRACT_NAME="$2"; shift ;;
        --secrets) shift; while [[ "$1" && "$1" != --* ]]; do SECRET_VALUES+=("$1"); shift; done ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

# Check if parameters are provided
if [ -z "$CONTRACT_NAME" ]; then
    echo "Error: --contract parameter is required"
    exit 1
fi

if [ -z "$CHAIN_NAME" ]; then
    echo "Error: --chain parameter is required"
    exit 1
fi

# Set environment variables
export CHAIN=$CHAIN_NAME
export CONTRACT=$CONTRACT_NAME
# Join array elements with spaces for the environment variable
export SECRETS=$(IFS=' '; echo "${SECRET_VALUES[*]}")

echo $(pwd)

# Run Hardhat script
npx hardhat run scripts/deploy.ts
