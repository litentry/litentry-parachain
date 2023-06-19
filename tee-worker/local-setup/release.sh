#!/bin/bash

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

generate_worker_service_file() {
  local command='./integritee-service --clean-reset -P 2000 -w 2001 -r 3443 -h 4545 --running-mode mock --enable-mock-server --parentchain-start-block 0 run --skip-ra --dev'
  local service_name='worker'
  local description='Worker Service for Litentry Side chain'
  local working_directory='/opt/worker/'

  generate_service_file "${service_name}" "${description}" "${command}" "${working_directory}"
}

generate_upgrade_worker_service_file() {
  local command='./integritee-service -P 2000 -w 2001 -r 3443 -h 4545 --running-mode mock --enable-mock-server --parentchain-start-block 0 run --skip-ra --dev'
  local service_name='worker'
  local description='Worker Service for Litentry Side chain'
  local working_directory='/opt/worker/'

  generate_service_file "${service_name}" "${description}" "${command}" "${working_directory}"
}

generate_parachain_service_file() {
  local command='./scripts/launch-local-binary.sh rococo'
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
  # TODO: Correct Working Directory
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
  # TODO: Correct Working Directory
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
  # TODO: Correct Log file
  # Fetch Latest Block Produced
  line=$(grep '\[.*\]$' log/worker0.log | tail -n 1 2>&1)
  number=$(echo "$line" | sed -E 's/.*\[([0-9]+)\]$/\1/')
  current_sidechain_end_block=$((number + 50))
  echo "The next enclave is scheduled to start producing blocks after: $current_sidechain_end_block blocks"

  export SCHEDULE_UPDATE_BLOCK="$current_sidechain_end_block"
}

start_service_files(){
  echo "Moving service files to /etc/systemd/system"
  cp -r *.service /etc/systemd/system/

  echo "Performing Daemon Reload"
  systemctl daemon-reload

  echo "Starting Parachain Service"
  systemctl start litentry-parachain.service

  echo "Sleep for 60s, Parachain can be started"
  sleep 60
  # TODO: Check for block finalization instead of 60s
  echo "Starting Working Service"
  systemctl start worker.service
  # TODO: Check for block production via Logs

  echo "Parachain and Worker Service have started succesfully, You can check logs at /data/logs"

}

perform_upgrade(){
  echo "Setting up the new Worker on Chain"
  ../scripts/ts-utils/setup_enclave.sh

  echo "Waiting for the old worker to stop producing blocks"
  scripts/litentry/stop_old_worker.sh

  echo "Performing migration for the worker"
  scripts/litentry/migrate_worker.sh
}

# Example usage
# generate_service_file "my_service" "My Service Description" "/path/to/my_script.sh" "/path/to/working_directory"
generate_worker_service_file
generate_parachain_service_file

if [ "$AUTO_START" = "true" ]; then
  # Start the services
  echo "Starting Services"
  start_service_files
fi

if [ "$UPGRADE_WORKER" = "true" ]; then
  # Assuming the worker has already stopped
  echo "Preparing to Upgrade Worker"
fi