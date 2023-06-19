#!/bin/bash

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

generate_parachain_service_file() {
  local command='./scripts/launch-local-binary.sh rococo'
  local service_name='litentry-parachain'
  local description='Parachain Setup for Litentry'
  local working_directory=$PARACHAIN_SOURCE

  generate_service_file "${service_name}" "${description}" "${command}" "${working_directory}"

}

# Example usage
# generate_service_file "my_service" "My Service Description" "/path/to/my_script.sh" "/path/to/working_directory"
generate_worker_service_file
generate_parachain_service_file

if [ "$AUTO_START" = "true" ]; then
  # Start the services
  echo "Starting Services"
fi

if [ "$UPGRADE_WORKER" = "true" ]; then
  # Assuming the worker has already stopped
  echo "Preparing to Upgrade Worker"
fi