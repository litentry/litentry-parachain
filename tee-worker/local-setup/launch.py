#!/usr/bin/env python3
"""
Launch handily a local dev setup consisting of one integritee-node and some workers.

Example usage: `./local-setup/launch.py /local-setup/simple-config.py`

The node and workers logs are piped to `./log/node.log` etc. folder in the current-working dir.

run: `cd local-setup && tmux_logger.sh` to automatically `tail -f` these three logs.

"""
import argparse
import json
import signal
from subprocess import Popen, PIPE, STDOUT, run
import sys
import os
from time import sleep
from typing import Union, IO
from dotenv import load_dotenv

import pycurl
from io import BytesIO

from py.worker import Worker
from py.helpers import GracefulKiller, mkdir_p

import socket

log_dir = 'log'
mkdir_p(log_dir)

OFFSET=10
PORTS = ['AliceWSPort', 'AliceRPCPort', 'AlicePort', 'BobWSPort', 'BobRPCPort', 'BobPort', 'CollatorWSPort', 'CollatorRPCPort', 'CollatorPort', 'TrustedWorkerPort', 'UntrustedWorkerPort', 'MuRaPort', 'UntrustedHttpPort']

def setup_worker(work_dir: str, source_dir: str, std_err: Union[None, int, IO]):
    print(f'Setting up worker in {work_dir}')
    print(f'Copying files from {source_dir}')
    worker = Worker(cwd=work_dir, source_dir=source_dir, std_err=std_err)
    worker.init_clean()
    print('Initialized worker.')
    return worker


def run_worker(config, i: int):
    log = open(f'{log_dir}/worker{i}.log', 'w+')
    # TODO: either hard-code 'local-setup' directory, or take from input config.json
    w = setup_worker(f'tmp/w{i}', config["source"], log)

    print(f'Starting worker {i} in background')
    return w.run_in_background(log_file=log, flags=config["flags"], subcommand_flags=config["subcommand_flags"])

# Function to check if a port is open
def is_port_open(port):
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.bind(('127.0.0.1', int(port)))
        sock.close()
        return True
    except OSError:
        return False

# Function to reallocate port if it is not available
def reallocate_ports(env_name, port):
    # Offset the original port by 10
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

    print("All Preliminary Port Checks Completed")


def generate_json_config_file():
    data = {
        "eth_address": "[0x4d88dc5d528a33e4b8be579e9476715f60060582]",
        "private_key": "0xe82c0c4259710bb0d6cf9f9e8d0ad73419c1278a14d375e5ca691e7618103011",
        "ocw_account": "5FEYX9NES9mAJt1Xg4WebmHWywxyeGQK8G3oEBXtyfZrRePX",
        "parachain_ws": "ws://localhost:" + os.environ.get("CollatorWSPort", "9944"),
        "relaychain_ws": "ws://localhost:" + os.environ.get("AliceWSPort", "9946")
    }
    file_path = "../ts-tests/config.local.json"

    with open(file_path, "w") as json_file:
        json.dump(data, json_file, indent=4)

    print("Config data has been written to", file_path)

import os

def generate_config_file():
    config = '''NODE_ENV = local
WORKER_END_POINT = ws://localhost:{}
SUBSTRATE_END_POINT = ws://localhost:{}
ID_HUB_URL='http://localhost:3000'''

    # Get the value of the environment variables or use default values
    worker_end_point = os.environ.get("TrustedWorkerPort", "2000")
    substrate_end_point = os.environ.get("CollatorWSPort", "9944")

    # Replace the placeholders with the environment variable values
    config = config.replace("{}", worker_end_point, 1)
    config = config.replace("{}", substrate_end_point, 1)

    file_path = "ts-tests/.env.local"

    with open(file_path, "w") as config_file:
        config_file.write(config)

    print("Configuration has been written to", file_path)




def main(processes, config_path, parachain_type):
    ## Load environment file
    load_dotenv('.env')
    ## Check Ports and Automatically Reallocate
    check_all_ports()
    generate_json_config_file()
    generate_config_file()


    print('Starting litentry-parachain in background')

    with open(config_path) as config_file:
        config = json.load(config_file)
        config['workers'][0]['flags'] = [flag.replace('$CollatorWSPort', os.environ.get('CollatorWSPort', '')) for flag in config['workers'][0]['flags']]
        config['workers'][0]['flags'] = [flag.replace('$TrustedWorkerPort', os.environ.get('TrustedWorkerPort', '')) for flag in config['workers'][0]['flags']]
        config['workers'][0]['flags'] = [flag.replace('$UntrustedWorkerPort', os.environ.get('UntrustedWorkerPort', '')) for flag in config['workers'][0]['flags']]
        config['workers'][0]['flags'] = [flag.replace('$MuRaPort', os.environ.get('MuRaPort', '')) for flag in config['workers'][0]['flags']]
        config['workers'][0]['flags'] = [flag.replace('$UntrustedHttpPort', os.environ.get('UntrustedHttpPort', '')) for flag in config['workers'][0]['flags']]

    # Litentry
    if parachain_type == "local" :
        # start parachain via shell script
        # TODO: use Popen and copy the stdout also to node.log
        run(['./scripts/litentry/start_parachain.sh'], check=True)

        print('Starting litentry-parachain done')
        print('----------------------------------------')

    c = pycurl.Curl()
    worker_i = 0
    worker_num = len(config["workers"])
    for w_conf in config["workers"]:
        processes.append(run_worker(w_conf, worker_i))
        print()
        # Wait a bit for worker to start up.
        sleep(5)

        idx = 0
        if ( "-h" in w_conf["flags"] ):
            idx = w_conf["flags"].index("-h") + 1
        elif ( "--untrusted-http-port" in w_conf["flags"]):
            idx = w_conf["flags"].index("--untrusted-http-port") + 1
        else:
            print("No \"--untrusted-http-port\" provided in config file")
            return 0
        untrusted_http_port = w_conf["flags"][idx]
        url = 'http://localhost:' + str(untrusted_http_port) + '/is_initialized'
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

                if "I am initialized." == buffer.getvalue().decode('iso-8859-1'):
                    break
                if counter >= 600:
                    print("Worker initialization timeout (3000s). Exit")
                    return 0
                counter = counter + 1

        worker_i += 1

    c.close()
    print("Worker(s) started!")

    # keep script alive until terminated
    signal.pause()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Run a setup consisting of a node and some workers')
    parser.add_argument('config', type=str, help='Config for the node and workers')
    parser.add_argument('parachain', nargs='?', default="local", type=str, help='Config for parachain selection: local / remote')
    args = parser.parse_args()

    process_list = []
    killer = GracefulKiller(process_list, args.parachain)
    if main(process_list, args.config, args.parachain) == 0:
        killer.exit_gracefully()
