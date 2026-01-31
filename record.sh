#!/bin/bash
# Record script for showcase GIF
# 1. Start screen recording
# 2. Run this script
# 3. Stop recording, convert to GIF

BINARY="./target/release/examples/showcase"
DURATION=20

# Pre-build
cargo build --example showcase --release 2>/dev/null

# Simulated typing
type_text() {
    for (( i=0; i<${#1}; i++ )); do
        printf '%s' "${1:$i:1}"
        sleep 0.015
    done
}

# Type command
type_text "cargo run --example showcase --release"
sleep 0.4
printf '\n'

# Fake compile pause
sleep 0.8

# Run showcase for one full cycle then auto-quit
$BINARY --duration $DURATION

# Show prompt returning (proves inline rendering)
sleep 1.5
