#!/bin/bash
set -euo pipefail

ROOT_DIR=$(git rev-parse --show-toplevel)
WORKER_DIR="${ROOT_DIR}/tee-worker"

# Edit the following variable(s):
COMMERCIAL_KEY_PATH=${WORKER_DIR}/enclave-runtime/Enclave_private.pem

##############################################################################
# Don't edit anything from here

CURRENT_DATE=$(date "+%Y%m%d")
GIT_HASH=$(git rev-parse --short HEAD)
RELEASE_DIR_NAME=release-${CURRENT_DATE}-${GIT_HASH}
RELEASE_DIR=${WORKER_DIR}/target/${RELEASE_DIR_NAME}

mkdir -p ${RELEASE_DIR}

cd "$WORKER_DIR"

# Build target files in production mode
SGX_PRODUCTION=1 SGX_COMMERCIAL_KEY=${COMMERCIAL_KEY_PATH} make

# Copy files
for Item in 'enclave.signed.so' 'integritee-service'; do
    cp "${WORKER_DIR}/bin/${Item}" "${RELEASE_DIR}"
done
for Item in 'prepare.sh' 'config.json.eg' 'ReadMe.md'; do
    cp -r "${WORKER_DIR}/scripts/litentry/release/${Item}" "${RELEASE_DIR}"
done

cp -r "${ROOT_DIR}/ts-utils" "${RELEASE_DIR}"

make mrenclave | grep -oP '^MRENCLAVE: \K.*$$' > "${RELEASE_DIR}/mrenclave.txt"

cd ${WORKER_DIR}/target/
tar -czf ${RELEASE_DIR_NAME}.tar.gz ${RELEASE_DIR_NAME}

echo "Release package generate: ${RELEASE_DIR}.tar.gz"

