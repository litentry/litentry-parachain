#!/bin/bash

set -eo pipefail

# most is copied from
# https://github.com/apache/incubator-teaclave-sgx-sdk/blob/v1.1.4/dockerfile/Dockerfile.2004.nightly

# install rust
curl -s https://sh.rustup.rs -sSf | sh -s -- -y
# shellcheck source=${HOME}/.cargo/env
source ${HOME}/.cargo/env
rustup show

# install substrate build deps
sudo apt-get update
sudo apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev gnupg2

# install llvm
sudo apt-get update
wget https://apt.llvm.org/llvm.sh && chmod +x llvm.sh && sudo ./llvm.sh 10

# override binutils
wget https://download.01.org/intel-sgx/sgx-linux/2.15.1/as.ld.objdump.r4.tar.gz
tar xzf as.ld.objdump.r4.tar.gz
sudo cp -f external/toolset/ubuntu20.04/* /usr/bin/

# install sgx_sdk
SDK_URL="https://download.01.org/intel-sgx/sgx-linux/2.15.1/distro/ubuntu20.04-server/sgx_linux_x64_sdk_2.15.101.1.bin"
curl -o sdk.sh $SDK_URL
chmod a+x sdk.sh
echo -e 'no\n/opt' | ./sdk.sh
source /opt/sgxsdk/environment

# install runtime sgx libs (psw)
CODENAME=focal
VERSION=2.15.101.1-focal1
DCAP_VERSION=1.12.101.1-focal1

curl -fsSL https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key | sudo apt-key add - && \
sudo add-apt-repository "deb https://download.01.org/intel-sgx/sgx_repo/ubuntu $CODENAME main" && \
sudo apt-get update && \
sudo apt-get install -y \
    libsgx-headers=$VERSION \
    libsgx-ae-epid=$VERSION \
    libsgx-ae-le=$VERSION \
    libsgx-ae-pce=$VERSION \
    libsgx-aesm-ecdsa-plugin=$VERSION \
    libsgx-aesm-epid-plugin=$VERSION \
    libsgx-aesm-launch-plugin=$VERSION \
    libsgx-aesm-pce-plugin=$VERSION \
    libsgx-aesm-quote-ex-plugin=$VERSION \
    libsgx-enclave-common=$VERSION \
    libsgx-enclave-common-dev=$VERSION \
    libsgx-epid=$VERSION \
    libsgx-epid-dev=$VERSION \
    libsgx-launch=$VERSION \
    libsgx-launch-dev=$VERSION \
    libsgx-quote-ex=$VERSION \
    libsgx-quote-ex-dev=$VERSION \
    libsgx-uae-service=$VERSION \
    libsgx-urts=$VERSION \
    sgx-aesm-service=$VERSION \
    libsgx-ae-qe3=$DCAP_VERSION \
    libsgx-pce-logic=$DCAP_VERSION \
    libsgx-qe3-logic=$DCAP_VERSION \
    libsgx-ra-network=$DCAP_VERSION \
    libsgx-ra-uefi=$DCAP_VERSION
mkdir -p /var/run/aesmd || true

# store env
echo "$(env)" >> $GITHUB_ENV