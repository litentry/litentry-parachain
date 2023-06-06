#!/bin/bash

#!/bin/bash

LOG_FILE="log/worker0.log"
TIMEOUT=240  # Timeout in seconds
SERVICE_PROCESS="./integritee-service"

# Function to check if the log file contains the desired string and kill the service process
check_log_file_and_kill_process() {
    if grep -q "Enclave did not produce sidechain blocks" "$LOG_FILE"; then
        echo "Enclave has stopped producing blocks."
        # Get the current user's username
        current_user=$(whoami)

        # Find the process IDs (PIDs) of all processes containing "integritee-service" for the current user
        pids=$(pgrep -u "$current_user" -f "integritee-service")

        # Check if any processes are running
        if [ -z "$pids" ]; then
            echo "No integritee-service processes found for user $current_user."
        else
            # Kill the processes
            echo "Killing integritee-service processes for user $current_user..."
            echo "$pids"
            kill -9 "$pids"
            echo "Processes killed."
        fi
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
