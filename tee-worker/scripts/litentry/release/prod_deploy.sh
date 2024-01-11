#!/bin/bash

set -eo pipefail

# ------------------------------
# path setting
# ------------------------------

# CONFIG_DIR contains private configs. Need to prepare in advance.
CONFIG_DIR="/opt/worker_configs"

BASEDIR=/opt/litentry
DOWNLOAD_BASEDIR="$BASEDIR/download"
WORKER_BASEDIR="$BASEDIR/worker"
BACKUP_BASEDIR="$BASEDIR/backup"
WORKER_BACKUP_BASEDIR="$BACKUP_BASEDIR/worker"
PARACHAIN_LOG_BACKUP_BASEDIR="$BACKUP_BASEDIR/parachain-log"
PARACHAIN_BASEDIR="$BASEDIR/parachain"
RELAYCHAIN_ALICE_BASEDIR="$PARACHAIN_BASEDIR/relay-alice"
RELAYCHAIN_BOB_BASEDIR="$PARACHAIN_BASEDIR/relay-bob"
PARACHAIN_ALICE_BASEDIR="$PARACHAIN_BASEDIR/para-alice"

# ------------------------------
# default arg setting
# ------------------------------
DISCARD=false
WORKER_CONFIG="$CONFIG_DIR/config.json"
INTEL_KEY="$CONFIG_DIR/key_production.txt"
INTEL_SPID="$CONFIG_DIR/spid_production.txt"
CHAIN=rococo
WITH_PARACHAIN=false
PARACHAIN_PORT=9944
PARACHAIN_HOST=localhost
REPO="litentry/litentry-parachain"
RELEASE_VERSION=
ACTION=


# ------------------------------
# Some global setting
# ------------------------------
WORKER_COUNT=
PARACHAIN_ID=
OLD_MRENCLAVE=
NEW_MRENCLAVE=
OLD_SHARD=
LATEST_FINALIZED_BLOCK=
RELEASE_PKGDIR=
TEMPLATE_DIR=
TS_TESTS_DIR=
TS_UTILS_DIR=

SGX_SDK=/opt/intel/sgxsdk
SGX_ENCLAVE_SIGNER=$SGX_SDK/bin/x64/sgx_sign

VERSION_PATTERN="^p[0-9]+\.[0-9]+\.[0-9]+-[0-9]+-w[0-9]+\.[0-9]+\.[0-9]+-[0-9]+$"

