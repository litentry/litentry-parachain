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
from time import sleep
from typing import Union, IO

import pycurl
from io import BytesIO

from py.worker import Worker
from py.helpers import GracefulKiller, mkdir_p

log_dir = 'log'
mkdir_p(log_dir)


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


def main(processes, config_path, parachain_type):
    print('Starting litentry-parachain in background')

    with open(config_path) as config_file:
        config = json.load(config_file)

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
