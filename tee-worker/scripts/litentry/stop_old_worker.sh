#!/bin/bash

#!/bin/bash

LOG_FILE="log/worker0.log"
TIMEOUT=60  # Timeout in seconds
SERVICE_PROCESS="integritee-service"

# Function to check if the log file contains the desired string and kill the service process
check_log_file_and_kill_process() {
    if grep -q "Enclave did not produce blocks successfully" "$LOG_FILE"; then
        echo "Found the desired string in the log file."
        echo "Killing the service process: $SERVICE_PROCESS"
        pkill -f "$SERVICE_PROCESS"
        exit 0
    fi
}

# Start the timer
SECONDS=0

# Check the log file continuously until the timeout is reached
while (( SECONDS < TIMEOUT )); do
    check_log_file_and_kill_process

    # Sleep for 10 seconds
    sleep 10
done

# Timeout reached, exit with timeout message
echo "Timeout: The desired string was not found in the log file."
exit 1