# ------------------------------
# main()
# ------------------------------
function main {
  if [ "$#" -eq 0 ]; then
    display_help
    exit 0
  fi

  # 0/ parse command lines
  echo "Parsing command line ..."
  while [ $# -gt 0 ]; do
    case "$1" in
      -h|--help)
        display_help
        exit 0
        ;;
      -d|--discard)
        DISCARD=true
        shift
        ;;
      -c|--config)
        WORKER_CONFIG="$(realpath -s $2)"
        shift 2
        ;;
      -k|--key)
        INTEL_KEY="$(realpath -s $2)"
        shift 2
        ;;
      -i|--spid)
        INTEL_SPID="$(realpath -s $2)"
        shift 2
        ;;
      -x|--chain)
        CHAIN="$2"
        shift 2
        ;;
      -w|--with-parachain)
        WITH_PARACHAIN=true
        shift
        ;;
      -p|--parachain-port)
        PARACHAIN_PORT="$2"
        shift 2
        ;;
      -z|--parachain-host)
        PARACHAIN_HOST="$2"
        shift 2
        ;;
      -r|--repo)
        REPO="$2"
        shift 2
        ;;
      -v|--version)
        RELEASE_VERSION="$2"
        shift 2
        ;;
      generate|restart|upgrade-worker)
        ACTION="$1"
        shift
        ;;
      *)
        echo "Error: unknown option or subcommand $1"
        display_help
        exit 1
        ;;
    esac
  done

  # 1/ sanity checks
  for f in "$WORKER_CONFIG" "$INTEL_KEY" "$INTEL_SPID"; do
    if [ ! -f "$f" ]; then
      echo "file not found: $f"
      exit 1
    fi
  done

  if [[ $RELEASE_VERSION =~ $VERSION_PATTERN ]]; then
    echo "Version is valid: $RELEASE_VERSION"
  else
      if [[ -z $RELEASE_VERSION ]]; then
          # If $RELEASE_VERSION is empty, read from current_version.txt
          if [[ -f $DOWNLOAD_BASEDIR/current_version.txt ]]; then
              RELEASE_VERSION=$(<$DOWNLOAD_BASEDIR/current_version.txt)
              echo "Using version from $DOWNLOAD_BASEDIR/current_version.txt: $RELEASE_VERSION"
          else
              echo "Error: No version provided, and $DOWNLOAD_BASEDIR/current_version.txt does not exist."
              exit 1
          fi
      else
          echo "Error: Invalid version: $RELEASE_VERSION"
          exit 1
      fi
  fi

  RELEASE_PKGDIR="$DOWNLOAD_BASEDIR/$RELEASE_VERSION/bin/"
  TEMPLATE_DIR="$DOWNLOAD_BASEDIR/$RELEASE_VERSION/template/"
  TS_TESTS_DIR="$DOWNLOAD_BASEDIR/$RELEASE_VERSION/ts_tests/"
  TS_UTILS_DIR="$DOWNLOAD_BASEDIR/$RELEASE_VERSION/ts_utils/"

  # 2/ check if $USER has sudo
  if sudo -l -U $USER 2>/dev/null | grep -q 'may run the following'; then
    source "$SGX_SDK/environment"
  else
    echo "$USER doesn't have sudo permission"
    exit 1
  fi

  # 3/ create folders if missing
  sudo mkdir -p "$BASEDIR"
  sudo chown -R $USER:$GROUPS "$BASEDIR"
  for d in "$WORKER_BASEDIR" "$WORKER_BACKUP_BASEDIR" "$PARACHAIN_LOG_BACKUP_BASEDIR" \
    "$RELAYCHAIN_ALICE_BASEDIR" "$RELAYCHAIN_BOB_BASEDIR" "$PARACHAIN_ALICE_BASEDIR" \
    "$RELEASE_PKGDIR" "$TEMPLATE_DIR" "$TS_TESTS_DIR" "$TS_UTILS_DIR"; do
    mkdir -p "$d"
  done

  echo "$RELEASE_VERSION" > "$DOWNLOAD_BASEDIR/current_version.txt"

  WORKER_COUNT=$(cat "$WORKER_CONFIG" | jq '.workers | length')
  echo "Worker count: $WORKER_COUNT"

  # 4/ download release packages and other files
  if [[ "$ACTION" == "generate" ]]; then
    download_release_package
    download_service_template_util_files
  fi

  # 5/ main business logic
  case "$ACTION" in
    generate)
      backup_services
      generate_services
      exit
      ;;
    restart)
      stop_services
      prune
      start_services
      ;;
    upgrade-worker)
      latest_parentchain_block
      set_scheduled_enclave
      wait_for_sidechain
      stop_worker_services
      get_old_mrenclave
      setup_working_dir
      migrate_shard
      remove_clean_reset_in_service
      update_parachain_start_block_in_service
      start_worker_services
      exit
      ;;
    *)
      echo "Unknow action: $ACTION"
      exit 1
      ;;
  esac

}

# ------------------------------
# helper functions
# ------------------------------
function print_divider {
  echo "------------------------------------------------------------"
}

function display_help {
  echo "usage: ./prod_deploy.sh <subcommands> [options]"
  echo ""
  echo "subcommands:"
  echo "  generate           Generate the parachain and worker systemd files"
  echo "  restart            Restart the services"
  echo "  upgrade-worker     Upgrade the worker"
  echo ""
  echo "options:"
  echo "  -h, --help                   Display this help message and exit"
  echo "  -d, --discard                Clean the existing state for parachain and worker (default: false)"
  echo "  -c, --config <config.json>   Config file for the worker"
  echo "  -k, --key <key.txt>          Production key for remote IAS attestation (Will be deprecated later)"
  echo "  -i, --spid <spid.txt>        Production SPID for remote IAS attestation (Will be deprecated later)"
  echo "  -x, --chain <chain name>     Chain type for launching the parachain network (default: rococo)"
  echo "  -w, --with-parachain         Start parachain locally, typically for testing purpose (default: false)"
  echo "  -p, --parachain-port <port>  Parachain ws port (default: 9944)"
  echo "  -z, --parachain-host <host>  Parachain ws URL (default: localhost)"
  echo "  -r, --repo <repo>            GitHub repo name (default: litentry/litentry-parachain)"
  echo "  -v, --version <version>      GitHub repo release version"
  echo ""
  echo "examples:"
  echo "  ./deploy.sh generate --config tmp.json"
  echo "  ./deploy.sh restart --config tmp.json"
  echo "  ./deploy.sh restart --config tmp.json --with-parachain"
  echo "  ./deploy.sh upgrade-worker"
  echo ""
  echo "notes:"
  echo "  - This script requires an OS that supports systemd."
  echo "  - It is mandatory to provide a JSON config file, production key and spid for the worker."
  echo "  - jq is required to be installed on the system "
  echo ""
  echo "For more information or assistance, please contact Litentry parachain team."
}

