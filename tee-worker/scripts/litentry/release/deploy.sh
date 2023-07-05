#!/bin/bash

export ROOTDIR=$(git rev-parse --show-toplevel)


function print_divider() {
  echo "------------------------------------------------------------"
}

# Function to display the script's usage
function display_help() {
  echo "Usage: ./deploy.sh restart --build --config config.json"
  echo ""
  echo "Options:"
  echo "  -h, --help         Display this help message and exit."
  echo "  -b, --build        Build the binary for Parachain and Worker."
  echo "  -d, --discard      Clean the existing state for Parachain and Worker."
  echo "  -c, --config [Config.json] Config file for the worker."
  echo "  -a, --only-worker   Start only the worker"
  echo "  -x, --chain        Chain to use for Parachain Deployment"
  echo "  -p, --parachain-port  Parachain Port Number (default: 9944)"
  echo "  -h, --parachain-host  Parachain Host Url (default: localhost)"
  echo "  -v, --copy-from-docker Copy the binary for Parachain from a docker image (default: litentry/litentry-parachain:tee-prod)"
  echo ""
  echo "Arguments:"
  echo "  restart            Restart the services."
  echo "  upgrade-worker     Upgrade the worker."
  echo ""
  echo "Examples:"
  echo "  ./deploy.sh restart --v --build --config github-staging-one-worker.json"
  echo "  ./deploy.sh restart --v --build --config github-staging-one-worker.json --discard"
  echo "  ./deploy.sh upgrade-worker --v --build --config github-staging-one-worker.json"
  echo ""
  echo "Additional Information:"
  echo "  - This script requires an OS that supports systemd."
  echo "  - It is mandatory to provide a JSON config file for the worker."
  echo "  - jq is required to be installed on the system "
  echo ""
  echo "For more information or assistance, please contact Faisal."
}

generate_service_file() {
  if [ "$#" -ne 5 ]; then
    echo "Usage: generate_service_file <service_name> <description> <command> <working_directory> <log_file_path>"
    exit 1
  fi

  local service_name="$1"
  local description="$2"
  local command="$3"
  local working_directory="$4"
  local log_file="$5"

  local service_template="[Unit]
Description=${description}

[Service]
ExecStart=${command}
WorkingDirectory=${working_directory}
Restart=always
StandardOutput=file:${log_file}
StandardError=inherit

[Install]
WantedBy=multi-user.target
"

  # If worker service, We use a different template
  if [[ $service_name == worker* ]]; then
    service_template="[Unit]
Description=${description}
After=network.target

[Service]
ExecStart=${command}
Restart=always
Environment='RUST_LOG=info,integritee_service=debug,ws=warn,sp_io=error,substrate_api_client=warn,itc_parentchain_light_client=info,jsonrpsee_ws_client=warn,jsonrpsee_ws_server=warn,enclave_runtime=debug,ita_stf=debug,its_rpc_handler=warn,itc_rpc_client=warn,its_consensus_common=debug,its_state=warn,its_consensus_aura=warn,aura*=warn,its_consensus_slots=warn,itp_attestation_handler=debug,http_req=debug,lc_mock_server=warn,itc_rest_client=debug,lc_credentials=debug,lc_identity_verification=debug,lc_stf_task_receiver=debug,lc_stf_task_sender=debug,lc_data_providers=debug,itp_top_pool=debug,itc_parentchain_indirect_calls_executor=debug'
WorkingDirectory=${working_directory}
StandardOutput=file:${log_file}
StandardError=inherit

[Install]
WantedBy=default.target
"
  fi

  local service_filename="${service_name}.service"
  echo "$service_template" > "$service_filename"
  echo "Service file \"${service_filename}\" generated successfully."
}

# Function responsible for restarting the services
function restart(){
  if [ "$ONLY_WORKER" = true ]; then
    stop_running_services
    echo "Starting only worker"
    print_divider
    restart_worker
    print_divider
  else
    stop_running_services
    print_divider
    echo "Launching the system"
    restart_parachain
    print_divider
    register_parachain
    restart_worker
    print_divider
    echo "Parachain and Worker restarted Succesfully "
  fi
}

