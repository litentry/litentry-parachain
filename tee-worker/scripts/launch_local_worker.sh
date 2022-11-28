#!/usr/bin/env bash

# Usage: ./launch_local_worker.sh workers(number) [true|false]"
# Example: ./launch_local_worker.sh 2 true

ROOTDIR=$(git rev-parse --show-toplevel)

workers=${1:-1}
reset=$2
option_clean=""
option_dev=""
option_request_state=""
if [ "${reset}" = 'true' ]; then
	option_clean="--clean-reset"
	option_dev="--dev"
fi

worker_endpoint="localhost"
#node_url="ws://integritee-node"
#node_port="9912"
node_url="ws://host.docker.internal"
node_port="9946"

RUST_LOG="info,integritee_service=info,ws=warn,sp_io=error,substrate_api_client=warn,\
itc_parentchain_light_client=debug,\
jsonrpsee_ws_client=warn,jsonrpsee_ws_server=warn, enclave_runtime=warn,ita_stf=debug,\
its_rpc_handler=warn,itc_rpc_client=warn,its_consensus_common=debug,its_state=warn,\
its_consensus_aura=warn,aura*=warn,its_consensus_slots=warn"

#./integritee-service init-shard H8wzxGBcKa1k5tXMALACo9P7uKS5rYFL8e3mMAEVe7Ln
echo "Number of workers: ${workers}"


for ((i = 0; i < workers; i++)); do
	if ((i > 0)); then
		option_request_state="--request-state"
	fi

	worker_name="worker${i}"
	if [ "${reset}" = 'true' ]; then
		echo "clear dir: $ROOTDIR/tmp/${worker_name}"
		rm -rf "$ROOTDIR"/tmp/"${worker_name}"
	fi
	mkdir -p "$ROOTDIR"/tmp/"${worker_name}"
	for Item in 'enclave.signed.so' 'key.txt' 'spid.txt' 'integritee-service' 'integritee-cli'; do
		cp "${ROOTDIR}/bin/${Item}" "$ROOTDIR"/tmp/"${worker_name}"
	done
	echo ""
	echo "--------------------setup worker(${worker_name})----------------------------------------"

	cd "${ROOTDIR}"/tmp/${worker_name} || exit
	echo "enter ${ROOTDIR}/tmp/${worker_name}"

	mu_ra_port=$((3443 + i))
	untrusted_http_port=$((4545 + i))
	trusted_worker_port=$((2000 + i))
	untrusted_worker_port=$((3000 + i))
	echo "${worker_name} ports:
		mu-ra-port: ${mu_ra_port}
		untrusted-http-port: ${untrusted_http_port}
		trusted-worker-port: ${trusted_worker_port}
		untrusted-worker-port: ${untrusted_worker_port}
	"

	launch_command="RUST_LOG=${RUST_LOG} ./integritee-service ${option_clean} \
--mu-ra-external-address ${worker_endpoint} --mu-ra-port ${mu_ra_port} --untrusted-http-port ${untrusted_http_port} --ws-external \
--trusted-external-address wss://${worker_endpoint} --trusted-worker-port ${trusted_worker_port} \
--untrusted-external-address ws://${worker_endpoint} --untrusted-worker-port ${untrusted_worker_port} \
--node-url ${node_url} --node-port ${node_port} \
run --skip-ra ${option_dev} ${option_request_state}"

	echo "${worker_name} command: ${launch_command}"
	eval "${launch_command}" >"${ROOTDIR}"/log/${worker_name}.log 2>&1 &
	echo "${worker_name}(integritee-service) started successfully. log: ${ROOTDIR}/log/${worker_name}.log"

	if ((workers > 0)); then
		"${ROOTDIR}"/dockerize -wait-retry-interval 10s -wait http://localhost:${untrusted_http_port}/is_initialized -timeout 600s
	fi
done
