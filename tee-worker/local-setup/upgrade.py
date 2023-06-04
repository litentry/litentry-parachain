#!/usr/bin/env python3

import os
import subprocess

output = subprocess.check_output('make mrenclave', shell=True).decode().strip().split('\n')[0].split(" ")

if output:
    mrenclave_value = output[1]
    print(f"MRENCLAVE value: {mrenclave_value}")
    os.environ['OLD_MRENCLAVE'] = mrenclave_value
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

# # Read logs and extract the value of x
# logs = subprocess.check_output('read_logs_command', shell=True).decode().strip()
# regex_pattern = r'\[(\d+)\]'
# matches = re.findall(regex_pattern, logs)
#
# if matches:
#     last_match = int(matches[-1])
#     incremented_value = last_match + 50
#     print(f"Last match: {last_match}")
#     print(f"Incremented value: {incremented_value}")
#     os.environ['INCREMENTED_VALUE'] = str(incremented_value)
#
# # Run the bash script that calls a yarn script and stores environment variables
# subprocess.run('bash_script.sh', shell=True)
