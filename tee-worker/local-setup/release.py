#!/usr/bin/env python3
# NOTE: This can only be run on Production/Staging Machine with necessary configuration and User Privileges

import os
import subprocess
import argparse

# TODO: This should also allow for logs under /usr/logs
def generate_service_file(service_name, description, command, working_directory):
    service_template = f'''[Unit]
Description={description}

[Service]
ExecStart={command}
WorkingDirectory={working_directory}
Restart=always

[Install]
WantedBy=multi-user.target
'''
    service_filename = f'{service_name}.service'
    with open(service_filename, 'w') as file:
        file.write(service_template)
    print(f'Service file "{service_filename}" generated successfully.')

# If we want to stop an existing worker which is not producing blocks
# def stop_running_worker():

def auto_start():
    print("Auto Starting Parachain and Worker")

def upgrade_worker():
    print("Preparing to upgrade the Worker")

# Build parachain and relay chain service files
def register_enclave():
    print("Register Enclave to Chain")


def build_parachain_release(parachain_source):
    command = './scripts/launch-local-binary.sh rococo'
    working_directory = parachain_source
    service_name = 'litentry-parachain'
    description = 'Parachain Setup for Litentry'

    generate_service_file(service_name, description, command, working_directory)
    print("Service file for Parachain has been built succesfully")


def build_worker_release():
    # Step 1: Make release-pkg
    output = subprocess.check_output('make release-pkg', shell=True).decode().strip()
    print(output)
    # Identify the command line arguments
    command = './integritee-service --clean-reset -P 2000 -w 2001 -r 3443 -h 4545 --running-mode mock --enable-mock-server --parentchain-start-block 0 run --skip-ra --dev'
    service_name = 'worker'
    description = 'Worker Service for Litentry Side chain'
    working_directory = '/opt/worker/'

    generate_service_file(service_name, description, command, working_directory)
    print("Service file for worker has been created succesfully!!")


def main(parachain_source, auto_start):
    # If complete reset:
    build_parachain_release(parachain_source)
    build_worker_release()
    # Perform Daemon Reload

    if auto_start:
        # start the services
        print("Starting Services")


    # upgrade


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Run a setup consisting of a node and some workers')
    parser.add_argument('--parachain-source', type=str, help='Source directory for Parachain')
    parser.add_argument('--auto-start', type=bool, nargs='?', default=False, help='To start the services automatically at the end of the script')
    parser.add_argument('--upgrade-worker', type=bool, nargs='?', default=False, help='Upgrade the worker and restart the worker.service')

    # parser.add_argument('parachain', nargs='?', default="local-docker", type=str, help='Config for parachain selection: local-docker / local-binary / remote')
    # arser.add_argument('offset', nargs='?', default="0", type=int, help='offset for port')
    args = parser.parse_args()


    process_list = []
    # killer = GracefulKiller(process_list, args.parachain)
    if main(args.parachain_source, args.auto_start) == 0:
        print("Program has exited ")