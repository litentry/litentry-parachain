#!/bin/bash

# Default values
CHAIN_NAME=""
CONTRACT_NAME=""
MNEMONIC_VALUE=""
SECRET_VALUES=()

# Parse command-line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --chain)
            CHAIN_NAME="$2"; shift 2 ;;
        --contract)
            CONTRACT_NAME="$2"; shift 2 ;;
        --mnemonic)
            MNEMONIC_VALUE="$2"; shift 2 ;;
        --secrets)
            shift
            while [[ "$1" && "$1" != --* ]]; do
                SECRET_VALUES+=("$1")
                shift
            done
            ;;
        *)
            echo "Unknown parameter passed: $1"
            exit 1
            ;;
    esac
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

if [ -n "$MNEMONIC_VALUE" ]; then
    export MNEMONIC=$MNEMONIC_VALUE
else
    unset MNEMONIC
fi

if [ ${#SECRET_VALUES[@]} -gt 0 ]; then
	# Join array elements with line break for the environment variable
    export SECRETS=$(printf "%s\n" ${SECRET_VALUES[*]})
else
    unset SECRETS
fi


# Run Hardhat script
npx hardhat run scripts/deploy.ts
