#!/usr/bin/env python3
"""
Launch handily a local dev setup consisting of the parachain network and some workers.

Example usage: `./local-setup/launch.py --config local-setup/development-worker.json --parachain local-binary`

The worker log is piped to `./log/worker0.log` etc. folder in the current-working dir.

"""
import argparse
import json
import signal
from subprocess import Popen, PIPE, STDOUT, run
import os
import sys
from time import sleep
from typing import Union, IO
from dotenv import load_dotenv

import pycurl
from io import BytesIO

from py.worker import Worker
from py.helpers import GracefulKiller, mkdir_p

import socket
import toml
import datetime

log_dir = "log"
mkdir_p(log_dir)

OFFSET = 100
PORTS = [
    "AliceWSPort",
    "AliceRPCPort",
    "AlicePort",
    "BobWSPort",
    "BobRPCPort",
    "BobPort",
    "CollatorWSPort",
    "CollatorRPCPort",
    "CollatorPort",
    "TrustedWorkerPort",
    "UntrustedWorkerPort",
    "MuRaPort",
    "UntrustedHttpPort",
]


def setup_worker(work_dir: str, source_dir: str, std_err: Union[None, int, IO], log_config_path):
    print(f"Setting up worker in {work_dir}")
    print(f"Copying files from {source_dir}")

    log_level_dic = setup_worker_log_level(log_config_path)
    worker = Worker(cwd=work_dir, source_dir=source_dir, std_err=std_err, log_level_dic=log_level_dic)
    worker.init_clean()
    print("Initialized worker.")
    return worker


def run_worker(config, i: int, log_config_path):
    id = config.get('id', i)
    log = open(f"{log_dir}/worker-{id}.log", "w+")
    # TODO: either hard-code 'local-setup' directory, or take from input config.json
    w = setup_worker(f"tmp/w-{id}", config["source"], log, log_config_path)

    print(f"Starting worker {id} in background")
    return w.run_in_background(
        log_file=log, flags=config["flags"], subcommand_flags=config["subcommand_flags"]
    )


# Function to check if a port is open
def is_port_open(port):
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.bind(("127.0.0.1", int(port)))
        sock.close()
        return True
    except OSError:
        return False

# Function to reallocate port if it is not available
def reallocate_ports(env_name, port):
    # Offset the original port by 100
    new_port = int(port) + int(OFFSET)
    while not is_port_open(str(new_port)):
        new_port = int(port) + int(OFFSET)

    # Set the new port value in the environment variable
    os.environ[env_name] = str(new_port)
    print("Port for {} changed to: {}".format(env_name, os.environ.get(env_name)))


# Function to iterate over all ports and automatically reallocate
def check_all_ports_and_reallocate():
    for x in PORTS:
        if is_port_open(os.environ.get(x)):
            continue
        else:
            reallocate_ports(x, os.environ.get(x))

    print("All preliminary port checks completed")


# Generate `config.local.json` used by parachain ts utils
def generate_config_local_json(parachain_dir):
    data = {
        "eth_endpoint": "http://127.0.0.1:8545",
        "eth_address": "[0x4d88dc5d528a33e4b8be579e9476715f60060582]",
        "private_key": "0xe82c0c4259710bb0d6cf9f9e8d0ad73419c1278a14d375e5ca691e7618103011",
        "ocw_account": "5FEYX9NES9mAJt1Xg4WebmHWywxyeGQK8G3oEBXtyfZrRePX",
        "genesis_state_path": parachain_dir+"/genesis-state",
        "genesis_wasm_path": parachain_dir+"/genesis-wasm",
        "parachain_ws": "ws://localhost:" + os.environ.get("CollatorWSPort", "9944"),
        "relaychain_ws": "ws://localhost:" + os.environ.get("AliceWSPort", "9946"),
        "bridge_path": "/tmp/parachain_dev/chainbridge",
    }
    config_file = "../ts-tests/config.local.json"

    with open(config_file, "w") as f:
        json.dump(data, f, indent=4)

    print("Successfully written ", config_file)

def run_node(config, i: int):
    node_log = open(f'{log_dir}/node{i}.log', 'w+')
    node_cmd = [config["bin"]] + config["flags"]
    print(f'Run node {i} with command: {node_cmd}')
    return Popen(node_cmd, stdout=node_log, stderr=STDOUT, bufsize=1)



# Generate `.env.local` used by local enclave ts-tests
def generate_env_local():
    env_local_example_file = "ts-tests/integration-tests/.env.local.example"
    env_local_file = env_local_example_file[: -len(".example")]

    with open(env_local_example_file, "r") as f:
        data = f.read()
        data = data.replace(":2000", ":" + os.environ.get("TrustedWorkerPort", "2000"))
        data = data.replace(":9944", ":" + os.environ.get("CollatorWSPort", "9944"))

    with open(env_local_file, "w") as f:
        f.write(data)

    print("Successfully written ", env_local_file)


def offset_port(offset):
    for x in PORTS:
        port = os.environ.get(x)
        new_port = int(port) + int(offset)
        os.environ[x] = str(new_port)


