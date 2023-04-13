#!/bin/bash
set -euo pipefail

# Edit the following variable(s):
WORKER_DIR=/tmp/worker/

RUNNING_MODE_CONFIG=$1/config.json
PRIVATE_ACCOUNT_JSON=$1/private_account.json
INTEL_KEY=$1/key_production.txt
INTEL_SPID=$1/spid_production.txt

##############################################################################
# Don't edit anything from here
if [ "$#" -ne 1 ]; then
  echo "Error: Please provide one and only one secret path as input."
  exit 1
fi
if [[ ! -e "$RUNNING_MODE_CONFIG" ]]; then
  echo "Error: $RUNNING_MODE_CONFIG is not a valid path."
  exit 1
fi
if [[ ! -e "$PRIVATE_ACCOUNT_JSON" ]]; then
  echo "Error: $PRIVATE_ACCOUNT_JSON is not a valid path."
  exit 1
fi
if [[ ! -e "$INTEL_KEY" ]]; then
  echo "Error: $INTEL_KEY_PATH is not a valid path."
  exit 1
fi
if [[ ! -e "$INTEL_SPID" ]]; then
  echo "Error: $INTEL_SPID_PATH is not a valid path."
  exit 1
fi

# Generate keys and copy around.
SRC_DIR=$(dirname "$0")
cd $SRC_DIR

./integritee-service signing-key | grep -oP '^Enclave account: \K.*$$' > enclave_account.txt
echo "Enclave account is prepared inside enclave_account.txt"

./integritee-service shielding-key


for Item in 'enclave.signed.so' 'integritee-service' 'aes_key_sealed.bin' 'ed25519_key_sealed.bin' 'enclave-shielding-pubkey.json' 'enclave-signing-pubkey.bin' 'rsa3072_key_sealed.bin' 'sidechain_db'; do
    cp -r "${Item}" "${WORKER_DIR}"
done

cp $RUNNING_MODE_CONFIG_PATH "${WORKER_DIR}/config.json"
cp $INTEL_KEY_PATH "${WORKER_DIR}/key_production.txt"
cp $INTEL_SPID_PATH "${WORKER_DIR}/spid_production.txt"

# Comment out for the moment. Need to adapt together with PR-1587 ts-utils.
# cp $PRIVATE_ACCOUNT_JSON "${WORKER_DIR}/ts-setup/private_account.json"
# cp "enclave_account.txt" "${WORKER_DIR}/ts-setup/enclave_account.txt"
# cp "mrenclave.txt" "${WORKER_DIR}/ts-setup/mrenclave.txt"