function stop_running_services() {
  cd ~/.config/systemd/user
  if [ "$ONLY_WORKER" = true ]; then
    worker_count=$(echo "$CONFIG" | jq '.workers | length')

    for ((i = 0; i < worker_count; i++)); do
      systemctl --user stop "worker${i}".service
    done
  else
    echo "Stopping Running Services if any"
    systemctl --user stop para-alice.service
    systemctl --user stop relay-alice.service
    systemctl --user stop relay-bob.service

    worker_count=$(echo "$CONFIG" | jq '.workers | length')

    for ((i = 0; i < worker_count; i++)); do
      systemctl --user stop "worker${i}".service
    done
  fi

}

# Note: Inspired from launch-local-binary.sh
function restart_parachain() {
  export TMPDIR=/tmp/parachain_dev
  [ -d "$TMPDIR" ] || mkdir -p "$TMPDIR"

  cd "$ROOTDIR"
  PARACHAIN_ID=$(grep DEFAULT_PARA_ID node/src/chain_specs/$CHAIN.rs  | grep u32 | sed 's/.* = //;s/\;//')
  export PARACHAIN_ID
  echo "Parachain ID: $PARACHAIN_ID"

  echo "Could not find Polkadot Binary, Downloading now"
  echo "no polkadot binary provided, download now ..."
  url="https://github.com/paritytech/polkadot/releases/download/v0.9.39/polkadot"
  POLKADOT_BIN="$TMPDIR/polkadot"
  wget -O "$POLKADOT_BIN" -q "$url"
  chmod a+x "$POLKADOT_BIN"

  if [ ! -s "$POLKADOT_BIN" ]; then
    echo "$POLKADOT_BIN is 0 bytes, download URL: $url"
    exit 1
  fi

  if ! "$POLKADOT_BIN" --version &> /dev/null; then
    echo "Cannot execute $POLKADOT_BIN, wrong executable?"
    usage
    exit 1
  fi

  echo "Fething Litentry Collator Binary"
  if [ "$COPY_FROM_DOCKER" = true ]; then
    PARACHAIN_BIN="$ROOTDIR/litentry-collator"
  else
    PARACHAIN_BIN="$ROOTDIR/target/release/litentry-collator"
  fi
  chmod a+x "$PARACHAIN_BIN"

  if ! "$PARACHAIN_BIN" --version &> /dev/null; then
    echo "Cannot execute $PARACHAIN_BIN, wrong executable?"
    usage
    exit 1
  fi

  cd "$TMPDIR"

  echo "starting dev network with binaries ..."
  ROCOCO_CHAINSPEC=rococo-local-chain-spec.json
  $POLKADOT_BIN build-spec --chain rococo-local --disable-default-bootnode --raw > $ROCOCO_CHAINSPEC

  $PARACHAIN_BIN export-genesis-state --chain $CHAIN-dev > genesis-state
  $PARACHAIN_BIN export-genesis-wasm --chain $CHAIN-dev > genesis-wasm

  # run alice and bob as relay nodes
  echo "Generate Service File"
  local service_name="relay-alice"
  local description="Alice Node for Relay Chain"
  local working_directory="$TMPDIR"
  local log_file=/tmp/parachain_dev/relay.alice.log
  local command="$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --alice --tmp --port ${AlicePort:-30336} --ws-port ${AliceWSPort:-9946} --rpc-port ${AliceRPCPort:-9936}"

  generate_service_file "${service_name}" "${description}" "${command}" "${working_directory}" "${log_file}"
  cp ./${service_name}.service ~/.config/systemd/user/

  systemctl --user daemon-reload
  systemctl --user start $service_name

  sleep 10

  RELAY_ALICE_IDENTITY=$(grep 'Local node identity' relay.alice.log | sed 's/^.*: //')

  local service_name="relay-bob"
  local description="Bob Node for Relay Chain"
  local working_directory="$TMPDIR"
  local log_file=/tmp/parachain_dev/relay.bob.log
  local command="$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --bob --tmp --port ${BobPort:-30337} --ws-port ${BobWSPort:-9947}  --rpc-port ${BobRPCPort:-9937} --bootnodes /ip4/127.0.0.1/tcp/${CollatorPort:-30333}/p2p/$RELAY_ALICE_IDENTITY"

  generate_service_file "${service_name}" "${description}" "${command}" "${working_directory}" "${log_file}"
  cp ./${service_name}.service ~/.config/systemd/user/

  systemctl --user daemon-reload
  systemctl --user start $service_name
  sleep 10

  local service_name="para-alice"
  local description="Parachain Collator for Litenry Parachain"
  local working_directory="$TMPDIR"
  local log_file=/tmp/parachain_dev/para.alice.log
  local command=
  # run a litentry-collator instance
  local command="${PARACHAIN_BIN} --alice --collator --force-authoring --tmp --chain $CHAIN-dev --unsafe-ws-external --unsafe-rpc-external --rpc-cors=all --port ${CollatorPort:-30333} --ws-port ${CollatorWSPort:-9944} --rpc-port ${CollatorRPCPort:-9933} --execution wasm --state-pruning archive --blocks-pruning archive -- --execution wasm --chain $ROCOCO_CHAINSPEC --port 30332 --ws-port 9943 --rpc-port 9932 --bootnodes /ip4/127.0.0.1/tcp/${AlicePort:-30336}/p2p/$RELAY_ALICE_IDENTITY"

  generate_service_file "${service_name}" "${description}" "${command}" "${working_directory}" "${log_file}"
  cp ./${service_name}.service ~/.config/systemd/user/

  systemctl --user daemon-reload
  systemctl --user start $service_name

  sleep 10

  echo "Finished restarting Parachain, Check logs at /tmp/parachain_dev/para.alice.log"
}

