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

RUNNING_MODE=${mode:-"dev"}

# Fixed values:
WORKER_ENDPOINT="localhost"
MU_RA_PORT="3443"
UNTRUSTED_HTTP_PORT="4545"
TRUSTED_WORKER_PORT="2000"
UNTRUSTED_WORKER_PORT="3000"

F_CLEAN=""
FSUBCMD_DEV=""
FSUBCMD_REQ_STATE=""

if [ "${CLEANUP}" = 'true' ]; then
	F_CLEAN="--clean-reset"
	FSUBCMD_DEV="--dev"
fi

echo "Number of WORKER_NUM: ${WORKER_NUM}"
##############################################################################
### Start execution
##############################################################################

ROOTDIR=$(git rev-parse --show-toplevel)
ROOTDIR="${ROOTDIR}/tee-worker"
RUST_LOG="info,integritee_service=info,ws=warn,sp_io=error,substrate_api_client=warn, \
itc_parentchain_light_client=debug, \
jsonrpsee_ws_client=warn,jsonrpsee_ws_server=warn, enclave_runtime=warn,ita_stf=debug, \
its_rpc_handler=warn,itc_rpc_client=warn,its_consensus_common=debug,its_state=warn, \
its_consensus_aura=warn,aura*=warn,its_consensus_slots=warn"

#./integritee-service init-shard H8wzxGBcKa1k5tXMALACo9P7uKS5rYFL8e3mMAEVe7Ln

for ((i = 0; i < ${WORKER_NUM}; i++)); do
	worker_name="worker${i}"
	echo ""
	echo "--------------------setup worker(${worker_name})----------------------------------------"

	if ((i > 0)); then
		FSUBCMD_REQ_STATE="--request-state"
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
--running-mode ${RUNNING_MODE} \
run --skip-ra ${FSUBCMD_DEV} ${FSUBCMD_REQ_STATE}"

	echo "${worker_name} command: ${launch_command}"
	eval "${launch_command}" > "${ROOTDIR}"/log/${worker_name}.log 2>&1 &
	echo "${worker_name}(integritee-service) started successfully. log: ${ROOTDIR}/log/${worker_name}.log"

	# How to get dockerirze: wget https://github.com/jwilder/dockerize/releases/download/v0.6.1/dockerize-linux-amd64-v0.6.1.tar.gz
	if ((${WORKER_NUM} > 0)); then
		"${ROOTDIR}"/dockerize -wait-retry-interval 10s -wait http://localhost:${untrusted_http_port}/is_initialized -timeout 600s
	fi
done

echo ""
echo "--- Setup work(s) done ---"
