#!/bin/bash

# Fetch Current MRENCLAVE Value
output=$(make mrenclave 2>&1)
if [[ $? -eq 0 ]]; then
    mrenclave_value=$(echo "$output" | awk '{print $2}')
    echo "MRENCLAVE value: $mrenclave_value"
    export OLD_MRENCLAVE="$mrenclave_value"
else
    echo "Failed to extract MRENCLAVE value."
fi

log=$(cd bin && ./integritee-service signing-key 2>&1)
enclave_account=$(echo "$log" | awk '/Enclave account:/{print $NF}')
if [[ -n $enclave_account ]]; then
    echo "Enclave account value: $enclave_account"
    export ENCLAVE_ACCOUNT="$enclave_account"
    echo "ENCLAVE_ACCOUNT exported successfully."
else
    echo "Failed to extract Enclave account value."
fi



# TODO: This will be different depending on who is calling it
export SGX_COMMERCIAL_KEY="/home/faisal/litentry-parachain/tee-worker/enclave-runtime/Enclave_private.pem"
export SGX_PRODUCTION="1"

make

# Fetch new MRENCLAVE Value
output=$(make mrenclave 2>&1)
if [[ $? -eq 0 ]]; then
    mrenclave_value=$(echo "$output" | awk '{print $2}')
    echo "MRENCLAVE value: $mrenclave_value"
    export NEW_MRENCLAVE="$mrenclave_value"
else
    echo "Failed to extract MRENCLAVE value."
fi

# Fetch Latest Block Produced
line=$(grep '\[.*\]$' log/worker0.log | tail -n 1 2>&1)
number=$(echo "$line" | sed -E 's/.*\[([0-9]+)\]$/\1/')
current_sidechain_end_block=$((number + 50))
echo "The next enclave is scheduled to start producing blocks after: $current_sidechain_end_block blocks"

export SCHEDULE_UPDATE_BLOCK="$current_sidechain_end_block"

echo "Setting up the new Worker on Chain"
../scripts/ts-utils/setup_enclave.sh

echo "Waiting for the old worker to stop producing blocks"
scripts/litentry/stop_old_worker.sh

echo "Performing migration for the worker"
scripts/litentry/migrate_worker.sh

echo "Starting new worker"
cd tmp/w0
./integritee-service -P 2000 -w 2001 -r 3443 -h 4545 --running-mode mock --enable-mock-server --parentchain-start-block 0 run --skip-ra --dev