function register_parachain() {
  echo "register parathread now ..."
  cd "$ROOTDIR/ts-tests"
  if [[ -z "${NODE_ENV}" ]]; then
      echo "NODE_ENV=ci" > .env
  else
      echo "NODE_ENV=${NODE_ENV}" > .env
  fi
  yarn
  yarn register-parathread 2>&1 | tee "$TMPDIR/register-parathread.log"
  print_divider

  echo "upgrade parathread to parachain now ..."
  # Wait for 90s to allow onboarding finish, after that we do the upgrade
  sleep 90
  cd "$ROOTDIR/ts-tests"
  if [[ -z "${NODE_ENV}" ]]; then
      echo "NODE_ENV=ci" > .env
  else
      echo "NODE_ENV=${NODE_ENV}" > .env
  fi
  yarn
  yarn upgrade-parathread 2>&1 | tee "$TMPDIR/upgrade-parathread.log"
  print_divider

  echo "done. please check $TMPDIR for generated files if need"

  print_divider
}

setup_working_dir() {
    local CONFIG_DIR=~/configs

    source_dir=$1
    target_dir=$2

    cd $source_dir
    ./integritee-service signing-key | grep -oP '^Enclave account: \K.*$$' > enclave_account.txt
    echo "Enclave account is prepared inside enclave_account.txt"

    export ENCLAVE_ACCOUNT=$(cat enclave_account.txt)
    echo "Enclave Account: $ENCLAVE_ACCOUNT"

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

    for Item in 'enclave.signed.so' 'integritee-service' 'aes_key_sealed.bin' 'ed25519_key_sealed.bin' 'enclave-shielding-pubkey.json' 'enclave-signing-pubkey.bin' 'rsa3072_key_sealed.bin' 'sidechain_db'; do
      cp -r "${Item}" "${target_dir}"
    done

}

function restart_worker() {
  # Need to make sure we have the JSON
  cd $ROOTDIR
  commands=()
  worker_count=$(echo "$CONFIG" | jq '.workers | length')

  for ((i = 0; i < worker_count; i++)); do
    WORKER_DIR=$ROOTDIR/tee-worker/tmp/w$i
    # Remove previous logs if any
    rm -r $ROOTDIR/tee-worker/log/worker${i}.log
    # Prepare the Worker Directory before restarting
    mkdir -p $ROOTDIR/tee-worker/tmp/w${i}
    setup_working_dir $ROOTDIR/tee-worker/bin $ROOTDIR/tee-worker/tmp/w$i

    # Transfer balance to the enclave account that is generated
    echo "Transferring balance to the enclave account"
    cd $ROOTDIR/scripts/ts-utils/
    yarn install
    npx ts-node transfer.ts  $ENCLAVE_ACCOUNT

    cd $ROOTDIR/tee-worker

    source=$(echo "$CONFIG" | jq -r ".workers[$i].source")
    flags=$(echo "$CONFIG" | jq -r ".workers[$i].flags[]")
    subcommand_flags=$(echo "$CONFIG" | jq -r ".workers[$i].subcommand_flags[]")

    command="./integritee-service"

    for flag in $flags; do
      command+=" $flag"
    done

    command+=" run"

    for subcommand_flag in $subcommand_flags; do
      command+=" $subcommand_flag"
    done

    local command_exec="/bin/bash -c  'cd ${WORKER_DIR} && ${command}'"
    local service_name="worker${i}"
    local description='Worker Service for Litentry Side chain'
    local working_directory='/usr/local/bin'
    local log="${ROOTDIR}/tee-worker/log/worker${i}.log"

    generate_service_file "${service_name}" "${description}" "${command_exec}" "${working_directory}" "${log}"

    # Move the service to systemd
    cp -r "worker${i}.service" ~/.config/systemd/user
    systemctl --user daemon-reload
    echo "Starting worker service"
    cd ~/.config/systemd/user/
    systemctl --user start "worker${i}".service
  done
}