function download_release_package {
  # download release package
  echo "Downloading release package binary ..."

  release_url="https://api.github.com/repos/$REPO/releases/tags/$RELEASE_VERSION"
  response=$(curl -s "$release_url")

  if [ "$(echo "$response" | jq -r '.message')" != "null" ]; then
    echo "Error: Failed to fetch release information."
    exit 1
  fi

  asset_urls=($(echo "$response" | jq -r '.assets[].browser_download_url'))
  for asset_url in "${asset_urls[@]}"; do
    output_filename="$RELEASE_PKGDIR/$(basename "$asset_url")"
    echo "Downloading: $asset_url, to: $output_filename"
    curl -LJ "$asset_url" -o "$output_filename"
  done

  # download polkadot
  echo "Downloading polkadot binary ..."
  url="https://github.com/paritytech/polkadot/releases/download/v0.9.42/polkadot"
  polkadot_bin="$RELEASE_PKGDIR/polkadot"
  wget -O "$polkadot_bin" -q "$url"
  if [ ! -s "$polkadot_bin" ]; then
    echo "$polkadot_bin is 0 bytes, download URL: $url" && exit 1
  fi
  if ! "$polkadot_bin" --version &> /dev/null; then
    echo "Cannot execute $polkadot_bin, wrong executable?" && exit 1
  fi
}

function download_service_template_util_files {
  file_list_url="https://api.github.com/repos/$REPO/git/trees/$RELEASE_VERSION?recursive=1"
  response=$(curl -s "$file_list_url")

  if [ "$(echo "$response" | jq -r '.message')" != "null" ]; then
      echo "Error: Failed to fetch file list from GitHub API."
      exit 1
  fi

  # download service template
  echo "Downloading service template ..."
  download_files "tee-worker/scripts/litentry/release/template/" "$TEMPLATE_DIR"

  # download ts-tests
  echo "Downloading ts tests ..."
  download_files "ts-tests/" "$TS_TESTS_DIR"

  # download ts-utils
  echo "Downloading ts utils ..."
  download_files "scripts/ts-utils/" "$TS_UTILS_DIR"

  # download extract_identity
  echo "Downloading extract_identity ..."
  extract_identity_url="https://raw.githubusercontent.com/$REPO/$RELEASE_VERSION/tee-worker/extract_identity"
  extract_identity_file="$TS_UTILS_DIR/extract_identity"
  echo "Downloading: $transfer_ts_url, to: $extract_identity_file"
  curl -LJ "$extract_identity_url" -o "$extract_identity_file"
}

function download_files {
  local directory="$1"
  local target_dir="$2"

  echo "Downloading files from $directory ..."

  for file_info in $(echo "$response" | jq -c '.tree[] | select(.path | startswith("'$directory'"))'); do
    file_name=$(echo "$file_info" | jq -r '.path')
    file_type=$(echo "$file_info" | jq -r '.type')

    if [ "$file_type" == "blob" ]; then
      # It's a file, download it
      file_url="https://raw.githubusercontent.com/$REPO/$RELEASE_VERSION/$file_name"
      output_filename="$target_dir${file_name#$directory}"

      # Check if the directory exists before creating it
      if [ ! -d "$(dirname "$output_filename")" ]; then
        echo "Creating directory: $(dirname "$output_filename")"
        mkdir -p "$(dirname "$output_filename")"
      fi

      echo "Downloading: $file_url, to: $output_filename"
      curl -LJ "$file_url" -o "$output_filename"
    elif [ "$file_type" == "tree" ]; then
      # It's a directory, create it
      mkdir -p "$target_dir${file_name#$directory}"
    fi
  done
}

