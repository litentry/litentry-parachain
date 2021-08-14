#!/bin/bash

TMP_DIR=$1

# pid files
RELAY_ALICE_PIDFILE="$TMP_DIR/relay.alice.pid"
RELAY_BOB_PIDFILE="$TMP_DIR/relay.bob.pid"
PARA_ALICE_PIDFILE="$TMP_DIR/para.alice.pid"
PARA_BOB_PIDFILE="$TMP_DIR/para.bob.pid"
TOKEN_SERVER_PIDFILE="$TMP_DIR/token-server.pid"