# Function responsible for upgrading worker
function upgrade_worker(){
  echo "Upgrading Worker"
  cd $ROOTDIR/tee-worker
  echo "Fetching New MRENCLAVE Value"
  output=$(make mrenclave 2>&1)
  if [[ $? -eq 0 ]]; then
      mrenclave_value=$(echo "$output" | awk '{print $2}')
      echo "MRENCLAVE value: $mrenclave_value"
      export NEW_MRENCLAVE="$mrenclave_value"
  else
      echo "Failed to extract MRENCLAVE value."
  fi
  echo "Fetching Enclave Signing Key"
  log=$(cd bin && ./integritee-service signing-key 2>&1)
  enclave_account=$(echo "$log" | awk '/Enclave account:/{print $NF}')
  if [[ -n $enclave_account ]]; then
      echo "Enclave account value: $enclave_account"
      export ENCLAVE_ACCOUNT="$enclave_account"
      echo "ENCLAVE_ACCOUNT exported successfully."
  else
      echo "Failed to extract Enclave account value."
  fi

  latest_sidechain_sync_block
  latest_parentchain_sync_block

  echo "Setting up the new Worker on Chain"
  cd $ROOTDIR/scripts/ts-utils/
  yarn install
  npx ts-node setup-enclave.ts  $ENCLAVE_ACCOUNT $NEW_MRENCLAVE $SCHEDULE_UPDATE_BLOCK

  echo "Stopping Currently running Worker..."
  stop_old_worker

  echo "Migrating shards for new worker.."
  migrate_worker


  cd $ROOTDIR
  commands=()
  worker_count=$(echo "$CONFIG" | jq '.workers | length')
  echo "Worker Count is: ${worker_count}"

  for ((i = 0; i < worker_count; i++)); do

      local WORKERTMPDIR=$ROOTDIR/tee-worker/tmp/w$i
      # Rename the original db to backup file
      mv $WORKERTMPDIR/light_client_db.bin $WORKERTMPDIR/light_client_db.bin.backup

      # Rename the backup file to replace the original file
      mv $WORKERTMPDIR/light_client_db.bin.1 $WORKERTMPDIR/light_client_db.bin

      echo "Replacement complete. light_client_db has been replaced with light_client_db.bin.1."

      rm $ROOTDIR/tee-worker/log/worker$i.log
      WORKER_DIR=$ROOTDIR/tee-worker/tmp/w$i


      source=$(echo "$CONFIG" | jq -r ".workers[$i].source")
      flags=$(echo "$CONFIG" | jq -r ".workers[$i].flags[]")
      subcommand_flags=$(echo "$CONFIG" | jq -r ".workers[$i].subcommand_flags[]")

      command="./integritee-service"


      skip_next_flag=false

      for flag in $flags; do
        if $skip_next_flag; then
          skip_next_flag=false
          continue
        fi

        if [[ $flag == "--clean-reset" ]]; then
          continue  # Skip adding "--clean-reset"
        fi

        if [[ $flag == "--parentchain-start-block" ]]; then
          skip_next_flag=true
          command+=" $flag $LATEST_FINALIZED_BLOCK"
        else
          command+=" $flag"
        fi
      done

      command+=" run"

      for subcommand_flag in $subcommand_flags; do
        command+=" $subcommand_flag"
      done

      local command_exec="/bin/bash -c  'cd ${WORKER_DIR} && ${command}'"
      local service_name="worker${i}"
      local description='Worker Service for Litentry Side chain'
      local working_directory='/usr/local/bin'
      local log="${ROOTDIR}/tee-worker/log/worker${i}.log"

      generate_service_file "${service_name}" "${description}" "${command_exec}" "${working_directory}" "${log}"

      # Move the service to systemd
      cp -r "worker${i}.service" ~/.config/systemd/user
      systemctl --user daemon-reload
      echo "Starting worker service"
      cd ~/.config/systemd/user/
      systemctl --user start "worker${i}".service
    done
}

