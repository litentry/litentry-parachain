#!/bin/bash

# Copy integritee-service binary and enclave_signed.so to ./tmp/w0
cp ./bin/integritee-service ./tmp/w0
cp ./bin/enclave.signed.so ./tmp/w0

# Navigate to ./tmp/w0
cd ./tmp/w0 || exit

echo "Old MRENCLAVE VALUE: $OLD_MRENCLAVE"
echo "New MRENCLAVE VALUE: $NEW_MRENCLAVE"
# Run the migration command
./integritee-service migrate-shard --old-shard $OLD_MRENCLAVE --new-shard $NEW_MRENCLAVE

# Navigate to ./tmp/w0/shards
cd shards || exit

# Find the two files and delete the older one
files=(*)
if [[ ${#files[@]} -eq 2 ]]; then
    file1="${files[0]}"
    file2="${files[1]}"
    if [[ $file1 -ot $file2 ]]; then
        echo "Deleting the older file: $file1"
        rm "$file1"
    else
        echo "Deleting the older file: $file2"
        rm "$file2"
    fi
fi

echo "Migration of shards completed"
