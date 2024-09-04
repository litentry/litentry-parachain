#!/usr/bin/env bash

# no `set -e` here as we allow commands to fail in this script

LITENTRY_PARACHAIN_DIR=${LITENTRY_PARACHAIN_DIR:-"/tmp/parachain_dev"}

case $(uname -s) in
  Darwin) os=macos ;;
  Linux) os=linux ;;
  *) echo "Unsupported os"; exit 1 ;;
esac

case $(uname -m) in
  aarch64*) arch=arm64 ;;
  x86_64) arch=x64 ;;
  *) echo "Unsuppported arch"; exit 1 ;;
esac

ZOMBIENET_BIN=zombienet-${os}-${arch}
ZOMBIENET_PID=$(pidof $ZOMBIENET_BIN)

[ -z $ZOMBIENET_PID ] || kill -2 $ZOMBIENET_PID
rm -rf "$LITENTRY_PARACHAIN_DIR"

echo "cleaned up"