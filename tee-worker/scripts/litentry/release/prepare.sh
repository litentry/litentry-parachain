#!/bin/bash
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "Error: Please provide one and only one secret path as input. Please check ReadMe.md for details."
  exit 1
fi

# Edit the following variable(s):
# This WORKER_DIR is the directory where worker will start from.
WORKER_DIR=/opt/worker/

RUNNING_MODE_CONFIG=$1/config.json
PRIVATE_ACCOUNT_JSON=$1/private_account.json
INTEL_KEY=$1/key_production.txt
INTEL_SPID=$1/spid_production.txt

##############################################################################
# Don't edit anything from here
if [[ ! -e "$WORKER_DIR" ]]; then
  mkdir -p $WORKER_DIR
fi

for Item in $RUNNING_MODE_CONFIG $PRIVATE_ACCOUNT_JSON $INTEL_KEY $INTEL_SPID; do
  if [[ ! -e "$Item" ]]; then
    echo "Error: $Item is not a valid path."
    exit 1
  fi
done

# Generate keys and copy around.
SRC_DIR=$(dirname "$0")
cd $SRC_DIR

./integritee-service signing-key | grep -oP '^Enclave account: \K.*$$' > enclave_account.txt
echo "Enclave account is prepared inside enclave_account.txt"

./integritee-service shielding-key

for Item in 'enclave.signed.so' 'integritee-service' 'aes_key_sealed.bin' 'ed25519_key_sealed.bin' 'enclave-shielding-pubkey.json' 'enclave-signing-pubkey.bin' 'rsa3072_key_sealed.bin' 'sidechain_db'; do
  cp -r "${Item}" "${WORKER_DIR}"
done

cp $RUNNING_MODE_CONFIG "${WORKER_DIR}/config.json"
cp $INTEL_KEY "${WORKER_DIR}/key_production.txt"
cp $INTEL_SPID "${WORKER_DIR}/spid_production.txt"

# Comment out for the moment. Need to adapt together with PR-1587 ts-utils.
# cp $PRIVATE_ACCOUNT_JSON "${WORKER_DIR}/ts-setup/private_account.json"
# cp "enclave_account.txt" "${WORKER_DIR}/ts-setup/enclave_account.txt"
# cp "mrenclave.txt" "${WORKER_DIR}/ts-setup/mrenclave.txt"

