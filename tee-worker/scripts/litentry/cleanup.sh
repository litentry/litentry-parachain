#!/bin/bash

set -eo pipefail

pid=$(ps aux | grep '[l]ocal-setup/launch' | awk '{print $2}')

if [ ! -z "$pid" ]; then
  echo "killing $pid"
  kill -9 "$pid"
fi

killall integritee-service 2>/dev/null || true
