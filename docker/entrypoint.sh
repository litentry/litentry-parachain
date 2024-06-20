#!/bin/bash
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

PARACHAIN_BASEDIR="/opt/litentry/parachain"
REPO_DIR="/code/litentry-parachain"
# Currently, only chain type rococo is supported.
CHAIN='rococo'

check(){
    if [ -z "$CHAIN" ]; then
        export CHAIN="rococo"
    fi

    ALLOWED_VALUES=("rococo" "litmus" "litentry")

    if [[ " ${ALLOWED_VALUES[@]} " =~ " ${CHAIN} " ]]; then
        echo "CHAIN is set to a valid value: $CHAIN"
    else
        echo "Error: CHAIN environment variable must be one of: ${ALLOWED_VALUES[@]}"
        exit 1
    fi
}

init(){
    cd "$PARACHAIN_BASEDIR" || exit
    polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-chain-spec.json
    litentry-collator export-genesis-state --chain $CHAIN-dev > genesis-state
    litentry-collator export-genesis-wasm --chain $CHAIN-dev > genesis-wasm
}

run_parachain_alice(){
    echo "Starting parachain alice..."
    local flags="--base-path ${PARACHAIN_BASEDIR}/para-alice \
    --alice \
    --collator \
    --force-authoring \
    --chain ${CHAIN}-dev \
    --unsafe-ws-external \
    --unsafe-rpc-external \
    --rpc-cors=all \
    --ws-max-connections 3000 \
    --port 30333 \
    --ws-port 9944 \
    --rpc-port 9933 \
    --execution wasm \
    --state-pruning archive \
    --blocks-pruning archive \
    -- --execution wasm --chain ${PARACHAIN_BASEDIR}/rococo-local-chain-spec.json --port 30332 --ws-port 9943 --rpc-port 9932"
    echo "Flags: ${flags}"

    nohup litentry-collator ${flags} >> ${PARACHAIN_BASEDIR}/para-alice.log 2>&1 &
    parachain_alice_pid=$!
}

run_relay_alice(){
    echo "Starting relay alice..."
    local flags="--base-path ${PARACHAIN_BASEDIR}/relay-alice \
    --chain ${PARACHAIN_BASEDIR}/rococo-local-chain-spec.json \
    --alice \
    --port 30336 \
    --ws-port 9946 \
    --rpc-port 9936"
    echo "Flags: ${flags}"

    nohup polkadot ${flags} >> ${PARACHAIN_BASEDIR}/relay-alice.log 2>&1 &
    relay_alice_pid=$!
}

run_relay_bob(){
    echo "Starting relay bob..."
    local flags="--base-path ${PARACHAIN_BASEDIR}/relay-bob \
    --chain ${PARACHAIN_BASEDIR}/rococo-local-chain-spec.json \
    --bob \
    --port 30337 \
    --ws-port 9947 \
    --rpc-port 9937"
    echo "Flags: ${flags}"

    nohup polkadot ${flags} >> ${PARACHAIN_BASEDIR}/relay-bob.log 2>&1 &
    relay_bob_pid=$!
}

register_parachain(){
    echo "Register parathread now ..."
    cd "$REPO_DIR" || exit
    export PARACHAIN_ID=$(grep DEFAULT_PARA_ID node/src/chain_specs/$CHAIN.rs  | grep u32 | sed 's/.* = //;s/\;//')
    cd "$REPO_DIR/ts-tests" || exit
    if [[ -z "$NODE_ENV" ]]; then
        echo "NODE_ENV=ci" > .env
    else
        echo "NODE_ENV=$NODE_ENV" > .env
    fi
    jq --arg genesis_state "$PARACHAIN_BASEDIR/genesis-state" --arg genesis_wasm "$PARACHAIN_BASEDIR/genesis-wasm" '.genesis_state_path = $genesis_state | .genesis_wasm_path = $genesis_wasm' config.ci.json > config.ci.json.1
    mv config.ci.json.1 config.ci.json
    pnpm install
    pnpm run register-parathread 2>&1 | tee "$PARACHAIN_BASEDIR/register-parathread.log"

    echo "Upgrade parathread to parachain in 90s ..."
    # Wait for 90s to allow onboarding finish, after that we do the upgrade
    sleep 90
    pnpm run upgrade-parathread 2>&1 | tee "$PARACHAIN_BASEDIR/upgrade-parathread.log"

    echo "wait for parachain to produce block #1..."
    pnpm run wait-finalized-block 2>&1

    echo
    echo "done. please check $PARACHAIN_BASEDIR for generated files if need"
}

print_help(){
    echo "Parachain ${CHAIN} initialized successfully!"
    echo "If you need to monitor the logs, please try the command 'docker exec <containers name> tail -f /opt/litentry/parachain/para-alice.log'."
    echo "Next, it will enter daemon mode."
}

watch_pid(){
    wait -n ${relay_alice_pid} ${relay_bob_pid} ${parachain_alice_pid}
    EXIT_STATUS=$?
    kill ${relay_alice_pid} ${relay_bob_pid} ${parachain_alice_pid}
    exit $EXIT_STATUS
}

main(){
    # check
    init
    run_relay_alice
    sleep 5
    run_relay_bob
    sleep 5
    run_parachain_alice
    sleep 5
    register_parachain
    print_help
    watch_pid
}

main 
