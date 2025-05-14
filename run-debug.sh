#!/bin/bash
echo "Starting Orion in debug mode..."
cd "$(dirname "$0")"
RUST_BACKTRACE=1 ./target/release/background &
BG_PID=$!
sleep 1
RUST_BACKTRACE=1 ./target/release/popup_ui
wait $BG_PID