def setup_environment(offset, config, parachain_dir):
    load_dotenv(".env.dev")
    offset_port(offset)
    check_all_ports_and_reallocate()
    generate_config_local_json(parachain_dir)
    generate_env_local()

    # TODO: only works for single worker for now
    for p in [
        "CollatorWSPort",
        "TrustedWorkerPort",
        "UntrustedWorkerPort",
        "MuRaPort",
        "UntrustedHttpPort",
    ]:
        config["workers"][0]["flags"] = [
            flag.replace("$" + p, os.environ.get(p, ""))
            for flag in config["workers"][0]["flags"]
        ]

def setup_worker_log_level(log_config_path):
    log_level_dic = {}
    with open(log_config_path) as f:
        log_data = toml.load(f)

        # Section
        for (section, item) in log_data.items():
            log_level_string = "";
            indx = 0

            for (k, v) in item.items():
                if indx == 0:
                    log_level_string += v+","
                else:
                    log_level_string += k+"="+v+","

                indx += 1

            log_level_dic[section] = log_level_string

    return log_level_dic


def main(processes, config_path, parachain_type, log_config_path, offset, parachain_dir):
    with open(config_path) as config_file:
        config = json.load(config_file)

    # Litentry
    print("Starting litentry parachain in background ...")
    if parachain_type == "local-docker":
        os.environ['LITENTRY_PARACHAIN_DIR'] = parachain_dir
        setup_environment(offset, config, parachain_dir)
        # TODO: use Popen and copy the stdout also to node.log
        run(["./scripts/litentry/start_parachain.sh"], check=True)
    elif parachain_type == "local-binary":
        # Export Parachain Directory as Global Variable
        os.environ['LITENTRY_PARACHAIN_DIR'] = parachain_dir
        setup_environment(offset, config, parachain_dir)
        run(["../scripts/launch-local-binary.sh", "rococo"], check=True)
    elif parachain_type == "remote":
        print("Litentry parachain should be started remotely")
    else:
        sys.exit("Unsupported parachain_type")

    print("Litentry parachain is running")
    print("------------------------------------------------------------")

    # n = 1
    # for n_conf in config["nodes"]:
    #     processes.append(run_node(n_conf, n))
    #     n += 1
    #     # let the first node begin before we start the second one, it is
    #     # easier to track the logs if they don't start at the same time.
    #     sleep(18)

    # # sleep to give the node some time to startup
    # sleep(5)

    c = pycurl.Curl()
    worker_i = 0
    worker_num = len(config["workers"])
    for w_conf in config["workers"]:
        processes.append(run_worker(w_conf, worker_i, log_config_path))
        print()
        # Wait a bit for worker to start up.
        sleep(5)

        idx = 0
        if "-h" in w_conf["flags"]:
            idx = w_conf["flags"].index("-h") + 1
        elif "--untrusted-http-port" in w_conf["flags"]:
            idx = w_conf["flags"].index("--untrusted-http-port") + 1
        else:
            print('No "--untrusted-http-port" provided in config file')
            return 0
        untrusted_http_port = w_conf["flags"][idx]
        url = "http://localhost:" + str(untrusted_http_port) + "/is_initialized"
        c.setopt(pycurl.URL, url)

        if worker_i < worker_num:
            counter = 0
            while True:
                sleep(5)
                buffer = BytesIO()
                c.setopt(c.WRITEDATA, buffer)
                try:
                    c.perform()
                except Exception as e:
                    print("Try to connect to worker error: " + str(e))
                    return 0

                if "I am initialized." == buffer.getvalue().decode("iso-8859-1"):
                    break
                if counter >= 600:
                    print("Worker initialization timeout (3000s). Exit")
                    return 0
                counter += 1

        worker_i += 1

    c.close()
    print("Worker(s) started!")

    # keep script alive until terminated
    signal.pause()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Run a setup consisting of a node and some workers"
    )
    parser.add_argument("-c", "--config", type=str, help="Config for the node and workers")
    parser.add_argument(
        "-p",
        "--parachain",
        nargs="?",
        default="local-docker",
        type=str,
        help="Config for parachain selection: local-docker / local-binary / remote",
    )
    parser.add_argument(
        "-l",
        "--log-config-path",
        nargs="?",
        default="./local-setup/worker-log-level-config.toml",
        type=str,
        help="log level config file path"
    )
    parser.add_argument(
        "-o", "--offset", nargs="?", default="0", type=int, help="offset for port"
    )
    args = parser.parse_args()

    today = datetime.datetime.now()
    formatted_date = today.strftime('%d_%m_%Y_%H%M')
    directory_name = f"parachain_dev_{formatted_date}"
    temp_directory_path = os.path.join('/tmp', directory_name)
    parachain_dir = temp_directory_path
    print("Directory has been assigned to:", temp_directory_path)

    process_list = []
    killer = GracefulKiller(process_list, args.parachain)
    if main(process_list, args.config, args.parachain, args.log_config_path, args.offset, parachain_dir) == 0:
        killer.exit_gracefully()
