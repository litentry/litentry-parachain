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

from py.worker import Worker
from py.helpers import GracefulKiller, mkdir_p

log_dir = 'log'
mkdir_p(log_dir)
node_log = open(f'{log_dir}/node.log', 'w+')


def setup_worker(work_dir: str, source_dir: str, std_err: Union[None, int, IO]):
    print(f'Setting up worker in {work_dir}')
    print(f'Copying files from {source_dir}')
    worker = Worker(cwd=work_dir, source_dir=source_dir, std_err=std_err)
    worker.init_clean()
    print('Initialized worker.')
    return worker


def run_worker(config, i: int):
    log = open(f'{log_dir}/worker{i}.log', 'w+')
    w = setup_worker(f'tmp/w{i}', config["source"], log)

    print(f'Starting worker {i} in background')
    return w.run_in_background(log_file=log, flags=config["flags"], subcommand_flags=config["subcommand_flags"])


def main(processes, config_path, parachain_type):
    print('Starting litentry-parachain in background')

    with open(config_path) as config_file:
        config = json.load(config_file)

    if parachain_type == "local" :
        # litentry: start parachain via shell script
        # TODO: use Popen and copy the stdout also to node.log
        run(['./scripts/litentry/start_parachain.sh'])

        print('Starting litentry-parachain done')
        print('----------------------------------------')

    i = 1
    for w_conf in config["workers"]:
        processes.append(run_worker(w_conf, i))
        # sleep to prevent nonce clash when bootstrapping the enclave's account
        sleep(3)
        if i == 1:
             # Give worker 1 some time to register itself, otherwise key & state sharing will not work.
             #
             # litentry: increase the gap between worker launch
             #           we need a cleaner solution though, see https://github.com/integritee-network/worker/issues/731
            sleep(180)

        i += 1

    # keep script alive until terminated
    signal.pause()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Run a setup consisting of a node and some workers')
    parser.add_argument('config', type=str, help='Config for the node and workers')
    parser.add_argument('parachain', nargs='?', default="local", type=str, help='Config for parachain selection: local / remote')
    args = parser.parse_args()

    process_list = []
    killer = GracefulKiller(process_list)
    main(process_list, args.config, args.parachain)
