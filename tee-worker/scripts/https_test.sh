#!/bin/bash
set -euo pipefail

# Runs https test demo: Either set `CLIENT_DIR` env var directly or run script with:
#
# source ./init_env.sh && ./https_test.sh

echo "$CLIENT_DIR"

cd "$CLIENT_DIR" || exit

LOG_1="${LOG_1:-$LOG_DIR/https_test.log}"

echo "[https_test.sh] printing to logs:"
echo "        $LOG_1"

touch "$LOG_1"

./demo_https_test.sh -p 9944 -P 2000 -t first 2>&1 | tee "$LOG_1"
