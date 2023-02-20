#!/usr/bin/env bash

# TODO: Sanity check of parameters
while getopts ":c:n:u:p:m:" opt; do
	case $opt in
		c)
			cleanup_flag=$OPTARG
			;;
		n)
			worker_num=$OPTARG
			;;
		u)
			node_url=$OPTARG
			;;
		p)
			node_port=$OPTARG
			;;
		m)
			mode=$OPTARG
			;;
	esac
done

CLEANUP=${cleanup_flag:-true}
WORKER_NUM=${worker_num:-1}

NODE_URL=${node_url:-"ws://127.0.0.1"}	# "ws://host.docker.internal"
NODE_PORT=${node_port:-"9944"}			# "9946"

RUNNING_MODE=${mode:-"mock"}

# Fixed values:
WORKER_ENDPOINT="localhost"
MU_RA_PORT="3443"
UNTRUSTED_HTTP_PORT="4545"
TRUSTED_WORKER_PORT="2000"
UNTRUSTED_WORKER_PORT="3000"

F_CLEAN=""
FSUBCMD_DEV=""
FSUBCMD_REQ_STATE=""

WAIT_INTERVAL_SECONDS=5
WAIT_ROUNDS=20

if [ "${CLEANUP}" = 'true' ]; then
	F_CLEAN="--clean-reset"
	FSUBCMD_DEV="--dev"
fi

function wait_worker_is_initialized()
{	
	for i in $(seq 1 $WAIT_ROUNDS); do
		state=$(curl -s http://localhost:$1/is_initialized)
		if [ "$state" == "I am initialized." ]; then
			echo "Initialization successful: $state"
            return
        else
			echo "sleep $WAIT_INTERVAL_SECONDS"
            sleep $WAIT_INTERVAL_SECONDS
        fi		
    done
	echo
    echo "Worker initialization failed"
    exit 1
}

echo "Number of WORKER_NUM: ${WORKER_NUM}"
##############################################################################
### Start execution
##############################################################################

ROOTDIR=$(git rev-parse --show-toplevel)
ROOTDIR="${ROOTDIR}/tee-worker"
RUST_LOG="info,integritee_service=info,ws=warn,sp_io=error,substrate_api_client=warn,\
itc_parentchain_light_client=warn,\
jsonrpsee_ws_client=warn,jsonrpsee_ws_server=warn,enclave_runtime=warn,ita_stf=debug,\
its_rpc_handler=warn,itc_rpc_client=warn,its_consensus_common=debug,its_state=warn,\
its_consensus_aura=warn,aura*=warn,its_consensus_slots=warn"

#./integritee-service init-shard H8wzxGBcKa1k5tXMALACo9P7uKS5rYFL8e3mMAEVe7Ln

for ((i = 0; i < ${WORKER_NUM}; i++)); do
	worker_name="worker${i}"
	echo ""
	echo "--------------------setup worker(${worker_name})----------------------------------------"

	if ((i > 0)); then
		FSUBCMD_REQ_STATE="--request-state"
	fi

	if ((i == 0)); then
		MOCK_SERVER="--enable-mock-server"
	fi

	if [ "${CLEANUP}" = 'true' ]; then
		echo "clear dir: ${ROOTDIR}/tmp/${worker_name}"
		rm -rf "${ROOTDIR}"/tmp/"${worker_name}"
	fi
	mkdir -p "${ROOTDIR}"/tmp/"${worker_name}"
	for Item in 'enclave.signed.so' 'key.txt' 'spid.txt' 'integritee-service' 'integritee-cli'; do
		cp "${ROOTDIR}/bin/${Item}" "${ROOTDIR}"/tmp/"${worker_name}"
	done	

	cd "${ROOTDIR}"/tmp/${worker_name} || exit
	echo "enter ${ROOTDIR}/tmp/${worker_name}"

	mu_ra_port=$((${MU_RA_PORT} + i))
	untrusted_http_port=$((${UNTRUSTED_HTTP_PORT} + i))
	trusted_worker_port=$((${TRUSTED_WORKER_PORT} + i))
	untrusted_worker_port=$((${UNTRUSTED_WORKER_PORT} + i))
	echo "${worker_name} ports:
	mu-ra-port: ${mu_ra_port}
	untrusted-http-port: ${untrusted_http_port}
	trusted-worker-port: ${trusted_worker_port}
	untrusted-worker-port: ${untrusted_worker_port}
	"

	launch_command="RUST_LOG=${RUST_LOG} ./integritee-service ${F_CLEAN} --ws-external \
--mu-ra-external-address ${WORKER_ENDPOINT} \
--mu-ra-port ${mu_ra_port} \
--node-port ${NODE_PORT} \
--node-url ${NODE_URL} \
--trusted-external-address wss://${WORKER_ENDPOINT} \
--trusted-worker-port ${trusted_worker_port} \
--untrusted-external-address ws://${WORKER_ENDPOINT} \
--untrusted-http-port ${untrusted_http_port} \
--untrusted-worker-port ${untrusted_worker_port} \
--running-mode ${RUNNING_MODE} ${MOCK_SERVER} \
run --skip-ra ${FSUBCMD_DEV} ${FSUBCMD_REQ_STATE}"

	echo "${worker_name} command: ${launch_command}"
	eval "${launch_command}" > "${ROOTDIR}"/log/${worker_name}.log 2>&1 &
	echo "${worker_name}(integritee-service) started successfully. log: ${ROOTDIR}/log/${worker_name}.log"

	if ((${WORKER_NUM} > 0)); then		
		wait_worker_is_initialized ${untrusted_http_port}
	fi
done

echo ""
echo "--- Setup work(s) done ---"
