import os
import signal
import subprocess
import shutil
import sys
import docker
from typing import Union, IO
from datetime import datetime


def run_subprocess(args, stdout: Union[None, int, IO], stderr: Union[None, int, IO], cwd: str = './'):
    """ Wrapper around subprocess that allows a less verbose call """

    # todo: make configurable
    env = dict(os.environ, RUST_LOG='debug,ws=warn,sp_io=warn,substrate_api_client=warn,enclave=debug')

    return subprocess.run(args, stdout=stdout, env=env, cwd=cwd, stderr=stderr).stdout.decode('utf-8').strip()


def setup_working_dir(source_dir: str, target_dir: str):
    """ Setup the working dir such that the necessary files to run a worker are contained.

     Args:
         source_dir: the directory containing the files the be copied. Usually this is the integritee-service/bin dir.
         target_dir: the working directory of the worker to be run.
     """

    optional = ['key.txt', 'spid.txt']

    for file in optional:
        source = f'{source_dir}/{file}'
        target = f'{target_dir}/{file}'

        if os.path.exists(source):
            shutil.copy(source, target)
        else:
            print(f'{source} does not exist, this is fine, but you can\'t perform remote attestation with this.')

    mandatory = ['enclave.signed.so', 'integritee-service']

    for file in mandatory:
        source = f'{source_dir}/{file}'
        target = f'{target_dir}/{file}'

        if os.path.exists(source):
            shutil.copy(source, target)
        else:
            print(f'{source} does not exist. Did you run make?')

def mkdir_p(path):
    """ Surprisingly, there is no simple function in python to create a dir if it does not exist."""
    return subprocess.run(['mkdir', '-p', path])


class GracefulKiller:
    signals = {
        signal.SIGINT: 'SIGINT',
        signal.SIGTERM: 'SIGTERM'
    }

    def __init__(self, processes, parachain_type):
        signal.signal(signal.SIGINT, self.exit_gracefully)
        signal.signal(signal.SIGTERM, self.exit_gracefully)
        self.processes = processes
        self.parachain_type = parachain_type

    def exit_gracefully(self, signum = signal.SIGTERM, frame = None):
        print("\nReceived {} signal".format(self.signals[signum]))

        print("Save Parachain/Relaychain logs")
        client = docker.from_env()
        container_list = client.containers.list()
        for container in container_list:
            if "generated-rococo-" in container.name:
                logs = container.logs()
                with open(f'log/{container.name}.log', 'w') as f:
                    f.write(logs.decode('utf-8'))

        print("Cleaning up processes.")
        for p in self.processes:
            try:
                p.kill()
            except:
                pass
        print('Cleaning tmp files, cwd = {}'.format(os.getcwd()))
        # i = 0
        # while os.path.isdir(f'tmp/w{i}'):
        #     shutil.rmtree(f'tmp/w{i}')
        #     print(f'Removed tmp/w{i}')
        #     i += 1
        if os.path.isdir(f'log'):
            new_folder_name = datetime.now().strftime("log-backup/log-%Y%m%d-%H%M%S")
            shutil.copytree(f'log', new_folder_name)
            print(f'Backup log into ' + new_folder_name)
        if self.parachain_type == "local":
            print("Cleaning up litentry-parachain...")
            subprocess.run(['./scripts/litentry/stop_parachain.sh', '||', 'true'])

        sys.exit(0)
