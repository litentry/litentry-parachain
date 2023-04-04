#!/usr/bin/env python3

# This script will prepare the worker(s) execution directory and start the
# worker(s) based a configuration file. It only takes care of worker(s).
# No parachain or relaychain.
#
# 'worker-config.json' is a reference and good start point, in which:
#   - 'execution_path': an absolute path for execution
#   - 'key_path'      : an absolute path for enclave keys (prepare in advance)
#   - '--running-mode': can have an absolute path of json file for mode config
#   - when '--running-mode' is not 'mock', then skip the '--enable-mock-server'
#
# For the running-mode config file, a good reference is:
#   ./tee-worker/service/src/running-mode-config.json
#   but only take out the 'prod' module for production environment.

import os
import argparse
import json
import shutil
import pycurl
from datetime import datetime
from subprocess import Popen, STDOUT
from time import sleep
from typing import Union, TextIO, IO
from io import BytesIO


def mkdir_p(path):
    if os.path.exists(path):
        print("Directory already exists: ", path, " - will be removed.")
        shutil.rmtree(path)

    os.makedirs(path)
    print("Directory created:", path)

class Worker:
    def __init__(self,
                 name,
                 cmd,
                 flags,
                 subcmd_flags,
                 rust_log,
                 log: TextIO,
                 cwd,
                 url
                 ):
        self.name = name
        self.cmd = [cmd]
        self.flags = flags
        self.subcmd_flags = subcmd_flags
        self.log = log
        self.cwd = cwd
        self.url = url
        self.rust_log = rust_log
        pass

    def _assemble_cmd(self):
        full_cmd = self.cmd
        if self.flags:
            full_cmd += self.flags
        full_cmd += ['run']
        if self.subcmd_flags:
            full_cmd += self.subcmd_flags
        return full_cmd

    def run(self):
        env = dict(os.environ, RUST_LOG=self.rust_log)
        full_cmd = self._assemble_cmd()
        print()
        print(f'Start {self.name}...')
        self.process = Popen(full_cmd,
                             env=env,
                             stdout=self.log,
                             stderr=STDOUT,
                             bufsize=1,
                             cwd=self.cwd)

        sleep(5)
        c = pycurl.Curl()
        c.setopt(pycurl.URL, self.url)
        while True:
            sleep(5)
            buffer = BytesIO()
            c.setopt(c.WRITEDATA, buffer)
            try:
                c.perform()
            except Exception as e:
                print("Connect to worker: " + str(e))
                return False

            if "I am initialized." == buffer.getvalue().decode('iso-8859-1'):
                break

        c.close()

        print(f'{self.name} initialized successfully.')
        return True

class WorkerManager:
    def __init__(self,
                 config_path,
                 fresh_start,
                 cmd: str = './integritee-service',
                 std_err: Union[None, int, IO] = STDOUT,
                 ):
        """
        WorkerManager manages worker(s).
        TODO: comment
        """
        self.config_path = config_path
        self.fresh_start = fresh_start
        self.cmd = cmd
        self.std_err = std_err
        self.workers = []
        self.processes = []

    def _check_config(self, config):
        bad_file = False
        if not 'rust_log' in config:
            print("missing 'rust_log' field")
            bad_file = True

        if not 'execution_path' in config:
            print("missing 'execution_path' field")
            bad_file = True
        else:
            if not os.path.isabs(config["execution_path"]):
                print("'execution_path' must be absolute path")
                bad_file = True

        worker_idx = 0
        for w_conf in config["workers"]:
            if not 'valid' in w_conf:
                print("missing 'valid' field in " +
                      str(worker_idx) + "(th) worker config")
                bad_file = True

            if not 'key_path' in w_conf:
                print("missing 'key_path' field in " +
                      str(worker_idx) + "(th) worker config")
                bad_file = True
            else:
                if not os.path.isabs(w_conf["key_path"]):
                    print("'key_path' field in " +
                          str(worker_idx) + "(th) worker config must be absolute path")
                    bad_file = True

            if not 'flags' in w_conf:
                print("missing 'flags' field in " +
                      str(worker_idx) + "(th) worker config")
                bad_file = True

            if not 'subcommand_flags' in w_conf:
                print("missing 'subcommand_flags' field in " +
                      str(worker_idx) + "(th) worker config")
                bad_file = True

            worker_idx = worker_idx + 1

        return bad_file

    def _setup_working_dir(self, source_dir: str, target_dir: str, files: [str]):
        for file in files:
            source = f'{source_dir}/{file}'
            target = f'{target_dir}/{file}'

            if os.path.exists(source):
                shutil.copy(source, target)
            else:
                print(f'{source} does not exist. Please check.')
                return False

        return True

    def prepare_workers(self):
        with open(self.config_path) as config_file:
            config = json.load(config_file)

        if self._check_config(config):
            print("Bad config file, exit")
            return False

        # By default, 'enclave.signed.so', 'integritee-service' should come along with this script.
        src_dir = os.path.dirname(os.path.abspath(__file__))
        log_dir = os.path.join(config["execution_path"], 'log')

        worker_idx = 0
        for w_conf in config["workers"]:
            worker_dir = os.path.join(
                config["execution_path"], f'worker{worker_idx}')
            if self.fresh_start:
                mkdir_p(worker_dir)
                files = ['enclave.signed.so', 'integritee-service']
                if not self._setup_working_dir(src_dir, worker_dir, files):
                    return False
                files = ['ed25519_key_sealed.bin', 'rsa3072_key_sealed.bin',
                         'aes_key_sealed.bin', 'key.txt', 'spid.txt']
                if not self._setup_working_dir(w_conf["key_path"], worker_dir, files):
                    return False
            else:
                if "--clean-reset" in w_conf["flags"]:
                    w_conf["flags"].remove("--clean-reset")

            if w_conf["valid"]:
                idx = 0
                if ("-h" in w_conf["flags"]):
                    idx = w_conf["flags"].index("-h") + 1
                elif ("--untrusted-http-port" in w_conf["flags"]):
                    idx = w_conf["flags"].index("--untrusted-http-port") + 1
                else:
                    print("No \"--untrusted-http-port\" provided in config file")
                    return False
                untrusted_http_port = w_conf["flags"][idx]
                url = 'http://localhost:' + \
                    str(untrusted_http_port) + '/is_initialized'

                now = datetime.now().strftime("%Y%m%d-%H%M%S")
                log_path = f'{worker_dir}/{now}.log'
                log = open(log_path, 'w+')
                self.workers.append(Worker(f'worker{worker_idx}',
                                           self.cmd,
                                           w_conf["flags"],
                                           w_conf["subcommand_flags"],
                                           config["rust_log"],
                                           log,
                                           worker_dir,
                                           url))

            worker_idx = worker_idx + 1

        return True

    def run_workers(self):
        for worker in self.workers:
            if worker.run():
                self.processes.append(worker.process)
            else:
                print(f'{worker.name} failed.')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        description='Run worker(s) in production env')
    parser.add_argument('config_path', type=str,
                        help='Config file for worker(s)')
    parser.add_argument('fresh_start', nargs='?', default='False', type=str,
                        help='Whether this is a fresh start, or continue with last start history')
    args = parser.parse_args()

    fresh_start = False if args.fresh_start.lower() == 'false' else True

    worker_mgr = WorkerManager(
        config_path=args.config_path, fresh_start=fresh_start)

    if not worker_mgr.prepare_workers():
        print("Worker preparation failure, exit!")
        exit()

    worker_mgr.run_workers()
    print(worker_mgr.processes)
    pass