function backup_services {
  echo "Backing up services ..."
  now=$(date +"%Y%m%d-%H%M%S")
  cd /etc/systemd/system || exit
  outdir="$WORKER_BACKUP_BASEDIR/service-$now"
  mkdir -p "$outdir"
  for f in para-alice.service relay-alice.service relay-bob.service $(ls worker*.service 2>/dev/null); do
    [ -f "$f" ] && cp "$f" "$outdir" || true
  done
}

function generate_services {
  echo "Generating systemd service files ..."
  temp_dir=$(mktemp -d)
  cd $TEMPLATE_DIR && cp * $temp_dir && cd $temp_dir
  sed -i "s/CHAIN/$CHAIN/g" *.service
  sed -i "s/USER/$USER/g" *.service
  for ((i = 0; i < $WORKER_COUNT; i++)); do
    cp worker.service worker$i.service
    sed -i "s/NUMBER/$i/g" worker$i.service
    # populate args
    flags=$(cat "$WORKER_CONFIG" | jq -r ".workers[$i].flags[]")
    subcommand_flags=$(cat "$WORKER_CONFIG" | jq -r ".workers[$i].subcommand_flags[]")
    args=
    for flag in $flags; do
      args+=" $flag"
    done
    args+=" run"
    for subcommand_flag in $subcommand_flags; do
      args+=" $subcommand_flag"
    done
    sed -i "s;ARGS;$args;" worker$i.service
    sudo mv worker$i.service -f /etc/systemd/system/
  done
  rm worker.service

  if [ "$WITH_PARACHAIN" = true ]; then
    sudo cp *.service -f /etc/systemd/system/
  fi

  cd ~
  rm -rf "$temp_dir"
  sudo systemctl daemon-reload
  echo "Done, please check files under /etc/systemd/system/"
  echo "Restart the services to take effect"
}

function stop_services {
  stop_worker_services

  if [ "$WITH_PARACHAIN" = true ]; then
    stop_parachain_services
  fi
}

function stop_worker_services {
  echo "Stopping worker services ..."
  for ((i = 0; i < $WORKER_COUNT; i++)); do
    sudo systemctl stop "worker$i.service"
    sleep 5
  done
  backup_workers
}

function backup_workers {
  echo "Backing up workers ..."
  now=$(date +"%Y%m%d-%H%M%S")
  cd "$WORKER_BASEDIR" || exit
  for i in $(ls -d * 2>/dev/null); do
    outdir="$WORKER_BACKUP_BASEDIR/$i-$now"
    cp -rf "$i" "$outdir"
    echo "Worker backed up into $outdir"
  done
}

function stop_parachain_services {
  echo "Stopping parachain services ..."
  sudo systemctl stop para-alice.service relay-alice.service relay-bob.service
  backup_parachain_logs
}

