#!/usr/bin/env python3

import os
import subprocess
import re

def extract_number(line):
    regex_pattern = r'\[(\d+)\]'
    matches = re.findall(regex_pattern, line)
    if matches:
        return int(matches[-1])
    return None

output = subprocess.check_output('make mrenclave', shell=True).decode().strip().split('\n')[0].split(" ")

if output:
    mrenclave_value = output
    print(f"MRENCLAVE value: {mrenclave_value}")
    os.environ['OLD_MRENCLAVE'] = mrenclave_value
else:
    print("Failed to extract MRENCLAVE value.")

output = subprocess.check_output('cd bin && ./integritee-service signing-key', shell=True).decode().strip().split('\n')[1].split(" ")[2]

print("Signing key", output)

if output:
    mrenclave_value = output[1]
    print(f"Enclave Signing key value: {mrenclave_value}")
    os.environ['ENCLAVE_ACCOUNT'] = mrenclave_value
else:
    print("Failed to extract MRENCLAVE value.")

os.environ['SGX_COMMERCIAL_KEY'] = '/home/faisal/litentry-parachain/tee-worker/enclave-runtime/Enclave_private.pem'
os.environ['SGX_PRODUCTION'] = '1'

command = 'make'
output = subprocess.check_output(command, shell=True).decode().strip()

# Execute command 'make mrenclave' and extract the MRENCLAVE value
output = subprocess.check_output('make mrenclave', shell=True).decode().strip().split('\n')[0].split(" ")

if output:
    mrenclave_value = output[1]
    print(f"MRENCLAVE value: {mrenclave_value}")
    os.environ['NEW_MRENCLAVE'] = mrenclave_value
else:
    print("Failed to extract MRENCLAVE value.")

# Get the latest sidechain block number

line = subprocess.check_output("grep '\[.*\]$' log/worker0.log | tail -n 1", shell=True).decode().strip();
number = extract_number(line)
current_sidechain_end_block = int(number) + 50
print("The next enclave is scheduled to start producing blocks after:", current_sidechain_end_block, "blocks ")

os.environ['SCHEDULE_UPDATE_BLOCK'] = str(current_sidechain_end_block)

# Call yarn to set the extrinsic

command = '../scripts/ts-utils/setup_enclave.sh'
output = subprocess.check_output(command, shell=True).decode().strip()

# Now we wait for existing enclave to finish producing block
command = 'scripts/litentry/stop_old_worker.sh'
output = subprocess.check_output(command, shell=True).decode().strip()

# Once the old worker cannot produce new blocks, We migrate the to new worker
command = 'scripts/litentry/migrate_worker.sh'
output = subprocess.check_output(command, shell=True).decode().strip()

# Once the above is completed, We launch the worker again
command = 'local-setup/launch.py ./local-setup/development-worker.json remote'
output = subprocess.check_output(command, shell=True).decode().strip()

# Check if the enclave has succesfully resumed or not





