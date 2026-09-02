#!/bin/bash

# Build and copy the daemon if not exist (using mock mode/local libavrocore.so if not installed in /usr/lib)
echo "Building components for IPC tests..."
fpc -Mdelphi -fPIC -Tlinux @src/core/fpc.cfg src/core/libavrocore.pas -obuild/libavrocore.so
fpc -Mdelphi -Tlinux @src/core/fpc.cfg src/core/avro_cli.pas -obuild/avro_cli

echo "Starting avro-daemon in background..."
export PYTHONPATH="$(pwd)/src/daemon"
python3 src/daemon/avro-daemon > /tmp/avro_daemon.log 2>&1 &
DAEMON_PID=$!

# Wait for socket to be created
sleep 2

FAILS=0

echo "Running avro-cli tests..."

# Check daemon is alive
if ! kill -0 $DAEMON_PID 2>/dev/null; then
    echo "Daemon failed to start!"
    cat /tmp/avro_daemon.log
    exit 1
fi

check() {
    cmd=$1
    expected=$2
    result=$(python3 src/daemon/avro-cli $cmd)
    if [ "$result" = "$expected" ]; then
        echo "PASS: avro-cli $cmd -> $result"
    else
        echo "FAIL: avro-cli $cmd -> $result (Expected: $expected)"
        FAILS=1
    fi
}

check "get-mode" "EN"
check "get-layout" "Avro Phonetic"
python3 src/daemon/avro-cli toggle > /dev/null
check "get-mode" "BN"
check "convert amra" "আম্রা"
check "convert bangla" "বাংলা"
python3 src/daemon/avro-cli toggle > /dev/null
check "get-mode" "EN"
check "convert amra" "amra"

# Clean up
echo "Killing daemon..."
kill $DAEMON_PID

if [ $FAILS -ne 0 ]; then
    echo "Some IPC tests failed!"
    exit 1
else
    echo "All IPC tests passed!"
    exit 0
fi