function backup_parachain_logs {
  if [[ -f $PARACHAIN_BASEDIR/*.log ]]; then
    echo "Backing up parachain logs ..."
    now=$(date +"%Y%m%d-%H%M%S")
    outdir="$PARACHAIN_LOG_BACKUP_BASEDIR/log-$now"
    mkdir -p "$outdir"
    cp "$PARACHAIN_BASEDIR"/*.log "$outdir" || true
    echo "Parachain logs backed up into $outdir"
  fi
}

function prune {
  if [ "$DISCARD" = true ]; then
    echo "Pruning the existing state ..."
    if [ "$WITH_PARACHAIN" = true ]; then
      echo "remove everything under $PARACHAIN_BASEDIR"
      rm -rf "$PARACHAIN_BASEDIR"/*
    fi

    echo "remove everything under $WORKER_BASEDIR"
    rm -rf "$WORKER_BASEDIR"/*
  fi
}

function start_services {
  if [ "$WITH_PARACHAIN" = true ]; then
    start_parachain_services
  fi

  latest_parentchain_block
  set_scheduled_enclave
  setup_working_dir
  update_parachain_start_block_in_service
  start_worker_services
  echo "Done"
}

function start_parachain_services {
  echo "Restarting parachain services ..."

  cp "$RELEASE_PKGDIR/polkadot" "$PARACHAIN_BASEDIR"
  chmod a+x "$PARACHAIN_BASEDIR/polkadot"
  cp "$RELEASE_PKGDIR/litentry-collator" "$PARACHAIN_BASEDIR"
  chmod a+x "$PARACHAIN_BASEDIR/litentry-collator"

  cd "$PARACHAIN_BASEDIR" || exit
  ./polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-chain-spec.json
  ./litentry-collator export-genesis-state --chain $CHAIN-dev > genesis-state
  ./litentry-collator export-genesis-wasm --chain $CHAIN-dev > genesis-wasm

  sudo systemctl daemon-reload
  sudo systemctl restart relay-alice.service
  sleep 5
  sudo systemctl restart relay-bob.service
  sleep 5
  sudo systemctl restart para-alice.service
  sleep 5
  register_parachain
}

function register_parachain {
  echo "Register parathread now ..."

  # get parachain id
  file_path="node/src/chain_specs/$CHAIN.rs"
  file_url="https://raw.githubusercontent.com/$REPO/$RELEASE_VERSION/$file_path"
  file_content=$(curl -s "$file_url")
  export PARACHAIN_ID=$(echo "$file_content" | grep "DEFAULT_PARA_ID" | grep "u32" | sed 's/.* = //;s/\;//')

  cd "$TS_TESTS_DIR" || exit
  if [[ -z "$NODE_ENV" ]]; then
      echo "NODE_ENV=ci" > .env
  else
      echo "NODE_ENV=$NODE_ENV" > .env
  fi
  cp config.ci.json config.ci.json.backup
  # The genesis state path file needs to be updated as it is hardcoded to be /tmp/parachain_dev
  jq --arg genesis_state "$PARACHAIN_BASEDIR/genesis-state" --arg genesis_wasm "$PARACHAIN_BASEDIR/genesis-wasm" '.genesis_state_path = $genesis_state | .genesis_wasm_path = $genesis_wasm' config.ci.json > config.ci.json.1
  mv config.ci.json.1 config.ci.json
  pnpm install
  pnpm run register-parathread 2>&1 | tee "$PARACHAIN_BASEDIR/register-parathread.log"
  print_divider

  echo "Upgrade parathread to parachain now ..."
  # Wait for 90s to allow onboarding finish, after that we do the upgrade
  sleep 90
  pnpm run upgrade-parathread 2>&1 | tee "$PARACHAIN_BASEDIR/upgrade-parathread.log"
  print_divider

  echo "done. please check $PARACHAIN_BASEDIR for generated files if need"
  print_divider
  mv config.ci.json.backup config.ci.json
}

function start_worker_services {
  echo "Restarting worker services ..."
  sudo systemctl daemon-reload
  for ((i = 0; i < $WORKER_COUNT; i++)); do
    sudo systemctl restart "worker$i.service"
    sleep 5
  done
}

function setup_working_dir {
  echo "Setting up working dir ..."

  for ((i = 0; i < $WORKER_COUNT; i++)); do
    worker_dir="$WORKER_BASEDIR/w$i"
    mkdir -p "$worker_dir"

    # INTEL_KEY and INTEL_SPID will be deprecated later once switch to DCAP working mode
    [ -f $INTEL_KEY ] && cp -f "$INTEL_KEY" "$worker_dir"
    [ -f $INTEL_SPID ] && cp -f "$INTEL_SPID" "$worker_dir"
    [ -f $RELEASE_PKGDIR/enclave.signed.so ] && cp -f "$RELEASE_PKGDIR/enclave.signed.so" "$worker_dir"
    [ -f $RELEASE_PKGDIR/litentry-worker ] && cp -f "$RELEASE_PKGDIR/litentry-worker" "$worker_dir"

    cd "$worker_dir"
    [ -f litentry_lcdb/db.bin.backup ] && cp -f litentry_lcdb/db.bin.backup litentry_lcdb/db.bin

    chmod a+x litentry-worker
    enclave_account=$(./litentry-worker signing-key | grep -oP '^Enclave account: \K.*$$')

    echo "Transferring balance to the enclave account $enclave_account ..."
    cd $TS_UTILS_DIR || exit
    pnpm install
    pnpm exec ts-node transfer.ts $enclave_account
    echo "Transferring balance finished"
  done
}

function set_scheduled_enclave {
  echo "Setting scheduled enclave ..."
  cd $RELEASE_PKGDIR || exit
  NEW_MRENCLAVE=$(<"mrenclave.txt")
  echo "new mrenclave: $NEW_MRENCLAVE"

  latest_sidechain_block

  echo "Setting up the new worker on chain ..."
  cd $TS_TESTS_DIR || exit
  pnpm install
  pnpm run setup-enclave $NEW_MRENCLAVE $SCHEDULED_UPDATE_BLOCK
}

# TODO: here we only read worker0 logs here
function latest_sidechain_block {
  if [ -f $WORKER_BASEDIR/w0/worker.log ]; then
    block_number=$(grep -F 'Enclave produced sidechain blocks' $WORKER_BASEDIR/w0/worker.log | tail -n 1 | sed 's/.*\[//;s/]//')
    SCHEDULED_UPDATE_BLOCK=$((block_number + 20))
    echo "Current sidechain block: $block_number, scheduled update block: $SCHEDULED_UPDATE_BLOCK"
  else
    SCHEDULED_UPDATE_BLOCK=0
    echo "No history data. Start worker from fresh. scheduled update block: $SCHEDULED_UPDATE_BLOCK"
  fi
}

function wait_for_sidechain {
  echo "Waiting for sidechain to reach block $SCHEDULED_UPDATE_BLOCK ..."
  found=false
  for _ in $(seq 1 20); do
    sleep 10
    block_number=$(grep -F 'Enclave produced sidechain blocks' $WORKER_BASEDIR/w0/worker.log | tail -n 1 | sed 's/.*\[//;s/]//')
    echo "current sidechain block: $block_number"
    if [ $((block_number+1)) -eq $SCHEDULED_UPDATE_BLOCK ]; then
      echo "we should stall soon ..."
    fi
    if tail -n 50 $WORKER_BASEDIR/w0/worker.log | grep -q "Skipping sidechain block $SCHEDULED_UPDATE_BLOCK due to mismatch MRENCLAVE"; then
      echo "we reach $SCHEDULED_UPDATE_BLOCK now"
      found=true
      break
    fi
  done
  if [ $found = false ]; then
    echo "not reached, timeout"
    exit 1
  fi
}

function get_old_mrenclave {
  cd "$WORKER_BASEDIR/w0" || exit
  OLD_SHARD=$(./litentry-worker mrenclave)
  $SGX_ENCLAVE_SIGNER dump -enclave ./enclave.signed.so -dumpfile df.out
  OLD_MRENCLAVE=$(python3 $TS_UTILS_DIR/extract_identity < df.out | awk '{print $2}')
  rm df.out
  echo "old shard: $OLD_SHARD"
  echo "old mrenclave: $OLD_MRENCLAVE"
}

function migrate_shard {
  echo "Migrating shards for workers ..."
  for ((i = 0; i < $WORKER_COUNT; i++)); do
    cd "$WORKER_BASEDIR/w$i" || exit
    echo "old MRENCLAVE: $OLD_MRENCLAVE"
    echo "new MRENCLAVE: $NEW_MRENCLAVE"
    ./litentry-worker migrate-shard --old-shard $OLD_MRENCLAVE --new-shard $NEW_MRENCLAVE

    cd shards || exit
    rm -rf $OLD_SHARD
  done
  echo "Done"
}

function remove_clean_reset_in_service {
  echo "Removing --clean-reset flag for workers service ..."
  for ((i = 0; i < $WORKER_COUNT; i++)); do
    sudo sed -i 's/--clean-reset//' /etc/systemd/system/worker$i.service
  done
  echo "Done"
}

function update_parachain_start_block_in_service {
  echo "Update parachain start block for workers service ..."
  for ((i = 0; i < $WORKER_COUNT; i++)); do
    sudo sed -i 's/--parentchain-start-block [0-9]\+/--parentchain-start-block '"$LATEST_FINALIZED_BLOCK"'/' /etc/systemd/system/worker$i.service
  done
  echo "Done"
}

function latest_parentchain_block {
  echo "get latest parentchain block ..."
  # JSON-RPC request payload
  request='{"jsonrpc":"2.0","id":1,"method":"chain_getHeader","params":[]}'

  # Make the JSON-RPC request and retrieve the latest finalized block
  response=$(curl -s -H "Content-Type: application/json" -d "$request" http://$PARACHAIN_HOST:$PARACHAIN_PORT)
  hex_number=$(echo "$response" | grep -oP '(?<="number":")[^"]+')
  LATEST_FINALIZED_BLOCK=$(printf "%d" "$hex_number")
  echo "Current parachain block: $LATEST_FINALIZED_BLOCK"
}

main "$@"