function stop_old_worker(){
  TIMEOUT=300  # Timeout in seconds
  SECONDS=0
  while (( SECONDS < TIMEOUT )); do
      LOG_FILE="$ROOTDIR/tee-worker/log/worker0.log"
        if grep -q "Enclave did not produce sidechain blocks" "$LOG_FILE"; then
            echo "Enclave has stoppped producing blocks, Stopping it now"
            worker_count=$(echo "$CONFIG" | jq '.workers | length')

            for ((i = 0; i < worker_count; i++)); do
              systemctl --user stop "worker${i}".service
            done
            echo "Sleeping for 120 seconds, So that old worker can be stopped gracefully.."
            sleep 120
        fi
      sleep 10
  done
}

function migrate_worker(){
  cd $ROOTDIR/tee-worker
  # Copy integritee-service binary and enclave_signed.so to ./tmp/w0
  cp ./bin/integritee-service ./tmp/w0
  cp ./bin/enclave.signed.so  ./tmp/w0
  cd ./tmp/w0 || exit

  echo "Old MRENCLAVE VALUE: $OLD_MRENCLAVE"
  echo "New MRENCLAVE VALUE: $NEW_MRENCLAVE"
  # Run the migration command
  ./integritee-service migrate-shard --old-shard $OLD_MRENCLAVE --new-shard $NEW_MRENCLAVE

  # Navigate to ./tmp/w0/shards
  cd shards || exit

  # Delete the old shard value
  rm -r $OLD_SHARD

  echo "Migration of shards completed"
}



function latest_sidechain_sync_block(){
  # Fetch Latest Block Produced
  line=$(grep '\[.*\]$' $ROOTDIR/tee-worker/log/worker0.log | tail -n 1 2>&1)
  number=$(echo "$line" | sed -E 's/.*\[([0-9]+)\]$/\1/')
  current_sidechain_end_block=$((number + 50))
  echo "The next enclave is scheduled to start producing blocks after: $current_sidechain_end_block blocks"
  export SCHEDULE_UPDATE_BLOCK="$current_sidechain_end_block"
}

function latest_parentchain_sync_block(){
  # JSON-RPC request payload
  request='{"jsonrpc":"2.0","id":1,"method":"chain_getHeader","params":[]}'

  # Make the JSON-RPC request and retrieve the latest finalized block
  response=$(curl -s -H "Content-Type: application/json" -d "$request" http://$PARACHAIN_HOST:$PARACHAIN_PORT)
  hex_number=$(echo "$response" | grep -oP '(?<="number":")[^"]+')
  dec_number=$(printf "%d" "$hex_number")


  # Store the latest finalized block number in an environment variable
  export LATEST_FINALIZED_BLOCK=${dec_number}

  echo "Latest finalized block number: $LATEST_FINALIZED_BLOCK"
}

function build_parachain(){
  if [ "$COPY_FROM_DOCKER" = true ]; then
    docker pull litentry/litentry-parachain:tee-prod
    img_id=$(docker create litentry/litentry-parachain:tee-prod)
    docker cp $img_id:/usr/local/bin/litentry-collator $ROOTDIR/
    docker rm -v $img_id
  else
    cd $ROOTDIR
    make build-node
  fi
}

function build_worker(){
  cd $ROOTDIR/tee-worker/
  source /opt/intel/sgxsdk/environment
  SGX_COMMERCIAL_KEY=$ROOTDIR/tee-worker/enclave-runtime/Enclave_private.pem SGX_PRODUCTION=1 make
}

# Default values
verbose=false
build=false
discard=false
config=""
auto_start=false
export CHAIN=rococo
export ONLY_WORKER=false
export PARACHAIN_HOST="localhost"
export PARACHAIN_PORT="9944"
export DOCKERIMAGE="litentry/litentry-parachain:tee-prod"
export COPY_FROM_DOCKER=false

# Parse command-line options and arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help)
      display_help
      exit 0
      ;;
    -b|--build)
      build=true
      shift
      ;;
    -d|--discard)
      discard=true
      shift
      ;;
    -c|--config)
      if [[ $# -lt 2 ]]; then
        echo "Error: The config file name is missing."
        display_help
        exit 1
      fi
      config="$2"
      shift 2
      ;;
    -a|--only-worker)
      export ONLY_WORKER=true
      shift
      ;;
    -x| --chain)
      export CHAIN="$2"
      shift
      ;;
    -p| --parachain-port)
      export PARACHAIN_PORT="$2"
      shift
      ;;
    -h| --parachain-host)
      export PARACHAIN_HOST="$2"
      shift
      ;;
    -v| --copy-from-docker)
      export COPY_FROM_DOCKER=true
      export DOCKERIMAGE="$2"
      shift
      ;;
    restart|upgrade-worker)
      action="$1"
      shift
      ;;
    *)
      echo "Error: Unknown option or argument '$1'."
      display_help
      exit 1
      ;;
  esac
