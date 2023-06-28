#!/bin/bash

setup_working_dir() {
    source_dir=$1
    target_dir=$2

    optional=("key.txt" "spid.txt")

    for file in "${optional[@]}"; do
        source="${source_dir}/${file}"
        target="${target_dir}/${file}"

        if [ -f "$source" ]; then
            cp "$source" "$target"
        else
            echo "$source does not exist, this is fine, but you can't perform remote attestation with this."
        fi
    done

    mandatory=("enclave.signed.so" "integritee-service")

    for file in "${mandatory[@]}"; do
        source="${source_dir}/${file}"
        target="${target_dir}/${file}"

        if [ -f "$source" ]; then
            cp "$source" "$target"
        else
            echo "$source does not exist. Did you run make?"
        fi
    done
}

# Example usage:
# setup_working_dir "/path/to/source/dir" "/path/to/target/dir"


echo "Launching Services"

echo "Checking  for available ports"
for p in ${CollatorWSPort:-9944} ${CollatorPort:-30333} ${UntrustedHttpPort:-4545}; do
  if [ ! -z "$(netstat -nat | grep $p)" ]; then
    echo "port $p is in use, quit now"
    exit 1
  fi
done
echo "Default ports available in the system"

echo "Starting Parachain Services"
../scripts/launch-local-binary.sh rococo
echo "Parachain Services have been setup, check tmp/parachain_dev for Logs"

echo "Cleaning up previous db files"
filename="tmp/w0"

# TODO: This if/block has undefined behaviour
if [ -f "$filename" ]; then
    echo "$filename exists."
    echo "Purging it now! since it is a clean reset"
else
    echo "$filename does not exist. Creating it now"
    mkdir tmp/w0
fi

echo "Moving files to tmp/w0"
setup_working_dir "bin" "tmp/w0"

echo "Starting Worker"
cd tmp/w0

export RUST_LOG='info,integritee_service=debug,ws=warn,sp_io=error,substrate_api_client=warn,
itc_parentchain_light_client=info,
jsonrpsee_ws_client=warn,jsonrpsee_ws_server=warn,enclave_runtime=debug,ita_stf=debug,
its_rpc_handler=warn,itc_rpc_client=warn,its_consensus_common=debug,its_state=warn,
its_consensus_aura=warn,aura*=warn,its_consensus_slots=warn,
itp_attestation_handler=debug,http_req=debug,lc_mock_server=warn,itc_rest_client=debug,
lc_credentials=debug,lc_identity_verification=debug,lc_stf_task_receiver=debug,lc_stf_task_sender=debug,
lc_data_providers=debug,itp_top_pool=debug,itc_parentchain_indirect_calls_executor=debug'

# Redirect stdout to a log file
log_file="../../log/worker0.log"

# Execute the command
exec ./integritee-service --clean-reset -P 2000 -w 2001 -r 3443 -h 4545 --running-mode mock --enable-mock-server --parentchain-start-block 0 run --skip-ra --dev >"$log_file" 2>&1
echo "Worker has started"

# TODO: Check for block production via logs
# TODO: Print divider :3

