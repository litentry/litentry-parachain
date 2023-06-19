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


