#!/bin/bash

export LOG="log/worker0.log"
export TARGET_DIRECTORY="tmp/w0"
export PARACHAIN_SOURCE=$(cd .. && pwd)
export WORKER_DIR=$(pwd)

echo "Source Directory for Parachain: ${PARACHAIN_SOURCE}"
echo "Worker Directory: ${WORKER_DIR}"

function print_divider() {
  echo "------------------------------------------------------------"
}

# TODO: Set Pipe fail if any of the commands fail
generate_service_file() {
  if [ "$#" -ne 4 ]; then
    echo "Usage: generate_service_file <service_name> <description> <command> <working_directory>"
    return 1
  fi

  local service_name="$1"
  local description="$2"
  local command="$3"
  local working_directory="$4"

  local service_template="[Unit]
Description=${description}

[Service]
ExecStart=${command}
WorkingDirectory=${working_directory}
Restart=always

[Install]
WantedBy=multi-user.target
"

  local service_filename="${service_name}.service"
  echo "$service_template" > "$service_filename"
  echo "Service file \"${service_filename}\" generated successfully."
}

generate_service_file_for_worker() {
 local service_name="$1"
 local description="$2"
 local command="$3"
 local working_directory="$4"

 local service_template="[Unit]
 Description=${description}
 After=network.target

 [Service]
 ExecStart=${command}
 Restart=always
 Environment='RUST_LOG=info,integritee_service=debug,ws=warn,sp_io=error,substrate_api_client=warn,itc_parentchain_light_client=info,jsonrpsee_ws_client=warn,jsonrpsee_ws_server=warn,enclave_runtime=debug,ita_stf=debug,its_rpc_handler=warn,itc_rpc_client=warn,its_consensus_common=debug,its_state=warn,its_consensus_aura=warn,aura*=warn,its_consensus_slots=warn,itp_attestation_handler=debug,http_req=debug,lc_mock_server=warn,itc_rest_client=debug,lc_credentials=debug,lc_identity_verification=debug,lc_stf_task_receiver=debug,lc_stf_task_sender=debug,lc_data_providers=debug,itp_top_pool=debug,itc_parentchain_indirect_calls_executor=debug'
 StandardOutput=file:${WORKER_DIR}/${LOG}
 StandardError=inherit

 [Install]
 WantedBy=default.target
 "
   local service_filename="${service_name}.service"
   echo "$service_template" > "$service_filename"
   echo "Service file \"${service_filename}\" generated successfully."
}

generate_worker_service_file() {
  local command="/bin/bash -c  'cd ${WORKER_DIR}/tmp/w0 && ./integritee-service --clean-reset -P 2000 -w 2001 -r 3443 -h 4545 --running-mode mock --enable-mock-server --parentchain-start-block 0 run --skip-ra --dev'"
  local service_name='worker'
  local description='Worker Service for Litentry Side chain'
  local working_directory='/usr/local/bin'

  generate_service_file_for_worker "${service_name}" "${description}" "${command}" "${working_directory}"
}

generate_upgrade_worker_service_file() {
  echo "Latest Finalized block: ${LATEST_FINALIZED_BLOCK}"
  local command="/bin/bash -c 'cd ${WORKER_DIR}/tmp/w0 && ./integritee-service -P 2000 -w 2001 -r 3443 -h 4545 --running-mode mock --enable-mock-server --parentchain-start-block ${LATEST_FINALIZED_BLOCK} run --skip-ra --dev'"
  local service_name='worker'
  local description='Worker Service for Litentry Side chain'
  local working_directory='/opt/worker/'

  generate_service_file_for_worker "${service_name}" "${description}" "${command}" "${working_directory}"
}

generate_parachain_service_file() {
  local command="/bin/bash -c 'source /home/faisal/.nvm/nvm.sh && cd ${PARACHAIN_SOURCE} && scripts/launch-local-binary.sh rococo && sleep infinity'"
  local service_name='litentry-parachain'
  local description='Parachain Setup for Litentry'
  local working_directory=$PARACHAIN_SOURCE

  generate_service_file "${service_name}" "${description}" "${command}" "${working_directory}"

}

current_mrenclave(){
  # TODO: Correct Working Directory
  output=$(make mrenclave 2>&1)
  if [[ $? -eq 0 ]]; then
      mrenclave_value=$(echo "$output" | awk '{print $2}')
      echo "MRENCLAVE value: $mrenclave_value"
      export OLD_MRENCLAVE="$mrenclave_value"
  else
      echo "Failed to extract MRENCLAVE value."
  fi
}

enclave_account(){
  log=$(cd bin && ./integritee-service signing-key 2>&1)
  enclave_account=$(echo "$log" | awk '/Enclave account:/{print $NF}')
  if [[ -n $enclave_account ]]; then
      echo "Enclave account value: $enclave_account"
      export ENCLAVE_ACCOUNT="$enclave_account"
      echo "ENCLAVE_ACCOUNT exported successfully."
  else
      echo "Failed to extract Enclave account value."
  fi
}

new_mrenclave(){
  output=$(make mrenclave 2>&1)
  if [[ $? -eq 0 ]]; then
      mrenclave_value=$(echo "$output" | awk '{print $2}')
      echo "MRENCLAVE value: $mrenclave_value"
      export NEW_MRENCLAVE="$mrenclave_value"
  else
      echo "Failed to extract MRENCLAVE value."
  fi
}

