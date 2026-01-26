#!/bin/bash

# Build the project first to avoid race conditions
echo "Building project..."
cargo build

# Function to cleanup background processes on exit
cleanup() {
    echo "Stopping Host..."
    if [ -n "$HOST_PID" ]; then
        kill $HOST_PID
    fi
    exit
}

# Trap Ctrl+C and script exit
trap cleanup SIGINT EXIT

# Start Host in background
echo "Starting Host (logs in host.log)..."
cargo run -- --host > host.log 2>&1 &
HOST_PID=$!

# Wait for host to start
sleep 2

# Start Client in foreground
echo "Starting Client..."
cargo run -- --client

# The trap will handle cleanup when the client exits
