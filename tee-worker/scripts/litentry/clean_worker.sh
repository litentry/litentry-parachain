#!/bin/bash

pids=$(pgrep -f "integritee-service")

# Check if any processes are running
if [ -z "$pids" ]; then
      echo "No integritee-service processes found for user $current_user."
else
      # Kill the processes
      echo "Killing integritee-service processes for user $current_user..."
      echo "Integritee Processes: $pids"
      kill -9 $pids
      echo "Processes killed."
fi