done

# Create systemd folder for user if not already present
mkdir -p ~/.config/systemd/user

if [ -n "$config" ]; then
  echo "Config file: $config"
fi

export CONFIG=$(cat $config)

# Move log files to log-backup
if [ -d "$ROOTDIR/tee-worker/log" ]; then
  new_folder_name=$(date +"$ROOTDIR/tee-worker/log-backup/log-%Y%m%d-%H%M%S")
  mkdir -p $new_folder_name
  cp -r "$ROOTDIR/tee-worker/log" "$new_folder_name"
  cp /tmp/parachain_dev/*.log $new_folder_name
  echo "Backup log into $new_folder_name"
fi

# Backup worker folder
worker_count=$(echo "$CONFIG" | jq '.workers | length')
for ((i = 0; i < worker_count; i++)); do
    if [ -d "$ROOTDIR/tee-worker/tmp/w$i" ]; then
        new_folder_name=$(date +"$ROOTDIR/tee-worker/tmp/w$i-%Y%m%d-%H%M%S")
        mkdir -p new_folder_name
        cp -r $ROOTDIR/tee-worker/tmp/w$i $new_folder_name
        echo "Backing up, previous worker binary $new_folder_name"
    fi
done

if [ "$discard" = true ]; then
  echo "Cleaning the existing state for Parachain and Worker."
  stop_running_services
  rm -rf /tmp/parachain_dev/
  worker_count=$(echo "$CONFIG" | jq '.workers | length')
  for ((i = 0; i < worker_count; i++)); do
      if [ -d "$ROOTDIR/tee-worker/tmp/w$i" ]; then
          echo "Deleting Previous worker $ROOTDIR/tmp/w$i"
          rm -r "$ROOTDIR/tee-worker/tmp/w$i"
      fi
  done
fi

# Get old MRENCLAVE
if [ "$action" = "upgrade-worker" ]; then
  cd $ROOTDIR/tee-worker
  output=$(make mrenclave 2>&1)
  if [[ $? -eq 0 ]]; then
    mrenclave_value=$(echo "$output" | awk '{print $2}')
    echo "MRENCLAVE value: $mrenclave_value"
    export OLD_MRENCLAVE="$mrenclave_value"
  else
    echo "Failed to extract MRENCLAVE value."
    exit 1
  fi

  # Fetch Base58 value for MRENCLAVE
  cd $ROOTDIR/tee-worker/bin
  export OLD_SHARD=$(./integritee-service mrenclave)
  echo "Old Shard value: ${OLD_SHARD}"
fi


if [ "$build" = true ]; then
  echo "Building the binary for Parachain and Worker."
  build_parachain
  build_worker
fi

echo "Action: $action"

if [ "$action" = "restart" ]; then
  echo "restarting Services"
  restart
elif [ "$action" = "upgrade-worker" ]; then
  echo "Upgrading Worker"
  upgrade_worker
fi