latest_sync_block(){
  # Fetch Latest Block Produced
  if [ "$PRODUCTION" = "1" ]; then
    line=$(grep '\[.*\]$' log/worker0.log | tail -n 1 2>&1)
    number=$(echo "$line" | sed -E 's/.*\[([0-9]+)\]$/\1/')
    current_sidechain_end_block=$((number + 50))
    echo "The next enclave is scheduled to start producing blocks after: $current_sidechain_end_block blocks"
    export SCHEDULE_UPDATE_BLOCK="$current_sidechain_end_block"
  else
    line=$(grep '\[.*\]$' log/worker0.log | tail -n 1 2>&1)
    number=$(echo "$line" | sed -E 's/.*\[([0-9]+)\]$/\1/')
    current_sidechain_end_block=$((number + 50))
    echo "The next enclave is scheduled to start producing blocks after: $current_sidechain_end_block blocks"
    export SCHEDULE_UPDATE_BLOCK="$current_sidechain_end_block"
  fi
}

latest_parentchain_sync_block(){

  # Parachain endpoint details
  parachain_host="localhost"
  parachain_port="9944"

  # JSON-RPC request payload
  request='{"jsonrpc":"2.0","id":1,"method":"chain_getHeader","params":[]}'

  # Make the JSON-RPC request and retrieve the latest finalized block
  response=$(curl -s -H "Content-Type: application/json" -d "$request" http://$parachain_host:$parachain_port)
  hex_number=$(echo "$response" | grep -oP '(?<="number":")[^"]+')
  dec_number=$(printf "%d" "$hex_number")


  # Store the latest finalized block numbe  r in an environment variable
  export LATEST_FINALIZED_BLOCK=${dec_number}

  echo "Latest finalized block number: $LATEST_FINALIZED_BLOCK"
}

start_service_files(){
  # echo "Moving worker binaries to /usr/local/bin"
  # setup_working_dir "bin" "/usr/local/bin"
  print_divider

  echo "Moving service files to ~/.config/systemd/user"
  cp -r *.service ~/.config/systemd/user

  echo "Performing Daemon Reload"
  systemctl --user daemon-reload

  echo "Starting Parachain Service"
  systemctl --user start litentry-parachain.service
  print_divider

  echo "Sleep for 120s, This gives enough time for the Parachain to be started.."
  sleep 120

  print_divider
  # TODO: Check for block finalization instead of 60s
  echo "Starting Working Service"
  systemctl --user start worker.service
  # TODO: Check for block production via Logs

  echo "Parachain and Worker Service have started succesfully, You can check logs at /log/worker0.log"
  print_divider

}

perform_upgrade(){
  # echo "Fetching Latest Finalized Block"
  # scripts/litentry/get_sync_block.sh
  echo "Setting up the new Worker on Chain"
  ../scripts/ts-utils/setup_enclave.sh

  print_divider
  echo "Waiting for the old worker to stop producing blocks"
  scripts/litentry/stop_old_worker.sh

  print_divider
  echo "Performing migration for the worker"
  scripts/litentry/migrate_worker.sh
}

upgrade_worker(){
  print_divider
  current_mrenclave
  enclave_account

  export SGX_COMMERCIAL_KEY="/home/faisal/litentry-parachain/tee-worker/enclave-runtime/Enclave_private.pem"
  export SGX_PRODUCTION="1"
  make

  new_mrenclave
  latest_sync_block
  latest_parentchain_sync_block
  print_divider
  perform_upgrade
  print_divider

  if [ "$PRODUCTION" = "1" ]; then
      generate_upgrade_worker_service_file
      cp worker.service ~/.config/systemd/user/
      echo "Removing old log life"
      rm -f $LOG
      systemctl --user daemon-reload
      systemctl --user start worker.service
      print_divider
      echo "Check logs for the upgraded worker here at /log/worker0/log"
  else
      export RUST_LOG='info,integritee_service=debug,ws=warn,sp_io=error,substrate_api_client=warn,
      itc_parentchain_light_client=info,
      jsonrpsee_ws_client=warn,jsonrpsee_ws_server=warn,enclave_runtime=debug,ita_stf=debug,
      its_rpc_handler=warn,itc_rpc_client=warn,its_consensus_common=debug,its_state=warn,
      its_consensus_aura=warn,aura*=warn,its_consensus_slots=warn,
      itp_attestation_handler=debug,http_req=debug,lc_mock_server=warn,itc_rest_client=debug,
      lc_credentials=debug,lc_identity_verification=debug,lc_stf_task_receiver=debug,lc_stf_task_sender=debug,
      lc_data_providers=debug,itp_top_pool=debug,itc_parentchain_indirect_calls_executor=debug'

      echo "Starting new worker"
      cd tmp/w0

      # Redirect stdout to a log file
      log_file="../../log/worker0.log"

      exec ./integritee-service -P 2000 -w 2001 -r 3443 -h 4545 --running-mode mock --enable-mock-server --parentchain-start-block 0 run --skip-ra --dev >"$log_file" 2>&1
  fi
}

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

if [ "$UPGRADE_WORKER" = "1" ]; then
  # Assuming the worker has already stopped
  echo "Preparing to Upgrade Worker"
  upgrade_worker
  exit 0
fi

# Example usage
# generate_service_file "my_service" "My Service Description" "/path/to/my_script.sh" "/path/to/working_directory"
if [ "$PRODUCTION" = "1" ]; then
    echo "Running in production mode."
    setup_working_dir "bin" "tmp/w0"
    generate_worker_service_file
    generate_parachain_service_file
else
    echo "Not running in production mode."
fi

if [ "$AUTO_START" = "true" ]; then
  # Start the services
  if [ "$PRODUCTION" = "1" ]; then
    echo "Starting Services"
    start_service_files
  else
    ./local-setup/launch.sh
  fi
fi

# TODO: Need to also perform clean reset of the service
if [ "$CLEAN_RESET" = "true" ]; then
  # Assuming the worker has already stopped
  echo "Preparing to Upgrade Worker"
fi